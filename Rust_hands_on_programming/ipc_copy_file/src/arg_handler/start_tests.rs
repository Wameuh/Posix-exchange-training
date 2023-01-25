use super::*;
use std::any::Any;


/// Will mock the print
#[derive(Default)]
struct DummyLogger(Vec<String>);
impl Logger for DummyLogger {
    fn print(&mut self, _value: &core::fmt::Arguments<'_>) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn flush(&mut self, _value: &core::fmt::Arguments<'_>) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn start_help() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-h".to_string());

    assert_eq!(start(args, &mut ostream), Ok(()));
}


#[test]
fn start_queue() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-q".to_string());
    args.push("-f".to_string());
    args.push("myFile".to_string());

    assert_eq!(start(args, &mut ostream), Err("Queue not implemented."));
}

#[test]
fn start_pipe() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-p".to_string());
    args.push("-f".to_string());
    args.push("myFile".to_string());

    assert_eq!(start(args, &mut ostream), Err("Pipe not implemented."));
}

#[test]
fn start_shm() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-s".to_string());
    args.push("-f".to_string());
    args.push("myFile".to_string());

    assert_eq!(start(args, &mut ostream), Err("Shm not implemented."));
}

#[test]
fn start_other() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-qa".to_string());

    assert_eq!(start(args, &mut ostream), Err("Wrong arguments"));
}

#[test]
fn start_miss_method() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("myFile".to_string());

    assert_eq!(start(args, &mut ostream), Err("No method provided"));
}


#[test]
fn start_miss_filepath() {
    let logger = DummyLogger::default();
    let mut ostream: Box<dyn Logger>= Box::new(logger);
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("-q".to_string());

    assert_eq!(start(args, &mut ostream), Err("Filepath missing."));
}