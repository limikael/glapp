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

mod android_log_thread;
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
    Resize{width:u32, height:u32},
    MouseDown{x:i32, y:i32, kind:MouseKind, button:MouseButton},
    MouseUp{x:i32, y:i32, kind:MouseKind, button:MouseButton},
    MouseMove{x:i32, y:i32, kind:MouseKind},
}

pub trait AppWindow {
    fn run(self: Box<Self>, handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>);
    fn post_redisplay(&mut self);
    fn size(&self)->(i32,i32);
    fn pixel_ratio(&self)->f32;
}

pub trait AppWindowBuilder {
    fn build(self: Box<Self>)->Box<dyn AppWindow>;
    fn title(&mut self, title:String);
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

    pub fn title(&mut self, title: &str) {
        self.builder.title(String::from(title));
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