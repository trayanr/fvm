use std::{env, path::PathBuf};

#[cfg(target_os = "linux")]
fn get_default_path() -> PathBuf {
    match env::var("HOME") {
        Ok(home_dir) => {
            let mut path = PathBuf::from(home_dir);
            path.push(".fvm");
            path
        }
        Err(e) => {
            panic!("Couldn't get FVM_PATH environmental variable or HOME, please declare FVM_PATH. Error: {}", e)
        }
    }
}

// add for windows and mac

pub fn get_installation_path() -> PathBuf {
    match env::var("FVM_PATH") {
        Ok(var) => {
            if var.len() == 0 {
                return PathBuf::from(&get_default_path());
            } else {
                return PathBuf::from(&var);
            }
        }
        Err(e) => match e {
            env::VarError::NotPresent => return PathBuf::from(&get_default_path()),
            env::VarError::NotUnicode(chars) => {
                panic!(
                    "Faulty charectars used in evironmental variable: {:#?}",
                    chars
                )
            } //make FMV_PATH a var
        },
    }
}

pub fn get_installed_versions() -> Vec<String> {
    let filesystem = std::fs::read_dir(get_installation_path()).unwrap();
    let mut res = Vec::new();
    for elem in filesystem {
        let e = elem.unwrap();
        if e.path().is_dir() && e.file_name().to_str().unwrap() != "smlink" {
            res.push(e.file_name().to_str().unwrap().to_string());
        }
    }
    res
}

pub fn delete_version(dir_name: &String) {
    let mut root = get_installation_path();
    root.push(dir_name);
    match std::fs::remove_dir_all(root.as_path()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error trying to delete version error: {}", e)
        }
    }
}
