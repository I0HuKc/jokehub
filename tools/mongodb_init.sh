#!/bin/bash

mongo -- "$MONGO_INITDB_DATABASE" <<EOF
    db.createUser({
    user: "$MONGO_USER",
    pwd: "$MONGO_USER_PASSWORD",
    roles: [
        {
        role: "dbOwner",
        db: "$MONGO_INITDB_DATABASE",
        },
    ],
    });
EOF