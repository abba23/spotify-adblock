NAME = spotify-adblock
PREFIX = /usr/local
PROFILE ?= release

# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    BINARY_EXT = dylib
else
    BINARY_EXT = so
endif

BINARY_NAME = libspotifyadblock.$(BINARY_EXT)
BINARY_PATH = target/$(PROFILE)/$(BINARY_NAME)
CONFIG_PATH = config.toml
BINARY_TARGET = $(DESTDIR)$(PREFIX)/lib/$(BINARY_NAME)
CONFIG_TARGET = $(DESTDIR)/etc/$(NAME)/config.toml

.PHONY: all
all: $(BINARY_PATH)

$(BINARY_PATH): src Cargo.toml
ifeq ($(PROFILE), release)
	cargo build --release
else
	cargo build
endif

.PHONY: clean
clean:
	rm -rf target

.PHONY: install
install: $(BINARY_PATH) $(CONFIG_PATH)
	mkdir -p $(dir $(BINARY_TARGET))
	install -m 644 $(BINARY_PATH) $(BINARY_TARGET)
ifeq ($(UNAME_S),Linux)
	strip $(BINARY_TARGET)
endif
	mkdir -p $(dir $(CONFIG_TARGET))
	install -m 644 $(CONFIG_PATH) $(CONFIG_TARGET)

.PHONY: uninstall
uninstall:
	rm -f $(BINARY_TARGET)
	rm -f $(CONFIG_TARGET)
