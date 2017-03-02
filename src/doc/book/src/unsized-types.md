# 未定大小类型

大多数类型都有一个特定的大小，以字节为单位，可以在编译时确定。例如，`i32`占用32个比特，或者说4个字节。不过，有些类型的大小可能并不能确定，我们称之为*未定大小*或者*动态大小*类型。例如，`[T]`这个类型表示某种类型`T`的序列，但是我们不知道这个序列包含多少个元素，因此它整体的大小也就不能确定下来。

Rust支持少数几种未定大小类型，但是它们在使用上有限三条限制：

1. 只能通过指针来操作未定大小类型的实例，例如，`&[T]`是合法的，`[T]`是非法的。
2. 变量和参数的类型不能是动态大小类型。
3. `struct`中，只有最后一个字段可以是动态大小类型，其他的字段不行。枚举变量不能用动态大小类型做数据。

既然后这么多限制，那为什么还要支持动态大小类型呢？ Well, because `[T]` can only be used behind a pointer, if we
didn’t have language support for unsized types, it would be impossible to write
this:

```rust,ignore
impl Foo for str {
```

or

```rust,ignore
impl<T> Foo for [T] {
```

Instead, you would have to write:

```rust,ignore
impl Foo for &str {
```

Meaning, this implementation would only work for [references][ref], and not
other types of pointers. With the `impl for str`, all pointers, including (at
some point, there are some bugs to fix first) user-defined custom smart
pointers, can use this `impl`.

[ref]: references-and-borrowing.html

# ?Sized

If you want to write a function that accepts a dynamically sized type, you
can use the special bound syntax, `?Sized`:

```rust
struct Foo<T: ?Sized> {
    f: T,
}
```

This `?Sized`, read as “T may or may not be `Sized`”, which allows us to match
both sized and unsized types. All generic type parameters implicitly
have the `Sized` bound, so the `?Sized` can be used to opt-out of the implicit
bound.
