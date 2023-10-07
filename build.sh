#!/usr/bin/env bash

if [ "$1" == "build" ]
then
    docker build -t kedy1ykh/mse-user-profile-service .
    echo "Build successful"
elif [ "$1" == "run" ]
then
    docker run -ti --rm -p 8001:8001 --name userprofile_service kedy1ykh/mse-user-profile-service
elif [ "$1" == "clean" ]
then
    docker rmi -f kedy1ykh/mse-user-profile-service
    docker system prune -f
fi