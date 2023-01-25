use std::env;
use std::process;
use ipc_copy_file::arg_handler::start;

fn main() {
    let mut ostream: Box<dyn ipc_copy_file::tools::Logger> = Box::new(ipc_copy_file::tools::StdoutLogger); //print
    
    match start(env::args().collect(), &mut ostream) {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1)
    }
}
