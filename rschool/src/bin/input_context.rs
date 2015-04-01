use std::io;

#[allow(dead_code)]
pub struct InputContext {
	reader: io::Stdin 
}

#[allow(dead_code)]
impl InputContext {
	#[allow(unused_variables)]
	fn check_result<T, E>(&self, value: Result<T, E>) {
		match value {
			Err(e) => {
				panic!("ERROR: Error reading data from input");
			},
			_ => {}
		}
	}	

	pub fn new() -> InputContext {
		InputContext{reader: io::stdin()}
	}

	pub fn read(&mut self, data: &mut String) {
		let result = self.reader.read_line(data);
		self.check_result(result);
	}

	pub fn read_student_name(&mut self) -> String {
		println!("Enter student name:");
		let mut data = "".to_string();
		let result = self.reader.read_line(&mut data);
		self.check_result(result);
		return data;			
	}

	pub fn read_school_name(&mut self) -> String {
		println!("Enter school name:");
		let mut data = "".to_string();
		let result = self.reader.read_line(&mut data);
		self.check_result(result);
		return data;
	}
}
