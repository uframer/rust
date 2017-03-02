# 循环

Rust提供了三种循环结构：`loop`、`while`和`for`。

## loop

无限循环`loop`是最简单的循环：

```rust,ignore
loop {
    println!("Loop forever!");
}
```

## while

Rust也支持`while`循环：

```rust
let mut x = 5; // mut x: i32
let mut done = false; // mut done: bool

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}
```

对于无限循环来说，你可以用`while`这样写：

```rust,ignore
while true {
```

不过，最好还是用`loop`：

```rust,ignore
loop {
```

Rust的控制流分析对`loop`和`while`区别对待。简单来说，给编译器的信息越多，编译器就越能生成更好的代码。

## for

Rust的`for`循环同其他系统程序设计语言的`for`有一些不同。例如，C语言的`for`循环像下面这样：

```c
for (x = 0; x < 10; x++) {
    printf( "%d\n", x );
}
```

而Rust的循环是这个样子：

```rust
for x in 0..10 {
    println!("{}", x); // x: i32
}
```

或者我们换成一种更抽象的写法：

```rust,ignore
for var in expression {
    code
}
```

这个写法中的`expression`是可以被[`IntoIterator`][`IntoIterator`]转换为[迭代器][iterator]的表达式。这个迭代器会返回一系列的元素。每个元素对应一个迭代，这个值被绑定到名字`var`，它的作用域是循环体。

[iterator]: iterators.html
[`IntoIterator`]: ../std/iter/trait.IntoIterator.html

在我们的例子中，`0..10`就是`expression`，迭代器由它产生。这个范围是左闭右开区间，也就是数学上的[0, 10)。

Rust不支持C风格的`for`循环。

### 枚举

如果你想要知道现在是第几次迭代，可以使用`.enumerate()`函数。

#### 枚举范围：

```rust
for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
}
```

输出：

```text
index = 0 and value = 5
index = 1 and value = 6
index = 2 and value = 7
index = 3 and value = 8
index = 4 and value = 9
```

不要忘记在范围`5..10`外边加上括号。

#### 枚举迭代器：

```rust
let lines = "hello\nworld".lines();

for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}
```

输出：

```text
0: hello
1: world
```

## 提前结束迭代

我们先回忆一下前面的`while`循环：

```rust
let mut x = 5;
let mut done = false;

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}
```

在这个例子中，我们需要`done`这个布尔类型的`mut`的变量绑定来判断什么时候应该退出循环。Rust有两个关键字控制循环的流程：`break`和`continue`。

在这个例子里，我们可以用`break`关键字：

```rust
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}
```

`return`语句也会退出循环。

`continue`同其他语言一样，会直接进入下一次迭代：

```rust
for x in 0..10 {
    if x % 2 == 0 { continue; }

    println!("{}", x);
}
```

## 循环标签

有时你需要退出嵌套的循环，这时普通的`break`和`continue`语句就不够用了，这时可以在`break`和`continue`后添加标签来跳转到指定的循环：

```rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }
}
```
