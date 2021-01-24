use structopt::StructOpt;
use dialoguer::{
    Select,
    Input,
    theme::ColorfulTheme,
    console::Term
};
use std::num::ParseIntError;
use std::str::FromStr;

use strum_macros::{
    Display,
    EnumString,
    EnumVariantNames,
};
use strum::VariantNames;

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
