FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y curl

WORKDIR /

RUN curl -LJO https://github.com/qtchaos/rusty-prism/releases/latest/download/rusty-prism
RUN chmod a+x ./rusty-prism

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ./rusty-prism