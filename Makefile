SHELL = /bin/bash
CACHE = no

GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p jokehub

DOCKER_ENV = local
DOCKER_DIR = docker

include .env

define is_need_to_use_cache
    if [ ! $(1) = no ]; then\
		echo --no-cache ;\
    fi
endef

define base_docker_cmd
	echo docker-compose -f $(1)/docker-compose.$(2).yml
endef

.PHONY: \
	down-backend \
	build-backend \
	run-backend \
	count-backend \
	config-backend \

	env \


config-backend:
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) config

count-backend:
	find backend/src -name tests -prune -o -type f -name '*.rs' | xargs wc -l

down-backend:	
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) down \
		--volumes \
		--remove-orphans

run-backend: build-backend
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) up

build-backend: env
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)" \
		$(shell $(call is_need_to_use_cache, $(CACHE)))


env:
	@if [ ! -e .env ]; then\
		echo .env file was not found && \
		exit 1 ;\
	fi

	$(shell sudo chmod +x scripts/create_env.sh)
	$(shell scripts/create_env.sh)


.DEFAULT_GOAL := run-backend