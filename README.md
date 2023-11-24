# bert-tokenizer-c-cpp-bindings

This repository provides Rust bindings for a BERT (Bidirectional Encoder Representations from Transformers) tokenizer. The BERT tokenizer is a powerful tool for natural language processing and understanding.

### Features

- **Fast Tokenization**: Efficient tokenization of input text.
- **Rust API**: Rust API for seamless integration into Rust projects.
- **C/C++ Bindings**: Easily use the tokenizer in C/C++ projects.

## Getting Started

### Building the Library

- build:
```bash
cargo build --release
```

After bulid you can find next artifacts:
- header file `rustbert_tokenizer.h` will be placed into `target/include/`
- libraries will be available at`target/${duild_type}/`

### Integrate into C/CPP project
- add custom target to build rust dependency for cpp project - [example](https://github.com/globaltrouble/tenserflow-lite-demo/blob/87125a694b063b26b8bd4a45f0f21449d597b7e0/CMakeLists.txt#L12)
- add include directory with `rustbert_tokenizer.h` header to cpp - [example](https://github.com/globaltrouble/tenserflow-lite-demo/blob/87125a694b063b26b8bd4a45f0f21449d597b7e0/CMakeLists.txt#L18)
- add `rustbert_tokenizer` library - [example](https://github.com/globaltrouble/tenserflow-lite-demo/blob/87125a694b063b26b8bd4a45f0f21449d597b7e0/CMakeLists.txt#L19)
- add precompiled static or shared to imported library - [example](https://github.com/globaltrouble/tenserflow-lite-demo/blob/87125a694b063b26b8bd4a45f0f21449d597b7e0/CMakeLists.txt#L20)
- add dependency to build `rustbert_tokenizer` for your cpp project - [example](https://github.com/globaltrouble/tenserflow-lite-demo/blob/87125a694b063b26b8bd4a45f0f21449d597b7e0/CMakeLists.txt#L25)
- link your cpp binary with rust library - [example](https://github.com/globaltrouble/tenserflow-lite-demo/blob/87125a694b063b26b8bd4a45f0f21449d597b7e0/CMakeLists.txt#L28)

  
