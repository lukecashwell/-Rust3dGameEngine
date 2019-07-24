pub struct Terrain {
	model: RawModel,
	texture: Texture,
	x: f32,
	z: f32,
}

impl Terrain {
	const SIZE: f32 = 2000.0;
	pub const VERTEX_COUNT: usize = 100;
	const HEIGHT_MUL: f32 = 1000.0;
	const TEXTURE_MUL: f32 = 10.0;
	
	pub fn construct(loader: &mut Loader, heights: &Vec<f32>, x: i32, z: i32, texture: Texture) -> Self {
		let model = create_mesh(loader, heights);
		Self{model: model, x: x as f32* Terrain::SIZE, z: z as f32 * Terrain::SIZE, texture: texture}
	} 
	
	pub fn get_x(&self) -> f32 {
		self.x
	}
	
	pub fn get_z(&self) -> f32 {
		self.z
	}
	
	pub fn get_texture(&self) -> &Texture {
		&self.texture
	}
	
	pub fn get_raw_model(&self) -> &RawModel {
		&self.model
	}
}

fn create_mesh(loader: &mut Loader, heights: &Vec<f32>) -> RawModel {
	let local_size = Terrain::SIZE/(Terrain::VERTEX_COUNT - 1) as f32;
	let mut verticies: Vec<f32> = Vec::new();
	let mut texture_coords: Vec<f32> = Vec::new();
	for i in 0..Terrain::VERTEX_COUNT {
		for j in 0..Terrain::VERTEX_COUNT {
			verticies.push(i as f32 * local_size as f32);
			verticies.push(heights[i + j*Terrain::VERTEX_COUNT]*-Terrain::HEIGHT_MUL);
			verticies.push(j as f32 * local_size as f32);
			texture_coords.push(i as f32/Terrain::VERTEX_COUNT as f32*Terrain::TEXTURE_MUL);
			texture_coords.push(j as f32/Terrain::VERTEX_COUNT as f32*Terrain::TEXTURE_MUL);
		} 
	}
	let mut normals: Vec<f32> = vec![0.0;verticies.len()];
	for i in 0..verticies.len()/3 {
		normals[i*3 + 1] = 1.0; 
	}
	let mut indices: Vec<u32> = Vec::new();		
	for i in 0..(Terrain::VERTEX_COUNT - 1) {
		for j in 0..(Terrain::VERTEX_COUNT - 1) {
			indices.push((0 + i + j*Terrain::VERTEX_COUNT) as u32);
			indices.push((1 + i + j*Terrain::VERTEX_COUNT) as u32);
			indices.push((1 + i + (1 + j)*Terrain::VERTEX_COUNT) as u32);
			let index = i*3 + j*Terrain::VERTEX_COUNT*3;
			let v1 = Vec3::new(verticies[index],verticies[index + 1],verticies[index + 2]);
			let v2 = Vec3::new(verticies[index + 3],verticies[index + 4],verticies[index + 5]);
			let v3 = Vec3::new(verticies[index + 3 + Terrain::VERTEX_COUNT*3],verticies[index + 4 + Terrain::VERTEX_COUNT*3],verticies[index + 5 + Terrain::VERTEX_COUNT*3]);				
			let normal = Vec3::find_normal(&v1, &v2, &v3);
			normals[index] = normal.x;
			normals[index + 1] = normal.y;
			normals[index + 2] = normal.z;
			indices.push((0 + i + j*Terrain::VERTEX_COUNT) as u32);
			indices.push((1 + i + (1 + j)*Terrain::VERTEX_COUNT) as u32);
			indices.push((0 + i + (1 + j)*Terrain::VERTEX_COUNT) as u32);
		}
	}
	loader.load_to_vao(&verticies,&indices,&normals,&texture_coords)
}