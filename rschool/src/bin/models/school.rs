use models::Student;

#[derive(Clone)]
pub struct School {
	pub name: String,
	pub students: Vec<Student>
}

impl School {
	pub fn new(name: &str) -> School {
		School{name: name.to_string(), students: Vec::new()}
	}
}	
