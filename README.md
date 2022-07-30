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

## Todos (UI)

- Landing page
  - [x] Register user
  - [ ] Login
  - [ ] Logout

- Logged-in pages
  - [ ] Add tweet
  - [ ] Show own tweets
  - [ ] Show tweets by user

## Todos (dev)

- [ ] Build apps as docker image
- [ ] Deploy all images for production
- [x] E2E tests
  - [ ] in local
  - [ ] in CI
- [ ] CI deployment
  - [ ] run tests
  - [ ] build image in CI
  - [ ] push image to personal repo

## Will not do

- Block/unblock user and tweets
- Followers
- Tweet likes
- Tweet replies
- Retweets