Whalepod <small>Quickly generate Dockerfiles</small>
===

Maintaining multiple Docker images for various version of an application - for example, Node.js -
can become tricky past a certain point. Whalepod aims to make it easier to generate and maintain
Dockerfiles using templates and YAML, and creating the neccesary tooling configuration to build
and push them.

whalepod.yml
---

The `whalepod.yml` file contains the information neccesary to generate the Docker images. It
requires a few things; first a name for the Docker image, followed by a list of versions, optional
parameters, and a template Dockerfile that will be processed.

### Example whalepod.yml

```yaml
name: whalepod-example
versions:
  - 1.0
  - 1.1
  - 1.2
  - 2.0
  - 3.0
parameters:
  hello_to: "Docker"
template: |
  FROM alpine:3.7
  LABEL version="<<version>>"

  RUN echo Hello <<params.hello_to>>!
```

Running
---

Running `whalepod` is as simple as executing the command with a valid `whalepod.yml` file. If your
`whalepod.yml` file is located elsewhere or is named something other than `whalepod.yml`, you can
specify another file with `whalepod <filename>`.

Each Dockerfile will live in it's own directory based on the version, and a Makefile will be
generated alongside it with pre-filled commands to build the images. An alternative output folder
can be specified by passing through the `--output <directory>` (`-o=<directory>`) flag.

> TODO: add `docker push` command to Makefile

Disclaimer
---

This is very much an experiment, and my first foray into Rust. Feedback and contributions are
welcome - I intend to maintain it moving forward, and I do not intend to introduce breaking
changes, however keep that in mind!
