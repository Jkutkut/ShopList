# ****** Operating System ******
OS = $(shell uname -s)
ifeq ($(OS),Linux)
	DIR = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	DIR = ${PWD}
endif
REPO = shoplist
STATE_DIR = .state

# ****** Rust Constants ******
CARGO = /root/.cargo/bin/cargo
CODE_VOLUME = -v ${DIR}/../:/${REPO}
CARGO_REGISTRY = -v cargo_registy:/root/.cargo/registry

# ****** Docker Constants ******
DOCKER_RUN = docker run --rm -t
DOCKER_RUN_IT = ${DOCKER_RUN} -it --name ${REPO}

RUN_ATTRS = ${CODE_VOLUME} ${CARGO_REGISTRY} -w /${REPO}/backend --network host

# ****** Docker Images ******
BUILD_IMAGE = jkutkut/${REPO}_builder:latest
BUILD_IMAGE_STATE = ${STATE_DIR}/build_image

${STATE_DIR}:
	mkdir -p $@

${BUILD_IMAGE_STATE}: Dockerfile ${STATE_DIR}
	docker build --target dev -t ${BUILD_IMAGE} -f $< .
	@touch $@

build_image: ${BUILD_IMAGE_STATE}

# ****** Commands ******

include .env

terminal: build_image
	${DOCKER_RUN_IT} ${RUN_ATTRS} ${BUILD_IMAGE}

reset_file_permissions:
	sudo chown -R ${USER}:${USER} .

build: build_image
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo ${BUILD_IMAGE} build

run_auth: build_image
	${DOCKER_RUN} ${RUN_ATTRS} -p 50051:50051 --entrypoint cargo ${BUILD_IMAGE} run -p auth

run_api: build_image
	${DOCKER_RUN} ${RUN_ATTRS} -p 80:80 \
		--env ROCKET_CONFIG=api/Rocket.toml \
		--env ROCKET_SECRET_KEY=$(ROCKET_SECRET_KEY) \
		--entrypoint cargo ${BUILD_IMAGE} run -p api

clean:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust clean
