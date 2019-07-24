#[derive(PartialEq)]
pub struct Texture {
	pub width: u32,
	pub height: u32,
	id: u32,
	shine_damper: f32,
	reflectivity: f32,
	is_transparent: bool
}

#[allow(non_snake_case, unused)]
impl Texture {
	pub fn new(_path: &str, _info: &std::path::PathBuf, scale_type: i32) -> Self {
		let mut t: Texture = Texture::default();
		let path: std::string::String = _info.to_str().unwrap().to_owned() + "\\resources\\images\\" + _path;
		let file = File::open(path);
		if file.is_err() {
			println!("[Texture] unable to load: ~\\{}", _path);
		}else{
			let decoder = png::Decoder::new(file.unwrap());
			let (info, mut reader) = decoder.read_info().unwrap();
			let mut buf = vec![0; info.buffer_size()];
			reader.next_frame(&mut buf).unwrap();
		//	let mut flipped_buf = buf;
			let mut flipped_buf = vec![0; info.buffer_size()];
	
			for i in 0..info.height as usize{
				for j in 0..(info.width*4) as usize {
					flipped_buf[(j) + (info.height as usize - i - 1)*((info.width*4) as usize)] = buf[(j) + i*((info.width*4) as usize)];
				}
			}
			
			let mut id: u32 = 0;
			unsafe {
				gl::GenTextures(1, &mut id);
				gl::BindTexture(gl::TEXTURE_2D, id);
				gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, info.width as i32, info.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, flipped_buf.as_ptr() as *const std::ffi::c_void);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, scale_type);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, scale_type);
				gl::BindTexture(gl::TEXTURE_2D, 0);
			}
			t = Texture{width: info.width, height: info.height, id: id, reflectivity: 0.0, shine_damper: 1.0, is_transparent: false};
		}
		t
	}

	pub fn unknown_image() -> Self {
		
		let buf: Vec<u8> = vec![255, 255, 255, 255, 0, 255, 0, 255,
								0, 255, 0, 255, 255, 255, 255, 255];

		let mut id: u32 = 0;
		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 2, 2, 0, gl::RGBA, gl::UNSIGNED_BYTE, buf.as_ptr() as *const std::ffi::c_void);
			
			gl::BindTexture(gl::TEXTURE_2D, 0);
	    }
		Texture{width: 2, height: 2, id: id, reflectivity: 0.0, shine_damper: 1.0, is_transparent: false }
	}

	pub fn white_image() -> Self {
	
		let buf: Vec<u8> = vec![255, 255, 255, 255];

		let mut id: u32 = 0;
		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 1, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, buf.as_ptr() as *const std::ffi::c_void);
			
			gl::BindTexture(gl::TEXTURE_2D, 0);
	    }
		Texture{width: 2, height: 2, id: id, reflectivity: 0.0, shine_damper: 1.0, is_transparent: false }
	}
	
	pub fn set_reflectivity(&mut self, reflectivity: f32) -> &mut Self {
		self.reflectivity = reflectivity;
		self
	}
	
	pub fn set_shine_damper(&mut self, shine_damper: f32) -> &mut Self {
		self.shine_damper = shine_damper;
		self
	}
	
	pub fn get_reflectivity(&self) -> f32 {
		self.reflectivity
	}
	
	pub fn get_shine_damper(&self) -> f32 {
		self.shine_damper
	}
	
	pub fn is_transparent(&self) -> bool {
		self.is_transparent
	}
	
	pub fn set_transparency(&mut self, transparency: bool) {
		self.is_transparent = transparency;
	}	
	
	pub fn default() -> Self {
		Texture{width: 0, height: 0, id: 0, reflectivity: 0.0, shine_damper: 1.0, is_transparent: false}
	}

	pub fn clone(&self) -> Texture {
		return Texture{width: self.width, height: self.height, id: self.id, reflectivity: self.reflectivity, shine_damper: self.shine_damper, is_transparent: self.is_transparent};
	}
	
	pub fn bind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}

	pub fn unbind() {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
	}

}