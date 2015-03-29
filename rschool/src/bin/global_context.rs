use managers::*;

pub struct GlobalContext<'a> {
	pub studentManager: StudentManager,
	pub schoolManager: SchoolManager<'a>
}

impl<'a> GlobalContext<'a> {	
	pub fn new() -> GlobalContext<'a> {
		GlobalContext {
			studentManager: StudentManager::new(),
			schoolManager : SchoolManager::new()
		}
	}
}