extern crate x11_dl;
extern crate libc;
extern crate walkdir;
extern crate rand;
#[macro_use]
extern crate slog;
extern crate slog_term;

mod x;
use x::XWrapper;
mod fs;


use std::path::PathBuf;
use std::time::{ Duration, Instant };

pub fn run_mookaite(logger: slog::Logger,
                 img_dir: PathBuf,
                 reload_time: u32,
                 mode: &str,
                 keep_going: bool,
                 timeout: u32,
                 feh_args: Option<&str>
             ) {
            trace!(&logger, "Initializing mookaite, mode: {}, timeout: {}",mode,timeout);
            let dir = fs::Directory::new(img_dir, logger.new(o!()));
            let mut m = Mookaite::new(logger, Duration::new(reload_time as u64,0), dir, mode, timeout, keep_going, feh_args);
            m.run();
}

pub struct Mookaite {
    since_timeout: Instant,
    timeout: Duration,
    keep_going: bool,
    mode: String,
    reload_time: Duration,
    logger: slog::Logger,
    img_dir: (fs::Directory, Instant),
    x: XWrapper,
    image_map: Vec<PathBuf>
}

impl Mookaite {
    pub fn new(logger: slog::Logger, reload_time: Duration,
        directory: fs::Directory,
        mode: &str,
        timeout: u32,
        keep_going: bool,
        feh_args: Option<&str>
    ) -> Mookaite {
        let xx = XWrapper::new(logger.clone(), feh_args);
        Mookaite {
            keep_going: keep_going,
            since_timeout: Instant::now(),
            timeout: Duration::new(timeout as u64,0),
            reload_time: reload_time,
            mode: mode.to_string(),
            x: xx,
            logger: logger,
            img_dir: (directory, Instant::now()),
            image_map: vec![],

        }
    }

    pub fn change_backgrounds(&mut self) {
        debug!(self.logger, "Changing backgrounds");
        self.image_map.clear();
        let nd = self.x.get_number_of_desktops() as usize;
        while self.image_map.len() < nd {
            let img = self.img_dir.0.random_selection().to_path_buf();
            if !self.image_map.contains(&img) {
                trace!(self.logger, "adding img {:?} to map.", img);
                self.image_map.push(img);
            }
        }
    }

    pub fn run_mapped(&mut self) {
        info!(self.logger, "Running in mapped mode!");

        loop {
            if self.img_dir.1.elapsed() > self.reload_time {
                self.img_dir.0.reload();
                self.img_dir.1 = Instant::now();
            }
            let cd = self.x.get_current_desktop();
            match self.x.next_event() {
                    Some(_) => {
                        let ref mut current_bg = self.image_map[cd];
                        self.x.change_background(current_bg);
                    },
                    None => {
                        if self.since_timeout.elapsed() > self.timeout {
                            self.change_backgrounds();
                            let ref mut current_bg = self.image_map[cd];
                            self.since_timeout = Instant::now();
                            self.x.change_background(current_bg);
                        }
                    }
            }

        }
    }

    pub fn run_random(&mut self) {
        info!(self.logger, "Running in random mode.");
        loop {
            match self.x.next_event() {
                Some(_) => self.x.change_background(&self.img_dir.0.random_selection().to_path_buf()),
                None => {
                    if self.img_dir.1.elapsed() > self.reload_time {
                        self.img_dir.0.reload();
                        self.img_dir.1 = Instant::now();
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        trace!(self.logger, "Beginning main loop");
        self.change_backgrounds();
        let cd = self.x.get_current_desktop();
        // initialy change background
        {
            let ref first_image = self.image_map[cd];
            self.x.change_background(&first_image);
        }

        if self.keep_going {
            trace!(self.logger, "exiting early due to --no-listen flag");
            return;
        }
        match &self.mode[..] {
            "random" => self.run_random(),
            "mapped" => self.run_mapped(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
