use std::fs::{self};
use std::io::{self};
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};

use users::get_user_by_uid;

pub fn execute(args: std::str::SplitWhitespace) -> Result<(), io::Error> {
    let mut flag_a = false;
    let mut flag_l = false;
    let mut flag_f = false;

    for arg in args {
        match arg {
            "-a" => flag_a = true,
            "-l" => flag_l = true,
            "-F" => flag_f = true,
            _ => {
                println!("ls: invalid option -- '{}'", arg);
                return Ok(());
            }
        }
    }

    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let mut result = String::new();
    let mut backslash = "";
    if flag_l || entries.len() > 10 {
        backslash = "\n";
    }
    let mut insert_total = true;
    for (i, entry) in entries.iter().enumerate() {
        let metadata = fs::metadata(&entry)?;
        let mut dir_suffix = "";
        if flag_f && metadata.is_dir() {
            dir_suffix = "/";
        }
        if !flag_a
            && entry
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(".")
        {
            continue;
        } else if i == 0 && flag_a {
            result.push_str(&format!(". .. {}", backslash));
        }
        if flag_l {
            let meta_permissions = format!("{:?}", metadata.permissions());
            let permissions = &meta_permissions[46..56];
            if insert_total {
                result.insert_str(0, &format!("total {}\n", metadata.blocks())); 
                insert_total = false;
            }
            // println!("blocks: {:?}, st_blocks: {:?}", metadata.blocks())
            println!("blocks: {:?} ", metadata.blocks());
            result.push_str(&format!("{} ", permissions));
            result.push_str(&format!("{} ", metadata.nlink()));
            if let Some(user) = get_user_by_uid(metadata.uid()) {
                result.push_str(&format!("{} ", user.name().to_string_lossy()));
            } else {
                result.push_str("unknown ");
            }
            if let Some(group) = get_user_by_uid(metadata.gid()) {
                result.push_str(&format!("{} ", group.name().to_string_lossy()));
            } else {
                result.push_str("unknown ");
            }
            let last_modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
            result.push_str(&format!("{} ", metadata.size()));
            result.push_str(&format!("{} ", last_modified.format("%b. %e %H:%M")));
        }
        if i == entries.len() - 1 {
            backslash = "";
        }
        result.push_str(&format!(
            "{}{} {}",
            entry.strip_prefix("./").unwrap_or(&entry).display(),
            dir_suffix,
            backslash
        ));
    }
    println!("{}", result);
    Ok(())
}
