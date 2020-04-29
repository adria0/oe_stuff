#!/bin/bash
echo VERSION `shasum $0`
echo RPC $1

RPC=$1
BLOCK_XM=("0x4C4B40" "0x5B8D80" "0x6ACFC0" "0x7A1200" "0x895440" "0x9815CF")
OUTPUT="-s -o /tmp/none"
ITER=3
TIME="/usr/bin/time --f %e"

# seq eth_getLogs ----------------------------------------------------------------
for (( i=0; i<${#BLOCK_XM[@]}-1 ; i++ ))
do
	blockFrom="${BLOCK_XM[$i]}"
	blockTo="${BLOCK_XM[(($i+1))]}"
	elapseds=
	for (( j=0; j<$ITER; j++ ));
	do
		elapsed=$($TIME curl -X POST -H "Content-Type: application/json" --data '{"id":1,"jsonrpc":"2.0","method":"eth_getLogs","params":[{"fromBlock":"'$blockFrom'","toBlock":"'$blockTo'","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",null,["0x00000000000000000000000031a0a43978171be41ea7c5d60b0a3afb475fbb8a"]]}]}' $RPC $OUTPUT 2>&1)
		elapseds="$elapseds $elapsed"
	done
	echo SEQUENTIAL ETH_GETLOGS $blockFrom $blockTo $elapseds
done

# par eth_getLogs ----------------------------------------------------------------
for (( i=0; i<${#BLOCK_XM[@]}-1 ; i++ ))
do
	blockFrom="${BLOCK_XM[$i]}"
	blockTo="${BLOCK_XM[(($i+1))]}"

	pids=()
	for (( j=0; j<$ITER; j++ ));
	do
		curl -X POST -H "Content-Type: application/json" --data '{"id":1,"jsonrpc":"2.0","method":"eth_getLogs","params":[{"fromBlock":"'$blockFrom'","toBlock":"'$blockTo'","topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",null,["0x00000000000000000000000031a0a43978171be41ea7c5d60b0a3afb475fbb8a"]]}]}' $RPC $OUTPUT &
		pids+=($!)
	done
	
	start=`date +%s`
	for pid in ${pids[@]};
	do
		wait $pid
	done
	end=`date +%s`
	echo "PARALLEL   ETH_GETLOGS $blockFrom $blockTo $((end-start))"
done


# 100Kblocks filter -------------------------------------------------------------
elapseds=
for (( j=0; j<$ITER; j++ ));
do
	elapsed=$($TIME curl -X POST -H "Content-Type: application/json" --data '{"method":"trace_filter","params":[{"fromBlock":"0x968DAB","toBlock":"0x98144B","toAddress":["0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A"]}],"id":1,"jsonrpc":"2.0"}' $RPC $OUTPUT 2>&1)
	elapseds="$elapseds $elapsed"
done
echo SEQUENTIAL TRACE_FILTER $elapseds

