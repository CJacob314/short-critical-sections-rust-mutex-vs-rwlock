MUTEX_DATA := mutex.dat
RWLOCK_DATA := rwlock.dat
SPINLOCK_DATA := spinlock.dat
GRAPH_PNG := read_ops_per_cycle.png
.DEFAULT_GOAL = $(GRAPH_PNG)

.PHONY: $(MUTEX_DATA) $(RWLOCK_DATA) clean

$(MUTEX_DATA):
	cargo r -r > $@

$(RWLOCK_DATA):
	cargo r -r -F rwlock > $@

$(SPINLOCK_DATA):
	cargo r -r -F spinlock > $@

$(GRAPH_PNG): $(MUTEX_DATA) $(RWLOCK_DATA) $(SPINLOCK_DATA)
	gnuplot plot.gp

clean:
	$(RM) $(MUTEX_DATA) $(RWLOCK_DATA) $(SPINLOCK_DATA) $(GRAPH_PNG)
