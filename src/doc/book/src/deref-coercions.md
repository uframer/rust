# `Deref`解引用转换

标准库提供了一名为[`Deref`][deref]的特殊的特征，它通常被用于重载解引用运算符`*`：

```rust
use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

fn main() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);
}
```

[deref]: ../std/ops/trait.Deref.html

这个特征常常用于编写自定义的指针类型。不过，Rust中还有一个同`Deref`相关的语言特征：*解引用转换（deref coercion）*，它的规则如下：假设已有类型`U`，并且它实现了`Deref<Target=T>`，也就是可以从`&U`解引用出`T`，它那么`&U`可以被自动转换为`&T`。看一个例子：

```rust
fn foo(s: &str) {
    // 借用一个字符串
}

// 这里to_string()返回的是String类型，也就是owned是String类型
let owned = "Hello".to_string();

// 由于String类型实现了Deref<Target=str>，因此，这句是合法的，&owned的类型会被自动转换为&str：
foo(&owned);
```

这条规则是Rust唯一的一条自动类型转换规则。它提供了很大的灵活性。例如，`Rc<T>`实现了`Deref<Target=T>`，所以下面的代码是合法的：

```rust
use std::rc::Rc;

fn foo(s: &str) {
    // 借用一个字符串
}

// 这里to_string()返回的是String类型，也就是owned是String类型
let owned = "Hello".to_string();
// 用Rc<T>封装owned，这里counted类型被推导为Rc<String>
let counted = Rc::new(owned);

// 这里其实用到了两次转换：
// 1. &Rc<String>可以被转换为&String
// 2. &String被转换为&str
// 至此，counted就可以被foo接受了
foo(&counted);
```

在转换时，Rust会尝试尽可能多的可能的转换。

标准库中另一种常见的用法是，也就是向量的引用可以解引用为切片：

```rust
fn foo(s: &[i32]) {
    // 借用一个切片
}

// owned的类型是Vec<T>
let owned = vec![1, 2, 3];

// Vec<T>实现了Deref<Target=[T]>，所以这里&[i32;3]可以被转换为&[i32]
foo(&owned);
```

## Deref and method calls

`Deref` will also kick in when calling a method. Consider the following
example.

```rust
struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

let f = &&Foo;

f.foo();
```

Even though `f` is a `&&Foo` and `foo` takes `&self`, this works. That’s
because these things are the same:

```rust,ignore
f.foo();
(&f).foo();
(&&f).foo();
(&&&&&&&&f).foo();
```

A value of type `&&&&&&&&&&&&&&&&Foo` can still have methods defined on `Foo`
called, because the compiler will insert as many * operations as necessary to
get it right. And since it’s inserting `*`s, that uses `Deref`.
