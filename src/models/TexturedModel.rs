static mut ModelCount: u32 = 0;

pub struct TexturedModel {
	raw_model: RawModel,
	texture: Texture,
	id: u32
}

impl TexturedModel {
	pub fn new(model: RawModel, texture: Texture) -> Self {
		unsafe {
			ModelCount += 1;
		}
		Self{raw_model: model, texture: texture, id: unsafe { ModelCount } }
	}
	
	pub fn get_raw_model(&self) -> &RawModel{
		&self.raw_model
	}
	
	pub fn get_texture(&self) -> &Texture{
		&self.texture
	}
	
	pub fn get_id(&self) -> u32 {
		self.id
	}
}
