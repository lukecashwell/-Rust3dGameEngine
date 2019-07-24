#[allow(unused)]
pub struct TextureLoader {
	textures: HashMap<std::string::String, Texture>,
	texture_ids: Vec<u32>,
	unknown: Texture,
	texture_list: Vec<std::string::String>,
	scaling_list: Vec<i32>
}

#[allow(unused)]
impl TextureLoader {
	pub fn new() -> Self {
		TextureLoader{ textures: HashMap::new(), texture_ids: Vec::new(), unknown: Texture::unknown_image(), texture_list: Vec::new(), scaling_list: Vec::new() }
	}

	pub fn default() -> Self {
		TextureLoader{ textures: HashMap::new(), texture_ids: Vec::new(), unknown: Texture::default(), texture_list: Vec::new(), scaling_list: Vec::new() }
	}

	pub fn add(&mut self, _path: &str, scale_type: u32) {
		self.texture_list.push(_path.to_owned());
		self.scaling_list.push(scale_type as i32);
	}

	pub fn fetch(&self, _path: &str) -> &Texture {
		let mut tex: &Texture = &self.unknown;
		match self.textures.get(&_path.to_owned()) {
			Some(result) => {
				tex = result;
			}, 
			None => {}
		}
		return tex;
	}

	pub fn load_all(&mut self, _info: &std::path::PathBuf) {
		for i in 0..self.texture_list.len() {
			let t = &(self.texture_list[i]);
			let string: &str = t.as_str();
			let tex: Texture = Texture::new(string, _info, self.scaling_list[i]);
			if tex != Texture::default() {
				self.texture_ids.push(tex.id);
				self.textures.insert(string.to_owned(), tex);
			}
		}
	}
	
	pub fn clean_up(&self) {
		unsafe {
			gl::DeleteTextures(self.texture_ids.len() as i32, self.texture_ids.as_ptr())
		}
	}
}