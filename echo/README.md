# [Echo](https://fly.io/dist-sys/1/)

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
    ~/Downloads/maelstrom/maelstrom test -w echo --bin ./target/debug/echo --node-count 1 --time-limit 10 --log-stderr
    ```
