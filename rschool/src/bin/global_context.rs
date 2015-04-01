use managers::*;

#[allow(dead_code)]
pub struct GlobalContext {
	pub student_manager: StudentManager,
	pub school_manager: SchoolManager
}

#[allow(dead_code)]
impl GlobalContext {	
	pub fn new() -> GlobalContext {
		GlobalContext {
			student_manager: StudentManager::new(),
			school_manager : SchoolManager::new()
		}
	}
}
