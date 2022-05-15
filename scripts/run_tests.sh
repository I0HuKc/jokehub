#!/bin/bash

error_exit()
{
    echo "Error: $1"
}


cd backend && cargo test || error_exit "tests failed."