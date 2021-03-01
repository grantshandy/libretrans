use clap::{crate_version, App, Arg, ArgMatches};
use libretranslate::{Translator, Language};
use colored::Colorize;

fn main() {
    // Set CLI application details through clap.
    let matches = App::new("libretrans")
        .version(crate_version!())
        .author("Grant Handy <grantshandy@gmail.com>")
        .about("Translates text from one language to another.")
        .arg(
            Arg::with_name("TEXT")
                .help("What text to translate.")
                .required(true)
                .takes_value(true)
                .index(2)
        )
        .arg(
            Arg::with_name("LANGUAGES")
                .help("Choose what languages to translate from.\n    Format: <INPUT>:<OUTPUT>\n    Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\"]\n")
                .required(true)
                .takes_value(true)
                .index(1)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("run with verbose output")
                .required(false)
                .takes_value(false)
        )
        .get_matches();

    let text = match matches.value_of("TEXT") {
        Some(data) => data,
        None => panic!("No value to TEXT..."),
    };

    if text == "" {
        trans_error("The following required arguments were not provided:", "<TEXT>", true);
        std::process::exit(1);
    };

    let languages: (&str, &str) = match matches.value_of("LANGUAGES") {
        Some(data) => {
            let langs = vec!["en", "ar", "zh", "fr", "de", "it", "pt", "ru", "es"];
            let possible_langs = "Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\"]";

            if data.chars().nth(2).unwrap() != ':' {
                trans_error("Malformed language argument: not separated by an ':'", possible_langs, false);
            };

            let split: Vec<&str> = data.split(':').collect();

            if split.len() != 2 {
                trans_error("Malformed language argument", possible_langs, false);
            };

            for x in &split {
                if !langs.contains(&x) {
                    trans_error("Unknown language", possible_langs, false);
                };
            };

            (split.get(0).unwrap(), split.get(1).unwrap())
        },
        None => panic!("No value for languages..."),
    };

    let input_lang = match_language(languages.0);
    let output_lang = match_language(languages.1);

    match Translator::translate(input_lang, output_lang, text) {
        Ok(data) => print_data(data, matches.clone()),
        Err(error) => trans_error("Translation request error", &error.to_string(), false),
    };
}

fn print_data(data: Translator, matches: ArgMatches) {
    if matches.is_present("verbose") {
        println!("{}: {}\n{}: {}", data.source.pretty(), data.input, data.target.pretty(), data.output);
    } else {
        println!("{}", data.output);
    }
}

fn match_language(lang: &str) -> Language {
    let language: Language = match lang {
        "en" => Language::English,
        "ar" => Language::Arabic,
        "zh" => Language::Chinese,
        "fr" => Language::French,
        "de" => Language::German,
        "it" => Language::Italian,
        "pt" => Language::Portuguese,
        "ru" => Language::Russain,
        "es" => Language::Spanish,
        &_ => panic!("Other value for lang..."),
    };

    return language;
}

fn trans_error(error: &str, details: &str, detail_red: bool) {
    eprintln!("{} {}", "error:".red().bold(), error);

    if detail_red {
        eprintln!("    {}\n", details.red().bold());
    } else {
        eprintln!("\n{}\n", details);
    }

    eprintln!("USAGE:\n    libretrans [FLAGS] <INPUT>:<OUTPUT> <TEXT>\n");
    eprintln!("For more information try {}", "--help".green());
    std::process::exit(1);
}
