mod conf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCommand>,
}
#[derive(Subcommand)]
enum SubCommand {
    Status { row: usize },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(SubCommand::Status { row }) => {
            if row == 0 {
                println!("{}", conf::status::row0());
            } else if row == 1 {
                println!("{}", conf::status::row1());
            }
        }
        None => {
            let _ = conf::conf_tmux().spawn();
        }
    }
}
