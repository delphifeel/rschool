use managers::*;

pub struct GlobalContext {
	pub student_manager: StudentManager,
	pub school_manager: SchoolManager
}

impl GlobalContext {	
	pub fn new() -> GlobalContext {
		GlobalContext {
			student_manager: StudentManager::new(),
			school_manager : SchoolManager::new()
		}
	}
}
