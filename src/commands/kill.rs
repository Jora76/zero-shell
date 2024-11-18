use std::sync::{Arc, Mutex};

use nix::{sys::signal::{self, Signal}, unistd::Pid};

use super::jobs::JobControl;

pub fn execute(args: &str, jobs: &mut Arc<Mutex<JobControl>>) {
    let pid = args[1..].parse::<usize>().unwrap();
    let jobs_clone = Arc::clone(&jobs);
    for (_id, job) in jobs_clone.lock().unwrap().get_jobs() {
        if job.id == pid{
            println!("[{}]+ Complete\t {}", job.id, job.command);
            let _ = signal::kill(Pid::from_raw(job.pid as i32), Signal::SIGKILL);
            return;
        }
    }
}
