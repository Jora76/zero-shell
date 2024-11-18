use std::{
    sync::{Arc, Mutex},
    thread,
};

use regex::Regex;

use crate::commands::jobs;

pub fn execute_script(mut args: std::str::SplitWhitespace) {
    let script = args.nth(0).unwrap();

    let status = std::process::Command::new("bash")
        .arg("-i") // Force interactive mode
        .arg(script)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Script execution failed with status: {}", status);
    }
}

pub fn interpret_command(
    command: &str,
    args: std::str::SplitWhitespace,
    jobs: &mut Arc<Mutex<jobs::JobControl>>,
) {
    let re = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]*="[^"]*"$"#).unwrap();

    if re.is_match(command) {
        let split_str: Vec<&str> = command.split("=").collect();
        let key = split_str[0];
        let value = split_str[1];
        std::env::set_var(key, value);
        return;
    }

    let full_args = format!("{} {}", command, args.collect::<Vec<&str>>().join(" "));

    if full_args.ends_with("&") {
        let full_args_clone = full_args.replace("&", "");
        let jobs_clone = Arc::clone(&jobs);
        let handle = std::thread::spawn(move || {
            if !execute_command(full_args_clone, false, jobs_clone) {
                return;
            }
        });
        handle.join().unwrap();
    } else {
        let jobs_clone = Arc::clone(&jobs);
        if !execute_command(full_args.to_string(), true, jobs_clone) {
            return;
        }
    }
}

pub fn execute_command(
    args: String,
    wait_for_output: bool,
    jobs: Arc<Mutex<jobs::JobControl>>,
) -> bool {
    let mut command = std::process::Command::new("bash");
    command
        .arg("-c")
        .arg(args.clone())
        .stdin(std::process::Stdio::inherit());

    // Si le process est en foreground
    if wait_for_output {
        let child = command
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("Failed to execute command");

        jobs.lock()
            .unwrap()
            .add_job(child.id(), args.clone(), jobs::ProcessType::Foreground);

        thread::spawn(move || {
            jobs::handle_sigstop(jobs);
        });
        child.wait_with_output().expect("Failed to wait on child");
        return true;
    }

    // Si le process est en background
    let child = command
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    let pid = child.id();
    jobs.lock()
        .unwrap()
        .add_job(pid, args.clone(), jobs::ProcessType::Background);

    // let jobs_clone = Arc::clone(&jobs);
    // thread::spawn(move || {
    //     jobs::handle_sigstop(jobs_clone);
    // });

    println!("[{}] {}", jobs.lock().unwrap().len(), pid);

    true
}
