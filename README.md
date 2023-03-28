# connect4-webapp

docker build -t connect4 .
docker run -it -p 8000:8000 -p 18800:18800 -v <connect4-webpp-abs-path>:/app --name connect4_test connect4 bash