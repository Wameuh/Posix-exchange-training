use super::*;
use fs::File;

struct ToolBoxes {}

impl ToolBox for ToolBoxes {}

#[test]
fn incorrect_file_path() {
    let file_name_too_long = (0..4097).map(|_| "X").collect::<String>();
    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_exist(&file_name_too_long), Err(IpcErrors::FileNameTooLong(file_name_too_long)))
}

#[test]
fn file_does_not_exist() {
    let file_name = "myFile.txt".to_string();
    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_exist(&file_name), Err(IpcErrors::FileDoestNotExist(file_name)))
}

#[test]
fn file_exist() {
    let mut dir = env::temp_dir();
    let file_name = format!("temfile.txt");
    dir.push(file_name);
    let _file = File::create(&dir).unwrap();
    
    
    let tb = ToolBoxes{};

    assert_eq!(tb.check_file_exist(&dir.to_str().unwrap().to_string()), Ok(()))
}
