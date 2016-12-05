//Randomly change background everytime the desktop changes

//Listens to XEvents and when _NET_CURRENT_DESKTOP changes
//randomly select a file from the image directory and display it using /usr/bin/feh
#[allow(non_snake_case)]
extern crate rand;
extern crate x11_dl;
extern crate libc;

use rand::Rng;
use std::fs::read_dir;
use std::path::Path;
use std::ffi::{ OsString, CString };
use x11_dl::xlib;
use std::ptr::{ null, null_mut };
use std::mem::zeroed;
use std::process::Command;


use libc::{ c_uchar, c_int,  c_ulong };
use std::mem::uninitialized;

use std::slice::from_raw_parts;


#[allow(non_snake_case)]
struct Backgrounded<'a> {
    directory: &'a Path,
    X: xlib::Xlib,
    dpy: *mut xlib::Display,
    root:  xlib::Window,
    current_desktop: i8,
}

impl <'a>Backgrounded<'a> {

    pub fn get_background_image(&mut self) -> String {
        let mut r: Vec<OsString> = Vec::new();
        for path in read_dir(&self.directory).unwrap() {
            r.push(path.unwrap().file_name());
        }
        let dir = self.directory.display();
        let file = &rand::thread_rng().choose(&r).unwrap();
        return format!("{}/{}",dir,file.as_os_str().to_str().unwrap());
    }

    pub fn get_atom_by_name(&self, s: &str) -> u64 {
        let d_atom_s = CString::new(s).unwrap();
        unsafe {
            (self.X.XInternAtom)(self.dpy, d_atom_s.as_ptr(), xlib::False) as u64
        }
    }
    pub fn setup(&self) {
       unsafe {
           (self.X.XSelectInput)(self.dpy, self.root, xlib::PropertyChangeMask);
       }
    }
    pub fn live(&mut self) {
     unsafe {
         let mut event = zeroed();
         self.current_desktop = self.detect_desktop();
         loop {
             (self.X.XNextEvent)(self.dpy, &mut event);
             if self.filter_events(event) {
                 self.change_background();
                 self.current_desktop = self.detect_desktop();
            }
         }
      }
    }

    pub fn change_background(&mut self){
       Command::new("/usr/bin/feh")
                     .arg("--bg-scale")
                     .arg(&self.get_background_image().to_owned())
                     .output()
                     .expect("failed to execute proces");
    }
    pub fn filter_events(&mut self, event: xlib::XEvent) -> bool {
        let e: xlib::XClientMessageEvent = From::from(event);
        if e.message_type == self.get_atom_by_name("_NET_CURRENT_DESKTOP") {
            let new_desktop = self.detect_desktop();
            if new_desktop != self.current_desktop {
                self.current_desktop = new_desktop;
                return true;
            }
        }
        return false;
    }
    fn detect_desktop(&self) -> i8 {
        unsafe {
   let mut actual_type_return : c_ulong = 0;
   let mut actual_format_return : c_int = 0;
   let mut nitems_return : c_ulong = 0;
   let mut bytes_after_return : c_ulong = 0;
   let mut prop_return : *mut c_uchar = uninitialized();

   (self.X.XGetWindowProperty) (self.dpy, self.root as c_ulong, self.get_atom_by_name("_NET_CURRENT_DESKTOP") as c_ulong,
                                    0, 0xFFFFFFFF, 0, 0,
                                    &mut actual_type_return,
                                    &mut actual_format_return,
                                    &mut nitems_return,
                                    &mut bytes_after_return,
                                    &mut prop_return);
   let rtn: Vec<u64> = from_raw_parts(prop_return as *const c_ulong, nitems_return as usize).iter()
                     .map(|&c| c as u64)
                     .collect();
   return rtn[0] as i8;
   }
   }
}
fn main () {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
        let display = (xlib.XOpenDisplay)(null());
        if display == null_mut() {
                 panic!("can't open display");
        }
        let rt = (xlib.XRootWindow)(display, (xlib.XDefaultScreen)(display));
        let mut b = Backgrounded { current_desktop: 0 as i8,
                                   directory: Path::new("/home/orphius/Pictures/backgrounds"),
                                   dpy: display,
                                   root: rt ,
                                   X: xlib
                                };

        b.setup();
        b.change_background();
        b.live()
    }

}
