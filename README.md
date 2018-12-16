# Statik

Statik is a simple static file server. It is written in Rust and uses the Actix web framework.

## How to use

Statik can be configured using either command line arguments or environment variables.

| Short | Long      | Environment | Description |
|-------|-----------|-------------|-------------|
| p     | port      | PORT        | The port on which the web server will accept connections |
| d     | directory | DIRECTORY   | The directory where the static files that should be served are located |
| f     | file      | FILE        | The static file to serve for requests to `/`. Defaults to `index.html`|

### Example

```
statik -p 4000 -d /Users/me/static -f index.htm
```
