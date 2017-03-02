# 样板宏

`macro_rules`允许用户用声明式的语法定义语法扩展。我们把这种语法扩展形式叫做*样板宏**（macros by example）*，或者简称为*宏*。

目前为止，宏可以展开为表达式、语句、项目（item）或者模式。

> `sep_token`是一个除了`*`或`+`之外的记号；`non_special_token`是一个除了分隔符或`$`之外的记号。

宏展开器会根据名字选择宏调用，然后依次尝试匹配这个宏定义的每一条规则，并根据匹配上的第一条规则转写宏内容。匹配和转写紧密相关，我们会一起介绍。

宏展开器匹配并转写所有不以`$`开头的记号，包括分隔符。由于解析器的原因，分隔符必须要配对，但是除了这一要求，它们就没有什么特别的了。

In the matcher, `$` _name_ `:` _designator_ matches the nonterminal in the Rust
syntax named by _designator_. Valid designators are:

* `item`: an [item]
* `block`: a [block]
* `stmt`: a [statement]
* `pat`: a [pattern]
* `expr`: an [expression]
* `ty`: a [type]
* `ident`: an [identifier]
* `path`: a [path]
* `tt`: a token tree (a single [token] by matching `()`, `[]`, or `{}`)
* `meta`: the contents of an [attribute]

[item]: items.html
[block]: expressions.html#block-expressions
[statement]: statements.html
[pattern]: expressions.html#match-expressions
[expression]: expressions.html
[type]: types.html
[identifier]: identifiers.html
[path]: paths.html
[token]: tokens.html
[attribute]: attributes.html

In the transcriber, the
designator is already known, and so only the name of a matched nonterminal comes
after the dollar sign.

In both the matcher and transcriber, the Kleene star-like operator indicates
repetition. The Kleene star operator consists of `$` and parentheses, optionally
followed by a separator token, followed by `*` or `+`. `*` means zero or more
repetitions, `+` means at least one repetition. The parentheses are not matched or
transcribed. On the matcher side, a name is bound to _all_ of the names it
matches, in a structure that mimics the structure of the repetition encountered
on a successful match. The job of the transcriber is to sort that structure
out.

The rules for transcription of these repetitions are called "Macro By Example".
Essentially, one "layer" of repetition is discharged at a time, and all of them
must be discharged by the time a name is transcribed. Therefore, `( $( $i:ident
),* ) => ( $i )` is an invalid macro, but `( $( $i:ident ),* ) => ( $( $i:ident
),*  )` is acceptable (if trivial).

When Macro By Example encounters a repetition, it examines all of the `$`
_name_ s that occur in its body. At the "current layer", they all must repeat
the same number of times, so ` ( $( $i:ident ),* ; $( $j:ident ),* ) => ( $(
($i,$j) ),* )` is valid if given the argument `(a,b,c ; d,e,f)`, but not
`(a,b,c ; d,e)`. The repetition walks through the choices at that layer in
lockstep, so the former input transcribes to `(a,d), (b,e), (c,f)`.

Nested repetitions are allowed.

### 解析器的限制

宏系统的解析器已经比较强大了，但是Rust的语法解析依然受到两种限制：

1. Macro definitions are required to include suitable separators after parsing
   expressions and other bits of the Rust grammar. This implies that
   a macro definition like `$i:expr [ , ]` is not legal, because `[` could be part
   of an expression. A macro definition like `$i:expr,` or `$i:expr;` would be legal,
   however, because `,` and `;` are legal separators. See [RFC 550] for more information.
2. The parser must have eliminated all ambiguity by the time it reaches a `$`
   _name_ `:` _designator_. This requirement most often affects name-designator
   pairs when they occur at the beginning of, or immediately after, a `$(...)*`;
   requiring a distinctive token in front can solve the problem.

[RFC 550]: https://github.com/rust-lang/rfcs/blob/master/text/0550-macro-future-proofing.md
