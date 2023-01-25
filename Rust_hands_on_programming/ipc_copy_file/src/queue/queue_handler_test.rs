use super::*;
use crate::tools;

#[test]
fn new_with_argument() {
    let queue_name = Some("/mySpecificQueue".to_string());
    let my_queue_handler = QueueHandler::new(queue_name, Some(8), Some(2589));

    assert_eq!(my_queue_handler.queue_name_, "/mySpecificQueue".to_string());
    assert_eq!(my_queue_handler.max_pending_, 8);
    assert_eq!(my_queue_handler.max_size_, 2589);
    assert!(my_queue_handler.queue_.is_none());
}

#[test]
fn new_without_argument() {
    let my_queue_handler = QueueHandler::new(None,None,None);

    assert_eq!(my_queue_handler.queue_name_, "/myQueueDefaultName".to_string());
    assert_eq!(my_queue_handler.max_pending_, 10);
    assert_eq!(my_queue_handler.max_size_, 4096);
    assert!(my_queue_handler.queue_.is_none());
}


#[test]
//test if the clean-up is down.
fn test_drop() {
    let queue_name = Some("/mySpecificQueue".to_string());
    {
        let mut my_queue_handler = QueueHandler::new(queue_name, None, None);
        assert!(my_queue_handler.create().is_ok());
        assert!(my_queue_handler.queue_.is_some());

        let mut options = posixmq::OpenOptions::writeonly();
        options.max_msg_len(10);
        options.capacity(4096);
        options.existing();
        assert!(options.open("/mySpecificQueue").is_ok());
    }
    let mut options = posixmq::OpenOptions::writeonly();
    options.max_msg_len(10);
    options.capacity(4096);
    options.existing();
    assert!(options.open("/mySpecificQueue").is_err());
}

#[test]
fn create_correct_name() {
    let queue_name = Some("/mySpecificQueue".to_string());
    let mut my_queue_handler = QueueHandler::new(queue_name, None, None);
    assert!(my_queue_handler.create().is_ok());
    assert!(my_queue_handler.queue_.is_some());
}


#[test]
fn create_incorrect_attr() {
    let queue_name = Some("/mySpecificQueue".to_string());
    let mut my_queue_handler = QueueHandler::new(queue_name, None, None);
    my_queue_handler.max_pending_=30;
    assert!(my_queue_handler.create().is_err());
    assert!(my_queue_handler.queue_.is_none());
}

/// Will mock the print
#[derive(Default)]
struct DummyLogger(Vec<String>);
impl tools::Logger for DummyLogger {
    fn print(&mut self, value: &std::fmt::Arguments<'_>) {
        self.0.push(value.to_string());
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn flush(&mut self, value: &std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.0.push(value.to_string());
        Ok(())
    }
}


#[test]
fn open_queue_not_exists() {
    let name = "/open_queue_not_exists".to_string();
    let max_attempt = 1;
    
    let logger = DummyLogger::default();
    let output_rc=Rc::new(RefCell::new(PrintWheel::new(Some(Box::new(logger)))));



    let queue_name = Some(name);
    let mut my_queue_handler = QueueHandler::new(queue_name, None, None);
    match my_queue_handler.open(max_attempt, &output_rc) {
        Ok(_) => assert!(false),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => assert!(true),
            _ => assert!(false)
        }
    }
}



#[test]
fn open_queue_ok() {
    
    let name = "/open_queue_ok".to_string();
    let max_attempt = 1;
    
    let logger = tools::StdoutLogger;
    let output_rc=Rc::new(RefCell::new(PrintWheel::new(Some(Box::new(logger)))));



    let queue_name = Some(name);
    let mut queue_creation = QueueHandler::new(queue_name.clone(), None, None);
    assert!(queue_creation.create().is_ok());

    let mut my_queue_handler = QueueHandler::new(queue_name.clone(), None, None);
    assert!(my_queue_handler.open(max_attempt, &output_rc).is_ok());
    assert!(my_queue_handler.queue_.is_some());
}

#[test]
fn open_queue_creation_delayed() {
    let name = "/open_queue_creation_delayed".to_string();
    let max_attempt = 2;
    let name_copy = name.clone();
    let logger = DummyLogger::default();

    let thread_open_queue = thread::spawn(move || {
        let output_rc=Rc::new(RefCell::new(PrintWheel::new(Some(Box::new(logger)))));
        let queue_name = Some(name_copy);
        let mut my_queue_handler = QueueHandler::new(queue_name, None, None);
        return my_queue_handler.open(max_attempt, &output_rc);
    });


    let queue_name = Some(name.clone());
    let mut queue_creation = QueueHandler::new(queue_name, None, None);
    assert!(queue_creation.create().is_ok());


    let res = thread_open_queue.join();
    assert!(res.is_ok());
}



use nix::mqueue;
use nix::sys::stat;
use std::ffi::CString;

#[test]
fn open_queue_insufficient_rights() {
    let name = "/open_queue_insufficient_rights".to_string();
    //use nix crates to create a queue with some restrictions

    let queue_name = CString::new(name.clone()).unwrap(); 
    let oflags = {
        let mut flags = mqueue::MQ_OFlag::empty();
        // Put queue in r/w mode
        flags.toggle(mqueue::MQ_OFlag::O_RDWR);
        // Enable queue creation
        flags.toggle(mqueue::MQ_OFlag::O_CREAT);
        // Fail if queue exists already
        flags.toggle(mqueue::MQ_OFlag::O_EXCL);
        flags
    };

    let attr = mqueue::MqAttr::new(
        0, 9, 9, 0
    );

    let queue_descriptor = mqueue::mq_open(
        &queue_name,
        oflags,
        stat::Mode::empty(),
        Some(&attr),
    ).unwrap();



    let queue_name2 = Some(name);
    let logger = DummyLogger::default();
    let output_rc=Rc::new(RefCell::new(PrintWheel::new(Some(Box::new(logger)))));
    let mut my_queue_handler = QueueHandler::new(queue_name2, None, None);
    assert!(my_queue_handler.open(2, &output_rc).is_err());


    mqueue::mq_close(queue_descriptor).unwrap();
    mqueue::mq_unlink(&queue_name).unwrap();

}

