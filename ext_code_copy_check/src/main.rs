use core::str::FromStr;
use easycontract::EasyContract;
use std::collections::HashSet;
use web3::futures::Future;
use web3::types::*;

/*
    function check(address[] calldata _addrs) external view returns (uint256) {
        uint256 o_code;
        assembly {
            o_code := mload(0x40)
        }
        uint256 start_gas = gasleft();
        for (uint256 i =0 ; i<_addrs.length; ++i) {
            address _addr = _addrs[i];
            assembly {
                extcodecopy(_addr, o_code, 0, 1)
            }
        }
        return start_gas-gasleft();
    }
    event Run();
    function run(address[] calldata _addrs) external {
        uint256 o_code;
        assembly {
            o_code := mload(0x40)
        }
        uint256 start_gas = gasleft();
        for (uint256 i =0 ; i<_addrs.length; ++i) {
            address _addr = _addrs[i];
            assembly {
                extcodecopy(_addr, o_code, 0, 1)
            }
        }
        emit Run();
    }
*/
const ABI: &str = r#"
[
        {
                "inputs": [
                        {
                                "internalType": "address[]",
                                "name": "_addrs",
                                "type": "address[]"
                        }
                ],
                "name": "check",
                "outputs": [
                        {
                                "internalType": "uint256",
                                "name": "",
                                "type": "uint256"
                        }
                ],
                "stateMutability": "view",
                "type": "function"
        }
]
"#;

const ADDRESS: &str = "cE0BD20292940F2D8484497FF62B4AcD1d985c7C";
const SCRAP_FILE: &str = "contract_list.txt";

fn new_web3() -> web3::Web3<web3::transports::Http> {
    let (eloop, transport) =
        web3::transports::Http::new("http://localhost:8545").expect("cannot create web3 connector");

    eloop.into_remote();
    web3::Web3::new(transport)
}

fn do_call<T : web3::Transport >(contract: &EasyContract<T>, addrs : Vec<Address>) {
    let len = addrs.len();
    let start = std::time::Instant::now();
    let gas_used: U256 = contract
        .query("check", addrs, None)
        .expect("cannot call check");

    println!("gas used in call with {} contracts: {} {}s", len, gas_used.as_u64(), start.elapsed().as_secs_f64());
}

/*
./openethereum --chain=dev --jsonrpc-interface=all --unsafe-expose --jsonrpc-cors=all --tx-gas-limit=20000000
fn do_tx(addrs : Vec<Address>) {
    let web3 = new_web3();
    let len = addrs.len();
    let parity_dev  = Account::from_secret_key("4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7");

    let evmcode = (include_str!("../code_sol_ext_code_size_check.bin"))
            .from_hex::<Vec<u8>>()
            .expect("cannot parse evmcode");

    let abi_json = include_bytes!("../code_sol_ext_code_size_check.abi");
    let address = EasyContract::deploy(&web3, &parity_dev, evmcode,U256::zero())
            .expect("cannot deploy contract");

    let contract = EasyContract::from_json(&web3, address, abi_json)
          .expect("cannot assign to contract");

    let start = std::time::Instant::now();
    let recipt = contract
        .call("check", addrs, &parity_dev, U256::zero())
        .expect("cannot tx check");

    println!("gas used in tx with {} contracts: {} {}s", len, recipt.gas_used.unwrap().as_u64(), start.elapsed().as_secs_f64());
}
*/

fn scrap_contracts() {
    let web3 = new_web3();
    let mut contracts = HashSet::new();
    for block_no in 1_000_000..10_000_000 {
        let block = web3
            .eth()
            .block_with_txs(BlockId::Number(BlockNumber::Number(block_no)))
            .wait().unwrap().unwrap();

        for tx in block.transactions {
            if let Some(to) = tx.to {
                if tx.input.0.len() > 0 {
                    contracts.insert(to);
                }
            }
        }
        if block_no % 1000 == 0 {
            println!("{} {}", block_no, contracts.len());
            std::fs::write(SCRAP_FILE,format!("{:?}",contracts)).unwrap();
        }
    }
}

fn main() {
    let web3 = new_web3();
    let contract =
        EasyContract::from_json(&web3, Address::from_str(ADDRESS).unwrap(), ABI.as_bytes())
            .expect("cannot assign to contract");

    let contracts : Vec<Address> = String::from_utf8_lossy(&std::fs::read(SCRAP_FILE).unwrap())
        .replace("{","").replace("}","").replace(" ","").replace("0x","").split(",")
        .take(14_000)
        .map(|v| Address::from_str(v).unwrap()).collect();


    do_call(&contract,vec![Address::from_str("0000000000000000000000000000000000000000").unwrap()]);
    do_call(&contract,contracts);
}
