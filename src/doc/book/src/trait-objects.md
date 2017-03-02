# 特征对象

如果代码中有多态机制，那么就需要确定具体使用的是哪一路代码，这个过程叫做*分派*。分派有两种：静态分派和动态分派。尽管Rust倾向于使用静态分派，但是它也通过名叫*特征对象*的机制支持动态分派。

## 背景

在本章的后续内容中，我们需要用一个特征和它的实现做例子讲解特征对象机制。我们举一个简单的例子，一个名叫`Foo`的特征，它有一个返回`String`类型的方法。

```rust
trait Foo {
    fn method(&self) -> String;
}
```

在这个例子中，我们还为`u8`和`String`实现了`Foo`这个特征：

```rust
# trait Foo { fn method(&self) -> String; }
impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}
```


## 静态分派

我们可以使用*特征绑定*机制执行静态分派：

```rust
# trait Foo { fn method(&self) -> String; }
# impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
# impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something<T: Foo>(x: T) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);
}
```

Rust使用所谓的*单态化**（monomorphization）*来实现静态分派。它的含义是Rust会为`u8`和`String`类型分别创建一个`do_something()`的特化版本，然后把对这个函数的调用依据类型不同替换为对应的特化版本。换句话说，Rust会生成类似如下的代码：

```rust
# trait Foo { fn method(&self) -> String; }
# impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
# impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something_u8(x: u8) {
    x.method();
}

fn do_something_string(x: String) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something_u8(x);
    do_something_string(y);
}
```

这种方法有一个很大的优点：由于被调用的是哪个特化版本在编译时是可以确定的，因此静态分派的函数可能被内联处理，而内联是一种常用的优化技巧。静态分派在运行时可能很快，但是也有代价：*代码膨胀*——每种类型对应的特化版本都需要占用一块空间。

此外，编译器也不是完美无缺的，许多所谓的*优化*甚至可能越*优化*越差。例如，如果函数内联策略过于激进，就会极大地增加代码大小，可能会撑爆指令缓存。这也是为什么我们应该小心使用`#[inline]`和`#[inline(always)]`特性，同理，有时候使用动态分派可能更为高效。

不过，多数情况下使用静态分派更为高效，而且你总是可以写一个封装了动态分派实现的静态分派函数，但是反过来就不行了，也就是说，静态分派更为灵活。正是由于这个原因，标准库总是尽可能地使用静态分派。

## 动态分派

Rust通过名为*特征对象*的方式提供了动态分派机制。如`&Foo`或`Box<Foo>`这样的特征对象， are normal values that store a value of
*any* type that implements the given trait, where the precise type can only be
known at runtime.

A trait object can be obtained from a pointer to a concrete type that
implements the trait by *casting* it (e.g. `&x as &Foo`) or *coercing* it
(e.g. using `&x` as an argument to a function that takes `&Foo`).

These trait object coercions and casts also work for pointers like `&mut T` to
`&mut Foo` and `Box<T>` to `Box<Foo>`, but that’s all at the moment. Coercions
and casts are identical.

由于这个操作可以被看做*抹掉*编译器中某个指针的类型信息，因此特征对象有时候也被叫做*类型擦除*。

回到前面的例子，we can use the same trait to perform dynamic
dispatch with trait objects by casting:

```rust
# trait Foo { fn method(&self) -> String; }
# impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
# impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something(x: &Foo) {
    x.method();
}

fn main() {
    let x = 5u8;
    do_something(&x as &Foo);
}
```

or by coercing:

```rust
# trait Foo { fn method(&self) -> String; }
# impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
# impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something(x: &Foo) {
    x.method();
}

fn main() {
    let x = "Hello".to_string();
    do_something(&x);
}
```

A function that takes a trait object is not specialized to each of the types
that implements `Foo`: only one copy is generated, often (but not always)
resulting in less code bloat. However, this comes at the cost of requiring
slower virtual function calls, and effectively inhibiting any chance of
inlining and related optimizations from occurring.

### Why pointers?

Rust does not put things behind a pointer by default, unlike many managed
languages, so types can have different sizes. Knowing the size of the value at
compile time is important for things like passing it as an argument to a
function, moving it about on the stack and allocating (and deallocating) space
on the heap to store it.

For `Foo`, we would need to have a value that could be at least either a
`String` (24 bytes) or a `u8` (1 byte), as well as any other type for which
dependent crates may implement `Foo` (any number of bytes at all). There’s no
way to guarantee that this last point can work if the values are stored without
a pointer, because those other types can be arbitrarily large.

Putting the value behind a pointer means the size of the value is not relevant
when we are tossing a trait object around, only the size of the pointer itself.

### Representation

The methods of the trait can be called on a trait object via a special record
of function pointers traditionally called a ‘vtable’ (created and managed by
the compiler).

Trait objects are both simple and complicated: their core representation and
layout is quite straight-forward, but there are some curly error messages and
surprising behaviors to discover.

Let’s start simple, with the runtime representation of a trait object. The
`std::raw` module contains structs with layouts that are the same as the
complicated built-in types, [including trait objects][stdraw]:

```rust
# mod foo {
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
# }
```

[stdraw]: ../std/raw/struct.TraitObject.html

That is, a trait object like `&Foo` consists of a ‘data’ pointer and a ‘vtable’
pointer.

The data pointer addresses the data (of some unknown type `T`) that the trait
object is storing, and the vtable pointer points to the vtable (‘virtual method
table’) corresponding to the implementation of `Foo` for `T`.


A vtable is essentially a struct of function pointers, pointing to the concrete
piece of machine code for each method in the implementation. A method call like
`trait_object.method()` will retrieve the correct pointer out of the vtable and
then do a dynamic call of it. For example:

```rust,ignore
struct FooVtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}

// u8:

fn call_method_on_u8(x: *const ()) -> String {
    // The compiler guarantees that this function is only called
    // with `x` pointing to a u8.
    let byte: &u8 = unsafe { &*(x as *const u8) };

    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 1,
    align: 1,

    // Cast to a function pointer:
    method: call_method_on_u8 as fn(*const ()) -> String,
};


// String:

fn call_method_on_String(x: *const ()) -> String {
    // The compiler guarantees that this function is only called
    // with `x` pointing to a String.
    let string: &String = unsafe { &*(x as *const String) };

    string.method()
}

static Foo_for_String_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    // Values for a 64-bit computer, halve them for 32-bit ones.
    size: 24,
    align: 8,

    method: call_method_on_String as fn(*const ()) -> String,
};
```

The `destructor` field in each vtable points to a function that will clean up
any resources of the vtable’s type: for `u8` it is trivial, but for `String` it
will free the memory. This is necessary for owning trait objects like
`Box<Foo>`, which need to clean-up both the `Box` allocation as well as the
internal type when they go out of scope. The `size` and `align` fields store
the size of the erased type, and its alignment requirements.

Suppose we’ve got some values that implement `Foo`. The explicit form of
construction and use of `Foo` trait objects might look a bit like (ignoring the
type mismatches: they’re all pointers anyway):

```rust,ignore
let a: String = "foo".to_string();
let x: u8 = 1;

// let b: &Foo = &a;
let b = TraitObject {
    // Store the data:
    data: &a,
    // Store the methods:
    vtable: &Foo_for_String_vtable
};

// let y: &Foo = x;
let y = TraitObject {
    // Store the data:
    data: &x,
    // Store the methods:
    vtable: &Foo_for_u8_vtable
};

// b.method();
(b.vtable.method)(b.data);

// y.method();
(y.vtable.method)(y.data);
```

## Object Safety

Not every trait can be used to make a trait object. For example, vectors implement
`Clone`, but if we try to make a trait object:

```rust,ignore
let v = vec![1, 2, 3];
let o = &v as &Clone;
```

We get an error:

```text
error: cannot convert to a trait object because trait `core::clone::Clone` is not object-safe [E0038]
let o = &v as &Clone;
        ^~
note: the trait cannot require that `Self : Sized`
let o = &v as &Clone;
        ^~
```

The error says that `Clone` is not ‘object-safe’. Only traits that are
object-safe can be made into trait objects. A trait is object-safe if both of
these are true:

* the trait does not require that `Self: Sized`
* all of its methods are object-safe

So what makes a method object-safe? Each method must require that `Self: Sized`
or all of the following:

* must not have any type parameters
* must not use `Self`

Whew! As we can see, almost all of these rules talk about `Self`. A good intuition
is “except in special circumstances, if your trait’s method uses `Self`, it is not
object-safe.”
