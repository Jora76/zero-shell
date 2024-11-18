use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use crate::scripting;

mod cat;
mod cd;
mod cp;
mod echo;
pub mod jobs;
mod kill;
mod ls;
mod mkdir;
mod mv;
mod pwd;
mod rm;
mod fg;
mod bg;
mod touch;

pub fn exec_command(line: String, jobs: &mut Arc<Mutex<jobs::JobControl>>) {
    let mut args = line.split_whitespace();
    let command = args.next().unwrap_or("");
    match command {
        "echo" => echo::execute(args),
        "cd" => cd::execute(args),
        "ls" => {
            let _result = ls::execute(args);
        }
        "pwd" => pwd::execute(),
        "cat" => cat::execute(args),
        "cp" => cp::execute(args),
        "rm" => {
            let _result = rm::execute(args);
        }
        "mv" => mv::execute(args),
        "mkdir" => mkdir::execute(args),
        "touch" => {
            let _result = touch::execute(args);
        }
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
        }
        "exit" => {
            std::process::exit(0);
        }
        "run" => {
            let _result = scripting::execute_script(args);
        }
        "jobs" => {
            jobs::list_jobs(args, jobs);
        }
        "kill" => {
            let arg = args.next();
            if arg.as_slice().join("").contains("%") {
                kill::execute(arg.unwrap(), jobs);
            } else {
                scripting::interpret_command(command, args, jobs);
            }
        }
        "fg" => {
            fg::execute(jobs);
        }
        "bg" => {
            bg::execute(jobs);
        }
        _ => {
            scripting::interpret_command(command, args, jobs);
        }
    }
}
