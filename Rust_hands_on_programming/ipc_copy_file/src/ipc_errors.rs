use thiserror;
use std::path;


#[derive(thiserror::Error, Debug, PartialEq)]
pub enum IpcErrors {
    #[error("Filepath provided is incorrect. The name of the file is too long: {0}")]
    FileNameTooLong(String),
    #[error("Filepath provided is incorrect. The path to the file is too long:. Considering the current directory: {currentdir}, and the path given: {givenpath}")]
    FilePathTooLong{
        currentdir: String,
        givenpath: String,
    },
    #[error("Filepath provided is incorrect. The path to the file is too long:. The path given: {givenpath}")]
    FilePathAbsTooLong{
        givenpath: String,
    },
    #[error("Filepath provided is incorrect. The name of the file is incorrect: {0}")]
    WrongFileName(String),
    #[error("Error trying to get the current directory: it does not exist or there are insufficient permissions.")]
    CurrentDir(),
    #[error("Error the file provided does not exists. Filepath provided: {0}")]
    FileDoestNotExist(String),
    #[error("Error while getting metadata of the file. Filepath provided: {0}. Error: {1}")]
    FileMetadata(String, String),
    #[error("Error while opening the file. Filepath provided: {0}. Error: {1}")]
    OpeningFile(String, String),
    #[error("Error not enough size in the system to copy the file. Available space: {1}, space needed: {0}")]
    NotEnoughSpace(u64, u64),
    #[error("Error when getting statsvfs in the path {0}, error: {1}")]
    SystemStats(String, String),
    #[error("Error, the file choosen and the IpcMethod name are the same. File: {0}, ipc method path: {1}")]
    SameFileAndMethod(path::PathBuf, path::PathBuf),

}


#[cfg(test)]
mod errors_tests;