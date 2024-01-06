use clap_mangen::Man;

include!("src/args.rs"); // Adjust the path to where your Cli struct is defined

fn main() {
    let app = Args::command();
    let mut file = File::create("dirdiff.1").expect("Could not create man page file");
    Man::new(app).render(&mut file).expect("Could not render man page");
}

