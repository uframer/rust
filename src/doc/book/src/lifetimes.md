# 生命周期

本文是介绍Rust所有权系统的三篇文章中的第一篇。所有权系统是Rust是最有特点和令人印象深刻的特性，每一个Rust开发者都应该熟悉这一系统。Rust依赖所有权系统来实现它最主要的目标——内存安全。所有权系统由几个概念构成，每个概念都会独立介绍：

* [所有权][ownership]，核心概念
* [借用][borrowing]，以及相关的*引用*概念
* 生命周期，也就是本文

这三个部分相互关联，你需要按顺序阅读完才能理解所有权系统。

[ownership]: ownership.html
[borrowing]: references-and-borrowing.html

# 导语

在开始了解细节之前，我们需要知道两件重要的事情。

1. Rust主要关注安全性和速度，它通过坚持“零代价抽象”这一理念来实现这些目标。所有权系统就是“零代价抽象”理念的重要体现。本文中我们谈及的所有抽象都**存留于编译时**，你不需要在运行时为这些抽象付出任何代价。

2. 所有权系统有另一种很高昂的代价：陡峭的学习曲线。对此我们的建议是，多联系，越用越熟。

# 生命周期

把资源的引用借给别人可能会很麻烦（🐷：原文的表述很混乱，和下面的例子对不上）。例如，请考虑如下操作：

1. 我获得了某个资源的句柄。
2. 我借给你一份这个资源的引用。
3. 我决定我不再需要这个资源，然后就释放了它，此时你仍然拿着你对它的引用。
4. 你决定使用这个资源。

注意，这个时候你的引用已经指向了一个无效的资源！如果这个资源是内存的话，那么通常称这种情况为*悬挂指针*或者*释放后引用*。我们看一小段代码：

```rust,compile_fail
let r;              // 声明引用：`r`。
{
    let i = 1;      // 在新的作用域内声明变量：`i`。
    r = &i;         // 令`r`成为指向`i`的引用。
}                   // `i`脱离作用域并被释放。

println!("{}", r);  // `r`依然去引用`i`。
```

为了解决这个问题，我们必须要确保第四步操作不会在第三步操作之后发生。对于上面那个小例子，Rust编译器能够通过检查各个变量的生命周期发现这个问题。

如果函数的参数是引用，那么情况就会更为复杂。请考虑如下的例子：

```rust,compile_fail,E0106
fn skip_prefix(line: &str, prefix: &str) -> &str {
    // ...
#   line
}

let line = "lang:en=Hello World!";
let lang = "en";

let v;
{
    let p = format!("lang:{}=", lang);  // -+ 声明`p`。
    v = skip_prefix(line, p.as_str());  //  |
}                                       // -+ `p`脱离作用域。
println!("{}", v);
```

`skip_prefix`这个函数接受两个`&str`类型的引用作为参数，并且返回一个`&str`类型的引用。调用它时，我们传进去的两个引用分别是`line`和`p`：*这两个变量的生命周期是不同的*。现在，`println!`调用的安全性就依赖于`skip_prefix`函数返回的引用会指向*依然活着*的`line`还是*已经被释放*的`p`。

由于编译器无法自己作出判断，所以它会给出编译错误。我们需要为编译器提供关于这些引用的生命周期的信息。下面的例子中明确地在函数声明中标出了生命周期：

```rust
fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
#   line
}
```

我们先看看有什么变化，一会儿再去研究语法。第一个变化是方法名后面添加了`<'a, 'b>`，引入了两个*生命周期参数*：`'a`和`'b`。第二个变化是，函数签名中的每个引用都添加了生命周期参数作为标签，注意这些标签是加在`&`之后的。这些标签的作用是把生命周期参数和引用关联起来。

做出这些修改后，编译器就能够推导出`skip_prefix`的返回值和参数`line`具有相同的生命周期，因此即使`p`已经脱离作用域，只要`v`还有效，那么就可以安全地使用这个返回值。

> 🐷：看到这里我有一个疑惑，这里的讨论只覆盖了返回值的生命周期同参数的生命周期相关联的情形，如果返回值的生命周期是独立的呢？例如在里面新分配了一个字符串并返回回来？

In addition to the compiler being able to validate the usage of `skip_prefix`
return value, it can also ensure that the implementation follows the contract
established by the function declaration. This is useful especially when you are
implementing traits that are introduced [later in the book][traits].

**Note** It's important to understand that lifetime annotations are
_descriptive_, not _prescriptive_. This means that how long a reference is valid
is determined by the code, not by the annotations. The annotations, however,
give information about lifetimes to the compiler that uses them to check the
validity of references. The compiler can do so without annotations in simple
cases, but needs the programmer's support in complex scenarios.

[traits]: traits.html

# 语法

The `'a` reads ‘the lifetime a’. Technically, every reference has some lifetime
associated with it, but the compiler lets you elide (i.e. omit, see
["Lifetime Elision"][lifetime-elision] below) them in common cases. Before we
get to that, though, let’s look at a short example with explicit lifetimes:

[lifetime-elision]: #lifetime-elision

```rust,ignore
fn bar<'a>(...)
```

We previously talked a little about [function syntax][functions], but we didn’t
discuss the `<>`s after a function’s name. A function can have ‘generic
parameters’ between the `<>`s, of which lifetimes are one kind. We’ll discuss
other kinds of generics [later in the book][generics], but for now, let’s
focus on the lifetimes aspect.

[functions]: functions.html
[generics]: generics.html

We use `<>` to declare our lifetimes. This says that `bar` has one lifetime,
`'a`. If we had two reference parameters with different lifetimes, it would
look like this:


```rust,ignore
fn bar<'a, 'b>(...)
```

Then in our parameter list, we use the lifetimes we’ve named:

```rust,ignore
...(x: &'a i32)
```

If we wanted a `&mut` reference, we’d do this:

```rust,ignore
...(x: &'a mut i32)
```

If you compare `&mut i32` to `&'a mut i32`, they’re the same, it’s that
the lifetime `'a` has snuck in between the `&` and the `mut i32`. We read `&mut
i32` as ‘a mutable reference to an `i32`’ and `&'a mut i32` as ‘a mutable
reference to an `i32` with the lifetime `'a`’.

# In `struct`s

You’ll also need explicit lifetimes when working with [`struct`][structs]s that
contain references:

```rust
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
    let f = Foo { x: y };

    println!("{}", f.x);
}
```

[structs]: structs.html

As you can see, `struct`s can also have lifetimes. In a similar way to functions,

```rust
struct Foo<'a> {
# x: &'a i32,
# }
```

declares a lifetime, and

```rust
# struct Foo<'a> {
x: &'a i32,
# }
```

uses it. So why do we need a lifetime here? We need to ensure that any reference
to a `Foo` cannot outlive the reference to an `i32` it contains.

## `impl` blocks

Let’s implement a method on `Foo`:

```rust
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}
```

As you can see, we need to declare a lifetime for `Foo` in the `impl` line. We repeat
`'a` twice, like on functions: `impl<'a>` defines a lifetime `'a`, and `Foo<'a>`
uses it.

## Multiple lifetimes

If you have multiple references, you can use the same lifetime multiple times:

```rust
fn x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str {
#    x
# }
```

This says that `x` and `y` both are alive for the same scope, and that the
return value is also alive for that scope. If you wanted `x` and `y` to have
different lifetimes, you can use multiple lifetime parameters:

```rust
fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
#    x
# }
```

In this example, `x` and `y` have different valid scopes, but the return value
has the same lifetime as `x`.

## Thinking in scopes

A way to think about lifetimes is to visualize the scope that a reference is
valid for. For example:

```rust
fn main() {
    let y = &5;     // -+ `y` comes into scope.
                    //  |
    // Stuff...     //  |
                    //  |
}                   // -+ `y` goes out of scope.
```

Adding in our `Foo`:

```rust
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5;           // -+ `y` comes into scope.
    let f = Foo { x: y }; // -+ `f` comes into scope.
                          //  |
    // Stuff...           //  |
                          //  |
}                         // -+ `f` and `y` go out of scope.
```

Our `f` lives within the scope of `y`, so everything works. What if it didn’t?
This code won’t work:

```rust,ignore
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let x;                    // -+ `x` comes into scope.
                              //  |
    {                         //  |
        let y = &5;           // ---+ `y` comes into scope.
        let f = Foo { x: y }; // ---+ `f` comes into scope.
        x = &f.x;             //  | | This causes an error.
    }                         // ---+ `f` and y go out of scope.
                              //  |
    println!("{}", x);        //  |
}                             // -+ `x` goes out of scope.
```

Whew! As you can see here, the scopes of `f` and `y` are smaller than the scope
of `x`. But when we do `x = &f.x`, we make `x` a reference to something that’s
about to go out of scope.

Named lifetimes are a way of giving these scopes a name. Giving something a
name is the first step towards being able to talk about it.

## 'static

The lifetime named ‘static’ is a special lifetime. It signals that something
has the lifetime of the entire program. Most Rust programmers first come across
`'static` when dealing with strings:

```rust
let x: &'static str = "Hello, world.";
```

String literals have the type `&'static str` because the reference is always
alive: they are baked into the data segment of the final binary. Another
example are globals:

```rust
static FOO: i32 = 5;
let x: &'static i32 = &FOO;
```

This adds an `i32` to the data segment of the binary, and `x` is a reference
to it.

## Lifetime Elision

Rust supports powerful local type inference in the bodies of functions, but it
deliberately does not perform any reasoning about types for item signatures.
However, for ergonomic reasons, a very restricted secondary inference algorithm called
“lifetime elision” does apply when judging lifetimes. Lifetime elision is concerned solely with inferring
lifetime parameters using three easily memorizable and unambiguous rules. This means lifetime elision
acts as a shorthand for writing an item signature, while not hiding
away the actual types involved as full local inference would if applied to it.

When talking about lifetime elision, we use the terms *input lifetime* and
*output lifetime*. An *input lifetime* is a lifetime associated with a parameter
of a function, and an *output lifetime* is a lifetime associated with the return
value of a function. For example, this function has an input lifetime:

```rust,ignore
fn foo<'a>(bar: &'a str)
```

This one has an output lifetime:

```rust,ignore
fn foo<'a>() -> &'a str
```

This one has a lifetime in both positions:

```rust,ignore
fn foo<'a>(bar: &'a str) -> &'a str
```

Here are the three rules:

* Each elided lifetime in a function’s arguments becomes a distinct lifetime
  parameter.

* If there is exactly one input lifetime, elided or not, that lifetime is
  assigned to all elided lifetimes in the return values of that function.

* If there are multiple input lifetimes, but one of them is `&self` or `&mut
  self`, the lifetime of `self` is assigned to all elided output lifetimes.

Otherwise, it is an error to elide an output lifetime.

### Examples

Here are some examples of functions with elided lifetimes.  We’ve paired each
example of an elided lifetime with its expanded form.

```rust,ignore
fn print(s: &str); // elided
fn print<'a>(s: &'a str); // expanded

fn debug(lvl: u32, s: &str); // elided
fn debug<'a>(lvl: u32, s: &'a str); // expanded
```

In the preceding example, `lvl` doesn’t need a lifetime because it’s not a
reference (`&`). Only things relating to references (such as a `struct`
which contains a reference) need lifetimes.

```rust,ignore
fn substr(s: &str, until: u32) -> &str; // elided
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded

fn get_str() -> &str; // ILLEGAL, no inputs

fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

fn get_mut(&mut self) -> &mut T; // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter; // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded
```
