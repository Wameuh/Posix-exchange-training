use std::fs;
use std::io::{Read, Write};


pub trait Reader {
    fn read(&self, buffer: &mut [u8], file: &mut fs::File) -> Result<usize, std::io::Error> {
        file.read(buffer)
    }
}

pub trait Writer {
    fn write(&self, buffer: &[u8], file: &mut fs::File) -> Result<(), std::io::Error> {
        file.write_all(buffer)
    }
}

#[cfg(test)]
mod writer_and_reader_tests;