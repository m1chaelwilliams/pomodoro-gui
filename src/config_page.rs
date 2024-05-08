use eframe::egui::Ui;
use eframe::*;
use crate::states::*;
use crate::app_state::{AppState, UserConfig};

pub fn drag_value(ui: &mut Ui, msg: &str, val: &mut u64) {
	ui.label(msg);
	ui.add(
		egui::DragValue::new(val)
			.clamp_range(1..=(60*60))
			.custom_formatter(|n, _| {
				let n = n as u64;
				let mins = n / 60;
				let secs = n % 60;
				format!("{mins:02}:{secs:02}")
			})	
			.custom_parser(|s| {
				let parts: Vec<&str> = s.split(":").collect();
				if parts.len() == 2 {
					let mins = parts[0].parse::<u64>().ok()?;
					let secs = parts[1].parse::<u64>().ok()?;

					Some(mins as f64 * 60.0 + secs as f64)
				} else {
					if let Ok(num) = parts[0].parse::<u64>() {
						Some(num as f64)
					} else {
						None
					}
				}
			})
	);
}

pub fn config_page(ctx: &eframe::egui::Context, app_state: &mut AppState) {

	egui::CentralPanel::default()
		.show(ctx, 
		|ui| {
			if ui.button("< Back").clicked() {
				app_state.active_page = Pages::Main;

				// update current time if app was paused while configuring to prevent duration overflows.
				if app_state.running == RunningState::Paused {
					app_state.record_time();
				}
			}

			ui.add_space(5.0);
			
			ui.heading("Settings");

			ui.add_space(5.0);

			egui::Grid::new("Config")
				.striped(true)
				.show(ui, |ui| {
					drag_value(ui, "Work time: ", app_state.user_config_draft.get_time_mut(&WorkState::Working));
					ui.end_row();
					drag_value(ui, "Break time: ", app_state.user_config_draft.get_time_mut(&WorkState::Resting));
					ui.end_row();
				});

			ui.add_space(5.0);
			ui.separator();

			ui.with_layout(
				egui::Layout::right_to_left(egui::Align::Min), 
				|ui| {
					if ui.button("Reset").clicked() {
						app_state.user_config_draft = UserConfig::default();
					}
					if ui.button("Save").clicked() {
						app_state.user_config.work_time = app_state.user_config_draft.work_time;
						app_state.user_config.break_time = app_state.user_config_draft.break_time;
					}
				});

			// continue recording time even in config page
			if app_state.running == RunningState::Running {
				app_state.record_time();

				ctx.request_repaint();	
			}
		});
}