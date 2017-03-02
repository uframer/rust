# 引用和借用

本章是Rust所有权系统介绍的第二部分。所有权系统是Rust是最有特点和令人印象深刻的特性，每一个Rust开发者都应该熟悉这一系统。Rust依赖所有权系统来实现它最主要的目标——内存安全。所有权系统由几个概念构成，每个概念都会独立介绍：

* [所有权][ownership]，核心概念
* 借用，也就是本文
* [生命周期][lifetimes]，借用的高级用法

这三个部分相互关联，你需要按顺序阅读完才能理解所有权系统。

[ownership]: ownership.html
[lifetimes]: lifetimes.html

# 导语

在开始了解细节之前，我们需要知道两件重要的事情。

1. Rust主要关注安全性和速度，它通过坚持“零代价抽象”这一理念来实现这些目标。所有权系统就是*零代价抽象*理念的重要体现。本文中我们谈及的所有抽象都*存留于编译时*，你不需要在运行时为这些抽象付出任何代价。

2. 所有权系统有另一种很高昂的代价：陡峭的学习曲线。对此我们的建议是，多联系，越用越熟。

# Borrow语义

在[所有权][ownership]这一章的结尾，我们见识了一个恶心的函数写法：

```rust
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // 使用`v1`和`v2`。

    // 返回函数的结果，同时交还`v1`和`v2`的所有权。
    (v1, v2, 42)
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let (v1, v2, answer) = foo(v1, v2);
```

但是，这不是地道的Rust写法，地道的Rust代码会使用*借用*。我们先改一版看看：

```rust
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // 使用`v1`和`v2`。

    // 返回函数的结果。
    42
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let answer = foo(&v1, &v2);

// 现在在这里就可以使用`v1`和`v2`！
```

再看一个更具体的例子：

```rust
fn main() {
    // 你看不懂`fold`是干什么的也没关系，这里的重点是借用了一个不可变的引用。
    fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a + b);
    }
    // 借用两个向量并相加。
    // 这种类型的借用不允许通过借用的引用改变对象。
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // 使用`v1`和`v2`。
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);
        // 返回答案。
        s1 + s2
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);
}
```

在上面的例子中，我们用`&Vec<i32>`而不是`Vec<i32>`作为参数的类型。调用函数时，参数我们也改用`&v1`和`&v2`。我们把`&T`类型称之为一个*引用*，医用不会*拥有*资源，它只会*借用*所有权。如果一个绑定借用了一个资源，那么当这个绑定脱离作用域时，不会释放这个资源。这意味着当`foo`函数返回后，我们就可以继续使用`v1`和`v2`了。

同绑定一样，引用默认是不可变的。也就是说，在`foo`内部你不能修改`v1`和`v2`。假如你尝试修改：

```rust,ignore
fn foo(v: &Vec<i32>) {
     v.push(5);
}

let v = vec![];

foo(&v);
```

就会收到编译错误：

```text
error: cannot borrow immutable borrowed content `*v` as mutable
v.push(5);
^
```

`push`方法会修改向量，所以我们不允许调用它。（🐷：如何标记一个方法会修改对象？）

# `&mut`引用

如果我们确实要修改引用的内容，那么就要用到*可变引用*：`&mut T`。你可以通过可变引用修改借来的对象。例如：

```rust
let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);
```

上面的代码会输出`6`。注意，`x`也被标记为`mut`，否则会有编译错误，我们不能对一个不可变的对象做*可变借用*。此外，我们用`*y`语法来访问引用的内容。你可能还注意到我们用`{`和`}`定义了额外的作用域，如果我们将这个作用域移除，就会遇到编译错误：

```text
error: cannot borrow `x` as immutable because it is also borrowed as mutable
    println!("{}", x);
                   ^
note: previous borrow of `x` occurs here; the mutable borrow prevents
subsequent moves, borrows, or modification of `x` until the borrow ends
        let y = &mut x;
                     ^
note: previous borrow ends here
fn main() {

}
^
```

这些错误信息说明，我们需要遵守一些*规则*。

# 借用的规则

Rust中的借用需要遵守如下规则：

1. 任何borrow引用的作用域都不能超过所有者的作用域；
2. 以下两种情形不能共存：
  2.1. 存在对资源的一个或多个引用（`&T`）
  2.2. 有且只有一个可变引用（`&mut T`）

你可能已经注意到，这些规则十分类似于*数据竞争*的定义：

> 如果同时满足下面的条件，就会出现数据竞争：
> 1. 两个或多个指针访问在同一时刻访问同一个内存地址；
> 2. 其中至少有一个访问是写操作；
> 3. 这些操作没有同步机制。

如果所有的访问都是读操作，你可以同时拥有任意多个引用。不过，由于我们在同一时刻至多允许一个`&mut`可变引用，就可能引起数据竞争。前面的规则就是Rust用来在编译时避免数据竞争的机制：如果你违反规则，就编译不过。

记好这些规则，我们再回头看看前面的例子。

## 正确理解作用域

先看代码：

```rust,ignore
fn main() {
    let mut x = 5;
    let y = &mut x;

    *y += 1;

    println!("{}", x);
}
```

这段代码的编译错误如下：

```text
error: cannot borrow `x` as immutable because it is also borrowed as mutable
    println!("{}", x);
                   ^
```

这个编译错误的原因是我们违反了前面提到的规则：我们有一个`&mut T`类型的可变引用指向`x`，所以我们就不能再创建任何`&T`类型的只读引用了。这两种情形只能存在一种。下面的note给出了如何处理问题的提示信息：

```text
note: previous borrow ends here
fn main() {

}
^
```

换句话说，前面的可变借用一直延续到`main()`的结尾。What
we want is for the mutable borrow by `y` to end so that the resource can be
returned to the owner, `x`. `x` can then provide an immutable borrow to `println!`.
In Rust, borrowing is tied to the scope that the borrow is valid for. And our
scopes look like this:

```rust,ignore
fn main() {
    let mut x = 5;

    let y = &mut x;    // -+ &mut从这里开始借用`x`
                       //  |
    *y += 1;           //  |
                       //  |
    println!("{}", x); // -+ - 在这里试图借用`x`
}                      // -+ &mut对`x`的借用到此为止

```

The scopes conflict: we can’t make an `&x` while `y` is in scope.

So when we add the curly braces:

```rust
let mut x = 5;

{
    let y = &mut x; // -+ &mut borrow starts here.
    *y += 1;        //  |
}                   // -+ ... and ends here.

println!("{}", x);  // <- Try to borrow `x` here.
```

There’s no problem. Our mutable borrow goes out of scope before we create an
immutable one. So scope is the key to seeing how long a borrow lasts for.

## Issues borrowing prevents

Why have these restrictive rules? Well, as we noted, these rules prevent data
races. What kinds of issues do data races cause? Here are a few.

### Iterator invalidation

One example is ‘iterator invalidation’, which happens when you try to mutate a
collection that you’re iterating over. Rust’s borrow checker prevents this from
happening:

```rust
let mut v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
}
```

This prints out one through three. As we iterate through the vector, we’re
only given references to the elements. And `v` is itself borrowed as immutable,
which means we can’t change it while we’re iterating:

```rust,ignore
let mut v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
    v.push(34);
}
```

Here’s the error:

```text
error: cannot borrow `v` as mutable because it is also borrowed as immutable
    v.push(34);
    ^
note: previous borrow of `v` occurs here; the immutable borrow prevents
subsequent moves or mutable borrows of `v` until the borrow ends
for i in &v {
          ^
note: previous borrow ends here
for i in &v {
    println!(“{}”, i);
    v.push(34);
}
^
```

We can’t modify `v` because it’s borrowed by the loop.

### Use after free

References must not live longer than the resource they refer to. Rust will
check the scopes of your references to ensure that this is true.

If Rust didn’t check this property, we could accidentally use a reference
which was invalid. For example:

```rust,ignore
let y: &i32;
{
    let x = 5;
    y = &x;
}

println!("{}", y);
```

We get this error:

```text
error: `x` does not live long enough
    y = &x;
         ^
note: reference must be valid for the block suffix following statement 0 at
2:16...
let y: &i32;
{
    let x = 5;
    y = &x;
}

note: ...but borrowed value is only valid for the block suffix following
statement 0 at 4:18
    let x = 5;
    y = &x;
}
```

In other words, `y` is only valid for the scope where `x` exists. As soon as
`x` goes away, it becomes invalid to refer to it. As such, the error says that
the borrow ‘doesn’t live long enough’ because it’s not valid for the right
amount of time.

The same problem occurs when the reference is declared _before_ the variable it
refers to. This is because resources within the same scope are freed in the
opposite order they were declared:

```rust,ignore
let y: &i32;
let x = 5;
y = &x;

println!("{}", y);
```

We get this error:

```text
error: `x` does not live long enough
y = &x;
     ^
note: reference must be valid for the block suffix following statement 0 at
2:16...
    let y: &i32;
    let x = 5;
    y = &x;

    println!("{}", y);
}

note: ...but borrowed value is only valid for the block suffix following
statement 1 at 3:14
    let x = 5;
    y = &x;

    println!("{}", y);
}
```

In the above example, `y` is declared before `x`, meaning that `y` lives longer
than `x`, which is not allowed.
