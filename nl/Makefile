CC = gcc
AR = ar

SRC = libnl.c
OBJ = libnl.o
LIB = libnl.a

CFLAGS = -Wall -Wextra -O2

.PHONY: all clean

all: $(LIB)

$(OBJ): $(SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(LIB): $(OBJ)
	$(AR) rcs $@ $<
	$(MAKE) clean

clean:
	rm -f $(OBJ)
