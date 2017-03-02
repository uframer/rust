# 方法的语法

Rust通过`impl`关键字提供方法调用的语法。

## 方法调用

看下面的例子：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
}
```

它会输出：`12.566371`。

我们定义了一个`struct`来表示一个圆。然后我们写了一个`impl`代码块，在里面定义了方法`area`。

方法的第一个参数是一个特殊参数，你有三种选择：`self`、`&self`和`&mut self`。通常，我们应该选择`&self`。下面分别演示了这三种不同的用法：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn reference(&self) {
       println!("taking self by reference!");
    }

    fn mutable_reference(&mut self) {
       println!("taking self by mutable reference!");
    }

    fn takes_ownership(self) {
       println!("taking ownership of self!");
    }
}
```

你可以为同一个`struct`写很多的`impl`代码块，前面的例子也可以写成：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn reference(&self) {
       println!("taking self by reference!");
    }
}

impl Circle {
    fn mutable_reference(&mut self) {
       println!("taking self by mutable reference!");
    }
}

impl Circle {
    fn takes_ownership(self) {
       println!("taking ownership of self!");
    }
}
```

## 方法的链式调用

我们可以连续调用一个对象上的多个方法，例如`foo.bar().baz()`。下面我们再看更为详细的例子：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    # 链式调用
    let d = c.grow(2.0).area();
    println!("{}", d);
}
```

## 关联函数

如果你的第一个参数不是`self`，那么你定义的就是关联函数。下面的模式在Rust代码里非常常见：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
}
```

关联函数构造了一个新的`Circle`。请注意，调用关联函数的语法是`Struct::function()`而不是`ref.method()`。在其他语言中，关联函数被称作静态方法。

## 构造器模式

假设我们希望我们的用户可以创建`Circle`对象，而且只需要用户设置他们感兴趣的属性的值。如果用户不设置，那么`x`和`y`的值会是`0.0`，`radius`会是`1.0`。Rust不支持方法重载、命名参数或者可变参数列表。为了实现这个需求，我们可以采用构造器模式：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

fn main() {
    let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
```

上面的例子中，`Circle`不再有`new`这个关联函数，构造对象的代码被放到了`CircleBuilder`中，由它来返回一个对象。说起来，还真是很不方便。
