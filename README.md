# libretrans
A command line tool for translating using the libretranslate API

## Command Line Arguments

```
libretrans 0.1.2
Grant Handy <grantshandy@gmail.com>
Translates text from one language to another

USAGE:
    libretrans [FLAGS] <LANGUAGES> <TEXT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    run with verbose output

ARGS:
    <LANGUAGES>    choose what languages to translate from
    <TEXT>         what text to translate
```

Note: use `<LANGUAGES>` in this format: `<INPUT>:<OUTPUT>`. See the example for better understanding.

## Examples
Basic:
```
$ libretrans en:fr "Hello World"
Bonjour à tous
```

With verbose tag:
```
$ libretrans -v fr:en "C'est la vie!"
French: C'est la vie!
English: It's life!
```
