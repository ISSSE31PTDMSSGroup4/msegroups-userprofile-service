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
        --env AWS_ACCESS_KEY_ID="ASIAQS3QSD754P5F54HR" `
        --env AWS_SECRET_ACCESS_KEY="HctIo9ZDecF0slhvEOH59zGC0DD5jcBCBosv5cnQ" `
        --ENV AWS_REGION="ap-southeast-1" `
        --env AWS_SESSION_TOKEN="IQoJb3JpZ2luX2VjEBYaDmFwLXNvdXRoZWFzdC0xIkcwRQIhAJm29ayzMae3Jj+AZE0ODHmtSHJO2kJoC6elKDtJOw6SAiARkZtlC+1IbsYqR0eFeYGoXehRfyZiRhDKbe2LqU2CtirqAggvEAAaDDA0MDUwMTM4NzI1OSIMgmnFndzvA1JIoitdKscCx3TJhm8h+CL7M8/QJRm9AIPdvE4Lcpc4e8yDbZOawY/RrZVWnvzS9qfAcJld6Q23BIaJN/o7roah7sK3L1n7UhDYaoOMi2jDGdoL4zwzBtS6KKQfNLxlxfW8mwmkyQX8XMTPeGw1F/890wS6RKHs9uNh94cMqtLqSeVsVyyCeJgcT1UdJDZJtXpNXCezRKjxrfXYpiWP2PsotKX5iK3n5leV+oAhtLgJRhHNn47UpdMlgzUs4lkwPeq82AbmcJs6nk16oVfv3QydARQESVmfVVojPcFa6R3t76MxVLJGadgKJjU2HU5ToU3krmL16lGwzGlTSfvc+tf4DYpxb1yfFWSjx9eaPdvMvhZVH+sAxniR58jRQioc0uThfoz402DmwouzXie5Ki72VD0SnCFGuE/uNOhT9eIKck19FruJNspBk5l8ylz+MPCipakGOqcBx1k1KELc91/iZawy4SckqqbvZFZSbANG23hixXQfs88Xn5ekevZeS1wN+1HxKXD8z56VoAmC02e9V/pNuRr7yPcOrJQ0BzvXmJEqV0zDPkl7RNdqYdxXBI091PvUB0/qJ97Has8Ig3Wgjw2a2ToQObXIG5V7yLJppkCJZQ0mmw+b2oYVKg38/x1gf/EvUYlaSBZ0Qhx1bNOPPWBOZDBoi0pmWXi22WI=" `
        --env BUCKET_URL="https://user-profilepic-bucket.s3.ap-southeast-1.amazonaws.com/assets/" `
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





