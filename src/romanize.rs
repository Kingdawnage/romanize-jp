use lindera::LinderaResult;
use lindera::dictionary::{DictionaryKind, load_dictionary_from_kind};
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use wana_kana::ConvertJapanese;

use crate::error::HttpError;

pub fn _test_lindera() -> LinderaResult<()> {
    let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)?;

    let segmenter = Segmenter::new(Mode::Normal, dictionary, None);

    // Create a tokenizer.
    let tokenizer = Tokenizer::new(segmenter);

    // Tokenize a text.
    let text = "迷宮攻略のための最適な戦略を考える。";
    let mut tokens = tokenizer.tokenize(text)?;

    // Print the text and tokens.
    println!("text:\t{}", text);

    let mut romaji_tokens = Vec::new();

    for token in tokens.iter_mut() {
        // let token_text = token.text.clone().into_owned();

        let token_details = token.details();
        // Print the token text and its details.
        // let details = token_details.join(",");
        // println!("token:\t{}\t{}", token_text, details);

        // Get the last field of the details, which is usually the part of speech.
        let last_field = token_details.last().unwrap_or(&"");

        // Convert to romaji
        let romaji = ConvertJapanese::to_romaji(*last_field);
        // Print the romaji conversion.
        // println!("romaji:\t{}", romaji);

        romaji_tokens.push(romaji);
    }

    let romanized = romaji_tokens.join(" ");
    // Print the romanized text.
    println!("romanized:\t{}", romanized);
    Ok(())
}

pub fn romanize_text(text: &str) -> Result<String, HttpError> {
    let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)
        .map_err(|e| HttpError::server_error(format!("Failed to load dictionary: {}", e)))?;

    let segmenter = Segmenter::new(Mode::Normal, dictionary, None);

    let tokenizer = Tokenizer::new(segmenter);

    let mut tokens = tokenizer
        .tokenize(text)
        .map_err(|e| HttpError::server_error(format!("Tokenization failed: {}", e)))?;

    let mut romaji_tokens = Vec::new();

    for token in tokens.iter_mut() {
        let token_details = token.details();
        let last_field = token_details.last().unwrap_or(&"");

        // Convert to romaji
        let romaji = ConvertJapanese::to_romaji(*last_field);

        romaji_tokens.push(romaji);
    }

    let romanized_sentence = romaji_tokens.join(" ");

    Ok(romanized_sentence)
}
