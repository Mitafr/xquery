cd $PWD
docker compose down
docker rmi app

docker compose down
docker compose up -d