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

由于编译器无法自己作出判断，所以它会给出编译错误。🐷：编译错误如下：

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:45
  |
1 | fn skip_prefix(line: &str, prefix: &str) -> &str {
  |                                             ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `line` or `prefix`
```

我们需要为编译器提供关于这些引用的生命周期的信息。下面的例子中明确地在函数声明中标出了生命周期：

```rust
fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
#   line
}
```

我们先看看有什么变化，一会儿再去研究语法。第一个变化是方法名后面添加了`<'a, 'b>`，引入了两个*生命周期参数*：`'a`和`'b`。第二个变化是，函数签名中的每个引用都添加了生命周期参数作为标签，注意这些标签是加在`&`之后的。这些标签的作用是把生命周期参数和引用关联起来。

做出这些修改后，编译器就能够推导出`skip_prefix`的返回值和参数`line`具有相同的生命周期，因此即使`p`已经脱离作用域，只要`v`还有效，那么就可以安全地使用这个返回值。

> 🐷：这里的讨论只覆盖了返回值的生命周期同参数的生命周期相关联的情形，如果返回值的生命周期是独立的呢？例如在里面新分配了一个字符串并返回回来？其实这个担心是多余的。由于函数的返回值类型是`&str`，是一个借用，那么它一定指向函数体作用域内能够访问的一个值。显然，这个值要么是两个参数之一，要么是内部分配的一个值。而按照Rust的定义，引用的作用域不能超出被引用值的作用域，而这个借用已经占用了返回值的位置，所以如果它引用的是一个内部分配的值的话，这个值是无法逃逸出函数体的作用域的，也就会造成返回值这个借用的生命周期比它引用的值还长，从而违反了上一节介绍的借用规则，所以这种情形是不可能存在的，返回值引用的一定是两个参数之一。

编译器除了能够验证`skip_prefix`返回值的用法之外，还能确保这个函数的实现符合函数声明确立的契约（contract）。如果你需要实现一个[特征][traits]的话，就需要编译器帮你检查其中的每个方法实现是否符合特征定义的接口要求。

**注意**：生命周期标记是*描述性**（descriptive）*的，不是*规定性**（prescriptive）*的。这意味着一个引用真正的生命周期有多长不是由这些标记定义的，而是依赖于具体的代码。不过，编译器使用这些标记来检查引用的有效性。在某些简单的场景下，编译器不用标记就可以自己推导出结论，但是如果场景比较复杂，就需要开发人员手动打上标记。

[traits]: traits.html

# 语法

`'a`表示‘生命周期a’。技术上，每个引用都有与其相关联的生命周期，但是在多数情形下编译器允许你省略它（参见后面的[省略生命周期][lifetime-elision]部分）。我们暂时不讨论那么深入，还是先看一个明确标注生命周期的小例子：

[lifetime-elision]: #lifetime-elision

```rust,ignore
fn bar<'a>(...)
```

我们前面介绍过一点儿[函数的语法][functions]，但是没有提过函数名后面的`<>`部分。`<>`里面包含的是函数的*泛型参数*，而生命周期就是其中一种。我们会在[泛型][generics]部分讨论其他种类的泛型参数，现在把重点放在生命周期上。

[functions]: functions.html
[generics]: generics.html

我们使用`<>`声明生命周期。上面的例子里，`bar`声明了一个生命周期——`'a`。如果我们有两个生命周期不同的函数参数，那么我们应该声明两个生命周期：


```rust,ignore
fn bar<'a, 'b>(...)
```

在参数列表中，我们需要标注前面声明的生命周期：

```rust,ignore
...(x: &'a i32)
```

如果我们的引用是可变的，那么要这样写：

```rust,ignore
...(x: &'a mut i32)
```

If you compare `&mut i32` to `&'a mut i32`, they’re the same, it’s that
the lifetime `'a` has snuck in between the `&` and the `mut i32`. We read `&mut
i32` as ‘a mutable reference to an `i32`’ and `&'a mut i32` as ‘a mutable
reference to an `i32` with the lifetime `'a`’.

# `struct`中的生命周期

如果[`struct`][structs]中包含引用的话，那么也需要明确地标出生命周期：

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

类似于函数，`struct`也可以带有生命周期：

```rust
struct Foo<'a> {
# x: &'a i32,
# }
```

声明并使用生命周期：

```rust
# struct Foo<'a> {
x: &'a i32,
# }
```

So why do we need a lifetime here? We need to ensure that any reference
to a `Foo` cannot outlive the reference to an `i32` it contains.

## `impl`块

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

## 标记多个生命周期

如果你有多个引用，并且它们的生命周期相同，你可以用同一个生命周期标签标记它们：

```rust
fn x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str {
#    x
# }
```

上面代码的意思是，`x`和`y`具有相同的生命周期，而且返回值和它们的生命周期也一样。如果`x`和`y`的生命周期不同，那么可以使用多个生命周期标记：

```rust
fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
#    x
# }
```

在上面的例子中，`x`和`y`具有不同的生命周期，返回值的生命周期同`x`相同。

## 从作用域的角度理解生命周期

如果我们将引用的有效作用域可视化，可能会帮助你理解生命周期的概念。例如：

```rust
fn main() {
    let y = &5;     // -+ `y`进入作用域。
                    //  |
    // Stuff...     //  |
                    //  |
}                   // -+ `y`脱离作用域。
```

Adding in our `Foo`:

```rust
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5;           // -+ `y`进入作用域。
    let f = Foo { x: y }; // -+ `f`进入作用域。
                          //  |
    // Stuff...           //  |
                          //  |
}                         // -+ `f`和`y`脱离作用域。
```

`f`和`y`同处一个作用域，所以一切OK。如果不是这样呢？下面的代码编译不过：

```rust,ignore
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let x;                    // -+ `x`进入作用域。
                              //  |
    {                         //  |
        let y = &5;           // ---+ `y`进入作用域。
        let f = Foo { x: y }; // ---+ `f`进入作用域。
        x = &f.x;             //  | | 这行代码会造成错误。
    }                         // ---+ `f`和`y`脱离作用域。
                              //  |
    println!("{}", x);        //  |
}                             // -+ `x`脱离作用域。
```

上面的例子中，`f`和`y`的作用域小于`x`的作用域。但是当我们写下`x = &f.x`时，`x`就引用了一个将要脱离作用域的资源。

生命周期标记给作用域起了一个名字。毕竟知道对方的名字才好打招呼嘛。

## `'static`

名为‘static’的生命周期是一个特殊的生命周期，它表示这个资源的生命周期是整个程序的生命周期。大多数Rust开发者是在使用字符串时第一次遇到`'static`的：

```rust
let x: &'static str = "Hello, world.";
```

字符串文字量的类型是`&'static str`，这个引用永远是可用的，因为这些文字量保存在二进制文件的数据段中。我们再看一个全局变量的例子：

```rust
static FOO: i32 = 5;
let x: &'static i32 = &FOO;
```

上面的代码会将一个`i32`类型的值添加到二进制文件的数据段中，然后令`x`引用它。

## *lifetime elision* 推断

Rust在函数体内支持本地类型推断，但是故意不去推断函数签名中的类型。不过，为了体恤开发者，Rust提供了一个名叫 *lifetime elision* 的生命周期推断算法。 *lifetime elision* 使用三条容易记忆并且清晰的规则来推断生命周期参数。

在讨论 *lifetime elision* 时，我们会用到两个术语： *输入生命周期* 和 *输出生命周期* 。所谓 *输入生命周期* 同函数参数相关联， *输出生命周期* 同函数的返回值相关联。例如，下面的函数标出了输入生命周期：

```rust,ignore
fn foo<'a>(bar: &'a str)
```

下面的函数标出了输出生命周期：

```rust,ignore
fn foo<'a>() -> &'a str
```

下面的函数两者兼有：

```rust,ignore
fn foo<'a>(bar: &'a str) -> &'a str
```

生命周期推断的三条规则包括：

* 为每个没有明确标出生命周期的函数参数创建一个独立的生命周期参数。
* 如果只有一个输入生命周期，无论开发者是否明确标出了它，都会用它作为函数返回值中所有未标明的引用的生命周期。
* 如果有多个输入生命周期，但是其中一个是`&self`或者`&mut self`（类似于其他语言中的`this`指针或者`self`指针），那么会把`self`用作函数返回值中所有未标明的引用的生命周期。

如果不能满足上面的规则，开发者就必须明确标出输出生命周期。

### 示例

下面给出一些省略函数生命周期的例子。在每一组例子中，我们同时给出了省略标记写法和对应的全标记形式。

```rust,ignore
fn print(s: &str); // 省略生命周期
fn print<'a>(s: &'a str); // 标记出所有生命周期

fn debug(lvl: u32, s: &str); // 省略生命周期
fn debug<'a>(lvl: u32, s: &'a str); // 标记出所有生命周期
```

前面例子中的`lvl`不需要标记生命周期，因为它不是一个引用（`&`）。只有同引用相关的资源（例如，包含引用的`struct`）才需要标记生命周期。

```rust,ignore
fn substr(s: &str, until: u32) -> &str; // 省略生命周期
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // 标记出所有生命周期

fn get_str() -> &str; // 非法！因为没有输入，返回值的引用没有可用的引用目标

fn frob(s: &str, t: &str) -> &str; // 非法！因为有两个输入，不知道输出应该引用哪一个输入
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // 标记出所有输入生命周期，没有标记出输出的生命周期，依然不能编译通过

fn get_mut(&mut self) -> &mut T; // 省略生命周期
fn get_mut<'a>(&'a mut self) -> &'a mut T; // 标记出所有生命周期

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // 省略生命周期
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // 标记出所有生命周期

fn new(buf: &mut [u8]) -> BufWriter; // 省略生命周期
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // 标记出所有生命周期
```
