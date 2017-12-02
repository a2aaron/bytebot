#!/bin/sh

date >> ~/logs/bytebot.log
head -2 .post-queue | timeout 300 ./target/release/bot >> ~/logs/bytebot.log 2>&1
sed -ie '1d;2d' .post-queue
