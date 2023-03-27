# SPDX-License-Identifier: GPL-3.0-or-later
NAME = spotify-adblock
PREFIX = /usr/local
PROFILE ?= release
BINARY_PATH = target/$(PROFILE)/libspotifyadblock.so
CONFIG_PATH = config.toml
BINARY_TARGET = $(DESTDIR)$(PREFIX)/lib/$(NAME).so
CONFIG_TARGET = $(DESTDIR)/etc/$(NAME)/config.toml

.PHONY: all
all: $(BINARY_PATH)

$(BINARY_PATH): src Cargo.toml
	# cargo build --profile $(PROFILE)
ifeq ($(PROFILE), release)
	cargo build --release
else
	cargo build
endif
	strip $(BINARY_PATH)

.PHONY: clean
clean:
	rm -rf target Cargo.lock

.PHONY: install
install: $(BINARY_PATH) $(CONFIG_PATH)
	install -D --mode=644 --strip $(BINARY_PATH) $(BINARY_TARGET) 
	install -D --mode=644 $(CONFIG_PATH) $(CONFIG_TARGET) 

.PHONY: uninstall
uninstall:
	rm -f $(BINARY_TARGET)
	rm -f $(CONFIG_TARGET)
