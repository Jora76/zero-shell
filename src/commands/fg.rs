use std::{
    sync::{Arc, Mutex},
    thread,
};

use nix::{
    libc::sleep,
    sys::signal::{self, Signal},
    unistd::Pid,
};

use crate::commands::jobs;

use super::jobs::JobControl;

pub fn execute(jobs: &mut Arc<Mutex<JobControl>>) {
    // let mut job_id = 0;
    for (_id, job) in jobs.lock().unwrap().get_jobs() {
        if job.position == super::jobs::Position::Current {
            println!("{}", job.command);

            let _ = signal::kill(Pid::from_raw(job.pid as i32), Signal::SIGKILL);

            let jobs_clone = Arc::clone(&jobs);
            thread::spawn(move || {
                jobs::handle_sigstop(jobs_clone);
            });
            unsafe {
                sleep(60);
            };
            break;
        }
    }
}
