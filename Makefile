NAME = spotify-adblock
PREFIX = /usr/local
PROFILE ?= release
TARGET_DIR ?= target
BINARY_SOURCE = $(TARGET_DIR)/$(PROFILE)/libspotifyadblock.so
BINARY_TARGET = $(DESTDIR)$(PREFIX)/lib/$(NAME).so
CONFIG_SOURCE = config.toml
CONFIG_TARGET = $(DESTDIR)/etc/$(NAME)/config.toml

.PHONY: all
all: $(BINARY_SOURCE)

$(BINARY_SOURCE): $(wildcard src/*.rs) Cargo.toml
	cargo build --profile $(PROFILE) --target-dir $(TARGET_DIR)

.PHONY: clean
clean:
	rm -rf $(TARGET_DIR)

.PHONY: install
install: $(BINARY_SOURCE) $(CONFIG_SOURCE)
	install -D --mode=644 --strip $(BINARY_SOURCE) $(BINARY_TARGET)
	install -D --mode=644 $(CONFIG_SOURCE) $(CONFIG_TARGET)

.PHONY: uninstall
uninstall:
	rm -f $(BINARY_TARGET)
	rm -f $(CONFIG_TARGET)
