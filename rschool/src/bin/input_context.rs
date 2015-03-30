use std::io;

pub struct InputContext {
	reader: io::Stdin 
}

impl InputContext {
	pub fn new() -> InputContext {
		InputContext{reader: io::stdin()}
	}

	pub fn read(&mut self, data: &mut String) {
		self.reader.read_line(data);
	}

	pub fn read_student_name(&mut self) -> String {
		println!("Enter student name:");
		let mut data = "".to_string();
		self.reader.read_line(&mut data);
		return data;			
	}

	pub fn read_school_name(&mut self) -> String {
		println!("Enter school name:");
		let mut data = "".to_string();
		self.reader.read_line(&mut data);
		return data;
	}
}
