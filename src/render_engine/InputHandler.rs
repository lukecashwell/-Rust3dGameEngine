pub struct InputHandler {
	pub key_hash_is_pressed: HashMap<glutin::VirtualKeyCode, bool>, 
	pub key_hash_first_press: HashMap<glutin::VirtualKeyCode, bool> 
}

#[allow(unused)]
impl InputHandler {
	pub fn new() -> Self {
		InputHandler{key_hash_is_pressed: HashMap::new(), key_hash_first_press: HashMap::new() }
	}
	
	pub fn key_pressed(&self, key: glutin::VirtualKeyCode) -> bool {
		let mut ret: bool = false;
		match self.key_hash_is_pressed.get(&key) {
			Some(result) => {
				ret = *result;
			},
			None =>{}
		}	
		return ret;
	}

	pub fn key_click(&mut self, key: glutin::VirtualKeyCode) -> bool {
		let mut ret: bool = false;
		match self.key_hash_first_press.get(&key) {
			Some(result) => {
				ret = *result;
			},
			None => {}
		}	
		if ret {
			self.key_hash_first_press.insert(key, false);
		}
		return ret;
	}
}