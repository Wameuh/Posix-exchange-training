use super::*;
use crate::tools;
use crate::header;

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
fn header() {
    let name = "/queue_send_receive".to_string();
    let name_copy = name.clone();
    let logger = DummyLogger::default();
    const KEY: u64 = 51254;
    const DEFAULT_BUFFER_SIZE: usize = 4096;

    let mut my_queue_writer = QueueHandler::new(
        Some(name_copy),
        None,
        Some(DEFAULT_BUFFER_SIZE)
    );
    assert!(my_queue_writer.create().is_ok());

    let my_header = header::Header::new(
        Some(51254),
        Some(50),
        DEFAULT_BUFFER_SIZE
    );

    assert!(my_queue_writer.send_data(my_header.get_data(), 10, 3).is_ok());

    let mut my_queue_reader = QueueHandler::new(
        Some(name),
        None,
        Some(DEFAULT_BUFFER_SIZE)
    );
    let output_rc=Rc::new(RefCell::new(PrintWheel::new(Some(Box::new(logger)))));
    assert!(my_queue_reader.open(2, &output_rc).is_ok());
    let mut data: Vec<u8> = Vec::new();
    data.resize(DEFAULT_BUFFER_SIZE, 0);

    let result = my_queue_reader.receive_data(&mut data, 2, 10);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DEFAULT_BUFFER_SIZE);
    
    

    let my_header2 = header::Header::new(
        None,
        None,
        DEFAULT_BUFFER_SIZE
    );

    assert!(my_header2.extract_key(&data).is_some());
    assert_eq!(my_header2.extract_key(&data).unwrap(), KEY);

    assert!(my_header2.extract_file_size(&data).is_some());
    assert_eq!(my_header2.extract_file_size(&data).unwrap(), 50);

    
    

}