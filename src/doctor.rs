use std::{env, process::Command};
use std::{
    fmt,
    path::{Path, PathBuf},
};

use crate::installation_path::get_installation_path;
use crate::releases::Channel;

enum DoctorError {
    PathNotFound(String),
}

impl fmt::Display for DoctorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DoctorError::PathNotFound(p) => write!(f, "Couldn't find this path: {}", p),
            DoctorError::PathNotFound(_) => {}
        }
    }
}

//change to errors
pub fn is_flutter_path_correct() -> Option<bool> {
    match find_it("flutter") {
        Some(path) => {
            let mut link_path = get_installation_path();
            link_path.push("smlink");
            link_path.push("flutter");
            link_path.push("bin");
            link_path.push("flutter");
            Some(path == link_path)
        }
        None => None,
    }
}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

pub fn smlink_version() {
    let mut link_path = get_installation_path();
    link_path.push("smlink");
    link_path.push("flutter");
    link_path.push("bin");
    link_path.push("flutter");
    get_flutter_version(&link_path.as_path());
}

// pub fn get_flutter_version(p: &Path) -> Result<(String, Channel)> {
//     let mut path_buf = PathBuf::from(p);
//     path_buf.push("flutter");
//     let output = Command::new(path_buf.to_str().unwrap())
//         .arg("--version")
//         .output()
//         .expect("Couldn't run flutter process to check the version");
//     let bytes = output.stdout;
//     println!("{:#?}", bytes);
// }
