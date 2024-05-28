use crate::app::App;

pub mod model;
mod app;



fn main() {
	env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
		..Default::default()
	};

	eframe::run_native(
		"My egui App",
		options,
		Box::new(|cc| {
			Box::<App>::default()
		}),
	).expect("egui error");
}
