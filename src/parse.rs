// parse.rs

use std::env;
use std::path::Path;

use crate::AppData;

pub fn parse_args(data: &mut AppData) -> bool {

	let args: Vec<String> = env::args().collect();
	let args_len = args.len();

	data.nom = Path::new(&args[0])
				.file_name().and_then(|s| s.to_str())
				.unwrap_or(&data.nom).to_string();

	if args_len == 1 {
		data.mess_erreur = Some(format!("{}: {}\n- - - - - -\n{}",
								data.nom,
								data.locale.usage,
								data.locale.options));
		return false;
	}
	if args_len > 3 {
		data.mess_erreur = Some(data.locale.un_ou_lautre.to_string());
		return false;
	}

	let mut index: usize = 1;

	while index < args_len {
		match args[index].as_str() {

			"-h" => {
				data.commande = Some(String::from("usage"));
				return true;
			}
			"--help" => {
				data.commande = Some(String::from("help"));
				return true;
			}
			"-ver" => {
				data.commande = Some(String::from("ver"));
				return true;
			}
			"--version" => {
				data.commande = Some(String::from("version"));
				return true;
			}

			"-f" | "--find" => {
				if data.commande != None {
					data.mess_erreur = Some(data.locale.f_s_mutually.to_string());
					return false;
				}
				if index + 1 < args_len {
					data.argument = Some(args[index + 1].clone());
					data.commande = Some(String::from("find"));
					index += 1;
				} else {
					data.mess_erreur = Some(data.locale.f_besoin_valeur.to_string());
					return false;
				}
			}

			"-s" | "--scan" => {
				if data.commande != None {
					data.mess_erreur = Some(data.locale.f_s_mutually.to_string());
					return false;
				}
				if index + 1 < args_len {
					data.argument = Some(args[index + 1].clone());
					data.commande = Some(String::from("scan"));
					index += 1;
				} else {
					data.mess_erreur = Some(data.locale.f_besoin_valeur.to_string());
					return false;
				}
			}

			_ => {
				data.mess_erreur = Some(format!("{} '{}'.",
										data.locale.arg_inconnu,
										args[index]));
				return false;
			}
		}
		index += 1;
	}

	return true;
}
