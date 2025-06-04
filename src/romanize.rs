use lindera::LinderaResult;
use lindera::dictionary::{DictionaryKind, load_dictionary_from_kind};
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use wana_kana::ConvertJapanese;

use crate::error::HttpError;

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
