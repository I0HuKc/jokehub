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
            "theme": "light",
            "tariff": "enterprice",
            "created_at": new Date(),
            "updated_at": new Date()
        }
    );
    db.users.insertOne(
        {
            "_id": "82e1f645-9b38-4773-8fe0-6d98c756f910",
            "username": "tmaster",
            "hash": "\$argon2i\$v=19\$m=4096,t=3,p=1\$fwgzbnRyxZiEVtwa5X42gWFVcOpkI3QY6iTRpUqbmDQ\$PRJm/l7Wlee/0i4bfHKEdWx3mpfHlIKH+hJqDPDP/rU",
            "level": "master",
            "theme": "light",
            "tariff": "enterprice",
            "created_at": new Date(),
            "updated_at": new Date()
        }
    );
    db.users.insertOne(
        {
            "_id": "82e1f645-9b38-4773-8fe0-6d98c756f320",
            "username": "tpadawan",
            "hash": "\$argon2i\$v=19\$m=4096,t=3,p=1\$fwgzbnRyxZiEVtwa5X42gWFVcOpkI3QY6iTRpUqbmDQ\$PRJm/l7Wlee/0i4bfHKEdWx3mpfHlIKH+hJqDPDP/rU",
            "level": "padawan",
            "tariff": "free",
            "theme": "light",
            "created_at": new Date(),
            "updated_at": new Date()
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
    db.joke.insertOne(
        {
            "_id": "11b923b0-4241-4c32-ac06-f560468fac20",
            "_header": { "counter": NumberLong(0), "timestamp": NumberLong(new Date().getTime()) },
            "category": "joke",
            "text": "test_joke_record_for_random__1",
            "_meta-data": {
                "flags": {
                    "nsfw": true,
                    "religious": false,
                    "political": false,
                    "racist": false,
                    "sexist": false
                },
                "author": "shavedkiwi",
                "tags": ["general"],
                "language": "russian"
            }
        }
    );
    db.joke.insertOne(
        {
            "_id": "11b923b0-4241-4c32-ac06-f560468fac21",
            "_header": { "counter": NumberLong(0), "timestamp": NumberLong(new Date().getTime()) },
            "category": "joke",
            "text": "test_joke_record_for_random__2",
            "_meta-data": {
                "flags": {
                    "nsfw": true,
                    "religious": false,
                    "political": false,
                    "racist": false,
                    "sexist": false
                },
                "author": "shavedkiwi",
                "tags": ["general"],
                "language": "russian"
            }
        }
    );
    db.joke.insertOne(
        {
            "_id": "11b923b0-4241-4c32-ac06-f560468fac22",
            "_header": { "counter": NumberLong(0), "timestamp": NumberLong(new Date().getTime()) },
            "category": "joke",
            "text": "test_joke_record_for_random__3",
            "_meta-data": {
                "flags": {
                    "nsfw": false,
                    "religious": true,
                    "political": false,
                    "racist": true,
                    "sexist": false
                },
                "author": "shavedkiwi",
                "tags": ["for_test"],
                "language": "russian"
            }
        }
    );
    db.joke.insertOne(
        {
            "_id": "11b923b0-4241-4c32-ac06-f560468fac23",
            "_header": { "counter": NumberLong(0), "timestamp": NumberLong(new Date().getTime()) },
            "category": "joke",
            "text": "test_joke_record_for_random__4",
            "_meta-data": {
                "flags": {
                    "nsfw": false,
                    "religious": false,
                    "political": false,
                    "racist": true,
                    "sexist": false
                },
                "author": "shavedkiwi",
                "tags": ["for_test"],
                "language": "english"
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