use eframe::*;

use crate::{app_state::AppState, states::Pages};
use crate::pages::*;

#[derive(Default)]
pub struct PomodoroApp {
	pub app_state: AppState,
}

impl PomodoroApp {
	pub fn new(_cc: &CreationContext<'_>) -> Self {
		Self::default()
	}

	pub fn run() -> eframe::Result<()> {
		run_native(
			"Pomodoro App", 
			NativeOptions::default(), 
			Box::new(
				|cc| Box::new(Self::new(cc))
			)
		)
	}
}

impl App for PomodoroApp {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		let app_state = &mut self.app_state;

		match app_state.active_page {
			Pages::Main => main_page(ctx, app_state),
			Pages::Config => config_page(ctx, app_state),
		};
	}
}