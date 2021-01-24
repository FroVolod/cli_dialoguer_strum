use structopt::StructOpt;

mod consts;
mod cli_command;
use cli_command::CliCommand;


#[derive(Debug)]
struct Args {
    name: String,
    subcommand: CliCommand,
}

#[derive(Debug, Default, StructOpt)]
struct CliArgs {
    #[structopt(long)]
    name: Option<String>,
    #[structopt(subcommand)]
    subcommand: Option<CliCommand>,
}

impl From<CliArgs> for Args {
    fn from(item: CliArgs) -> Self {
        Self {
            name: item.name.unwrap_or_default(),
            subcommand: item.subcommand.unwrap_or_else(|| {
                CliCommand::choose_command()
            })
        }
    }
}



fn main() {
    let cli = CliArgs::default();
    println!("cli: {:?}", cli);

    let args = Args::from(cli);
    println!("args {:#?}", args);
}
