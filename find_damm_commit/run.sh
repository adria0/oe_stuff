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
openethereum/target/release/openethereum $OEARG -d data --log-file $COMMIT$OEARG.oe.log &
PID=$!
../oe_monitor/target/release/oe_monitor $PID > $COMMIT$OEARG.monitor.log &
