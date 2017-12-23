#!/bin/sh
for i in `seq 40`; do
    ./target/release/random | tee -a beats.log | ./target/release/encode && mplayer out.mp4 
done