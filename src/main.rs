mod file_manager;
mod error_handling;
mod helpers;
mod tui;
mod parsing;

//use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::Result;
use std::env;
use crossterm::event::{read, KeyEvent, KeyCode, KeyModifiers};
use crossterm::event::Event::Key;

use crate::file_manager::open_file;
use crate::parsing::markdown::legacy;

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
    let mut f = open_file("test.html");
	let mut g: String = String::new();
	let _= f.read_to_string(&mut g);
	println!("{}", g);
	
	let mut input = String::new();
	
	loop {
		match read() {
			Ok(evnt) => {
				std::io::stdin().read_to_string(&mut input).unwrap();
				println!("GUH");
				match evnt {
					Key( KeyEvent { code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL, .. } ) => { println!("guh!"); break },
					Key( KeyEvent { code: KeyCode::Char('x'), modifiers: KeyModifiers::CONTROL, .. } ) => { println!("guhh!"); break },
					_ => ()
				}
			},
			Err(error) => panic!("{}", error)
		}
	}
	file_manager::save_file(&legacy::sanitise(input).ok_or(ErrorKind::Other)?, "test.html");
	Ok(())
}
