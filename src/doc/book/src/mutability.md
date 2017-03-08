# 可变性

Rust语言中的可变性同其他语言有些不同。第一点是，Rust的绑定默认是不可变的：

```rust,ignore
let x = 5;
x = 6; // 错误！
```

我们通过`mut`关键字标记可变性：

```rust
let mut x = 5;

x = 6; // 没问题！
```

`x`是可变的[变量绑定][vb]。如果一个绑定是可变的，意味着你可以改变它指向哪个值。在上面的例子中，并不是原来`x`指向的值本身从`5`变成了`6`，而是`x`从指向`5`变为了指向`6`。

> 🐷：如果你是C/C++开发者，可能脑中的内存模型还在影响着你，认为`x = 6`改变的一定是`x`绑定的内存，那么我们看一下下面的例子：
>
> ```rust
> let x = &mut 5;
> *x = 6;
> println!("x is {}", *x);
> ```
>
> 这个例子是可以成功编译运行的：
>
> ```text
> x is 6
> ```
>
> 从上面的例子可以看出，Rust中的值是可以独立于绑定名存在的，当然编译器优化后另说。

[vb]: variable-bindings.html

如果想通过一个[引用][ref]`&x`修改`x`的话，至少先要保证这个引用是可变的：

```rust
let mut x = 5;
let y = &mut x;
```

[ref]: references-and-borrowing.html

`y`是一个不可变绑定，它绑定到对`x`的可变引用上。这意味着你不能再把`y`绑定到其他东西上，例如`y = &mut z`是不合法的；但是，你可以通过`y`把`x`绑定到其他东西上，例如`*y = 5`。这里面的差别需要细细体会。

当然，你也可以给`y`的绑定也加上`mut`：

```rust
let mut x = 5;
let mut y = &mut x;
```

现在`y`就可以绑定到别的值了，而且它所引用的值也可以改变。

需要注意，`mut`也是[模式][pattern]的一部分，所以你可以这样做：

```rust
let (mut x, y) = (5, 6);

fn foo(mut x: i32) {
# }
```

模式匹配后，`x`是可变的，`y`是不可变的。

[pattern]: patterns.html

# 内部可变性和外部可变性

当我们说Rust中的某个东西是*不可变*的时候，并不意味着它真的不能改变：我们说的是*外部可变性*意义上的不可变。请考虑下面[`Arc<T>`][arc]的例子：

```rust
use std::sync::Arc;

let x = Arc::new(5);
let y = x.clone();
```

[arc]: ../std/sync/struct.Arc.html

当我们调用`clone()`时，`Arc<T>`的实现需要更新内部的引用计数。尽管我们在这里没有标注`mut`，`x`是一个不可变的绑定，而且我们也没有通过类似`&mut 5`这样的引用直接修改值。那么到底为什么内容可以改变呢？

为了理解这个例子，我们需要回忆一下Rust的核心哲学——内存安全性，以及保证内存安全的核心机制——[所有权][ownership]系统，或者更具体的说，[借用][borrowing]的概念：

> 下面两种借用的情形同时只能有一种存在：
>
> * 一个或多个不可变引用（`&T`）
> * 一个可变引用（`&mut T`）

[ownership]: ownership.html
[borrowing]: references-and-borrowing.html#borrowing

所以，*不可变* 的真正含义是：是否可以安全地存在两个指向它的引用？在`Arc<T>`的例子中，这个答案是**yes**，因为变化完全发生在结构体内部，用户并不知情。注意，`clone()`的返回类型是`&Arc<T>`；如果返回的是`&mut Arc<T>`，就会出问题了。（🐷：调用两次`clone()`就会出现两个`&mut T`指向同一个资源的情况，违反了借用的规则。）

其他类型，例如[`std::cell`][stdcell]，的行为却恰恰相反，它们具有*内部可变性*。例如：

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow_mut();
```

[stdcell]: ../std/cell/index.html

RefCell的`borrow_mut()`方法会返回`RefMut<T>`类型，这个返回类型是一个封装，你可以把它当做是一个指向内部值（`T`）的可变引用使用，通过它可以改变内部的值。听起来是不是很危险？

```rust,ignore
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow_mut();
let z = x.borrow_mut();
# (y, z);
```

上面的代码确实会在运行时panic。`RefCell`的功能是：它在运行时确保程序的行为符合Rust的借用规则，如果违反了这些规则，就会`panic!`。我们通过它可以绕开Rust的另一条可变性规则——*字段可变性*。我们先介绍一下这条规则。

## 字段可变性

可变性是借用（`&mut`）或绑定（`let mut`）的属性。这意味着，比如，你不能将[`struct`][struct]的某些字段标记为可变、另一些字段标记为不可变：

```rust,ignore
struct Point {
    x: i32,
    mut y: i32, // Nope.
}
```

结构体的可变性体现对它的绑定上：

```rust,ignore
struct Point {
    x: i32,
    y: i32,
}

let mut a = Point { x: 5, y: 6 };

a.x = 10;

let b = Point { x: 5, y: 6};

b.x = 10; // 错误：不能给不可变字段`b.x`赋值。
```

[struct]: structs.html

不过，通过使用[`Cell<T>`][cell]，你可以模拟出字段级的可变性：

```rust
use std::cell::Cell;

struct Point {
    x: i32,
    y: Cell<i32>,
}

let point = Point { x: 5, y: Cell::new(6) };

point.y.set(7);

println!("y: {:?}", point.y);
```

[cell]: ../std/cell/struct.Cell.html

上面的代码会打印出`y: Cell { value: 7 }`。我们成功地改变了`y`。
