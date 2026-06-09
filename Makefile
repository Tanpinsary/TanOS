DOCKER_TAG ?= tanos:latest
.PHONY: help run build kernel user docker build_docker fmt

help:
	@echo "TanOS make targets:"
	@echo "  make run          Build and run TanOS in QEMU"
	@echo "  make build        Build kernel and user filesystem image"
	@echo "  make kernel       Build the TanOS kernel only"
	@echo "  make user         Build user programs only"
	@echo "  make docker       Enter the TanOS Docker environment"
	@echo "  make build_docker Build the TanOS Docker image"
	@echo "  make fmt          Format Rust code"

run:
	$(MAKE) -C os run

build:
	$(MAKE) -C os build

kernel:
	$(MAKE) -C os kernel

user:
	$(MAKE) -C user build

docker:
	docker run --rm -it -v ${PWD}:/mnt -w /mnt --name tanos ${DOCKER_TAG} bash

build_docker: 
	docker build -t ${DOCKER_TAG} --target build .

fmt:
	cargo fmt --manifest-path easy-fs/Cargo.toml
	cargo fmt --manifest-path easy-fs-fuse/Cargo.toml
	cargo fmt --manifest-path os/Cargo.toml
	cargo fmt --manifest-path user/Cargo.toml
