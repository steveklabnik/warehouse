# Warehouse

This project serves up the index for Crates.io.

This is mostly a fun hack and also a yak of mine. Not ready to be put anywhere
on the web. Yet. Iron on the backend, Ember on the front.

## Backend

To start up the backend:

```
$ cargo run
```

## Frontend

To start up the frontend:

```
$ cd frontend
$ ember serve
```

Once both of these are running, hit up [http://localhost:4200/crates][]. The first load will take
a while, as it has to load up the index.
