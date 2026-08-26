use std::{
    env::{Args, args},
    process::exit,
};

use larpfetch::{
    config::{config_file_location, make_config},
    *,
};

fn main() {
    // First check if correct OS!;
    let os = get_os();
    if &os == &TargetOS::MacOS {
        println!("Error MacOS is not supported!!!!!!!!");
        exit(1);
    } else if &os == &TargetOS::Unknown {
        println!("Error you are using unsuported OS!!!");
        exit(1);
    }
    let args = args();
    if !handle_args(args) {
        exit(0);
    }
    let config_file_path = config_file_location(&os).unwrap();
    let pre_config = Config::from_file(&config_file_path);
    if pre_config.is_err() {
        let error = pre_config.unwrap_err();
        error.print();
        exit(1);
    }
    let config = Config::from_file(&config_file_path).unwrap();
    let larp = Larp::new(config.icon, config.elements);
    larp.print();
}

pub fn handle_args(args: Args) -> bool {
    for arg in args.skip(1) {
        if arg == "--make_config" {
            let _ = make_config(&config_file_location(&get_os()).unwrap());
            return false;
        } else if arg == "-v" || arg == "--version" {
            println!("{LARPFETCH_VERSION}");
            return false;
        }
    }
    return true;
}
