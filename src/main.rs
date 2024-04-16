mod utils;

use clap::Parser;
use colored::Colorize;
use dirs::*;
use utils::{ incognito, cli };

const BANNER: &'static str = color_print::cstr!(
    r#"<bold><red>
_       _________                    _________ _        _______  _______  _______  _       __________________ _______ 
( (    /|\__   __/|\     /|           \__   __/( (    /|(  ____ \(  ___  )(  ____ \( (    /|\__   __/\__   __/(  ___  )
|  \  ( |   ) (   ( \   / )              ) (   |  \  ( || (    \/| (   ) || (    \/|  \  ( |   ) (      ) (   | (   ) |
|   \ | |   | |    \ (_) /    _____      | |   |   \ | || |      | |   | || |      |   \ | |   | |      | |   | |   | |
| (\ \) |   | |     ) _ (    (_____)     | |   | (\ \) || |      | |   | || | ____ | (\ \) |   | |      | |   | |   | |
| | \   |   | |    / ( ) \               | |   | | \   || |      | |   | || | \_  )| | \   |   | |      | |   | |   | |
| )  \  |___) (___( /   \ )           ___) (___| )  \  || (____/\| (___) || (___) || )  \  |___) (___   | |   | (___) |
|/    )_)\_______/|/     \|           \_______/|/    )_)(_______/(_______)(_______)|/    )_)\_______/   )_(   (_______)
</red>   
                  ============================================================================== 
                  ||  🪪  Created by: dedsyn4ps3       ✨ Inspiration from: Kali (of course!)  ||
                  ==============================================================================
    "#
);

const OVERVIEW: &'static str = color_print::cstr!(
    r#"<bold><red>
_       _________                    _________ _        _______  _______  _______  _       __________________ _______ 
( (    /|\__   __/|\     /|           \__   __/( (    /|(  ____ \(  ___  )(  ____ \( (    /|\__   __/\__   __/(  ___  )
|  \  ( |   ) (   ( \   / )              ) (   |  \  ( || (    \/| (   ) || (    \/|  \  ( |   ) (      ) (   | (   ) |
|   \ | |   | |    \ (_) /    _____      | |   |   \ | || |      | |   | || |      |   \ | |   | |      | |   | |   | |
| (\ \) |   | |     ) _ (    (_____)     | |   | (\ \) || |      | |   | || | ____ | (\ \) |   | |      | |   | |   | |
| | \   |   | |    / ( ) \               | |   | | \   || |      | |   | || | \_  )| | \   |   | |      | |   | |   | |
| )  \  |___) (___( /   \ )           ___) (___| )  \  || (____/\| (___) || (___) || )  \  |___) (___   | |   | (___) |
|/    )_)\_______/|/     \|           \_______/|/    )_)(_______/(_______)(_______)|/    )_)\_______/   )_(   (_______)
</red>   
                  ============================================================================== 
                  ||  🪪  Created by: dedsyn4ps3       ✨ Inspiration from: Kali (of course!)  ||
                  ==============================================================================

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

    /// Custom config path [Default: ~/.config/incognito/current_system_config.txt]
    #[arg(
        short,
        long,
        default_value_t = String::from(format!("{}/.config/incognito/current_system_config.txt", home_dir().unwrap().to_str().unwrap()))
    )]
    config: String,

    /// Custom wallpaper path
    #[arg(
        short,
        long,
        default_value_t = String::from("/run/current-system/sw/share/backgrounds/incognito/win11.jpg")
    )]
    wallpaper: String,

    /// User theme to implement
    #[arg(short, long, default_value_t = String::from("Win11OS-Dark"))]
    theme: String,

    /// Icon theme to implement
    #[arg(short, long, default_value_t = String::from("Win11-Dark"))]
    icons: String,

    /// Disable incognito and restore previous system settings
    #[arg(short, long, default_value_t = false)]
    restore: bool,
}

fn main() {
    let args = Args::parse();

    match args.restore {
        true => {
            match args.silent {
                true => {
                    incognito::load_previous_system(args.config);
                }
                false => {
                    println!();
                    println!("{}", BANNER);
                    println!();
                    println!("🗃️ {}", "Restoring previous system settings...".magenta().bold());
                    incognito::load_previous_system(args.config);
                }
            }
        }
        false => {
            match args.silent {
                true => {
                    //incognito::backup_key_values(true);
                    incognito::save_current_system(true, args.config);
                    incognito::enable_incognito(args.wallpaper, args.theme, args.icons, true);
                }
                false => {
                    println!();
                    println!("{}", BANNER);
                    print!("\n\n");
                    println!("💬 {}", "Running in Verbose Mode".cyan().bold());
                    //incognito::backup_key_values(false);
                    incognito::save_current_system(false, args.config);
                    incognito::enable_incognito(args.wallpaper, args.theme, args.icons, false);
                }
            }
        }
    }
}
