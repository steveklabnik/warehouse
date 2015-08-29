# Warehouse

This project serves up the index for Crates.io.

This is mostly a fun hack and also a yak of mine. Not ready to be put anywhere
on the web. Yet. Iron on the backend, Ember on the front.

Requires Rust nightly.

Don’t forget that this has a submodule!

```bash
$ git submodule update --init --recursive # first time
$ git submodule update
```

## Backend

To start up the backend:

```bash
$ cargo run
```

## Frontend

To start up the frontend:

```bash
$ cd frontend
$ npm install && bower install
$ ember serve
```

Once both of these are running, hit up [http://localhost:4200/crates](http://localhost:4200/crates). The first load will take
a while, as it has to load up the index.
