#/bin/bash

OE=/home/adria/bin/oe-3.0.0-alpha 
#OE=/home/adria/bin/parity-2.7.2-stable 
#OE=/home/adria/bin/parity-2.5.13-stable 

BOOTNODES="enode://14b3f9ff18928230ec35bc0938ee8a0f6d29cfa1172e385995a47db6528f9e62d2ea46b2bdd112f720d0b35cb4aba741cede4aa1ae19c17fa82727ecbce8053c@127.0.0.1:30303,enode://9de648d5d7f60c6e0d2ceb6785323dffeeaeb03b9bd18219ac803d424d466718fc56ee1d9984c320e3d1ee019f2fd0733ab603c73fa2b8996f9207a2991b636c@127.0.0.1:30304,enode://3b8e57ca809a61049abe2a97219eb931039eb98be98fe4a61bc849e2f20b571d72d13af03e1a6647b711b1e03d721256e926d814e3e90d707a4054a5608fd08b@127.0.0.1:30305"
CONFIG="--chain=chain.json --config=gateway.toml --password ./password.txt --bootnodes $BOOTNODES"  
EXTRA="--no-ancient-blocks"  
PARAMS="$CONFIG $EXTRA" 

rm -rf node4/cache node4/chains

echo ====================================
echo `$OE --version`
echo ====================================
echo ======= PRESS ENTER TO STOP ========
echo ====================================

$OE --identity=node4 --base-path=node4 --port=30305 --unlock=0xcfa3ae1840e38d1e54b0ef6300d6e91b22964a75 $PARAMS 2>&1 |(sed 's/^/[1] /') &
NODE4PID=$!

read x
kill NODE4PID 


