FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/todo-actix .
CMD ["/app/todo-actix"]