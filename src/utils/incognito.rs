use crate::utils::gsettings;
use std::process::Command;
use std::collections::HashMap;
use colored::Colorize;
use std::fs;
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Hash)]
struct GSetting<'a> {
    key: &'a str,
    field: &'a str,
}

type Settings<'a> = HashMap<GSetting<'a>, String>;

pub fn save_current_system(silent: bool, config: String) {
    let output = Command::new("dconf").arg("dump").arg("/").output().expect("Failed to dump data");

    match check_file_exists(&config) {
        true => {
            if !silent {
                if output.status.success() {
                    println!();
                    print!(
                        "{}\n{}\n{}\n\n",
                        "==============================".yellow(),
                        " Saving Current System Config ".magenta().bold(),
                        "==============================".yellow()
                    );

                    let path = Path::new(&config);
                    let display = path.display();

                    // Open file in write-only mode
                    let mut file = match File::create(&path) {
                        Err(why) => panic!("Couldn't create {}: {}", display, why),
                        Ok(file) => file,
                    };

                    // Write the `output` string to `file`
                    match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                        Err(why) => panic!("Couldn't write to {}: {}", display, why),
                        Ok(_) =>
                            println!(
                                "✅ {} {}",
                                "Successfully wrote to file:".bold(),
                                display.to_string().cyan().bold()
                            ),
                    }
                } else {
                    println!("❗ {}", "Error dumping system config".red().bold());
                }
            } else {
                //println!("Running in silent mode...");
                let path = Path::new(&config);
                let display = path.display();

                // Open file in write-only mode
                let mut file = match File::create(&path) {
                    Err(why) => panic!("Couldn't create {}: {}", display, why),
                    Ok(file) => file,
                };

                // Write the `output` string to `file`
                match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                    Err(why) => panic!("Couldn't write to {}: {}", display, why),
                    Ok(_) => (),
                }
            }
        }
        false => {
            let path = PathBuf::from(&config);
            let dir = path.parent().unwrap();

            match dir.exists() {
                true => (),
                false => {
                    fs::create_dir_all(&dir)
                        .unwrap_or_else(|why| println!("Failed to create! -> {:?}", why.kind()));
                }
            };

            match path.exists() {
                true => (),
                false => {
                    touch(&path).unwrap_or_else(|why| {
                        println!("! {:?}", why.kind());
                    });
                }
            };

            if !silent {
                if output.status.success() {
                    println!();
                    print!(
                        "{}\n{}\n{}\n\n",
                        "==============================".yellow(),
                        " Saving Current System Config ".magenta().bold(),
                        "==============================".yellow()
                    );

                    let path = Path::new(&config);
                    let display = path.display();

                    // Open file in write-only mode
                    let mut file = match File::create(&path) {
                        Err(why) => panic!("Couldn't create {}: {}", display, why),
                        Ok(file) => file,
                    };

                    // Write the `output` string to `file`
                    match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                        Err(why) => panic!("Couldn't write to {}: {}", display, why),
                        Ok(_) =>
                            println!(
                                "✅ {} {}",
                                "Successfully wrote to file:".bold(),
                                display.to_string().cyan().bold()
                            ),
                    }
                } else {
                    println!("❗ {}", "Error dumping system config".red().bold());
                }
            } else {
                //println!("Running in silent mode...");
                let path = Path::new(&config);
                let display = path.display();

                // Open file in write-only mode
                let mut file = match File::create(&path) {
                    Err(why) => panic!("Couldn't create {}: {}", display, why),
                    Ok(file) => file,
                };

                // Write the `output` string to `file`
                match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                    Err(why) => panic!("Couldn't write to {}: {}", display, why),
                    Ok(_) => (),
                }
            }
        }
    }
}

pub fn backup_key_values(silent: bool) {
    let mut current_value_map: Settings = HashMap::from([
        (
            GSetting { key: "org.gnome.desktop.background", field: "picture-uri" },
            gsettings::get("org.gnome.desktop.background", "picture-uri"),
        ),
        (
            GSetting { key: "org.gnome.desktop.background", field: "picture-uri-dark" },
            gsettings::get("org.gnome.desktop.background", "picture-uri-dark"),
        ),
        (
            GSetting { key: "org.gnome.desktop.background", field: "picture-options" },
            gsettings::get("org.gnome.desktop.background", "picture-options"),
        ),
        (
            GSetting { key: "org.gnome.shell.extensions.user-theme", field: "name" },
            gsettings::get("org.gnome.shell.extensions.user-theme", "name"),
        ),
        (
            GSetting { key: "org.gnome.desktop.interface", field: "icon-theme" },
            gsettings::get("org.gnome.desktop.interface", "icon-theme"),
        ),
    ]);

    if !silent {
        print!(
            "{}\n{}\n{}\n\n",
            "==========================".yellow(),
            " Backing Up Config Values ".magenta().bold(),
            "==========================".yellow()
        );
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////////
    // TODO: Write current settings to a file in $HOME called ~/.config/incognito/before_incognito.json //
    //////////////////////////////////////////////////////////////////////////////////////////////////////

    for (map, value) in current_value_map.iter_mut() {
        print!("Current Setting {}: {} \nValue: {}\n\n", map.key, map.field, value);
    }
}

pub fn load_previous_system(file: String) {
    match check_file_exists(&file) {
        true => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("dconf load / < {}", &file))
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                print!(
                    "{}\n{}\n{}\n\n",
                    "========================".yellow(),
                    " Loaded Previous Config ".magenta().bold(),
                    "========================".yellow()
                );
            } else {
                println!("Error loading previous configuration file")
            }
        }
        false => {
            println!(
                "👀 {}{}",
                "Config file not found: ".red().bold(),
                "Enable Incognito to generate it!".bold()
            );
        }
    }
}

pub fn enable_incognito(wallpaper: String, theme: String, icons: String, silent: bool) {
    //! Define the keys and fields to be set in incognito mode
    //! and set them to the desired values
    let incognito_value_map: Settings = HashMap::from([
        (
            GSetting { key: "org.gnome.desktop.background", field: "picture-uri" },
            String::from(format!("file://{}", &wallpaper)),
        ),
        (
            GSetting { key: "org.gnome.desktop.background", field: "picture-uri-dark" },
            String::from(format!("file://{}", &wallpaper)),
        ),
        (
            GSetting { key: "org.gnome.desktop.background", field: "picture-options" },
            String::from("stretched"),
        ),
        (
            GSetting { key: "org.gnome.shell.extensions.user-theme", field: "name" },
            String::from(format!("{}", &theme)),
        ),
        (
            GSetting { key: "org.gnome.desktop.interface", field: "icon-theme" },
            String::from(format!("{}", &icons)),
        ),
    ]);

    if !silent {
        print!(
            "{}\n{}\n{}\n\n",
            "========================".yellow(),
            " Engaging Nix Incognito ".green().bold(),
            "========================".yellow()
        );

        // Set the given key and field to the provided value
        for (map, value) in incognito_value_map.iter() {
            // gsettings::set("org.gnome.desktop.background", "picture-uri", "file:///run/system/backgrounds/incognito/windows.jpg");
            //println!("Setting {}: {}\nValue: {}\n\n", map.key, map.field, value);
            match gsettings::set(map.key, map.field, value) {
                Ok(s) => println!("✅ {}", s.bold().cyan()),
                Err(e) => println!("🚨 {}", e.bold().red()),
            };
        }
    } else {
        // Run without printing result to stdout
        for (map, value) in incognito_value_map.iter() {
            // gsettings::set("org.gnome.desktop.background", "picture-uri", "file:///run/system/backgrounds/incognito/windows.jpg");
            //println!("Setting {}: {}\nValue: {}\n\n", map.key, map.field, value);
            let _res = gsettings::set(map.key, map.field, value);
        }
    }
}

/// Check if the file exists
fn check_file_exists(file: &str) -> bool {
    //! Check if the file exists
    Path::new(file).exists()
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
