# build rust backend
FROM rust:1.65.0 as build-backend
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
WORKDIR /usr/src
COPY ./backend ./backend
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./.cargo ./.cargo
RUN cargo update
RUN cargo build --release --all

# build vue frontend 
FROM node:latest as build-frontend
WORKDIR /usr/src
RUN yarn config set registry https://registry.npm.taobao.org
COPY ./frontend ./frontend
COPY ./package.json .
COPY ./yarn.lock .
RUN yarn install
RUN yarn build


FROM debian
WORKDIR /usr/local/bin
COPY --from=build-backend /usr/src/target/release/blog-server ./blog-server
COPY --from=build-backend /usr/src/target/release/migration ./migration
COPY --from=build-frontend /usr/src/www ./www
ENV SERVER_CONFIG=/etc/blog-server/config.toml
ENV MODE=production
CMD [ "blog-server" ]
