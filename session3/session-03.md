title: Rust Workshop - Session 3
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
  - Take a step back from code and consider everything around it
  - Goal is well-rounded and good habits from the start

---

title:
class: middle

# Topics

1. Crates - binaries vs libraries
2. Crates - Finding and depending on libraries
3. Documentation - writing it
4. Tests - writing them
5. CI habits

???

I'll also be encouraging you all to actually work hard to write Rusty code. It
seems awkward now, but getting into the habit of common Rust idioms will make
your code easier to deal with long term.

---

title: Crates - Libraries vs. Binaries

```shell
$ cargo init --bin mybinary
$ cargo init --lib mylibrary
```

???

Rust calls its primary grouping of code the `crate`. This could be
a number of kinds of library, and/or some number of executable programs.
By default, a `lib` crate is a single library, and a `bin` crate is a single
program, but that's simply the default shape of the template crates.

---

title: Layout of a cargo project

```
  + Cargo.toml
  + Cargo.lock (typically not for pure libraries)
  + src
  |  + main.rs (for binaries)
  |  \ lib.rs  (for libraries)
  + tests      (integration tests)
  \ target     (created by cargo for building in)
```

???

Every crate needs a `Cargo.toml` which is, as we have said before, the core
of the crate. It's how Cargo knows what you're building, what it depends on,
etc. In addition, publication data is present here for if you're pushing your
crate to `crates.io`.

The lock file is only really for binaries - libraries don't get to lock very
specific versions (excepting via the semver of their deps). There is some
discussion around this at the moment, but for now, this is the case.

Libraries start at `src/lib.rs` typically, and binaries at `src/main.rs` though
there are ways around this.

While unit tests are _inside_ the crate code, integration tests get to live in
a separate `tests` directory.

The `target` directory is made by cargo, it'll be git-ignored by default and
should be considered throw-away.

---

title: Cargo Workspaces

```toml
[workspace]
members = [
  "sub-crate-1",
  "sub-crate-2",
  # ...
]
```

???

Cargo supports the concept of workspaces which are where a crate (or just a
directory) can have other crates stored inside it. This is commonly done for
composite libraries or when you're writing a complex program with one or more
internal libraries.

We'll be using this in our homework (though I'll walk you through it step by
step)

With a workspace in place you can run things like `cargo build --all` to get
all crates built, even if they're not direct dependencies of one another.

---

title: Finding and depending on libraries

- Sometimes you don't want to write code because there'll be a library already
  written which will do what you need.
- But it can be a pain to find and to know what you then need to do to use it.
- There are some tricks to using `crates.io` and `docs.rs` to work it all out.
- Cue a demonstration...

???

For this case, we're going to go through the process of learning how to do a
very simple curl type program, finding external crates, and using them to great
effect.

Find `reqwest` and use the blocking API to fetch a URL and print it out.

---

title: Documenting a crate

- For now we'll only consider library crates
- The compiler can help us to know if we've written docs
- And we can make our docs part of our tests

???

We're going to do another live-coding demonstration here. Make a new library
crate which has a struct `Accumulator` which is parameterised on the number type
to accumulate. We're going to document the lot including writing some simple
tests in the docs.

This will need us to write useful trait bounds etc.

---

title: Tests - writing them

- Cargo automatically finds your doc tests and runs them
- But it can also find test code inside your library and run those
- And it can also run integration tests

???

Tests within your library have access to non-public functions and so this is
a good place to add tests for things which aren't necessarily public. We'll
write a test module and run the tests.

Then we'll write a very simple integration test, noting how even the
integration test code uses the `#[test]` attribute as well.

---

title: CI Habits

- Always run all the tests
- Generating the documentation for `main`
- Ensuring the formatting of your code
- Using clippy to be meaner to yourself

???

An obvious thing to do in CI is to run all your tests by means of `cargo test`
but there're also other things worth doing. For example, I'd suggest having a
pipeline step which runs `cargo doc --all` and publishes the docs when commits
are merged to `main`.

You can use `cargo fmt --all -- --check` in your pipeline to refuse commits
which aren't well formatted.

We mentioned clippy a while back, but we can also use that if we want. If we
do, then I suggest it's a permitted-fail, because clippy can be very pedantic
in the short-term and we don't want to be limited in our coding because of it.

---

title: Putting it all together

- A good library crate will have all of its public members documented.
- Ideally it'll have all its private members documented too, though that is
  much less strict
- Any good crate will have tests, some unit, maybe some doc, and some
  integration.
- A good crate will be consistently and cleanly formatted, and ideally be
  warning, if not lint, free.

???

These are some reasonable rules for you to try and stick to when you do your
homework from now on. This week's homework will finally be persistent in terms
of the code you write as well as what you're learning.

---

title: Homework

Things for you to research for next week

- These common traits:

```rust
// Revisit and really understand the relationship and differences for:
std::clone::Clone && std::marker::Copy

std::fmt::{Debug, Display} // Refresh
std::cmp::{Eq, PartialEq, Ord, PartialOrd} // Refresh
std::default::Default // Refresh
```

- Go and properly grok the following:

```rust
std::process::{Command, Stdio}
```

- The rest will be emailed to you like last week.

---

count: false
class: impact

# Any questions?

???

Obvious any-questionsness
