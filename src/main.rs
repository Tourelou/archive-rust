mod find; // Inclusion du module find
mod scan;
mod locale;

use std::env;
use std::process;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

const PRG_NAME: &str = "archive";
const VERSION: &str = "2025-06-14";

const ERREUR_ARGUMENTS: i32 = 1; // Code pour les erreurs liées aux arguments Clap
const ERREUR_DIR_NOT_EXIST: i32 = 2;

fn version() {
	println!("{PRG_NAME}, version: {VERSION}");
}

fn usage(arc_loc: &locale::Strings) {
	println!("Usage: {PRG_NAME} {}\n", arc_loc.usage);
	println!("{}", arc_loc.options);
}

fn dir_exist(dir_2_test: &String) -> (bool, PathBuf) {
	let search_path = PathBuf::from(dir_2_test);

	if search_path.is_dir() { return (true, search_path); }
	return (false, search_path);
}

fn get_archive_path(arc_loc: &locale::Strings) -> String {
	env::var("HOME")
		.map(|home| format!("{}/Documents/Archives/Volumes", home))
		.expect(arc_loc.no_home)
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let arc_locale = locale::set_arc_locale(); // Récupérer les strings

	if args.len() == 1 {
		eprintln!("{}", arc_locale.un_ou_lautre);
		usage(&arc_locale);
		std::process::exit(ERREUR_ARGUMENTS);
	}
	if args.len() > 1 && (args[1] == "-ver" || args[1] == "--version") {
		version();
		return;
	}
	if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
		usage(&arc_locale);
		return;
	}

	let mut find_pattern: Option<String> = None;
	let mut scan_folder: Option<String> = None;

	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"-f" | "--find" => {
				if i + 1 < args.len() {
					find_pattern = Some(args[i + 1].clone());
					i += 1;
				} else {
					eprintln!("{}", arc_locale.f_besoin_valeur);
					std::process::exit(ERREUR_ARGUMENTS);
				}
			}
			"-s" | "--scan" => {
				if i + 1 < args.len() {
					scan_folder = Some(args[i + 1].clone());
					i += 1;
				} else {
					eprintln!("{}", arc_locale.s_besoin_valeur);
					std::process::exit(ERREUR_ARGUMENTS);
				}
			}
			_ => {
				eprintln!("{} '{}'.", arc_locale.arg_inconnu, args[i]);
				std::process::exit(ERREUR_ARGUMENTS);
			}
		}
		i += 1;
	}
	if find_pattern.is_some() == scan_folder.is_some() {
		eprintln!("{}", arc_locale.f_s_mutually);
		std::process::exit(ERREUR_ARGUMENTS);
	}

// ================================================================================

	let archive_path: String = get_archive_path(&arc_locale);

// ================ FIND ================
	if let Some(pattern) = find_pattern {
		let (test_dir, arc_path_buf) = dir_exist(&archive_path);
		if test_dir {
			if let Err(e) = find::find_pattern(&pattern, &arc_path_buf, &arc_locale) {
//				eprintln!("Erreur {}: Problème lors de la recherche dans '{}'. {}",
//				find::ERREUR_RECHERCHE, archive_path, e);
				let error_message = arc_locale.prob_recherche
					.replace("{1}", &find::ERREUR_RECHERCHE.to_string())
					.replace("{2}", &archive_path)
					.replace("{3}", &e.to_string());
				eprintln!("{}", error_message);
				std::process::exit(find::ERREUR_RECHERCHE);
			}
		}
		else {
//			eprintln!("Le dossier '{}' n'existe pas ou n'est pas valide", archive_path);
			let error_message: String = arc_locale.invalid_dir
								.replace("{1}", &archive_path);
			eprintln!("{}", error_message);
			process::exit(ERREUR_DIR_NOT_EXIST);
		}
	}
// ================ SCAN ================
	else if let Some(folder) = scan_folder {
		if ! PathBuf::from(&folder).is_dir() {
//			eprintln!("Le dossier à scanner '{}' n'existe pas ou n'est pas valide", folder);
			let error_message: String = arc_locale.scan_invalid_dir
								.replace("{1}", &folder);
			eprintln!("{}", error_message);
			process::exit(ERREUR_DIR_NOT_EXIST);
		}

		let (test_dir, arc_path_buf) = dir_exist(&archive_path);
		if ! test_dir {
			if let Err(e) = fs::create_dir_all(Path::new(&archive_path)) {
//				eprintln!("Erreur: Impossible de créer le dossier '{}': {}", archive_path, e);
				let error_message: String = arc_locale.err_create_dir
					.replace("{1}", &archive_path)
					.replace("{2}", &e.to_string());
				eprintln!("{}", error_message);
				process::exit(ERREUR_DIR_NOT_EXIST);
			}
		}
		if let Err(e) = scan::scan_directory(&folder, &arc_path_buf, &arc_locale) {
//			eprintln!("Erreur {}: Problème lors du scan de '{}'. {}",
//										scan::ERREUR_SCAN, archive_path, e);
			let error_message = arc_locale.err_scan_dir
				.replace("{1}", &scan::ERREUR_SCAN.to_string())
				.replace("{2}", &archive_path)
				.replace("{3}", &e.to_string());
			eprintln!("{}", error_message);
			std::process::exit(scan::ERREUR_SCAN);
		}
	}
	return ();
}
