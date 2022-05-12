#!/bin/bash

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

mongo --username $MONGO_USER --password $MONGO_USER_PASSWORD --authenticationDatabase $MONGO_INITDB_DATABASE $MONGO_INITDB_DATABASE <<EOF
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
EOF