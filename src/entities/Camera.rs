pub struct Camera {
	location: Vec3,
	pitch: f32,
	yaw: f32
} 
 
impl Camera {
	pub fn new(location: Vec3, pitch: f32, yaw: f32) -> Self{
		Camera{location: location, pitch: pitch, yaw: yaw}
	}
	
	pub fn get_location(&self) -> Vec3 {
		Vec3::new(self.location.x, self.location.y, self.location.z)
	}
	pub fn get_pitch(&self) -> f32 {
		self.pitch
	}
	
	pub fn get_yaw(&self) -> f32 {
		self.yaw
	}
	
	pub fn set_pitch(&mut self, pitch: f32) {
		self.pitch = pitch;
	}
	
	pub fn set_yaw(&mut self, yaw: f32) {
		self.yaw = yaw;
	}
	
	pub fn inc_pitch(&mut self, pitch: f32) {
		self.pitch += pitch;
	}
	
	pub fn inc_yaw(&mut self, yaw: f32) {
		self.yaw += yaw;
	}
	
	
	pub fn set_location(&mut self, location: &Vec3) {
		self.location = Vec3::new(location.x, location.y, location.z);
	}
	
	pub fn inc_location(&mut self, x: f32, y: f32, z: f32) {
		self.location.x -= x;
		self.location.y += y;
		self.location.z -= z;
	}
	
	pub fn update(&mut self, handler: &InputHandler, delta_time: f64) {
		unsafe {
			self.inc_pitch(clampf32(display::mouse_change_y * 100.0 * delta_time as f32, -10.0, 10.0));
			self.inc_yaw(clampf32(display::mouse_change_x  * 100.0 * delta_time as f32, -10.0, 10.0));
			self.set_pitch(clampf32(self.get_pitch(), 0.0, 180.0));
			display::mouse_change_x = 0.0;
			display::mouse_change_y = 0.0;
		}
		let speed = (200.0 * delta_time) as f32;
		if handler.key_pressed(glutin::VirtualKeyCode::W) {
			self.inc_location(-sind(self.yaw) * speed, 0.0, -cosd(self.yaw) * speed);
		} 
		if handler.key_pressed(glutin::VirtualKeyCode::S) {
			self.inc_location(sind(self.yaw) * speed, 0.0, cosd(self.yaw) * speed);
		} 
		if handler.key_pressed(glutin::VirtualKeyCode::A) {
			self.inc_location(cosd(self.yaw) * speed, 0.0, -sind(self.yaw) * speed);
		} 
		if handler.key_pressed(glutin::VirtualKeyCode::D) {
			self.inc_location(-cosd(self.yaw) * speed, 0.0, sind(self.yaw) * speed);
		} 
		if handler.key_pressed(glutin::VirtualKeyCode::LShift) {
			self.inc_location(0.0, -speed, 0.0);
		} 
		if handler.key_pressed(glutin::VirtualKeyCode::Space) {
			self.inc_location(0.0, speed, 0.0);
		}
	}
	
	pub fn create_matrix(&self) -> Option<Matrix> {
		let matrix = Matrix::create_camera_matrix(&Vec3::new(-(self.location.x), self.location.y, -(self.location.z)), self.pitch + 90.0, self.yaw).unwrap();
		Some(matrix)
	}
	
	pub fn apply_matrix(&self, shader: &StaticShader) {
		let matrix = Matrix::create_camera_matrix(&Vec3::new(-(self.location.x), self.location.y, -(self.location.z)), self.pitch + 90.0, self.yaw).unwrap();
		shader.shader.start();
		shader.load_camera_matrix(&matrix);
		shader.shader.stop();
	}
	
}