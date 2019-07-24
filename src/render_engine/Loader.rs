pub struct Loader {
	vaos: Vec<u32>,
	vbos: Vec<u32>,
	path: std::path::PathBuf
}

impl Loader {
	
	pub fn create() -> Self{
		Loader{vaos: Vec::new(), vbos: Vec::new(), path: std::env::current_dir().unwrap() }
	}
	
	pub fn load_obj_to_vao(&mut self, file: &str) -> RawModel {
		let mut v: Vec<f32> = Vec::new();
		let mut vt: Vec<f32>;
		let mut vn: Vec<f32>;
		let mut indecies: Vec<u32> = Vec::new();
		
		let mut obj = std::fs::read_to_string(&(self.path.to_str().unwrap().to_owned() + "\\resources\\models\\" + file)).expect("Error in opening file.");	
		let obj = unsafe { obj.as_mut_vec() };
		{
			let mut uvt: Vec<f32> = Vec::new();
			let mut uvn: Vec<f32> = Vec::new();
		
			let mut lines: Vec<Vec<String>> = Vec::new();
			let mut line: Vec<String> = Vec::new();
			let mut sub_line: Vec<u8> = Vec::new();
			for i in 0..obj.len() {
				let chr = obj[i] as char;
				if chr == '\n' {
					if sub_line.len() > 0 {
						line.push(std::str::from_utf8(sub_line.clone().as_slice()).unwrap().to_owned().replace("\r", ""));
						sub_line = Vec::new();
						lines.push(line);
						line = Vec::new();
					}
					continue;
				}
				if chr != ' ' {
					if chr != '\r' {
						sub_line.push(obj[i]);	
					}
				} else {
					if sub_line.len() > 0 {
						line.push(std::str::from_utf8(&sub_line.as_slice()).unwrap().to_owned());
						sub_line = Vec::new();	
					}
				}
			}
			for l in lines.iter() {
				if l.len() > 0 {
					if l[0] == "v"  {
						v.push(l[1].parse::<f32>().unwrap());
						v.push(-l[2].parse::<f32>().unwrap());
						v.push(l[3].parse::<f32>().unwrap());
					}
					if l[0] == "vt"  {
						uvt.push(l[1].parse::<f32>().unwrap());
						uvt.push(l[2].parse::<f32>().unwrap());
					}
					if l[0] == "vn"  {
						uvn.push(l[1].parse::<f32>().unwrap());
						uvn.push(l[2].parse::<f32>().unwrap());
						uvn.push(l[3].parse::<f32>().unwrap());
					}
				}
			}
			vt = vec![0.0;  v.len()/3*2];
			vn = vec![0.0; v.len()];
			for l in lines.iter() {
				if l[0] == "f" {
					if l.len() == 4 {
						for i in 1..4 {
							let vinfo = Loader::parse_slash_array(&l[i]);
							if vinfo[1] != 0 {
								vt[vinfo[0] as usize * 2 + 0 - 2] = uvt[vinfo[1] as usize * 2 + 0 - 2];
								vt[vinfo[0] as usize * 2 + 1 - 2] = uvt[vinfo[1] as usize * 2 + 1 - 2];	
							}
							if uvn.len() > 0 {
								vn[vinfo[0] as usize * 3 + 0 - 3] = uvn[vinfo[2] as usize * 3 + 0 - 3];
								vn[vinfo[0] as usize * 3 + 1 - 3] = uvn[vinfo[2] as usize * 3 + 1 - 3];
								vn[vinfo[0] as usize * 3 + 2 - 3] = uvn[vinfo[2] as usize * 3 + 2 - 3];
							}
							indecies.push(vinfo[0] - 1);							
						}
					}
					if l.len() >= 5 {
						let mut info = Vec::new();
						for i in 1..5 {
							let vinfo = Loader::parse_slash_array(&l[i]);
							if vinfo[1] != 0 {
								vt[vinfo[0] as usize * 2 + 0 - 2] = uvt[vinfo[1] as usize * 2 + 0 - 2];
								vt[vinfo[0] as usize * 2 + 1 - 2] = uvt[vinfo[1] as usize * 2 + 1 - 2];	
							}
							if uvn.len() > 0 {
								vn[vinfo[0] as usize * 3 + 0 - 3] = uvn[vinfo[2] as usize * 3 + 0 - 3];
								vn[vinfo[0] as usize * 3 + 1 - 3] = uvn[vinfo[2] as usize * 3 + 1 - 3];
								vn[vinfo[0] as usize * 3 + 2 - 3] = uvn[vinfo[2] as usize * 3 + 2 - 3];
							}
							info.push(vinfo);
						}
						indecies.push(info[0][0] - 1);
						indecies.push(info[1][0] - 1);
						indecies.push(info[3][0] - 1);
						indecies.push(info[1][0] - 1);
						indecies.push(info[2][0] - 1);
						indecies.push(info[3][0] - 1);
					}
				}
			}
		}
		
		self.load_to_vao(&v, &indecies, &vn, &vt)
	}
	
	fn parse_slash_array(text: &String) -> Vec<u32> {
		let char_array = text.as_str().as_bytes();
		let mut ret = Vec::new();
		let mut number = Vec::new();
		for i in 0..char_array.len() {
			let chr = char_array[i] as char;
			if chr != '/' {
				number.push(char_array[i]);
			} else {
				let num = std::str::from_utf8(&number.as_slice()).unwrap().to_owned();
				if num.len() > 0 {
					ret.push(num.parse::<u32>().unwrap());
					number = Vec::new();
				} else {
					ret.push(0);
				}
			}
		}
		let num = std::str::from_utf8(&number.as_slice()).unwrap().to_owned().replace("\r", "");
		ret.push(num.parse::<u32>().unwrap());
		ret
	}
	
	
	pub fn load_to_vao(&mut self, positions: &Vec<f32>, indices: &Vec<u32>, normals: &Vec<f32>, texture_coordinates: &Vec<f32>) -> RawModel {
		let vao = self.create_vao();
		self.bind_indices_buffer(indices);
		self.store_data_in_attrib_list(0, 3, positions);
		self.store_data_in_attrib_list(1, 2, texture_coordinates);
		self.store_data_in_attrib_list(2, 3, normals);
		self.unbind_vao();
		self.vaos.push(vao);
		return RawModel::new(vao, indices.len() as i32);
	}
	
	fn create_vao(&self) -> u32 {
		let mut vao = 0; 
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
		}
		return vao;
	}
	
	fn store_data_in_attrib_list(&mut self, attrib_number: u32, chunk_size: i32, data: &Vec<f32>) {
		unsafe {
			let mut vbo = 0;
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(gl::ARRAY_BUFFER, data.len() as isize * 4, data.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
			gl::VertexAttribPointer(attrib_number, chunk_size, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			self.vbos.push(vbo);
		}
	}
	
	fn unbind_vao(&self) {
		unsafe {
			gl::BindVertexArray(0);
		}
	}
	
	fn bind_indices_buffer(&mut self, indices: &Vec<u32>) {
		unsafe {
			let mut vbo = 0;
			gl::GenBuffers(1, &mut vbo);
			self.vbos.push(vbo);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices.len() as isize * 4, indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
		}
	}
	
	pub fn clean_up(&mut self) {
		unsafe {
			gl::DeleteVertexArrays(self.vaos.len() as i32, self.vaos.as_ptr() as *const u32);
			gl::DeleteBuffers(self.vbos.len() as i32, self.vbos.as_ptr() as *const u32);
		}
	}
}

