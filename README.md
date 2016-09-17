Ruroonga
===
[![Build Status](https://travis-ci.org/cosmo0920/ruroonga.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga)
[![](http://meritbadge.herokuapp.com/ruroonga)](https://crates.io/crates/ruroonga)
[![Build status](https://ci.appveyor.com/api/projects/status/jhibfy8rr7rtpv7h/branch/master?svg=true)](https://ci.appveyor.com/project/cosmo0920/ruroonga/branch/master)

[Documentation](http://cosmo0920.github.io/ruroonga/ruroonga/index.html)

A Rust lang binding for Groonga.

Rust + Groonga = Ruroonga!

### Target Rust Version

* 1.11.0 or later.

### Dependencies

* Groonga 6.0.8 or later.

### Japanese pronunciation

Ruroonga（るーるんが）

### For Windows

If you try to use this crate on Windows, please set the following environment variables:

```
* GROONGA_INCLUDE_DIR
* GROONGA_LIB_DIR
* GROONGA_BIN_DIR
```

If you install Groonga in `C:\Groonga`, above variables should set like this:

```powershell
PS> $Env:GROONGA_INCLUDE_DIR = "C:\Groonga\include"
PS> $Env:GROONGA_LIB_DIR = "C:\Groonga\lib"
PS> $Env:GROONGA_BIN_DIR = "C:\Groonga\bin"
```

For now, Groonga project does not provide MSVC compatible executables and binaries,
you should use `x86_64-pc-windows-gnu` target instead of `x86_64-pc-windows-msvc`.

If you use `rustup`, execute the following command:

```powershell
PS> rustup default stable-gnu
```

If you want to use `x86_64-pc-windows-msvc` target, please try to compile Groonga with MSVC by yourself.
In more detail, please refer to [the build from source section for Windows](http://groonga.org/docs/install/windows.html#build-from-source).

### LICENSE

#### Source code

[LGPL-2.1](LICENSE).

#### Slide

[CC BY-NC-SA 2.0](https://creativecommons.org/licenses/by-nc-sa/2.0/).
