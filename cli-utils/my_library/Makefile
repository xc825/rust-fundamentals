SHELL := /bin/bash
.PHONY: all build clean help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build:
	cargo build

clean:
	cargo clean

test:
	cargo test

all: build test ## Build and test the library
