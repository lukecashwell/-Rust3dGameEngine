mod math;
mod render_engine;
mod models;
mod textures;
mod shaders;
mod entities;
mod terrain;

use crate::render_engine::*;
use crate::shaders::*;
use crate::models::*;
use crate::textures::*;
use crate::math::*;
use crate::entities::*;
use crate::terrain::*;
use rand::prelude::*;

//--------REMOVE--------
use noise::{NoiseFn, Perlin};

//----------------------

fn main() {
	let path_root = std::env::current_dir().unwrap();
	
	let mut display = Display::create();
	let mut texture_loader = TextureLoader::new();
	
	//--REMOVE----------------
	let mut heights = Vec::new();
	{
		let path: std::string::String = path_root.to_str().unwrap().to_owned() + "\\resources\\maps\\map.png";
		let file = std::fs::File::open(path).expect("Unable to load MAPPPPPPP");
		let decoder = png::Decoder::new(file);
		let (info, mut reader) = decoder.read_info().unwrap();
		let mut buf = vec![0; info.buffer_size()];
		reader.next_frame(&mut buf).unwrap(); 
		for i in 0..10000 {
			heights.push(buf[i*4] as f32/255 as f32);
		}
	}
	/*let mh = 10;
	let mw = 10;
	let perlin = Perlin::new();
	for j in 0..mh*mw {
		let mut heightmap = Vec::new();
		for i in 0..Terrain::VERTEX_COUNT*Terrain::VERTEX_COUNT {
			let x = (floor64(i as f64/Terrain::VERTEX_COUNT as f64) + (floor64(j as f64/mw as f64))*(Terrain::VERTEX_COUNT as f64 - 1.0))/30.0;
			let y = 100.0;
			let z = ((i%Terrain::VERTEX_COUNT) as f64 + ((j%mh) as f64*(Terrain::VERTEX_COUNT - 1) as f64))/30.0;
			let mut val = perlin.get([x, 0.4, z]) as f32;
			if val < -0.2 {
				val = -0.2;
			}			
			heightmap.push(val);
		}
		heights.push(heightmap);
	} */

	//------------------------
	
	println!("Loading Textures...");
	texture_loader.add("image.png", gl::NEAREST);
	texture_loader.add("dragon.png", gl::NEAREST);
	texture_loader.add("stallTexture.png", gl::NEAREST);
	texture_loader.add("grass.png", gl::LINEAR);
	texture_loader.add("grass2.png", gl::LINEAR);
	texture_loader.load_all(&path_root);
	println!("Loaded...");
	
	let mut loader = Loader::create();
	let mut master_renderer = MasterRenderer::create();
	
	println!("Loading Models...");
	let dragon_model = TexturedModel::new(loader.load_obj_to_vao("dragon.obj"), texture_loader.fetch("dragon.png").clone()
																					.set_reflectivity(1.5)
																					.set_shine_damper(10.0)
																					.clone());
																					
	let ar_model = TexturedModel::new(loader.load_obj_to_vao("untitled.obj"), texture_loader.fetch("image.png").clone()
																					.set_reflectivity(1.0)
																					.set_shine_damper(10.0)
											   										.clone());
	
	println!("Loaded.");
	
	let mut terrains = Vec::new();
	let terrain = Terrain::construct(&mut loader, &heights, 0, 0, texture_loader.fetch("grass.png").clone()
																				.set_reflectivity(0.0)
																				.set_shine_damper(1.0)
																				.clone());
		
	terrains.push(&terrain);	
	
	let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0);
	let mut sun: Light = Light::new(Vec3::new(-15000.0, 10000.0, 0.0), Vec3::new(1.0, 1.0, 1.0), 500000000.0);

	let mut entity1 = Entity::new(&dragon_model, Vec3::new(20.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
	let mut entity2 = Entity::new(&ar_model, Vec3::new(-100.0, 0.0,  -100.0), Vec3::new(0.0, 0.0, 0.0));
	
	while display.is_running() {
		let mut delta_time = display.get_delta_time();
		while delta_time <= 0.0 { delta_time = display.get_delta_time(); }
		camera.update(&display.input, delta_time);
		let mut entities: Vec<&Entity> = Vec::new();
		
		entity1.inc_rotation(0.1, 0.2, 0.0);
		
		master_renderer.prossess_entity(&mut entities, &entity1);
		master_renderer.prossess_entity(&mut entities, &entity2);
		
		//sun.set_location(camera.get_location());
		
		master_renderer.render(&sun, &camera, &entities, &terrains);
		display.update();
	}
	
	//Clean Up.
	master_renderer.clean_up();
	loader.clean_up();
	texture_loader.clean_up();
	
}
