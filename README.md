# React + Rust(axum) Template

## Client

- vite
- React

## Server

- axum
- tower-http to statically serve the client

## How to use

```
# In the root
make dev

# To build docker image
make build VER=0.1.0

# To push to docker.io
make push VER=0.1.0
```
