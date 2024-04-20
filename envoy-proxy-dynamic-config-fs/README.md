docker rmi envoy-dynamic-fs-shk 
docker build -t envoy-dynamic-fs-shk .
docker run --rm -it -p 19000:19000 -p 10000:10000 envoy-dynamic-fs-shk