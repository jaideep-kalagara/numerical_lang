
CC = gcc
AR = ar

SRC = libnl.c
OBJ = libnl.o
LIB = libnl.a

CFLAGS = -ffreestanding -fno-builtin -fno-stack-protector \
         -nostdlib -nostdinc -nodefaultlibs \
         -fno-pic -fno-pie \
         -Wall -Wextra -O2

.PHONY: all clean

all: $(LIB)

$(OBJ): $(SRC)
	$(CC) $(CFLAGS) -c $< -o $@

$(LIB): $(OBJ)
	$(AR) rcs $@ $<
	$(MAKE) clean

clean:
	rm -f $(OBJ)
