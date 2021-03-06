# name of experiment
EXPR="data_batches"

# files created/overwritten
DATA="../target/data/${EXPR}.data"
PLOT="../target/data/${EXPR}.pdf"

# benchmark program and fixed parameters
BENCH="cargo run --release --example presort_bench --"
ARGS="--tag ${EXPR} -t 50 -s 0 -c 0.1"
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
		for edits in 1 3 10 33 100 333 1000 3333 10000; do
			$BENCH $ARGS --$ver -e $edits -o $DATA
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
# this a log plot
set logscale x
# plot lines
plot \\" > gnuplotscript

i=0
for ver in $VERS; do
	# add plot line for version
	echo "'$DATA' i $i using (\$7*(1+${i}*0.1)):(\$17+\$19):(ss(\$18,\$20)) \\" >> gnuplotscript
	echo "title '$ver update+sort time' with errorbars, \\" >> gnuplotscript
	((i++))
done

# make plot
gnuplot gnuplotscript

# delete plot script
rm gnuplotscript
