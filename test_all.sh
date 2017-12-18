#! /bin/sh

for project in day*
do
    cd "$project"
    cargo test
    cd ..
done
