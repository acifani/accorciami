FROM node:lts-alpine

EXPOSE 8080
WORKDIR /usr/src/app

COPY package*.json ./
RUN yarn
COPY . .

CMD [ "node", "index.js" ]
