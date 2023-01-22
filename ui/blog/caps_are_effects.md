---
title: "wasmCloud Capabilities are Managed Algebraic Effects for WebAssembly Functions"
image: "/img/algebra.jpg"
date: 2022-05-25T9:00:00-04:00
author: "Kevin Hoffman"
author_profile: "https://www.linkedin.com/in/%F0%9F%A6%80-kevin-hoffman-9252669/"
description: "wasmCloud Capabilities are a managed, distributed implementation of algebraic effects"
categories: ["webassembly", "wasmcloud"]
draft: false
---

![algebra](/img/algebra.jpg)

We spend a lot of time talking about how the wasmCloud capability provider system, from its abstract contracts to the ability to hot-swap providers, is a way to separate non-functional requirements from business logic code. While all of that is true, it's also a fairly _enterprisey_ way to describe it. In this blog post, I'll describe them another way using terms from functional programming.

<!--truncate-->

Let's take a look at a simple function:

```
f(x) = x + 12
```

This function is _"pure"_. For every input of `x` there is only one output. In math terms that makes it a proper function, but in programming terms that means it's deterministic, and determinism means we can write unit tests for it. We could write a test that ensures that when I supply a `2` to this function, it returns `14`.

Unfortunately, the second we expand our code beyond the realm of "hello world", it becomes more and more difficult to maintain purity. Our code needs to interact with the world, and it often does so in messy, unpredictable ways. Even if it isn't messy, we frequently see functions where the answer is only predictable for a short period of time.

Let's take a look at this (psuedocode) function from a hypothetical bank back-end that handles an international withdrawal from a customer's account:

```
internationalWithdrawal account amount localCurrency =
    exchangeRate = Market.getRate(localCurrency)
    newAmount = amount * exchangeRate
    fee = Market.getFee(localCurrency)
    Ledger.withdraw(account, amount, newAmount, exchangeRate)
    Ledger.fee(account, fee)
    Ledger.balance(account)
```

This function reaches out to "the market" to get the currency exchange rate between the canonical currency and the local currency. Then it reaches out to get the current fee for international withdrawals. Finally, it consumes a ledger to perform the withdrawal, take away the international transaction fee, and finally return the current/updated balance.

There's nothing pure about this function, but it's an _extremely_ common idiom. This function relies on two external interactions, a `Market` and a `Ledger`. The exchange rate is something that fluctuates constantly throughout the day, and the ledger presumably gives the function access to the account ledger for a specific account.

This function is non-deterministic because it has a number of _algebraic effects_. Such effects are a category for the messy, "impure" things that happen in our functions. For most of us, we probably write more impure functions than not.

I've already tried to make this function fairly clean and simple. In contrast, I've seen functions like this turned into "kitchen sinks" where a single function establishes a connection to two different databases and issues queries directly to them. We fool ourselves into thinking the function is pure by hiding the tight coupling a few layers down in the library, but this is still tight coupling at its worst. In cases like this, you can't test this function without live access to real databases, and after that you have to figure out how to make your tests deterministic (which often involves "test databases").

A lot of us are used to patterns like (micro)services, abstractions, anti-corruption layers, and more all designed to help us mitigate the ugly side effects this function has. But what if we could embrace these effects and write functions that are explicit about their effect needs, _without_ losing testability, flexibility, and purity?

In a traditional object-oriented language or framework, we might treat each of these "effect providers" as an interface and then use something like dependency injection to shunt in an implementation for the effect at runtime (and presumably shunt in a mock during test time).

In wasmCloud, we manage algebraic effects through <u>[capability providers](https://wasmcloud.dev/reference/host-runtime/capabilities/)</u>. Here the capability provider, as seen by the WebAssembly module (<u>[actor](https://wasmcloud.dev/reference/host-runtime/actors/)</u>), is just an abstraction. It's a versioned contract through which the WebAssembly function gets its effects.

The host runtime is responsible for providing an implementation for those effects or effect providers. This implementation is hot-swappable and dynamically configurable. This means that in our preceding international withdrawal example, we could provide a "test market" at unit test time and then a real connection to the market service when running in production. We could also configure the market connection so it could be "real", but point to a different service in staging than in production.

Algebraic effects don't need to be big, high-level concepts like database or networking clients. Even something as basic as logging is an effect (because all I/O is "effectful"). So we might use yet another provider like this:

```elixir
Logger.debug("Performing international withdrawal")
```

wasmCloud takes these algebraic effects even further by requiring each of our WebAssembly modules to be <u>[cryptographically signed](https://wasmcloud.dev/reference/host-runtime/security/)</u> with the explicit list of capabilities it can use (effects it can produce).

Ultimately what we've done is provided a means to maintain portable function purity in WebAssembly modules while allowing for all algebraic effects to not only be testable, but distributed, hot-swappable, and dynamically scalable across a flat topology system comprised of multiple disparate environments.

If you're interested in learning more about capabilities and seeing them in action, take a look at our <u>[examples](https://github.com/wasmcloud/examples/)</u> repository.
