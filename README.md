# High Performance Leaderboard Project

## Front

- [Vite](https://vitejs.dev/) + [pnpm](https://pnpm.io/)
- [Solid JS](https://www.solidjs.com/)
- [Tailwind CSS](https://tailwindcss.com/)

## Back

- [Rust](https://www.rust-lang.org/)
- [Actix](https://actix.rs/)

## DB

- [Redis](https://redis.io/)

## Load Test

- [Locust](https://locust.io/)

#### Dev. Env. Setup

> cd db
> redis-server

> cd back
> cargo run

> cd front
> pnpm dev

> cd test
> locust -f locustfile.py -H http://127.0.0.1:8080
