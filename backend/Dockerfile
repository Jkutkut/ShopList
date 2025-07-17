# -----------------------------------
FROM jkutkut/docker4rust as builder

RUN apk add protobuf # protobuf-compiler

# -----------------------------------
# FROM alpine:3.16 as release
# 
# RUN apk add --update XXXXXXXX && \
#   rm -rf /var/cache/apk/*
# 
# WORKDIR /app
# 
# COPY target/release/XXXXXXXX/XXXXXXXX .
# 
# ENTRYPOINT ["./XXXXXXXX"]
