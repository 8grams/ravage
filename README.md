<p align="center">
    <img alt="Ravage" src="https://raw.githubusercontent.com/8grams/ravage/refs/heads/develop/assets/icon.png" height="200">
</p>

# Ravage

A load test suite based on [Goose](https://github.com/tag1consulting/goose/).

## Background

<strong>Goose</strong> is an excellent load testing tool. It is lightweight, fast, and highly efficient compared to other tools like JMeter and Locust. However, one major drawback of Goose is its lack of a graphical user interface (GUI), which can be particularly challenging for non-engineering teams, such as QA teams. Running load tests with Goose requires knowledge of the Rust programming language, which many QA professionals may not have.

The goal of this project is to provide a simple and user-friendly GUI that enables users to leverage Goose for load testing without requiring any Rust programming expertise.

## Tech Stack

- [Rust](https://www.rust-lang.org/) as main programming language
- SQLite as a database
- [Goose](https://github.com/tag1consulting/goose) as the load testing tool behind the scenes
- [HTMX](https://htmx.org/) and [AlpineJS](https://alpinejs.dev/) as JS Framework
- [Tailwind](https://tailwindcss.com/) and [DaisyUI](https://daisyui.com/) as CSS Framework
- [Caddy](https://caddyserver.com) as Webserver

## Run on Production

The fastest way to get started is by using Docker. First, create `.env` file, then run:

```
mkdir data
mkdir tls
docker run -v ./data:/opt/data -p 80:80 -v ./tls:/opt/caddy --env-file .env ghcr.io/8grams/ravage
```

Once the container is running, open your browser and go to `http://localhost:80`. Login using `ADMIN_USERNAME` and `ADMIN_PASSWORD` specified in `.env` file.

To use Caddy's Auto TLS feature, set on `.env`, and bind port 443

```
APP_URL=example.com
```

```
docker run -v ./data:/opt/data -p 80:80 -p 443:443 -v ./tls:/opt/caddy --env-file .env ghcr.io/8grams/ravage
```

## Local Development

### Pre-requisites

- NodeJS >= 20
- bun
- [Rustup](https://rustup.rs/)
- sqlite3 (libsqlite3-dev for ubuntu/mint)

### Prepare

Create `.env` file from `.env.template`

```
cp .env.template .env
```

### Initialize dependencies

Initialize

```bash
make init
```

Run migration

```
make migrate-up
```

Run Application

```bash
make dev
```

## Concept

### Request

A Request represents an HTTP request to an endpoint. A Request is associated with a single path. The base path/URL is defined in the collection. It inherits all HTTP headers defined in its collection, but we can override them or define request-specific headers.

### Collection

A Collection is a group of Requests. A Collection is associated with a single Host or Base URL. It has its own default HTTP headers, which will be inherited by Requests.

### Load Test

We can create a Load Test from a Request or a Collection. If we create a Load Test from a Collection, it will use all associated Requests to attack the target.

## Demo

For demo: https://ravage.8grams.tech

Login: `admin:admin`

## License

MIT
