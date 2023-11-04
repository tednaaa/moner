FROM node:18.18.2-alpine as builder

WORKDIR /app

COPY package-lock.json ./
COPY package.json ./

RUN npm ci --silent

COPY . .

RUN npm run build --prod

FROM nginx:stable-alpine

COPY --from=builder /app/dist/frontend /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
