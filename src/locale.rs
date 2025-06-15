use std::env;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Lang {
	En,
	Fr,
	Es, // Ajout de langues si besoin
}

const OPTIONS_EN: &str =
r#"    -f,   --find  <regex pattern> : Pattern to search: Put in single cote '...'.
    -s,   --scan  <folder>        : Path to volume/folder to scan.
    -ver, --version               : Program info version.
    -h,   --help                  : Help message then exit."#;

const OPTIONS_FR: &str =
r#"    -f,   --find  <motif regex>   : Motif à chercher: Mettre entre '...'.
    -s,   --scan  <dossier>       : Analyse un dossier.
    -ver, --version               : Affiche la version du programme.
    -h,   --help                  : Affiche ce message d'aide."#;

const OPTIONS_ES: &str =
r#"    -f,   --find     <patrón regex>       : Patrón a buscar: Poner entre '...'
    -s,   --scan     <volumen a escanear> : Ruta del directorio a escanear.
    -ver, --version                       : Proporciona información de la versión del programa.
    -h,   --help                          : Muestra este mensaje de ayuda y finaliza."#;



pub struct Strings {
	pub options: &'static str,
	pub usage: &'static str,
	pub no_home: &'static str,
	pub un_ou_lautre: &'static str,
	pub f_besoin_valeur: &'static str,
	pub s_besoin_valeur: &'static str,
	pub arg_inconnu: &'static str,
	pub f_s_mutually: &'static str,
	pub prob_recherche: &'static str,
	pub invalid_dir: &'static str,
	pub scan_invalid_dir: &'static str,
	pub err_create_dir: &'static str,
	pub err_scan_dir: &'static str,
	pub info_find_dir: &'static str,
	pub pas_fichier_txt: &'static str,
	pub pas_trouve: &'static str,
	pub err_cd_dir: &'static str,
	pub info_scan_dir: &'static str,
	pub err_cmd_find: &'static str,
	pub message_final: &'static str,
}

fn get_system_lang() -> String {
	let raw_lang = std::env::var("LC_ALL")
		.or_else(|_| env::var("LANG"))
		.or_else(|_| env::var("LANGUAGE"))
		.unwrap_or_else(|_| "en".to_string()); // Langue par défaut (anglais)

	// Extraire uniquement le code de langue avant le premier '_'
	let lang_code = raw_lang.split('_').next().unwrap_or(&raw_lang);
	lang_code.to_string() // Retourne "fr" au lieu de "fr_CA.UTF-8"
}


pub fn set_arc_locale() -> Strings {
	let system_lang = get_system_lang();
	// Table de correspondance des langues
	let mut lang_map = HashMap::new();
	lang_map.insert("en", Lang::En);
	lang_map.insert("fr", Lang::Fr);
	lang_map.insert("es", Lang::Es); // Ajout d'autres langues facilement

	let lang = lang_map.get(system_lang.as_str()).unwrap_or(&Lang::En); // Défaut : anglais

	get_strings(*lang)
}

pub fn get_strings(lang: Lang) -> Strings {
	match lang {
		Lang::En => Strings {
			options: OPTIONS_EN,
			usage: "-f <pattern> | -s <folder>",
			no_home: "Error: Unable to fetch HOME environment variable.",
			un_ou_lautre: "You must specify either -f/--find or -s/--scan.",
			f_besoin_valeur: "Error: Argument -f/--find requires a value.",
			s_besoin_valeur: "Error: Argument -s/--scan requires a value.",
			arg_inconnu: "Error: Unknown argument",
			f_s_mutually: "Error: You must specify either -f/--find or -s/--scan, not both.",
			prob_recherche: "Error {1}: Problem with search in '{2}'. {3}",
			invalid_dir: "The folder '{1}' does not exist or is not valid.",
			scan_invalid_dir: "The scan folder '{1}' does not exist or is not valid.",
			err_create_dir: "Error: Unable to create directory '{1}': {2}.",
			err_scan_dir: "Error {1}: Unable to scan directory '{2}': {3}.",
			info_find_dir: "Searching pattern: '{1}', in directory: '{2}'",
			pas_fichier_txt: "The folder '{1}'\ndoes not contain any .txt files to perform the search.",
			pas_trouve: "No file with the pattern '{1}' was found.",
			err_cd_dir: "Error: Unable to change directory",
			info_scan_dir: "Scanning files in directory:",
			err_cmd_find: "Error executing find command",
			message_final: "Scan completed ! Results saved in",
		},
		Lang::Fr => Strings {
			options: OPTIONS_FR,
			usage: "-f <motif> | -s <dossier>",
			no_home: "Erreur: Impossible de récupérer la variable d'environnement HOME.",
			un_ou_lautre: "Vous devez spécifier soit -f/--find, soit -s/--scan.",
			f_besoin_valeur: "Erreur: L'argument -f/--find nécessite une valeur.",
			s_besoin_valeur: "Erreur: L'argument -s/--scan nécessite une valeur.",
			arg_inconnu: "Erreur: Argument inconnu",
			f_s_mutually: "Erreur: Vous devez spécifier soit -f/--find, soit -s/--scan, pas les deux.",
			prob_recherche: "Erreur {1}: Problème lors de la recherche dans '{2}'. {3}",
			invalid_dir: "Le dossier '{1}' n'existe pas ou n'est pas valide",
			scan_invalid_dir: "Le dossier d'analyse '{1}' n'existe pas ou n'est pas valide.",
			err_create_dir: "Erreur: Impossible de créer le dossier '{1}': {2}",
			err_scan_dir: "Erreur {1}: Impossible de scanner le dossier '{2}': {3}.",
			info_find_dir: "Recherche du motif: '{1}', Dans le dossier: '{2}'",
			pas_fichier_txt: "Le dossier '{1}'\nne contient aucun fichier .txt pour effectuer la recherche.",
			pas_trouve: "Aucun fichier contenant le motif '{1}' n'a été trouvé.",
			err_cd_dir: "Erreur: Impossible de changer de répertoire",
			info_scan_dir: "Scan des fichiers du répertoire:",
			err_cmd_find: "Erreur lors de l'exécution de la commande find",
			message_final:"Scan terminé ! Résultats enregistrés dans"
		},
		Lang::Es => Strings {
			options: OPTIONS_ES,
			usage: "-f <Patrón> | -s <directorio>",
			no_home: "Error: No se puede recuperar la variable de entorno HOME.",
			un_ou_lautre: "Debe especificar -f/--find o -s/--scan.",
			f_besoin_valeur: "Error: El argumento -f/--find requiere un valor.",
			s_besoin_valeur: "Error: El argumento -s/--scan requiere un valor.",
			arg_inconnu: "Error: Argumento desconocido",
			f_s_mutually: "Error: Debe especificar -f/--find o -s/--scan, no ambos.",
			prob_recherche: "Error {1}: Problema al buscar en '{2}'. {3}",
			invalid_dir: "La carpeta '{1}' no existe o no es válida.",
			scan_invalid_dir: "El directorio de escaneo '{1}' no existe o no es válido.",
			err_create_dir: "Error: No se puede crear el directorio '{1}': {2}.",
			err_scan_dir: "Error {1}: No se puede escanear el directorio '{2}': {3}.",
			info_find_dir: "Búsqueda del motivo: '{1}', En la carpeta: '{2}'",
			pas_fichier_txt: "La carpeta '{1}'\nno contiene archivos .txt para realizar la búsqueda.",
			pas_trouve: "No se encontró ningún archivo con el patrón '{1}'.",
			err_cd_dir: "Error: No se puede cambiar de directorio",
			info_scan_dir: "Escaneando archivos en el directorio:",
			err_cmd_find: "Error al ejecutar el comando find",
			message_final: "¡Escaneo completado! Resultados guardados en",
		},
	}
}
