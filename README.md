# my_loadbalancer
Simple load balancer implementation using multithreading written in rust 🦀 and with a simple round robbin algorithm, at this stage it forwards only GET requests.

## How to use it
You must define your target servers by using an environment variable named LB_SERVERS, servers must be separated by a comma.
```sh
export LB_SERVER="http://127.0.0.1:1234,http://127.0.0.1:8888"
```

And you must define your own server address by using another environment variable named LB_HOST.
```sh
export LB_HOST="127.0.0.1:3000"
```

Finally, you can run the rust program by using
```sh
cargo run
```