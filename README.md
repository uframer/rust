# Rust程序设计语言

这里是[Rust][Rust]程序设计语言的源码仓库，包括编译器、标准库和文档。

[Rust]: https://www.rust-lang.org

## 快速上手

可以阅读[The Book][The Book]中["Installing Rust"]["Installing Rust"]部分。

["Installing Rust"]: https://doc.rust-lang.org/book/getting-started.html#installing-rust
[The Book]: https://doc.rust-lang.org/book/index.html

## 从源码构建

1. 确保你安装了如下所需软件：

   * `g++` 4.7或更高版本，或者`clang++` 3.x
   * `python` 2.7（不支持3.x版本）
   * GNU `make` 3.81或更高版本
   * `cmake` 3.4.3或更高版本
   * `curl`
   * `git`

2. 用`git`从[GitHub][source]克隆源码：

   ```sh
   $ git clone https://github.com/rust-lang/rust.git
   $ cd rust
   ```

[source]: https://github.com/rust-lang/rust

3. 构建并安装：

    ```sh
    $ ./x.py build && sudo ./x.py dist --install
    ```

    > ***注意：***可以修改配置文件调整安装目录。将`./src/bootstrap/config.toml.example`复制到`./config.toml`，然后修改`[install]`下的`prefix`选项指定安装前缀。配置文件里还有很多选项可以配置，请参考配置文件中内置的说明。

    `sudo ./x.py dist --install`运行完成后，会在`/usr/local/bin`里添加几个程序：
    * `rustc`：Rust编译器
    * `rustdoc`：API文档的生成工具

    不过，这个命令不会给你安装Rust的包管理器[Cargo]，你需要单独构建它。

[Cargo]: https://github.com/rust-lang/cargo

### 在Windows上构建

Windows平台上有两种主要的ABI：Visual Studio使用的MSVC原生ABI，以及GCC工具链使用的GNU ABI。Which version of Rust
you need depends largely on what C/C++ libraries you want to interoperate with:
for interop with software produced by Visual Studio use the MSVC build of Rust;
for interop with GNU software built using the MinGW/MSYS2 toolchain use the GNU
build.

#### MinGW

可以用[MSYS2][msys2]轻松地在Windows上构建Rust：

[msys2]: https://msys2.github.io/

1. 从[这里][msys2]下载最新的安装程序并安装。

2. Run `mingw32_shell.bat` or `mingw64_shell.bat` from wherever you installed
   MSYS2 (i.e. `C:\msys64`), depending on whether you want 32-bit or 64-bit
   Rust. (As of the latest version of MSYS2 you have to run `msys2_shell.cmd
   -mingw32` or `msys2_shell.cmd -mingw64` from the command line instead)

3. 在终端中安装所需的工具：

   ```sh
   # Update package mirrors (may be needed if you have a fresh install of MSYS2)
   $ pacman -Sy pacman-mirrors

   # Install build tools needed for Rust. If you're building a 32-bit compiler,
   # then replace "x86_64" below with "i686". If you've already got git, python,
   # or CMake installed and in PATH you can remove them from this list. Note
   # that it is important that you do **not** use the 'python2' and 'cmake'
   # packages from the 'msys2' subsystem. The build has historically been known
   # to fail with these packages.
   $ pacman -S git \
               make \
               diffutils \
               tar \
               mingw-w64-x86_64-python2 \
               mingw-w64-x86_64-cmake \
               mingw-w64-x86_64-gcc
   ```

4. 进入Rust的源码目录，然后配置并构建：

   ```sh
   $ ./x.py build && ./x.py dist --install
   ```

#### MSVC

MSVC builds of Rust additionally require an installation of Visual Studio 2013
(or later) so `rustc` can use its linker. Make sure to check the “C++ tools”
option.

With these dependencies installed, you can build the compiler in a `cmd.exe`
shell with:

```sh
> python x.py build
```

Currently building Rust only works with some known versions of Visual Studio. If
you have a more recent version installed the build system doesn't understand
then you may need to force rustbuild to use an older version. This can be done
by manually calling the appropriate vcvars file before running the bootstrap.

```
CALL "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64\vcvars64.bat"
python x.py build
```

#### 如何指定一种ABI

Each specific ABI can also be used from either environment (for example, using
the GNU ABI in powershell) by using an explicit build triple. The available
Windows build triples are:
- GNU ABI (using GCC)
    - `i686-pc-windows-gnu`
    - `x86_64-pc-windows-gnu`
- The MSVC ABI
    - `i686-pc-windows-msvc`
    - `x86_64-pc-windows-msvc`

The build triple can be specified by either specifying `--build=ABI` when
invoking `x.py` commands, or by copying the `config.toml` file (as described
in Building From Source), and modifying the `build` option under the `[build]`
section.

### `configure`和`make`

While it's not the recommended build system, this project also provides a
configure script and makefile (the latter of which just invokes `x.py`).

```sh
$ ./configure
$ make && sudo make install
```

When using the configure script, the generated config.mk` file may override the
`config.toml` file. To go back to the `config.toml` file, delete the generated
`config.mk` file.

## 如何构建文档

运行如下命令构建文档：

```sh
$ ./x.py doc
```

生成的文档会出现在最上层的`doc`目录。

## 备注

Since the Rust compiler is written in Rust, it must be built by a
precompiled "snapshot" version of itself (made in an earlier state of
development). As such, source builds require a connection to the Internet, to
fetch snapshots, and an OS that can execute the available snapshot binaries.

Snapshot binaries are currently built and tested on several platforms:

| Platform / Architecture        | x86 | x86_64 |
|--------------------------------|-----|--------|
| Windows (7, 8, Server 2008 R2) | ✓   | ✓      |
| Linux (2.6.18 or later)        | ✓   | ✓      |
| OSX (10.7 Lion or later)       | ✓   | ✓      |

You may find that other platforms work, but these are our officially
supported build environments that are most likely to work.

Rust currently needs between 600MiB and 1.5GiB to build, depending on platform.
If it hits swap, it will take a very long time to build.

There is more advice about hacking on Rust in [CONTRIBUTING.md].

[CONTRIBUTING.md]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md

## 如何获得帮助

Rust社区主要集中在下面几个地方：

* [Stack Overflow] - 直接针对语言用法的问题。
* [users.rust-lang.org] - General discussion and broader questions.
* [/r/rust] - 新闻和综合讨论。

[Stack Overflow]: http://stackoverflow.com/questions/tagged/rust
[/r/rust]: http://reddit.com/r/rust
[users.rust-lang.org]: https://users.rust-lang.org/

## 如何做出贡献

To contribute to Rust, please see [CONTRIBUTING](CONTRIBUTING.md).

Rust has an [IRC] culture and most real-time collaboration happens in a
variety of channels on Mozilla's IRC network, irc.mozilla.org. The
most popular channel is [#rust], a venue for general discussion about
Rust. And a good place to ask for help would be [#rust-beginners].

[IRC]: https://en.wikipedia.org/wiki/Internet_Relay_Chat
[#rust]: irc://irc.mozilla.org/rust
[#rust-beginners]: irc://irc.mozilla.org/rust-beginners

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
