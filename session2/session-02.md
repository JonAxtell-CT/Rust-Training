title: Rust Workshop - Session 2
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
  - Extend from last week's learning to encompass non-trivial data
  - Learn a bit more about those thing we might have encountered in
    our homework

---

title:
class: middle

# Topics

???

1. Double-check no further homework related questions
2. Structured data - `struct` and `enum`
3. Adding code to those - `impl`
4. Bringing in other behaviours - Traits

---

title:
class: impact

# Any questions on your homework?

???

Hopefully you're all good, but if there are any further questions, now is
the time to ask them.

---

title: Structuring data

The trivial data structure:

```rust
type PairOfNumbers = (i32, i32);

let foo: PairOfNumbers = (10, 17);

println!("The first number is {}, the second {}", foo.0, foo.1);
```

???

The most trivial data structure is the _tuple_ which is represented as a
number of comma-separated types in parentheses. The empty tuple is called _unit_
and you've seen that before as the type of statment expressions. It's a bit like
_void_ in C.

While we could structure all our data in tuples, life would become somewhat painful
and so there are two more "proper" data structuring mechanisms in Rust...

---

title: Structuring data

Two primary kinds of structured data:

```rust
struct MyStruct {
    field1: SomeType,
    field2: SomeOtherType,
    ...
}

enum MyEnum {
    UnitKind,
    TupleKind(SomeType, SomeOtherType),
    StructKind{ field1: SomeType, field2: SomeOtherType }
}
```

???

There are two primary types of structuring which Rust tends to use.

The first and
perhaps most similar to code you've seen in the past is the `struct`. The
struct is what we call a _product type_ since the set of values it could contain
is the product of the sets of values each of its fields.

The second, and the one which many newcomers to Rust struggle with is the `enum`
or _enumeration_. This is called a _sum type_ because its values are the sums
of the values of all of the variants in it.

---

title: structs

```rust
struct MyStruct {
    field1: SomeType,
    field2: SomeType,
    ...
}
```

???

The `struct` is the main kind of complex data type you're likely to end up
writing, since it closely resembles in purpose C style `struct`s and to some
extent Python style `class`es.

Like many things in Rust, `struct`s can be parameterised with additional types
to be used in the struct. While we won't be doing this for the near future,
it's useful to know how that looks for when you're looking at error messages
since it's very popular in the standard library

---

title: Parameterised structs

```rust
struct Rectangle<T> {
    top: T,
    left: T,
    width: T,
    height: T,
}
```

???

Here we see a struct with a single type parameter `T` which can be used to set
the types of the coordinate points of the rectangle.

---

title: Parameterised structs - An example

```rust
pub struct Vec<T> {
    len: usize,
    ...
}
```

???

A more useful example which you've perhaps seen after playing with your homework
is the `Vec<T>` type which is essentially a resizeable array of pretty much
any type whatsoever. The only real limits here are the available amount of
RAM and that `T` must be what Rust called `Sized` which means that at the point
of use, `T` must have an obvious and _single_ quantity of RAM it might consume.

---

title: Enumerations

```rust
enum MyEnum {
    UnitKind,
    TupleKind(SomeType, SomeOtherType),
    StructKind { field1: SomeType, field2: SomeOtherType }
}
```

???

I intend to spend a bit longer on enumerations than I did on structures.
You've actually already seen a number of `enum`s in use, such as `Option`
and `Result` though you perhaps only thought of them as structured data
at the time.

---

title: Unit-style enumerations

```rust
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
    EndLoop,
}
```

???

When none of the variants of an enumeration take any data (i.e. they are all
_unit kind variants_) then the whole enumeration is very similar to a C enum
in that each of the variants essentially has a number assigned, and the size
of the enumeration is of the order of some kind of integer.

---

title: Unit-style enumeration example

```rust
/// Enum to store the various types of errors that can cause
/// parsing an integer to fail.
pub enum IntErrorKind {
    /// Value being parsed is empty.
    ///
    /// Among other causes, this variant will be constructed when
    /// parsing an empty string.
    Empty,
    /// Contains an invalid digit.
    ///
    /// Among other causes, this variant will be constructed when
    /// parsing a string that contains a letter.
    InvalidDigit,
    /// Integer is too large to store in target integer type.
    Overflow,
    /// Integer is too small to store in target integer type.
    Underflow,
}
```

???

One unit-style enumeration we've already seen is from parsing strings
into various forms of integer. As you can see, each variant can have its
own documentation strings and as you get into writing more and more Rust, you
will likely want to do this as much as you can in your own code.

---

title: Tuple-style enumerations

```rust
enum TextSpan {
    Bold(String),
    Italic(String),
    Plain(String),
}
```

???

One place you'll find enumerations in Rust where you might find an explicit
`enum` and `struct` (and sometimes `union`) in C is in this kind of situation.

You've encountered an enumeration like this already, though it was parameterised
over one (or yes, two) types:

---

title: A parameterised enumeration - `Option<T>`

```rust
/// The `Option` type.
pub enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}
```

???

Here you can see the `Option<T>` type and you can see it's an enumeration
which is parameterised over one type `T` and it can have two variants. One
is a unit-kind variant `None` and the other a tuple style variant `Some(T)`

You can mix variant kinds in a single enumeration usefully.

---

title: Enumerations with struct kind variants

```rust
enum Shape {
    Circle { radius: usize },
    Rectangle { width: usize, height: usize }
}
```

???

While you _can_ have struct kind variants in an enumeration, this is much
less often done because there's more value in creating structs for what would
be in the struct-kind variant and then using that type in a tuple-kind variant
instead.

i.e. …

---

```rust
struct CircleShape { radius: usize }
struct RectangleShape { width: usize, height: usize }
enum Shape {
    Circle(CircleShape),
    Rectangle(RectangleShape)
}
```

???

For completeness I thought I'd show them here, though they are pretty rare in
day to day use of Rust. I tend to only use them if I really don't want to
add another struct in, and I want to be very clear about a number of values of
the same type in a tuple-kind variant.

---

title: Adding code to types
class: impact

## Enough types, what about code?

???

When we have a whole bunch of types in our code, there's value in adding code
to those types to allow us to group behaviour into the types a little like
traditional object oriented programming.

Even though I laboured the point before that Rust is more about behaviours and
much less like traditional OO programming, this is about as close as we get to
OO, and it's a useful starting point for grouping behaviour in simple cases.

---

title: implementing our way to success

```rust
impl MyStruct {
    ...
}
```

???

Rust calls the blocks of code where we add behaviour to things `impl` blocks.

I tend to read `impl` as _implements_ so this is the block of code which
implements `MyStruct` behaviours.

Within those blocks you might have functions...

---

title: creating new things - 1

```rust
struct Person {
    name: String,
    age: usize,
}
```

???

Let's consider a `Person` which we will give a name and age to.

---

title: creating new things - 2

```rust
impl Person {
    pub fn new(name: &str, age: usize) -> Self {
        Self {
            name: name.to_owned(),
            age,
        }
    }
}
```

???

Traditionally we create a function called `new` which creates a new instance
of the type we're implementing. These functions generally can't fail. As you
can see, the syntax to create a struct is simply `Structname { fields }` and
you can name expressions to put them in particular fields, or merely have
expressions which consist of a variable of a name matching a field.

You can also see that within an `impl` block we can use `Self` to refer to the
type we're implementing.

---

title: creating new things - 3

```rust
impl Person {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> usize {
        self.age
    }
}
```

???

By default, the fields of a `struct` are private to functions associated with
the `struct` by means of the `impl` block. Unless it is really sensible to
allow others to delve inside your structure, you shouldn't make the fields
public. Instead there is a technique of providing read-accessors by means of
methods of the same name as fields. This is allowed because the compiler can
pretty much always work out what you mean.

A subtlety worth knowing but not thinking about for now is that private members
are accessible to other non-associated functions/types within the same code module.
We'll worry about that much later in the course, so for now, treat privacy as
at the type level.

This `name()` method takes a read-only borrow of `self` and returns a read-only
borrow of the `name` field inside the `Person` instance.

Like last week, we're taking advantage of the fact that a borrow of a `String`
can be coerced into a borrow of a string slice (`&str`).

---

title: Using our Person

```rust
...

fn get_author() -> Person {
    Person::new("Daniel Silverstone", 42 /* For now. */)
}

fn main () {
    let person = get_author();

    // This will fail to compile
    println!("My {} year old author's name is: {}",
             person.age, person.name);

    // This won't.
    println!("My {} year old author's name is: {}",
             person.age(), person.name());
}
```

???

Here we create a new person object and store it into `person` before trying to
report the person's name.

---

title: Mutating people

```rust
impl Person {
    // …
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }
    // …
}
```

???

Notice how this method takes a mutable borrow of `self` instead of a read-only
one? This means that this method will be able to change the field values.

---

title: Happy Birthday Daniel?

```rust
...

fn main() {
    let person = get_author();

    person.have_birthday();

    println!("Now {} is {} years old", person.name(), person.age());
}
```

???

Will this compile?

---

title: Happy Birthday Daniel!

```rust
...

fn main() {
*   let mut person = get_author();

    person.have_birthday();

    println!("Now {} is {} years old", person.name(), person.age());
}
```

???

This will compile though. In order to call methods which take a mutable borrow
of something, we need to be able to mutate it ourselves. This means we either
need to own it mutably, or we need to own a mutable borrow which we can lend
to the method while it runs.

What if we want to debug people? Compare them?

---

title: Deriving traits

```rust
#[derive(Debug)]
struct Person {
    // …
}
```

???

The rust compiler is able to derive a number of standard traits providing it can
understand enough about the fields of your `struct` or variants of your `enum`.

For example, we can derive `Debug` providing all the fields are of types for
which `Debug` is already implemented. In our case `String` and `usize` are
definitely blessed with such implementations.

---

title: Traits as behaviour

```rust
trait Identifiable {
    fn identify(&self) -> &str;
}
```

???

Typically in object-oriented languages, we combine behaviour together using
combinations of classes. For example, we might have a generic `Animal` class
which defines a bunch of behaviour and then derive `Cat` from that, overriding
some and adding others. This is called _inheritance_.

In Rust, we do not do this. The only OO form available in Rust is _encapsulation_
i.e. putting something inside something else. Rust's mechanism for sharing
behavioural properties is the `trait`. For Rust, `struct`s and `enum`s are
**data** and `impl` blocks are **behaviour**. Traits are a way to declare a
set of behaviour which must be present for a type to claim that property.

Importantly, traits cannot define fields, only methods and other associated
functions and types.

---

title: Implementing Identifiable

```rust
impl Identifiable for Person {
    fn identify(&self) -> &str {
        self.name()
    }
}
```

???

To implement a trait for a type, you need an `impl` block `for` the type.

---

title: Using that usefully

```rust
fn greet(who: &impl Identifiable) {
    println!("Hello {}", who.identify());
}
```

???

Read this as: "A function, called `greet`, which takes as an argument, called
`who`, which is a borrow, of _something_, which `impl`ements the behavioural
trait called `Identifiable`.

Importantly, despite calling `greet` with a borrow of a `Person` it will never
be able to call `.name()` on `who`.

Note: While the compiler will currently allow you to elide the `impl` in that
function declaration but including it is
good practice since it helps a reader to know that they should be looking for
an `Identifiable` **trait** rather than a `struct` or `enum`.

---

title: Extra credit - the dirty `M` word

```rust
fn greet(who: &impl Identifiable) {
    println!("Hello {}", who.identify());
}
```

???

When Rust encounters an `impl TraitName` situation, this is a point where the
compiler does something perhaps unintuitive. Every time you call this function
the compiler will look at what type you call it with, and will create a new
copy of the function to work with that type.

This is called _monomorphization_ and you will encounter that term when you
look at more complex Rust documentation in the future, so it helps to be aware
of it.

This means that if you have a lot of generic functions of this form, your
program will get bigger and take longer to compile, purely because each time
the compiler will have to monomorphize the function for the given input types.

---

title: Extra credit - faster compilation

```rust
fn greet(who: &dyn Identifiable) {
    println!("Hello {}", who.identify());
}
```

???

If we read the previous function definition as taking a borrow of _something_ which
implements the `Identifiable` trait, we can read this definition as taking a borrow
of _anything_ which implements the `Identifiable` trait.

When you see the `dyn` keyword you can read it as "any" in most cases. However,
in this case, the important thing is that by switching from `impl Identifiable` to
`dyn Identifiable` we tell the compiler we only want one copy of this function,
so instead it should compile it to be able to decide at runtime how to find the
identify function. The compiler does this by passing what it calls a _fat pointer_
which consists of both a pointer to the value, and a pointer to a special construct
called a `vtable` which is a set of function pointers for the trait combination needed
to call the function.

This indirection through the vtable costs runtime performance, however it isn't that
much runtime performance so don't be afraid to use `dyn` if it makes sense.

---

title: Homework

Things for you to research for next week

- These common traits:

```rust
Clone + Copy // It's time to start understanding when we can copy data
std::iter::Iterator // Always worth re-reading
std::fmt::{Debug, Display} // Probably useful for your homework
std::default::Default // Definitely necessary for your homework
// And while you may not need them yet, start to learn:
std::cmp::{Eq, PartialEq, Ord, PartialOrd}
```

- We've played with `Vec<T>` so let's learn another data
  structure:

```rust
std::collections::HashMap
std::collections::hash_map::Entry
```

- The actual homework questions will be provided to you like last week.

---

count: false
class: impact

# Any questions?

???

Obvious any-questionsness
