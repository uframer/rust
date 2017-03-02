% 属性

Rust的声明可以附带*属性（attribute）*，例如：

```rust
#[test]
# fn foo() {}
```

或者：

```rust
# mod foo {
#![test]
# }
```

上面两个例子的差别在于一个感叹号`!`，这个符号会改变属性应用的对象：

```rust,ignore
#[foo]
struct Foo;

mod bar {
    #![bar]
}
```

`#[foo]`这个属性应用于下一个项目，也就是`struct Foo;`这个声明。`#![bar]`这个属性则应用于包含它的那个外部项目，也就是`mod bar`设个声明。除此以外，它们的功能是相同的。

例如，请看下面这个函数：

```rust
#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}
```

它被标记为`#[test]`，这表示它是特殊的：当你运行[测试][tests]时，这个函数会被执行；当你做普通编译时，它甚至都不参加编译。也就是说，这个函数是一个测试专用函数。

[tests]: testing.html

属性也可以带有参数：

```rust
#[inline(always)]
fn super_fast_fn() {
# }
```

或者：

```rust
#[cfg(target_os = "macos")]
mod macos_only {
# }
```

Rust的属性用于很多方面。完整的用法请参考[这里][reference]。现在，还不支持自定义属性，你只能使用Rust编译器提供的属性。

[reference]: ../reference/attributes.html
