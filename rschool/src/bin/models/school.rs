use models::Student;

pub struct School<'a> {
	pub name: String,
	pub students: Vec<&'a Student>
}

impl<'a> School<'a> {
	pub fn new(name: &str) -> School<'a> {
		School{name: name.to_string(), students: Vec::new()}
	}
}	