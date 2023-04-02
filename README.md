# connect4-webapp

docker build -t connect4 .
docker run -it -p 8000:8000 -p 10000:10000 -p 12000:12000 -p 14000:14000 -p 16000:16000 -p 18000:18000 -v <connect4-webpp-abs-path>:/app --add-host=host.docker.internal:172.17.0.1 --name connect4_test connect4:latest bash

mongodb:
mongod --bind_ip 0.0.0.0 --port=8080 --dbpath=backend/data

backend:
docker exec -it connect4_test bash
export MONGOURI=mongodb://172.31.60.145:8080
cd backend
cargo run

frontend:
docker exec -it connect4_test bash
cd frontend
trunk serve --address=0.0.0.0 --port=10000
