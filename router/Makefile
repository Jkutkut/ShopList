include ../.utils/colors

MAKE=make --no-print-directory

TARGET_DIR=.
DOCKER_CONTAINER_NAME=shoplist-nginx

NETWORK=shoplist

update:
	@echo "${GREEN}Updating${NC} reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}"
	docker cp $(TARGET_DIR)/nginx.conf $(DOCKER_CONTAINER_NAME):/etc/nginx/
	docker exec -it $(DOCKER_CONTAINER_NAME) nginx -t
	docker exec -it $(DOCKER_CONTAINER_NAME) nginx -s reload

connect:
	@echo "${GREEN}Connecting${NC} to reverse proxy: ${TITLE}${DOCKER_CONTAINER_NAME}${NC}"
	@docker exec -it $(DOCKER_CONTAINER_NAME) /bin/sh

.PHONY: update connect
