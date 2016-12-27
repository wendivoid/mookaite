extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stream;
extern crate mookaite;

use slog::DrainExt;
use clap::{ App, Arg };
use std::fs::OpenOptions;
use mookaite::run_mookaite;

fn main() {
    let matches = App::new("mookaite")
                        .author("Bytebuddha <shadowcynical@gmail.com>")
                        .version("1.0")
                        .about("Keeps track of your desktop backgrounds")
                        .arg(Arg::with_name("mode")
                                    .short("m")
                                    .long("mode")
                                    .value_name("MODE")
                                    .help("The mode to of changing desktops")
                                    .takes_value(true)
                                    .possible_values(&["mapped", "random"])
                        )
                        .arg(Arg::with_name("log_level")
                                    .short("v")
                                    .long("verbose")
                                    .multiple(true)
                                    .help("Set the log level.")
                        )
                        .arg(Arg::with_name("image_dir")
                                .short("d")
                                .long("image-directory")
                                .value_name("IMG_DIR")
                                .help("The directory to select random images from")
                                .takes_value(true)

                        )
                        .arg(Arg::with_name("reload_time")
                                    .short("r")
                                    .long("reload")
                                    .help("The time in secs to wait before searching for new files in IMG_DIR.")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("log_file")
                                        .short("l")
                                        .long("log-file")
                                        .help("The file to log output too")
                                        .takes_value(true)
                        )
                        .arg(Arg::with_name("timeout")
                                    .short("t")
                                    .long("timeout")
                                    .help("How long to keep a image on a given desktop")
                                    .takes_value(true)
                        )
                        .arg(Arg::with_name("no_listen")
                                    .short("-n")
                                    .long("no-listen")
                                    .takes_value(false)
                                    .help("Just change the background and quit")
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
    let timeout = matches.value_of("timeout").unwrap_or("300").parse::<u32>().expect("Timeout was not an integer");

    let image_directory = matches.value_of("image_dir").unwrap_or("/home/$USER/Pictures");

    let reload_time = matches.value_of("reload_time").unwrap_or("6000").parse::<u32>().expect("Reload time in not an integer!");

    let mode = matches.value_of("mode").unwrap_or("random");

    let no_listen = matches.is_present("no_listen");

    run_mookaite(logger, image_directory, reload_time, mode, no_listen, timeout);
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
