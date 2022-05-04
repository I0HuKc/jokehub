FROM rust:1.60 as build

ARG BUILD_ARGS

# Создаю новый пустой проект оболочки
RUN USER=root cargo new --bin jokehub
WORKDIR /jokehub

# Копирую манифесты
COPY ./backend/Cargo.lock ./Cargo.lock
COPY ./backend/Cargo.toml ./Cargo.toml
COPY ./backend/Rocket.toml ./Rocket.toml

# Кэширую зависимости сборки
RUN cargo build $BUILD_ARGS
RUN rm src/*.rs

# Копирую исходное дерево проекта
COPY ./backend/src ./src

# Сборка релиза
RUN rm ./target/release/deps/jokehub*
RUN cargo build $BUILD_ARGS


# Финальная сборка
FROM rust:1.60 

# Копирую артефакты сборки с этапа сборки
COPY --from=build /jokehub/target/release/jokehub .
COPY --from=build /jokehub/Rocket.toml .

ENTRYPOINT [ "./jokehub" ]