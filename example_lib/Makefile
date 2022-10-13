CC=gcc
CFLAGS=-I.
DEPS = library.c
OBJ = library.o

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

library: $(OBJ)
	$(CC) -shared -o lib_worker.so $^ $(CFLAGS)

clean: 
	rm -rf *.o *.so