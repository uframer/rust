# 全局函数调用语法

有时候，函数可能会重名。请考虑如下的代码：

```rust
trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

let b = Baz;
```

如果我们想要调用`b.f()`，会遇到一个编译错误：

```text
error: multiple applicable methods in scope [E0034]
b.f();
  ^~~
note: candidate #1 is defined in an impl of the trait `main::Foo` for the type
`main::Baz`
    fn f(&self) { println!("Baz’s impl of Foo"); }
    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: candidate #2 is defined in an impl of the trait `main::Bar` for the type
`main::Baz`
    fn f(&self) { println!("Baz’s impl of Bar"); }
    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

```

我们需要一种方式来指明需要调用哪一个`f`方法。这种方式被称作*全局函数调用语法*，它的用法是：

```rust
# trait Foo {
#     fn f(&self);
# }
# trait Bar {
#     fn f(&self);
# }
# struct Baz;
# impl Foo for Baz {
#     fn f(&self) { println!("Baz’s impl of Foo"); }
# }
# impl Bar for Baz {
#     fn f(&self) { println!("Baz’s impl of Bar"); }
# }
# let b = Baz;
Foo::f(&b);
Bar::f(&b);
```

让我们仔细看看其中的关键之处：

```rust,ignore
Foo::
Bar::
```

These halves of the invocation are the types of the two traits: `Foo` and
`Bar`. This is what ends up actually doing the disambiguation between the two:
Rust calls the one from the trait name you use.

```rust,ignore
f(&b)
```

When we call a method like `b.f()` using [method syntax][methodsyntax], Rust
will automatically borrow `b` if `f()` takes `&self`. In this case, Rust will
not, and so we need to pass an explicit `&b`.

[methodsyntax]: method-syntax.html

# 尖括号形式

前面我们讲到的全局函数调用语法：

```rust,ignore
Trait::method(args);
```

其实是一种简写形式。有些情况下你需要用到它的完整格式：

```rust,ignore
<Type as Trait>::method(args);
```

The `<>::` syntax is a means of providing a type hint. The type goes inside
the `<>`s. In this case, the type is `Type as Trait`, indicating that we want
`Trait`’s version of `method` to be called here. The `as Trait` part is
optional if it’s not ambiguous. Same with the angle brackets, hence the
shorter form.

Here’s an example of using the longer form.

```rust
trait Foo {
    fn foo() -> i32;
}

struct Bar;

impl Bar {
    fn foo() -> i32 {
        20
    }
}

impl Foo for Bar {
    fn foo() -> i32 {
        10
    }
}

fn main() {
    assert_eq!(10, <Bar as Foo>::foo());
    assert_eq!(20, Bar::foo());
}
```

Using the angle bracket syntax lets you call the trait method instead of the
inherent one.
