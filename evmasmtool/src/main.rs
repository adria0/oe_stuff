
use clap::{App, Arg, SubCommand, AppSettings};

mod evmcodegen;
use evmcodegen::EVMCodeGen;
use rustc_hex::{FromHex, ToHex};
use easycontract::{Account,EasyContract};
use web3::futures::Future;
use web3::types::*;
use anyhow::{Error, Result};

/*
fn deploy() {
    let rpc_url = "https://rinkeby.infura.io/v3/";
    let (eloop, transport) = web3::transports::Http::new(rpc_url)
        .expect("cannot create web3 connector");
    eloop.into_remote();
    let web3 = web3::Web3::new(transport);

    let account1  = Account::from_secret_key("");
    let address = EasyContract::deploy(&web3, &account1, b.gen_tx_code(),U256::zero()).expect("cannot deploy contract");

    assert_eq!("address",format!("{}",address));
}
*/

fn main() -> Result<()> {
    let matches = App::new("evmasmtool")
        .version("1.0")
        .author("@adria0")
        .about("Tool to assembly evm")
        .subcommand(
            SubCommand::with_name("asm")
                .about("assemble code")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("code"))
        )
        .get_matches();

    // You can see which subcommand was used
    match matches.subcommand() {
        ("asm",Some(args)) => {
            if let Some(code) = args.value_of("code") {
                let code = if code.starts_with("@") {
                    std::fs::read_to_string(&code[1..])?
                } else {
                   code.replace(',', "\n")
                };
                let code : String = EVMCodeGen::asm(&code)?.to_hex();
                println!("{}",code);
            }
        },
        ("", None) => println!("No subcommand specified"),
        _ => unreachable!()
    }

    Ok(())
}