NAME = spotify-adblock
PREFIX = /usr/local
PROFILE ?= release
BINARY_PATH = target/$(PROFILE)/libspotifyadblock.so
CONFIG_PATH = config.toml
BINARY_TARGET = $(DESTDIR)$(PREFIX)/lib/$(NAME).so
CONFIG_TARGET = $(DESTDIR)/etc/$(NAME)/config.toml

# Is the prerequisite "cargo" installed?
PREREQ := $(shell command -v cargo 2> /dev/null)

.PHONY: all
all: $(BINARY_PATH)

$(BINARY_PATH): src Cargo.toml
	# cargo build --profile $(PROFILE)
ifndef PREREQ
	# Give some useful error message
	$(error "It appears Rust and/or Cargo is not available please install it")
endif
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
	install -D --mode=644 --strip $(BINARY_PATH) $(BINARY_TARGET) 
	install -D --mode=644 $(CONFIG_PATH) $(CONFIG_TARGET) 

.PHONY: uninstall
uninstall:
	rm -f $(BINARY_TARGET)
	rm -f $(CONFIG_TARGET)
