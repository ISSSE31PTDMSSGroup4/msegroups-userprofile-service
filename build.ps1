# Use this to bypass
# powershell -ExecutionPolicy ByPass -File ScriptFileName.ps1
param (
    [string]$option
)

# Define the Docker image name
$imageName = "kedy1ykh/mse-user-profile-service"

# Check the provided option
switch ($option) {
    "build" {
        # Build Docker image
        Write-Host "Building Docker image $imageName"
        docker build -t $imageName .
    }
    "run" {
        # Run Docker container from the image
        Write-Host "Running Docker container from image $imageName"
        docker run -ti --rm -p 8001:8001 `
        --name userprofile_service `
        --env MONGOURI='mongodb+srv://admin:Password123!@cluster0.litlwrq.mongodb.net/?retryWrites=true&w=majority' `
        --env AWS_ACCESS_KEY_ID="{ID}" `
        --env AWS_SECRET_ACCESS_KEY="{KEY}" `
        --ENV AWS_REGION="ap-southeast-1" `
        --env AWS_SESSION_TOKEN="{TOKEN}" `
        --env BUCKET_URL="https://user-profilepic-bucket.s3.ap-southeast-1.amazonaws.com" `
        --env AWS_S3_BUCKET="user-profilepic-bucket" `
        $imageName
    }
    "clean" {
        # Remove Docker image and prune the system
        Write-Host "Cleaning Docker resources for image $imageName"
        docker rmi -f $imageName
        docker system prune -f
    }
    default {
        Write-Host "Invalid option. Please choose 'build', 'run', or 'clean'."
    }
}





