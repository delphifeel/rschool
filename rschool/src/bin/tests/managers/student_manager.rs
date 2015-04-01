use managers::StudentManager;

#[test]
fn student_manager_create() {
	let mut sm = StudentManager::new();
	sm.create("Student".to_string());
	let student_getted = sm.get_student("Student".to_string());
	assert!(student_getted.is_some());
	assert!(student_getted.unwrap().name.eq(&"Student".to_string()));	
}
