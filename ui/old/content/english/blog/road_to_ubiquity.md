---
title : "WebAssembly and the Road to Ubiquity"
image : "images/blogs/ij_map_sample.jpeg"
date: 2022-09-19T9:00:00-04:00
author: "Kevin Hoffman"
author_profile: "https://www.linkedin.com/in/%F0%9F%A6%80-kevin-hoffman-9252669/"
description : "A brief look down WebAssembly's road to ubiquity"
categories: ["webassembly", "wasmcloud", "ubiquity", "wasi", "component model"]
draft : false
---

It may seem odd or counter-intuitive, but most of us within the WebAssembly community are eagerly awaiting the day that WebAssembly becomes _"boring"_. Choosing so-called boring technology is a good, safe bet for building production systems. Boring technology does what it's supposed to, it's easy to work with, it doesn't crash or break down, and has a simple developer experience. This is what we want WebAssembly to be: boring and **_ubiquitous_**.

Unfortunately, we're not there yet. WebAssembly is far from boring these days. Building a `.wasm`  module involves a lot of bespoke tooling, knowledge (much of which is tribal or hard to discover), plugins, and extensions. The experience varies between languages, and, much to our chagrin, even operating systems. Worse, the experience also varies whether you're targeting a browser or not, despite our protests that the browser should never be treated as a "more equal" citizen in the Wasm community. Working in the WebAssembly ecosystem today involves a number of high-caliber footguns.

As we move forward along the road to ubiquity, speed bumps and friction points will be smoothed out and the high-drama of today will become the "boring" of tomorrow.

In the not-so-distant future, WebAssembly will cease to become an end goal and will instead become a simple implementation detail; a mere checkbox or command-line flag. Developers using Apple's Xcode tools will create a project and check a box indicating that they're targeting the build as a WebAssembly Component. VScode and other IDEs will have integrated support for components that expose their interfaces through the component model's `.wit`  (or whatever it becomes next) definitions, so people will get full syntax highlighting and type checking when they create a component that relies upon another component. There will be dependency visualization tools showing the chain of required components. We will have public repositories, artifactory, and other repository vendors will all natively support storing, querying, and annotating WebAssembly modules.

Developers using wasmCloud or Cosmonic will simply choose features from the SDK, compile, and deploy—all while blissfully unaware of how much WebAssembly contributed to that flow.

In this (hopefully near) future, developers will have the luxury of choice at multiple levels: the _engine_ level, the _specification_ level, and the _application_ level.

## Engine-Level Choice
At the **engine** level, as the name implies, developers can choose which low-level engine they want to use for their `wasm` modules. Think of this like picking the right tool from the tool chest; optimizing the choice of engine to your particular needs.

This engine could be optimized and focused on any number of targets or categories like small devices, the cloud, a browser, or other bespoke environments with highly tuned characteristics. In keeping with WebAssembly's portability promise, the choice of engine should never require a refactor, redesign, or even a recompilation.

In the current state of the world, we can (for the most part) pick and choose which engine we want based on features or size or performance optimizations. If we want to explore features from standards that haven't yet been ratified, then we'll want to use [wasmtime](https://wasmtime.dev/).

## Specification-Level Choice
It seems likely that ultimately WASI will not be a single all-encompassing walled box of extras that developers get to use with their modules, but rather a grab-bag of opt-in choices to multiple standards like networking, cryptography, file system and OS access, video cards/GPUs, etc. It's also pretty likely that these opt-in specifications will be defined as components using the component model.

Developers wishing to take advantage of specific WASI features are going to have to make that choice at design time and this choice gets baked into their WebAssembly module. If a developer chooses to write to stdout via WebAssembly, this may require one component while publishing a message over a message broker topic would be yet another component.

WASI is also likely going to be used as a gap bridging device. People who want the portability of WebAssembly, have little interest in the component model, and still want their big legacy libraries to compile and work[^1] properly will leverage WASI-aware compilers/linkers (like the way Rust/LLVM is today) to enjoy the best of both worlds. The ecosystem will likely have multiple "smoke and mirror" tools that highly leverage WASI in clever ways to further provide "magic" and "hand-waving" to hide complexity from developers and operations. Unfortunately, many of these fancy shims will probably still require a browser.

At the **engine** and **specification** level, there should be no issues around vendor lock-in and forcing developers to choose between clouds, platforms, environments, etc. People will be able to choose the engine they need based on their requirements and pick and choose which component or WASI level they need based on requirements, not the artificial limitations we have today.

## Application-Level Choice
Today, the choice of application or application platform to support WebAssembly modules is extremely limited. In addition to the lack of choice variety, each choice requires a _total embrace_ and **_lock_** into a particular vendor's environment, SDK, tool chain, and so on.

In the current ecosystem, if you want edge functions written in WebAssembly, you'll have to use _Fastly_ or _CloudFlare_'s SDK. If you want to add third-party plugins to your application, you'd use _E2 Core_'s SDK. If you want self-contained, freestanding microservices written in WebAssembly, you could use the _Spin_ SDK. If you want portable distributed actors loosely coupled to capability contracts, you may want to use the _wasmCloud_ SDK.

## Embracing the Road to Ubiquity
As the WebAssembly community and ecosystem moves forward, we plan on continuing to move at the vanguard of this momentum, so that we're adapting our foundations to incorporate new power, features, and flexibility as it becomes available and more mature.

### The Developer SDK
The wasmCloud developer experience today is very much influenced by the notion that we originally wanted there to be no measurable difference between consuming [first party capability providers](https://wasmcloud.dev/app-dev/std-caps/) and consuming third party providers.

However, since everyone making an application or platform decision at this level will be opting into the vendor's SDK, it is necessary for us to make this SDK as optimized for developers as as possible, making the following first-party activities and capabilities "brain-dead simple" _by default_:

* HTTP Server
* HTTP Client
* Message Broker
* Blob Store (e.g. S3)
* Key Value
* SQL DB
* Actor-to-Actor Calls

This wasmCloud developer SDK will be a thin wrapper on top of the generated interface code designed to smooth and optimize the developer experience and ease of use. Using this SDK will insulate developers from the churn and change happening at the engine and specification level. As `wit` and the component model mature, we'll simply integrate that work into our SDK, sparing developers from as many breaking changes as possible.

### The Component Model
Once WebAssembly is boring and ubiquitous and tooling exists for all manner of scenarios and the component model has become mature, developers will be able to rapidly adapt their existing code bases to any number of platform providers because consuming them becomes just a matter of generating client code against wit models.

It's worth noting that practically speaking, most developers won't be doing this. The "easy default" case will likely be that developers will choose to use the ready-made, shrink-wrapped SDK of whichever platform they choose to run their code on.

However, being built on the [component model](https://wasmcloud.com/blog/webassembly_components_and_wasmcloud_actors_a_glimpse_of_the_future/) gives us the ability to rapidly adapt to any changes in the community and, more importantly, assert that there is _no vendor lock-in for consuming wasmCloud-based services like Cosmonic_, only the choice of relying on any of the portable component modules that represent the developer-facing contract for capability providers.

There is an effort within the standards community to describe a means by which a wasm component can specify the requirements of its host in terms of the components it uses. At the moment, these specifications are called _world files_, but that nomenclature could change at any moment.

The concept of this type of requirements specification is subtle yet _incredibly_ powerful. In a hypothetical future world where WebAssembly and the component model are both boring, a module running in "vendor A"'s cloud that requires a key-value store could be moved to "vendor B"'s cloud without design change or recompilation, provided both vendors support exposing the WASI key-value component in their "world". This could truly be a fulfillment of the portability promise beyond just escaping from the confines of OS and CPU coupling.

In this bright, idyllic, near-future world[^2], people write code _one way_ and then the selection of where they deploy their code becomes one of personal preference, financial concerns, and available vendor features. No longer will we be locked into a particular vendor because that's the SDK we started experimenting with a year ago.

## Summary
Those of us in the WebAssembly and, more specifically, _wasmCloud_, community are not only looking forward to this bright future, but doing everything in our power to make it happen. If you're interested in helping us usher in this new era, please join our [slack](https://slack.wasmcloud.com) or our weekly community meetings (1PM EST on Wednesdays. Stop into our slack for the Zoom invite) or look for places to contribute to our code.

[^1]: Most organizations who think this will work out of the box without lots of re-engineering will be disappointed.

[^2]: Note that it will take a lot of effort, collaboration, standards work, and coding to bring about this utopian future. But it _is_ possible.