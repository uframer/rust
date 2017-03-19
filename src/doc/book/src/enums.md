# 枚举

Rust使用`enum`关键字定义枚举类型——一组 *变体* 的集合。`enum`中的每个变体都可以带有可选的数据：

```rust
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
```

定义枚举变体的语法类似于定义结构体的语法：变体可以不带有任何数据（类似于 *单元结构体* ）；变体可以带有有名字的数据；变体可以带有没名字的数据（类似于 *元组结构体* ）。`enum`定义的是一个单独的类型，不是一组松散的结构体。一个枚举值可以匹配到任意一个变体。For this reason, an
enum is sometimes called a ‘sum type’: the set of possible values of the
enum is the sum of the sets of possible values for each variant.

We use the `::` syntax to use the name of each variant: they’re scoped by the name
of the `enum` itself. This allows both of these to work:

```rust
# enum Message {
#     Move { x: i32, y: i32 },
# }
let x: Message = Message::Move { x: 3, y: 4 };

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
```

Both variants are named `Move`, but since they’re scoped to the name of
the enum, they can both be used without conflict.

A value of an `enum` type contains information about which variant it is,
in addition to any data associated with that variant. This is sometimes
referred to as a ‘tagged union’, since the data includes a ‘tag’
indicating what type it is. The compiler uses this information to
enforce that you’re accessing the data in the enum safely. For instance,
you can’t simply try to destructure a value as if it were one of the
possible variants:

```rust,ignore
fn process_color_change(msg: Message) {
    let Message::ChangeColor(r, g, b) = msg; // This causes a compile-time error.
}
```

Not supporting these operations may seem rather limiting, but it’s a limitation
which we can overcome. There are two ways: by implementing equality ourselves,
or by pattern matching variants with [`match`][match] expressions, which you’ll
learn in the next section. We don’t know enough about Rust to implement
equality yet, but we’ll find out in the [`traits`][traits] section.

[match]: match.html
[traits]: traits.html

## 构造函数

`enum`的构造函数也可以如函数那样使用。例如：

```rust
# enum Message {
# Write(String),
# }
let m = Message::Write("Hello, world".to_string());
```

同下面的代码是等价的：

```rust
# enum Message {
# Write(String),
# }
fn foo(x: String) -> Message {
    Message::Write(x)
}

let x = foo("Hello, world".to_string());
```

This is not immediately useful to us, but when we get to
[`closures`][closures], we’ll talk about passing functions as arguments to
other functions. For example, with [`iterators`][iterators], we can do this
to convert a vector of `String`s into a vector of `Message::Write`s:

```rust
# enum Message {
# Write(String),
# }

let v = vec!["Hello".to_string(), "World".to_string()];

let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
```

[closures]: closures.html
[iterators]: iterators.html
