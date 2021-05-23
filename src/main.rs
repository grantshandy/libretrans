use clap::{crate_version, App, Arg};
use libretranslate::{translate, translate_url, Language};

mod tools;

#[async_std::main]
async fn main() {
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
                .help("Choose what languages to translate from.\n    Possible values: [\"en\", \"ar\", \"zh\", \"fr\", \"de\", \"it\", \"pt\", \"ru\", \"es\", \"ja\"]\n    Tip: You can format languages like \":<OUTPUT>\" to detect the input language.\n")
                .required(true)
                .takes_value(true)
                .index(1)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Run with verbose output.")
                .required(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("url")
                .long("url")
                .short("u")
                .help("What libretranslate instance to translate from.")
                .required(false)
                .takes_value(true)
        )
        .get_matches();

    // Get our text from clap.
    let text = match matches.value_of("TEXT") {
        Some(data) => data,
        None => panic!("No value to TEXT..."),
    };

    // If we have no input then just exit :/ 
    if text == "" {
        std::process::exit(0);
    };

    // Get our languages from get_languages() in tools.
    let (input, output): (Language, Language) = match matches.value_of("INPUT>:<OUTPUT") {
        Some(data) => tools::get_languages(data, matches.clone()),
        None => panic!("No value for languages..."),
    };

    // Translate it, then send the data or error off to their respective functions.
    match matches.value_of("url") {
        Some(data) => {
            match translate_url(input, output, text, data).await {
                Ok(data) => tools::print_data(data, matches.clone()),
                Err(error) => tools::trans_error("Translation error:", &error.to_string(), text, matches.clone()),
            };
        },
        None => {
            match translate(input, output, text).await {
                Ok(data) => tools::print_data(data, matches.clone()),
                Err(error) => tools::trans_error("Translation error:", &error.to_string(), text, matches.clone()),
            };
        },
    }
}