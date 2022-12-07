# ggtmBlog

## Development

Install the dependencies and build the server

```bash
yarn install
cargo build
```

Start development

```bash
yarn dev
```

## Run in Docker

### Build the Image

Run the fllowing script to build the blog app image in local machine.

```bash
docker build -t blog-app .
```

### Run the container

mount the configuration file into the container and create an external port map for the container to connect to it.

```bash
docker run --name blog-app -p 8080:8080 --link postgres  \
    -v /path/to/config.toml:/etc/blog-server/config.toml \
    -d blog-app
```

## Settings

### TLS/HTTPS

Put the self-signed certificate in this directory as an example but your browser would complain thatit isn't secure. So we recommend to use [`mkcert`](https://github.com/FiloSottile/mkcert) to trust it. To use local CA, running:

```bash
mkcert -install
```

If you want to generate your own cert/private key file, then run:

```bash
mkcert 127.0.0.1 localhost
```

Set the corresponding options in [`config.toml`](./config.toml) as follows:

```toml
[actix.tls]
enabled = true
certificate = "/path/to/*.crt"
private-key = "/path/to/*.key"
```
