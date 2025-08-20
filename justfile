default: build up prune

alpine: build-alpine up prune

up:
  docker compose up -d

down:
  docker compose down --volumes --remove-orphans --rmi all

build:
  docker build -t actix-test .

build-alpine:
  docker build -f Dockerfile.Alpine -t actix-test .

prune:
  docker image prune -f

prune-all:
  docker container prune -f
  docker image prune -af

sh:
  docker exec -it actix-test sh

logs:
  docker logs actix-test
