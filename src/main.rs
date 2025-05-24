mod find; // Inclusion du module find
mod scan;

use std::env;
use std::process;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

const PRG_NAME: &str = "archive";
const VERSION: &str = "2025-05-24";

const ERREUR_ARGUMENTS: i32 = 1; // Code pour les erreurs liées aux arguments Clap
const ERREUR_DIR_NOT_EXIST: i32 = 2;

fn version() {
	println!("{}, version: {}", PRG_NAME, VERSION);
}

fn usage() {
	println!("Usage: {} -f <motif> | -s <dossier>\n", PRG_NAME);
	println!("-f,   --find  <motif>   : Recherche un motif.");
	println!("-s,   --scan  <dossier> : Analyse un dossier.");
	println!("-ver, --version         : Affiche la version du programme.");
	println!("-h,   --help            : Affiche ce message d'aide.");
}

fn dir_exist(dir_2_test: &String) -> (bool, PathBuf) {
	let search_path = PathBuf::from(dir_2_test);

	if search_path.is_dir() { return (true, search_path); }
	return (false, search_path);
}

fn get_archive_path() -> String {
	env::var("HOME")
		.map(|home| format!("{}/Documents/Archives/Volumes", home))
		.expect("Erreur: Impossible de récupérer la variable d'environnement HOME.")
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		eprintln!("Erreur : Vous devez spécifier -f/--find ou bien -s/--scan.\n");
		usage();
		std::process::exit(ERREUR_ARGUMENTS);
	}
	if args.len() > 1 && (args[1] == "-ver" || args[1] == "--version") {
		version();
		return;
	}
	if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
		usage();
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
					eprintln!("Erreur : Argument -f/--find nécessite une valeur.");
					std::process::exit(ERREUR_ARGUMENTS);
				}
			}
			"-s" | "--scan" => {
				if i + 1 < args.len() {
					scan_folder = Some(args[i + 1].clone());
					i += 1;
				} else {
					eprintln!("Erreur : Argument -s/--scan nécessite une valeur.");
					std::process::exit(ERREUR_ARGUMENTS);
				}
			}
			_ => {
				eprintln!("Erreur : Argument inconnu '{}'.", args[i]);
				std::process::exit(ERREUR_ARGUMENTS);
			}
		}
		i += 1;
	}
	if find_pattern.is_some() == scan_folder.is_some() {
		eprintln!("Erreur : Il faut spécifier soit -f/--find, soit -s/--scan, mais pas les deux.");
		std::process::exit(ERREUR_ARGUMENTS);
	}

// ================================================================================

	let archive_path: String = get_archive_path();

// ================ FIND ================
	if let Some(pattern) = find_pattern {
		let (test_dir, arc_path_buf) = dir_exist(&archive_path);
		if test_dir {
			if let Err(e) = find::find_pattern(&pattern, &arc_path_buf) {
				eprintln!("Erreur {}: Problème lors de la recherche dans '{}'. {}",
				find::ERREUR_RECHERCHE, archive_path, e);
				std::process::exit(find::ERREUR_RECHERCHE);
			}
		}
		else {
			eprintln!("Le dossier {} n'existe pas ou n'est pas valide", archive_path);
			process::exit(ERREUR_DIR_NOT_EXIST);
		}
	}
// ================ SCAN ================
	else if let Some(folder) = scan_folder {
		if ! PathBuf::from(&folder).is_dir() {
			eprintln!("Le dossier à scanner '{:?}' n'existe pas ou n'est pas valide", folder);
			process::exit(ERREUR_DIR_NOT_EXIST);
		}

		let (test_dir, arc_path_buf) = dir_exist(&archive_path);
		if ! test_dir {
			if let Err(e) = fs::create_dir_all(Path::new(&archive_path)) {
				eprintln!("Erreur: Impossible de créer le dossier '{}': {}", archive_path, e);
				process::exit(ERREUR_DIR_NOT_EXIST);
			}
		}
		if let Err(e) = scan::scan_directory(&folder, &arc_path_buf) {
			eprintln!("Erreur {}: Problème lors du scan dans '{}'. {}",
										scan::ERREUR_SCAN, archive_path, e);
			std::process::exit(scan::ERREUR_SCAN);
		}
	}
	return ();
}
