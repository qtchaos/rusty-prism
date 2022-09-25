FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y curl

EXPOSE 449
WORKDIR /

RUN curl -LJO https://github.com/qtchaos/rusty-prism/releases/latest/download/rusty-prism
RUN chmod a+x ./rusty-prism

CMD ./rusty-prism
