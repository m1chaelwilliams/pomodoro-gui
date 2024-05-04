use eframe::*;
use crate::states::*;
use crate::app_state::AppState;

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
			
			ui.heading("Settings");

			ui.label("Work Time");
			ui.with_layout(
				egui::Layout::left_to_right(egui::Align::Min), 
				|ui| {

					ui.label("Work Time: ");
					ui.add(
						egui::DragValue::new(app_state.user_config_draft.get_time_mut(&WorkState::Working))
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
					ui.label("Break Time: ");
					ui.add(
						egui::DragValue::new(app_state.user_config_draft.get_time_mut(&WorkState::Resting))
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