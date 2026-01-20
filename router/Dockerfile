FROM nginx:1.29-alpine-slim AS base

COPY ./nginx.conf /etc/nginx/nginx.conf.template
COPY ./entrypoint.sh /entrypoint.sh

EXPOSE 80

ENTRYPOINT ["/entrypoint.sh"]
CMD ["nginx", "-g", "daemon off;"]
