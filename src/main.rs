// get_args()
use std::env;
// hipporead()
// set_permissions()
use std::fs;
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;
// setup
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
extern crate serde_json;

fn main() {
    get_args();
    set_permissions();
    create_dictionary();
}
fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_dictionary() -> HashMap<String, String> {
    let mut dictionary: HashMap<String, String> = HashMap::new();
    let path = "auth_data.json";
    let keys = ["intra_user_key", "intra_pass_key", "author_name", "github_user", "github_profile"];
    let prompts = [
        "Enter Your Intranet Username:",
        "Enter Your Intranet Password:",
        "Enter Your Full Name:",
        "Enter Your Github Username:",
        "Enter Your GitHub Profile Link:",
    ];
    if !Path::new(path).exists() {
	 for (key, prompt) in keys.iter().zip(prompts.iter()) {
             println!("{}", prompt);
             let input = read_input();
             dictionary.insert(key.to_string(), input);
	 }
	 let json = serde_json::to_string(&dictionary).unwrap();

	 let mut file = File::create("auth_data.json").unwrap();
	 file.write_all(json.as_bytes()).unwrap();
	 println!("Data written to auth_data.json");
     } else {
	 let file2 = fs::read_to_string(path).unwrap();
	 dictionary =  serde_json::from_str(&file2).unwrap();
     }

    dictionary
}

fn get_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("[ERROR] Too many arguments (must be one), but we're working on allowing more.");
        std::process::exit(1);
    } else if args.len() < 2 {
        println!("[ERROR] Too few arguments (must be one)");
        std::process::exit(1);
    }
    return args[1].clone();
}
pub fn set_permissions() {
    let entries = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(err) => {
            println!("[ERROR] Failed to read directory: {}", err);
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                println!("[ERROR] Failed to read entry: {}", err);
                continue;
            }
        };
	println!("  -> Validating permissions on {}", entry.path().display());
        let path = entry.path();
	let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(err) => {
                println!("     [ERROR] Failed to retrieve file metadata: {}", err);
                continue;
            }
        };

        if metadata.len() == 0 {
	    println!("       -> NOTE: File is not the correct type for X permissions: {}", entry.path().display());
            continue;
        }

        let file = match fs::File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                println!("     [ERROR] Failed to open file: {}", err);
                continue;
            }
        };
	let mut reader = BufReader::new(file);
	let mut buf = [0; 2];
	match reader.read_exact(&mut buf) {
	    Ok(n) => n,
	    Err(err) => {
		println!("     [ERROR] Failed to read from file {} : {}", entry.path().display(), err);
		continue;
	    }
	};

	if &buf == b"#!" {
	    println!("     -> Setting permissions on {}", entry.path().display());
	    let mut perms = match fs::metadata(&path) {
		Ok(metadata) => metadata.permissions(),
		Err(err) => {
		    println!("     [ERROR] Failed to retrieve file metadata: {}", err);
		    continue;
		}
	    };
	    perms.set_mode(0o755);
	    match fs::set_permissions(&path, perms) {
		Ok(()) => (),
		Err(err) => println!("     [ERROR] Failed to set permissions: {}", err),
	    }
	} else {
	    println!("       -> File doesn't need X: {}", entry.path().display())
	}
    }
    println!("done");
}
