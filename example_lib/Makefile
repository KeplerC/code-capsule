CC=gcc
CFLAGS=-I.
DEPS = worker_impl_1.c worker_impl_2.c
OBJ = library.o

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

all: lib1 lib2 

lib1: worker_impl_1.o
	$(CC) -shared -o lib_worker_1.so $^ $(CFLAGS)

lib2: worker_impl_2.o
	$(CC) -shared -o lib_worker_2.so $^ $(CFLAGS)

clean: 
	rm -rf *.o *.so