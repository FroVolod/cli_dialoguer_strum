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

use crate::consts;
use consts::{
    TESTNET_API_SERVER_URL,
    MAINNET_API_SERVER_URL,
    BETANET_API_SERVER_URL,
};
mod server;
use server::{
    Server,
    ActionSubcommand
};


#[derive(Debug, StructOpt)]
pub struct OnOffLineMode {
    #[structopt(subcommand)]
    mode: Mode
}

#[derive(Debug, Display, EnumVariantNames, StructOpt)]
enum Mode {
    Online(OnlineArgs),
    Offline(OfflineArgs),
}

#[derive(Debug, StructOpt)]
struct OfflineArgs {
    #[structopt(long)]
    online: bool,
    sender: String,
    // #[structopt(long)]
    // block_height: u64,
}

#[derive(Debug, Default, StructOpt)]
struct OnlineArgs {
    #[structopt(long)]
    online: bool,
    // network: String,
    sender: String,
    receiver: String,
    #[structopt(subcommand)]
    selected_server: SelectServer
}

#[derive(Debug, Display, EnumString,  EnumVariantNames, StructOpt)]
enum SelectServer {
    Testnet(Server),
    Mainnet(Server),
    Betanet(Server),
    Custom(Server),
}

impl Default for SelectServer {
    fn default() -> Self {
        SelectServer::Testnet(Server{
            url: TESTNET_API_SERVER_URL.to_string(),
            transaction_subcommand: ActionSubcommand::default()
        })
    }
}

impl OnOffLineMode {
    pub fn choose_command() -> Self {
        let choose_mode= vec![
            "Yes, I keep it simple",
            "No, I want to work in no-network (air-gapped) environment"
        ];
        println!("\n");
        let select_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
                 \nDo you want to derive some information required for transaction construction automatically querying it online?"
            )
            .items(&choose_mode)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        
        let servers= SelectServer::VARIANTS;
        let select_server = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select NEAR protocol RPC server:")
            .items(&servers)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();
        let custom_api_server_url: String = "".to_string();

        let transaction_subcommand= ActionSubcommand::choose_action_command();
        let selected_server: SelectServer = match select_server {
            // Some(num) => SelectServer::from_str(servers[num]).unwrap() ,
            Some(0) => SelectServer::Testnet(Server{
                url: TESTNET_API_SERVER_URL.to_string(),
                transaction_subcommand
            }),
            Some(1) => SelectServer::Mainnet(Server{
                url: MAINNET_API_SERVER_URL.to_string(),
                transaction_subcommand
            }),
            Some(2) => SelectServer::Betanet(Server{
                url: BETANET_API_SERVER_URL.to_string(),
                transaction_subcommand
            }),
            Some(3) => {
                SelectServer::Custom(Server{
                    url: custom_api_server_url,
                    transaction_subcommand
                })
            },
            _ => SelectServer::Betanet(Server{
                url: BETANET_API_SERVER_URL.to_string(),
                transaction_subcommand
            })
        };
        let sender : String = Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap();
        let receiver : String = Input::new()
            .with_prompt("What is the account ID of the receiver?")
            .interact_text()
            .unwrap();
        
        match select_mode {
            Some(0) => {
                println!("============== {:?}", choose_mode[0]);
                Self {
                    mode: Mode::Online(OnlineArgs {
                        sender,
                        receiver,
                        online: true,
                        selected_server
                    }) 
                }
            },
            Some(1) => {
                println!("============== {:?}", choose_mode[1]);
                Self {
                    mode: Mode::Offline(OfflineArgs {
                        sender,
                        online: false,
                    })
                }
            }
            _ => unreachable!("Error")
        }
    }
}
