Getting started with the Rust Programming Language, following
[TRPL](https://doc.rust-lang.org/book/second-edition/).

Personal notes:

Rust is a typed, compiled language with a strict compiler, and it tries to
catch most errors at compile time rather than at runtime.

Its main feature is ownership, which is a way to use scopes to release memory
without a Garbage Collector or the need for the developer to implicitly free
variables. This seems a good balance between, on one hand, giving the
possibility to developers to have (safe) control of memory handling without
needing to write much code, and on the other hand most of the time they won't
have to even think about it. Once again the goal is to catch most errors at
compile time.

The borrow checker can be extremely useful to catch race conditions and force
us to safely go around them.

So far the syntax looks very intuitive and pleasant to use. It has similarities
with many other languages, and I've noticed some elements close to Go.
I really like the `_` prefix in unused variable names and the `match` syntax.

On the first approach, Rust reminds me of Go, probably because they are both
quite recent languages, focus on concurrency and safe memory handling.
But they actually differ in many ways: Go will usually be simpler and more
straight-forward while Rust will give more options and be more performant. Go
has a GC, Rust has not. I do not think the two languages even compete as they
do not embrace the same philosophy and goals (Go => simplicity, Rust =>
performance with safety).

> [Rust is for replacing C/C++ with a safer alternative. Go is basically for
developers that never fell in love with Java or C# but need the same kind of
language.](https://news.ycombinator.com/item?id=13430538)
