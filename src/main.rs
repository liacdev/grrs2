use clap::Parser;

/// Search for  a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let _args = Cli::parse();

    let content = std::fs::read_to_string(&_args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&_args.pattern) {
            println!("{}", line);
        }
    }

}
