# 宏

Rust中有些特性没有重要到需要为它们设计语法，但是用函数又不能直接实现。我们给这些功能指定独立的名字`some_extension`，然后使用`some_extension!(...)`这样的语法调用它们。

`rustc`的用户可以通过两种不同的方式定义宏：

* [样板宏][Macros] 通过一种高级的声明式语法定义新功能。
* [过程宏][Procedural Macros] 用于实现自定义派生。

此外，unstable版本的Rust还支持[编译器插件][compiler plugins]。

[Macros]: ../book/macros.html
[Procedural Macros]: ../book/procedural-macros.html
[compiler plugins]: ../unstable-book/plugin.html
