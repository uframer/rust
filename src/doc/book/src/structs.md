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
    mut x: i32, // 这行会造成编译错误。
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

    let point = point; // `point`现在是不可变的。

    point.y = 6; // 这行会造成编译错误。
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

如果初始化所用参数的变量名同字段的名字是一样的话，可以简化数据结构初始化（`struct`、`enum`、`union`）的写法。注意下面例子中的`name`和`age`：

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // 使用简化写法初始化结构体
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 用Debug的功能打印这个结构体
    println!("{:?}", peter);
}
```

> 🐷：这个简化的写法现在需要`unstable`的编译器。

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

Rust的 *元组结构体* 就像是[元组][tuple]和`struct`的混合体。元组结构体有一个名字，但是它的字段没有名字。它们也用`struct`关键字声明，但是名字后面跟着的是一个元组：

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

[match表达式][match]中也会用到形如`Point(_, origin_y, origin_z)`的模式。

一个能让元组结构体大展拳脚的场景是当它只含有一个元素的时候。我们把这种用法成为 *newtype* 模式，因为你可以在保持原有值的语义的情况下创建一个新的类型：

```rust
struct Inches(i32);

let length = Inches(10);

let Inches(integer_length) = length;
println!("length is {} inches", integer_length);
```

如同上面的例子，你可以通过所谓的 *析构* `let`来匹配得到真正的值：`let Inches(integer_length)`会把`10`赋值给`integer_length`。我们也可以用点语法获取数值：

```rust
# struct Inches(i32);
# let length = Inches(10);
let integer_length = length.0;
```

所有使用元组结构体的场景下，你都可以用`struct`替代元组结构体，而且也会更为清晰。我们的`Color`和`Point`可以写成这样：

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

给字段起一个好名字是很重要的事情，尽管元组结构体的成员可以通过点语法访问，却不可能像`struct`那样给每个字段一个清晰的名字。

[match]: match.html

# 单元结构体（unit-like `struct`）

你可以定义一个没有任何成员的结构体`struct`：

```rust
struct Electron {} // 只有两个大括号...
struct Proton;     // ...或者直接跟着一个分号。

// 不论你声明结构体时用的是上面那种写法，创建它的实例的时候要使用同声明一样的解法。
let x = Electron {};
let y = Proton;
```

之所以把这种`struct`称作 *单元结构体* ，是因为它同空元组`()`很像，而空元组有个别名就叫做 *单元* 。同元组结构体一样，它也定义了一种新的类型。

这个写法本身并没有太多用处（尽管有时候可以把它当做标记类型），通常需要和别的语言特性结合使用。例如，一个库可能要求你创建一个实现了特定[特征][trait]的结构体来处理事件（🐷：类似于别的语言中Callback Handler的概念）。如果你并不需要在结构体中保存任何数据的话，就可以使用单元结构体。

[trait]: traits.html
