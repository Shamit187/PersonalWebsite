#!/bin/bash

# Set variables
BLOG_DIR="blog"
SERVICE_NAME="blog_server"

# Function to log messages
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Navigate to the blog directory
if [ ! -d "$BLOG_DIR" ]; then
    log "Error: Directory '$BLOG_DIR' does not exist."
    exit 1
fi

cd "$BLOG_DIR" || { log "Error: Failed to change directory to '$BLOG_DIR'."; exit 1; }

# Stop the service
log "Stopping $SERVICE_NAME service..."
if ! sudo systemctl stop "$SERVICE_NAME"; then
    log "Error: Failed to stop $SERVICE_NAME service."
    exit 1
fi

# Clean previous build artifacts
log "Cleaning previous build artifacts..."
if ! cargo clean; then
    log "Error: Failed to clean build artifacts."
    exit 1
fi

# Build the project in release mode
log "Building the project in release mode..."
if ! cargo build --release; then
    log "Error: Build failed."
    sudo systemctl start "$SERVICE_NAME" # Attempt to restore the service if the build fails
    exit 1
fi

# Start the service
log "Starting $SERVICE_NAME service..."
if ! sudo systemctl start "$SERVICE_NAME"; then
    log "Error: Failed to start $SERVICE_NAME service."
    exit 1
fi

# Return to the original directory
cd - > /dev/null || log "Warning: Failed to return to the original directory."

log "Deployment completed successfully."
exit 0
