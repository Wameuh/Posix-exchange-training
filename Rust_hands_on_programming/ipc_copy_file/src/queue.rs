use crate::tools::{ToolBox, PrintWheel};
use std::time::Duration;
use posixmq;
use std::thread;
use std::time;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ffi::CString;
use std::io::{Error, ErrorKind};




pub struct QueueHandler {
    queue_name_: String,
    max_pending_: usize,
    max_size_: usize,
    queue_: Option<posixmq::PosixMq>
}

impl ToolBox for QueueHandler{}

impl Drop for QueueHandler {
    fn drop(&mut self) {
        // Test if the queue is created or open, in this case unlink the queue. Closing the queue is a part of the Queue Drop implementation
        if let Some(_queue_descriptor) = std::mem::take(&mut self.queue_) {
            posixmq::remove_queue_c(CString::new(self.queue_name_.to_owned()).unwrap().as_c_str()).ok();
        }
    }
}
impl  QueueHandler{
    pub fn new(queue_name: Option<String>, max_pending_option: Option<usize>, max_size_option: Option<usize>) -> QueueHandler {
        let max_pending = max_pending_option.unwrap_or(10);
        let max_size = max_size_option.unwrap_or(4096);
        QueueHandler {
            queue_name_: queue_name.unwrap_or("/myQueueDefaultName".to_string()),
            max_pending_: max_pending,
            max_size_: max_size,
            queue_: None
        }
    }

    pub fn create(&mut self) -> Result<(), std::io::Error> {
        let mut options = posixmq::OpenOptions::writeonly();
        options.max_msg_len(self.max_size_);
        options.capacity(self.max_pending_);
        options.create();
        
        match options.open(&self.queue_name_) {
            Ok(q) => {
                self.queue_ = Some(q);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn open(&mut self, max_attempt: u32, output_stream_rc: &Rc<RefCell<PrintWheel>>) -> Result<(), std::io::Error> {
        let starting_time = time::Instant::now();
        let mut options = posixmq::OpenOptions::readonly();
        options.max_msg_len(self.max_size_);
        options.capacity(self.max_pending_);
        options.existing();

        loop {
            let mut output_stream: RefMut<PrintWheel> = output_stream_rc.borrow_mut();
            match options.open(&self.queue_name_) {
                Ok(q) => {
                    self.queue_ = Some(q);
                    return Ok::<(),std::io::Error>(());
                }
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        let waiting_print = "Waiting for creation of the queue.".to_string();
                        self.update_printing_elements(&waiting_print, false,&mut output_stream);
                        
                        if starting_time.elapsed().as_secs() > max_attempt as u64 {
                            return Err(e);
                        }
                        thread::sleep(time::Duration::from_millis(50));
                    },
                    _ => {return  Err::<(), Error>(e);}, // others cases (no access right for instance)
                }
            }
        }
    }

    pub fn send_data(&mut self, chunk: &Vec<u8>, priority: u32, max_attempt: u32) -> Result<(), Error> {
        if let Some(queue) = &self.queue_ {
            let waiting_time = Duration::new(max_attempt as u64,0);
            queue.send_timeout(priority, chunk, waiting_time)?;
            Ok(())
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }

    pub fn receive_data(&mut self, buffer: &mut Vec<u8>, max_attempt: u32, priority: u32) -> Result<usize, Error> {
        buffer.resize(self.max_size_, 0);
        if let Some(queue) = &self.queue_ {
            let waiting_time = Duration::new(max_attempt as u64,0);
            let (prio, size_of_data) = queue.recv_timeout(buffer, waiting_time)?;
            if prio != priority { //priority is not as expected: maybe another program is pushing some data
                return Err(Error::from(ErrorKind::InvalidInput));
            } else {
                return Ok(size_of_data);
            }

        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }

} 

#[cfg(test)]
mod queue_handler_test;

#[cfg(test)]
mod queue_transfer_test;