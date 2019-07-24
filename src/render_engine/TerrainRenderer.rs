
pub struct TerrainRenderer {
}

impl TerrainRenderer {
	pub fn create(shader: &TerrainShader, projection_matrix: &Matrix) -> Self {
		shader.start();
		shader.load_projection_matrix(projection_matrix);
		shader.stop();
		TerrainRenderer{}
	}
	
	pub fn render<'a>(&self, shader: &TerrainShader, terrains: &Vec<&'a Terrain>) {
		for terrain in terrains.iter() {
			self.prepare_terrain(terrain, shader);
			self.load_model_matrix(terrain, shader);	
			unsafe {
				gl::DrawElements(gl::TRIANGLES, terrain.get_raw_model().get_vertex_count(), gl::UNSIGNED_INT, std::ptr::null());
			}	
			self.unbindTerrain();
		}
	}
	
	fn prepare_terrain(&self, terrain: &Terrain, shader: &TerrainShader) {
		terrain.get_texture().bind();
		let model = terrain.get_raw_model();
		unsafe {
			gl::BindVertexArray(model.get_vao());
			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);
			gl::EnableVertexAttribArray(2);
			gl::ActiveTexture(gl::TEXTURE0);
			terrain.get_texture().bind();
			shader.load_texture(&(terrain.get_texture()));
			
		}
	}
	
	fn unbindTerrain(&self) {
		unsafe {
			gl::DisableVertexAttribArray(0);
			gl::DisableVertexAttribArray(1);
			gl::DisableVertexAttribArray(2);
			gl::BindVertexArray(0);	
		}
		Texture::unbind();
	}
	
	fn load_model_matrix(&self, terrain: &Terrain, shader: &TerrainShader) {
		let matrix1 = Matrix::create_transformation_matrix(&Vec3::new(terrain.get_x(), 0.0, terrain.get_z()), 0.0, 0.0, 0.0, 1.0).unwrap();
		shader.load_transformation_matrix(&matrix1);
	}
}