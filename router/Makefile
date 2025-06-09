TARGET_DIR=.
DOCKER_CONTAINER_NAME=nginx-shoplist

NETWORK=shoplist

all: run

run: ensure_network_exits
	docker run -d \
		--name $(DOCKER_CONTAINER_NAME) \
		-p 80:80 \
		--network $(NETWORK) \
		--restart=always \
		nginx:1.25.0-alpine-slim
	make update

ensure_network_exits:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || \
		docker network create $(NETWORK)

update:
	docker cp $(TARGET_DIR)/nginx.conf $(DOCKER_CONTAINER_NAME):/etc/nginx/
	docker exec -it $(DOCKER_CONTAINER_NAME) nginx -t
	docker exec -it $(DOCKER_CONTAINER_NAME) nginx -s reload

stop:
	docker stop $(DOCKER_CONTAINER_NAME) || true

start:
	docker start $(DOCKER_CONTAINER_NAME)

remove:
	docker rm -f $(DOCKER_CONTAINER_NAME)

connect:
	docker exec -it $(DOCKER_CONTAINER_NAME) /bin/sh


.PHONY: run update
