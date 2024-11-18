use std::str::SplitWhitespace;

pub fn execute(mut args: SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("cat: missing arguments");
        return;
    }
    if args.clone().count() > 1 {
        println!("cat: too many arguments");
        return;
    }
    let arg = args.next().unwrap_or("");
    match std::fs::read_to_string(arg) {
        Ok(content) => {
            println!("{}", content);
        }
        Err(err) => {
            println!("cat: {}", err);
            return;
        }
    }
}