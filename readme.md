# SpecDB Query

## Overview

SpecDB Query is a small service that exposes a searchable database of CPU and GPU (graphics) specifications. The repository contains:

- A Rust HTTP server (`src/`) that serves JSON and Protobuf endpoints and query logic.
- Protobuf definitions used by the Protobuf endpoints.

Goals:
- Provide fast search over component specifications (CPUs, GPUs, APU, architectures).
- Offer both JSON and Protobuf interfaces for clients.
- Allow optional per-spec "extra" data (controlled via configuration).

The intended usecase is with protobufs. The JSON API is for quickly searching/experimenting with the database as a human.

## Prerequisites

A cloning of [SpecDB-rs](https://github.com/Sam-Mear/specdb-rs)

## Api implementation referenece

cli `protoc` is needed to compile. [instalation guide](https://protobuf.dev/installation/). With this, you can generate SpecDB objects in most languages.

See `/php`.

## API Reference

### JSON endpoints:

- `GET /v1/search/{query}` — Search specs (JSON). Searches for CPUs & GPUs.
- `GET /v1/spec/{name}` — Full spec for a named component (JSON). Use the name you found in the search to get the fulls specs.

### Protobuf endpoint:

- `GET /v1/protobuf/search/{query}` — Protobuf search response.
- `GET /v1/protobuf/search_full_specs/{query}` — Full specs in Protobuf.
- `GET /v1/protobuf/cpu/{query}` — CPU-specific Protobuf resource.
- `GET /v1/protobuf/graphics_card/{query}` — Graphics card Protobuf resource.
- `GET /v1/protobuf/apu/{query}` — APU Protobuf resource.

#### Extras

There are more endpoints for adding additional data for the service. These are lost when the SpecDB Query api is stopped. This is for local use & is disabled in the public API.

- `GET /v1/protobuf/extra/{spec_name}`
- `GET /v1/protobuf/extra/export/{spec_name}`
- `POST /v1/protobuf/extra/import/{spec_name}`
- `GET /v1/protobuf/extra/export_all`
- `POST /v1/protobuf/extra/import_all`

## Quickstart — Build & Run (Rust)

Prerequisites:
- Rust toolchain (rustc + cargo)

Build and run (development):

```bash
cargo build
cargo run --release
```

Notes:
- The server reads `config.yaml` from the OS project config directory. On first run it creates an empty file — populate it with `spec_db_path` before starting.
- The server binds to `0.0.0.0:8082` by default (see `src/main.rs`).

Making a simple search request:

```bash
curl "http://localhost:8082/v1/search/ryzen"
```

## Configuration

The service reads configuration from a YAML file stored in the platform configuration directory (using `directories::ProjectDirs`).

Location and format:
- The application creates a config file at the per-user config directory and expects `config.yaml` to exist.
- The config requires at least one key: `spec_db_path` which points to the folder or file containing spec data.
- Optional key: `allow_extras` (boolean, defaults to `false`) — when `true`, the service will enable "extras" import/export and per-spec extras handling.

Example `config.yaml`:

```yaml
spec_db_path: /path/to/specs
allow_extras: true
```

Behavior:
- If `config.yaml` is empty or missing required keys the server `panic!`s on startup (the runtime checks are in `src/main.rs`).


## Protobuf Interfaces

Protobuf interfaces are used to provide compact binary responses.

You wont need to worry about the contents of the proto files, other than knowing what to expect, and what to map the binary responses to which generated object. See the PHP example.

Location:
- Protobuf source files appear in `php/query.proto` and `src/queries/protobuf/query.proto` (and related generated modules).

Endpoints using Protobuf return messages described by those `.proto` files. Common messages include:
- `SearchRequest` / `SearchResponse` — top-level search I/O
- `Spec` / `FullSpec` — full spec payloads for CPU/GPU/APU
- Type-specific messages for CPU, GPU, architecture etc.

Using the Protobuf endpoints:
- Clients should set `Accept: application/x-protobuf` (or consume binary response) and decode with matching generated code.
- For POST endpoints, send the Protobuf-serialized request in the body with the correct Content-Type.
