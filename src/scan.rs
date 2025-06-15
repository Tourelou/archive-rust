use std::process::Command;
use std::io::{self, Write};
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::locale::Strings;

pub const ERREUR_SCAN: i32 = 4;

/// Scan récursif d'un dossier et enregistrement dans le dossier d'archives
pub fn scan_directory(scan_path: &String, archive_path: &PathBuf, loc_arc: &Strings) -> Result<i32, io::Error> {

	let scan_dir = PathBuf::from(&scan_path);

	// Nom du fichier = nom du dossier scanné + .txt
	let dir_name = scan_dir.file_name().unwrap_or_else(|| scan_dir.as_os_str()).to_string_lossy();
	let output_file = archive_path.join(format!("{}.txt", dir_name)); // Sauvegarde dans le dossier d'archives
	let mut file = File::create(&output_file)?;

	if let Err(e) = env::set_current_dir(Path::new(&scan_path)) {
		eprintln!("{} {}",loc_arc.err_cd_dir , e);
		return Ok(ERREUR_SCAN);
	}
	println!("{}\n{} {}\n{}",
				"-".repeat(80), loc_arc.info_scan_dir, scan_path, "-".repeat(80));

	// Exécution de `find` pour récupérer tous les fichiers
	let output = Command::new("find")
		.arg(".") // Passer le chemin en String
		.arg("-type")
		.arg("f")
		.output()
		.expect(loc_arc.err_cmd_find);

	//println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
	file.write_all(&output.stdout)?; // Écriture des résultats dans le fichier

	println!("{} '{}'\n{}", loc_arc.message_final, output_file.display(),"-".repeat(80));
	return Ok(0);
}
