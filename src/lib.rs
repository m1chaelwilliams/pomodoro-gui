pub mod app;
pub mod states;
pub mod app_state;
pub mod main_page;
pub mod config_page;

pub mod pages {
	pub use crate::main_page::main_page;
	pub use crate::config_page::*;
}

pub use self::app::*;