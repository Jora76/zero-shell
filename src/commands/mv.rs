use std::{path::Path, str::SplitWhitespace};

pub fn execute(args: SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("mv: missing operand");
        return;
    }
    if args.clone().count() == 1 {
        println!("mv: missing destination file operand after '{}'", args.clone().next().unwrap_or(""));
        return;
    }
    let file = args.clone().next().unwrap_or("");
    let dest = args.clone().nth(1).unwrap_or("");
    if args.clone().count() > 2 {
        println!("mv: operands after '{}' are ignored", dest);
        return;
    }
    let mut formated_dest = dest.to_string();
    if Path::new(&dest).is_dir() {
        let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
        formated_dest = format!("{}/{}", dest, file_name);
    }
    match std::fs::rename(file, formated_dest) {
        Ok(_) => {}
        Err(err) => {
            println!("mv: {}", err);
        }
    }
}