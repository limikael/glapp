use glapp::*;

#[glapp_main]
fn main(mut app:App) {
	app.title("Testing SDL");
	app.size(400.,300.);
 	app.units(AppUnits::DeviceIndependent);//HardwarePixels);
	app.run(|_w,e|{
		println!("event: {:?}",e)
	});
}
