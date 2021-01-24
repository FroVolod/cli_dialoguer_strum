use structopt::StructOpt;
use strum_macros::{
    Display,
    EnumString,
    EnumVariantNames,
};
use strum::VariantNames;
use dialoguer::{
    Select,
    Input,
    theme::ColorfulTheme,
    console::Term
};

mod on_off_line_mode;
use on_off_line_mode::OnOffLineMode;


#[derive(Debug, Display, EnumVariantNames, StructOpt)]
#[strum(serialize_all = "kebab_case")]
pub enum CliCommand {
    ConstructTransactionCommand(OnOffLineMode),
    Utils,
}

impl CliCommand {
    pub fn choose_command() -> Self {
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
                Self::ConstructTransactionCommand(OnOffLineMode::choose_command())
            },
            Some(1) => {
                println!("============== {:?}", commands[1]);
                Self::Utils
            }
            _ => unreachable!("Error")
        }
    }
}
