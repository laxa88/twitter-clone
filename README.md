# Twitter clone challenge

[![Playwright Tests](https://github.com/laxa88/twitter-clone/actions/workflows/playwright.yml/badge.svg?branch=main)](https://github.com/laxa88/twitter-clone/actions/workflows/playwright.yml)

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

## Quick start (Server and app)

Run `docker-compose -f docker-compose.dev.yml up`. You can access the app from `localhost:1234`.

Don't forget to run `docker-compose -f docker-compose.dev.yml down` to teardown unused containers when you're done.

## Tests

Full e2e (end-to-end) tests are run from CI in from `docker-compose.ci.yml`. This is only used in Github actions because the variables are injected as secrets from Github workflow.

To run the e2e tests locally with local environments variables:

- Duplicate `.env.example` file and rename it to `.env`
- Run `docker-compose up` to start the DB and Rust server in production mode.
- Go to `fe` folder and run `npm run test` to run the e2e tests.

## Local development

- Start the DB docker container. It's more convenient that installing Postgres on your computer.

  ```bash
  # Spin up DB (for local dev) in detached mode
  docker-compose up -d db

  # Stop and teardown when you're done
  docker-compose down
  ```

- For backend development, navigate to `rust` folder and start the API:

  ```bash
  cargo run
  ```

  You can now test that the API is running via `localhost:8000`

- For frontend development, navigate to `fe` folder and start the app:

  ```bash
  npm run dev
  ```

  Check the logs for what host and port to access, e.g. `http://127.0.0.1:5173` (default)

## Todos (API)

User
- [x] Create user
  - [ ] Validate empty inputs
- [x] List users
- [x] Get user by ID
- [x] Auth user

Tweet
- [x] Create tweet
- [x] Get tweet by ID
- [x] Get tweets by user
- [ ] ~Delete tweet~

Other
- [ ] Generic error handler
- [ ] List pagination
- [ ] Generic error handler (e.g. when JWT expires)

## Todos (UI)

- Landing page
  - [x] Register user
  - [x] Login
  - [x] Logout

- Logged-in pages
  - [x] Add tweet
  - [x] Show own tweets
  - [ ] Show tweets by user

## Todos (dev)

- [x] Build apps as docker image
- [ ] Deploy all images for production
- [x] E2E tests
  - [x] in local
  - [x] in CI
- [ ] CI deployment
  - [x] run tests
  - [x] build image in CI
  - [ ] ~push image to personal repo~

## Will not do

- Block/unblock user and tweets
- Followers
- Tweet likes
- Tweet replies
- Retweets
