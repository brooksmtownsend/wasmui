---
title: 2022-11-16 Community Meeting
authors: [brooks]
tags: [community, meeting]
---

### Agenda

- (Demo) `wash up` allowing multiple wasmCloud hosts
- Discuss .NET WebAssembly ecosystem status
- wash-lib and "the great refactor" to a reusable and consumable wash
- Moving to `wit` from smithy, migration discussion

<!--truncate-->

### Meeting Notes

- Moved demo of `wash up` multiple hosts to next week so we can provide a PR link and some better scripts
- Kevin discussed .NET Wasm support and an upcoming blog
  - Good news, .NET is moving in the right direction and the experimental WASI SDK allows you to build fairly portable WASI modules.
  - There doesn't seem to be a current roadmap for .NET or TinyGo around adopting the component model. More research and hopefully comments from those developers needed
  - Currently, the .NET template `wasm-browser` for Blazor builds the module in a way that it requires a JavaScript runtime with proprietary functions, e.g. running in a browser and not on the server-side. It can sort-of work with a NodeJS runtime.
- Upcoming stream on Rust libraries and the wash-lib migration, to be posted to the wasmCloud YouTube
  - Planning to separate out different features into optional installable portions, eventually this will result in a WASI-supported wash distribution
  - With WASI preview 2, the ability to set up Unix-style command line interfaces is proposed in a PR for WASI: https://github.com/Kylebrown9/component-model/blob/typed-main/design/mvp/CLIEmbedding.md
- A few companies and projects have come up with their own way to access keyvalue stores, send messages, etc. We want to be able to collaborate on common interfaces so that WebAssembly modules can easily plug into different projects and be interchangeable. We don't want Wasm standards to divide based on different common interfaces.
  - Bailey pitched a few options for migration models, with the general consensus leaning towards **Option B**. Reasoning: People are okay with breaking changes _occasionally_ but not all the time, so a simple migration path with a potential change is preferred.
    - Option A for aggressive. We can strive for early and perhaps frequent iteration with major revs as the standard evolves. One major for going from what we have directly to wit. Another to adopt the standards we collaborating on in the w3c.
    - Option B for barely major. One major for wit (as close to 1:1 as we have today with smithy). We put out alphas as the standard evolves and no major rev until the standard is fully accepted (there are 4 phases, so we’re talking phase 4)
    - Option C, careful majors. Skip the first 1:1 major rev, and select a set of standards that have mostly early acceptance, e.g. phase 2. This could add 2 weeks. We could start with alpha only before this happens
  - There will still be wasmCloud-specific `wit` files for things that are not common to many many applications out there, but we'll be adopting each standardized interface.
  - wasmCloud is breaking this into chunks
    - Moving the underlying host to `wasmtime`, a mature WebAssembly engine that supports components now
    - Then moving to interfaces and bringing in standards

### Recording

Recording is being uploaded to YouTube and will be displayed promptly
