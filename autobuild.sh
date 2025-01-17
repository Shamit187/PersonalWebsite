#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Function to handle errors
error_handler() {
    echo "Error occurred in script at line $1."
    exit 1
}

# Trap any error and call error_handler
trap 'error_handler $LINENO' ERR

# Navigate to frontend directory
echo "Changing directory to 'frontend'..."
cd frontend || { echo "Failed to change directory to 'frontend'"; exit 1; }

# Install dependencies
echo "Installing dependencies with pnpm..."
pnpm install || { echo "Failed to install dependencies with pnpm"; exit 1; }

# Build the project
echo "Building the project..."
pnpm run build || { echo "Build failed"; exit 1; }

# Copy build output to /var/www
echo "Copying build output to /var/www..."
cp -r out/ /var/www/ || { echo "Failed to copy build output to /var/www"; exit 1; }

# Change ownership of the copied files
echo "Changing ownership of the copied files..."
chown -R www-data:www-data /var/www/out || { echo "Failed to change ownership of files"; exit 1; }

# Set appropriate permissions
echo "Setting permissions on the copied files..."
chmod -R 755 /var/www/out || { echo "Failed to set permissions"; exit 1; }

# Return to the previous directory
echo "Returning to the previous directory..."
cd - || { echo "Failed to return to the previous directory"; exit 1; }

echo "Script completed successfully."
