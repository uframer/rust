# 基本类型

Rust语言中有一些类型被称作*基本类型*，它们内置于语言之中。Rust的标准库以这些基本类型为基础，提供了很多有用的类型。

# 布尔类型

Rust的内建布尔类型叫`bool`。它有两个可选值，`true`和`false`：

```rust
let x = true;

let y: bool = false;
```

布尔类型经常用于[`if`表达式的条件][if]中。

[if]: if.html

你可以在[标准库文档][bool]中找到更多关于`bool`的信息。

[bool]: ../std/primitive.bool.html

# `char`

`char`类型代表一个Unicode标量值。你可以用单引号（`'`）书写`char`类型的文字量：

```rust
let x = 'x';
let two_hearts = '💕';
```

不同于其他语言，这意味着Rust的`char`类型不是一个字节，而是4个字节。

你可以在[标准库的`char`文档][char]中找到更多信息。

[char]: ../std/primitive.char.html

# 数值类型

Rust的数值类型可以按照不同的标准划分为不同的类别：有符号和无符号、定长和变长、浮点数和整数。

These types consist of two parts: the category, and the size. For example,
`u16` is an unsigned type with sixteen bits of size. More bits lets you have
bigger numbers.

If a number literal has nothing to cause its type to be inferred, it defaults:

```rust
let x = 42; // `x` has type `i32`.

let y = 1.0; // `y` has type `f64`.
```

Here’s a list of the different numeric types, with links to their documentation
in the standard library:

* [i8](../std/primitive.i8.html)
* [i16](../std/primitive.i16.html)
* [i32](../std/primitive.i32.html)
* [i64](../std/primitive.i64.html)
* [u8](../std/primitive.u8.html)
* [u16](../std/primitive.u16.html)
* [u32](../std/primitive.u32.html)
* [u64](../std/primitive.u64.html)
* [isize](../std/primitive.isize.html)
* [usize](../std/primitive.usize.html)
* [f32](../std/primitive.f32.html)
* [f64](../std/primitive.f64.html)

Let’s go over them by category:

## Signed and Unsigned

Integer types come in two varieties: signed and unsigned. To understand the
difference, let’s consider a number with four bits of size. A signed, four-bit
number would let you store numbers from `-8` to `+7`. Signed numbers use
“two’s complement representation”. An unsigned four bit number, since it does
not need to store negatives, can store values from `0` to `+15`.

Unsigned types use a `u` for their category, and signed types use `i`. The `i`
is for ‘integer’. So `u8` is an eight-bit unsigned number, and `i8` is an
eight-bit signed number.

## Fixed-size types

Fixed-size types have a specific number of bits in their representation. Valid
bit sizes are `8`, `16`, `32`, and `64`. So, `u32` is an unsigned, 32-bit integer,
and `i64` is a signed, 64-bit integer.

## Variable-size types

Rust also provides types whose particular size depends on the underlying machine
architecture. Their range is sufficient to express the size of any collection, so
these types have ‘size’ as the category. They come in signed and unsigned varieties
which account for two types: `isize` and `usize`.

## Floating-point types

Rust also has two floating point types: `f32` and `f64`. These correspond to
IEEE-754 single and double precision numbers.

# 数组

Like many programming languages, Rust has list types to represent a sequence of
things. The most basic is the *array*, a fixed-size list of elements of the
same type. By default, arrays are immutable.

```rust
let a = [1, 2, 3]; // a: [i32; 3]
let mut m = [1, 2, 3]; // m: [i32; 3]
```

Arrays have type `[T; N]`. We’ll talk about this `T` notation [in the generics
section][generics]. The `N` is a compile-time constant, for the length of the
array.

There’s a shorthand for initializing each element of an array to the same
value. In this example, each element of `a` will be initialized to `0`:

```rust
let a = [0; 20]; // a: [i32; 20]
```

You can get the number of elements in an array `a` with `a.len()`:

```rust
let a = [1, 2, 3];

println!("a has {} elements", a.len());
```

You can access a particular element of an array with *subscript notation*:

```rust
let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]

println!("The second name is: {}", names[1]);
```

Subscripts start at zero, like in most programming languages, so the first name
is `names[0]` and the second name is `names[1]`. The above example prints
`The second name is: Brian`. If you try to use a subscript that is not in the
array, you will get an error: array access is bounds-checked at run-time. Such
errant access is the source of many bugs in other systems programming
languages.

You can find more documentation for `array`s [in the standard library
documentation][array].

[array]: ../std/primitive.array.html

# 切片

A ‘slice’ is a reference to (or “view” into) another data structure. They are
useful for allowing safe, efficient access to a portion of an array without
copying. For example, you might want to reference only one line of a file read
into memory. By nature, a slice is not created directly, but from an existing
variable binding. Slices have a defined length, and can be mutable or immutable.

Internally, slices are represented as a pointer to the beginning of the data
and a length.

## 切片语法

You can use a combo of `&` and `[]` to create a slice from various things. The
`&` indicates that slices are similar to [references], which we will cover in
detail later in this section. The `[]`s, with a range, let you define the
length of the slice:

```rust
let a = [0, 1, 2, 3, 4];
let complete = &a[..]; // A slice containing all of the elements in `a`.
let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
```

Slices have type `&[T]`. We’ll talk about that `T` when we cover
[generics][generics].

[generics]: generics.html

You can find more documentation for slices [in the standard library
documentation][slice].

[slice]: ../std/primitive.slice.html

# `str`

Rust’s `str` type is the most primitive string type. As an [unsized type][dst],
it’s not very useful by itself, but becomes useful when placed behind a
reference, like `&str`. We'll elaborate further when we cover
[Strings][strings] and [references].

[dst]: unsized-types.html
[strings]: strings.html
[references]: references-and-borrowing.html

You can find more documentation for `str` [in the standard library
documentation][str].

[str]: ../std/primitive.str.html

# 元组

A tuple is an ordered list of fixed size. Like this:

```rust
let x = (1, "hello");
```

The parentheses and commas form this two-length tuple. Here’s the same code, but
with the type annotated:

```rust
let x: (i32, &str) = (1, "hello");
```

As you can see, the type of a tuple looks like the tuple, but with each
position having a type name rather than the value. Careful readers will also
note that tuples are heterogeneous: we have an `i32` and a `&str` in this tuple.
In systems programming languages, strings are a bit more complex than in other
languages. For now, read `&str` as a *string slice*, and we’ll learn more
soon.

You can assign one tuple into another, if they have the same contained types
and [arity]. Tuples have the same arity when they have the same length.

[arity]: glossary.html#arity

```rust
let mut x = (1, 2); // x: (i32, i32)
let y = (2, 3); // y: (i32, i32)

x = y;
```

You can access the fields in a tuple through a *destructuring let*. Here’s
an example:

```rust
let (x, y, z) = (1, 2, 3);

println!("x is {}", x);
```

Remember [before][let] when I said the left-hand side of a `let` statement was more
powerful than assigning a binding? Here we are. We can put a pattern on
the left-hand side of the `let`, and if it matches up to the right-hand side,
we can assign multiple bindings at once. In this case, `let` “destructures”
or “breaks up” the tuple, and assigns the bits to three bindings.

[let]: variable-bindings.html

This pattern is very powerful, and we’ll see it repeated more later.

You can disambiguate a single-element tuple from a value in parentheses with a
comma:

```rust
(0,); // A single-element tuple.
(0); // A zero in parentheses.
```

## 元组索引

You can also access fields of a tuple with indexing syntax:


```rust
let tuple = (1, 2, 3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);
```

Like array indexing, it starts at zero, but unlike array indexing, it uses a
`.`, rather than `[]`s.

You can find more documentation for tuples [in the standard library
documentation][tuple].

[tuple]: ../std/primitive.tuple.html

# 函数

Functions also have a type! They look like this:

```rust
fn foo(x: i32) -> i32 { x }

let x: fn(i32) -> i32 = foo;
```

In this case, `x` is a ‘function pointer’ to a function that takes an `i32` and
returns an `i32`.
