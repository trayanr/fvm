use std::env;

use alias::{Alias, AliasFile};
// use doctor::smlink_version;
use download_release::download_progress_bar;
use installation_path::{delete_version, get_installed_versions};
use releases::{get_list_of_versions, Channel, Release};
use symlink::{delete_symlink, is_target_of_symlink};

mod alias;
mod doctor;
mod download_release;
mod installation_path;
mod progress_bar;
mod releases;
mod symlink;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (_, commands) = args.split_first().unwrap();

    let (first, other) = commands.split_first().unwrap(); //match
    match first.as_str() {
        "list" => list_handler(other.to_vec()),
        "download" => donwload_handler(other.to_vec()),
        "alias" => alias_handler(other.to_vec()),
        "select" => select_handler(other.to_vec()),
        "remove" => remove_handler(other.to_vec()),
        "show" => show_handler(other.to_vec()),
        "doctor" => doctor_handler(other.to_vec()),
        _ => println!("Couldn't find command [{}]", first),
    }
}

// doctor - show installed versions and alliases
// list [channel] (empty for all ?) - err check
// download - err check
// allias - maybe after download
// remove - by allias, by name
// select by allias or by version // maybe download too? empty show downloaded

// store alliases in a file in isntallation path
// create symlink in installation path when doctor is ran
//have separete function for flutter doctor that may return error, call it before every other

fn list_handler(commands: Vec<String>) {
    if commands.len() == 0 {
        println!("There are not enough arguments [stable/dev/beta] needed");
        return;
    }

    if commands.len() > 1 {
        println!("Too many arguments");
        return;
    }
    let channel = &commands[0]; //check if not empty
    match channel.as_str() {
        "stable" => list_releases(Channel::Stable),
        "dev" => list_releases(Channel::Dev),
        "beta" => list_releases(Channel::Beta),
        _ => {} // help for list
    }
}

fn list_releases(channel: Channel) {
    let future = get_list_of_versions();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(future);
    match res {
        Ok(rels) => {
            for r in rels {
                if r.channel != channel {
                    continue;
                }
                println!("{}", r) // maybe split in columns
            }
        }
        Err(e) => {} //??
    }
}

fn donwload_handler(commands: Vec<String>) {
    if commands.len() == 0 {
        println!("There are not enough arguments [version] needed");
        return;
    }

    if commands.len() > 1 {
        println!("Too many arguments");
        return;
    }

    let future = get_list_of_versions();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(future);
    match res {
        Ok(rels) => {
            let filtered: Vec<Release> = rels
                .to_vec()
                .iter()
                .cloned()
                .filter(|r| format!("{}", r).contains(&commands[0]))
                .collect();

            if filtered.len() == 1 {
                download_progress_bar(&filtered[0]);
            } else {
                println!("There are many other versions that meet the one given:");
                for r in filtered {
                    println!("{}", r);
                }
            }
        }
        Err(e) => {}
    }
}
// split on add, remove, show?
fn alias_handler(commands: Vec<String>) {
    if commands.len() == 0 {
        println!("Please choose one of the commands add, remove, show");
        return;
    }
    let (_, other) = commands.split_first().unwrap();
    match commands[0].as_str() {
        "add" => add_alias(other.to_vec()),
        "remove" => remove_alias(other.to_vec()),
        "show" => show_aliases(),
        _ => println!(
            "Please choose one of the commands add, remove, show. Command given: {}",
            commands[0]
        ),
    }
}

fn add_alias(commands: Vec<String>) {
    if commands.len() == 0 || commands.len() == 1 {
        println!("missing [version] [alias]");
        return;
    }

    if commands.len() > 2 {
        println!("too many arguments only two needed: [version] [alias]");
        return;
    }

    let versions = get_installed_versions();
    let version_cmd = commands[0].clone();
    if !versions.iter().any(|r| r == &version_cmd) {
        println!("no such version could be found: {}", version_cmd);
        let matches: Vec<String> = versions
            .to_vec()
            .iter()
            .cloned()
            .filter(|v| v.contains(&commands[0]))
            .collect();
        if matches.len() != 0 {
            println!("you might have meant:");
            for m in matches {
                println!("{}", m);
            }
        }
        // maybe download
        return;
    }

    let mut af = AliasFile::open();
    let aliases = af.get();
    if aliases.iter().cloned().any(|a| a.dir_name == commands[0]) {
        println!("this version already has an alias. If you want to change it, please remove the old one and create another");
        return;
    }
    af.push(Alias::new(commands[0].to_string(), commands[1].to_string()));
    af.save();
}

fn remove_alias(commands: Vec<String>) {
    if commands.len() == 0 {
        println!("Pleas provide an alias to remove. This wiil not remove downloaded version just the alias to it");
        return;
    }

    if commands.len() != 1 {
        println!("Too many arguments. Only [alias name] needed");
        return;
    }
    let mut af = AliasFile::open();
    let aliases = af.get();
    if !aliases.iter().cloned().any(|a| a.name == commands[0]) {
        println!("Couldn't find this alias: {}", commands[0]);
        let matches: Vec<Alias> = aliases
            .iter()
            .cloned()
            .filter(|a| a.name.contains(&commands[0]) || commands[0].contains(&a.name))
            .collect();
        if matches.len() != 0 {
            println!("you might have meant: ");
            for m in matches {
                println!("{}", m);
            }
        }
        return;
    }
    af.remove_by_alias(commands[0].clone());
}
// TODO: add too many arguments
fn show_aliases() {
    let af = AliasFile::open();
    let aliases = af.get();
    for a in aliases {
        println!("{}", a);
    }
}

fn select_handler(commands: Vec<String>) {
    if commands.len() == 0 {
        println!("Please select a version or alias");
    }

    if commands.len() > 1 {
        println!("Too many arguments provided");
        return;
    }

    let installed_version = get_installed_versions();
    let af = AliasFile::open();
    let aliases = af.get();
    if !installed_version.contains(&commands[0]) && !aliases.iter().any(|a| a.name == commands[0]) {
        println!("couldn't find this version or alias: {}", commands[0]);
        let mut matched: Vec<String> = installed_version
            .iter()
            .cloned()
            .filter(|s| s.contains(&commands[0]) || commands[0].contains(s))
            .collect();
        matched.extend(
            aliases
                .iter()
                .cloned()
                .filter(|a| a.name.contains(&commands[0]) || commands[0].contains(&a.name))
                .map(|a| a.name),
        );
        if matched.len() != 0 {
            println!("you might have meant:");
            for m in matched {
                println!("{}", m);
            }
        }
    }

    if installed_version.contains(&commands[0]) {
        if symlink::smlink_exist() {
            symlink::delete_symlink();
        }
        symlink::create_symlink(commands[0].clone());
    } else {
        let al: Vec<String> = aliases
            .iter()
            .cloned()
            .filter(|a| a.name == commands[0])
            .map(|a| a.dir_name)
            .collect();
        println!("{}", symlink::smlink_exist());
        if symlink::smlink_exist() {
            symlink::delete_symlink();
        }
        symlink::create_symlink(al[0].clone());
    }
    println!("{} has been selected", &commands[0]);
}

//should remove alias and folder, remove it from the path and replace it with something else
fn remove_handler(commands: Vec<String>) {
    if commands.len() == 0 {
        println!("Please provide a version or an alias");
        return;
    }

    if commands.len() > 1 {
        println!("Too many arguments provided");
        return;
    }

    let installed_version = get_installed_versions();
    let mut af = AliasFile::open();
    let aliases = af.get();
    if !installed_version
        .iter()
        .any(|v| v.contains(&commands[0].clone()) || commands.clone().contains(v))
        && !aliases.iter().any(|a| a.name == commands[0])
    {
        println!("Couldn't find version or alias: {}", commands[0]);
        let mut matched: Vec<String> = installed_version
            .iter()
            .cloned()
            .filter(|s| s.contains(&commands[0]) || commands[0].contains(s))
            .collect();
        matched.extend(
            aliases
                .iter()
                .cloned()
                .filter(|a| a.name.contains(&commands[0]) || commands[0].contains(&a.name))
                .map(|a| a.name),
        );
        if matched.len() != 0 {
            println!("you might have meant:");
            for m in matched {
                println!("{}", m);
            }
        }
        return;
    }

    if installed_version.contains(&commands[0]) {
        let version = &commands[0];
        delete_version(version);
        delete_symlink();
        af.remove_by_version(version);
    } else {
        let al: Vec<String> = aliases
            .iter()
            .cloned()
            .filter(|a| a.name == commands[0])
            .map(|a| a.dir_name)
            .collect();
        // &al[0]
        let version = &al[0];
        delete_version(version);
        delete_symlink();
        af.remove_by_version(version);
    };

    println!("{} has been deleted", commands[0])
}

fn show_handler(commands: Vec<String>) {
    if commands.len() != 0 {
        println!("Too many arguments");
        return;
    }
    let installed_version = get_installed_versions();
    for v in installed_version {
        println!("{}", v);
    }
}

fn doctor_handler(commands: Vec<String>) {
    if commands.len() != 0 {
        println!("Too many arguments");
        return;
    }
    //check if path has the path to the symlink + /flutter/bin
    // match doctor::is_flutter_path_correct() {
    //     Some(b) => {
    //         if !b {
    //             println!(
    //                 "Please remove the current path to flutter and put FVM_PATH/smlink/flutter/bin"
    //             );
    //         }
    //     }
    //     None => println!("Please seth the flutter path to FVM_PATH/smlink/flutter/bin"),
    // }
    //check if symlink points to actual version of flutter

    // smlink_version();
    //check if FVM_PATH is set
    //check if all versions of flutter downloaded correspond to the folder name and change accordingly
    //maybe check for update, and ask to change default alias latest
}
