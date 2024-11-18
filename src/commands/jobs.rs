use std::{
    borrow::Borrow, collections::HashMap, os::unix::process, process::Child, sync::{Arc, Mutex}
};

use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use signal_hook::iterator::Signals;
use signal_hook::consts::SIGTSTP;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProcessType {
    Foreground,
    Background,
}

#[derive(Debug, PartialEq)]
pub enum Position {
    Current,
    Previous,
    Older
}

pub struct Job {
    pub id: usize,
    pub pid: u32,
    pub command: String,
    pub process_type: ProcessType,
    pub position: Position
}
pub struct JobControl {
    jobs: HashMap<usize, Job>,
}

impl JobControl {
    pub fn new() -> JobControl {
        JobControl {
            jobs: HashMap::new(),
        }
    }

    pub fn add_job(&mut self, pid: u32, command: String, process_type: ProcessType) {
        println!("Adding job 1: {}", command);
        for (_id, job) in self.jobs.iter_mut() {
            if job.position == Position::Current {
                job.position = Position::Previous;
            } else if job.position == Position::Previous {
                job.position = Position::Older;
            }
        }

        println!("Adding job: {}", command);

        let id = self.jobs.len() + 1;
        self.jobs.insert(
            id,
            Job {
                id,
                pid,
                command,
                process_type,
                position: Position::Current,
            },
        );
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn get_jobs(&mut self) -> &HashMap<usize, Job> {
        &mut self.jobs
    }

    pub fn remove_job(&mut self, id: usize) {
        self.jobs.remove(&id);
    }

    pub fn get_job(&mut self, id: usize) -> Option<&mut Job> {
        self.jobs.get_mut(&id)
    }
}

pub fn list_jobs(args: std::str::SplitWhitespace, jobs: &mut Arc<Mutex<JobControl>>) {
    let mut flag_p = false;
    let mut flag_r = false;
    let mut flag_s = false;
    let mut flag_l = false;
    for arg in args {
        match arg {
            "-p" => flag_p = true,
            "-r" => flag_r = true,
            "-s" => flag_s = true,
            "-l" => flag_l = true,
            _ => {
                println!("Invalid argument: {}", arg);
                return;
            }
        }
    }
    let mut jobs_to_remove = Vec::new();
    for (id, job) in jobs.lock().unwrap().get_jobs() {
        let mut output = String::new();
        output.push_str(&format!("[{}]", id));
        match job.position {
            Position::Current => output.push_str("+ "),
            Position::Previous => output.push_str("- "),
            Position::Older => output.push_str(" "),
        }
        if flag_p {
            println!("{}", job.pid);
            continue;
        }
        if flag_l {
            output.push_str(&format!("{} ", job.pid));
        }
        let status = nix::sys::wait::waitpid(
            nix::unistd::Pid::from_raw(job.pid as i32),
            Some(nix::sys::wait::WaitPidFlag::WNOHANG),
        );
        match status {
            Ok(nix::sys::wait::WaitStatus::Exited(_, _)) => {
                if flag_r || flag_s {
                    continue;
                }
                output.push_str("Done\t");
            }
            Ok(nix::sys::wait::WaitStatus::Signaled(_, _, _)) => {
                if flag_r || flag_s {
                    continue;
                }
                output.push_str("Done\t");
            }
            Ok(nix::sys::wait::WaitStatus::Stopped(_, _)) => {
                if flag_r {
                    continue;
                }
                output.push_str("Stopped\t");
            }
            Ok(nix::sys::wait::WaitStatus::Continued(_)) => {
                if flag_s {
                    continue;
                }
                output.push_str("Running\t");
            }
            Err(_) => {
                jobs_to_remove.push(*id);
                continue;
            }
            _ => {
                output.push_str("Running\t");
            }
        }

        output.push_str(&format!("{}", job.command));
        println!("{}", output);
    }
    for id in jobs_to_remove {
        jobs.lock().unwrap().remove_job(id);
    }
}

pub fn handle_sigstop(jobs: Arc<Mutex<JobControl>>) {
    println!("Waiting for SIGTSTP");
    let mut signals = Signals::new(&[SIGTSTP]).unwrap();
    for signal in signals.forever() {
        println!("Received signal: {:?}", signal);
        match signal {
            SIGTSTP => {
                let jobs_clone = Arc::clone(&jobs);
                for (_id, job) in jobs_clone.lock().unwrap().get_jobs() {
                    if job.process_type == ProcessType::Foreground {
                        println!("Stopping process: {}", job.command);
                        let _ = signal::kill(Pid::from_raw(job.pid as i32), Signal::SIGKILL);
                        println!("\n[{}]+ Stopped\t {}", job.id, job.command);
                        return;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
