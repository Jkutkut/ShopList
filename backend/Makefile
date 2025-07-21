include ../.utils/colors

MAKE=make --no-print-directory

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

RUN_ATTRS = ${CODE_VOLUME} ${CARGO_REGISTRY} -w /${REPO}/backend

# ****** Docker Images ******
BUILD_IMAGE = jkutkut/${REPO}_builder:latest
BUILD_IMAGE_STATE = ${STATE_DIR}/build_image

${STATE_DIR}:
	mkdir -p $@

${BUILD_IMAGE_STATE}: Dockerfile ${STATE_DIR}
	@echo "${TITLE}Building ${YELLOW}${BUILD_IMAGE}${NC}"
	docker build --target builder -t ${BUILD_IMAGE} -f $< .
	@touch $@

build_image: ${BUILD_IMAGE_STATE}

# ****** Actions ******
reset_file_permissions:
	@echo "${TITLE}Resetting file permissions${NC}"
	sudo chown -R ${USER}:${USER} .

terminal: build_image
	@echo "${TITLE}Opening ${YELLOW}${REPO}${NC} terminal${NC}"
	${DOCKER_RUN_IT} ${RUN_ATTRS} ${BUILD_IMAGE}
	${MAKE} reset_file_permissions
