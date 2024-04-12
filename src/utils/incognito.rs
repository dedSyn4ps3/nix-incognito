use crate::utils::gsettings;
use std::process::Command;
use std::collections::HashMap;
use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


#[derive(Debug, PartialEq, Eq, Hash)]
struct GSetting<'a>{
    key: &'a str,
    field: &'a str,
}

type Settings<'a> = HashMap<GSetting<'a>, String>;

pub fn save_current_system() {
    let output = Command::new("dconf")
        .arg("dump")
        .arg("/")
        .output()
        .expect("Failed to get data");

    if output.status.success() {
        println!();
        print!("{} {} {}\n\n", "========================".yellow(), "Saving Current System Config".magenta().bold(), "=======================".yellow());
        //println!("{:?}", String::from_utf8_lossy(&output.stdout))
    } else {
        println!("Error retrieving configuration dump")
    }

    let path = Path::new("current_system_config.txt");
    let display = path.display();

    // Open file in write-only mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `output` string to `file`
    match file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("✅ {} {}", "Successfully wrote to file:".bold(), display.to_string().cyan().bold()),
    }
}

pub fn backup_key_values() {

    let mut current_value_map: Settings = HashMap::from([
        (GSetting{key: "org.gnome.desktop.background", field: "picture-uri"}, gsettings::get("org.gnome.desktop.background", "picture-uri")),
        (GSetting{key: "org.gnome.desktop.background", field: "picture-uri-dark"}, gsettings::get("org.gnome.desktop.background", "picture-uri-dark")),
        (GSetting{key: "org.gnome.desktop.background", field: "picture-options"}, gsettings::get("org.gnome.desktop.background", "picture-options")),
        (GSetting{key: "org.gnome.shell.extensions.user-theme", field: "name"}, gsettings::get("org.gnome.shell.extensions.user-theme", "name"))
        ]);
    
    print!("{} {} {}\n\n", "========================".yellow(), "Key Hash Map Config Values".magenta().bold(), "=======================".yellow());
    for (map, value) in current_value_map.iter_mut() {
        print!("Current Setting {}: {} \nValue: {}\n\n", map.key, map.field, value); 
    }
    
    //println!("{:?}", current_value_map);
}

#[allow(dead_code)]
pub fn load_previous_config() {
    //! Load the previous configuration from the dump file
    //! and updates the system
    let output = Command::new("sh")
        .arg("-c")
        .arg("dconf load / < current_system_config.txt")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        print!("{} {} {}\n\n", "========================".yellow(), "Loading Previous Config".magenta().bold(), "=======================".yellow());
        //println!("{:?}", String::from_utf8_lossy(&output.stdout))
    } else {
        println!("Error loading previous configuration dump")
    }
}

#[allow(dead_code)]
pub fn enable_incognito() {
    //! Define the keys and fields to be set in incognito mode
    //! and set them to the desired values
    let incognito_value_map: Settings = HashMap::from([
        (GSetting{key: "org.gnome.desktop.background", field: "picture-uri"}, String::from("file:///usr/share/backgrounds/gnome/adwaita-day.jpg").trim_end_matches('\n').to_string()),
        (GSetting{key: "org.gnome.desktop.background", field: "picture-uri-dark"}, String::from("file:///usr/share/backgrounds/gnome/adwaita-day.jpg")),
        (GSetting{key: "org.gnome.desktop.background", field: "picture-options"}, String::from("stretched")),
        (GSetting{key: "org.gnome.shell.extensions.user-theme", field: "name"}, String::from("Orchis-Dark"))
    ]);

    print!("{} {} {}\n\n", "========================".yellow(), "Engaging Nix Incognito".green().bold(), "=======================".yellow());

    for (map, value) in incognito_value_map.iter() {
        println!("Setting {}: {}\nValue: {}\n\n", map.key, map.field, value); 
    }
    

    
    // Set the keys and fields to the desired values
    // gsettings::set("org.gnome.desktop.background", "picture-uri", "file:///usr/share/backgrounds/gnome/adwaita-day.jpg");
}