# Ravage

A load test suite based on Goose.

## Background

Goose is an excellent load testing tool. It is lightweight, fast, and highly efficient compared to other tools like JMeter and Locust. However, one major drawback of Goose is its lack of a graphical user interface (GUI), which can be particularly challenging for non-engineering teams, such as QA teams. Running load tests with Goose requires knowledge of the Rust programming language, which many QA professionals may not have.

The goal of this project is to provide a simple and user-friendly GUI that enables users to leverage Goose for load testing without requiring any Rust programming expertise.

## Tech Stack

- [Rust](https://www.rust-lang.org/) as main programming language
- [Goose](https://github.com/tag1consulting/goose) as the load testing tool behind the scenes
- [HTMX](https://htmx.org/) and [AlpineJS](https://alpinejs.dev/) as JS Framework
- [Tailwind](https://tailwindcss.com/) and [DaisyUI](https://daisyui.com/) as CSS Framework 

## Local Development

### Pre-requisites

- NodeJS >= 20
- pnpm
- [Rustup](https://rustup.rs/)

### Prepare 

Create `.env` file from `.env.template`

```
cp .env.template .env
```

Run

```bash
make init
```

### Run Application

```bash
make dev
```

## Run on Production

The fastest way  to get started is by using Docker. First, create `.env` file, then run:

```
docker run -v .:/opt/data -p 8080:8080 ghcr.io/8grams/ravage  
```

Once the container is running, open your browser and go to `http://localhost:8080`. Login using `ADMIN_USERNAME` and `ADMIN_PASSWORD` specified in `.env` file. 
It's very recommended to use Reverse Proxy like Nginx.