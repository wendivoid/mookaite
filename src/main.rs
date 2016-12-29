extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stream;
extern crate mookaite;

use slog::DrainExt;
use clap::{ App, Arg, AppSettings };
use std::fs::OpenOptions;
use std::path::PathBuf;
use mookaite::run_mookaite;

fn main() {
    let matches = App::new("mookaite")
                        .usage("mookaite [FLAGS] [OPTIONS] --feh-args [FEH_ARGS]")
                        .author("Bytebuddha <shadowcynical@gmail.com>")
                        .version("1.0")
                        .about("A utility for randomaly changing desktop background based on
virtual desktops.")
                        .arg(Arg::with_name("mode")
                                    .short("m")
                                    .long("mode")
                                    .value_name("MODE")
                                    .help("The mode of changing desktops, if not given random is used")
                                    .takes_value(true)
                                    .possible_values(&["mapped", "random"])
                        )
                        .arg(Arg::with_name("log_level")
                                    .short("v")
                                    .long("verbose")
                                    .multiple(true)
                                    .help("Set the log level. 0 = Info, 1, Info, 2 = Debug, >3 = trace")
                        )
                        .arg(Arg::with_name("image_dir")
                                .short("d")
                                .long("image-directory")
                                .value_name("IMG_DIR")
                                .help("The directory to select random images from,
if no directory is given /home/$USER/Pictures is used.")
                                .takes_value(true)

                        )
                        .arg(Arg::with_name("reload_time")
                                    .short("r")
                                    .long("reload")
                                    .value_name("RELOAD_TIME")
                                    .help("The time in secs to wait before searching for new files in IMG_DIR.")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("log_file")
                                        .short("l")
                                        .long("log-file")
                                        .value_name("LOG_FILE")
                                        .help("The file to log output too. if not given stdout
is used.")
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
                        .arg(Arg::with_name("no_listen")
                                    .short("-n")
                                    .long("no-listen")
                                    .takes_value(false)
                                    .help("Just Randomaly change the background and quit.")
                        )
                        .setting(AppSettings::TrailingVarArg)
                        .setting(AppSettings::AllowLeadingHyphen)
                        .arg(Arg::with_name("feh_args")
                                .short("f")
                                .allow_hyphen_values(true)
                                .long("feh-args")
                                .value_name("FEH_ARGS")
                                .help("Provide optional arguments to pass to feh each time it is run.")
                                .takes_value(true)
                    ).get_matches();

    let log_level = match matches.occurrences_of("log_level") {
        0 => slog::Level::Critical,
        1 => slog::Level::Info,
        2 => slog::Level::Debug,
        3 | _ => slog::Level::Trace
    };
    let logger = match matches.value_of("log_file") {
        Some(d) => {
            let file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(d).unwrap();
            let drain = slog_stream::stream(file, Formater).fuse();
            let drain = slog::level_filter(log_level, drain);
            slog::Logger::root(drain, o!())
        },
        None => {
             let drain = slog_term::streamer().compact().build().fuse();
             let drain = slog::level_filter(log_level, drain);
             slog::Logger::root(drain, o!("version" => "1.0"))
        }
    };
    trace!(&logger, "Initializing logger.");
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

    let reload_time = matches.value_of("reload_time").unwrap_or("6000").parse::<u32>().expect("Reload time in not an integer!");

    let mode = matches.value_of("mode").unwrap_or("random");

    let no_listen = matches.is_present("no_listen");

    let feh_args = match matches.value_of("feh_args") {
        Some(d) => Some(d),
        None => None
    };

    run_mookaite(logger, image_directory, reload_time, mode, no_listen, timeout, feh_args);
}

struct Formater;

impl slog_stream::Format for Formater {
    fn format(&self,
              io: &mut std::io::Write,
              rinfo: &slog::Record,
              _logger_values: &slog::OwnedKeyValueList)
              -> std::io::Result<()> {
        let msg = format!("{} - {}\n", rinfo.level(), rinfo.msg());
        let _ = try!(io.write_all(msg.as_bytes()));
        Ok(())
    }
}
