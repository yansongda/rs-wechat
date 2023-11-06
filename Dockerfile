FROM rust:alpine AS builder

WORKDIR /www

COPY ./ .

RUN apk add --no-cache musl-dev pkgconfig libressl-dev g++ \
    && cargo build --release


FROM alpine:latest AS runtime

WORKDIR /www

ENV TZ=Asia/Shanghai

COPY --from=builder /www/target/release/miniprogram-api ./app

RUN apk add -U --no-cache tzdata \
    && sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories \
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && date

CMD ["/www/app"]