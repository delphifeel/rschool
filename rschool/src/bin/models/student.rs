#[derive(Clone)]
pub struct Student {
	pub name: String	
}

impl Student {
	pub fn new(name: &str) -> Student {
		Student{name: name.to_string()}
	}
}
