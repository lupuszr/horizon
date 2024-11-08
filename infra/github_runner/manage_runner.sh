#!/bin/bash

function start() {
    docker-compose up -d
    echo "Runner started"
}

function stop() {
    docker-compose down
    echo "Runner stopped"
}

function logs() {
    docker-compose logs -f
}

function restart() {
    stop
    start
}

function status() {
    docker-compose ps
}

case "$1" in
    start)
        start
        ;;
    stop)
        stop
        ;;
    restart)
        restart
        ;;
    logs)
        logs
        ;;
    status)
        status
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs|status}"
        exit 1
        ;;
esac
