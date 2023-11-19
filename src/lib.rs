use rust_tokenizers::{
    tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy},
    vocab::{BertVocab, Vocab},
};

use libc::{c_char, size_t};
use std::{ffi::CStr, sync::OnceLock};

static TOKENIZER: OnceLock<BertTokenizer> = OnceLock::new();

#[repr(C)]
pub enum BertTokenizerInitStatus {
    BertTokenizerInitStatusOK = 0,
    BertTokenizerInitStatusCanNotConvertVocabPath,
    BertTokenizerInitStatusCanNotLoadVocab,
}

#[no_mangle]
pub extern "C" fn bert_tokenizer_init(
    vocab_path: *const c_char,
    lowercase: bool,
    strip_accents: bool,
) -> BertTokenizerInitStatus {
    let vocab_c_str: &CStr = unsafe { CStr::from_ptr(vocab_path) };
    let vocab_str: &str = match vocab_c_str.to_str() {
        Ok(v) => v,
        Err(_) => {
            return BertTokenizerInitStatus::BertTokenizerInitStatusCanNotConvertVocabPath;
        }
    };

    let vocab = match BertVocab::from_file(vocab_str) {
        Ok(v) => v,
        Err(_) => {
            return BertTokenizerInitStatus::BertTokenizerInitStatusCanNotLoadVocab;
        }
    };

    let _ = TOKENIZER.set(BertTokenizer::from_existing_vocab(
        vocab,
        lowercase,
        strip_accents,
    ));

    return BertTokenizerInitStatus::BertTokenizerInitStatusOK;
}

#[repr(C)]
pub enum BertTokenizerPreprocessingStatus {
    BertTokenizerPreprocessingStatusOK = 0,
    BertTokenizerPreprocessingStatusTokenizerWasNotInit,
    BertTokenizerPreprocessingStatusCanNotConvertText,
}

#[no_mangle]
pub extern "C" fn bert_tokenizer_process(
    text: *const c_char,
    token_id: *mut i64,
    token_type: *mut i64,
    mask_type: *mut i64,
    token_len: size_t,
) -> BertTokenizerPreprocessingStatus {
    let tk = match TOKENIZER.get() {
        Some(v) => v,
        None => {
            return BertTokenizerPreprocessingStatus::BertTokenizerPreprocessingStatusTokenizerWasNotInit;
        }
    };

    let c_str: &CStr = unsafe { CStr::from_ptr(text) };
    let str: &str = match c_str.to_str() {
        Ok(v) => v,
        Err(_) => {
            return BertTokenizerPreprocessingStatus::BertTokenizerPreprocessingStatusCanNotConvertText;
        }
    };

    let res = tk.encode(str, None, token_len, &TruncationStrategy::LongestFirst, 0);

    let last_token = std::cmp::min(res.token_ids.len(), token_len);

    let tokens = &res.token_ids[..last_token];

    const TOKEN_TYPE_DEFAULT: i64 = 0;
    const MASK_ATTENTION_ON: i64 = 1;
    const MASK_ATTENTION_OFF: i64 = 0;

    // fill tokenized
    for (i, t_id) in tokens.iter().enumerate() {
        let token_dst = unsafe { token_id.add(i) };
        let type_dst = unsafe { token_type.add(i) };
        let mask_dst = unsafe { mask_type.add(i) };

        unsafe { token_dst.write(*t_id) };
        unsafe { type_dst.write(TOKEN_TYPE_DEFAULT) };
        unsafe { mask_dst.write(MASK_ATTENTION_ON) };
    }

    // fill rest padding
    let vocab = tk.vocab();
    let pad_token = vocab.token_to_id(vocab.get_pad_value());

    for i in last_token..token_len {
        let token_dst = unsafe { token_id.add(i) };
        let type_dst = unsafe { token_type.add(i) };
        let mask_dst = unsafe { mask_type.add(i) };

        unsafe { token_dst.write(pad_token) };
        unsafe { type_dst.write(TOKEN_TYPE_DEFAULT) };
        unsafe { mask_dst.write(MASK_ATTENTION_OFF) };
    }

    return BertTokenizerPreprocessingStatus::BertTokenizerPreprocessingStatusOK;
}
