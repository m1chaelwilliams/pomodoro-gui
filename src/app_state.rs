use std::time::{Duration, Instant};
use kira::{manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings}, sound::static_sound::{StaticSoundData, StaticSoundSettings}};

use crate::states::*;

pub const WORK_TIME_SECS: u64 = 25*60;
pub const BREAK_TIME_SECS: u64 = 5*60;

pub struct UserConfig {
	pub work_time: u64,
	pub break_time: u64,
}

impl UserConfig {
	pub fn get_time_mut(&mut self, work_state: &WorkState) -> &mut u64 {
		match work_state {
			WorkState::Resting => &mut self.break_time,
			WorkState::Working => &mut self.work_time
		}
	}

	pub fn get_duration_mins(&self, work_state: &WorkState) -> Duration {
		match work_state {
			WorkState::Resting => Duration::from_secs(self.break_time/60),
			WorkState::Working => Duration::from_secs(self.work_time/60),
		}
	}
	pub fn get_duration_secs(&self, work_state: &WorkState) -> Duration {
		match work_state {
			WorkState::Resting => Duration::from_secs(self.break_time),
			WorkState::Working => Duration::from_secs(self.work_time),
		}
	}
}

impl Default for UserConfig {
	fn default() -> Self {
		Self {
			work_time: WORK_TIME_SECS,
			break_time: BREAK_TIME_SECS,
		}
	}
}

/// Houses the state of the program.
/// Seperate from `PomodoroApp` to allow dependency 
/// injection while avoiding tight coupling.
pub struct AppState {
	pub active_page: Pages,
	pub running: RunningState,
	pub start_time: Instant,
	pub cur_time: Instant,
	pub stop_time: Instant,
	pub pause_interval: Duration,
	pub elapsed: Duration,
	pub work_state: WorkState,

	// config
	pub user_config: UserConfig,
	pub user_config_draft: UserConfig,

	// audio
	pub audio_manager: AudioManager,
	pub notif_sound: StaticSoundData,
}

impl Default for AppState {
	fn default() -> Self {

		let manager = AudioManager::<DefaultBackend>::new(
			AudioManagerSettings::default()
		).expect("Failed to initialize Kira audio manager");
		let notif_sound = StaticSoundData::from_file(
			"assets/alarm.mp3", 
			StaticSoundSettings::default()
		).expect("Failed to load sound data");

		Self {
			start_time: std::time::Instant::now(),
			cur_time: std::time::Instant::now(),
			stop_time: std::time::Instant::now(),
			active_page: Pages::default(),
			running: RunningState::Stopped,
			pause_interval: Duration::from_secs(0),
			elapsed: Duration::from_secs(0),
			user_config: UserConfig::default(),
			user_config_draft: UserConfig::default(),
			work_state: WorkState::default(),
			audio_manager: manager,
			notif_sound
		}
	}
}

impl AppState {
	pub fn start_timer(&mut self) {
		self.reset();
		self.running = RunningState::Running;
		self.start_time = std::time::Instant::now();
	}

	pub fn record_time(&mut self) {
		self.elapsed += self.cur_time.elapsed();
		self.cur_time = std::time::Instant::now();

		if self.elapsed > self.user_config.get_duration_secs(&self.work_state) {
			self.audio_manager.play(self.notif_sound.clone())
				.unwrap();
			let prev_work_state = self.work_state.clone();
			self.reset();
			
			match prev_work_state {
				WorkState::Resting => self.work_state = WorkState::Working,
				WorkState::Working => self.work_state = WorkState::Resting,
			};
		}		
	}

	pub fn get_elapsed_secs(&self) -> u64 {
		self.elapsed
			.as_secs()
	}
	pub fn get_subsec_millis(&self) -> u32 {
		self.elapsed
			.subsec_millis()
	}

	pub fn pause(&mut self) {
		self.running = RunningState::Paused;
		self.stop_time = std::time::Instant::now();
	}

	pub fn unpause(&mut self) {
		self.running = RunningState::Running;
		self.cur_time = std::time::Instant::now();
	}

	pub fn reset(&mut self) {
		self.elapsed = Duration::from_secs(0);
		let now = std::time::Instant::now();
		self.pause_interval = Duration::from_secs(0);
		self.cur_time = now.clone();
		self.stop_time = now.clone();
		self.work_state = WorkState::Working;
	}

	pub fn stop(&mut self) {
		self.running = RunningState::Stopped;
		self.reset();
	}
}