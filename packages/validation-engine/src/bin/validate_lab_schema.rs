use clap::Parser;
use kubelab_validation_engine::models::DeclarativeLabDef;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Validate KubeLab Declarative Lab YAML Schema")]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let content = match fs::read_to_string(&args.path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading lab file {:?}: {}", args.path, e);
            std::process::exit(1);
        }
    };

    match serde_yaml::from_str::<DeclarativeLabDef>(&content) {
        Ok(lab) => {
            println!(
                "SUCCESS: Lab '{}' (id: {}) with {} tasks is 100% valid schema.",
                lab.title,
                lab.id,
                lab.tasks.len()
            );
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("VALIDATION FAILED for {:?}: {}", args.path, e);
            std::process::exit(1);
        }
    }
}
