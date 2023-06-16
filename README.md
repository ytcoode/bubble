# Bubble

A Rust-based versatile proxy server that supports SOCKS5, HTTP, and HTTPS protocols, providing fast, efficient, and secure Internet access.

# Usage
```sh
$ bubble --help
   ___          __    __    __
  / _ ) __ __  / /   / /   / / ___
 / _  |/ // / / _ \ / _ \ / / / -_)
/____/ \_,_/ /_.__//_.__//_/  \__/

A Rust-based versatile proxy server that supports SOCKS5, HTTP, and HTTPS protocols, providing fast, efficient, and secure Internet access.

Usage: bubble [OPTIONS] <--socks5|--http|--https>

Options:
      --socks5              Start the socks5 proxy server on the <socks5-ip>:<socks5-port> address
      --socks5-ip <IP>      Specify the IP address for the socks5 proxy server to listen on [default: 0.0.0.0]
      --socks5-port <PORT>  Specify the port number for the socks5 proxy server to listen on [default: 1080]
      --http                Start the http proxy server on the <http-ip>:<http-port> address
      --http-id <IP>        Specify the IP address for the http proxy server to listen on [default: 0.0.0.0]
      --http-port <PORT>    Specify the port number for the http proxy server to listen on [default: 1081]
      --https               Start the https proxy server on the <https-ip>:<https-port> address
      --https-id <IP>       Specify the IP address for the https proxy server to listen on [default: 0.0.0.0]
      --https-port <PORT>   Specify the port number for the https proxy server to listen on [default: 1082]
  -h, --help                Print help
  -V, --version             Print version

Examples:

  Start the socks5 proxy server, listening on '0.0.0.0:1080'

    ./bubble --socks5

  Start the socks5 proxy server, listening on '127.0.0.1:9999'

    ./bubble --socks5 --socks5-ip=127.0.0.1 --socks5-port=9999

  Start the http proxy server, listening on '0.0.0.0:1081'

    ./bubble --http

  Start the http proxy server, listening on '0.0.0.0:9999'

    ./bubble --http --http-port=9999

  Start both the socks5 and http proxy servers simultaneously, listening on their default addresses

    ./bubble --socks5 --http
```

# How to configure XXX to use a socks5 proxy server

### Git

```
git config --global http.proxy socks5h://<socks5-ip>:<socks5-port>
```

### Curl

```
curl --proxy socks5h://<socks5-ip>:<socks5-port> https://www.google.com
```

### Ncat

```
ncat --proxy <socks5-ip>:<socks5-port> --proxy-type socks5 --proxy-dns=remote <hostname> <port>
```
