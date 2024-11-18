use std::str::SplitWhitespace;

pub fn execute(args: SplitWhitespace) {
    if args.clone().count() == 0 {
        println!("echo: missing arguments");
        return;
    }
    
    let result: Vec<String> = args
        .map(|arg| {
            let letters_only: String = arg.chars().filter(|c| c.is_alphabetic()).collect();
            let other_chars: String = arg.chars().filter(|c| !c.is_alphabetic() && *c != '\"' && *c != '$').collect();

            if arg.starts_with("$") && !arg.starts_with("$(pwd)") {
                return std::env::var(&letters_only).unwrap_or_else(|_| "".to_string()).replace("\"", "") + other_chars.as_str();
            }
            arg.replace("\"", "")
                .replace(
                    "$(pwd)",
                    format!("{}", std::env::current_dir().unwrap().to_string_lossy()).as_str(),
                )
                .to_string()
        })
        .collect();
    println!("{}", result.join(" "));
}
