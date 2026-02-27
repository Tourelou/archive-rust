//use std::process::Command;
use std::io;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::locale::ArchiveStrings;

pub const ERREUR_SCAN: i32 = 4;

/// Scanne récursivement et remplit le vecteur
fn scanner(dir: &Path, accumulation: &mut Vec<PathBuf>, loc: &ArchiveStrings) {
	match fs::read_dir(dir) {
		Ok(entries) => {
			for entry in entries.flatten() {
				let path = entry.path();
				if path.is_dir() { scanner(&path, accumulation, &loc); }
				 else { accumulation.push(path); }
			}
		}
		Err(e) => {
			// On affiche l'erreur sur la sortie d'erreur standard (stderr)
			eprintln!("{} {}", loc.err_scan_dir.replace("{1}", &ERREUR_SCAN.to_string())
												.replace("{2}", &dir.to_str().unwrap())
												.replace("{3}", &e.to_string()), dir.display());
		}
	}
}

/// Scan récursif d'un dossier et enregistrement dans le dossier d'archives
pub fn scan_directory(scan_path: &String, archive_path: &PathBuf, loc_arc: &ArchiveStrings) -> Result<i32, io::Error> {

	let scan_dir = PathBuf::from(&scan_path);

	// Nom du fichier = nom du dossier scanné + .txt
	let dir_name = scan_dir.file_name().unwrap_or_else(|| scan_dir.as_os_str()).to_string_lossy();
	let output_file = archive_path.join(format!("{}.txt", dir_name)); // Sauvegarde dans le dossier d'archives
//	let mut file = File::create(&output_file)?;

	if let Err(e) = env::set_current_dir(Path::new(&scan_path)) {
		eprintln!("{} {}",loc_arc.err_cd_dir , e);
		return Ok(ERREUR_SCAN);
	}
	println!("{}\n{} {}\n{}",
				"-".repeat(80), loc_arc.info_scan_dir, scan_path, "-".repeat(80));

	// Récupération de tous les fichiers à la `find`

	let mut liste = Vec::new();
	scanner(Path::new("."), &mut liste, &loc_arc);
	liste.sort();

	// 3. TRANSFORMATION : On fusionne tout en une seule String
	// On utilise map pour convertir en String et join pour ajouter les sauts de ligne
	let contenu = liste.iter().filter_map(|p| p.to_str())
									.collect::<Vec<_>>().join("\n");

	// 4. ÉCRITURE UNIQUE : On décharge tout d'un coup
	fs::write(&output_file, contenu).map_err(|e| {
	eprintln!("{} {:?} : {}", loc_arc.err_write_file, output_file, e);
	e // On retourne l'erreur io::Error d'origine
	})?;

	println!("{} '{}'\n{}", loc_arc.message_final, output_file.display(),"-".repeat(80));
	return Ok(0);
}
