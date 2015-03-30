use models::*;

pub struct SchoolManager {
	list: Vec<School>
}

impl SchoolManager {
	pub fn new() -> SchoolManager {
		SchoolManager{list: Vec::new()}	
	}	

	pub fn create(&mut self, name: String) {
		let school = School::new(name.trim());
		println!("School {} saved", school.name);
		self.save(school);			
	} 

	pub fn save(&mut self, school: School) {
		self.list.push(school);
	}
	
	pub fn get_school(&mut self, name: String) -> Option<School> {
		for i in self.list.iter() {
			if i.name.eq(&name) {
				return Some(i.clone());
			}
		}
		return None;
	}
}
