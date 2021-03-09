# libretrans
A command line tool for translating using the libretranslate API

## Examples
Basic usage:
```
$ libretrans en:fr "Hello World"
Bonjour à tous
```

The verbose flag can also be used:
```
$ libretrans -v fr:en "C'est la vie!"
French: "C'est la vie!"
English: "It's life!"
```

## Compiling From Source
Cargo/Rust is required to build.

Just install it to your cargo path like so:
```
$ cargo install --path="."
```

## Command Line Arguments
```
libretrans 0.1.4
Grant Handy <grantshandy@gmail.com>
Translates text from one language to another.

USAGE:
    libretrans [FLAGS] <INPUT>:<OUTPUT> <TEXT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    run with verbose output

ARGS:
    <INPUT>:<OUTPUT>    Choose what languages to translate from.
                            Possible values: ["en", "ar", "zh", "fr", "de", "it", "pt", "ru", "es"]
    <TEXT>              What text to translate.
```
