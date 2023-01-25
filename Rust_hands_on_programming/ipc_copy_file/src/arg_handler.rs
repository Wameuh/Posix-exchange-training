use crate::tools::{ToolBox, Logger};

#[derive(PartialEq, Debug)]
pub enum QueryType{
    IpcPipe,
    IpcQueue,
    IpcShm,
    Help,
    FilePathMissing, // file path is not provided even if -f is used
    MethodMissing, // file selected but no method
    Other, // default
}



pub struct Arguments {
    pub query: QueryType,
    pub file_path: String,
    pub method_path: String,
}

impl ToolBox for Arguments {}

impl Arguments {
/// new is for instanciate a new Argument structure. The function will go throught the arguments provided and indentify:
/// * the method choosen
/// * if there is a method path provided
/// * the path of the file provided
    pub fn new(args: Vec<String>) -> Arguments {
        let mut retval= Arguments {query: QueryType::Other, file_path: "".to_string(), method_path: "".to_string()};
      
        let mut args_iter = args.iter();
        let mut option = args_iter.next();
        
        if option == None {
            return retval;
        }

        option = args_iter.next();
        loop {
            if let Some(content) = option {
                if content == "-h" || content == "--help" {
                    retval.query = QueryType::Help;
                } else if content == "--pipe" || content == "-p" {
                    retval.query = QueryType::IpcPipe;

                    // Look if there is a method path provided
                    option = args_iter.next();
                    if let Some(content) = option {
                        if content.starts_with("-") {
                            continue;
                        } else {
                            retval.method_path = content.to_string();
                        }
                    }

                } else if content == "--queue" || content == "-q" {
                    retval.query = QueryType::IpcQueue;

                    // Look if there is a method path provided
                    option = args_iter.next();
                    if let Some(content) = option {
                        if content.starts_with("-") {
                            continue;
                        } else {
                            retval.method_path = content.to_string();
                        }
                    }
                } else if content == "--shm" || content == "-s" {
                    retval.query = QueryType::IpcShm;

                    // Look if there is a method path provided
                    option = args_iter.next();
                    if let Some(content) = option {
                        if content.starts_with("-") {
                            continue;
                        } else {
                            retval.method_path = content.to_string();
                        }
                    }
                } else if content == "--file" || content == "-f" {

                    option = args_iter.next();
                    if let Some(content) = option {
                        if content.starts_with("-") {
                            retval.query = QueryType::FilePathMissing;
                            break;
                        } else {
                            retval.file_path = content.to_string();

                            if retval.query == QueryType::Other {
                                retval.query = QueryType::MethodMissing;
                            } 
                        }
                    } else {
                        retval.query = QueryType::FilePathMissing;
                        break;
                    }
                } else { //argument do not fit to the pattern => will be ignored
                    break;
                }
            } else { //no more arguments
                break;
            }

            option = args_iter.next();
        }

        retval
    }

}


/// start function will pass the arguments throught Arugments::new() and give the corresponding response
pub fn start(args: Vec<String>, ostream: &mut Box<dyn Logger>) -> Result<(),  &'static str> {
    let args = Arguments::new(args);

    match args.query {
        QueryType::Help => {
            args.print_instruction(ostream);
            Ok(())
        }
        QueryType::IpcPipe => {
            ostream.print(&format_args!("Pipe is not implemented. Use -h or --help option to know which method is implemented."));
            Err("Pipe not implemented.")
        }
        QueryType::IpcQueue => {
            ostream.print(&format_args!("Message queue is not implemented. Use -h or --help option to know which method is implemented."));
            Err("Queue not implemented.")
        }
        QueryType::IpcShm => {
            ostream.print(&format_args!("Shared memory is not implemented. Use -h or --help option to know which method is implemented."));
            Err("Shm not implemented.")
        }
        QueryType::FilePathMissing => {
            ostream.print(&format_args!("Error, filepath is missing."));
            Err("Filepath missing.")
        }
        QueryType::MethodMissing => {
            ostream.print(&format_args!("Error, IPC method is missing."));
            Err("No method provided")
        }
        QueryType::Other => {
            ostream.print(&format_args!("Oups... Arguments provided are not recognized. Use -h or --help option to know how you can use the program."));
            Err("Wrong arguments")
        }
    }
}





#[cfg(test)]
mod arguments_new_tests;

#[cfg(test)]
mod start_tests;

#[cfg(test)]
mod query_type_tests;