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
            Arg::with_name("INPUT>:<OUTPUT")
                .help("Choose what languages to translate from.\n    Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\", \"ja\"]\n")
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
        trans_error("The following required arguments were not provided:", "<TEXT>", text, matches.clone());
    };

    let languages: (&str, &str) = match matches.value_of("INPUT>:<OUTPUT") {
        Some(data) => {
            let langs = vec!["en", "ar", "zh", "fr", "de", "it", "pt", "ru", "es", "ja"];
            let possible_langs = "Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\", \"ja\"]";

            if data.chars().nth(2).unwrap() != ':' {
                trans_error("Malformed language argument: not separated by an ':'", possible_langs, data, matches.clone());
            };

            let split: Vec<&str> = data.split(':').collect();

            if split.len() != 2 {
                trans_error("Malformed language argument", possible_langs, data, matches.clone());
            };

            for x in &split {
                if !langs.contains(&x) {
                    trans_error("Unknown languages", possible_langs, data, matches.clone());
                };
            };

            (split.get(0).unwrap(), split.get(1).unwrap())
        },
        None => panic!("No value for languages..."),
    };

    let input_lang = Language::from(languages.0).unwrap();
    let output_lang = Language::from(languages.1).unwrap();

    match Translator::translate(input_lang, output_lang, text) {
        Ok(data) => print_data(data, matches.clone()),
        Err(error) => trans_error("Translation request error", &error.to_string(), text, matches.clone()),
    };
}

fn print_data(data: Translator, matches: ArgMatches) {
    if matches.is_present("verbose") {
        println!("{}: \"{}\"\n{}: \"{}\"", data.source.pretty().green().bold(), data.input, data.target.pretty().green().bold(), data.output);
    } else {
        println!("{}", data.output);
    }
}

fn trans_error(error: &str, details: &str, text: &str, matches: ArgMatches) {
    eprintln!("{} {}", "error:".red().bold(), error);

    if text == "" {
        eprintln!("    {}\n", details.red().bold());
    } else {
        eprintln!("\n{}\n", details);
    };
    
    if matches.is_present("verbose") {
        eprint!("USAGE:\n    libretrans <INPUT>:<OUTPUT> <TEXT> --verbose\n\n");
    } else {
        eprint!("USAGE:\n    libretrans [FLAGS] <INPUT>:<OUTPUT> <TEXT>");
    };

    eprintln!("For more information try {}", "--help".green());
    std::process::exit(1);
}
