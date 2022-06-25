# Twitter clone challenge

This repository is just a practice project for me to write a full stack application, and as a boilerplate and reference for my future projects.

## Backend

- Language: Rust
- Framework: Rocket

## Frontend

- Language: Typescript
- Framework: SvelteKit
- E2E test: Playwright

## Quick start (Node)

The Node backend is just a hello-world project to test that a node API server is able to communicate with a Postgres DB in a docker container.

Node application test:
- Start the docker container(s) in detached mode: `docker-compose up -d`
- Start the server(s): `cd node && npm start`
- Hit the GET or POST endpoints on `localhost:3000`
- Refer to `routes` and `services` source for implemention details.
