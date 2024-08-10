CFLAGS += -O3 -g -flto -Wall -Wextra
CFLAGS +=
LDFLAGS +=

SRCS = $(wildcard *.c)
OBJS = $(SRCS:.c=.o)

BIN = waybar-network-applet

install:
	mkdir -p $(DESTDIR)/usr/bin
	install -m 775 $(BIN) $(DESTDIR)/usr/bin/

%.o: %.c
	$(CC) $(CFLAGS) `pkg-config --cflags gtk4 gtk4-layer-shell-0` $< -c -o $@

$(BIN): $(OBJS)
	$(CC) $(OBJS) `pkg-config --libs gtk4 gtk4-layer-shell-0` -o $@

clean:
	rm -rf *.o $(BIN)
