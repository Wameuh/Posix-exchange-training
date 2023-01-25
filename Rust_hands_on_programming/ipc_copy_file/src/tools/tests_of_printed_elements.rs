use super::*;



struct ToolBoxes {}
impl ToolBox for ToolBoxes {}

/// Will mock the print
#[derive(Default)]
struct DummyLogger(Vec<String>);
impl Logger for DummyLogger {
    fn print(&mut self, value: &Arguments<'_>) {
        self.0.push(value.to_string());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn flush(&mut self, value: &Arguments<'_>) -> std::io::Result<()> {
        self.0.push(value.to_string());
        Ok(())
    }
}


#[test]
fn print_instruction_test(){

    let logger = DummyLogger::default();
    let tb = ToolBoxes {};
    let mut ostream: Box<dyn Logger>= Box::new(logger);

    tb.print_instruction(&mut ostream);

    let logger = (*ostream).as_any().downcast_ref::<DummyLogger>().unwrap();

    assert_eq!(logger.0[0], "Welcome to this dummy program which can copy a file with an uneffective way.

To choose a method use the corresponding parameter: 
-q or --queue for queue message passing. (not implemented)
-p or --pipe for pipes. (not implemented)
-s or --shm for shared memory. (not implemented)\nIn option, you can specify the name used for the method (e.g.: `-p myPipe`)

You have to specify which file will used with the command -f or --file

For instance: `-p myPipe -f myFile` or  `-q -f myFile`.\n");

}


#[test]
fn update_printed(){
    
    let logger = DummyLogger::default();
    let tb = ToolBoxes {};

    let mut output=PrintWheel::new(Some(Box::new(logger)));
    tb.update_printing_elements(&"test".to_string(), true, &mut output);

    let logger = (*output.ostream_).as_any().downcast_ref::<DummyLogger>().unwrap();

    assert_eq!(logger.0.len(), 1);
    assert_eq!(logger.0[0], "\r| test");

}

#[test]
fn update_not_enough_time(){

    let logger = DummyLogger::default();
    let tb = ToolBoxes {};

    let mut output=PrintWheel::new(Some(Box::new(logger)));
    tb.update_printing_elements(&"test".to_string(), true, &mut output);
    tb.update_printing_elements(&"test".to_string(), false, &mut output);

    let logger = (*output.ostream_).as_any().downcast_ref::<DummyLogger>().unwrap();

    assert_eq!(logger.0.len(), 1);

}


#[test]
fn print_file_size_test() {
    let size0:u64 = 84597970829312;
    let size1:u64 = 2097152;
    let size2:u64 = 523;
    let size3:u64 = 293888;
    


    let logger = DummyLogger::default();
    let mut tb = ToolBoxes {};
    let mut ostream: Box<dyn Logger>= Box::new(logger);


    tb.print_file_size(size0, &mut ostream);
    tb.print_file_size(size1, &mut ostream);
    tb.print_file_size(size2, &mut ostream);
    tb.print_file_size(size3, &mut ostream);

    let logger = (ostream).as_any().downcast_ref::<DummyLogger>().unwrap();
    assert_eq!(logger.0.len(), 4);
    assert_eq!(logger.0[0], "Transferring a file which size: 78788 GB 0 MB 0 KB 0 B.");
    assert_eq!(logger.0[1], "Transferring a file which size: 0 GB 2 MB 0 KB 0 B.");
    assert_eq!(logger.0[2], "Transferring a file which size: 0 GB 0 MB 0 KB 523 B.");
    assert_eq!(logger.0[3], "Transferring a file which size: 0 GB 0 MB 287 KB 0 B.");
}