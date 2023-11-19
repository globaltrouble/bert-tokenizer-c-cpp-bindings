use std::{
    env,
    ffi::{c_char, CString},
};

use rustbert_tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong arg list, usage: progname path/to/vocab.txt");
        std::process::exit(-1);
    }
    let p: &String = args.get(1).expect("No path specified");
    let path_cstr = CString::new(p.clone()).expect("can't convert path to cstr");
    let c_str: *const c_char = path_cstr.as_ptr() as *const c_char;
    rustbert_tokenizer::bert_tokenizer_init(c_str, true, true);

    let inpt = "This is a test";

    let inpt_cstr = CString::new(inpt).expect("can't convert inpt to cstr");
    let inpt = inpt_cstr.as_ptr() as *const c_char;

    const SIZE: usize = 512;
    let mut inpt_ids = Vec::new();
    inpt_ids.resize(SIZE, 0_i64);

    let mut inpt_types = Vec::new();
    inpt_types.resize(SIZE, 0_i64);

    let mut inpt_mask = Vec::new();
    inpt_mask.resize(SIZE, 0_i64);

    rustbert_tokenizer::bert_tokenizer_process(
        inpt,
        inpt_ids.as_mut_ptr(),
        inpt_types.as_mut_ptr(),
        inpt_mask.as_mut_ptr(),
        SIZE,
    );

    println!("inpt_ids: {:?}", inpt_ids);
    println!("inpt_types: {:?}", inpt_types);
    println!("inpt_mask: {:?}", inpt_mask);
    
}
