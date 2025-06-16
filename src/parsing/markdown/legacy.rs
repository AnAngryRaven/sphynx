
use std::iter::Peekable;

pub fn sanitise(contents: String) -> Option<String> {
	/*
		Input: String; MUST be valid UTF-8.
		Ouptut: String Option.

		Notes:
			CC01 - Inserts the appropriate level heading tag for the respective number of `#`.
				Outputs `#` there's only one
	*/

	let b = contents.into_bytes();
	let mut b_peek = b.iter().peekable();
	let mut out: Vec<u8> = Vec::new();
	//let mut counter = 0;
	let mut header: bool = false;

	let h1_beginning: Vec<u8> = vec![60,104,49,62];
	let h1_end: Vec<u8> = vec![60,47,104,49,62,13];
	let hashes: Vec<u8> = vec![35,35,35,35,35,35,35];


	loop {
		let g = match b_peek.next() {
			Some(w) => w,
			None => break
		};

		match g {
			13 => {
				if header {
					out.extend(&h1_end);
				}
				header = false;
			}
			35 => { //Hash check; CC01
				//TODO: Prevent mid-line `#` from being converted into headings.
				match match_heading(&mut b_peek)? {
					0 => out.push(*g),
					1 => out.extend(&h1_beginning),
					2 => todo!(),
					3 => todo!(),
					4 => todo!(),
					5 => todo!(),
					6 => todo!(),
					7 => { out.extend(&hashes); continue; },
					_ => println!("Aborting! Invalid heading value...")
				}
				header = true;
			},
			42 => println!("asterisk!"),
			60 => println!("less than!"),
			62 => println!("greater than!"),
			95 => println!("underscore!"),
			123 => println!("opening brace!"),
			125 => println!("closing brace!"),
			126 => println!("tilde!"),
			_ => {
				match b_peek.peek() {
					None => {
						out.push(*g);
						if header == true {
							out.extend(&h1_end);
						}
					},
					_ => out.push(*g)
				}
			}
		}
	}

	let out_str = String::from_utf8_lossy(&out).to_string();
	println!("{out_str}");
	Some(String::from_utf8_lossy(&out).to_string())
}

pub fn match_heading(bytes: &mut Peekable<std::slice::Iter<'_, u8>>) -> Option<u8> {
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
