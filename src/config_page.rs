use std::time::Duration;

use eframe::*;
use crate::{app, states::*};
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

			ui.label("Work Time (Minutes)");
			ui.add(
				egui::Slider::new(&mut app_state.user_config.work_time, 1..=1000)
			);

			ui.label("Break Time (Minutes)");
			ui.add(
				egui::Slider::new(&mut app_state.user_config.break_time, 1..=1000)
			);

			// continue recording time even in config page
			if app_state.running == RunningState::Running {
				app_state.record_time();

				ctx.request_repaint();	
			}
		});
}