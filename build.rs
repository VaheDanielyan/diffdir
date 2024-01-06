use clap_mangen::Man;
use std::env;

include!("src/args.rs"); // Adjust the path to where your Cli struct is defined

fn main() {
    let app = Args::command();
    let out_dir = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = File::create(out_dir.join("dirdiff.1")).expect("Could not create man page file");
    Man::new(app).render(&mut file).expect("Could not render man page");
}

