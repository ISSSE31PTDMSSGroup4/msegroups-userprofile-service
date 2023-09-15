#!/usr/bin/env bash

if [ "$1" == "build" ]
then
    docker build -t user_profile_app .
    echo "Build successful"
elif [ "$1" == "run" ]
then
    docker run -ti --rm -p 8001:8001 --name userprofile_service user_profile_app
elif [ "$1" == "clean" ]
then
    docker rmi -f user_profile_app
    docker system prune -f
fi