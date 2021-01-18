use structopt::StructOpt;
use dialoguer::{
    Select,
    Input,
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

mod construct_transaction_command;


#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long)]
    name: String,
    #[structopt(subcommand)]
    subcommand: CliCommand,
}


#[derive(Debug, Default, StructOpt)]
struct CliArgs {
    #[structopt(long)]
    name: Option<String>,
    #[structopt(subcommand)]
    subcommand: Option<CliCommand>,
}

#[derive(Debug, Display, EnumString, EnumVariantNames, StructOpt)]
#[strum(serialize_all = "kebab_case")]
enum CliCommand {
    ConstructTransactionCommand(OnOffLine),
    Utils,
}

#[derive(Debug, Default, StructOpt)]
struct OnOffLine {
    #[structopt(long)]
    online: bool
}

trait ChooseCommand {
    fn choose_command() -> Self;
}

impl ChooseCommand for CliCommand {
    fn choose_command() -> Self {
        let commands= Self::VARIANTS;
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your action")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => {
                println!("============== {:?}", commands[0]);
                Self::ConstructTransactionCommand(OnOffLine::choose_command())
            },
            Some(1) => {
                println!("============== {:?}", commands[1]);
                Self::Utils
            }
            _ => unreachable!("Error")
        }
    }
}

impl ChooseCommand for OnOffLine {
    fn choose_command() -> Self {
        let commands= vec![
            "Yes, I keep it simple",
            "No, I want to work in no-network (air-gapped) environment"
        ];
        println!("\n");
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
                 \nDo you want to derive some information required for transaction construction automatically querying it online?"
            )
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => {
                println!("============== {:?}", commands[0]);
                OnOffLine {online: true}
            },
            Some(1) => {
                println!("============== {:?}", commands[1]);
                OnOffLine {online: false}
            }
            _ => unreachable!("Error")
        }
    }
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
    println!("args {:?}", args)
}
