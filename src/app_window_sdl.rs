use sdl2::event::{Event, WindowEvent};
use crate::{MouseKind, MouseButton, AppWindow, AppEvent, AppWindowBuilder, AppUnits};

const SDL_TOUCH_MOUSEID: u32 = u32::MAX;

fn decode_mouse(mouse_id:u32, mouse_btn:sdl2::mouse::MouseButton)
		->(MouseKind,MouseButton) {
	if mouse_id==SDL_TOUCH_MOUSEID {
		return (MouseKind::Touch,MouseButton::Unknown)
	}

	match mouse_btn {
		sdl2::mouse::MouseButton::Left=>(MouseKind::Mouse,MouseButton::Left),
		sdl2::mouse::MouseButton::Right=>(MouseKind::Mouse,MouseButton::Right),
		_=>(MouseKind::Mouse,MouseButton::Unknown)
	}
}

pub struct AppWindowBuilderSdl {
	title: String,
	size: (f32,f32),
	units: AppUnits,
}

impl AppWindowBuilder for AppWindowBuilderSdl {
    fn build(self:Box<Self>)->Box<dyn AppWindow> {
    	Box::new(AppWindowSdl::new(&self)) //.title.clone()))
    }

    fn title(&mut self, title:String) {
        self.title=title;
    }

    fn size(&mut self, w:f32, h:f32) {
    	self.size=(w,h);
    }

    fn units(&mut self, units: AppUnits) {
    	self.units=units;
    }
}

impl AppWindowBuilderSdl {
	pub fn new()->Box<Self> {
		Box::new(Self {
			title: "Unknown".to_string(),
			size: (800.,600.),
			units: AppUnits::HardwarePixels
		})
	}
}

pub struct AppWindowSdl {
	sdl: sdl2::Sdl,
	window: sdl2::video::Window,
	_gl_context: sdl2::video::GLContext,
	_video_subsystem: sdl2::VideoSubsystem,
	hw_size: (f32,f32),
	expose_requested: bool,
	quit_requested: bool,
	pixel_ratio: f32,
	units: AppUnits,
}

impl AppWindow for AppWindowSdl {
	fn size(&self)->(f32,f32) {
		(
			self.units.hw_to_units(self.pixel_ratio,self.hw_size.0),
			self.units.hw_to_units(self.pixel_ratio,self.hw_size.1)
		)
	}

	fn pixel_ratio(&self)->f32 {
		self.pixel_ratio
	}

	fn post_redisplay(&mut self) {
		self.expose_requested=true;
	}

    fn run(self: Box<Self>, handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
    	self.run_impl(handler);
    }
}

impl AppWindowSdl {
	pub fn new(builder:&AppWindowBuilderSdl)->Self {
		let sdl=sdl2::init().unwrap();
		let video_subsystem=sdl.video().unwrap();

		let mut pixel_ratio=1.0;
		let dpi=video_subsystem.display_dpi(0).unwrap().0;
		if dpi>160.0 {
			pixel_ratio=dpi/160.0;
		}

		//pixel_ratio=2.0; // debug
    	println!("pixel ratio: {:?}",pixel_ratio);

    	let hw_size=(
    		builder.units.units_to_hw(pixel_ratio,builder.size.0),
    		builder.units.units_to_hw(pixel_ratio,builder.size.1)
    	);

		let window=video_subsystem
			.window(&builder.title,hw_size.0 as u32,hw_size.1 as u32)
			.opengl()
			.resizable()
			.build()
			.unwrap();

		let gl_context=window.gl_create_context().unwrap();
		let _gl_loaded=gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let window_size = window.size();
        let hw_size=(
        	window_size.0 as f32,
        	window_size.1 as f32
        );

		Self {
			sdl,
			window,
			_video_subsystem: video_subsystem,
			_gl_context: gl_context,
			hw_size,
			expose_requested: false,
			pixel_ratio,
			quit_requested: false,
			units: builder.units.clone()
		}
	}

	fn handle_event(&mut self, handler:&mut Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>, e:&Event) {
		//println!("{:?}",e);

		match *e {
			Event::Quit{..}=>{
				self.quit_requested=true;
			},
			/*Event::Window {win_event: WindowEvent::Shown, ..}=>{
				handler(&mut self,AppEvent::Show);
			},*/
			Event::Window {win_event: WindowEvent::Exposed, ..}=>{
				self.post_redisplay();
			}
			Event::Window {win_event: WindowEvent::Resized(w,h), ..}=>{
                unsafe { gl::Viewport(0, 0, w, h) };
                self.hw_size=(w as f32, h as f32);
                let e=AppEvent::Resize{
                	width:self.units.hw_to_units(self.pixel_ratio,self.hw_size.0),
                	height:self.units.hw_to_units(self.pixel_ratio,self.hw_size.1)
                };

                handler(self,e);

                // For some reason android need this extra render pass.
                self.do_render(handler);
                self.post_redisplay();

			}
			Event::Window {win_event: WindowEvent::Restored, ..}=>{
				self.post_redisplay();
			}
			Event::MouseButtonDown {x, y, mouse_btn, which, ..} => {
				let (kind,button)=decode_mouse(which,mouse_btn);
				let e=AppEvent::MouseDown{
					x: self.units.hw_to_units(self.pixel_ratio,x as f32),
					y: self.units.hw_to_units(self.pixel_ratio,y as f32),
					kind,
					button
				};

				handler(self,e);
			}
			Event::MouseButtonUp {x, y, mouse_btn, which, ..} => {
				let (kind,button)=decode_mouse(which,mouse_btn);
				let e=AppEvent::MouseUp{
					x: self.units.hw_to_units(self.pixel_ratio,x as f32),
					y: self.units.hw_to_units(self.pixel_ratio,y as f32),
					kind,
					button
				};

				handler(self,e);
			}
			Event::MouseMotion {x, y, which, ..} => {
				let (kind,_)=decode_mouse(which,sdl2::mouse::MouseButton::Unknown);
				let e=AppEvent::MouseMove{
					x: self.units.hw_to_units(self.pixel_ratio,x as f32),
					y: self.units.hw_to_units(self.pixel_ratio,y as f32),
					kind
				};

				handler(self,e);
			}
			_ => {}
		}
	}

	fn do_render(&mut self, handler:&mut Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
		unsafe {
			gl::ClearColor(0.0,0.0,0.0,0.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		handler(self,AppEvent::Render);
		self.window.gl_swap_window();
	}

    fn run_impl(mut self, mut handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
		let mut event_pump=self.sdl.event_pump().unwrap();

		handler(&mut self,AppEvent::Show);
		self.expose_requested=true;

		while !self.quit_requested {
			let mut e=if self.expose_requested {
				event_pump.poll_event()
			} else {
				Some(event_pump.wait_event())
			};

			while e.is_some() {
				self.handle_event(&mut handler,&e.unwrap());
				e=event_pump.poll_event()
			}

			if self.expose_requested {
				self.expose_requested=false;
				self.do_render(&mut handler);
			}
		}
    }
}