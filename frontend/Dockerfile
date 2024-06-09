FROM node:20 as builder
WORKDIR /app
COPY . .
RUN yarn install
RUN yarn build

FROM nginx:alpine
COPY --from=builder /app/build /usr/share/nginx/html
COPY /deployment/nginx.default.conf /etc/nginx/conf.d/default.conf
