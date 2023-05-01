//! # glapp - OpenGL meta window and context creation
//!
//! Glapp is a library for OpenGL window and context creation.
//!
//! Example:
//!
//! ```rust
//! use glapp::*;
//! 
//! #[glapp_main]
//! fn main(mut app:App) {
//!     app.title("Testing Glapp");
//!     app.run(|window,event|{
//!         match event {
//!             AppEvent::Open=>{
//!                 // The OpenGL context is created and made current.
//!             },
//!             AppEvent::Render=>{
//!                 // This is where we render our scene.
//!             },
//!             // ... see docs for more events, e.g. input and such ...
//!         }
//!     });
//! }
//! ```
//!

pub use glapp_macros::*;
pub use gl;

#[cfg(target_os="android")]
mod android_log_thread;

#[cfg(target_os="android")]
pub use android_log_thread::*;

#[derive(Debug)]
pub enum MouseButton {
	Left,
	Right,
	Unknown
}

#[derive(Debug)]
pub enum MouseKind {
	Mouse,
	Touch
}

#[derive(Debug)]
pub enum AppEvent {
    Show,
    Render,
    Resize{width:f32, height:f32},
    MouseDown{x:f32, y:f32, kind:MouseKind, button:MouseButton},
    MouseUp{x:f32, y:f32, kind:MouseKind, button:MouseButton},
    MouseMove{x:f32, y:f32, kind:MouseKind},
}

#[derive(Debug,Clone)]
pub enum AppUnits {
    DeviceIndependent,
    HardwarePixels
}

impl AppUnits {
    #[allow(unused_parens)]
    pub fn hw_to_units(&self, pixel_ratio:f32, value:f32)->f32 {
        match self {
            AppUnits::DeviceIndependent=>(value/pixel_ratio),
            AppUnits::HardwarePixels=>value
        }
    }

    #[allow(unused_parens)]
    pub fn units_to_hw(&self, pixel_ratio:f32, value:f32)->f32 {
        match self {
            AppUnits::DeviceIndependent=>(value*pixel_ratio),
            AppUnits::HardwarePixels=>value
        }
    }
}

pub trait AppWindow {
    fn run(self: Box<Self>, handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>);
    fn post_redisplay(&mut self);
    fn size(&self)->(f32,f32);
    fn pixel_ratio(&self)->f32;
}

pub trait AppWindowBuilder {
    fn build(self: Box<Self>)->Box<dyn AppWindow>;
    fn title(&mut self, title:String);
    fn size(&mut self, w:f32, h:f32);
    fn units(&mut self, units:AppUnits);
}

pub struct App {
    builder: Box<dyn AppWindowBuilder>
}

impl App {
    pub fn new(builder:Box<dyn AppWindowBuilder>)->Self {
        Self {
            builder
        }
    }

    pub fn units(&mut self, units:AppUnits) {
        self.builder.units(units);
    }

    pub fn title(&mut self, title: &str) {
        self.builder.title(String::from(title));
    }

    pub fn size(&mut self, w:f32, h:f32) {
        self.builder.size(w,h);
    }

    pub fn run<T>(self:Self, handler: T)
            where T: FnMut(&mut dyn AppWindow,AppEvent) + 'static {
        let app_window=self.builder.build();
        app_window.run(Box::new(handler));
    }
}

#[cfg(feature="glutin")]
pub mod app_window_glutin;

#[cfg(feature="sdl")]
pub mod app_window_sdl;

#[cfg(feature="miniquad")]
pub mod app_window_miniquad;