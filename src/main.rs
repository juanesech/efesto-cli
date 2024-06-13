use structopt::StructOpt;
mod data_sources;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage efesto app")]
enum Efesto {
    DataSources {
        #[structopt(subcommand)]
        cmd: Command,
    },
}

#[derive(Debug, StructOpt)]
enum Command {
    Create {
        #[structopt(short, long)]
        interactive: bool,
        #[structopt(short, long)]
        name: String,
    },
}

fn main() {
    let efesto = Efesto::from_args();
    println!("{:?}", efesto);
    match efesto {
        Efesto::DataSources { cmd } => match cmd {
            Command::Create { interactive, name } => {
                if interactive {
                    println!("Is using interactive mode");
                }
                data_sources::create::create(name);
            }
        },
    }
}
