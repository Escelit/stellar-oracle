FROM node:20-alpine AS builder

WORKDIR app/

COPY sdk/package*.json ./

RUN npm ci
COPY sdk/ ./
RUN npm run build


FROM node:20-alpine

WORKDIR app/

COPY --from=builder /app .

CMD ["npm", "run", "publisher-bot"]




