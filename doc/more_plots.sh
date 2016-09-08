# name of experiments
EXPRSS="additions removals shape_edit data_incr data_change data_edit"
EXPRSB="data_batches"
VERS="vec presort presort_pad permute permute_pad merge merge_pad"

for exp in $EXPRSS; do
	# files created/overwritten
	DATA="../target/data/${exp}.data"
	PLOTU="../target/data/${exp}-U.pdf"
	PLOTR="../target/data/${exp}-R.pdf"

	# Make plots
	# ----------

	#clear old plot
	rm -f $PLOTU
	rm -f $PLOTR

	# start plot script
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptu
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptr

	echo "set output '$PLOTU'" >> gnuplotscriptu
	echo "set output '$PLOTR'" >> gnuplotscriptr

	echo "plot \\" >> gnuplotscriptu
	echo "plot \\" >> gnuplotscriptr

	i=0
	for ver in $VERS; do
		# add plot line for version
		echo "'$DATA' i $i using (\$6+${i}*1000):(\$17):(\$18) \\" >> gnuplotscriptu
		echo "title '$ver update time' with errorbars, \\" >> gnuplotscriptu
		echo "'$DATA' i $i using (\$6+${i}*1000):(\$19):(\$20) \\" >> gnuplotscriptr
		echo "title '$ver resort time' with errorbars, \\" >> gnuplotscriptr
		((i++))
	done

	# make plots
	gnuplot gnuplotscriptu
	gnuplot gnuplotscriptr

	# delete plot scripts
	rm gnuplotscriptu
	rm gnuplotscriptr

done


for exp in $EXPRSB; do
	# files created/overwritten
	DATA="../target/data/${exp}.data"
	PLOTU="../target/data/${exp}-U.pdf"
	PLOTR="../target/data/${exp}-R.pdf"

	# Make plots
	# ----------

	#clear old plot
	rm -f $PLOTU
	rm -f $PLOTR

	# start plot script
	echo "set key autotitle columnhead
	set logscale x
	set terminal pdf" > gnuplotscriptu
	echo "set key autotitle columnhead
	set logscale x
	set terminal pdf" > gnuplotscriptr

	echo "set output '$PLOTU'" >> gnuplotscriptu
	echo "set output '$PLOTR'" >> gnuplotscriptr

	echo "plot \\" >> gnuplotscriptu
	echo "plot \\" >> gnuplotscriptr

	i=0
	for ver in $VERS; do
		# add plot line for version
		echo "'$DATA' i $i using (\$7*(1+${i}*0.1)):(\$17):(\$18) \\" >> gnuplotscriptu
		echo "title '$ver update time' with errorbars, \\" >> gnuplotscriptu
		echo "'$DATA' i $i using (\$7*(1+${i}*0.1)):(\$19):(\$20) \\" >> gnuplotscriptr
		echo "title '$ver resort time' with errorbars, \\" >> gnuplotscriptr
		((i++))
	done

	# make plots
	gnuplot gnuplotscriptu
	gnuplot gnuplotscriptr

	# delete plot scripts
	rm gnuplotscriptu
	rm gnuplotscriptr

done

exp="resort_chance"
col="10"
	# files created/overwritten
	DATA="../target/data/${exp}.data"
	PLOTU="../target/data/${exp}-U.pdf"
	PLOTR="../target/data/${exp}-R.pdf"

	# Make plots
	# ----------

	#clear old plot
	rm -f $PLOTU
	rm -f $PLOTR

	# start plot script
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptu
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptr

	echo "set output '$PLOTU'" >> gnuplotscriptu
	echo "set output '$PLOTR'" >> gnuplotscriptr

	echo "plot \\" >> gnuplotscriptu
	echo "plot \\" >> gnuplotscriptr

	i=0
	for ver in $VERS; do
		# add plot line for version
		echo "'$DATA' i $i using (\$$col+${i}*0.01):(\$17):(\$18) \\" >> gnuplotscriptu
		echo "title '$ver update time' with errorbars, \\" >> gnuplotscriptu
		echo "'$DATA' i $i using (\$$col+${i}*0.01):(\$19):(\$20) \\" >> gnuplotscriptr
		echo "title '$ver resort time' with errorbars, \\" >> gnuplotscriptr
		((i++))
	done

	# make plots
	gnuplot gnuplotscriptu
	gnuplot gnuplotscriptr

	# delete plot scripts
	rm gnuplotscriptu
	rm gnuplotscriptr


exp="shape_chance"
col="8"
	# files created/overwritten
	DATA="../target/data/${exp}.data"
	PLOTU="../target/data/${exp}-U.pdf"
	PLOTR="../target/data/${exp}-R.pdf"

	# Make plots
	# ----------

	#clear old plot
	rm -f $PLOTU
	rm -f $PLOTR

	# start plot script
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptu
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptr

	echo "set output '$PLOTU'" >> gnuplotscriptu
	echo "set output '$PLOTR'" >> gnuplotscriptr

	echo "plot \\" >> gnuplotscriptu
	echo "plot \\" >> gnuplotscriptr

	i=0
	for ver in $VERS; do
		# add plot line for version
		echo "'$DATA' i $i using (\$$col+${i}*0.01):(\$17):(\$18) \\" >> gnuplotscriptu
		echo "title '$ver update time' with errorbars, \\" >> gnuplotscriptu
		echo "'$DATA' i $i using (\$$col+${i}*0.01):(\$19):(\$20) \\" >> gnuplotscriptr
		echo "title '$ver resort time' with errorbars, \\" >> gnuplotscriptr
		((i++))
	done

	# make plots
	gnuplot gnuplotscriptu
	gnuplot gnuplotscriptr

	# delete plot scripts
	rm gnuplotscriptu
	rm gnuplotscriptr

# shared
# ------

	# files created/overwritten
	DATA="../target/data/removals.data"
	PLOTU="../target/data/init_dump.pdf"
	PLOTR="../target/data/init_sort.pdf"

	# Make plots
	# ----------

	#clear old plot
	rm -f $PLOTU
	rm -f $PLOTR

	# start plot script
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptu
	echo "set key autotitle columnhead
	set terminal pdf" > gnuplotscriptr

	echo "set output '$PLOTU'" >> gnuplotscriptu
	echo "set output '$PLOTR'" >> gnuplotscriptr

	echo "plot \\" >> gnuplotscriptu
	echo "plot \\" >> gnuplotscriptr

	i=0
	for ver in $VERS; do
		# add plot line for version
		echo "'$DATA' i $i using (\$6+${i}*1000):(\$11):(\$12) \\" >> gnuplotscriptu
		echo "title '$ver initial dump time' with errorbars, \\" >> gnuplotscriptu
		echo "'$DATA' i $i using (\$6+${i}*1000):(\$13):(\$14) \\" >> gnuplotscriptr
		echo "title '$ver initial sort time' with errorbars, \\" >> gnuplotscriptr
		((i++))
	done

	# make plots
	gnuplot gnuplotscriptu
	gnuplot gnuplotscriptr

	# delete plot scripts
	rm gnuplotscriptu
	rm gnuplotscriptr


