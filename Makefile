GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p jokehub

.PHONY: \
	init \
	build-backend \
	run-backend \
	test

init:
	if [ ! $(BACKEND_BUILD_TYPE) ] ; then  \		
		export $(cat .env | xargs) ;\
	fi

run-backend: build-backend
	docker-compose -f docker-compose.yml up

build-backend: 
	docker-compose -f docker-compose.yml build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)"


.DEFAULT_GOAL := run-backend