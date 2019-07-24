pub struct MasterRenderer {
	static_shader: StaticShader,	
	entity_renderer: EntityRenderer,
	entities: HashMap<u32, Vec<u32>>,
	models: Vec<u32>,
	terrain_shader: TerrainShader,
	terrain_renderer: TerrainRenderer
}

impl MasterRenderer {
	
	pub fn create() -> Self {
		unsafe {
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::LEQUAL);
		}
		MasterRenderer::enable_culling();
		let matrix = Matrix::projection4f(0.1, 3000.0, 90.0, (display::HEIGHT/display::WIDTH) as f32);
		let static_shader = StaticShader::create();
		let entity_renderer = EntityRenderer::create(&static_shader, &matrix);
		
		let terrain_shader = TerrainShader::create();
		let terrain_renderer = TerrainRenderer::create(&terrain_shader, &matrix);
		
		Self{static_shader: static_shader,
			 entity_renderer: entity_renderer, 
			 entities: HashMap::new(),
		     models: Vec::new(),
			 terrain_shader: terrain_shader,
			 terrain_renderer: terrain_renderer
		 }
	}
	
	pub fn render(&mut self, light: &Light, camera: &Camera, entity_pointers: &Vec<&Entity>, terrains: &Vec<&Terrain>) {
		self.prepare();
		self.static_shader.start();
		self.static_shader.load_light(light);
		self.static_shader.load_camera_matrix(&(camera.create_matrix().unwrap()));
		self.entity_renderer.render(&self.entities, &self.models, entity_pointers, &self.static_shader);
		self.static_shader.stop();
		
		self.entities.clear();
		self.models.clear();		
		
		self.terrain_shader.start();
		self.terrain_shader.load_light(light);
		self.terrain_shader.load_camera_matrix(&(camera.create_matrix().unwrap()));
		self.terrain_renderer.render(&self.terrain_shader, terrains);
		self.terrain_shader.stop();
	
	}
	
	pub fn prossess_entity<'a>(&mut self, entity_pointers: &mut Vec<&'a Entity<'a>>, entity: &'a Entity<'a>) {
		let id = entity_pointers.len() as u32;
		entity_pointers.push(entity);
	
		let model_id;
		{
			let model = entity.get_textured_model();
			model_id = model.get_id().clone();
			self.models.push(model_id);
		}
		match self.entities.get(&model_id) {
			Some(batch) => {
				let mut b = batch.to_vec();
				b.push(id);
				self.entities.insert(model_id, b);
			}
			None => {
				let mut batch = Vec::new();
				batch.push(id);
				self.entities.insert(model_id, batch);
			}
		};
	}
	
	pub fn prepare(&self) {
		unsafe {
			gl::Flush();
			gl::ClearColor(0.5, 0.6, 1.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}
	
	pub fn clean_up(&self) {
		self.static_shader.clean_up();
		self.terrain_shader.clean_up();
	}
	
	pub fn enable_culling() {
		unsafe {
			gl::Enable(gl::CULL_FACE);
			gl::CullFace(gl::FRONT);
		}
	}
	
	pub fn disable_culling() {
		unsafe {
			gl::Disable(gl::CULL_FACE);
		}
	}
}