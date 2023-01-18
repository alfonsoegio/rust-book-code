# rust-book-code

Main first goal is to adapt a [game](https://github.com/fr3ising/fgng) implemented
using SDL2 in C++ to a 100% Rust implementation using [https://crates.io/crates/sdl2](https://crates.io/crates/sdl2)
SDL2 bindings in Rust.

[Implementation directory](./game/)

## Current Screenshot

![Screenshot](./game.png)


## How to setup Emacs

### Install rust-analyzer

- [How to](https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary)

```
$ mkdir -p ~/.local/bin
$ curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.local/bin/rust-analyzer
$ chmod +x ~/.local/bin/rust-analyzer
```

Make sure that `~/.local/bin` is listed in the $PATH variable:

```
export PATH="$PATH:~/.local/bin"
```

In `~/.bashrc`

- [Add sources manually](https://rust-analyzer.github.io/manual.html#installation)

```
$ rustup component add rust-src
```


Add to `~/.emacs` needed packages after installing using `M-x package-install`

```
(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)

(add-hook 'rust-mode-hook 'eglot-ensure)
(add-hook 'rust-mode-hook #'lsp)

(custom-set-variables
...
    '(package-selected-packages
    '(magit lsp-mode company-racer emojify rustic racer cargo))
...
)
```

# rust-book-code
