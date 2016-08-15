INDATA="target/data/chance.data"
OUTPLOT="target/data/chance.svg"

#clear old plot
rm -f $OUTPLOT

# start plot script
echo "# skip header
set key autotitle columnhead
# select svg file format
set terminal svg
# select output file
set output '$OUTPLOT'
plot \\" > gnuplotscript

for ver in vec presort presort_pad; do
	# add plot line for version
	echo "'${INDATA}.$ver' using 10:(\$14+\$15) title '$ver update+sort time', \\" >> gnuplotscript
done

# make plot
gnuplot gnuplotscript

# delete plot script
rm gnuplotscript
