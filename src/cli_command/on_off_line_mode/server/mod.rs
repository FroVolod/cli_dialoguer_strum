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

mod transfer_near_tokens_type;
use transfer_near_tokens_type::TransferNEARTokens;


#[derive(Debug, Default, StructOpt)]
pub struct Server {
    pub url: String,
    #[structopt(subcommand)]
    pub transaction_subcommand: ActionSubcommand
}

#[derive(Debug, EnumVariantNames, StructOpt)]
pub enum ActionSubcommand {
    TransferNEARTokens(TransferNEARTokens),
    CallFunction,
    StakeNEARTokens,
    CreateAccount,
    DeleteAccount,
    AddAccessKey,
    DeteteAccessKey,
    Skip
}

impl Default for ActionSubcommand {
    fn default() -> Self {
        ActionSubcommand::Skip
    }
}

impl ActionSubcommand {
    pub fn choose_action_command() -> Self {
        let action_subcommands= ActionSubcommand::VARIANTS;
        let select_action_subcommand = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&action_subcommands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        match select_action_subcommand {
            Some(0) => ActionSubcommand::TransferNEARTokens(TransferNEARTokens::input_amount()),
            Some(1) => ActionSubcommand::CallFunction,
            _ => ActionSubcommand::default()
        }
    }
}
