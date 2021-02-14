FROM node:latest

COPY pkg /usr/src/app/pkg
COPY www /usr/src/app/www

WORKDIR /usr/src/app/www
EXPOSE 8080
CMD npm run start
