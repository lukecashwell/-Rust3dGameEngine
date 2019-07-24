
pub struct EntityRenderer {
}

impl EntityRenderer {
	pub fn create(shader: &StaticShader, projection_matrix: &Matrix) -> Self {
		shader.start();
		shader.load_projection_matrix(projection_matrix);
		shader.stop();
		EntityRenderer{}
	}
	
	pub fn render(&self, entities: &HashMap<u32, Vec<u32>>, models: &Vec<u32>, entity_pointers: &Vec<&Entity>, shader: &StaticShader) {
		for model_id in models.iter() {
			let batch = entities.get(&(model_id)).unwrap();
			let model = entity_pointers[batch[0] as usize].get_textured_model();
			self.prepare_textured_model(model, shader);
			for point in batch.iter() {
				let entity = entity_pointers[*point as usize];
				self.prepare_instance(entity, shader);
				unsafe {
					gl::DrawElements(gl::TRIANGLES, model.get_raw_model().get_vertex_count(), gl::UNSIGNED_INT, std::ptr::null());
				}	
			}
			self.unbind_textured_model(model);
		}
	}
	
	fn prepare_textured_model(&self, textured_model: &TexturedModel, shader: &StaticShader) {
		textured_model.get_texture().bind();
		let model = textured_model.get_raw_model();
		unsafe {
			gl::BindVertexArray(model.get_vao());
			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);
			gl::EnableVertexAttribArray(2);
			gl::ActiveTexture(gl::TEXTURE0);
			textured_model.get_texture().bind();
			shader.load_texture(&(textured_model.get_texture()));
			if textured_model.get_texture().is_transparent() {
				MasterRenderer::disable_culling();
			}
		}
	}
	
	fn unbind_textured_model(&self, textured_model: &TexturedModel) {
		unsafe {
			gl::DisableVertexAttribArray(0);
			gl::DisableVertexAttribArray(1);
			gl::DisableVertexAttribArray(2);
			gl::BindVertexArray(0);	
			if textured_model.get_texture().is_transparent() {
				MasterRenderer::enable_culling();
			}
		}
		Texture::unbind();
	}
	
	fn prepare_instance(&self, entity: &Entity, shader: &StaticShader) {
		let rot = entity.get_rotation();
		let matrix1 = Matrix::create_transformation_matrix(entity.get_location(), rot.x, rot.y, rot.z, 1.0).unwrap();
		shader.load_transformation_matrix(&matrix1);
	}
}