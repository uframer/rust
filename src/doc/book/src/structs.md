# 结构体

可以用`struct`创建复杂数据类型。例如，我们要做二维空间的几何运算，我们需要`x`坐标和`y`坐标的值：

```rust
let origin_x = 0;
let origin_y = 0;
```

可以用`struct`将这两个变量组合在一起：

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0 }; // origin: Point

    println!("The origin is at ({}, {})", origin.x, origin.y);
}
```

按照约定，`struct`类型的名字应该采用驼峰命名法，例如：`PointInSpace`。

我们可以用`let`创建`struct`的实例，并用`键:值`对的风格初始化每个字段。初始化字段的顺序不一定要同声明时的顺序一样。

字段通过`.`来访问，例如`origin.x`。

默认情况下，`struct`中的值是不可变的。可以使用`mut`声明可变的`struct`：

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);
}
```

上面的程序会输出：`The point is at (5, 0)`。

在语言级别，Rust不支持单独定义字段的可变性，所以你不能这么写：

```rust,ignore
struct Point {
    mut x: i32, // This causes an error.
    y: i32,
}
```

可变性是绑定的属性，不是结构体的属性。如果你习惯了字段级别的可变性约束，可能会不太适应，但是Rust的设计可以简化很多事情。你甚至可以临时改变一个绑定的可变性：

```rust,ignore
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    let point = point; // `point` is now immutable.

    point.y = 6; // This causes an error.
}
```

不过，你的结构体可以包含`&mut`指针，这样就可以实现某种程度上的可变性：

```rust
struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);
}
```

Initialization of a data structure (struct, enum, union) can be simplified when
fields of the data structure are initialized with variables of the same
names as the fields.

```
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Debug-print struct
    println!("{:?}", peter);
}
```

# 更新语法

`struct`可以包含`..`关键字来表示你希望从另一个`struct`来复制某些值：

```rust
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

let mut point = Point3d { x: 0, y: 0, z: 0 };
point = Point3d { y: 1, .. point };
```

上面的代码会给`point`的`y`字段赋一个新的值，同时维持`x`和`z`的值不变。在这个例子里，`..`作用于同一个`stuct`，但是它可以应用于不同的`struct`，此时会从旧的`stuct`复制原有的值：

```rust
# struct Point3d {
#     x: i32,
#     y: i32,
#     z: i32,
# }
let origin = Point3d { x: 0, y: 0, z: 0 };
let point = Point3d { z: 1, x: 2, .. origin };
```

# 元组结构体

Rust的*元组结构体*就像是[元组][tuple]和`struct`的混合体。元组结构体有一个名字，但是它的字段没有名字。它们也用`struct`关键字声明，但是名字后面跟着的是一个元组：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```


可以通过`.`来访问元组结构体的成员，也可以通过所谓的destructuring
`let`语法来访问，就如同普通的元组一样：

```rust
# struct Color(i32, i32, i32);
# struct Point(i32, i32, i32);
# let black = Color(0, 0, 0);
# let origin = Point(0, 0, 0);
let black_r = black.0;
let Point(_, origin_y, origin_z) = origin;
```

Patterns like `Point(_, origin_y, origin_z)` are also used in
[match expressions][match].

One case when a tuple struct is very useful is when it has only one element.
We call this the ‘newtype’ pattern, because it allows you to create a new type
that is distinct from its contained value and also expresses its own semantic
meaning:

```rust
struct Inches(i32);

let length = Inches(10);

let Inches(integer_length) = length;
println!("length is {} inches", integer_length);
```

As above, you can extract the inner integer type through a destructuring `let`.
In this case, the `let Inches(integer_length)` assigns `10` to `integer_length`.
We could have used dot notation to do the same thing:

```rust
# struct Inches(i32);
# let length = Inches(10);
let integer_length = length.0;
```

It's always possible to use a `struct` instead of a tuple struct, and can be
clearer. We could write `Color` and `Point` like this instead:

```rust
struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}
```

Good names are important, and while values in a tuple struct can be
referenced with dot notation as well, a `struct` gives us actual names,
rather than positions.

[match]: match.html

# Unit-like structs

You can define a `struct` with no members at all:

```rust
struct Electron {} // Use empty braces...
struct Proton;     // ...or just a semicolon.

// Whether you declared the struct with braces or not, do the same when creating one.
let x = Electron {};
let y = Proton;
```

Such a `struct` is called ‘unit-like’ because it resembles the empty
tuple, `()`, sometimes called ‘unit’. Like a tuple struct, it defines a
new type.

This is rarely useful on its own (although sometimes it can serve as a
marker type), but in combination with other features, it can become
useful. For instance, a library may ask you to create a structure that
implements a certain [trait][trait] to handle events. If you don’t have
any data you need to store in the structure, you can create a
unit-like `struct`.

[trait]: traits.html
