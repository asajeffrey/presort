# name of experiment
EXPR="removals"

# files created/overwritten
DATA="../target/data/${EXPR}.data"
PLOT="../target/data/${EXPR}.pdf"

# benchmark program and fixed parameters
BENCH="cargo run --release --example presort_bench --"
ARGS="--tag ${EXPR} -t 50 -e 100 -s 1 -a 0"
VERS="vec presort presort_pad permute permute_pad merge merge_pad"

# Collect Data
# ------------

# skip data collection if there's any parameter
if [ $# = 0 ]; then

	# clear old data
	rm -f $DATA

	for ver in $VERS; do
		
		# header
		$BENCH -h -t 0 -o $DATA

		# run benches
		for nodes in `seq 10000 10000 100000`; do
			# log-2 of nodes
			depth=$(echo "l($nodes)/l(2)" | bc -l)
			# truncate to integer
			depth=$(echo "$depth/1" | bc)
			$BENCH $ARGS --$ver -d $depth -n $nodes -o $DATA
		done

		# separate by 2 lines for gnuplot data indexes
		echo >> $DATA
		echo >> $DATA

	done
fi

# Make plot
# ---------

#clear old plot
rm -f $PLOT

# start plot script
echo "# skip header
set key autotitle columnhead
# select pdf file format
set terminal pdf
# select output file
set output '$PLOT'
# compute stddev sum
ss(x,y) = sqrt((x**2 + y**2) / 2)
# plot lines
plot \\" > gnuplotscript

i=0
for ver in $VERS; do
	# add plot line for version
	echo "'$DATA' i $i using (\$6+${i}*1000):(\$17+\$19):(ss(\$18,\$20)) \\" >> gnuplotscript
	echo "title '$ver update+sort time' with errorbars, \\" >> gnuplotscript
	((i++))
done

# make plot
gnuplot gnuplotscript

# delete plot script
rm gnuplotscript
