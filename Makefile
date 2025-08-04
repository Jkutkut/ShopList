include ./.utils/colors
include .env

MAKE=make --no-print-directory

DEV_DOCKER_CONFIG="--rm"
PRD_DOCKER_CONFIG="--restart=unless-stopped"

# ########   DB Controler   #########

DB_CONTROLER_NAME=db_controller

run_db_controler:
	docker ps | grep ${DB_CONTROLER_NAME} > /dev/null || \
	( \
		docker ps -a | grep ${DB_CONTROLER_NAME} > /dev/null && \
		docker start ${DB_CONTROLER_NAME} \
	) || \
	docker run -d \
		--name ${DB_CONTROLER_NAME} \
		-p ${DB_CONTROLER_PORT}:80 \
		-e PGADMIN_DEFAULT_EMAIL="${DB_CONTROLER_EMAIL}" \
		-e PGADMIN_DEFAULT_PASSWORD="${DB_CONTROLER_PASSWORD}" \
		--network "shoplist_db-network" \
		dpage/pgadmin4
	open http://localhost:${DB_CONTROLER_PORT}

delete_db_controler:
	docker rm -f ${DB_CONTROLER_NAME}

run_valkey_controller:
	docker run -it --rm \
		--name shoplist-valkey-controller \
		--network shoplist_db-network \
		shoplist-valkey \
		valkey-cli -h shoplist-valkey

# ########   Docker Compose   ########
PROJECTS = api auth db nginx

logs:
	@echo "${TITLE}Monitoring ${YELLOW}all ${TITLE}logs...${NC}"
	docker-compose logs -f

$(PROJECTS:%=logs_%): logs_%:
	@echo "${TITLE}Monitoring logs in ${YELLOW}$*${NC}..."
	docker-compose logs -f $*

$(PROJECTS:%=terminal_%): terminal_%:
	@echo "${TITLE}Running terminal in ${YELLOW}$*${NC}..."
	docker-compose exec $* sh

# $(PROJECTS:%=build_%):
# $(PROJECTS:%=run_%):
# $(PROJECTS:%=test_%):
# $(PROJECTS:%=doc_%):

$(PROJECTS:%=clean_%): clean_%:
	@echo "${TITLE}Shutting down ${YELLOW}$*${NC}..."
	docker-compose rm -s -v $*
	@echo " - ${TITLE}${YELLOW}$*${NC} down and removed${NC}: ${LGREEN}OK${NC}"

fclean:
	@echo "${TITLE}Cleaning Shoplist...${NC}"
	docker-compose down -t 2
	@echo " - ${TITLE}Shoplist${NC}: ${LGREEN}OK${NC}"

.PHONY: all re build clean fclean run remove stop
