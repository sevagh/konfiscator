FROM fedora

RUN dnf update -y
RUN dnf install -y python3 gcc strace

COPY entrypoint.sh /entrypoint.sh
COPY ./libkonfiscator.so /libkonfiscator.so
COPY driver-program /driver-program
COPY prometheus-metrics /prometheus-metrics
COPY src/konfiscator_stats.h /driver-program/konfiscator_stats.h

RUN python -m pip install prometheus_client

EXPOSE 8000

ENTRYPOINT /entrypoint.sh 
