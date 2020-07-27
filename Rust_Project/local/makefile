SRC = $(wildcard *.c)
OBJ = $(SRC:.c=.o)

CC = clang
CFLAGS = -O3

client: $(OBJ)	
	$(CC) $(CFLAGS) -o $@ $^

.PHONY: clean

clean:
	rm client $(OBJ)