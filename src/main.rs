//! # Overview
//! **A transformative tool designed for Penetration Testers, Red-Teamers, and general Linux enthusiasts who need to mask their tooling environment, or simply crave the familiarity of the Windows interface.**
//!
//! This project is your secret agent, seamlessly morphing your Linux desktop to mirror the look and feel of a typical Windows environment. It's perfect for those who want to blend in with the Windows crowd, while still benefitting from the power and flexibility of NixOS!
//!
//! **Experience the best of both worlds with Nix Incognito!**
//!
//! ![<https://gatlabs.com/blogpost/10-dangerous-incognito-myths-part-one/>](https://gatlabs.com/wp-content/uploads/2021/12/10-Dangerous-Incognito-Myths_-You-sure-Its-what-you-think-it-is_.jpg)
//!
//! The tool is implemented as a command-line application using the `clap` crate for argument parsing and the `colored` crate for colored output.
//! The main function parses arguments using the `Args` struct, which it then executes the appropriate actions based on the provided arguments, such as enabling incognito mode, restoring previous system settings, or displaying help information.
//!
//! The `BANNER`, `OVERVIEW`, and `AFTER_HELP` constants are used to display formatted text in the command-line interface.

mod utils;

use clap::Parser;
use colored::Colorize;
use dirs::*;
use std::thread::sleep;
use std::time::Duration;
use utils::{ incognito, cli };

const BANNER: &'static str = color_print::cstr!(
    r#"<bold><red>
    
    
    ███╗   ██╗██╗██╗  ██╗    ██╗███╗   ██╗ ██████╗ ██████╗  ██████╗ ███╗   ██╗██╗████████╗ ██████╗ 
    ████╗  ██║██║╚██╗██╔╝    ██║████╗  ██║██╔════╝██╔═══██╗██╔════╝ ████╗  ██║██║╚══██╔══╝██╔═══██╗
    ██╔██╗ ██║██║ ╚███╔╝     ██║██╔██╗ ██║██║     ██║   ██║██║  ███╗██╔██╗ ██║██║   ██║   ██║   ██║
    ██║╚██╗██║██║ ██╔██╗     ██║██║╚██╗██║██║     ██║   ██║██║   ██║██║╚██╗██║██║   ██║   ██║   ██║
    ██║ ╚████║██║██╔╝ ██╗    ██║██║ ╚████║╚██████╗╚██████╔╝╚██████╔╝██║ ╚████║██║   ██║   ╚██████╔╝
    ╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝    ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝   ╚═╝    ╚═════╝    
</red>     
          =============================================================================== 
          ||  🪪  Created by: dedsyn4ps3       ✨ Inspiration from: Kali (of course!)  ||
          ===============================================================================

    "#
);

const OVERVIEW: &'static str = color_print::cstr!(
    r#"<bold><red>


            ███╗   ██╗██╗██╗  ██╗    ██╗███╗   ██╗ ██████╗ ██████╗  ██████╗ ███╗   ██╗██╗████████╗ ██████╗ 
            ████╗  ██║██║╚██╗██╔╝    ██║████╗  ██║██╔════╝██╔═══██╗██╔════╝ ████╗  ██║██║╚══██╔══╝██╔═══██╗
            ██╔██╗ ██║██║ ╚███╔╝     ██║██╔██╗ ██║██║     ██║   ██║██║  ███╗██╔██╗ ██║██║   ██║   ██║   ██║
            ██║╚██╗██║██║ ██╔██╗     ██║██║╚██╗██║██║     ██║   ██║██║   ██║██║╚██╗██║██║   ██║   ██║   ██║
            ██║ ╚████║██║██╔╝ ██╗    ██║██║ ╚████║╚██████╗╚██████╔╝╚██████╔╝██║ ╚████║██║   ██║   ╚██████╔╝
            ╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝    ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝   ╚═╝    ╚═════╝   
</red>   
                   =============================================================================== 
                   ||  🪪  Created by: dedsyn4ps3       ✨ Inspiration from: Kali (of course!)  ||
                   ===============================================================================


</bold>
Ever find yourself on an engagement where you need to blend in with the environment of other workstations around you?
Perhaps you're just out to have some fun and don't want to attract attention to yourself by having a flashy desktop for 
others to see...

<bold>This tool will help you do just that!</bold>
    "#
);

const AFTER_HELP: &'static str = color_print::cstr!(
    r#"<bold><blue>Examples:</blue></bold>
<dim>$</dim> <bold><green>nix-incognito</green> <yellow>--silent --config</yellow> "path/to/use"</bold>             <dim># Enable with no output and custom config path</dim>
<dim>$</dim> <bold><green>nix-incognito</green> <yellow>-s -w</yellow> /path/to/wallpaper.jpg</bold>                <dim># Enable with no output using custom wallpaper</dim>
<dim>$</dim> <bold><green>nix-incognito</green> <yellow>-i</yellow> "Icon Pack" <yellow>-t</yellow> "Theme"</bold>  <dim># Enable using different theme and icons</dim>
        "#
);

#[derive(Parser, Debug)]
#[command(version, about = OVERVIEW, after_help = AFTER_HELP, styles = cli::get_styles())]
struct Args {
    /// Run without any output [Default: false]
    #[arg(short, long, default_value_t = false)]
    silent: bool,

    /// Custom config path
    #[arg(
        short,
        long,
        default_value_t = String::from(
            format!(
                "{}/.config/incognito/current_system_config.txt",
                home_dir().unwrap().to_str().unwrap()
            )
        )
    )]
    config: String,

    /// Custom wallpaper path
    #[arg(
        short,
        long,
        default_value_t = String::from(
            "/run/current-system/sw/share/backgrounds/incognito/win11.jpg"
        )
    )]
    wallpaper: String,

    /// User theme to implement
    #[arg(short, long, default_value_t = String::from("Fluent-Round-Dark"))]
    theme: String,

    /// Icon theme to implement
    #[arg(short, long, default_value_t = String::from("Windows-Eleven"))]
    icons: String,

    /// Disable incognito and restore previous system settings
    #[arg(short, long, default_value_t = false)]
    restore: bool,
}

/// ## Primary entry-point of the tool
///
/// This function is the entry point of the application. It parses the command-line arguments using the `Args` struct,
/// performs the necessary actions based on the provided arguments, and prints output to the console.
///
/// ## Example:
/// ```
/// let args = Args::parse();
///
/// match args.silent {
///     true => {
///         incognito::save_current_system(true, args.config);
///         incognito::enable_incognito(args.wallpaper, args.theme, args.icons, true);
///     }
///     false => {
///         println!();
///         println!("{}", BANNER);
///         print!("\n\n");
///         println!("💬 {}", "Running in Verbose Mode".cyan().bold());
///
///         incognito::save_current_system(false, args.config);
///         incognito::enable_incognito(args.wallpaper, args.theme, args.icons, false);
///     }
/// }
/// ```
///
fn main() {
    let args = Args::parse();

    match args.restore {
        true => {
            match args.silent {
                true => {
                    incognito::load_previous_system(args.config);
                }
                false => {
                    // Clear terminal screen
                    print!("{esc}c", esc = 27 as char);
                    println!();
                    println!("{}", BANNER);
                    println!();

                    println!(
                        "         ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
                        "Updating System".yellow().bold()
                    );
                    println!();
                    println!(
                        "            🗃️ {}",
                        "Restoring previous system settings...".magenta().bold()
                    );

                    sleep(Duration::from_secs(1));
                    incognito::load_previous_system(args.config);

                    println!(
                        "\n         ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
                    );
                }
            }
        }
        false => {
            match args.silent {
                true => {
                    incognito::save_current_system(true, args.config);
                    incognito::enable_incognito(args.wallpaper, args.theme, args.icons, true);
                }
                false => {
                    // Clear terminal screen
                    print!("{esc}c", esc = 27 as char);
                    println!();
                    println!("{}", BANNER);
                    println!();

                    println!(
                        "         ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
                        "Updating System".yellow().bold()
                    );

                    sleep(Duration::from_secs(1));
                    incognito::save_current_system(false, args.config);
                    incognito::enable_incognito(args.wallpaper, args.theme, args.icons, false);

                    println!(
                        "\n         ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
                    );
                }
            }
        }
    }
}
