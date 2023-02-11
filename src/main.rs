// get_args()
use std::env;
// hipporead()
// set_permissions()
use std::fs;
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;

fn main() {
    get_args();
    set_permissions();
    println!("Hello, world!");
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
	    println!("  -> NOTE: File is empty: {}", entry.path().display());
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
	    println!("  -> Setting permissions on {}", entry.path().display());
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
	    println!("       -> File has no business with X perms: {}", entry.path().display())
	}
    }
    println!("done");
}
