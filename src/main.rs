#![allow(non_snake_case)]
//use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use crossterm::event::{read, KeyEvent, KeyCode, KeyModifiers};
use crossterm::event::Event::Key;

fn main() {
    let mut f = match OpenOptions::new().read(true).open("test.html") {
		Ok(file) => {
			file
		}
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				println!("GUH FILE NOT FOUND SRRY");
				let _ = File::create("test.html");
				OpenOptions::new().read(true).open("test.html".to_string()).unwrap()
			},
			other_err => panic!("PANIC {}", other_err)
		}
	};
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
	
	
	saveFile(sanitise(input));
}

fn saveFile(contents: String) {
	let mut f = match OpenOptions::new().write(true).open("test.html") {
		Ok(file) => {
			file
		}
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				println!("GUH FILE NOT FOUND SRRY");
				let _ = File::create("test.html");
				OpenOptions::new().write(true).open("test.html".to_string()).unwrap()
			},
			other_err => panic!("PANIC {}", other_err)
		}
	};
	f.set_len(0).unwrap();
	f.write_all(contents.as_bytes()).unwrap();
}

fn sanitise(contents: String) -> String {
	
	let b = contents.into_bytes();
	let mut out: Vec<u8> = Vec::new();
	//let mut counter = 0;
	let mut header: bool = false;
	
	let h1_beginning: Vec<u8> = vec![60,104,49,62];
	let h1_end: Vec<u8> = vec![60,47,104,49,62];
	
	for chars in b {
		match chars {
			10 => {
				if header{
					out.extend(&h1_end);
					header = false;
				}
			},
			42 => println!("asterisk!"),
			60 => println!("less than!"),
			62 => println!("greater than!"),
			95 => println!("underscore!"),
			123 => println!("opening brace!"),
			125 => println!("closing brace!"),
			126 => println!("tilde!"),
			_ => out.push(chars)
		}
		
	}
	String::from_utf8_lossy(&out).to_string()
}

fn matchHeading(bytes: &mut Peekable<std::slice::Iter<'_, u8>>) -> Option<u8> {
	/*
		Input: Peekable Iterator Vec of u8 that SHOULD be bytes.
		Output: u8 Option; Minimum 0, Maximum 7.
		
		TODO: Refactor so that variable numbers of `#` don't result in only 7 `#` from being inserted.
		
		Notes:
			CC01 - Only counts the heading level until 7. Any `#` past that isn't counted, and just results in the Iter being advanced.
				This makes sure that any future `#` aren't erroneously counted towards a heading.
	*/
	let mut heading_level: u8 = 1;
		
	loop {
		match bytes.peek()? {
			32 => { //Space check; Makes sure that a space after a string of too many `#` is preserved.
				if heading_level < 7 {
					bytes.next();
				}
				break;
			},
			35 => { //Hash check; CC01
				if heading_level < 7 {
					heading_level = heading_level + 1;
					bytes.next();
				}else {
					bytes.next();
				}
			},
			_ => { //Misc check; Prevents headings from being applied to hashtags.
				if heading_level < 7 {
					heading_level = 0;
					break;
				}
				heading_level = 7;
				break;
			}
		}
		
	}
	if heading_level > 7 {
		heading_level = 7; //Prevent heading_level from being greater than 7 accidentally.
	}
	Some(heading_level)
}