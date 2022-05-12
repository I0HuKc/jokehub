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

DOCKER_ENV = local
DOCKER_DIR = docker


include .env

# Функция для установки флага о запере кеширования
define is_need_to_use_cache
    if [ ! $(1) = yes ]; then\
		echo --no-cache ;\
    fi
endef

# Функция генерирующая базу для работы с docker
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


# Предварительный просмотр docker-compose файла
config-backend:
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) config

count-backend:
	find backend/src -name tests -prune -o -type f -name '*.rs' | xargs wc -l

# Удалить все volumes и сети созданые этим проектом
down-backend:	
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) down \
		--volumes \
		--remove-orphans

# Запуск сервера
run-backend: build-backend
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) up

# Компиляция сервера
build-backend: env
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)" \
		$(shell $(call is_need_to_use_cache, $(CACHE)))


# Автоматизированная работа с .env файлами
#
# Команда автоматически создает файлы окружения по нужным каталогам 
# и распихивает по ним нужные данные указанные в глобаьном .env файле.
# Команду нужно выполнять после обновления .env. При выполнении 
# команды обновляется .env.example, но только если глобальное окружение
# (переменная ENV) содержит значение local.
env:
# 	Проверяю наличие .env
	@if [ ! -e .env ]; then\
		echo .env file was not found && \
		exit 1 ;\
	fi

#	Проверяю что скрипт инициализации файлов окружения наделен соответствующими правами
	@if [  ! $(shell stat -c %A scripts/create_env.sh) = -rwxrwxr-x ]; then\
		sudo chmod +x scripts/create_env.sh ;\
	fi
	
	$(shell scripts/create_env.sh)


.DEFAULT_GOAL := run-backend