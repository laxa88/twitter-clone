FROM node:16

COPY package.json .
COPY package-lock.json .
RUN npm install
COPY . .

RUN npm run build

# Expose this port so docker-compose can map this to host
EXPOSE 1234

# 0.0.0.0 ensures that docker can be access via localhost from outside
CMD npm run preview -- --host 0.0.0.0