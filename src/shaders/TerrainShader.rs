pub struct TerrainShader {
	pub shader: Shader, 
	transformation_matrix: i32,
	projection_matrix: i32,
	camera_matrix: i32,
	light_location: i32,
	light_color: i32,
	light_brightness: i32,
	shine_damper: i32,
	reflectivity: i32
}

impl TerrainShader {
	
	pub fn create() -> Self {
		let shader = Shader::new(include_str!("terrain_vertex.glsl"), include_str!("terrain_fragment.glsl"));
		TerrainShader{
					transformation_matrix: 
							Shader::get_uniform_location(shader.program_id, "transformationMatrix"),
					projection_matrix: 
							Shader::get_uniform_location(shader.program_id, "projectionMatrix"),
					camera_matrix: 
							Shader::get_uniform_location(shader.program_id, "cameraMatrix"),
					light_location: 
							Shader::get_uniform_location(shader.program_id, "lightLocation"),
					light_color: 
							Shader::get_uniform_location(shader.program_id, "lightColor"),
					light_brightness: 
							Shader::get_uniform_location(shader.program_id, "lightBrightness"),
					shine_damper: 
							Shader::get_uniform_location(shader.program_id, "shineDamper"),
					reflectivity: 
							Shader::get_uniform_location(shader.program_id, "reflectivity"),
					 shader: shader }
	}
	
	pub fn start(&self) {
		self.shader.start();
	}
	
	pub fn stop(&self) {
		self.shader.stop();
	}
	
	pub fn clean_up(&self) {
		self.shader.clean_up();
	}
	
	pub fn load_transformation_matrix(&self, matrix: &Matrix) {
		Shader::set_uniform_mat4f(self.transformation_matrix, matrix);
	}
	
	pub fn load_projection_matrix(&self, matrix: &Matrix) {
		Shader::set_uniform_mat4f(self.projection_matrix, matrix);
	}
	
	pub fn load_camera_matrix(&self, matrix: &Matrix) {
		Shader::set_uniform_mat4f(self.camera_matrix, matrix);
	}
	
	pub fn load_light(&self, light: &Light) {
		Shader::set_uniform_3f(self.light_location, light.get_location().x, light.get_location().y, light.get_location().z);
		Shader::set_uniform_3f(self.light_color, light.get_color().x, light.get_color().y, light.get_color().z);
		Shader::set_uniform_1f(self.light_brightness, light.get_brightness());
	}
	
	pub fn load_texture(&self, texture: &Texture) {
		Shader::set_uniform_1f(self.reflectivity, texture.get_reflectivity());
		Shader::set_uniform_1f(self.shine_damper, texture.get_shine_damper());
	}
	
}