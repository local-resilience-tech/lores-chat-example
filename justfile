setup:
    cd frontend && npm install

dev:
    mprocs

docker:
    #!/usr/bin/env bash
    set -e
    docker build -t lores-chat-example .
    trap 'docker stop lores-chat-example' INT TERM EXIT
    docker run --rm --name lores-chat-example -p 3000:3000 lores-chat-example &
    echo "Press Control-C 3 times to exit"
    wait
