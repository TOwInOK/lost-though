# lost-though

# Что тут есть:
  * [Для кого](#это-серверный-инструмент-api-ориентированный-на-разработчиков-создающих)
  * [RestFul?](#к-чему-отнести-api)
  * [Что может](#что-умеет-api-на-данный-момент)
  * []

    
#### Это серверный инструмент (API) ориентированный на разработчиков создающих:
  - сайты-постовики
  - личный блок
  - социальную сеть

# К чему отнести API?
 - Данное API стараться быть RESTful.
 - Вся валидация идёт через блок body.
   * Пример смены пароля:
     ```json
     {
    	"auth":
      {
        "name": "example_user",
        "password": "OLD??"
    	},
    	"newpassword": "NEW__#@!PASSWORD"
     }
      ```

# Что умеет API на данный момент?
- Пользователи
  * Создавать пользователя
      - [URL/user/USERNAME](https://api.lost-umbrella.com/user/example_user)
  * Почуить информацию о пользователе - AUTH
      - [URL/user/USERNAME/settings](https://api.lost-umbrella.com/user/example_user/settings)
  * Изменять пароль пользователя - AUTH
    - [URL/user/USERNAME/settings/changepass](https://api.lost-umbrella.com/user/example_user/settings/changepass)
      ```json
      {
        "auth":
        {
          "name": "example_user",
          "password": "example"
        },
        "newpassword": "example"
      }
      ```
  * Изменять пароль + почту пользователя - AUTH
    - [URL/user/USERNAME/settings/change](https://api.lost-umbrella.com/user/example_user/settings/change)
      ```json
      {
      	"user": {
      	"name": "example_user",
      	"password": "oldtest123",
      	"email": "CHANGABLEexample@example",
      	},
      	"newpassword": "CHANGABLEtest123"
      }
      ```
  * Получать посты пользователя
    - [URL/user/USERNAME/posts](https://api.lost-umbrella.com/user/example_user/posts)
  * Удалять пользователя - AUTH
    - [URL/user/USERNAME/delete](https://api.lost-umbrella.com/user/delete)
      ```json
      {
      	"name": "username",
      	"password": "password"
      }
      ```
  - Коды
    * Отсылать код на валидацию
      - [URL/user/code/USERNAME](https://api.lost-umbrella.com/user/code/example_user)
      ```json
      ```
    * Принимать коды на валидацию
      - URL/user/code/CODE
      ```json
      {
      	"name": "example_user",
      	"password": "example"
      }
      ```
- Посты
  * Получить все посты
      - [URL/post/page/all](https://api.lost-umbrella.com/post/page/all)
  * Получить все посты по страницам
      - [URL/post/page/NUMBER](https://api.lost-umbrella.com/post/page/1)
  * Создать пост - AUTH
      - [URL/post/create](https://api.lost-umbrella.com/post/create)
      ```json
      {
      	"post": {
      		"author": ["example_user", "my_frined"],
      		"underlabel": "Do you wanna some cars???",
      		"label": "My summer car",
      		"text": "This is best game in my life!!!",
      		"footer": "UDP: i'm not shure about that...",
      		"tags": [
      			"car",
      			"summer"
      		]
      	},
      	"auth": {
      		"name": "example_user",
      		"password": "example"
      	}
      }
      ```
  * Получить пост по id
      - URL/post/ID
  * Редактировать пост - AUTH
      ```json
      {
        "_id": {
        "$oid": "UUID"
        },
      	"post": {
      		"author": ["example_user", "my_frined"],
      		"underlabel": "Do you wanna some cars???",
      		"label": "My summer car in 2023",
      		"text": "I HATE THIS F*CKin 6yme",
      		"footer": "hapy new year in 2023!",
      		"tags": [
      			"car",
      			"summer",
            "hate"
      		]
      	},
      	"auth": {
      		"name": "example_user",
      		"password": "example"
      	}
      }
      ```
  - Коммантарии
    * Добавлять - AUTH
      - URL/post/ID/comment/add
      ```json
      {
        "comment":
        {
          "author": "example_user",
          "text": "www",
          //Option!
          "reject": "ID"
        },
        "auth":
        {
          "name": "example_user",
          "password": "example"
        }
      }
      ```
    * Удалять - AUTH
     - URL/post/ID/comment/delete
      ```json
      {
      	"name": "example_user",
      	"password": "test123"
      }
      ```
- Поиск
  * Не точный поиск
    - URL/search/vague/TEXT

  * Не точный поиск по странично
    - URL/search/vague/TEXT/NUMBER

  * Точный поиск
    - URL/search/fair/TEXT

  * Точный поиск по странично
    - URL/search/fair/TEXT/NUMBER

 
## Что такое "по странично"?
1 страница = 10 объектов в ответном запросе.




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
