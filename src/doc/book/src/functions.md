# 函数

每个Rust程序都至少有一个函数，`main`函数：

```rust
fn main() {
}
```

它可能是最简单的函数声明形式了。`fn`关键字表示这一个一个函数，后面跟着函数名，括号里列出了函数的参数，这里参数列表为空，最后的大括号定义了函数体。

接下来我们看看参数如何定义。下面是一个打印数字的函数：

```rust
fn print_number(x: i32) {
    println!("x is: {}", x);
}
```

下面的程序演示了如何调用`print_number`：

```rust
fn main() {
    print_number(5);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}
```

如你所见，函数参数的形式同`let`声明十分类似：先写参数名，然后在冒号后面跟上参数类型。

下面这个函数将两个参数值相加并打印出来：

```rust
fn main() {
    print_sum(5, 6);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}
```

多个参数之间使用逗号分隔。

与`let`不同，你*必须*声明函数参数的类型，下面这样写不行：

```rust,ignore
fn print_sum(x, y) {
    println!("sum is: {}", x + y);
}
```

编译会遇到错误：

```text
expected one of `!`, `:`, or `@`, found `)`
fn print_sum(x, y) {
```

这个设计是经过仔细权衡的。尽管做完全的类型推断是可行的，但是那些支持完全类型推断的语言，例如Haskell，通常也会建议你在参数列表里标注上参数类型，因为这样可以形成更为详细的接口文档。Rust社区认为在参数列表上强制要求类型标注并在内部使用类型推断是一个较好的平衡点。

函数的返回值如何定义呢？下面这个函数将参数加一并返回：

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

Rust函数只能返回一个值，返回值的类型定义在`->`后。函数的最后一行确定了函数的返回值。你可能注意到了，上面例子中函数的最后一行没有分号。如果我们加上分号：

```rust,ignore
fn add_one(x: i32) -> i32 {
    x + 1;
}
```

就会发生编译错误：

```text
error: not all control paths return a value
fn add_one(x: i32) -> i32 {
     x + 1;
}

help: consider removing this semicolon:
     x + 1;
          ^
```

这个错误给出了两个信息：
* Rust是一个基于表达式的语言；
* Rust里的分号同其他使用分号作为语句结束符的语言（例如C和Java）不同。

## 表达式 vs. 语句

Rust基本上就是一个基于表达式的语言，它只有两种类型的语句：*声明语句*和*表达式语句*，其他的都是表达式。表达式会返回一个值，语句则没有返回值。

我们先聊聊声明语句。

在某些语言中，变量绑定可以被写作表达式而不是语句。如Ruby：

```ruby
x = y = 5
```

不过在Rust中，我们用`let`关键字来声明绑定，它*不是*表达式。下面的代码无法通过编译：

```rust,ignore
let x = (let y = 5); // 编译器期望这里是标识符，但是却找到了`let`关键字。
```

编译器告诉我们这里应该是一个表达式，但是`let`关键字定义的是一条语句，不是表达式。

需要注意的是，给已经已经做过绑定的变量赋值的动作（例如，`y = 5`）也是一个表达式，不过它的值一般来说没有什么意义。有些语言的赋值语句会返回所赋的值（例如，C语言里`y = 5`会返回`5`），但是在Rust中，赋值表达式的值是一个空的元组`()`。这么设计的原因是*被赋的值*应该[只有一个所有者](ownership.html)，如果把赋值表达式的值设计成其他语言里的行为，就会同这一行为发生冲突。

```rust
let mut y = 5;

let x = (y = 6);  // `x`的值是`()`，不是`6`！
```

Rust中的第二种语句是*表达式语句*，也就是把表达式转换为一个语句。实际上，Rust期待着一条语句跟着另一条语句，也就是要求你用分号分隔各个表达式。这意味着Rust程序看起来很像其他要求用分号做语句结尾的语言。在Rust中，几乎所有的语句结尾都有`;`。既然说是*几乎所有*，就意味着还有例外：

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

这个函数里就一行，而且没有`;`，这样写，`add_one`返回的就是一个`i32`；如果加上了`;`，`add_one`返回的就是`()`。

## 提前返回

可以用`return`关键字在函数体中提前返回：

```rust
fn foo(x: i32) -> i32 {
    return x;

    // 我们永远运行不到这！
    x + 1
}
```

在Rust里，在函数的最后一行使用`return`语句被认为是糟糕的编码风格：

```rust
fn foo(x: i32) -> i32 {
    return x + 1;
}
```

如果你之前没有接触过基于表达式的语言，那么这种不用`return`的写法可能看起来不怎么习惯，时间长了你就习惯了。

## “发散函数”

Rust将那些永远不会返回的函数叫做“发散函数”，并为这类函数提供了特殊的语法：

```rust
fn diverges() -> ! {
    panic!("This function never returns!");
}
```

`panic!`是一个宏，会让当前正在执行的线程崩溃，因此它永远不会返回，你可以注意到它的返回类型是‘`!`’。

如果你在`main`函数里调用`diverges()`的话，应该会看到类似下面的输出：

```text
thread ‘main’ panicked at ‘This function never returns!’, hello.rs:2
```

如果想要看到更多信息，可以通过设置`RUST_BACKTRACE`环境变量查看栈信息：

```text
$ RUST_BACKTRACE=1 ./diverges
thread 'main' panicked at 'This function never returns!', hello.rs:2
Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
  hello::diverges
        at ./hello.rs:2
  hello::main
        at ./hello.rs:6
```

If you want the complete backtrace and filenames:

```text
$ RUST_BACKTRACE=full ./diverges
thread 'main' panicked at 'This function never returns!', hello.rs:2
stack backtrace:
   1:     0x7f402773a829 - sys::backtrace::write::h0942de78b6c02817K8r
   2:     0x7f402773d7fc - panicking::on_panic::h3f23f9d0b5f4c91bu9w
   3:     0x7f402773960e - rt::unwind::begin_unwind_inner::h2844b8c5e81e79558Bw
   4:     0x7f4027738893 - rt::unwind::begin_unwind::h4375279447423903650
   5:     0x7f4027738809 - diverges::h2266b4c4b850236beaa
   6:     0x7f40277389e5 - main::h19bb1149c2f00ecfBaa
   7:     0x7f402773f514 - rt::unwind::try::try_fn::h13186883479104382231
   8:     0x7f402773d1d8 - __rust_try
   9:     0x7f402773f201 - rt::lang_start::ha172a3ce74bb453aK5w
  10:     0x7f4027738a19 - main
  11:     0x7f402694ab44 - __libc_start_main
  12:     0x7f40277386c8 - <unknown>
  13:                0x0 - <unknown>
```

如果`RUST_BACKTRACE`为0，不会输出栈的信息；否则，任何其他的值（甚至压根儿不设置）都会输出栈信息。

```text
$ export RUST_BACKTRACE=1
...
$ RUST_BACKTRACE=0 ./diverges 
thread 'main' panicked at 'This function never returns!', hello.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Cargo的`run`命令也支持`RUST_BACKTRACE`环境变量：

```text
$ RUST_BACKTRACE=full cargo run
     Running `target/debug/diverges`
thread 'main' panicked at 'This function never returns!', hello.rs:2
stack backtrace:
   1:     0x7f402773a829 - sys::backtrace::write::h0942de78b6c02817K8r
   2:     0x7f402773d7fc - panicking::on_panic::h3f23f9d0b5f4c91bu9w
   3:     0x7f402773960e - rt::unwind::begin_unwind_inner::h2844b8c5e81e79558Bw
   4:     0x7f4027738893 - rt::unwind::begin_unwind::h4375279447423903650
   5:     0x7f4027738809 - diverges::h2266b4c4b850236beaa
   6:     0x7f40277389e5 - main::h19bb1149c2f00ecfBaa
   7:     0x7f402773f514 - rt::unwind::try::try_fn::h13186883479104382231
   8:     0x7f402773d1d8 - __rust_try
   9:     0x7f402773f201 - rt::lang_start::ha172a3ce74bb453aK5w
  10:     0x7f4027738a19 - main
  11:     0x7f402694ab44 - __libc_start_main
  12:     0x7f40277386c8 - <unknown>
  13:                0x0 - <unknown>
```

发送函数的返回值可以被绑定到任何类型的变量：

```rust,should_panic
# fn diverges() -> ! {
#    panic!("This function never returns!");
# }
let x: i32 = diverges();
let x: String = diverges();
```

## 函数指针

我们也可以创建指向函数的变量绑定：

```rust
let f: fn(i32) -> i32;
```

`f`是一个指向函数的变量绑定，这个函数有一个类型为`i32`的参数，并且返回类型为`i32`的值。例如：

```rust
fn plus_one(i: i32) -> i32 {
    i + 1
}

// 不使用类型推断：
let f: fn(i32) -> i32 = plus_one;

// 使用类型推断：
let f = plus_one;
```

我们可以直接使用函数指针`f`调用函数：

```rust
# fn plus_one(i: i32) -> i32 { i + 1 }
# let f = plus_one;
let six = f(5);
```
