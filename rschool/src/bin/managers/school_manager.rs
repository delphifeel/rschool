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
	
	pub fn get_school(&self, name: String) -> Option<School> {
		for i in self.list.iter() {
			if i.name.eq(&name) {
				return Some(i.clone());
			}	
		}	
		return None;
	}

	pub fn get_school_mut<'a>(&'a mut self, name: String) -> Option<&'a mut School> {
		for i in self.list.iter_mut() {
			if i.name.eq(&name) {
				return Some(i);		
			}	
		}
		return None;
	}

	pub fn add_student_to_school<'a>(&'a mut self, student: Student, school_name: String) {
		let school_opt = self.get_school_mut(school_name);
		if school_opt.is_none() {
			println!("School not found");
			return;
		} 
		let mut school = school_opt.unwrap();
		
		for i in school.students.iter() {
			if i.name.eq(&student.name) {
				println!("Student already in this school");
				return;	
			}
		}

		school.students.push(student);
	}
}
