#!/bin/bash

source .env

# Создание окружения для docker
function docker {
cat << EOF > docker/.env
MONGO_INITDB_ROOT_USERNAME=$MONGO_INITDB_ROOT_USERNAME
MONGO_INITDB_ROOT_PASSWORD=$MONGO_INITDB_ROOT_PASSWORD
MONGO_INITDB_DATABASE=$MONGO_INITDB_DATABASE
MONGO_USER=$MONGO_USER
MONGO_USER_PASSWORD=$MONGO_USER_PASSWORD
MONGO_USER_ROLE=$MONGO_USER_ROLE
EOF
}

# Создание окружения для backend
function backend {
cat << EOF > backend/.env
SERVER_PORT=$SERVER_PORT
SERVER_HOST=$SERVER_HOST

MONGO_DB_URL=mongodb://$MONGO_USER:$MONGO_USER_PASSWORD@jokehub_mongodb:27017/$MONGO_INITDB_DATABASE?w=majority
MONGO_DATABASE_NAME=$MONGO_INITDB_DATABASE
EOF
}

docker
backend

# Автоматическое обновление .env.example
if [ $ENV = "local" ]; then\
    cat .env > .env.example ;\
fi
