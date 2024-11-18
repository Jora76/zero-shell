use std::str::SplitWhitespace;

pub fn execute(args: SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("mkdir: missing operand");
        return;
    }
    for arg in args {
        match std::fs::create_dir(arg) {
            Ok(_) => {}
            Err(err) => {
                println!("mkdir: {}", err);
            }
        }
    }
}