include ./.utils/colors

MAKE=make --no-print-directory

DEV_DOCKER_CONFIG="--rm"
PRD_DOCKER_CONFIG="--restart=unless-stopped"

all: build
re: clean build

run:
	@echo "${GREEN}Running${NC}"
	@DOCKER_CONFIG="${PRD_DOCKER_CONFIG}" ${MAKE} -C router run

remove:
	@${MAKE} -C router remove

stop:
	@${MAKE} -C router stop

build:
	echo "TODO: build"

clean:
	echo "TODO: clean"

fclean: clean remove

.PHONY: all re build clean fclean run remove stop
