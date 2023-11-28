# ut3

simple command-line game of [ultimate tic-tac-toe](https://mathwithbaddrawings.com/2013/06/16/ultimate-tic-tac-toe/) between two players


## how to install

0. get rust
1. download this repo
2. in the downloaded repo, run the following commands:
	*	`cargo build --release`
	*  `cargo install --path .`
3. done - now you can run the game using the `ut3` command

## how to set up & connect

### setting up the server

you can run `ut3 --server -port [PORT]` to set up a server listening on the specified port (if the port isn't given, it defaults to port `3333`)

### connecting as a client

you can run `ut3 --port [PORT] -ip [IP_ADDRESS]` to connect to the server on that port with the given ip address (as before, port defaults to `3333` if not given, and ip defaults to `localhost`)

## note on convention for denoting squares

for both the supersquares and subsquares in ultimate tic-tac-toe, I denote the squares in this way:

```
 1 | 2 | 3
---+---+---
 4 | 5 | 6 
---+---+---
 7 | 8 | 9
```

to specify which square you want in the game, use these numbers

## misc.

* if you find that I've completely bungled the game logic in some way, let me know

* my thanks to the author of [this repo](https://github.com/margual56/connect4), margual56, which helped me figure out how to work with tcp