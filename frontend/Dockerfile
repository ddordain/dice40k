FROM oven/bun:latest AS builder
WORKDIR /app
COPY package.json ./
COPY . .
RUN bun install
RUN bun run build
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
