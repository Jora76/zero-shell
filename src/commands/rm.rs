use std::io;

pub fn execute(args: std::str::SplitWhitespace) -> Result<(), io::Error> {
    let mut flag_d = false;
    let mut flag_r = false;
    for (i, arg) in args.into_iter().enumerate() {
        if (i == 0 || i == 1) && arg == "-d" {
            flag_d = true;
            continue;
        }
        if (i == 0 || i == 1) && arg == "-r" {
            flag_r = true;
            continue;
        }
        if flag_r {
            match std::fs::remove_dir_all(arg) {
                Ok(_) => {}
                Err(err) => {
                    println!("rm: {}", err);
                }
            }
            continue;
        }
        if flag_d {
            match std::fs::remove_dir(arg) {
                Ok(_) => {}
                Err(err) => {
                    println!("rm: {}", err);
                }
            }
        } else {
            match std::fs::remove_file(arg) {
                Ok(_) => {}
                Err(err) => {
                    println!("rm: {}", err);
                }
            }
        }
    }
    Ok(())
}
