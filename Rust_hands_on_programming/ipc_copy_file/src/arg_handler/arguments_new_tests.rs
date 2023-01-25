use super::*;

#[test]
fn no_arg() {
    let args: Vec<String> = vec![];

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::Other);
}

#[test]
fn arg_help() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("--help".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::Help);
}

#[test]
fn arg_h() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-h".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::Help);
}

#[test]
fn arg_pipe() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("--pipe".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcPipe);
}

#[test]
fn arg_p() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-p".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcPipe);
}

#[test]
fn arg_p_p_with_path() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-p".to_string());
    args.push("myPipe".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcPipe);
    assert_eq!(argument.method_path, "myPipe".to_string());
}


#[test]
fn arg_file_without_filepath() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("--file".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::FilePathMissing);
}

#[test]
fn arg_f_without_filepath() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::FilePathMissing);
}

#[test]
fn arg_f() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("myFilePath".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::MethodMissing);
    assert_eq!(argument.file_path, "myFilePath".to_string());
}


#[test]
fn arg_p_f() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-p".to_string());
    args.push("-f".to_string());
    args.push("myFilePath".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcPipe);
    assert_eq!(argument.file_path, "myFilePath".to_string());
}

#[test]
fn arg_f_p() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("myFilePath".to_string());
    args.push("-p".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcPipe);
    assert_eq!(argument.file_path, "myFilePath".to_string());
}


#[test]
fn arg_queue() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("--queue".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcQueue);
}

#[test]
fn arg_q() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-q".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcQueue);
}

#[test]
fn arg_q_queuepath() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-q".to_string());
    args.push("myQueue".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcQueue);
    assert_eq!(argument.method_path, "myQueue".to_string());
}

#[test]
fn arg_f_q() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("myFilePath".to_string());
    args.push("-q".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcQueue);
    assert_eq!(argument.file_path, "myFilePath".to_string());
}

#[test]
fn arg_shm() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("--shm".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcShm);
}

#[test]
fn arg_s() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-s".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcShm);
}


#[test]
fn arg_s_shmpath() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-s".to_string());
    args.push("myShm".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcShm);
    assert_eq!(argument.method_path, "myShm".to_string());
}


#[test]
fn arg_f_s() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("myFilePath".to_string());
    args.push("-s".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::IpcShm);
    assert_eq!(argument.file_path, "myFilePath".to_string());
}

#[test]
fn arg_f_fpm_s() {
    let mut args: Vec<String> = vec![];
    args.push("myProgram".to_string());
    args.push("-f".to_string());
    args.push("-s".to_string());

    let argument = Arguments::new(args);

    assert_eq!(argument.query, QueryType::FilePathMissing);
}