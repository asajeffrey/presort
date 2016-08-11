OUTDATA="target/data/chance.data"

# invoke benchmark program
BENCH="cargo run --release --example presort_bench --"
ARGS="--tag chance -t 1 -d 13 -n 10000 -r 0 -a 0 -e 100"

for ver in vec presort presort_pad; do
	
	# clear old data
	rm ${OUTDATA}.$ver
	
	# header
	$BENCH -h -t 0 -o ${OUTDATA}.$ver

	# run benches
	for chance in `seq 0 0.01 1`; do
		$BENCH $ARGS --$ver -c $chance -o ${OUTDATA}.$ver
	done

done
