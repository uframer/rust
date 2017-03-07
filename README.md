# Rust程序设计语言

这里是[Rust][Rust]程序设计语言的源码仓库，包括编译器、标准库和文档。

[Rust]: https://www.rust-lang.org

## 快速上手

可以阅读[Rust程序设计语言][The Book]一书中[安装Rust]["Installing Rust"]这一节。

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

    > ***注意：*** 可以修改配置文件调整安装目录。将`./src/bootstrap/config.toml.example`复制到`./config.toml`，然后修改`[install]`下的`prefix`选项指定安装前缀。配置文件里还有很多选项可以配置，请参考配置文件中内置的说明。

    `sudo ./x.py dist --install`运行完成后，会在`/usr/local/bin`里添加几个程序：
    * `rustc`：Rust编译器
    * `rustdoc`：API文档的生成工具

    不过，这个命令不会给你安装Rust的包管理器[Cargo]，你需要单独构建它。

[Cargo]: https://github.com/rust-lang/cargo

### 在Windows上构建

Windows平台上有两种主要的ABI：Visual Studio使用的MSVC原生ABI，以及GCC工具链使用的GNU ABI。选择哪种版本的Rust取决于你依赖于哪些C/C++库：如果你主要同Visual Studio生成的软件交互，那么请选择MSVC构建的Rust；如果你主要同MinGW/MSYS2生成的GNU软件交互，那么请使用GNU工具链构建的Rust。

#### MinGW

可以用[MSYS2][msys2]轻松地在Windows上构建Rust：

[msys2]: https://msys2.github.io/

1. 从[这里][msys2]下载最新的安装程序并安装。

2. 从你的MSYS2安装目录（例如，`C:\msys64`）运行`mingw32_shell.bat`或`mingw64_shell.bat`（取决于你要用32位还是64位的Rust）。如果你用的是最新版的MSYS2，可能要运行`msys2_shell.cmd -mingw32`或`msys2_shell.cmd -mingw64`。

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

MSVC版本的Rust还需要你在系统里安装Visual Studio 2013（或更新版本），`rustc`需要使用它提供的链接器。请注意在安装时勾选`C++ tools`选项。

安装完这些依赖关系之后，你就可以在`cmd.exe`命令行窗口中运行构建命令了：

```sh
> python x.py build
```

目前，你只能使用特定版本的Visual Studio中编译Rust。如果系统中的VS版本太新以至于Rust的构建系统还没有支持的话，你可能需要强制要求`rustbuild`使用一个旧版本的VS，做法是先调用合适版本的vcvars批处理文件：

```
CALL "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64\vcvars64.bat"
python x.py build
```

#### 如何指定一种ABI

只要指明编译三元组，是可以在一种ABI的运行环境中编译生成另一种ABI的目标文件的，例如，你可以在PowerShell中编译出GNU ABI的目标文件。可用的Windows编译三元组包括：
- GNU ABI（使用GCC）
    - `i686-pc-windows-gnu`
    - `x86_64-pc-windows-gnu`
- MSVC ABI
    - `i686-pc-windows-msvc`
    - `x86_64-pc-windows-msvc`

可以用`x.py`的`--build=ABI`选项指定编译三元组，或者也可以如前面介绍过的那样修改`config.toml`文件中的`build`选项（在`[build]`部分）。

### `configure`和`make`

尽管不推荐，我们还提供了configure脚本和makefile（其实它只是简单地调用`x.py`）。

```sh
$ ./configure
$ make && sudo make install
```

如果使用configure脚本，那么生成的config.mk文件会优先于`config.toml`文件。如果想回头使用`config.toml`文件，请删除生成的`config.mk`文件。

## 如何构建文档

运行如下命令构建文档：

```sh
$ ./x.py doc
```

生成的文档会出现在最上层的`doc`目录。

## 备注

由于Rust编译器是使用Rust开发的，所以你需要一个已经编译好的*快照*版本的Rust编译器才能编译Rust编译器自身。正因如此，你需要联网才能执行构建动作，构建脚本会先去网上下载一个可用的快照版本编译器。

目前我们在如下平台上构建并测试Rust编译器的快照：

| 平台 / 架构                     | x86 | x86_64 |
|--------------------------------|-----|--------|
| Windows (7, 8, Server 2008 R2) | ✓   | ✓      |
| Linux (2.6.18 or later)        | ✓   | ✓      |
| OSX (10.7 Lion or later)       | ✓   | ✓      |

其他的平台可能也会有，但是我们官方支持的平台只有这几个。

现在你需要600MiB到1.5GiB的内存来构建Rust，具体的大小可能随着平台变化。如果用到了交换空间，那需要的时间可就长了。

[贡献力量][CONTRIBUTING.md]一节中更为详细地介绍了如何hack Rust的建议。

[CONTRIBUTING.md]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md

## 如何获得帮助

Rust社区主要集中在下面几个地方：

* [Stack Overflow] - 直接针对语言用法的问题。
* [users.rust-lang.org] - 一般讨论和相关问题。
* [/r/rust] - 新闻和综合讨论。

[Stack Overflow]: http://stackoverflow.com/questions/tagged/rust
[/r/rust]: http://reddit.com/r/rust
[users.rust-lang.org]: https://users.rust-lang.org/

## 如何做出贡献

如果想要为Rust贡献一份力量，请阅读[CONTRIBUTING](CONTRIBUTING.md)的介绍。

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
