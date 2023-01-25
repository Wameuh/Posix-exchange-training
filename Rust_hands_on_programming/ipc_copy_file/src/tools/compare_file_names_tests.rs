use super::*;


struct ToolBoxes {}

impl ToolBox for ToolBoxes {}

#[test]
fn current_dir_fail() {
    let file1 = String::from("sss/file2.ext");
    let file2 = String::from("sss/file2.ext");

    let current_dir = env::current_dir().unwrap();
    fs::create_dir("tempsfolder").unwrap();
    std::env::set_current_dir("tempsfolder").unwrap();
    fs::remove_dir("../tempsfolder").unwrap();


    let tb = ToolBoxes{};
    assert_eq!(tb.compare_file_names(&file1, &file2), Err(IpcErrors::CurrentDir()));

    std::env::set_current_dir(current_dir).unwrap();
}

#[test]
fn file_path_1_relative_2_relative() {
    
    let file1 = String::from("sss/file.ext");
    let file2 = String::from("sss/file.ext");
    let file3 = String::from("sss/files.ext");


    let mut path1 = env::current_dir().unwrap();
    path1.push(&file1);
    let mut path2 = env::current_dir().unwrap();
    path2.push(&file2);
    let mut path3 = env::current_dir().unwrap();
    path3.push(&file3);
    


    let tb = ToolBoxes{};
    assert_eq!(tb.compare_file_names(&file1, &file2), Err(IpcErrors::SameFileAndMethod(path1, path2)));
    assert_eq!(tb.compare_file_names(&file1, &file3), Ok(()));
    assert_eq!(tb.compare_file_names(&file2, &file3), Ok(()));

}

#[test]
fn file_path_1_absolute_2_relative() {
    
    let mut file1 = String::from("sss/file.ext");
    let file2 = String::from("sss/file.ext");
    let file3 = String::from("sss/files.ext");


    let mut path1 = env::current_dir().unwrap();
    path1.push(&file1);
    file1 = path1.to_str().unwrap().to_string();
    let mut path2 = env::current_dir().unwrap();
    path2.push(&file2);
    let mut path3 = env::current_dir().unwrap();
    path3.push(&file3);
    


    let tb = ToolBoxes{};
    assert_eq!(tb.compare_file_names(&file1, &file2), Err(IpcErrors::SameFileAndMethod(path1, path2)));
    assert_eq!(tb.compare_file_names(&file1, &file3), Ok(()));
}

#[test]
fn file_path_2_absolute_1_relative() {
    
    let mut file1 = String::from("sss/file.ext");
    let file2 = String::from("sss/file.ext");
    let file3 = String::from("sss/files.ext");


    let mut path1 = env::current_dir().unwrap();
    path1.push(&file1);
    file1 = path1.to_str().unwrap().to_string();
    let mut path2 = env::current_dir().unwrap();
    path2.push(&file2);
    let mut path3 = env::current_dir().unwrap();
    path3.push(&file3);
    


    let tb = ToolBoxes{};
    assert_eq!(tb.compare_file_names(&file2, &file1), Err(IpcErrors::SameFileAndMethod(path1, path2)));
    assert_eq!(tb.compare_file_names(&file3, &file1), Ok(()));
}