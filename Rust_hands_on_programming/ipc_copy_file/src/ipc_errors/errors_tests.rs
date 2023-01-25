use super::*;
use itertools::Itertools;

#[test]
fn ipc_errors_to_string() {
    assert_eq!(IpcErrors::FileNameTooLong("test".to_string()).to_string(), "Filepath provided is incorrect. The name of the file is too long: test");
    assert_eq!(IpcErrors::FilePathTooLong{currentdir: "test".to_string(), givenpath: "test".to_string()}.to_string(), "Filepath provided is incorrect. The path to the file is too long:. Considering the current directory: test, and the path given: test");
    assert_eq!(IpcErrors::FilePathAbsTooLong{givenpath: "test".to_string()}.to_string(), "Filepath provided is incorrect. The path to the file is too long:. The path given: test");
    assert_eq!(IpcErrors::WrongFileName("test".to_string()).to_string(), "Filepath provided is incorrect. The name of the file is incorrect: test");
    assert_eq!(IpcErrors::CurrentDir().to_string(), "Error trying to get the current directory: it does not exist or there are insufficient permissions.");
    assert_eq!(IpcErrors::FileDoestNotExist("test".to_string()).to_string(), "Error the file provided does not exists. Filepath provided: test");
    assert_eq!(IpcErrors::FileMetadata("test".to_string(),"test".to_string()).to_string(), "Error while getting metadata of the file. Filepath provided: test. Error: test");
    assert_eq!(IpcErrors::OpeningFile("test".to_string(),"test".to_string()).to_string(), "Error while opening the file. Filepath provided: test. Error: test");
    assert_eq!(IpcErrors::SystemStats("/path/does/not/exist".to_string(), "ENOENT: No such file or directory".to_string()).to_string(), "Error when getting statsvfs in the path /path/does/not/exist, error: ENOENT: No such file or directory");
    assert_eq!(IpcErrors::NotEnoughSpace(0,0).to_string(), "Error not enough size in the system to copy the file. Available space: 0, space needed: 0");
    assert_eq!(IpcErrors::SameFileAndMethod(std::path::Path::new("test").to_path_buf(),std::path::Path::new("test2").to_path_buf()).to_string(), "Error, the file choosen and the IpcMethod name are the same. File: test, ipc method path: test2");

}

#[test]
fn ipc_errors_eq() {
    let error_list = [
        IpcErrors::FileNameTooLong("test".to_string()),
        IpcErrors::FilePathTooLong{currentdir: "test".to_string(), givenpath: "test".to_string()},
        IpcErrors::FilePathAbsTooLong{givenpath: "test".to_string()},
        IpcErrors::WrongFileName("test".to_string()),
        IpcErrors::CurrentDir(),
        IpcErrors::FileDoestNotExist("test".to_string()),
        IpcErrors::FileMetadata("test".to_string(),"test".to_string()),
        IpcErrors::OpeningFile("test".to_string(),"test".to_string()),
        IpcErrors::NotEnoughSpace(0,0),
        IpcErrors::SystemStats("/".to_string(),"err".to_string()),
        IpcErrors::SameFileAndMethod(std::path::Path::new("test").to_path_buf(),std::path::Path::new("test2").to_path_buf()),
    ];
    assert_eq!(IpcErrors::FileNameTooLong("test".to_string()), error_list[0]);
    assert_eq!(IpcErrors::FilePathTooLong{currentdir: "test".to_string(), givenpath: "test".to_string()}, error_list[1]);
    assert_eq!(IpcErrors::FilePathAbsTooLong{givenpath: "test".to_string()}, error_list[2]);
    assert_eq!(IpcErrors::WrongFileName("test".to_string()), error_list[3]);
    assert_eq!(IpcErrors::CurrentDir(), error_list[4]);
    assert_eq!(IpcErrors::FileDoestNotExist("test".to_string()), error_list[5]);
    assert_eq!(IpcErrors::FileMetadata("test".to_string(),"test".to_string()), error_list[6]);
    assert_eq!(IpcErrors::OpeningFile("test".to_string(),"test".to_string()), error_list[7]);
    assert_eq!(IpcErrors::NotEnoughSpace(0,0), error_list[8]);
    assert_eq!(IpcErrors::SystemStats("/".to_string(),"err".to_string()), error_list[9]);
    assert_eq!(IpcErrors::SameFileAndMethod(std::path::Path::new("test").to_path_buf(),std::path::Path::new("test2").to_path_buf()), error_list[10]);


    let it = (1..5).combinations(3);

    for combination in it {
        assert_ne!(combination[0], combination[1]);
    }
}

#[test]
fn ipc_errors_fmt() {
    let err1 = IpcErrors::FileNameTooLong("test".to_string());
    let err2 = IpcErrors::FilePathTooLong{currentdir: "test".to_string(), givenpath: "test".to_string()};
    let err3 = IpcErrors::FilePathAbsTooLong{givenpath: "test".to_string()};
    let err4 = IpcErrors::WrongFileName("test".to_string());
    let err5 = IpcErrors::CurrentDir();
    let err6 = IpcErrors::FileDoestNotExist("test".to_string());
    let err7 = IpcErrors::FileMetadata("test".to_string(),"test".to_string());
    let err8 = IpcErrors::OpeningFile("test".to_string(),"test".to_string());
    let err9 = IpcErrors::NotEnoughSpace(0,0);
    let err10 = IpcErrors::SystemStats("/".to_string(),"err".to_string());
    let err11 = IpcErrors::SameFileAndMethod(std::path::Path::new("test").to_path_buf(),std::path::Path::new("test2").to_path_buf());

    assert_eq!(format!("{err1:?}"), "FileNameTooLong(\"test\")");
    assert_eq!(format!("{err2:?}"), "FilePathTooLong { currentdir: \"test\", givenpath: \"test\" }");
    assert_eq!(format!("{err3:?}"), "FilePathAbsTooLong { givenpath: \"test\" }");
    assert_eq!(format!("{err4:?}"), "WrongFileName(\"test\")");
    assert_eq!(format!("{err5:?}"), "CurrentDir");
    assert_eq!(format!("{err6:?}"), "FileDoestNotExist(\"test\")");
    assert_eq!(format!("{err7:?}"), "FileMetadata(\"test\", \"test\")");
    assert_eq!(format!("{err8:?}"), "OpeningFile(\"test\", \"test\")");
    assert_eq!(format!("{err9:?}"), "NotEnoughSpace(0, 0)");
    assert_eq!(format!("{err10:?}"), "SystemStats(\"/\", \"err\")");
    assert_eq!(format!("{err11:?}"), "SameFileAndMethod(\"test\", \"test2\")");
}