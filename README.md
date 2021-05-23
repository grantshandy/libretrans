# libretrans
A command line tool for translating using the libretranslate API

## Examples
Basic usage:
```
$ libretrans en:fr "Hello World"
Bonjour à tous
```

A verbose flag can also be used:
```
$ libretrans -v fr:en "C'est la vie!"
French: "C'est la vie!"
English: "It's life!"
```

As of right now, libretranslate.com needs a key while they upgrade, so you can input a custom url to another instance:
```
$ libretrans -u https://libretranslate.de en:es "This is a test"
Esta es una prueba
```

## Compiling From Source
Cargo/Rust is required to build.

Just install it to your cargo path like so:
```
$ cargo install --path="."
```

Or just install from crates.io:
```
cargo install libretrans
```

## Command Line Arguments
```
libretrans 0.1.8
Grant Handy <grantshandy@gmail.com>
Translates text from one language to another.

USAGE:
    libretrans [FLAGS] [OPTIONS] <INPUT>:<OUTPUT> <TEXT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Run with verbose output.

OPTIONS:
    -u, --url <url>    What libretranslate instance to translate from.

ARGS:
    <INPUT>:<OUTPUT>    Choose what languages to translate from.
                            Possible values: ["en", "ar", "zh", "fr", "de", "it", "pt", "ru", "es", "ja"]
                            Tip: You can format languages like ":<OUTPUT>" to detect the input language.
    <TEXT>              What text to translate.
```
