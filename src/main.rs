mod utils;

use clap::Parser;
use colored::Colorize;
use utils::{ incognito, cli };

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
Ever find yourself on an engagement where you need to blend in with the environment 
of other workstations around you? Perhaps you're just out to have some fun and don't want 
to attract attention to yourself by having a flashy desktop for others to see...

<bold>This tool will help you do just that!</bold>
    "#
);

const AFTER_HELP: &'static str = color_print::cstr!(
    r#"<bold><blue>Examples:</blue></bold>
<dim>$</dim> <bold><green>nix-incognito</green> <yellow>--silent</yellow></bold>                      <dim># Enable with no output</dim>
<dim>$</dim> <bold><green>nix-incognito</green> <yellow>-s -w</yellow> /path/to/wallpaper.jpg</bold>  <dim># Enable with no output using custom wallpaper</dim>
        "#
);

#[derive(Parser, Debug)]
#[command(version, about = OVERVIEW, after_help = AFTER_HELP, styles = cli::get_styles())]
struct Args {
    /// Run without any output
    #[arg(short, long, default_value_t = false)]
    silent: bool,

    /// Custom wallpaper path
    #[arg(short, long, default_value_t = String::from("/run/system/backgrounds/incognito/windows.jpg"))]
    wallpaper: String,
}

fn main() {
    let _args = Args::parse();

    //incognito::backup_key_values();
    //incognito::save_current_system();

}