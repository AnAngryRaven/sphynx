use std::io::{Result, ErrorKind, Error};
use std::fs::File;

use crate::print_noline;



fn retry() {

}

pub fn file_error_handling(res: Result<File>) -> File {
    match res {
        Ok(file) => return file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => panic!(),
            ErrorKind::PermissionDenied => panic!(),
            ErrorKind::IsADirectory => panic!(),
            ErrorKind::ReadOnlyFilesystem => panic!(),
            ErrorKind::InvalidFilename => panic!(),
            ErrorKind::Interrupted => panic!(),
            _ => panic!("{}", err)
        }
    }
}

pub fn unhandleable_err(err: Error) {
    println!("An unhandled exception has occurred: {} \n################################", err);
    println!("IMPORTANT: YOUR WORK HAS NOT BEEN SAVED TO DISK!");
    print_noline!("Would you like to go back to the editor? [Y/n]: ");
}

pub fn catastrophic_err(err: Error) {
    eprintln!("An fatal exception has occurred: {}\n################################", err);
    println!("WARNING: Check the state of your work!");
    panic!();
}
