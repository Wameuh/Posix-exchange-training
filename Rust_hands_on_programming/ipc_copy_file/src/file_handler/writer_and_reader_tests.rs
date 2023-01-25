use crate::tools::ToolBox;

use super::*;
struct MyWriter{}
impl Writer for MyWriter{}

struct MyReader{}
impl Reader for MyReader{}


#[test]
fn write_and_read() {

    let mut dir = std::env::temp_dir();
    let file_name = format!("temfile.txt");
    
    dir.push(file_name);
    let temp_file_path = dir.to_str().unwrap().to_string();


    let mut file_writer = fs::OpenOptions::new().write(true).open(temp_file_path.clone()).unwrap();
    let mut file_reader = fs::OpenOptions::new().read(true).open(temp_file_path).unwrap();

    let my_writer = MyWriter{};
    let my_reader = MyReader{};

    assert!(my_writer.write(b"This is a test\n", &mut file_writer).is_ok());

    let mut buffer = [0 as u8; 15];
    assert!(my_reader.read(&mut buffer, &mut file_reader).is_ok());
    assert_eq!(&buffer, b"This is a test\n")
}

use close_file::Closable;
#[test]
#[allow(unused)]
fn write_failure() {

    let mut dir = std::env::temp_dir();
    let file_name = format!("tempfile_write_failure2.txt");
    dir.push(file_name);
    let temp_file_path = dir.to_str().unwrap().to_string();

    let mut file_writer = fs::OpenOptions::new().write(true).truncate(true).create(true).open(temp_file_path.clone()).unwrap();

    let my_writer = MyWriter{};
    
    file_writer.close();
    file_writer = fs::OpenOptions::new().read(true).open(temp_file_path).unwrap();
    assert!(my_writer.write(b"This is a test\n", &mut file_writer).is_err());
}

struct ToolBoxes {}
impl ToolBox for ToolBoxes{}

#[test]
fn file_size() {

    let mut dir = std::env::temp_dir();
    let file_name = format!("tempfile_size.txt");
    dir.push(file_name);
    let temp_file_path = dir.to_str().unwrap().to_string();
    let mut file_writer = fs::OpenOptions::new().write(true).truncate(true).create(true).open(temp_file_path.clone()).unwrap();
    
    let my_writer = MyWriter{};
    assert!(my_writer.write(b"This is a test\n", &mut file_writer).is_ok());  

    let my_toolbox = ToolBoxes{};
    assert_eq!(my_toolbox.return_file_size(&temp_file_path),Ok(15));
}