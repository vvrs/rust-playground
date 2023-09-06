FROM ubuntu:22.04

WORKDIR /home/

COPY . .

RUN chmod +x ./setup.sh

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"
