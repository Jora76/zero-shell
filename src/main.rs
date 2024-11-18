use std::{sync::{Arc, Mutex}, thread};

use colored::*;
use zero_shell::{commands::*, commands::jobs::JobControl};
fn main() -> rustyline::Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;
    // let mut jobs = JobControl::new();
    let mut jobs = Arc::new(Mutex::new(JobControl::new()));
    // let jobs_clone = Arc::clone(&jobs);
    

    loop {
        let dir = std::env::var("HOME").unwrap_or(".".to_string()) + "/.zero_shell_history";
        let readline = rl.readline(
            (format!("~{}$ ", std::env::current_dir().unwrap().to_string_lossy())
                .green()
                .bold())
            .to_string()
            .as_str(),
        );
        rl.save_history(dir.as_str()).unwrap();
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                exec_command(line.clone(), &mut jobs);
                if let Err(err) = rl.add_history_entry(line.as_str()) {
                    println!("Error adding history entry: {:?}", err);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                // println!("CTRL-C");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
