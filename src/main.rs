// main.rs

mod locale;
mod help_ver;
mod parse;
mod find;
mod scan;

use std::path::PathBuf;
use std::path::Path;
use std::fs;

const PRG_NAME: &str = "archive";
const VERSION: &str = "2026-04-25";

//#[derive(Debug)]
pub struct AppData {
	nom: String,
	version: String,
	locale: locale::ArchiveStrings,
	commande: Option<String>,
	argument: Option<String>,
	mess_erreur: Option<String>,
}

fn dir_exist(dir_2_test: &String) -> (bool, PathBuf) {
	let search_path = PathBuf::from(dir_2_test);

	if search_path.is_dir() { return (true, search_path); }
	return (false, search_path);
}

fn get_archive_path(data: &mut AppData) -> String {
	std::env::var("HOME")
		.map(|home| format!("{}/Documents/Archives/Volumes", home))
		.expect( data.locale.no_home)
}

fn main() {

	let mut app_data = AppData {
		nom: PRG_NAME.to_string(),
		version: VERSION.to_string(),
		locale: locale::get_app_lang(),
		commande: None,
		argument: None,
		mess_erreur: None,
	};

	// parse_args: retourne un bool
	// true: a réussi à identifier une commande et un argument.
	// false: Quelque chose n'a pas marché. Erreur en mess_erreur.
	if parse::parse_args(&mut app_data) {

		let archive_path: String = get_archive_path(&mut app_data);

		match app_data.commande.as_deref() {
			Some("usage") => help_ver::usage(&mut app_data),
			Some("help") => help_ver::help(&mut app_data),
			Some("ver") => help_ver::ver(&mut app_data),
			Some("version") => help_ver::version(&mut app_data),

			Some("find") => {
				let (existe, home_path_buf) = dir_exist(&archive_path);
				if existe {
					if let Err(e) = find::find_pattern(app_data.argument.unwrap().as_str(),
													&home_path_buf, &app_data.locale) {
						let error_message = app_data.locale.prob_recherche
												.replace("{1}", &find::ERREUR_RECHERCHE.to_string())
												.replace("{2}", &archive_path)
												.replace("{3}", &e.to_string());
							eprintln!("{}", error_message);
							std::process::exit(find::ERREUR_RECHERCHE);
					}
				}
				else {
					let error_message: String = app_data.locale.invalid_dir
												.replace("{1}", &archive_path);
					eprintln!("{}", error_message);
					std::process::exit(10);
				}
			},

			Some("scan") => {
				if ! PathBuf::from(&app_data.argument.clone().unwrap()).is_dir() {
					let error_message: String = app_data.locale.scan_invalid_dir
								.replace("{1}", &app_data.argument.unwrap().as_str());
					eprintln!("{}", error_message);
					std::process::exit(10);
				}
				let (existe, home_path_buf) = dir_exist(&archive_path);
				if ! existe {
					if let Err(e) = fs::create_dir_all(Path::new(&archive_path)) {
						let error_message = app_data.locale.err_create_dir
													.replace("{1}", &archive_path)
													.replace("{2}", &e.to_string());
						eprintln!("{}", error_message);
						std::process::exit(10);
					}
				}
				if let Err(e) = scan::scan_directory(&app_data.argument.unwrap(),
									&home_path_buf, &app_data.locale) {
					let error_message = app_data.locale.err_scan_dir
										.replace("{1}", &scan::ERREUR_SCAN.to_string())
										.replace("{2}", &archive_path)
										.replace("{3}", &e.to_string());
					eprintln!("{}", error_message);
					std::process::exit(scan::ERREUR_SCAN);
				}
			}
			_ => {
				println!("???");
				std::process::exit(20);
			}
		}
	}
	else {
		eprintln!("{}", app_data.mess_erreur.unwrap());
		std::process::exit(5);
	}
	std::process::exit(0);
}
