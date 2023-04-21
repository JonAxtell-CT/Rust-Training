title: Rust Workshop - Session 7
class: animation-fade
layout: true

.bottom-bar[
{{title}}
]

---

count: false

# Leader slide

- Press 'b' to toggle 'blackout mode'
- Press 'p' to toggle 'presenter mode'
- Press 'c' to create a clone of this window which will keep up
- Press left/right to move around slides
- The next slide is where the presentation begins

???

This is where presenter notes appear

---

class: impact

# {{title}}

## Daniel Silverstone <br /><tt>&lt;dsilvers@digital-scurf.org&gt;</tt>

???

- Welcome everyone
- Explain the purpose of the session
  - Discuss API design

---

title:
class: middle

# Topics

1. Coherence vs. Coupling
2. Visibility of items
3. Structs vs. Traits
4. Semantic versioning

---

title: Coherence vs. Coupling
class: impact

## What are 'coherence' and 'coupling'

???

These are terms which are often defined in terms of one another. While not
exact antonyms, they do oppose one another in terms of the design of your code.
When you read around this topic you may also encounter the term 'cohesion' and
while that's not the same as 'coherence' it's related and worth recognising.
We'll mention it later briefly.

---

title: Coupling

# Coupling

- Changes in one part of the code mandate changes elsewhere

???

Coupling is the degree to which changes in one part of your code mandate
changes elsewhere. These might be simple refactoring (changing a function
name) or they might be deeply complex (changing around the order in which API
functions need to be called to make something work).

It can be hard to know if a change will cause previously perfectly good uses
of the code to go bad.

---

title: Coupling

# Coupling

- Changes in one part of the code mandate changes elsewhere
- Code is hard to isolate and thus hard to test

???

Coupled code is often very hard to isolate for the purpose of testing. If your
algorithm is coupled to your UI then it's hard to write tests for it without
also testing your UI at the same time, making it harder to know why a test
failed.

---

title: Coherence

# Coherence

- Isolated (or isolatable)

???

Coherence is the semi-opposite of that - a coherent piece of code is well
isolated (or at least isolatable) - this means that it can be tested without
depending on many other parts of your code, perhaps by means of mock services
or simply because it's a leaf component of some kind.

---

title: Coherence

# Coherence

- Isolated (or isolatable)
- Comprehensible (Grokable)

???

By dint of being coherent, such code is often much easier to document, clearer
in its intent and scope, and thus much easier for a human to understand. Once
a coherent component's functioning has been grokked at an API/surface level
then the human can use the component without caring about its innards. This
is much harder to do in tightly coupled codebases.

---

title: Increasing coherence, decreasing coupling

# Increase one, decrease the other

- Prototype

???

It's very common when you start coding to begin by prototyping something. That
code will usually be quite tightly coupled, often very hard to test, but it is
good enough to prove the point of your intent.

This is the starting point

---

title: Increasing coherence, decreasing coupling

# Increase one, decrease the other

- Prototype
- Architect

???

At this point you step back from your code and you think about the structure
of your prototype and look for logical units which could be extracted from
the prototype's behaviour and turned into coherent testable code. If you're
lucky this might be suitable for extracting into a re-usable crate, but failing
that, into a module of its own.

This process is one way to consider the work of a 'software architect'.
Decomposing complex systems into coherent components is a critical function of
such a person. Most programmers do this at some level, but experience here
will be useful as you progress.

---

title: Increasing coherence, decreasing coupling

# Increase one, decrease the other

- Prototype
- Architect
- Decompose

???

At this point, you can take the components you've identified and create
implementations which are independent of your prototype. You may have to
define several additional APIs in order to make that happen, but that's all
part of the cycle of architecture and decomposition. Each time you do this
you reduce the coupling and increase the coherence of your codebase.

Each decomposed piece should be as tested as it can be, have good
documentation, well thought out APIs, and good separation of concerns.

You may be able to use the dreaded 'r' word here and 'refactor' your prototype
but only if you wrote strong tests back when you prototyped.

---

title: Increasing coherence, decreasing coupling

# Increase one, decrease the other

- Prototype
- Architect
- Decompose
- Arrange

???

The final step in the loop of improving your code is to arrange the code into
coherent modules. At this point another term 'cohesion' comes in a bit
because a nicely coherent module and a properly cohesive module are similar in
concept.

Basically put things which should be together together, things which
might not belong together apart. It should be easy for a user to find things
in your crate. If everything is lumped into one module then it can be daunting
to discover things. If you have a clear "fs" module and an associated "io"
module then the user can usually find the right thing after gaining a feel for
your crate design. Good arrangement improves discoverability of your crate
functionality.

---

class: impact

## Bringing this to Rust code

---

title: Ways Rust can help you with this

# Doing this in Rust

- API horizons

???

When I think of how to decompose code into functional blocks, I think of API
horizons. For me, an API horizon is a way to limit how much a user of a piece
of code can see and reason about the interior of the code block. For example,
in Rust, the API horizon of a struct is any (if there are any) public members,
and public associated functions, types, and other items. This extends to any
traits which that type implements too (including blanket traits).

A really simple way to construct strong API horizons is to have a small public
API, and isolate all the rest behind a crate boundary.

---

title: Ways Rust can help you with this

# Doing this in Rust

- API horizons
- Item visibility

???

In Rust, many items can be given visibility modifiers. These can be set on
structs, enums, functions, traits, types, modules, etc. By default all these
are "not public" (which I'll refer to for now as 'private', however you can
modify that. Before we talk about other visibility levels though, let's
consider what 'private' actually means here.

---

title: Ways Rust can help you with this

# Doing this in Rust

- API horizons
- Item visibility
  - Not public (private)

???

When an item is private, it means that it is only accessible to the module
it is defined in, and to any _child_ modules of that module. What this means
is that when you have (as is quite common) a test submodule of your module
it can access the private items in its parent module (allowing it to unit-test
code which would not be accessible to the outside).

This also allows a crate to contain some items at the top level which are
available to all its modules, but not exported to the users of the crate.

---

title: Ways Rust can help you with this

# Doing this in Rust

- API horizons
- Item visibility
  - Not public (private)
  - Good old public (`pub`)

???

When an item is labelled `pub` this sets its visibility to public. This means
that _any_ piece of code which is capable of finding a path to this item will
be able to access it. The subtleties here are important. If a public item
is part of a submodule in a crate, but that submodule is not public, then
no external user of that crate will be able to access it, even though the item
itself is labelled `pub`. The visibility modifier on an item is the _maximum_
visibility that the item could have.

If you can path to the item, then you get full access to it. This can mean
being able to see its public members, invoke public functions on it, etc.

Obviously any functionality you want a user to
directly consume should be public. Rust idiomatically doesn't expose members
of a struct but instead encourages trivial public accessor methods which
compile away in the release mode. This allows for the internal behaviour of
a struct to be changed between releases and so long as the public accessors
can remain implemented in some fashion, the API has not changed.

---

title: Ways Rust can help you with this

# Doing this in Rust

- API horizons
- Item visibility
  - Not public (private)
  - Good old public (`pub`)
  - Restricted public access
    - `pub (crate)`
    - `pub (super)`
    - `pub (self)`
    - `pub (in some::path)`

???

There are some special forms of `pub` which provide a more restricted variant
of public access. Since sometimes a user of a crate might be able to path
to a module which contains implementation details which need to be `pub` in
some sense in order to be useful inside the crate, but which should not be
exposed outside of the crate, there exists some restricted forms of `pub`.

These are, in order, public within the crate (but not exposed to users of the
crate), public within the super-module of this module (remember, private means
this and descendents), public within this module (kinda odd, but here for
consistency), and all three of those are effectively shortcuts for the final
one which is public within a specified path which **must** be part of the
ancestry path of the current module. (can use `self` and `super`)

---

title: Ways Rust can help you with this

# Doing this in Rust

- API horizons
- Item visibility
  - Not public (private)
  - Good old public (`pub`)
  - Restricted public access
    - `pub (crate)`
    - `pub (super)`
    - `pub (self)`
    - `pub (in some::path)`
- Using those in practice

???

Let's imagine a library crate.

Arranging your crate into nice modules is important because it helps you to
design coherent code as discussed before. However you want to make it possible
for your sibling modules to gain access to your functionality so you might use
one of the restricted public access markers. For example, you might have a
module `throbber` which manages things which throb. It might have a submodule
called `core` which concerns itself only with core throbbing functionality.
Since `core` is a submodule of `throbber` it would need to declare content
as public in order to allow `throbber` to use it. But if `throbber` were a
public module, we might want to ensure that we don't accidentally re-export
anything from `core` so we mark it `pub (crate)` or `pub (super)`.

That way, even if a user of the crate could try to _path_ to something like
`throbber::CoreThingy` they wouldn't be able to actually do it because while
that item exists, the _maximal_ visibility was `pub (crate)`.

---

title: Structs vs. Traits

- Structs (and enums) are concrete types.

???

When we design APIs, we can often fall into the hole of defining only structs
and enums. They're nice concrete types which mean that the compiler can reason
quickly and efficiently to know our code is good. Unfortunately this can lead
to very tightly coupled code. For example, if you had a struct which allowed
for filesystem access, and you had a module which managed a database stored
on disk, you might define methods which consume the one from the other...

---

title: Structs vs. Traits

```rust
impl MyDB {
    pub fn new(location: FileSystem::Entry) -> MyDB { ... }
}
```

???

Here we say that to construct a database we take an entry into our filesystem
and return a database at that entry. It's not the greatest of examples, but
it'll do.

Now imagine how you might document this in a way that `cargo` can run a
doctest. Or perhaps imagine how you might write a more complex test suite
without ending up with databases stomping all over one another, without using
random names etc.

You quickly wish that you'd abstracted the filesystem a bit.

---

title: Structs vs. Traits

- Abstractions can be done at the struct level

???

You could write a single `FileSystem` struct which supports both talking to the
real filesystem, or being ephemeral (in memory perhaps). You could make it so
that your real filesystem implementation can be rooted at a fixed subdir which
would permit you to have multiple instances for testing, but that kind of code
rapidly becomes very hard to deal with as it tends to be full of conditional code.

---

title: Structs vs. Traits

- Using a trait
  - Monomorphisation
  - Trait objects (Dynamic dispatch)

???

Instead you could define a `FileSystem` trait which allows you to take any
kind of filesystem into your database constructor and work from there. If you
parameterise your database struct by the filesystem implementation then you
will end up monomorphising it to however many filesystem implementations there
exist. If the trait is public and your crate users can define their own useful
filesystem implementations, that could be very powerful and yet compile down to
exactly the code it needs to.

Or you could instead take a trait object - that is either a borrow of a `dyn Trait`
or a `Box<dyn Trait>`, and you have a single set of code which uses dynamic
dispatch to talk to however many filesystem implementations there end up being.

This is faster to compile because it only need compile your database once
rather than once per type of filesystem used with it, but at runtime you pay
the dynamic dispatch penalty.

Which of the two approaches is appropriate will depend on your
particular work and I can't recommend one over the other at this time.

---

class: impact

# Semantic Versioning

---

title: Semantic versioning

- What is it?

???

Semantic versioning is, I hope you're all aware, a way to provide a
version number for a library, program, or whatever, where the difference
between two version numbers provides semantic information as to the scope of
the difference between the versions of the software itself.

Semantic versioning is typically split into 3 parts - major, minor, and patch. In the general
case, patch number increments mean bugfixes, minor number increments mean
additional functionality (and maybe bugfixes), major number increments mean
removed or fundamentally changed functionality (and maybe additional
functionality, and maybe bugfixes).

---

title: Semantic versioning

- What is it?
- Is the cornerstone of your users trust in being able to upgrade

???

With Semantic Versioning, a user of your crate can confidently upgrade a
dependency if only minor or patch levels change, and cargo uses this to
automatically provide updated deps if it can. If the major number changes,
then the user is immediately aware that they may have to (a) review the
changes to see if any of them affect their use of your crate, and (b) may have
to amend **their** code in order to be able to upgrade effectively.

If the major is zero, then the minor is treated as major, and the patch as
minor. If the major _and_ minor are zero, then the patch is considered major.

Given that, you can release "0.1.x" where x increases over time and users can
be confident that they can upgrade, but "0.2" is a potentially breaking API
change and users will have to be careful. Ditto if you release "0.0.x" then
every increment of x is a potential API break and users will have to be careful

---

title: Semantic versioning

- What is it?
- Is the cornerstone of your users trust in being able to upgrade
- Must be foremost in your thinking when making changes

???

When you make changes to your code, you therefore must be thinking about
whether or not your change is a breaking API change. Does it make an existing
API behave differently under any circumstances? Does it remove an API which
other users might have been relying on? If so, it's a major version increment.

If your change only adds new API then it's a minor increment.

If it only fixes bugs, then it's a patch level increment.

Given this, you need to be super-careful when arranging your publically visible
items. If a user could previously reach a public item then moving it such that
it's no longer accessible at that path is a breaking change. Making it no
longer public is a breaking change. Thinking very carefully about your public
API surface is critical. One nice trick here is to build the docs for your
crate and then look at them, if there's anything you weren't expecting to be
public, go back and check it and see if it can be hidden from the users of your
crate via `pub (crate)` or similar.

---

title: Homework

- Coherence and coupling
  - wikipedia and others
- Item visibility
  - New Rustacean episode 30
  - `rustup doc --reference` chapter 12.6 (Item visibility)
- Semantic versioning
  - `https://semver.org/`
  - `https://docs.rs/semver/`

No fixed programming homework, either complete/clean up your `bft` project
or have a go at something else.

???

Go and learn about coherence and coupling. There's a similar term 'cohesion'
which you may also see and is related. This is all a very loose thing because
what I understand as coherence is not exactly what you might find, but it's
all good and you should learn about it and where the balance lies for different
kinds of code.

For item visibility, there's a really good podcast episode of New Rustacean (
episode 30) and there's also chapter 6.16 of the rust _reference_

For semantic versioning, the specification is excellent and you should read it.
The Rust crate `semver` is also useful since it helps you to understand how to
tell if one version _satisfies_ a version dependency limit as you might use
in a `Cargo.toml`.

---

count: false
class: impact

# Any questions?

???

Obvious any-questionsness
