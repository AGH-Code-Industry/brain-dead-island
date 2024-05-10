# The shell to run the makefile with must be defined to work properly in Linux systems
SHELL := /bin/bash

# all the recipes are phony (no files to check).
.PHONY: docs tests build dev run

.DEFAULT_GOAL := run

# Output descriptions of all commands
help:
	@echo "Please use 'make <target>', where <target> is one of"
	@echo ""
	@echo "  help                             outputs this helper"
	@echo "  install-deps                     install dependencies"
	@echo "  clippy                           run cargo clippy"
	@echo "  build                            builds the project"
	@echo "  test                             run all the tests, locally"
	@echo "  fmt-test                         run rust fmt test"
	@echo "  coverage                         run code coverage"
	@echo "  pipeline                         runs all targets defined in pipeline"
	@echo "  all                         	  runs all targets"
	@echo ""
	@echo ""
	@echo "Check the Makefile to know exactly what each target is doing."

#Install dependencies
install-deps:
	sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev libsdl2-gfx-dev gcc

# Build the project
build:
	@echo "Building the project"
	cargo build

# Run the tests
test:
	@echo "Running tests"
	cargo test --all-features

# Run the fmt tests
fmt-test:
	@echo "Running fmt tests"
	cargo fmt --all -- --check

# Cargo clippy
clippy:
	@echo "Running clippy"
	cargo clippy

# Code coverage
coverage:
	@echo "Running code coverage"
	cargo tarpaulin --ignore-tests

# Run all the targets defined in the pipeline
pipeline: build test fmt-test clippy coverage

# Run all the targets
all: install-deps pipeline