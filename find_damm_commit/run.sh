# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# sudo apt install build-essential llvm libssl-dev clang pkg-config

if [ "$#" -lt 1 ]; then
    echo "Illegal number of parameters"
    exit
fi
COMMIT=$1
OEARG=$2
( cd ../oe_monitor && cargo build --release )
git clone http://github.com/openethereum/openethereum
git checkout $1
( cd openethereum && cargo build --release --features=final )

mkdir data
openethereum/target/release/openethereum $OEARG -d data --log-file log.$COMMIT$OEARG.oe &
PID=$!
../oe_monitor/target/release/oe_monitor $PID > log.$COMMIT$OEARG.monitor &
