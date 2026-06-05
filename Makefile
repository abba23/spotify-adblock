NAME = spotify-adblock
PREFIX = /usr/local
PROFILE ?= release
BINARY_SOURCE = target/$(PROFILE)/libspotifyadblock.so
BINARY_TARGET = $(DESTDIR)$(PREFIX)/lib/$(NAME).so
CONFIG_SOURCE = config.toml
CONFIG_TARGET = $(DESTDIR)/etc/$(NAME)/config.toml

.PHONY: all
all: $(BINARY_SOURCE)

$(BINARY_SOURCE): src Cargo.toml
	cargo build --profile $(PROFILE)

.PHONY: clean
clean:
	rm -rf target

.PHONY: install
install: $(BINARY_SOURCE) $(CONFIG_SOURCE)
	install -D --mode=644 --strip $(BINARY_SOURCE) $(BINARY_TARGET)
	install -D --mode=644 $(CONFIG_SOURCE) $(CONFIG_TARGET)

.PHONY: uninstall
uninstall:
	rm -f $(BINARY_TARGET)
	rm -f $(CONFIG_TARGET)
