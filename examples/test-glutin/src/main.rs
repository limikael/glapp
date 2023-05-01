use glapp::*;

#[glapp_main]
fn main(mut app:App) {
	app.title("Testing SDL");
	app.run(|_w,e|{
		println!("event: {:?}",e)
	});
}
