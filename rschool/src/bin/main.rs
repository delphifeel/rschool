mod models;
mod consts;
mod managers;

use models::*;
use consts::*;
use managers::*;

use std::io;

fn main() {
	// init managers
	let mut studentManager = StudentManager::new();

	let mut input = "".to_string();
	let mut reader = io::stdin();
	while true {
		input = "".to_string();
		reader.read_line(&mut input);
		
		if input.eq(&commands::EXIT) {
			break;	
		} else if input.eq(&commands::ADD_STUDENT) {
			studentManager.create_from_input(&mut reader);
		}	
	}
}
