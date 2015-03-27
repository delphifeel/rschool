use models::Student;
use std::io;

pub struct StudentManager {
	list: Option<Vec<Student>>
}

impl<'a> StudentManager {
	pub fn new() -> StudentManager {
		StudentManager{list: None}	
	}	

	fn get_list(&'a mut self) -> &'a mut Vec<Student> {
		if self.list.is_none() {
			self.list = Some(Vec::new());
		}
		return self.list.as_mut().unwrap();
	}

	pub fn create_from_input(&mut self, reader: &mut io::Stdin) {
		println!("Enter student name: ");
		let mut name = "".to_string();
		reader.read_line(&mut name);
		let student = Student::new(name.as_slice());
		println!("Student {} saved", student.name);
		self.save(student);			
	} 

	pub fn save(&mut self, student: Student) {
		self.get_list().push(student);
	}
}
