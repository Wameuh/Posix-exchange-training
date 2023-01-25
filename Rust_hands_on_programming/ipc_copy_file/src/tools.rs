use std::time::{Instant, Duration};
use std::path::Path;
use std::env;
use libc::{PATH_MAX, FILENAME_MAX};
use nix::sys::statvfs;
use std::fs;
use std::any::Any;
use std::io::{Write,stdout};
use std::thread;


use crate::ipc_errors;
use ipc_errors::IpcErrors;

use std::fmt::Arguments;

pub struct PrintWheel {
    last_char_: u8,
    wheel_: [char; 4],
    last_update_ : Instant,
    ostream_: Box<dyn Logger>,
}

impl PrintWheel{
    pub fn new(ostream_option: Option<Box<dyn Logger>>) -> PrintWheel {
        let ostream = ostream_option.unwrap_or(Box::new(StdoutLogger));
        PrintWheel {last_char_: 0,
            wheel_: ['|','/','-','\\'],
            last_update_: Instant::now() - Duration::from_millis(100),
            ostream_: ostream}
    }
}


pub trait Logger {
    fn print(&mut self, value: &Arguments<'_>);
    fn flush(&mut self, value: &Arguments<'_>) -> std::io::Result<()> ;
    fn as_any(&self) -> &dyn Any;
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
    fn print(&mut self, value: &Arguments<'_>) {
      println!("{}", value);
    }
    fn flush(&mut self, value: &Arguments<'_>) -> std::io::Result<()> {
      print!("\r{}", value);
      stdout().flush()
    }
    fn as_any(&self) -> &dyn Any {
      self
    }
}

pub trait ToolBox {
    /// check_file_path is a function which guarranty that the file path given is valid.
    fn check_file_path(&self, path: &String) -> Result<(), ipc_errors::IpcErrors> {
        if let Some(file_name) = Path::new(path).file_name() {
            if file_name.len() >= FILENAME_MAX as usize {
                return Err(IpcErrors::FileNameTooLong(file_name.to_string_lossy().to_string()));
            }
            if Path::new(path).is_absolute() && path.len()-file_name.len() > PATH_MAX as usize {
                return Err(IpcErrors::FilePathAbsTooLong {givenpath: path.to_string()});
            }
            if let Ok(current_dir) = env::current_dir() {
                if current_dir.to_str().unwrap().len()+path.len()-file_name.len() > PATH_MAX as usize {
                    return Err(IpcErrors::FilePathTooLong { currentdir: current_dir.to_string_lossy().to_string(),givenpath: path.to_string()});
                }
            } else {
                return Err(IpcErrors::CurrentDir());
            }

            Ok(())
        } else {
            return Err(IpcErrors::WrongFileName(path.to_string()));
        }
    }

/// This function will just check if the file exist. It will call check_file_path first.
    fn check_file_exist(&self, file_path: &String) -> Result<(), ipc_errors::IpcErrors> {
        self.check_file_path(file_path)?;

        let path = Path::new(file_path);
        if path.is_file() {
            Ok(())
        } else {
            Err(IpcErrors::FileDoestNotExist(file_path.to_string()))
        }
    }

/// return the size of the file
    fn return_file_size(&self, file_path: &String)-> Result<u64, ipc_errors::IpcErrors> {
        match fs::metadata(file_path) {
            Ok(metadata) => Ok(metadata.len()),
            Err(err) => Err(IpcErrors::FileMetadata(file_path.to_string(), err.to_string())),
        }
    }

/// check if there is enough space available
    fn is_there_enough_space_available(&self, size: &u64, path: &str, default_buffer_size: &usize) -> Result<(), ipc_errors::IpcErrors> {
        match statvfs::statvfs(path) {
            Ok(stats) => {
                if *size  <= std::cmp::min(stats.blocks_available()*stats.block_size() - *default_buffer_size as u64,0) {
                    return Ok(());
                } else { 
                    return Err(IpcErrors::NotEnoughSpace(*size, stats.blocks_available()*stats.block_size() as u64))
                }
            },
            Err(err) => Err(IpcErrors::SystemStats(path.to_string(), err.to_string()))
        }
    }

/// print instructions how to use the programs
    fn print_instruction(&self, ostream: &mut Box<dyn Logger>) {
        let instructions = "Welcome to this dummy program which can copy a file with an uneffective way.

To choose a method use the corresponding parameter: 
-q or --queue for queue message passing. (not implemented)
-p or --pipe for pipes. (not implemented)
-s or --shm for shared memory. (not implemented)\nIn option, you can specify the name used for the method (e.g.: `-p myPipe`)

You have to specify which file will used with the command -f or --file

For instance: `-p myPipe -f myFile` or  `-q -f myFile`.\n";
        ostream.print(&format_args!("{}", instructions));
    }

/// Print elements whitout creating a new line, checking that there is not too much element printed.
    fn update_printing_elements<'a>(&self, content: &String, force_print: bool, output: &'a mut PrintWheel) -> &'a mut PrintWheel {

        if force_print || output.last_update_.elapsed().as_millis() > 100 {
            output.last_update_ = Instant::now();
            output.ostream_.flush(&format_args!("\r{} {}", output.wheel_[output.last_char_ as usize], content)).unwrap(); //can panic
            output.last_char_ = (output.last_char_+1) % 4;
        }

        output
    }

    fn nap(&self, duration: Duration) {
        thread::sleep(duration);
    }

    fn print_file_size(&mut self, fise_size: u64, ostream:&mut Box<dyn Logger>) {
        let gb = fise_size/(1024*1024*1024);
        let mb = (fise_size-gb*1024*1024*1024)/(1024*1024);
        let kb = (fise_size-gb*1024*1024*1024-mb*1024*1024)/1024;
        let b = fise_size-gb*1024*1024*1024-mb*1024*1024-kb*1024;

        ostream.print(&format_args!("Transferring a file which size: {} GB {} MB {} KB {} B.", gb, mb, kb, b));
    }

    fn compare_file_names(&self, file1: &String, file2: &String) -> Result<(),ipc_errors::IpcErrors> {
        if let Ok(current_dir) = env::current_dir() {

            let mut full_file1= current_dir.clone();
            let mut full_file2= current_dir.clone();

            if Path::new(file1).is_relative() {
                full_file1.push(file1);
            } else {
                full_file1 = Path::new(file1).to_path_buf();
            }

            if Path::new(file2).is_relative() {
                full_file2.push(file2);
            } else {
                full_file2 = Path::new(file2).to_path_buf();
            }

            if full_file1 == full_file2 {
                return Err(IpcErrors::SameFileAndMethod(full_file1, full_file2));
            }
        } else {
            return Err(IpcErrors::CurrentDir());
        }
        Ok(())
    }
}




#[cfg(test)]
mod check_file_path_tests;

#[cfg(test)]
mod check_file_exist_tests;

#[cfg(test)]
mod return_file_size_tests;

#[cfg(test)]
mod is_there_enough_space_available_tests;

#[cfg(test)]
mod tests_of_printed_elements;

#[cfg(test)]
mod stdoutlogger_tests;


#[cfg(test)]
mod nap_testing;


#[cfg(test)]
mod compare_file_names_tests;
