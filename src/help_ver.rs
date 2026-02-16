// help_ver.rs

use crate::AppData;

pub fn usage(data: &mut AppData) {
	println!("Usage: {} {}", data.nom, data.locale.usage);
}

pub fn help(data: &mut AppData) {
	println!("Usage: {} {}\n--", data.nom, data.locale.usage);
	println!("{}", data.locale.options);
}

pub fn ver(data: &mut AppData) {
	println!("{}: {}", data.nom, data.version);
}

pub fn version(data: &mut AppData) {
	println!("{}: {} {}", data.nom, data.locale.description, data.version);
}
