FROM fedora

COPY entrypoint.sh /entrypoint.sh
COPY ./libkonfiscator.so /libkonfiscator.so
RUN dnf update -y
RUN dnf install -y strace

ENTRYPOINT /entrypoint.sh 
