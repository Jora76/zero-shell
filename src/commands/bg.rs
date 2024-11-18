use std::sync::{Arc, Mutex};

use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};

use super::jobs::JobControl;

pub fn execute(jobs: &mut Arc<Mutex<JobControl>>) {
    for (_id, job) in jobs.lock().unwrap().get_jobs() {
        if job.position == super::jobs::Position::Current {
            println!("{} &", job.command);
            let _ = signal::kill(Pid::from_raw(job.pid as i32), Signal::SIGCONT);
            break;
        }
    }
}
