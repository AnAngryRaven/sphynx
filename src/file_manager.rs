use std::{fs::{File, OpenOptions}, io::{ErrorKind, Result, Write}};

use crate::error_handling::file_error_handling;

/// Saves a file with the given name.
///
/// Automatically creates the file if it was not found. All other errors handled by `file_error_handling`.
pub fn save_file(contents: &String, name: impl Into<String>) {
    let name: String = name.into();
    let mut f = match OpenOptions::new().write(true).open(&name) {
		Ok(file) => {
			file
		}
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				_ = file_error_handling(File::create(&name));
				file_error_handling(OpenOptions::new().write(true).open(&name))
			},
			other_err => panic!("PANIC {}", other_err)
		}
	};
	f.set_len(0).unwrap();
	f.write_all(contents.as_bytes()).unwrap();
}

/// Opens a file with the given name, handling any errors with the built-in `file_error_handling` function.
pub fn open_file(name: impl Into<String>) -> File {
    let name: String = name.into();
    match OpenOptions::new().read(true).open(name) {
        Ok(file) => file,
        Err(err) => file_error_handling(Err(err))
    }
}

/// Checks for, then appends a `.html` file extension to the filename.
pub fn append_html(name: impl Into<String>) -> String {
    let name: String = name.into();
    match name.ends_with(".html") {
        true => name,
        false => name + ".html"
    }
}
