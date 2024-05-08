use std::option;

use eframe::egui::Vec2;
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
		let mut options = NativeOptions {
			centered: true,
			..Default::default()
		};

		options.viewport.inner_size = Some(Vec2::new(300.0, 140.0));
		options.viewport.resizable = Some(false);
		options.viewport = options.viewport.with_maximize_button(false);

		run_native(
			"Pomodoro App", 
			options, 
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