#!/bin/bash

cargo build --release
cp target/release/cluster-cost darwin
rm -rf target

docker build -t buildercontainer:debian -f docker/Dockerfile.debian .
docker create --name=buildercontainer-debian buildercontainer:debian
docker cp buildercontainer-debian:/opt/cluster-cost/target/release/cluster-cost debian
docker rm buildercontainer-debian
