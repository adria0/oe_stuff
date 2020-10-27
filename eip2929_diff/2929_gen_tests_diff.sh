cargo test --release --features=json-tests --all ethereum_json_tests  | grep "post state root mismatch" > 2929_oe.log
sed -i s/0x//g 2929_oe.log
sed -i 's/\/res\/ethereum\/tests\/GeneralStateTests//g' 2929_oe.log
../oe_stuff/eip2929_diff/target/debug/diff 
../oe_stuff/eip2929_diff/target/debug/diff > 2929_mismatch
