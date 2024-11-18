pub fn execute() {
    println!("{}", std::env::current_dir().unwrap().to_string_lossy());
}