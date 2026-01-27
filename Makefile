include ./.utils/colors
include .env

MAKE=make --no-print-directory

DEV_DOCKER_CONFIG="--rm"
PRD_DOCKER_CONFIG="--restart=unless-stopped"

default: usage

# ########   DB Controler   #########

DB_CONTROLER_NAME=db_controller

run_db_controler:
	@echo "${TITLE}Starting DB Controler: ${YELLOW}${DB_CONTROLER_NAME}${TITLE}...${NC}"
	@docker ps | grep ${DB_CONTROLER_NAME} > /dev/null || \
	( \
		docker ps -a | grep ${DB_CONTROLER_NAME} > /dev/null && \
		docker start ${DB_CONTROLER_NAME} \
	) || \
	docker run -d --rm \
		--name ${DB_CONTROLER_NAME} \
		-p ${DB_CONTROLER_PORT}:80 \
		-e PGADMIN_DEFAULT_EMAIL="${DB_CONTROLER_EMAIL}" \
		-e PGADMIN_DEFAULT_PASSWORD="${DB_CONTROLER_PASSWORD}" \
		-v db_controller_data:/var/lib/pgadmin \
		dpage/pgadmin4
	@make -s connect_db_controler
	@echo "${LGREEN}http://localhost:${DB_CONTROLER_PORT}${NC}"
	@open http://localhost:${DB_CONTROLER_PORT}

connect_db_controler:
	@echo "${TITLE}Connecting DB Controler: ${YELLOW}${DB_CONTROLER_NAME}${TITLE}...${NC}"
	@docker network connect "shoplist_db-network" ${DB_CONTROLER_NAME} || true;
	@docker network connect "shoplist_db-network-test" ${DB_CONTROLER_NAME} || true;

delete_db_controler:
	@echo "${TITLE}Deleting DB Controler: ${YELLOW}${DB_CONTROLER_NAME}${TITLE}...${NC}"
	@docker rm -f ${DB_CONTROLER_NAME}

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
TEST_PROJECTS = auth api

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

up_test:
	@echo "${TITLE}Starting ${YELLOW}test${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml up

up_d_test:
	@echo "${TITLE}Starting ${YELLOW}test${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml up -d

down_test:
	@echo "${TITLE}Shutting down ${YELLOW}test${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml down

logs_test:
	@echo "${TITLE}Monitoring logs in ${YELLOW}test${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml logs

$(TEST_PROJECTS:%=test_%): test_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml up $*-test

$(TEST_PROJECTS:%=test_d_%): test_d_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml up -d $*-test

$(TEST_PROJECTS:%=test_logs_%): test_logs_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml logs --no-log-prefix -f $*-test

$(TEST_PROJECTS:%=test_terminal_%): test_terminal_%:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml -f docker-compose.test.yaml exec $*-test sh

$(PROJECTS:%=clean_%): clean_%:
	@echo "${TITLE}Shutting down ${YELLOW}$*${NC}..."
	docker compose -f docker-compose.yaml -f docker-compose.${GENERIC_ENV}.yaml rm -s -v $*
	@echo " - ${TITLE}${YELLOW}$*${NC} down and removed${NC}: ${LGREEN}OK${NC}"

fclean:
	@echo "${TITLE}Cleaning Shoplist...${NC}"
	docker-compose -f docker-compose.yaml -f docker-compose.${GENERIC_ENV}.yaml down -t 2
	@echo " - ${TITLE}Shoplist${NC}: ${LGREEN}OK${NC}"

.PHONY: all re build clean fclean run remove stop

usage:
	@echo "${TITLE}Usage:${NC}"
	@echo "make usage;"
	@echo "";
	@echo "${TITLE}Start environment - up environment:${NC}"
	@for env in $(ENVS); do \
		echo "make up_$$env; ${GREEN}# $$env${NC}"; \
		echo "make up_d_$$env; ${GREEN}# $$env detached${NC}"; \
	done
	@echo ""
	@echo "${TITLE}Stop environment - down environment:${NC}"
	@for env in $(ENVS); do \
		echo "make down_$$env; ${GREEN}# $$env${NC}"; \
	done
	@echo "make down_test; ${GREEN}# all tests${NC}"
	@echo ""
	@echo "${TITLE}Monitoring logs in environment:${NC}"
	@for env in $(ENVS); do \
		echo "make logs_$$env; ${GREEN}# $$env${NC}"; \
	done
	@echo ""
	@echo "${TITLE}Monitoring logs of service:${NC}"
	@for project in $(PROJECTS); do \
		echo "make logs_$$project; ${GREEN}# $$project${NC}"; \
	done
	@echo ""
	@echo "${TITLE}Open a terminal in service:${NC}"
	@for project in $(PROJECTS); do \
		echo "make terminal_$$project; ${GREEN}# $$project${NC}"; \
	done
	@for test in $(TEST_PROJECTS); do \
		echo "make test_terminal_$$test; ${GREEN}# test $$test${NC}"; \
	done
	@echo ""
	@# TODO build
	@# TODO doc
	@echo "${TITLE}Test service:${NC}"
	@for project in $(TEST_PROJECTS); do \
		echo "make test_$$project; ${GREEN}# $$project${NC}"; \
		echo "make test_d_$$project; ${GREEN}# $$project detached${NC}"; \
	done
	@echo ""
	@echo "${TITLE}Monitoring logs of test:${NC}"
	@for project in $(TEST_PROJECTS); do \
		echo "make test_logs_$$project; ${GREEN}# $$project${NC}"; \
	done
	@echo ""
	@echo "${TITLE}Cleaning base services:${NC} (based on ${YELLOW}${GENERIC_ENV}${NC})"
	@for project in $(PROJECTS); do \
		echo "make clean_$$project; ${GREEN}# $$project${NC}"; \
	done
	@echo "make fclean; ${GREEN}# all${NC}"
	@echo ""
	@echo "${TITLE}DB Controler:${NC}"
	@echo "make run_db_controler; ${GREEN}# run db controller (pgadmin)${NC}"
	@echo "make connect_db_controler; ${GREEN}# connect to docker networks${NC}"
	@echo "make delete_db_controler; ${GREEN}# delete db controller${NC}"
	@echo "make run_valkey_controller; ${GREEN}# Open a cli to work with valkey${NC}"
