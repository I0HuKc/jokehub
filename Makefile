GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p jokehub

DOCKER_ENV = local
DOCKER_DIR = docker

.PHONY: \
	init \
	build-backend \
	run-backend \

init:
	if [ ! $(BACKEND_BUILD_TYPE) ] ; then  \		
		export $(cat .env | xargs) ;\
	fi

run-backend: build-backend
	docker-compose -f $(DOCKER_DIR)/docker-compose.$(DOCKER_ENV).yml up

build-backend:
	docker-compose -f $(DOCKER_DIR)/docker-compose.$(DOCKER_ENV).yml build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)"


.DEFAULT_GOAL := run-backend