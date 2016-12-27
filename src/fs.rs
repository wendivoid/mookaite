use walkdir;
use std::path::{ Path, PathBuf };
use std::time::Instant;
use rand;
use rand::Rng;
use std::process::exit;
use std::error::Error;

use slog::Logger;
#[derive(Debug)]
pub struct Directory {
    logger: Logger,
    path: PathBuf,
    last_load: Instant,
    images: Vec<PathBuf>
}

impl Directory {
    pub fn new(image_directory: &str, logger: Logger) -> Directory {
        trace!(logger, "Creating Directory store");
        let path = Path::new(image_directory);
        if !path.exists() {
            crit!(&logger, "The Directory {} does not exist!",image_directory);
            exit(1);
        }
        let mut a = Directory {
            logger: logger,
            path: path.to_owned(),
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
                                                trace!(self.logger, "Found file {:?} that is not png|jpg.", p);
                                            }
                                        }
                                    },
                                }
                            },
                            None => {}
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
        let mut rng = rand::thread_rng();
        rng.choose(&self.images).expect("Unable to randomly select image")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_paths() {
        let mut d = Directory::new(".");
        println!("{:#?}",d);
    }

    #[test]
    fn random_image() {
        let mut a = Directory::new("/home/orphius/Pictures/backgrounds");
        println!("{:#?}",a.random_selection());
    }
}
