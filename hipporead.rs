use std::env;
use std::fs::File;
use std::io::Write;

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

fn hipporead() {
    println!("\nHipposcraper version 1.1.1");
    println!("Creating README.md file:");

    let link = get_args();
    let parse_data = BaseParse::new(link);

    println!("  -> Scraping information...");
    let r_scraper = ReadScraper::new(parse_data.soup);
    println!("done");

    let mut file = File::create("README.md").unwrap();

    file.write_all(r_scraper.write_title().as_bytes()).unwrap();
    file.write_all(r_scraper.write_rsc().as_bytes()).unwrap();
    file.write_all(r_scraper.write_info().as_bytes()).unwrap();
    file.write_all(r_scraper.write_tasks().as_bytes()).unwrap();

    let author = parse_data.json_data.author_name.to_string();
    let user = parse_data.json_data.github_username.to_string();
    let git_link = parse_data.json_data.github_profile_link.to_string();

    file.write_all(r_scraper.write_footer(author, user, git_link).as_bytes()).unwrap();

    println!("README.md all set!");
}
