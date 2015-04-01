use models::Student;

#[test]
fn student_new() {
	let student = Student::new("Bob");
	assert!(student.name.eq(&"Bob".to_string()));
}
