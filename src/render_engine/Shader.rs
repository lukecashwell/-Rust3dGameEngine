#[allow(unused)]
pub struct Shader {
	pub program_id: u32,
	pub fragment_id: u32,
	pub vertex_id: u32
}

#[allow(unused, non_snake_case)]
impl Shader {
	
	pub fn new(vertex_program: &str, fragment_program: &str) -> Self {
		let id: u32;
		let vertID: u32;
		let fragID: u32;
		unsafe {
			id = gl::CreateProgram();
			vertID = gl::CreateShader(gl::VERTEX_SHADER);
			fragID = gl::CreateShader(gl::FRAGMENT_SHADER);
		
			gl::ShaderSource(vertID, 1, &(CString::new(vertex_program).unwrap().as_ptr()), std::ptr::null());
			gl::ShaderSource(fragID, 1, &(CString::new(fragment_program).unwrap().as_ptr()), std::ptr::null());
			
			gl::CompileShader(vertID);
			let mut status = 0;
			gl::GetShaderiv(vertID, gl::COMPILE_STATUS, &mut status);
			if status == 0 {
				println!("[Shader] Unable to compile vertex shader: ");
				let mut length: i32 = 0;
				let mut report_buffer = [0 as i8; 512];
				gl::GetShaderInfoLog(vertID, 512, &mut length,  report_buffer.as_mut_ptr() as *mut i8);
				println!("{}", length as usize);
				for i in 1..length {
					print!("{}", report_buffer[i as usize] as u8 as char);
				}
				print!("\n");
			}


			gl::CompileShader(fragID);
			status = 0; 
			gl::GetShaderiv(fragID, gl::COMPILE_STATUS, &mut status);
			if status == 0 {
				println!("[Shader] Unable to compile fragment shader: ");
				let mut length: i32 = 0;
				let mut report_buffer = [0 as i8; 512];
				gl::GetShaderInfoLog(fragID, 512, &mut length,  report_buffer.as_mut_ptr() as *mut i8);
				println!("{}", length as usize);
				for i in 1..length {
					print!("{}", report_buffer[i as usize] as u8 as char);
				}
				print!("\n");
			}
			
			gl::AttachShader(id, vertID);
			gl::AttachShader(id, fragID);
			gl::LinkProgram(id);
			gl::ValidateProgram(id);
		}
		Shader{program_id: id, fragment_id: fragID, vertex_id: vertID}
	}
	
	pub fn start(&self) {
		unsafe {
			gl::UseProgram(self.program_id);
		}
	}
	
	pub fn stop(&self) {
		unsafe {
			gl::UseProgram(0);
		}
	}
	
	pub fn clean_up(&self) {
		self.stop();
		unsafe {
			gl::DetachShader(self.program_id, self.vertex_id);
			gl::DetachShader(self.program_id, self.fragment_id);
			gl::DeleteShader(self.vertex_id);
			gl::DeleteShader(self.fragment_id);
			gl::DeleteProgram(self.program_id);
		}
	}
	
	
	
	#[allow(unused)]
	pub fn set_uniform_1f(uniform: i32, data: f32) {
		unsafe {
			gl::Uniform1f(uniform, data);
		}
	}
	
	#[allow(unused)]
	pub fn set_uniform_2f(uniform: i32, data_1: f32, data_2: f32) {	
		unsafe {
			gl::Uniform2f(uniform, data_1, data_2);
		}
	}
	
	#[allow(unused)]
	pub fn set_uniform_3f(uniform: i32, data_1: f32, data_2: f32, data_3: f32) {
		unsafe {
			gl::Uniform3f(uniform, data_1, data_2, data_3);
		}
	}
	
	#[allow(unused)]
	pub fn set_uniform_4f(uniform: i32, data_1: f32, data_2: f32, data_3: f32, data_4: f32) {
		unsafe {
			gl::Uniform4f(uniform, data_1, data_2, data_3, data_4);
		}
	}
	
	#[allow(unused)]
	pub fn set_uniform_mat4f(uniform: i32, matrix: &crate::math::Matrix) {
		unsafe {
			gl::UniformMatrix4fv(uniform, 1, gl::FALSE, matrix.get_data_ptr());
		}
	}
	
	#[allow(unused)]
	pub fn get_uniform_location(shader: u32, uniform: &str) -> i32 {
		unsafe {
			return gl::GetUniformLocation(shader, CString::new(uniform).unwrap().as_ptr());
		}
	}
	
}