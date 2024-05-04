use std::time::Duration;

use eframe::*;
use crate::{app_state::AppState, states::{Pages, RunningState, WorkState}};

pub fn format_duration(dur: &Duration) -> String {
	let mins = dur.as_secs() / 60;
	let secs = dur.as_secs() % 60;
	let millis = dur.subsec_millis();

	format!("{:02}.{:02}.{:03}", mins, secs, millis)
}

pub fn main_page(ctx: &eframe::egui::Context, app_state: &mut AppState) {
	egui::CentralPanel::default()
		.show(ctx, 
		|ui| {
			ui.heading(format!("State: {}", match app_state.work_state {
				WorkState::Resting => "Rest Time",
				WorkState::Working => "Work Time",
			}));

			ui.with_layout(
				egui::Layout::left_to_right(egui::Align::Min), 
				|ui| {

					let start_resp = ui.button("Start");

					let pause_resp = ui.add_enabled(
						!(app_state.running == RunningState::Stopped), 
						egui::Button::new(match app_state.running {
							RunningState::Running => "Pause",
							_ => "Unpause"
						})
					);

					if pause_resp.clicked() {
						match app_state.running {
							RunningState::Running => app_state.pause(),
							RunningState::Paused => app_state.unpause(),
							_ => (),
						}
					}

					let stop_resp = ui.button("Stop");

					if start_resp.clicked() {
						app_state.start_timer();
					}
					
					if stop_resp.clicked() {
						app_state.stop();
					}

				});

			if app_state.running == RunningState::Running {
				app_state.record_time();

				ctx.request_repaint();
			}

			// display stats
			let time_left = match app_state.work_state {
				WorkState::Working => {
					app_state.user_config.get_duration_secs(&WorkState::Working) - app_state.elapsed
				},
				WorkState::Resting => {
					app_state.user_config.get_duration_secs(&WorkState::Resting) - app_state.elapsed
				}
			};

			ui.heading(match app_state.running {
				RunningState::Running | RunningState::Paused => {
					format_duration(&app_state.elapsed)
				},
				_ => {"00.00.000".to_string()}
			});

			ui.label(
				format!("Time Left: {}", format_duration(&time_left))
			);

			if ui.button("Settings").clicked() {
				app_state.active_page = Pages::Config;
			}
		});
}