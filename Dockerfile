FROM rust:latest AS builder

WORKDIR /www

COPY ./ .

RUN apt-get update  \
    && update-ca-certificates  \
    && apt-get install -y musl-tools musl-dev pkg-config libssl-dev \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:latest AS runtime

WORKDIR /www

ENV TZ=Asia/Shanghai

COPY --from=builder /www/target/x86_64-unknown-linux-musl/release/miniprogram-api ./app

RUN apk add -U --no-cache tzdata \
    && sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories \
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && date

CMD ["/www/app"]