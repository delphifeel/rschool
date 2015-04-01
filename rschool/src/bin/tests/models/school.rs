use models::{Student, School};

#[test]
fn school_new() {
    let mut school = School::new("School");
    let student = Student::new("Student");
    school.students.push(student);
    assert!(school.name.eq(&"School".to_string()));
    assert_eq!(school.students.len(), 1);
    assert!(school.students.get(0).unwrap().name.eq(&"Student".to_string()));
}

