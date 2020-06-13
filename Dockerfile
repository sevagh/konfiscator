FROM fedora

COPY ./libkonfiscator.so /libkonfiscator.so
ENV LD_PRELOAD=/libkonfiscator.so

CMD ["ls"] 
