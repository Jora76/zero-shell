use std::str::SplitWhitespace;

pub fn execute(args: SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("touch: missing operand");
        return;
    }
    for arg in args {
        match std::fs::File::create(arg) {
            Ok(_) => {}
            Err(err) => {
                println!("touch: {}", err);
            }
        }
    }
}