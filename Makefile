SHELL = /bin/bash
CACHE = yes

GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p jokehub

DOCKER_ENV = local
DOCKER_DIR = docker

include .env

define is_need_to_use_cache
    if [ $(1) = no ]; then\
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


down-backend:	
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) down \
		--volumes \
		--remove-orphans

run-backend: build-backend
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) up

build-backend:
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)" \
		--build-arg DATABASE_URL="$(DATABASE_URL)" \
		$(shell $(call is_need_to_use_cache, $(CACHE)))
	

.DEFAULT_GOAL := run-backend