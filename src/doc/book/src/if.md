# if

Rust中`if`的设计并不是很复杂，但是它的行为更像是动态类型语言中的`if`而不是传统系统程序设计语言中的`if`。我们看一个简单的例子：

```rust
let x = 5;

if x == 5 {
    println!("x is five!");
}
```

如果`if`后面的表达式的值是`true`，那么就执行代码块；如果是`false`，那么就执行`else`语句后面的代码块：

```rust
let x = 5;

if x == 5 {
    println!("x is five!");
} else {
    println!("x is not five :(");
}
```

如果有很多分支，可以使用`else if`写法：

```rust
let x = 5;

if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    println!("x is not five or six :(");
}
```

上面是一种相当标准的写法，不过，你也可以这样写：

```rust
let x = 5;

let y = if x == 5 {
    10
} else {
    15
}; // y: i32
```

但是，最好还是写成这样：

```rust
let x = 5;

let y = if x == 5 { 10 } else { 15 }; // y: i32
```

显然，我们能这样写的原因是`if`结构也是一个表达式，这个表达式的值是被选中的分支代码块中最后一条语句（或者更为准确地说，最后一个表达式）的值。如果这个`if`结构没有`else`，那么如果条件不满足，就会返回`()`。
