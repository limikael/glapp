use proc_macro::{*};
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn glapp_main(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut ast=parse_macro_input!(input as ItemFn);
	ast.sig.ident=format_ident!("_glapp_main_{}",ast.sig.ident.clone().to_string());
	let name=ast.sig.ident.clone();

	if cfg!(all(not(feature="glutin"),not(feature="sdl"),not(feature="miniquad"))) {
		panic!("Welcome to Glapp! Please enable exactly one of the features \"sdl\" or \"glutin\" to select rendering backend. Enjoy!");
	}

	let mut out=quote!{#ast};

	if cfg!(feature="glutin") {
		out.extend(quote!{
			#[cfg(not(target_os="android"))]
			pub fn main() {
				let b=::glapp::app_window_glutin::AppWindowBuilderGlutin::new();
				#name(::glapp::App::new(b));
			}

			#[cfg(target_os="android")]
			#[no_mangle]
			pub fn android_main(android_app: ::glapp::app_window_glutin::AndroidApp) {
				let b=::glapp::app_window_glutin::AppWindowBuilderGlutin::new();
				b.with_android_app(android_app);
				#name(::glapp::App::new(b));
			}
		});
	}

	if cfg!(feature="sdl") {
		out.extend(quote!{
			#[cfg(not(target_os="android"))]
			pub fn main() {
				let b=::glapp::app_window_sdl::AppWindowBuilderSdl::new();
				#name(::glapp::App::new(b));
			}

			#[cfg(target_os="android")]
			#[no_mangle]
			#[allow(non_snake_case)]
			pub fn SDL_main() {
				let b=::glapp::app_window_sdl::AppWindowBuilderSdl::new();
				#name(::glapp::App::new(b));
			}
		});
	}

	/*if cfg!(feature="miniquad") {
		out.extend(quote!{
			pub fn main() {
				let w=::appy::sys::app_window_miniquad::MiniquadAppWindowBuilder::new()
					.build();

				::appy::core::Appy::new(#name).run(w);
			}
		});
	}*/

	TokenStream::from(out)
}
