use std::fs;
use std::process;

use clap::Parser;
use colored::Colorize;
use symlink::remove_symlink_dir;

use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR};
use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk uninstall",
    about = "sdk subcommand to remove a specific candidate version"
)]
struct Args {
    #[arg(short = 'f', long = "force")]
    force: bool,

    #[arg(required(true))]
    candidate: String,

    #[arg(required(true))]
    version: String,
}

fn main() {
    let args = Args::parse();
    let candidate = args.candidate;
    let version = args.version;
    let force = args.force;
    let sdkman_dir = infer_sdkman_dir();

    let all_candidates = known_candidates(sdkman_dir.to_owned());
    if !all_candidates.contains(&candidate.as_str()) {
        eprint!("{} is not a valid candidate.", candidate.bold());
        process::exit(1);
    }

    let candidate_path = sdkman_dir.join(CANDIDATES_DIR).join(&candidate);
    let version_path = sdkman_dir.join(&candidate_path).join(&version);
    let current_link_path = candidate_path.join(CURRENT_DIR);
    if current_link_path.is_dir() {
        let canonical_link_path =
            fs::read_link(current_link_path.to_owned()).expect("panic! can't read link");
        if version_path == canonical_link_path && force {
            remove_symlink_dir(current_link_path).expect("panic! can't remove current symlink");
        } else if version_path == canonical_link_path && !force {
            eprint!(
                "\n{} {} is the {} version and should not be removed.",
                candidate,
                version,
                "current".bold(),
            );
            println!(
                "\n\nOverride with {}, but leaves the candidate unusable!",
                "--force".italic()
            );
            process::exit(1);
        }
    }

    if version_path.is_dir() {
        fs::remove_dir_all(version_path).expect("panic! could not delete directory");
        println!("removed {} {}", candidate.bold(), version.bold());
    } else {
        eprintln!(
            "{} {} is not installed on your system.",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}
