#!/bin/bash

# Создание пользователя для БД
mongo -- "$MONGO_INITDB_DATABASE" <<EOF
    db.createUser({
        user: "$MONGO_USER",
        pwd: "$MONGO_USER_PASSWORD",
        roles: [
            {
                role: "$MONGO_USER_ROLE",
                db: "$MONGO_INITDB_DATABASE",
            },
        ],
    });
EOF

# Создание коллекций и установка индексов
mongo --username $MONGO_USER --password $MONGO_USER_PASSWORD --authenticationDatabase $MONGO_INITDB_DATABASE $MONGO_INITDB_DATABASE <<EOF
    db.createCollection("users");
    db.users.createIndex(
        {
            "username": 1
        }, 
        {
            "unique": true, 
            "partialFilterExpression": {
                "username": {
                    \$type: "string"
                }
            }
        }
    );

    db.createCollection("anecdote");
    db.anecdote.createIndex(
        {
            "text": 1
        }, 
        {
            "unique": true, 
            "partialFilterExpression": {
                "text": {
                    \$type: "string"
                }
            }
        }
    );

    db.createCollection("punch");
    db.punch.createIndex(
        {
            "setup": 1,
            "punchline": 1,
        }, 
        {
            "unique": true, 
            "partialFilterExpression": {
                "setup": {
                    \$type: "string"
                },
                
                "punchline": {
                    \$type: "string"
                },
            }
        }
    );
EOF