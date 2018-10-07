image_tag = ridhoq/nuke

ifeq ($(OS),Windows_NT)
	make_os := Windows
else
	make_os := $(shell uname -s)
endif

ifeq ($(make_os), Windows)
	pwd = $(shell echo %cd%)
else
	pwd = $(shell pwd)
endif

dr = docker run -it --rm -v ${pwd}:/nuke -e CARGO_HOME=/nuke/.cargo ${image_tag}

.PHONY: docker_build docker_bash docker_test

docker_build:
	docker build -t ${image_tag} .

docker_bash:
	${dr} /bin/bash

docker_test:
	${dr} cargo test --release