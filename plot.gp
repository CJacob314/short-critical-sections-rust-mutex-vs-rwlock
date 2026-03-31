set terminal pngcairo size 1200,800 enhanced font "Arial,14"
set output "read_ops_per_cycle.png"

set title "Read Lock Throughput vs Thread Count"
set xlabel "Number of threads"
set ylabel "Read ops per cycle"
set grid
set key outside right top
set datafile separator whitespace

plot "mutex.dat" using 1:2 with linespoints linewidth 2 pointtype 7 title "Mutex", \
     "rwlock.dat" using 1:2 with linespoints linewidth 2 pointtype 5 title "RwLock"
