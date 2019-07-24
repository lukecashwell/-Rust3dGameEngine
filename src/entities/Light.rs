pub struct Light {
	location: Vec3, 
	color: Vec3,
	brightness: f32
}

impl Light {
	pub fn new(location: Vec3, color: Vec3, brightness: f32) -> Self {
		Light{location: location, color: color, brightness: brightness}
	}
	
	pub fn get_location(&self) -> &Vec3{
		&self.location
	}
	
	pub fn get_color(&self) -> &Vec3 {
		&self.color
	}
	
	pub fn get_brightness(&self) -> f32 {
		self.brightness
	}
	
	pub fn set_location(&mut self, location: Vec3) {
		self.location = location;
	}
	
	pub fn set_color(&mut self, color: Vec3) {
		self.color = color;
	}	
	
	pub fn set_brightness(&mut self, brightness: f32) {
		self.brightness = brightness;
	}
}