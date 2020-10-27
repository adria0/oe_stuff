TEST=$1
( cd evmbin && cargo build )
target/debug/openethereum-evm state-test --std-json $TEST 2> json_oe.tmp 
../holiman-go-ethereum/build/bin/evm --json statetest $TEST 2> json_geth.tmp

echo =========================================================================================================
echo DIFF FOR $TEST
echo =========================================================================================================
../oe_stuff/json_logs_diff/target/debug/json_logs_diff json_geth.tmp json_oe.tmp
