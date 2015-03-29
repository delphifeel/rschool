#![feature(core)]
mod models;
mod consts;
mod managers;
mod global_context;

use models::*;
use consts::*;
use managers::*;
use global_context::*;

use std::io;

fn main() {
	// Global variables
	let mut gc = GlobalContext::new();

	let mut input = "".to_string();
	let mut reader = io::stdin();
	while true {
		println!(">>");
		input = "".to_string();
		reader.read_line(&mut input);
		input = input.trim().to_string();
		if input.eq(&commands::EXIT) {
			break;	
		} else if input.eq(&commands::ADD_STUDENT) {
			gc.studentManager.create_from_input(&mut reader);
		} else if input.eq(&commands::ADD_SCHOOL) {
			gc.schoolManager.create_from_input(&mut reader);
		}	
	}
}
