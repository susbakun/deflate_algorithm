use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_FILE_ID: AtomicUsize = AtomicUsize::new(0);

#[test]
fn lz77_and_huffman_pipeline_restores_input_text() {
    let input = "Moby Dick: call me Ishmael. Call me Ishmael.".repeat(8);
    let tokens = lz77::compress(input.as_bytes());
    let symbols = lz77::tokens_to_symbols(&tokens);

    let id = NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("deflate-pipeline-{}-{id}.dflt", std::process::id()));
    let path_string = path.to_string_lossy().into_owned();

    huffman::encode(&symbols, &path_string);
    let decoded_symbols = huffman::decode(&path_string).unwrap();
    let decoded_tokens = lz77::symbols_to_tokens(&decoded_symbols);
    let restored = lz77::decompress(&decoded_tokens).unwrap();

    assert_eq!(restored, input);
    std::fs::remove_file(path).unwrap();
}
