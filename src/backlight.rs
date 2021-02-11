use crate::device_file::{read, write};
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub struct Backlight {
    file: RefCell<File>,
    max_brightness: u64,
}

impl Backlight {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(Path::new(path).join("brightness"))?;

        let max_brightness = fs::read_to_string(Path::new(path).join("max_brightness"))?
            .trim()
            .parse()?;

        Ok(Self {
            file: RefCell::new(file),
            max_brightness,
        })
    }

    pub fn get(&self) -> Result<u64, Box<dyn Error>> {
        Ok(read(&mut self.file.borrow_mut())? as u64)
    }

    pub fn set(&self, value: u64) -> Result<(), Box<dyn Error>> {
        write(
            &mut self.file.borrow_mut(),
            value.max(1).min(self.max_brightness) as f64,
        )
    }
}
