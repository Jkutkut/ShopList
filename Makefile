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
		-v db_controller_data:/var/lib/pgadmin \
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
ENVS = dev
GENERIC_ENV = dev
PROJECTS = api auth db front nginx
TEST_PROJECTS = auth

$(ENVS:%=up_%): up_%:
	@echo "${TITLE}Starting ENV ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.$*.yaml up --build

$(ENVS:%=up_d_%): up_d_%:
	@echo "${TITLE}Starting ENV ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.$*.yaml up -d --build

$(ENVS:%=down_%): down_%:
	@echo "${TITLE}Shutting down ENV ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.$*.yaml down

$(ENVS:%=logs_%): logs_%:
	@echo "${TITLE}Monitoring logs in ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.$*.yaml logs -f

$(PROJECTS:%=logs_%): logs_%:
	@echo "${TITLE}Monitoring logs in ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.${GENERIC_ENV}.yaml logs --no-log-prefix -f $*

$(PROJECTS:%=terminal_%): terminal_%:
	@echo "${TITLE}Running terminal in ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.${GENERIC_ENV}.yaml exec $* sh

# $(PROJECTS:%=build_%):
# $(PROJECTS:%=doc_%):

$(TEST_PROJECTS:%=test_%): test_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml up $*-test

$(TEST_PROJECTS:%=test_d_%): test_d_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml up -d $*-test

$(TEST_PROJECTS:%=test_logs_%): test_logs_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml logs --no-log-prefix -f $*-test

down_test:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml down

$(PROJECTS:%=clean_%): clean_%:
	@echo "${TITLE}Shutting down ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.${GENERIC_ENV}.yaml rm -s -v $*
	@echo " - ${TITLE}${YELLOW}$*${NC} down and removed${NC}: ${LGREEN}OK${NC}"

fclean:
	@echo "${TITLE}Cleaning Shoplist...${NC}"
	docker-compose -f docker-compose.yaml -f docker-compose.${GENERIC_ENV}.yaml down -t 2
	@echo " - ${TITLE}Shoplist${NC}: ${LGREEN}OK${NC}"

.PHONY: all re build clean fclean run remove stop
