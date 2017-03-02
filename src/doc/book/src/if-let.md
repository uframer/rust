# if let

`if let`语法允许你在[if][if]语句中使用[模式][patterns]匹配，从而简化某些使用[模式][patterns]匹配的代码。

> 🐷：可以参考Swift语言的`if let`语法。

例如，假设有某种类型的`Option<T>`，我们希望如果它是`Some<T>`，那么调用它的某个函数；如果它是`None`时，什么也不做。我们可以这样写：

```rust
# let option = Some(5);
# fn foo(x: i32) { }
match option {
    Some(x) => { foo(x) },
    None => {},
}
```

如果我们用`if`，就可以不写`match`：

```rust
# let option = Some(5);
# fn foo(x: i32) { }
if option.is_some() {
    let x = option.unwrap();
    foo(x);
}
```

这两种写法都不怎么好看，用`if let`语法可以让代码更好看一些：

```rust
# let option = Some(5);
# fn foo(x: i32) { }
if let Some(x) = option {
    foo(x);
}
```

如果一个模式[pattern][patterns]成功匹配，它会将合适的值绑定到模式汇总的标识符上，然后执行`if`代码块的内容。如果这个模式匹配失败，则执行`else`代码块的内容：

```rust
# let option = Some(5);
# fn foo(x: i32) { }
# fn bar() { }
if let Some(x) = option {
    foo(x);
} else {
    bar();
}
```

## `while let`

对于循环来说，也有类似的写法`while let`，只要模式匹配成功，就会继续执行下一次迭代，否则跳出循环。我们可以将下面的代码：

```rust
let mut v = vec![1, 3, 5, 7, 11];
loop {
    match v.pop() {
        Some(x) =>  println!("{}", x),
        None => break,
    }
}
```

改写成这个样子：

```rust
let mut v = vec![1, 3, 5, 7, 11];
while let Some(x) = v.pop() {
    println!("{}", x);
}
```

[patterns]: patterns.html
[if]: if.html
