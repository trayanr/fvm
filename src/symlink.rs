use std::fs::File;

use crate::installation_path::get_installation_path;

#[cfg(target_family = "unix")]
pub fn create_symlink(dir_name: String) {
    let mut path = get_installation_path();
    let mut link_path = get_installation_path();
    path.push(dir_name);
    link_path.push("smlink");
    match std::os::unix::fs::symlink(path.to_str().unwrap(), link_path.to_str().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            println!("Error creating symlink: {}", &err);
        }
    }
}

#[cfg(target_family = "windows")]
pub fn create_symlink(dir_name: String) {
    let mut path = get_installation_path();
    let mut link_path = get_installation_path();
    path.push(dir_name);
    link_path.push("smlink");
    match std::os::windows::fs::symlink_dir(path.to_str().unwrap(), link_path.to_str().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            println!("Error creating symlink: {}", &err);
        }
    }
}
#[cfg(target_family = "unix")]
pub fn delete_symlink() {
    let mut link_path = get_installation_path();
    link_path.push("smlink");

    match std::fs::remove_file(link_path.to_str().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            println!("Error deleting symlink: {}", err);
        }
    }
}

#[cfg(target_family = "windows")]
pub fn delete_symlink() {
    let mut link_path = get_installation_path();
    link_path.push("smlink");

    match std::fs::remove_dir(link_path.to_str().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            println!("Error deleting symlink: {}", err);
        }
    }
}

pub fn smlink_exist() -> bool {
    let mut link_path = get_installation_path();
    link_path.push("smlink");
    match std::fs::read_link(link_path.to_str().unwrap()) {
        Ok(fp) => true,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => false,
            std::io::ErrorKind::InvalidInput => {
                println!("There is a file or folder called the FVM_PATH/smlink, please remove it");
                return false;
            }
            _ => {
                println!("Error trying to read FVM_PATH/smlink: {:?}", e.kind());
                false
            }
        },
    }
}

pub fn is_target_of_symlink(dir_name: String) -> bool {
    let mut link_path = get_installation_path();
    link_path.push("smlink");
    match std::fs::read_link(link_path.to_str().unwrap()) {
        Ok(fp) => fp.to_str().unwrap().contains(&dir_name),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => false,
            std::io::ErrorKind::InvalidInput => {
                println!("There is a file or folder called the FVM_PATH/smlink, please remove it");
                return false;
            }
            _ => {
                println!("Error trying to read FVM_PATH/smlink: {:?}", e.kind());
                false
            }
        },
    }
}
