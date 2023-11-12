#!/usr/bin/env bash

if [ "$1" == "build" ]
then
    docker build -t kedy1ykh/mse-user-profile-service .
    echo "Build successful"
elif [ "$1" == "run" ]
then
    docker run -ti --rm -p 8001:8001 \
    --name userprofile_service \
    --env MONGOURI='mongodb+srv://admin:{password}@cluster0.litlwrq.mongodb.net/?retryWrites=true&w=majority' \
    --env AWS_ACCESS_KEY_ID="{ID}" \
    --env AWS_SECRET_ACCESS_KEY="{KEY}}" \
    --env AWS_REGION="ap-southeast-1" \
    --env AWS_SESSION_TOKEN="{TOKEN}" \
    --env BUCKET_URL="https://user-profilepic-bucket.s3.ap-southeast-1.amazonaws.com" \
    --env AWS_S3_BUCKET="user-profilepic-bucket" \
    kedy1ykh/mse-user-profile-service
elif [ "$1" == "clean" ]
then
    docker rmi -f kedy1ykh/mse-user-profile-service
    docker system prune -f
fi