# mini-balancer

A toy and mini load-balancer.

# Disclaimer

> This is just a toy project of mine, only meant to serve as something fun for me to build with Rust. I do not intend to maintain nor make it production-ready at all. 

# Description

`mini-balancer` is a simple connection-based, transport-level, TCP-only, mini load-balancer, where a *frontend* listens for TCP connections and then forwards them to one of the *backends*.

As an example, one may have more than one instance of a given application offering a TCP server to many clients and wants to distribute connections among such instances.

## Balancing Algorithms

The only balancing algorithm implemented thus far is a simplified round-robin, where a frontend receives incoming connection and then forwards them to a given sequence of `n` backends `(b_1, ..., b_n)` in a cyclical manner, such that:

* the first connection `c_1` goes to `b_1`,
* ...
* the nth connection `c_n` goes to `b_n`,
* the (n+1)th connection `c_n+1` goes again to `b_1` and so the process forever repeats.

## Configuration Schema

An informal description of the [TOML](https://github.com/toml-lang/toml) configuration schema is as follows:

```toml
[frontend]
bind_on = "<IP:PORT>"

[[frontend.backend]]
forward_to = "<IP:PORT>"
```

## Examples

### Balancing Connections Between Two Backends

Let's consider we have two instances of an application listening for connections on `127.0.0.1:8080` and `127.0.0.1:9090`, and we want to have a single end-point on `127.0.0.1:7070` where all connections will be received and then distributed among our instances.

For simplicity, let's simulate our instances as echo servers with `socat`:

```
λ socat -v tcp-l:8080,fork exec:'/bin/cat'
```

```
λ socat -v tcp-l:9090,fork exec:'/bin/cat'
```

Assuming we have saved the following configuration file as `mini-balancer.local.toml`:

```toml
[frontend]
bind_on = "127.0.0.1:7070"

[[frontend.backend]]
forward_to = "127.0.0.1:8080"

[[frontend.backend]]
forward_to = "127.0.0.1:9090"
```

We can then start `mini-balancer` as:

```
λ mini-balancer -v -c mini-balancer.local.toml
```

To simulate a client, we once again rely on `socat` to send a message as:

```
λ echo "first connection" | socat tcp:127.0.0.1:7070 -
```

```
λ echo "second connection" | socat tcp:127.0.0.1:7070 -
```

The first connection should go to the first backend, whereas the second connection should go to the second backend. The logs should look like as follows:

```
May 08 14:18:45.305  INFO frontend{local_address=127.0.0.1:7070}: mini_balancer::balancing::frontend: listening for connections
May 08 14:19:01.488  INFO handle_connection{peer_address=127.0.0.1:49438}: mini_balancer::balancing::frontend: serving connection
May 08 14:19:01.488  INFO handle_connection{peer_address=127.0.0.1:49438}:through{target_address=127.0.0.1:8080}: mini_balancer::balancing::middleware::client: opened client connection
May 08 14:19:01.490  INFO handle_connection{peer_address=127.0.0.1:49438}: mini_balancer::balancing::frontend: served connection
May 08 14:19:13.622  INFO handle_connection{peer_address=127.0.0.1:49444}: mini_balancer::balancing::frontend: serving connection
May 08 14:19:13.623  INFO handle_connection{peer_address=127.0.0.1:49444}:through{target_address=127.0.0.3:9090}: mini_balancer::balancing::middleware::client: opened client connection
May 08 14:19:13.627  INFO handle_connection{peer_address=127.0.0.1:49444}: mini_balancer::balancing::frontend: served connection
```

Lastly, if we open a third connection, it should again go to the first backend.

# Instructions

* Minimum Supported Rust Version (MSRV): `1.51.0`.

## Linting

```
cargo clippy
```

## Building

```
cargo build
```

## Testing

```
cargo test
```
