---
title: 2022-11-23 Community Meeting
authors: [brooks]
tags: [community, meeting]
---

### Agenda
- (DEMO) `wash up` with multiple hosts
- (DEMO) Kafka `wasmcloud:messaging` Capability Provider 
- Repository organization proposal for wasmCloud organization 
- wasmtime engine migration update

<!--truncate-->

### Meeting Notes
- With some special environment variables, `RELEASE_NODE` and `WASMCLOUD_DASHBOARD_PORT`, `wash up` can run multiple hosts
- Sample implementation of kafka-compatible pubsub messaging in [https://github.com/wasmCloud/examples/pull/177](https://github.com/wasmCloud/examples/pull/177), compatible with any Kafka-compliant API
- Organization proposal
  - Merge interfaces, capability providers, and examples into `wasmcloud/wasmcloud`. The goal would be for better discoverability of what you can do with wasmCloud and to provide a central home for all things useful for wasmCloud developers
  - No action to be taken here quite yet
- `wasmtime` is coming along as a reusable Elixir library that's specific for wasmCloud, but can progress independently of the wasmCloud host
- Discussion on Wasm engines and `wasmtime` revolved around component model support, and our Elixir library will be able to support different engines once other engines release NIF-compatible SDKs and work well with the component model. This goes towards our goal of running on a variety of devices.

### Recording
Recording is being uploaded to YouTube and will be displayed promptly
