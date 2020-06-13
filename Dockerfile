FROM gcc

COPY entrypoint.sh /entrypoint.sh
COPY ./libkonfiscator.so /libkonfiscator.so
COPY driver-program /driver-program
COPY src/konfiscator_stats.h /driver-program/konfiscator_stats.h

ENTRYPOINT /entrypoint.sh 
