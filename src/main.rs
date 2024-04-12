mod utils;

use colored::Colorize;
use utils::incognito;

fn main() {
    // println!("\n{}", "Current desktop background: ".cyan().bold());
    // match gsettings::get("org.gnome.desktop.background", "picture-uri") {
    //     Ok(res) => println!("{}\n", &res),
    //     Err(e) => println!("{}\n", &e),
    // };

    print!(
        "{} \n\n",
        "Running incognito functionality...".green().bold()
    );
    incognito::backup_key_values();
    incognito::save_current_system();
}
