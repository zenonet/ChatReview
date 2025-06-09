# ChatReview Backend

The backend is written in Rust using the Axum http server library and Tokio (obviously, lol)
as the async runtime.

## Prerequisites

- Postgresql Database
    Host a postgresql db (preferrebly in a container) and apply the db creation sql [here](../db/create_db.sql)
- A `.env` file
    The .env file contains data necessary to run the API. The following variables need to be set:
    - `DATABASE_URL`: The URL to your Postgresql database (with password and all that)
    - `SERVER_ADDRESS`: The URL you want to host the API on
    - `JWT_SECRET`: 32 random bytes encoded using base64 which serve as the secret for JWT tokens

### Getting 32 random bytes for the JWT_SECRET

In GNU linux, you can use:
```sh
head -c 32 /dev/urandom | base64
```

This will read 32 random bytes, base64 encode them and send them to stdout.

## Containerization

Notes:
- The dockerfile uses the `.env.production` file for parameters so make sure to put your parameters into that instead of the normal `.env` file.
- The container exposes port 80 so it's recommended to set the API port in `.env.production` to 80 and change it later in the docker port mappings.
- SQLX requires the database to be running while building to prepare SQL-Statements and validate queries. Since the DB-URL from the .env file is used, you have to run the database at it's prod URL while building the image.
    I am aware that this is not ideal, but it'll be like that for now. (Tip: You might be able to inject environment variables overriding the DB-URL at build only)

To build the container, run
```sh
podman build -t chatreviewapi .
```
`chatreviewapi` is the tag, the image is saved as.

You can then run the container like this:
```sh
podman run -p 2555:80 --name chatreviewapi chatreviewapi
```

### Shipping the container

To ship the container to a prod server:

- Export the image to a file:
```sh
rm apiImage.tar && podman save -o apiImage.tar localhost/chatreviewapi:latest
```
(Try to remove the image if it already exists becaue podman won't override an image file)
- Move the image to an ssh server using `scp`:
```sh
scp apiImage.tar <username>@<serverAddr>:chatreviewapi.tar
```
- Login to your ssh server
- Import the copied image
    In your home dir, you should now find the file you just copied via scp.
    Import it into your containerization engine:
```sh
podman image load -i chatreviewapi.tar
```
- Run the container
```sh
docker run --name chatreviewapi -d -p 2555:80 localhost/chatreviewapi
```

## Containerization with reverse proxy

If you want to use a reverse proxy, run the container like this instead:
```sh
docker run --name chatreviewapi -d -p 127.0.0.1:2555:80 localhost/chatreviewapi
```
This will not expose it to connection from the outside world.
Then in your reverse proxy, forward connections from your preferred port to this local port.

You might need to enable websocket proxying in your reverse proxy.
I use nginx and I had to do this:
```nginx
location /websocket/ {

    proxy_pass ​http://127.0.0.1:2555;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_read_timeout 86400;
}
```
[Source:Harlan T Wood's Answer on Stackoverflow](https://stackoverflow.com/a/14969925)