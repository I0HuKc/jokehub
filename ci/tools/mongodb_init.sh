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
    db.users.insertOne(
        {
            "_id": "82e1f645-9b38-4773-8fe0-6d98c756f920",
            "username": "shavedkiwi",
            "hash": "\$argon2i\$v=19\$m=4096,t=3,p=1\$polzlXI0YXGFxBp2aFq8orG8XG/VhwlBTlyLP+ZSrCE\$$MONGO_JOKERHUB_SITH_HASH",
            "level": "sith",
            "theme": "light",
            "tariff": "enterprice",
            "created_at": new Date(),
            "updated_at": new Date(),
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

    db.createCollection("joke");
    db.joke.createIndex(
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

    db.createCollection("sessions");
    db.sessions.createIndex(
        {
            "stamp" : 1
        }, 
        {
            "expireAfterSeconds" : 60 * 60 * 24 * 7
        }
    )

    db.createCollection("api_keys");
    db.api_keys.createIndex(
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

    db.createCollection("notifications");
    db.notifications.createIndex(
        {
            "_meta-data.created_at" : 1
        }, 
        {
            "expireAfterSeconds" : 60
        }
    );

    db.createCollection("favorite");
    db.favorite.createIndex(
        {
            "content_id": 1,
        }, 
        {
            "unique": true, 
            "partialFilterExpression": {
                "content_id": {
                    \$type: "string"
                },
            }
        }
    );
EOF