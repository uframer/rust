# 析构（`Drop`）

我们已经讨论过trait，现在看看Rust标准库提供的一个特别的trait：[`Drop`][drop]。`Drop`提供了一种在值脱离作用域时运行代码的方法。例如：

[drop]: ../std/ops/trait.Drop.html

```rust
struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let x = HasDrop;

    // 程序逻辑

} // `x`在这里脱离作用域
```

当`x`脱离`main()`的作用域时，会运行`Drop`里的代码。`Drop`里有一个方法`drop()`，它的参数是对`self`的可变引用。

`Drop`的原理说起来很简单，但是还是有很多细节需要注意。例如，同一个作用域之内的变量绑定按照声明的**相反顺序**被drop：

```rust
struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.strength);
    }
}

fn main() {
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
}
```

它会输出：

```text
BOOM times 100!!!
BOOM times 1!!!
```

`tnt`在`firecracker`之前被drop，因为它声明在前面。后进先出。

所以，`Drop`有什么好处呢？简单地说，`Drop`用于清理同一个`struct`相关联的资源。例如，[`Arc<T>`类型][arc]是一个引用计数类型。当`Drop`被调用时，它会给引用计数减一，如果总的引用计数变成了0，就会清除它所封装的值。

[arc]: ../std/sync/struct.Arc.html
