

#[derive(Default)]
pub enum Pages {
	#[default]
	Main,
	Config
}

#[derive(Default, PartialEq, Eq)]
pub enum RunningState {
	Running,
	Paused,
	#[default]
	Stopped
}

#[derive(Default, PartialEq, Eq)]
pub enum WorkState {
	#[default]
	Working,
	Resting,
}