use std::str::SplitWhitespace;

pub fn execute(args: SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("cp: missing operand");
        return;
    }
    if args.clone().count() == 1 {
        println!("cp: missing destination file operand after '{}'", args.clone().next().unwrap_or(""));
        return;
    }
    let file = args.clone().next().unwrap_or("");
    let dest = args.clone().nth(1).unwrap_or("");
    if args.clone().count() > 2 {
        println!("cp: operands after '{}' are ignored", dest);
        return;
    }
    let mut formated_dest = dest.to_string();
    if std::path::Path::new(&dest).is_dir() {
        let file_name = std::path::Path::new(&file).file_name().unwrap().to_str().unwrap();
        formated_dest = format!("{}/{}", dest, file_name);
    }
    match std::fs::copy(file, formated_dest) {
        Ok(_) => {}
        Err(err) => {
            println!("cp: {}", err);
        }
    }
}