use models::*;

pub struct SchoolManager {
	list: Vec<School>
}

impl<'b> SchoolManager {
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
	
	pub fn get_school<'a>(&'a mut self, name: String) -> Option<&'a mut &'a School> {
		let result: Option<&mut &School> = None;
		for ref mut i in self.list.iter() {
			if i.name.eq(&name) {
				let result = i;
				break;
			}
		}
		return result;
	}

	pub fn add_student_to_school(&mut self, student: Student, school_name: String) {
		let school_opt = self.get_school(school_name);
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
