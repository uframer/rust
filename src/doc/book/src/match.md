# `match`匹配

很多时候，你可能要处理很多的分支，简单的[`if`][if]/`else`语句就不够用了。而且，判断条件也可能变得很复杂。Rust提供了一个名叫`match`的关键字，你可以用它来取代一连串的`if`/`else`判断。看一下代码：

```rust
let x = 5;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

[if]: if.html

`match`接受一个表达式，然后根据表达式的值跳转到对应的分支。每个分支的语法格式是`val => expression`。如果某个分支匹配成功，那么它右边的表达式会被求值。关键字的名字`match`来自于术语 *模式匹配（pattern matching）* ，`match`表达式是Rust中的一种模式匹配应用。我们会在[模式][patterns]一章中详细介绍所有可能的模式。

[patterns]: patterns.html

`match`的优点之一是，它会检查是否覆盖了所有的情况。例如，如果我们移除上面例子中的最后一行的`_`，编译器会给出错误：

```text
error: non-exhaustive patterns: `_` not covered
```

Rust告诉我们忘了匹配一些情况。编译器从`x`推断出它需要处理所有的32位整数值，从-2,147,483,648到2,147,483,647。`_`的作用是匹配任何值，用于捕捉其它所有匹配模式不能匹配的情况。As you can see in the previous example, we provide `match`
arms for integers 1-5, if `x` is 6 or any other value, then it is caught by `_`.

`match` is also an expression, which means we can use it on the right-hand
side of a `let` binding or directly where an expression is used:

```rust
let x = 5;

let number = match x {
    1 => "one",
    2 => "two",
    3 => "three",
    4 => "four",
    5 => "five",
    _ => "something else",
};
```

Sometimes it’s a nice way of converting something from one type to another; in
this example the integers are converted to `String`.

# 匹配枚举值

Another important use of the `match` keyword is to process the possible
variants of an enum:

```rust
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn quit() { /* ... */ }
fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
fn move_cursor(x: i32, y: i32) { /* ... */ }

fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x, y: new_name_for_y } => move_cursor(x, new_name_for_y),
        Message::Write(s) => println!("{}", s),
    };
}
```

Again, the Rust compiler checks exhaustiveness, so it demands that you
have a match arm for every variant of the enum. If you leave one off, it
will give you a compile-time error unless you use `_` or provide all possible
arms.

Unlike the previous uses of `match`, you can’t use the normal `if`
statement to do this. You can use the [`if let`][if-let] statement,
which can be seen as an abbreviated form of `match`.

[if-let]: if-let.html
