default: build up prune

up:
  docker compose up -d

down:
  docker compose down --volumes --remove-orphans --rmi all

build:
  docker build -t actix-test .

prune:
  docker image prune -f

prune-all:
  docker image prune -af

sh:
  docker exec -it actix-test sh
