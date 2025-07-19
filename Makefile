include ./.utils/colors

MAKE=make --no-print-directory

DEV_DOCKER_CONFIG="--rm"
PRD_DOCKER_CONFIG="--restart=unless-stopped"

# ########   Router   ########
run:
	@echo "${GREEN}Running${NC}"
	@DOCKER_CONFIG="${PRD_DOCKER_CONFIG}" ${MAKE} -C router run

remove:
	@${MAKE} -C router remove

stop:
	@${MAKE} -C router stop

# ########   Docker Compose   ########
PROJECTS = api auth db

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
	docker-compose down
	@echo " - ${TITLE}Shoplist${NC}: ${LGREEN}OK${NC}"

.PHONY: all re build clean fclean run remove stop
