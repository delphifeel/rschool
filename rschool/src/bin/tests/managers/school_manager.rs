use managers::SchoolManager;
use models::Student;

#[test]
fn school_manager_create() {
	let test_school_name = "School".to_string();
	let test_student = "Bob";	

	let mut sm = SchoolManager::new();
	sm.create(test_school_name.clone());
	let mut school_getted = sm.get_school(test_school_name.clone());
	assert!(school_getted.is_some());
	assert!(school_getted.unwrap().name.eq(&test_school_name));
	let mut student = Student::new(test_student);
	sm.add_student_to_school(student, test_school_name.clone());
	school_getted = sm.get_school(test_school_name.clone());
	let mut school = school_getted.unwrap(); 
	let student_opt = school.students.get(0);
	assert!(student_opt.is_some());
}
