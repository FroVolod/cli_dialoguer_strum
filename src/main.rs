use structopt::StructOpt;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::io::Result;
use std::str::FromStr;
use strum_macros::{
    Display,
    EnumString,
    EnumVariantNames,
};
use strum::VariantNames;


#[derive(Debug, StructOpt)]
struct CliArgs {
    #[structopt(long)]
    name: String,
    #[structopt(subcommand)]
    subcommand: CliCommand,
}

#[derive(Debug, Display, EnumString, EnumVariantNames, StructOpt)]
#[strum(serialize_all = "kebab_case")]
enum CliCommand {
    ConstructTransactionCommand,
    Utils,
}

impl CliCommand {
    fn choose_command() -> Result<()> {
        let commands= CliCommand::VARIANTS;
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your action")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => {
                println!("============== {}", commands[0]);
            },
            Some(1) => println!("++++++++++++++ {}", commands[1]),
            _ => println!("-----------"),
        };
        Ok(())
    }
}

fn main() {
    println!("%%%%%%%.  list of CliCommand: {:?}", CliCommand::VARIANTS);
    CliCommand::choose_command();
}
