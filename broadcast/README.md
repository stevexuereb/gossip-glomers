# [Broadcast](https://fly.io/dist-sys/3a/)

## Running

1. Install [maelstrom](https://github.com/jepsen-io/maelstrom/blob/main/doc/01-getting-ready/index.md#prerequisites)

    ```sh
    brew install openjdk graphviz gnuplot
    curl -L --output ~/Downloads/maelstrom.tar.bz2 https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2
    tar -xvf maelstrom.tar.bz2
    ```

1. Compile

    ```shell
    cargo build
    ```

1. Run tests

    ```shell
    ~/Downloads/maelstrom/maelstrom test -w broadcast --bin ./target/debug/broadcast --node-count 1 --time-limit 20 --rate 10
    ```
