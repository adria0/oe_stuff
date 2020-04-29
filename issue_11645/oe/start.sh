#/bin/bash

OE=/home/adria/bin/oe-3.0.0-alpha 
#OE=/home/adria/bin/parity-2.7.2-stable 
#OE=/home/adria/bin/parity-2.5.13-stable 

BOOTNODES="enode://14b3f9ff18928230ec35bc0938ee8a0f6d29cfa1172e385995a47db6528f9e62d2ea46b2bdd112f720d0b35cb4aba741cede4aa1ae19c17fa82727ecbce8053c@127.0.0.1:30303,enode://9de648d5d7f60c6e0d2ceb6785323dffeeaeb03b9bd18219ac803d424d466718fc56ee1d9984c320e3d1ee019f2fd0733ab603c73fa2b8996f9207a2991b636c@127.0.0.l:30304,enode://3b8e57ca809a61049abe2a97219eb931039eb98be98fe4a61bc849e2f20b571d72d13af03e1a6647b711b1e03d721256e926d814e3e90d707a4054a5608fd08b@127.0.0.1:30305"
CONFIG="--chain=chain.json --config=config.toml --password ./password.txt --bootnodes $BOOTNODES"  
EXTRA="--no-ancient-blocks"  
PARAMS="$CONFIG $EXTRA" 

rm -rf node1/cache node1/chains
rm -rf node2/cache node2/chains
rm -rf node3/cache node3/chains

echo ====================================
echo `$OE --version`
echo ====================================
echo ======= PRESS ENTER TO STOP ========
echo ====================================

$OE --identity=node1 --base-path=node1 --port=30303 --unlock=55f1191d5e195cd50a67b30bdfc3a47b1b19cde3 --engine-signer=55f1191d5e195cd50a67b30bdfc3a47b1b19cde3 $PARAMS 2>&1 |(sed 's/^/[1] /') &
NODE1PID=$!

$OE --identity=node2 --base-path=node2 --port=30304 --unlock=73b1e510fafefc83fb5738d3314f78731aad1f37 --engine-signer=73b1e510fafefc83fb5738d3314f78731aad1f37 $PARAMS 2>&1 |(sed 's/^/[2] /') & 
NODE2PID=$!

$OE --identity=node3 --base-path=node3 --port=30305 --unlock=a65be5e25b8763584c8e16070500bb55dfc92dba --engine-signer=a65be5e25b8763584c8e16070500bb55dfc92dba -$PARAMS 2>&1 |(sed 's/^/[3] /') &
NODE3PID=$!

read x
kill $NODE1PID $NODE2PID $NODE3PID 


