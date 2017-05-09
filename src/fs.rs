use walkdir;
use std::path::{ Path, PathBuf };
use rand::Rng;
use rand;
use std::error::Error;


pub struct Directory {
    path: PathBuf,
    images: Vec<PathBuf>
}

impl Directory {

    pub fn new(image_directory: PathBuf) -> Directory {
        if !image_directory.exists() {
            panic!("Image directory does not exist!");
        }
        let mut a = Directory {
            path: image_directory,
            images: Vec::new(),
        };
        a.load();
        a
    }

    fn load(&mut self) {
        for entry in walkdir::WalkDir::new(&self.path) {
            match entry {
                Err(err) => panic!("Walkdir failed to get path infos: {}",err.description()),
                Ok(a) => {
                    let file_type = a.file_type();
                    if !file_type.is_dir() {
                        let p = a.path();
                        match p.extension().and_then(|d|d.to_str()) {
                            Some(d) => {
                                match d {
                                    "jpg"|"png"|"jpeg" => {
                                        if !self.images.contains(&p.to_path_buf()) {
                                            self.images.push(p.to_owned());
                                        }
                                    }
                                    //"png" => self.images.push(p.to_owned()),
                                    _ => {}
                                }
                            },
                            None => unreachable!()
                        }
                    }
                }
            }
        }
    }

    pub fn random_selection(&self) -> &Path {
        let mut rng = rand::thread_rng();
        rng.choose(&self.images).expect("Unable to randomly select image")
    }
}
