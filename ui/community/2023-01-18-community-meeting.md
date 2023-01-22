---
title: 2023-01-18 Community Meeting
authors: [brooks]
tags: [community, meeting]
description: Agenda, notes, and recording for the 2023-01-18 community meeting
---

import ReactPlayer from 'react-player/youtube';

### Agenda

- Community meeting streams implications
- (DEMO) wasmCloud, wash, and starting actors from file
- wascap 0.9.2 upgrade, breaking changes and compatibility notes

<!--truncate-->

### Meeting Notes

- Patrick demoed the feature in the wasmCloud PR https://github.com/wasmCloud/wasmcloud-otp/pull/529, which allows developers to send a `wash` command to start an actor from a local file
  - Historically, we held off on this feature in wasmCloud due to the security and distributed systems implications
    - Security: Loading arbitrary bytes off disk is not a production-safe feature
    - Distributed: The same command is handled differently by different wasmCloud hosts, losing a filesystem if a virtual machine restarts can lead to corrupt environments
  - After brainstorming, we decided to accept Patrick's PR along with an environment variable that is _false_ by default for loading actors from files. Developer tools like `wash up` can enable this by default for a local development experience
  - A nice feature to have after the initial PR would be a `wash watch` or `wash auto` command that automatically watches a local actor project for changes and automatically updates the actor (e.g. hot reloading)
  - We also discussed adding the ability to start wasmCloud actors by transmitting bytes over NATS, so a valid control interface connection can remove the need for a file on disk. This will require more brainstorming and is captured as a task for the wasmCloud maintainers
- Cloud Native Wasm Day EU CFPs are now open! https://events.linuxfoundation.org/kubecon-cloudnativecon-europe/cncf-hosted-co-located-events/cloud-native-wasm-day/

### Recording

<ReactPlayer url='https://youtu.be/ouCl7gicXTc' controls />
