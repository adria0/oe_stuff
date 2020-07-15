# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# source $HOME/.cargo/env
# sudo apt update
# sudo apt install -y cmake build-essential llvm libssl-dev clang pkg-config
# git clone http://github.com/adria0/oe_stuff
# cd oe_stuff/find_damm_commit
# tmux
# --no-periodic-snapshot

if [ "$#" -lt 1 ]; then
    echo "Illegal number of parameters"
    exit
fi
COMMIT=$1
OEARG=$2
BOOTNODES=--bootnodes enode://68f46370191198b71a1595dd453c489bbfe28036a9951fc0397fabd1b77462930b3c5a5359b20e99677855939be47b39fc8edcf1e9ff2522a922b86d233bf2df@144.217.153.76:30303,enode://ffed6382e05ee42854d862f08e4e39b8452c50a5a5d399072c40f9a0b2d4ad34b0eb5312455ad8bcf0dcb4ce969dc89a9a9fd00183eaf8abf46bbcc59dc6e9d5@51.195.3.238:30303
( cd ../oe_monitor && cargo build --release )
git clone http://github.com/openethereum/openethereum
( cd openethereum && git checkout $1 )
read -p "Press enter to continue"
( cd openethereum && cargo build --locked --release --features=final )
mkdir data
openethereum/target/release/parity $OEARG $BOOTNODES -d data --log-file log.$COMMIT$OEARG.oe &
PID=$!
../oe_monitor/target/release/oe_monitor $PID > log.$COMMIT$OEARG.monitor &
