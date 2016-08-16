INDATA="target/data/chance.data"
OUTPLOT="target/data/chance.pdf"

#clear old plot
rm -f $OUTPLOT

# start plot script
echo "# skip header
set key autotitle columnhead
# select pdf file format
set terminal pdf
# select output file
set output '$OUTPLOT'
# compute stddev sum
ss(x,y) = sqrt((x**2 + y**2) / 2)
# plot lines
plot \\" > gnuplotscript

i=0
for ver in vec presort presort_pad; do
	# add plot line for version
	echo "'$INDATA' i $i using (\$10+${i}*0.01):(\$17+\$19):(ss(\$18,\$20)) \\" >> gnuplotscript
	echo "title '$ver update+sort time' with errorbars, \\" >> gnuplotscript
	((i++))
done

# make plot
gnuplot gnuplotscript

# delete plot script
rm gnuplotscript
