use walkdir;
use std::path::{ Path, PathBuf };
use std::time::Instant;
use rand::Rng;
use rand;
use std::process::exit;
use std::error::Error;

use slog::Logger;

pub struct Directory {
    rng: rand::ThreadRng,
    logger: Logger,
    path: PathBuf,
    last_load: Instant,
    images: Vec<PathBuf>
}

impl Directory {

    pub fn new(image_directory: PathBuf, logger: Logger) -> Directory {
        trace!(logger, "Creating Directory store");
        if !image_directory.exists() {
            crit!(&logger, "The Directory {:?} does not exist!",image_directory);
            exit(1);
        }
        let rng = rand::thread_rng();
        let mut a = Directory {
            rng: rng,
            logger: logger,
            path: image_directory,
            images: Vec::new(),
            last_load: Instant::now()
        };
        a.load();
        a
    }

    fn load(&mut self) {
        debug!(self.logger, "Loading images from IMG_DIR");
        for entry in walkdir::WalkDir::new(&self.path) {
            match entry {
                Err(err) => panic!("Walkdir failed to get path infos: {}",err.description()),
                Ok(a) => {
                    let file_type = a.file_type();
                    if !file_type.is_dir() {
                        let p = a.path();
                        match p.extension() {
                            None => {},
                            Some(d) => {
                                match d.to_str() {
                                    None => {},
                                    Some(d) => {
                                        match d {
                                            "jpg"|"png"|"jpeg" => {
                                                if !self.images.contains(&p.to_path_buf()) {
                                                    trace!(self.logger, "Found image file {:?}!",p);
                                                    self.images.push(p.to_owned());
                                                }
                                            }
                                            //"png" => self.images.push(p.to_owned()),
                                            _ => {
                                                trace!(self.logger, "Found file {:?} that is not png|jpg|jpeg.", p);
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn reload(&mut self) {
        info!(self.logger, "Reloading image directory");
        self.last_load = Instant::now();
        self.load();
    }

    pub fn random_selection(&mut self) -> &Path {

        self.rng.choose(&self.images).expect("Unable to randomly select image")
    }
}
