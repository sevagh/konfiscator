FROM fedora

RUN dnf update -y
RUN dnf install -y python3 python3-pip gcc strace

COPY entrypoint.sh /entrypoint.sh
COPY ./libkonfiscator.so /libkonfiscator.so
COPY driver-program /driver-program
COPY prometheus-metrics /prometheus-metrics
COPY empty-stats /tmp/konfiscator-stats

RUN python3 -m pip install prometheus_client

EXPOSE 8000

ENTRYPOINT /entrypoint.sh 
