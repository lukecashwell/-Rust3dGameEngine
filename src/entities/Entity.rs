pub struct Entity<'a> {
	model: &'a TexturedModel,
	location: Vec3,
	rotation: Vec3,
}

impl<'a> Entity<'a>  {
	pub fn new(model: &'a TexturedModel, location: Vec3, rotation: Vec3) -> Self {
		Entity{model: model, location: location, rotation: rotation}
	}
	
	
	pub fn get_textured_model(&self) -> &TexturedModel{
		self.model
	}

	pub fn get_location(&self) -> &Vec3{
		&self.location
	}
	
	pub fn get_rotation(&self) -> &Vec3 {
		&self.rotation
	}
	
	pub fn set_location(&mut self, location: Vec3) {
		self.location = location;
	}
	
	pub fn set_rotation(&mut self, rotation: Vec3) {
		self.rotation = rotation;
	}
	
	pub fn inc_location(&mut self, x: f32, y: f32, z: f32) {
		self.location.x += x;
		self.location.y += y;
		self.location.z += z;
	}
	
	pub fn inc_rotation(&mut self, x: f32, y: f32, z: f32) {
		self.rotation.x += x;
		self.rotation.y += y;
		self.rotation.z += z;
	}
}