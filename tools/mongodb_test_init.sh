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
            "username": "tsith",
            "hash": "\$argon2i\$v=19\$m=4096,t=3,p=1\$fwgzbnRyxZiEVtwa5X42gWFVcOpkI3QY6iTRpUqbmDQ\$PRJm/l7Wlee/0i4bfHKEdWx3mpfHlIKH+hJqDPDP/rU",
            "level": "sith",
            "tariff": "enterprice",
            "created_at": "2022-05-21T08:49:20.516119282",
            "updated_at": "2022-05-21T08:49:20.516119963"
        }
    );
    db.users.insertOne(
        {
            "_id": "82e1f645-9b38-4773-8fe0-6d98c756f910",
            "username": "tmaster",
            "hash": "\$argon2i\$v=19\$m=4096,t=3,p=1\$fwgzbnRyxZiEVtwa5X42gWFVcOpkI3QY6iTRpUqbmDQ\$PRJm/l7Wlee/0i4bfHKEdWx3mpfHlIKH+hJqDPDP/rU",
            "level": "master",
            "tariff": "enterprice",
            "created_at": "2022-05-21T08:49:20.516119282",
            "updated_at": "2022-05-21T08:49:20.516119963"
        }
    );
    db.users.insertOne(
        {
            "_id": "82e1f645-9b38-4773-8fe0-6d98c756f320",
            "username": "tpadawan",
            "hash": "\$argon2i\$v=19\$m=4096,t=3,p=1\$fwgzbnRyxZiEVtwa5X42gWFVcOpkI3QY6iTRpUqbmDQ\$PRJm/l7Wlee/0i4bfHKEdWx3mpfHlIKH+hJqDPDP/rU",
            "level": "padawan",
            "tariff": "free",
            "created_at": "2022-05-21T08:49:20.516119282",
            "updated_at": "2022-05-21T08:49:20.516119963"
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
EOF