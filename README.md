# lost-though

API позволяющее создать сайт-соцсеть

## Docker-compose example
```yml
version: '3.8'
services:
  # Nginx
  app:
    image: 'jc21/nginx-proxy-manager:latest'
    container_name: 'nginx-proxy-manager'
    restart: unless-stopped
    ports:
      - '80:80'   # Public HTTP Port
      - '443:443' # Public HTTPS Port
      - '81:81'   # Admin Web Port
      # - '21:21' # FTP
    volumes:
      - ./encrypt/data:/data
      - ./encrypt/letsencrypt:/etc/letsencrypt
    networks:
      - main

  # Redis
  redis:
    image: redis:latest
    container_name: redis
    restart: unless-stopped
    ports:
      - '6379:6379' # Redis public port
    networks:
      - main

  # MongoDB
  mongo:
    image: mongo:latest
    container_name: mongo
    restart: unless-stopped
    ports:
      - '27017:27017'
    environment:
      MONGO_INITDB_ROOT_USERNAME: root
      MONGO_INITDB_ROOT_PASSWORD: example
    networks:
      - main

  # Api server 
  monotipe:
    image: ghcr.io/towinok/monotipe:latest
    container_name: monotipe
    restart: unless-stopped
    ports:
      - '8080:8080' # Public Http port
    environment:
      - MONGO_ADDRESS=mongo # Mongo Db address
      - REDIS_ADDRESS=redis # Redis address
      - MONGO_LOGIN=root    # Login for MongoDB
      - MONGO_PASSWORD=example # Password for MongoDB
      - SMTP_LOGIN=example@example.ex # SMTP login
      - SMTP_PASSWORD=XXX777XXX123 # SMTP Password
      - SMTP_ADDRESS=smtp.example@example.com # SMTP Address of provider smtp.
    depends_on:
      - mongo
      - redis
    networks:
      - main

networks:
  main:
    driver: bridge
```
