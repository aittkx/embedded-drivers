SHELL := /bin/bash
.DEFAULT_GOAL := help

###########################
# VARIABLES
###########################

.PHONY: help
help:  ## help target to show available commands with information
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk '{split($$0,a,":.*?## "); printf " %-15s %s\n", a[1], a[2]}'

.PHONY: release
release:  ## release custom crate to crates.io (带--execute参数)
	cargo-release release --registry crates-io --workspace --package $(package) $(args)

.PHONY: patch
patch:  ## patch custom crate to crates.io
	cargo-release release patch --registry crates-io --workspace --package $(package) $(args)
