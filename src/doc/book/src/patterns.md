# 模式

Rust中到处都可以见到模式。We use them in [variable
bindings][bindings], [match expressions][match], and other places, too. Let’s go
on a whirlwind tour of all of the things patterns can do!

[bindings]: variable-bindings.html
[match]: match.html

A quick refresher: you can match against literals directly, and `_` acts as an
‘any’ case:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

上面的代码会打印出`one`。

用一条 *全匹配* 绑定来匹配所有值是合法的：

```rust
let x = 1;

match x {
    y => println!("x: {} y: {}", x, y),
}
```

上面的代码会输出：

```text
x: 1 y: 1
```

注意，不能同时使用`_`和一个全匹配绑定：

```rust
let x = 1;

match x {
    y => println!("x: {} y: {}", x, y),
    _ => println!("anything"), // 错误！这行代码不可达
}
```

模式有一个缺陷：如同任何可以引入绑定的语言结构一样，它可能会 *遮蔽* 同名变量：

```rust
let x = 1;
let c = 'c';

match c {
    x => println!("x: {} c: {}", x, c),
}

println!("x: {}", x)
```

上面的代码会输出：

```text
x: c c: c
x: 1
```

换句话说，`x =>`这个模式会同时引入一个名为`x`的新绑定。这个新绑定的作用域它右边的语句，并且它的值会是`c`。注意，match表达式之外定义的`x`不会影响match表达式里面的`x`。因为match表达式之内`x`遮蔽了前面的`x`。

# Multiple patterns

You can match multiple patterns with `|`:

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one or two`.

# Destructuring

If you have a compound data type, like a [`struct`][struct], you can destructure it
inside of a pattern:

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x, y } => println!("({},{})", x, y),
}
```

[struct]: structs.html

We can use `:` to give a value a different name.

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
}
```

If we only care about some of the values, we don’t have to give them all names:

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 2, y: 3 };

match point {
    Point { x, .. } => println!("x is {}", x),
}
```

This prints `x is 2`.

You can do this kind of match on any member, not only the first:

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 2, y: 3 };

match point {
    Point { y, .. } => println!("y is {}", y),
}
```

This prints `y is 3`.

This ‘destructuring’ behavior works on any compound data type, like
[tuples][tuples] or [enums][enums].

[tuples]: primitive-types.html#tuples
[enums]: enums.html

# Ignoring bindings

You can use `_` in a pattern to disregard the type and value.
For example, here’s a `match` against a `Result<T, E>`:

```rust
# let some_value: Result<i32, &'static str> = Err("There was an error");
match some_value {
    Ok(value) => println!("got a value: {}", value),
    Err(_) => println!("an error occurred"),
}
```

In the first arm, we bind the value inside the `Ok` variant to `value`. But
in the `Err` arm, we use `_` to disregard the specific error, and print
a general error message.

`_` is valid in any pattern that creates a binding. This can be useful to
ignore parts of a larger structure:

```rust
fn coordinate() -> (i32, i32, i32) {
    // Generate and return some sort of triple tuple.
# (1, 2, 3)
}

let (x, _, z) = coordinate();
```

Here, we bind the first and last element of the tuple to `x` and `z`, but
ignore the middle element.

It’s worth noting that using `_` never binds the value in the first place,
which means that the value does not move:

```rust
let tuple: (u32, String) = (5, String::from("five"));

// Here, tuple is moved, because the String moved:
let (x, _s) = tuple;

// The next line would give "error: use of partially moved value: `tuple`".
// println!("Tuple is: {:?}", tuple);

// However,

let tuple = (5, String::from("five"));

// Here, tuple is _not_ moved, as the String was never moved, and u32 is Copy:
let (x, _) = tuple;

// That means this works:
println!("Tuple is: {:?}", tuple);
```

This also means that any temporary variables will be dropped at the end of the
statement:

```rust
// Here, the String created will be dropped immediately, as it’s not bound:

let _ = String::from("  hello  ").trim();
```

You can also use `..` in a pattern to disregard multiple values:

```rust
enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}

let x = OptionalTuple::Value(5, -2, 3);

match x {
    OptionalTuple::Value(..) => println!("Got a tuple!"),
    OptionalTuple::Missing => println!("No such luck."),
}
```

This prints `Got a tuple!`.

# ref and ref mut

If you want to get a [reference][ref], use the `ref` keyword:

```rust
let x = 5;

match x {
    ref r => println!("Got a reference to {}", r),
}
```

This prints `Got a reference to 5`.

[ref]: references-and-borrowing.html

Here, the `r` inside the `match` has the type `&i32`. In other words, the `ref`
keyword _creates_ a reference, for use in the pattern. If you need a mutable
reference, `ref mut` will work in the same way:

```rust
let mut x = 5;

match x {
    ref mut mr => println!("Got a mutable reference to {}", mr),
}
```

# Ranges

You can match a range of values with `...`:

```rust
let x = 1;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
}
```

This prints `one through five`.

Ranges are mostly used with integers and `char`s:

```rust
let x = '💅';

match x {
    'a' ... 'j' => println!("early letter"),
    'k' ... 'z' => println!("late letter"),
    _ => println!("something else"),
}
```

This prints `something else`.

# Bindings

You can bind values to names with `@`:

```rust
let x = 1;

match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

This prints `got a range element 1`. This is useful when you want to
do a complicated match of part of a data structure:

```rust
#[derive(Debug)]
struct Person {
    name: Option<String>,
}

let name = "Steve".to_string();
let x: Option<Person> = Some(Person { name: Some(name) });
match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
}
```

This prints `Some("Steve")`: we’ve bound the inner `name` to `a`.

If you use `@` with `|`, you need to make sure the name is bound in each part
of the pattern:

```rust
let x = 5;

match x {
    e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

# Guards

You can introduce ‘match guards’ with `if`:

```rust
enum OptionalInt {
    Value(i32),
    Missing,
}

let x = OptionalInt::Value(5);

match x {
    OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
    OptionalInt::Value(..) => println!("Got an int!"),
    OptionalInt::Missing => println!("No such luck."),
}
```

This prints `Got an int!`.

If you’re using `if` with multiple patterns, the `if` applies to both sides:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
}
```

This prints `no`, because the `if` applies to the whole of `4 | 5`, and not to
only the `5`. In other words, the precedence of `if` behaves like this:

```text
(4 | 5) if y => ...
```

not this:

```text
4 | (5 if y) => ...
```

# 混合使用各种匹配方式

前面介绍了很多匹配的方式，而且它们可以混起来用！

```rust,ignore
match x {
    Foo { x: Some(ref name), y: None } => ...
}
```

模式非常强大，善用它们。
