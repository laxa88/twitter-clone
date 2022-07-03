# Twitter clone challenge

This repository is just a practice project for me to write a full stack application, and as a boilerplate and reference for my future projects.

## Backend

- Language: Rust
- Framework: Rocket

## Backend (Node)

- Language: Javascript
- Framework: Express
- DB: Postgres

## Frontend

- Language: Typescript
- Framework: SvelteKit
- E2E test: Playwright

## Quick start (Node)

The Node backend is just a hello-world project to test that a node API server is able to communicate with a Postgres DB in a docker container.

- Start the docker container(s) in detached mode: `docker-compose up -d`
- Start the server: `cd node && npm start`
- Hit the endpoints on `localhost:3000`
- Refer to `routes` and `services` source for implemention details.

## Quick start (Rust)

- Start the docker container(s) in detached mode: `docker-compose up -d`
- Start the server: `cd rust && cargo run`
- Hit the endpoints on `localhost:8000`
- Refer to `main.rs` for details.

## Todos (API)

User
- [ ] Create user
- [ ] Auth user

Tweet
- [ ] Create tweet
- [ ] Get tweet
- [ ] Delete tweet

## Todos (dev)

- [ ] Build apps as docker image
- [ ] Deploy all images for production
- [ ] E2E tests
  - [ ] in local
  - [ ] in CI
- [ ] CI deployment
  - [ ] run tests
  - [ ] build image in CI
  - [ ] push image to personal repo