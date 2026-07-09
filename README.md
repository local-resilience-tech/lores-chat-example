# Lores Chat Example

## Running this example

Developers can drive this using the justfile. Install [Just](https://github.com/casey/just) and then execute the following:

To fetch needed dependencies:

```bash
just setup
```

To run:

```bash
just dev
```

## Deployment

If you'd like to deploy this to a server, the best approach is to use [the Co-op Cloud recipe](https://recipes.coopcloud.tech/lores-chat-example).

If you'd like to deploy a custom build, then you can compile and install with Docker, using the [Dockerfile](./Dockerfile) in this repository.
