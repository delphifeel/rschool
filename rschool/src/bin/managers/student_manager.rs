use models::Student;

pub struct StudentManager {
	list: Vec<Student>
}

impl StudentManager {
	pub fn new() -> StudentManager {
		StudentManager{list: Vec::new()}	
	}	

	pub fn create(&mut self, name: String) {
		let student = Student::new(name.trim());
		println!("Student {} saved", student.name);
		self.save(student);			
	} 

	pub fn save(&mut self, student: Student) {
		self.list.push(student);
	}

	pub fn get_student(&mut self, name: String) -> Option<Student> {
		for i in self.list.iter() {
			if i.name.eq(&name) {
				return Some(i.clone());
			}	
		}
		return None;
	}
}
