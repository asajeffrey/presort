OUTDATA="target/data/chance.data"

# invoke benchmark program
BENCH="cargo run --release --example presort_bench --"
ARGS="--tag chance -t 50 -d 13 -n 10000 -e 100 -s 0 -a 0 "

# clear old data
rm $OUTDATA

for ver in vec presort presort_pad; do
	
	# header
	$BENCH -h -t 0 -o $OUTDATA

	# run benches
	for chance in `seq 0 0.1 1`; do
		$BENCH $ARGS --$ver -c $chance -o $OUTDATA
	done

	#separate by 2 lines for gnuplot data indexes
	echo >> $OUTDATA
	echo >> $OUTDATA

done
