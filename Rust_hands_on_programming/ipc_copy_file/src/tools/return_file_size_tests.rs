use super::*;
use std::fs;
use fs::File;


struct ToolBoxes {}
impl ToolBox for ToolBoxes {}

#[test]
fn file_does_not_exist() {
    let file_name = "myFile.txt".to_string();
    let tb = ToolBoxes {};
    assert_eq!(tb.return_file_size(&file_name), Err(IpcErrors::FileMetadata("myFile.txt".to_string(), "No such file or directory (os error 2)".to_string())))
}

#[test]
fn file_exists() {
    let mut dir = env::temp_dir();
    let file_name = format!("temfile.txt");
    dir.push(file_name);
    {
        let _file = File::create(&dir).unwrap();
    }
        
    let tb = ToolBoxes {};
    assert_eq!(tb.return_file_size(&dir.to_str().unwrap().to_string()), Ok(0))
}