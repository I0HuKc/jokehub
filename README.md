## Makefile

Загрузка окружения

```
source .env
```

Простой запуск приложения

```
make
```

Запустить приложение с обновлением миграций:

```
make BACKEND_BUILD_MIGRATE=true
```

# Postgres

Обновить схему 

```
diesel migration run --database-url postgres://joker:qwerty@127.0.0.1/jokehub_db
```