# Websocket PublicAPI - Time

[![Build Status](https://travis-ci.com/darkpools/darkpools_publicapi_ws_time.svg?branch=master)](https://travis-ci.com/darkpools/darkpools_publicapi_ws_time)

Copyright (C) 2019 Bitwyre Technologies LLC

## API Usages

This websocket service doesn't have TLS enabled nor cross-origin checking, it is suppose to be handled by intermediary proxy.

Example using [websocat](https://github.com/vi/websocat):

```bash
websocat ws://localhost:4000/ws/public/time
```

Periodic message:

```json
{"time":{"unixtime":1571744594571020435,"rfc3339":"2019-10-22T11:43:14.571020435+00:00"}}
{"time":{"unixtime":1571744595571648685,"rfc3339":"2019-10-22T11:43:15.571648685+00:00"}}
{"time":{"unixtime":1571744596571345719,"rfc3339":"2019-10-22T11:43:16.571345719+00:00"}}
{"time":{"unixtime":1571744597571387480,"rfc3339":"2019-10-22T11:43:17.571387480+00:00"}}
```

Message anatomy:

- Structure name will always be "time"
- "unixtime" will be in nanoseconds
- "rfc3339" is the time format (in strings) in RFC-3339

## Development

### Pre-requisites

Rust-lang

```bash
curl https://sh.rustup.rs -sSf | sh
```

Development setup

```bash
./dev-setup.sh
```

### Configuration (via Environment)

- Maximum client count for this service to handle, new client(s) beyond this limit will be rejected during handshake
  - Environment Key is **MAX_CLIENT**
  - Default value is **16384**
- Interval in milliseconds for periodic server time broadcast
  - Environment Key is **BLAST_INTERVAL_MS**
  - Default value is **1000**
- IP version 4 for this service to bind/listen to
  - Environment Key is **SERVICE_IP**
  - Default value is **0.0.0.0**
- The TCP port for this service to bind/listen to
  - Environment Key is **SERVICE_PORT**
  - Default value is **4000**
- Route path for the service
  - Environment Key is **SERVICE_PATH**
  - Default value is **/ws/public/time**
- Fastest interval between 2 ping originating from client, the client will get disconnected if too fast
  - Environment Key is **RAPID_REQUEST_LIMIT_MS**
  - Default value is **1000**

### Running with script

Apply recommended code formatting

```text
./run.sh formatter
```

Tests

```text
./run.sh test
```

Lints

```text
./run.sh lint
```

Dependencies audit

```text
./run.sh audit
```

Development run

```text
./run.sh
```

## Contributors

- [Aditya Kresna](https://github.com/ujang360)
