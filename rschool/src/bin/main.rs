#![allow(unused_features)]
#![feature(core)]
mod models;
mod consts;
mod managers;
mod input_context;
mod global_context;
#[cfg(test)]
mod tests;

use consts::*;
use global_context::*;
use input_context::InputContext;

#[allow(dead_code)]
fn main() {
	// Global variables
	let mut gc = GlobalContext::new();
	// Input manager
	let mut ic = InputContext::new();

	let mut input;
	loop {
		println!(">>");
		input = "".to_string();
		ic.read(&mut input);
		input = input.trim().to_string();
		if input.eq(&commands::EXIT) {
			break;	
		} else if input.eq(&commands::ADD_STUDENT) {
			let student_name = ic.read_student_name();
			gc.student_manager.create(student_name);
		} else if input.eq(&commands::ADD_SCHOOL) {
			let school_name = ic.read_school_name();
			gc.school_manager.create(school_name);
		} else if input.eq(&commands::ADD_STUDENT_TO_SCHOOL) {
			let student_name = ic.read_student_name();
			let school_name = ic.read_school_name();
			let student_option = gc.student_manager.get_student(student_name);
			if student_option.is_none() {
				println!("Student not found");
			} else {
				let student = student_option.unwrap();
				gc.school_manager.add_student_to_school(student, school_name);
			}
		}	
	}
}
