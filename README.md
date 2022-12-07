# ggtmBlog

## Development

```bash
yarn install
cargo build
```

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
