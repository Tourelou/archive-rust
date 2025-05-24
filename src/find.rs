use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub const ERREUR_RECHERCHE: i32 = 3;

/// Recherche un pattern dans tous les fichiers .txt du dossier donné et imprime les résultats
pub fn find_pattern(pattern: &str, search_path: &PathBuf) -> Result<i32, io::Error> {
	let pattern = pattern.to_lowercase();
	
	println!("{}\nRecherche du pattern: {}, Dans le dossier: {:?}\n{}",
				"-".repeat(80), pattern, search_path, "-".repeat(80));

	let mut entries: Vec<_> = fs::read_dir(&search_path)?
		.filter_map(|entry| entry.ok()) // Ignore les erreurs de lecture
		.map(|entry| entry.path()) // Récupère le chemin complet
		.filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "txt")) // Filtre uniquement les fichiers .txt
		.collect();

	if entries.is_empty() {
		eprint!("Le dossier '{}' ne contient\naucun fichier .txt pour effectuer la recherche.\n", search_path.display());
		return Ok(ERREUR_RECHERCHE);
	}
	// Trie les fichiers et répertoires par ordre alphabétique
	entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

	let mut any_match = false;

	for path in entries {
		if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
			if search_in_file(&path, &pattern)? {
				println!("{}", "=".repeat(80)); // Séparation entre les fichiers
				any_match = true;
			}
		}
	}
	if !any_match {
		println!("Aucun fichier contenant le pattern '{}' n'a été trouvé.\n{}", pattern, "=".repeat(80));
	}
	Ok(0)
}

/// Recherche un pattern dans un fichier donné et affiche les correspondances
/// Retourne `true` si au moins un match est trouvé, sinon `false`
fn search_in_file(file_path: &Path, pattern: &String) -> io::Result<bool> {
	let file = fs::File::open(file_path)?;
	let reader = io::BufReader::new(file);
	let mut found = false;

	let basename = file_path.file_name().unwrap_or_else(|| file_path.as_os_str()).to_string_lossy();
	for line in reader.lines() {
		let line = line?.to_lowercase();
		if line.contains(pattern) {
			println!("{} <<== {}", basename, line);
			found = true;
		}
	}
	Ok(found)
}
