use std::env;

pub fn execute(mut args: std::str::SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("cd: missing arguments");
        return;
    }
    if args.clone().count() > 1 {
        println!("cd: too many arguments");
        return;
    }
    let arg = args.next().unwrap_or("");
    match env::set_current_dir(arg) {
        Ok(_) => {},
        Err(err) => {
            println!("cd: {}", err);
            return;
        }
    }
}