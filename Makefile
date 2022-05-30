# Собрать без Docker кеша
#
# make CACHE=no


# Запустить приложение с обновлением миграций
#
# make BACKEND_BUILD_MIGRATE=true


SHELL = /bin/bash
CACHE = yes

GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p jokehub

DOCKER_ENV = 
DOCKER_DIR = ci/docker


include .env

# Функция для установки флага о запере кеширования
define is_need_to_use_cache
    if [ ! $(1) = yes ]; then\
		echo --no-cache ;\
    fi
endef

# Функция генерирующая базу для работы с docker
define base_docker_cmd
	echo docker-compose -f $(1)/docker-compose$(2).yml
endef


.PHONY: \
	down-backend \
	build-backend \
	lib-build \
	run-backend \
	count-backend \
	config-backend \
	test-backend \

	run-frontend \
	build-backend \ 

	env \

run-frontend:
	cd frontend && npx next dev

# Предварительный просмотр docker-compose файла
config-backend:
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) config


count-backend:
	find backend/src backend/tests -name jokehub -prune -o -type f -name '*.rs' | xargs wc -l


# Удалить все volumes и сети созданые этим проектом
down-backend:	
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) down \
		--volumes \
		--remove-orphans

test-backend:
#	Проверяю наличие файла
	@if [ ! -e .env.test ]; then\
		echo .env.test file was not found && \
		exit 1 ;\
	fi

#	Проверяю права файла
	@if [  ! $(shell stat -c %A scripts/create_test_env.sh) = -rwxrwxr-x ]; then\
		sudo chmod +x scripts/create_test_env.sh ;\
	fi

#	Запускаю создание переменных тестового окружения
	scripts/create_test_env.sh

#	Запускаю необходимые службы
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),.test)) up -d

#	Запускаю тесты
	scripts/run_tests.sh	

#	Останавливаю контейнеры всех запущенных тестовых служб
	docker stop jokehub_mongodb_test	

#	Удалаю все что создали контейнеры тестовых служб
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),.test)) down \
		--volumes \
		--remove-orphans

#	Возвращаю назад текущие перепенные окружения
	scripts/create_env.sh

# Запуск сервера
run-backend: build-backend
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) up

# Компиляция библиотеки
lib-build:
	cd backend/lib && cargo build

# Компиляция сервера
build-backend: env
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)" \
		$(shell $(call is_need_to_use_cache, $(CACHE)))


env:
#	Проверяю наличие файла
	@if [ ! -e .env ]; then\
		echo .env file was not found && \
		exit 1 ;\
	fi

#	Проверяю права файла
	@if [  ! $(shell stat -c %A scripts/create_env.sh) = -rwxrwxr-x ]; then\
		sudo chmod +x scripts/create_env.sh ;\
	fi
	
#	Запускаю создание переменных окружения
	$(shell scripts/create_env.sh)


.DEFAULT_GOAL := run-backend