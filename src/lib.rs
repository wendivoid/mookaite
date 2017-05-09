#![allow(deprecated)]

extern crate walkdir;
extern crate rand;
extern crate mio;
extern crate xlib;


mod fs;

use std::os::unix::io::AsRawFd;
use std::path::{ PathBuf, Path };
use std::time::Duration;
use mio::timer::Builder;
use mio::unix::EventedFd;
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

            let display = xlib::Display::default().unwrap();

            let root_window = display.root_window(display.default_screen());
            root_window.select_input(&display, xlib::PropertyChangeMask);

            let fd = display.as_raw_fd();
            let mio_fd = EventedFd(&fd);

            let poll = Poll::new().unwrap();

            poll.register(&timer, Token(2), Ready::readable(), PollOpt::edge()).expect("");
            poll.register(&mio_fd, Token(3), Ready::readable(), PollOpt::edge()).expect("");

            let mut events = Events::with_capacity(1024);

            let curr_desk_atom = display.intern_atom("_NET_CURRENT_DESKTOP").unwrap();

            loop {
                poll.poll(&mut events, None).unwrap();
                for event in events.iter() {
                    match event.token() {
                        Token(2) => {
                            change_background(cmd, &feh_args, dir.random_selection());
                            timer.set_timeout(Duration::from_secs(timeout as u64),()).unwrap();
                        },
                        Token(3) => {
                            for _ in 0..events.len() {
                                if let Ok(xev) = display.get_event() {
                                    let prop: xlib::XPropertyEvent = From::from(xev.event);
                                    if prop.atom == curr_desk_atom {
                                          //print!("Wanted: Time: {:?}, atom {}",Instant::now(),prop.atom);
                                          //println!(" {:?}",display.get_atom_name(prop.atom));
                                          change_background(cmd, &feh_args, dir.random_selection());
                                    } else {
                                        //print!("Ignored: Time: {:?}, atom {}",Instant::now(),prop.atom);
                                        //println!(" {:?}",display.get_atom_name(prop.atom));
                                    }
                                } else {
                                    unreachable!()
                                }
                            }
                         },
                        Token(_) => {}
                    }

                }
            }
}
