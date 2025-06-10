include ../.utils/colors

MAKE=make --no-print-directory

TARGET_DIR=.
DOCKER_CONTAINER_NAME=nginx-shoplist

NETWORK=shoplist

all: run

run: ensure_network_exits
	@docker inspect $(DOCKER_CONTAINER_NAME) >/dev/null 2>&1 || ( \
		echo "${GREEN}Creating${NC} reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}" && \
		docker run -d \
			--name $(DOCKER_CONTAINER_NAME) \
			-p 80:80 \
			--network $(NETWORK) \
			$(DOCKER_CONFIG) \
			nginx:1.25.0-alpine-slim && \
		${MAKE} update) && \
		${MAKE} start

ensure_network_exits:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || \
		docker network create $(NETWORK)

update:
	docker cp $(TARGET_DIR)/nginx.conf $(DOCKER_CONTAINER_NAME):/etc/nginx/
	docker exec -it $(DOCKER_CONTAINER_NAME) nginx -t
	docker exec -it $(DOCKER_CONTAINER_NAME) nginx -s reload

start:
	@echo "${GREEN}Starting${NC} reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}"
	@docker start $(DOCKER_CONTAINER_NAME) >/dev/null
	@echo "http://localhost"

stop:
	@echo -n "${LRED}Stopping${NC} reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}"
	@docker stop $(DOCKER_CONTAINER_NAME) >/dev/null || true
	@echo " ${LGREEN}OK${NC}"

remove:
	@echo -n "${LRED}Removing${NC} reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}"
	@docker rm -f $(DOCKER_CONTAINER_NAME) >/dev/null || true
	@echo " ${LGREEN}OK${NC}"

connect:
	@echo "${GREEN}Connecting${NC} to reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}"
	@docker exec -it $(DOCKER_CONTAINER_NAME) /bin/sh

.PHONY: all run update start stop remove connect ensure_network_exits
