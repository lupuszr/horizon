# Github Runner for building and running tests for horizon

## Usage

# Start runner
./manage-runner.sh start

# View logs
./manage-runner.sh logs

# Check status
./manage-runner.sh status

# Stop runner
./manage-runner.sh stop

# Restart runner
./manage-runner.sh restart


## Maintainance

# Update runner
docker-compose build --no-cache
docker-compose up -d

# View resource usage
docker stats github-runner

# Clean up old containers/images
docker system prune
