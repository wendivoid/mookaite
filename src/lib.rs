#![allow(deprecated)]

extern crate walkdir;
extern crate rand;
extern crate mio;


mod fs;

use std::path::{ PathBuf, Path };
use std::time::Duration;
use mio::timer::Builder;
use mio::{ Poll, PollOpt, Ready, Token, Events};


fn change_background(cmd: &str, args: &Vec<&str>, path: &Path) {
    ::std::process::Command::new(cmd).args(args).arg(path).spawn().expect("Failed To Run System Command");
    ::std::thread::sleep(Duration::from_millis(500));
}

pub fn run_mookaite(img_dir: PathBuf,
                 timeout: u32,
                 cmd: &str,
                 feh_args: Vec<&str>
             ) {
            let dir = fs::Directory::new(img_dir);

            let timer = Builder::default();
            let mut timer = timer.tick_duration(Duration::from_secs(timeout as u64)).build::<()>();
            timer.set_timeout(Duration::from_secs(timeout as u64),()).unwrap();

            let poll = Poll::new().unwrap();

            poll.register(&timer, Token(2), Ready::readable(), PollOpt::edge()).expect("");

            let mut events = Events::with_capacity(1024);

            loop {
                poll.poll(&mut events, None).unwrap();
                for event in events.iter() {
                    match event.token() {
                        Token(2) => {
                            change_background(cmd, &feh_args, dir.random_selection());
                            timer.set_timeout(Duration::from_secs(timeout as u64),()).unwrap();
                        },
                        Token(_) => {}
                    }

                }
            }
}
