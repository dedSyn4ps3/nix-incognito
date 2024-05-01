//! # Incognito Mode Utility Functions
//! This module contains utility functions for managing system configurations in incognito mode.
//! It provides functions to save the current system configuration, load a previous system configuration,
//! and enable incognito mode by setting specific keys and fields to desired values.
//!
//! The `save_current_system` function saves the current system configuration by using the `dconf dump` command
//! and writes the output to a specified file. It can be run in silent mode to suppress console output.
//!
//! The `load_previous_system` function loads a previous system configuration from a specified file
//! using the `dconf load` command. If the file does not exist, it displays an error message.
//!
//! The `enable_incognito` function enables incognito mode by setting specific keys and fields to desired values.
//! It takes parameters for the wallpaper, theme, icons, and a silent flag to suppress console output.
//!
//! The module also defines a `GSetting` struct to represent a key and field pair, and a `Settings` type
//! which is a HashMap of `GSetting` keys to string values.
//!
//! Note: The code contains commented out code for the `backup_key_values` function, which is not currently implemented.
//! It is intended to backup the current system configuration values to a file.

use crate::utils::gsettings;
use std::process::Command;
use std::collections::HashMap;
use colored::Colorize;
use indicatif::{ ProgressBar, ProgressStyle };
use std::fs;
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash)]
struct GSetting<'a> {
    key: &'a str,
    field: &'a str,
}

type Settings<'a> = HashMap<GSetting<'a>, String>;

/// Save the current system configuration to a file
pub fn save_current_system(silent: bool, config: String) {
    let output = Command::new("dconf").arg("dump").arg("/").output().expect("Failed to dump data");

    match check_file_exists(&config) {
        true => {
            if !silent {
                if output.status.success() {
                    //println!();
                    //print!("   💾 {}  ", "Saving Current System Config".magenta().bold());

                    let pb = ProgressBar::new_spinner();
                    pb.enable_steady_tick(Duration::from_millis(120));
                    pb.set_style(
                        ProgressStyle::with_template("{msg} {spinner:.green} ")
                            .unwrap()
                            // For more spinners check out the cli-spinners project:
                            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
                    );
                    pb.set_message(
                        format!(
                            "\n           💾 {} ",
                            "Saving Current System Config".magenta().bold()
                        )
                    );
                    sleep(Duration::from_secs(3));

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
                            println!("\n\n           ✅ {}", "Successfully wrote config!".bold()),
                    }
                } else {
                    println!("\n           ❗ {}", "Error dumping system config".red().bold());
                }
            } else {
                //println!("Running in silent mode...");
                let path = Path::new(&config);
                let display = path.display();

                // Open file in write-only mode
                let mut file = match File::create(&path) {
                    Err(why) =>
                        panic!(
                            "\n\n{}",
                            format!(
                                "🚨 {} {}{} {}",
                                "Couldn't create".bold().red(),
                                display,
                                ":".bold().red(),
                                why
                            )
                        ),
                    Ok(file) => file,
                };

                // Write the `output` string to `file`
                match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                    Err(why) =>
                        panic!(
                            "\n\n{}",
                            format!(
                                "🚨 {} {}{} {}",
                                "Couldn't write to".bold().red(),
                                display,
                                ":".bold().red(),
                                why
                            )
                        ),
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
                    fs::create_dir_all(&dir).unwrap_or_else(|why|
                        println!("\n           Failed to create! -> {:?}", why.kind())
                    );
                }
            }

            match path.exists() {
                true => (),
                false => {
                    touch(&path).unwrap_or_else(|why| {
                        println!("\n           ❗{:?}", why.kind());
                    });
                }
            }

            if !silent {
                if output.status.success() {
                    //println!();
                    //print!("   💾 {}  ", "Saving Current System Config".magenta().bold());

                    let pb = ProgressBar::new_spinner();
                    pb.enable_steady_tick(Duration::from_millis(120));
                    pb.set_style(
                        ProgressStyle::with_template("{msg} {spinner:.green} ")
                            .unwrap()
                            // For more spinners check out the cli-spinners project:
                            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
                    );
                    pb.set_message(
                        format!(
                            "\n           💾 {} ",
                            "Saving Current System Config".magenta().bold()
                        )
                    );
                    sleep(Duration::from_secs(3));

                    let path = Path::new(&config);
                    let display = path.display();

                    // Open file in write-only mode
                    let mut file = match File::create(&path) {
                        Err(why) => panic!("\nCouldn't create {}: {}", display, why),
                        Ok(file) => file,
                    };

                    // Write the `output` string to `file`
                    match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                        Err(why) => panic!("\nCouldn't write to {}: {}", display, why),
                        Ok(_) =>
                            println!("\n\n           ✅ {}", "Successfully wrote config!".bold()),
                    }
                } else {
                    println!("\n\n           ❗ {}", "Error dumping system config".red().bold());
                }
            } else {
                //println!("Running in silent mode...");
                let path = Path::new(&config);
                let display = path.display();

                // Open file in write-only mode
                let mut file = match File::create(&path) {
                    Err(why) =>
                        panic!(
                            "\n\n{}",
                            format!(
                                "🚨 {} {}{} {}",
                                "Couldn't create".bold().red(),
                                display,
                                ":".bold().red(),
                                why
                            )
                        ),
                    Ok(file) => file,
                };

                // Write the `output` string to `file`
                match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
                    Err(why) =>
                        panic!(
                            "\n\n{}",
                            format!(
                                "🚨 {} {}{} {}",
                                "Couldn't write to".bold().red(),
                                display,
                                ":".bold().red(),
                                why
                            )
                        ),
                    Ok(_) => (),
                }
            }
        }
    }
}

// pub fn backup_key_values(silent: bool) {
//     let mut current_value_map: Settings = HashMap::from([
//         (
//             GSetting { key: "org.gnome.desktop.background", field: "picture-uri" },
//             gsettings::get("org.gnome.desktop.background", "picture-uri"),
//         ),
//         (
//             GSetting { key: "org.gnome.desktop.background", field: "picture-uri-dark" },
//             gsettings::get("org.gnome.desktop.background", "picture-uri-dark"),
//         ),
//         (
//             GSetting { key: "org.gnome.desktop.background", field: "picture-options" },
//             gsettings::get("org.gnome.desktop.background", "picture-options"),
//         ),
//         (
//             GSetting { key: "org.gnome.shell.extensions.user-theme", field: "name" },
//             gsettings::get("org.gnome.shell.extensions.user-theme", "name"),
//         ),
//         (
//             GSetting { key: "org.gnome.desktop.interface", field: "icon-theme" },
//             gsettings::get("org.gnome.desktop.interface", "icon-theme"),
//         ),
//     ]);

//     if !silent {
//         print!(
//             "{}\n{}\n{}\n\n",
//             "==========================".yellow(),
//             " Backing Up Config Values ".magenta().bold(),
//             "==========================".yellow()
//         );
//     }

//     //////////////////////////////////////////////////////////////////////////////////////////////////////
//     // TODO: Write current settings to a file in $HOME called ~/.config/incognito/before_incognito.json //
//     //////////////////////////////////////////////////////////////////////////////////////////////////////

//     for (map, value) in current_value_map.iter_mut() {
//         print!("Current Setting {}: {} \nValue: {}\n\n", map.key, map.field, value);
//     }
// }

/// Load a previous system configuration from a file
pub fn load_previous_system(file: String) {
    match check_file_exists(&file) {
        true => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("dconf load / < {}", &file))
                .output()
                .expect("Failed to execute command");

            //print!("🛠️ {}  ", "Loading Previous Config".magenta().bold());
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(120));
            pb.set_style(
                ProgressStyle::with_template("{msg} {spinner:.green} ")
                    .unwrap()
                    // For more spinners check out the cli-spinners project:
                    // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                    .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            );
            pb.set_message(
                format!("\n            🛠️ {} ", "Loading Previous Config".cyan().bold())
            );
            sleep(Duration::from_secs(3));

            if output.status.success() {
                print!("\n\n            ✅ {}\n\n", "DONE! ".bold());
            } else {
                println!(
                    "\n            ❗{}\n\n",
                    "Error loading previous configuration file".red().bold()
                )
            }
        }
        false => {
            println!(
                "\n\n            👀 {}{}\n\n",
                "Config file not found: ".red().bold(),
                "Enable Incognito to generate it!".bold()
            );
        }
    }
}

/// Enable incognito mode by setting specific keys and fields to desired values
/// ## Args:
///     * wallpaper: String - The path to the wallpaper image file
///     * theme: String - The name of the theme to use
///     * icons: String - The name of the icon theme to use
///     * silent: bool - Flag to suppress console output
/// ## Example:
///    ```
///     enable_incognito("/path/to/wallpaper.jpg", "Fluent-Round-Dark", "Windows-Eleven", false);
///    ```
/// ## Note:
///   This function sets the following keys and fields in incognito mode:
///     - `org.gnome.desktop.background`: `picture-uri`, `picture-uri-dark`, `picture-options`
///     - `org.gnome.shell.extensions.user-theme`: `name`
///     - `org.gnome.shell.extensions.dash-to-dock`: `extend-height`
///     - `org.gnome.desktop.interface`: `icon-theme`, `gtk-theme`
///     - `org.gnome.desktop.wm.preferences`: `theme`
pub fn enable_incognito(wallpaper: String, theme: String, icons: String, silent: bool) {
    //! Define the keys and fields to be set in incognito mode
    //! and set them to the desired values
    let gsetting_value_map: Settings = HashMap::from([
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
        (
            GSetting { key: "org.gnome.desktop.interface", field: "gtk-theme" },
            String::from(format!("{}", &theme)),
        ),
        (
            GSetting { key: "org.gnome.desktop.wm.preferences", field: "theme" },
            String::from(format!("{}", &theme)),
        ),
    ]);

    if !silent {
        println!("           🥷 {}  ", "Engaging Nix Incognito...".bold());
        println!();

        // Set the given key and field to the provided value
        for (map, value) in gsetting_value_map.iter() {
            match gsettings::set(map.key, map.field, value) {
                Ok(s) => println!("           ✅ {}", s.bold().cyan()),
                Err(e) => println!("           🚨 {}", e.bold().red()),
            };
        }

        match
            gsettings::set_dconf("/org/gnome/shell/extensions/dash-to-dock/extend-height", "true")
        {
            Ok(s) => println!("           ✅ {}", s.bold().cyan()),
            Err(e) => println!("           🚨 {}", e.bold().red()),
        };
    } else {
        // Run without printing result to stdout
        for (map, value) in gsetting_value_map.iter() {
            let _res = gsettings::set(map.key, map.field, value);
        }
        let _res = gsettings::set_dconf(
            "org/gnome/shell/extensions/dash-to-dock/extend-height",
            "true"
        );
    }
}

/// Check if the file exists
fn check_file_exists(file: &str) -> bool {
    Path::new(file).exists()
}

/// Create a file if it doesn't exist
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
