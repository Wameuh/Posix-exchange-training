use super::*;

struct ToolBoxes {}

impl ToolBox for ToolBoxes {}

#[test]
fn filename_too_long() {
    let file_name_too_long = (0..4097).map(|_| "X").collect::<String>();
    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_path(&file_name_too_long), Err(IpcErrors::FileNameTooLong(file_name_too_long)))
}

#[test]
fn filename_incorrect() {
    let file_name_incorrect = String::from("safasf/..");
    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_path(&file_name_incorrect), Err(IpcErrors::WrongFileName(file_name_incorrect)))
}

#[test]
fn path_relative_too_long() {
    let mut file_path_too_long = (0..4097).map(|_| "X").collect::<String>();
    file_path_too_long.push_str("/file.ext");
    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_path(&file_path_too_long), Err(IpcErrors::FilePathTooLong{currentdir: env::current_dir().unwrap().to_string_lossy().to_string(), givenpath: file_path_too_long}))
}


#[test]
fn path_absolute_too_long() {
    let mut file_path_too_long = String::from("/");
    let long_path = (0..4097).map(|_| "X").collect::<String>();
    file_path_too_long.push_str(&long_path);
    file_path_too_long.push_str("/file.ext");

    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_path(&file_path_too_long), Err(IpcErrors::FilePathAbsTooLong{givenpath: file_path_too_long}))
}

#[test]
fn file_path_is_ok() {
    let file_path_too_long = String::from("sss/file.ext");

    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_path(&file_path_too_long), Ok(()))
}

use std::fs;
#[test]
fn current_dir_nok() {
    let file_path = String::from("sss/file.ext");

    let current_dir = env::current_dir().unwrap();
    fs::create_dir("tempsfolder").unwrap();
    std::env::set_current_dir("tempsfolder").unwrap();
    fs::remove_dir("../tempsfolder").unwrap();


    let tb = ToolBoxes{};
    assert_eq!(tb.check_file_path(&file_path), Err(IpcErrors::CurrentDir()));

    std::env::set_current_dir(current_dir).unwrap();
}
