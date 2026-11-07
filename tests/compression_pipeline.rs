use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_FILE_ID: AtomicUsize = AtomicUsize::new(0);

use deflate::bucket;

#[test]
fn lz77_and_huffman_pipeline_restores_input_text() {
    let input = "Moby Dick: call me Ishmael. Call me Ishmael.".repeat(8);
    let tokens = lz77::compress(input.as_bytes());
    let syms = bucket::tokens_to_syms(&tokens);

    let id = NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("deflate-pipeline-{}-{id}.dflt", std::process::id()));
    let path_string = path.to_string_lossy().into_owned();

    huffman::encode(&syms, &path_string);
    let decoded_syms = huffman::decode(&path_string, bucket::extra_bits_for).unwrap();
    let decoded_tokens = bucket::syms_to_tokens(&decoded_syms).unwrap();
    let restored = lz77::decompress(&decoded_tokens);

    assert_eq!(restored, input.as_bytes());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn pipeline_restores_binary_data() {
    // every byte value, repeated, so there are both literals and matches
    let input: Vec<u8> = (0..=255u8).cycle().take(5000).collect();
    let tokens = lz77::compress(&input);
    let syms = bucket::tokens_to_syms(&tokens);

    let id = NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("deflate-binary-{}-{id}.dflt", std::process::id()));
    let path_string = path.to_string_lossy().into_owned();

    huffman::encode(&syms, &path_string);
    let decoded_syms = huffman::decode(&path_string, bucket::extra_bits_for).unwrap();
    let decoded_tokens = bucket::syms_to_tokens(&decoded_syms).unwrap();

    assert_eq!(lz77::decompress(&decoded_tokens), input);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn repeated_text_produces_matches() {
    let input = "Call me Ishmael. ".repeat(20);
    let tokens = lz77::compress(input.as_bytes());

    assert!(
        tokens
            .iter()
            .any(|t| matches!(t, lz77::Token::Match { .. }))
    );
}
