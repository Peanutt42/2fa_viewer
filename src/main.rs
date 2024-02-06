use libreauth::oath::TOTPBuilder;
use eframe::egui;
use std::time::{SystemTime, UNIX_EPOCH};

fn seconds_until_regeneration(interval_seconds: u64) -> u64 {
    let now = SystemTime::now();
    let unix_epoch = UNIX_EPOCH;
    let elapsed = now.duration_since(unix_epoch).unwrap().as_secs();
    interval_seconds - (elapsed % interval_seconds)
}

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

	let mut totp_secret = String::new();
	
	eframe::run_simple_native("2FA Viewer", options, move |ctx, _frame| {
		ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("TOTP secret: ");
				ui.text_edit_singleline(&mut totp_secret);
			});
			match TOTPBuilder::new().base32_key(&totp_secret).finalize() {
				Ok(totp) => {
					let code = totp.generate();
					ui.label(format!("{} - {} seconds left!", code, seconds_until_regeneration(30)));
					if ui.button("Copy").clicked() {
						ui.output_mut(|o| o.copied_text = code);
					}
				},
				Err(e) => {
					ui.label(format!("Error: {e}"));
				},
			};
		});
	})
}