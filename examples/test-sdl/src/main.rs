use glapp::*;

#[glapp_main]
fn main(mut app:App) {
	app.title("Testing SDL");
	app.run(|w,e|{
		println!("event: {:?}",e)
	});
}
