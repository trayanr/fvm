use crate::{installation_path::get_installation_path, releases::Release};
use std::io::prelude::*;
use std::{fmt, fs::File, fs::OpenOptions, io::ErrorKind, io::Read, path::PathBuf};

pub struct AliasFile {
    aliases: Vec<Alias>,
}

impl AliasFile {
    pub fn open() -> AliasFile {
        let aliases = get_aliases_path();
        // println!("{}", aliases.to_str().unwrap());
        match File::open(aliases.to_str().unwrap()) {
            Ok(mut file) => {
                let mut string_file = String::new();
                file.read_to_string(&mut string_file).unwrap();
                let lines: Vec<&str> = string_file.split("\n").collect();
                let aliases = lines
                    .iter()
                    .cloned()
                    .filter(|l| l != &"")
                    .map(|l| Alias::parse(l.to_string()));
                AliasFile {
                    aliases: aliases.collect(),
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create(aliases.to_str().unwrap()) {
                    Ok(f) => AliasFile { aliases: vec![] },
                    Err(ec) => {
                        panic!("Couldn't create file - {}", ec)
                    }
                },
                _ => {
                    println!("Couldn't open file - {}", e);
                    AliasFile { aliases: vec![] }
                }
            },
        }
    }

    pub fn save(&self) {
        let contents = self
            .aliases
            .iter()
            .map(|a| format!("{}", a))
            .collect::<Vec<String>>()
            .join("\n");
        let aliases = get_aliases_path();
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(aliases.to_str().unwrap())
        {
            Ok(f) => {
                f.set_len(0).unwrap();
                let mut writer = std::io::BufWriter::new(f);
                writer.write_all(contents.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error saving aliases: {}", e);
            }
        }
    }

    pub fn push(&mut self, a: Alias) {
        self.aliases.push(a);
        self.save();
    }

    pub fn get(&self) -> Vec<Alias> {
        self.aliases.to_vec()
    }

    pub fn remove_by_alias(&mut self, name: String) {
        self.aliases = self
            .aliases
            .iter()
            .cloned()
            .filter(|a| a.name != name)
            .collect();
        self.save();
    }

    pub fn remove_by_version(&mut self, version: &String) {
        self.aliases = self
            .aliases
            .iter()
            .cloned()
            .filter(|a| &a.dir_name != version)
            .collect();
        self.save();
    }
}

fn get_aliases_path() -> PathBuf {
    let mut root = get_installation_path();
    root.push("aliases");
    root
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub dir_name: String,
    pub name: String,
}

impl Alias {
    pub fn parse(s: String) -> Alias {
        let elem: Vec<&str> = s.split(",").collect();
        Alias {
            dir_name: elem[0].to_string(),
            name: elem[1].to_string(),
        }
    }

    pub fn new(dir_name: String, name: String) -> Alias {
        Alias {
            dir_name: dir_name,
            name,
        }
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.dir_name, self.name)
    }
}
