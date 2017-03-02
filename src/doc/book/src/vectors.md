# 向量

*vector*是一个动态的或者说可增长的数组，它实现为一个标准库类型[`Vec<T>`][vec]。这里的`T`表示它里面可以容纳任意类型的值（参见[泛型][generic]）。Vector总是在堆上分配内存。可以使用`vec!`宏创建它的实例：

```rust
let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
```

> 注意：我们之前调用`println!`宏是用的是`()`，这里调用`vec!`宏用的是`[]`。Rust对这两种写法都支持，具体用哪种只是一种使用习惯或者说约定。

如果`vec!`所有成员的初始值都是一样的，那么还有一种简单的写法：

```rust
let v = vec![0; 10]; // 十个0组成的向量。
```

Vector将它的内容存储为堆上的连续的`T`类型元素的数组。这意味着编译器必须能够在编译时就知道`T`的大小。如果有些类型的大小在编译时无法确定，那么你就只能保存指向它们的指针了，[`Box`][box]类型就是为此而生的。

## 访问元素

使用`[]`通过索引访问元素：

```rust
let v = vec![1, 2, 3, 4, 5];

println!("The third element of v is {}", v[2]);
```

索引从`0`开始。注意，索引的类型必须是`usize`：

```rust,ignore
let v = vec![1, 2, 3, 4, 5];

let i: usize = 0;
let j: i32 = 0;

// Works:
v[i];

// Doesn’t:
v[j];
```

如果索引的类型不是`usize`，那么编译器会给出类似下面的错误：

```text
error: the trait bound `collections::vec::Vec<_> : core::ops::Index<i32>`
is not satisfied [E0277]
v[j];
^~~~
note: the type `collections::vec::Vec<_>` cannot be indexed by `i32`
error: aborting due to previous error
```

上面的消息里标点符号满天飞，但是它传达的消息是很清楚的：你不能用`i32`做索引。

## 访问越界

如果你试图访问一个并不存在的索引：

```rust,ignore
let v = vec![1, 2, 3];
println!("Item 7 is {}", v[7]);
```

那么当前线程会[panic]并输出一条类似这样的消息：

```text
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 7'
```

如果你想避免出现panic，那么可以使用[`get`][get]或者[`get_mut`][get_mut]方法，它们会在索引不存在时返回`None`：

```rust
let v = vec![1, 2, 3];
match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
}
```

## 迭代

可以用`for`遍历向量的所有元素。有三种不同的写法：

```rust
let mut v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
```

注意：在迭代过程获得向量的所有权之后，你就不能再使用它了。You can iterate the vector multiple times by taking a reference to the vector whilst iterating.
For example, the following code does not compile.

```rust,ignore
let v = vec![1, 2, 3, 4, 5];

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
```

Whereas the following works perfectly,

```rust
let v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("This is a reference to {}", i);
}

for i in &v {
    println!("This is a reference to {}", i);
}
```

Vectors have many more useful methods, which you can read about in [their
API documentation][vec].

[vec]: ../std/vec/index.html
[box]: ../std/boxed/index.html
[generic]: generics.html
[panic]: concurrency.html#panics
[get]: ../std/vec/struct.Vec.html#method.get
[get_mut]: ../std/vec/struct.Vec.html#method.get_mut
