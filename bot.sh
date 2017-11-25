#!/bin/sh

date >> ~/logs/bytebot.log
head -1 .post-queue | ./target/release/bot >> ~/logs/bytebot.log 2>&1
sed -ie 1d .post-queue
