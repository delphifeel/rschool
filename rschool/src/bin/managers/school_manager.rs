use models::*;
use std::io;

pub struct SchoolManager<'a> {
	list: Vec<School<'a>>
}

impl<'a> SchoolManager<'a> {
	pub fn new() -> SchoolManager<'a> {
		SchoolManager{list: Vec::new()}	
	}	

	pub fn create_from_input(&mut self, reader: &mut io::Stdin) {
		println!("Enter school name: ");
		let mut name = "".to_string();
		reader.read_line(&mut name);
		let school = School::new(name.trim());
		println!("School {} saved", school.name);
		self.save(school);			
	} 

	pub fn save(&mut self, school: School<'a>) {
		self.list.push(school);
	}
	
	pub fn get_school(&self, name: String) -> Option<&School> {
		for i in self.list.iter() {
			if i.name.eq(&name) {
				return Some(i);
			}
		}
		return None;
	}
}
