pub mod display {
	pub const WIDTH: f64 = 800.0;
	pub const HEIGHT: f64 = 800.0;
	pub const RESIZABLE: bool = false;
	pub const HAS_VSYNC: bool = false;
	#[allow(unused, non_upper_case_globals)]
	pub static mut mouse_change_x: f32 = 0.0;
	#[allow(unused, non_upper_case_globals)]
	pub static mut mouse_change_y: f32 = 0.0;
}


pub struct Display {
	pub input: InputHandler,
	pub event_loop: glutin::EventsLoop,
	pub context: glutin::WindowedContext,
	pub start_time: Instant,
	pub time_stamp: u64,
	close: bool
}

impl Display {
	pub fn create() -> Self {
		let time = std::time::SystemTime::now();
		let wb = glutin::WindowBuilder::new()
			.with_title("Minecraft")
			.with_dimensions(LogicalSize::new(display::WIDTH, display::HEIGHT))
			.with_resizable(display::RESIZABLE);
		
		let el = glutin::EventsLoop::new();
		let windowed_context = glutin::ContextBuilder::new()
			.with_vsync(display::HAS_VSYNC)
			.build_windowed(wb, &el)
			.unwrap();
			
        unsafe { windowed_context.make_current().unwrap() };
   		println!(
        	"Pixel format of the window's GL context: {:?}",
        	windowed_context.get_pixel_format()
    	);

		gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
		Self{input: InputHandler::new(), event_loop: el, context: windowed_context, start_time: Instant::now(), time_stamp: 0, close: false}
	}
	
	fn get_current_millis(&self) -> u64 {
		let elapsed = self.start_time.elapsed();
		elapsed.as_secs() * 1000 +
            elapsed.subsec_nanos() as u64 / 1_000_000
	}
	
	pub fn get_delta_time(&mut self) -> f64 {
		let millis = self.get_current_millis();
		let ret = (millis - self.time_stamp) as f64 / 1000.0;
		self.time_stamp = millis;
		ret
	}
	
	pub fn is_running(&self) -> bool {
		return !self.close;
	}
		
	pub fn close(&mut self) {
		self.close = true;
	}
	pub fn update(&mut self) {
		self.close = Display::update_display(&mut self.context, &mut self.event_loop, &mut self.input);
	}
	
	fn update_display(context: &mut glutin::WindowedContext, event_loop: &mut glutin::EventsLoop, input_handler: &mut InputHandler) -> bool {
				let mut ret = false;
				event_loop.poll_events(|event| {
				match event {
					glutin::Event::WindowEvent{ event, .. } => match event {
						glutin::WindowEvent::CloseRequested => {
							ret = true; 
						},
						glutin::WindowEvent::Resized(logical_size) => {
							let dpi_factor = context.get_hidpi_factor();
							context.resize(logical_size.to_physical(dpi_factor));
						},
						glutin::WindowEvent::KeyboardInput{input, ..} => {
							Display::update_input_handler(input_handler, &input);
							if input.virtual_keycode == Some(glutin::VirtualKeyCode::Escape) {
								ret = true;
							}
						},
						glutin::WindowEvent::CursorMoved{position, ..} => {
							context.set_cursor_position(LogicalPosition::new(display::WIDTH/2.0, display::HEIGHT/2.0)).unwrap();
							if floor64(position.x) != display::WIDTH/2.0 ||floor64(position.y) != display::HEIGHT/2.0 {
								unsafe {
									display::mouse_change_x = (position.x - display::WIDTH/2.0) as f32;
									display::mouse_change_y = (position.y - display::HEIGHT/2.0) as f32;
								}
							}
						},
						_ => ()
					},
					_ => ()
				}
			});
			context.swap_buffers().unwrap();
			ret
	}
	
	fn update_input_handler(input_handler: &mut InputHandler, input: &glutin::KeyboardInput) {
		if input.state == glutin::ElementState::Pressed {
			match input.virtual_keycode {
				Some(code) => {
					let mut ret: bool = false;
					match input_handler.key_hash_is_pressed.get(&code) {
						Some(result) => {
							ret = *result;
						},
						None =>{}
					}	
					if !ret {
						input_handler.key_hash_first_press.insert(code, true);
					}
					input_handler.key_hash_is_pressed.insert(code, true);
				},
				None => {}
			}
		}	
		if input.state == glutin::ElementState::Released {
			match input.virtual_keycode {
				Some(code) => {
					input_handler.key_hash_is_pressed.insert(code, false);
					input_handler.key_hash_first_press.insert(code, false);
				},
				None => {}
			}
		}	
	}
}