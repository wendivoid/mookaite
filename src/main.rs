#[macro_use]
extern crate clap;
extern crate mookaite;

use clap::{ App, Arg, AppSettings };
use std::path::PathBuf;
use mookaite::run_mookaite;

fn main() {
    let matches = App::new("mookaite")
                        .usage("mookaite [FLAGS] [OPTIONS] --args [ARGS]")
                        .author("Bytebuddha <shadowcynical@gmail.com>")
                        .version(crate_version!())
                        .about("A utility for randomaly changing desktop background based on
virtual desktops.")
                        .arg(Arg::with_name("image_dir")
                                .short("d")
                                .long("image-directory")
                                .value_name("IMG_DIR")
                                .help("The directory to select random images from,
if no directory is given /home/$USER/Pictures is used.")
                                .takes_value(true)

                        )
                        .arg(Arg::with_name("background-command")
                                    .short("c")
                                    .long("background-command")
                                    .value_name("SET_COMMAND")
                                    .help("Specify the background set command to use, default is /usr/bin/feh.")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("timeout")
                                    .short("t")
                                    .long("timeout")
                                    .value_name("TIMEOUT")
                                    .help("How long in secs to wait before randomaly changing all desktop backgrounds.
If not given 900(15mins) is used.")
                                    .takes_value(true)
                        )
                        .setting(AppSettings::TrailingVarArg)
                        .setting(AppSettings::AllowLeadingHyphen)
                        .arg(Arg::with_name("other-args")
                                .short("a")
                                .allow_hyphen_values(true)
                                .long("args")
                                .value_name("FEH_ARGS")
                                .multiple(true)
                                .help("Provide optional arguments to pass to feh each time it is run.")
                                .takes_value(true)
                    ).get_matches();

    let timeout = matches.value_of("timeout").unwrap_or("900").parse::<u32>().expect("Timeout was not an integer");

    let image_directory = match matches.value_of("image_dir"){
        Some(d) => PathBuf::from(d),
        None => {
            let mut p = PathBuf::from("/home");
            p.push(std::env::var("USER").expect("Unable to find current user and no directory set!"));
            p.push("Pictures");
            p
        }
    };

    let feh_args = match matches.value_of("other-args") {
        Some(d) => {
            d.split(" ").collect::<Vec<&str>>()
        },
        None => vec!["--bg-scale"]
    };

    let cmd = matches.value_of("background-command").unwrap_or("/usr/bin/feh");

    run_mookaite(image_directory, timeout, cmd, feh_args);
}
