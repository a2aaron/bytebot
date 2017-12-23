#!/bin/sh

date >> ~/logs/bytebot.log
number=$RANDOM
let "number %= 4"
if [ "$number" -eq 0 ]
then
    echo "random"
    ./target/release/random | tee ~/logs/bytebot.log 2>&1 | ./target/release/bot
else
    echo "curated"
    head -2 .post-queue | timeout 300 ./target/release/bot >> ~/logs/bytebot.log 2>&1
    sed -ie '1d;2d' .post-queue
fi
