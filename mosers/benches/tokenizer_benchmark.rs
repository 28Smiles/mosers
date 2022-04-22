use criterion::{black_box, Criterion, criterion_group, criterion_main};
use mosers::{Language, MosesTokenizer};

pub fn tokenizer_benchmark(c: &mut Criterion) {
    let texts = [
        ("Machine Learning is great, isn\'t it?", "test_1"),
        ("This ain't funny. It's actually hillarious, yet double Ls. | [] < > [ ] & You're gonna shake it off? Don't?", "test_escape_xml"),
        ("By the mid 1990s a version of the game became a Latvian television series (with a parliamentary setting, and played by Latvian celebrities).", "test_opening_brackets"),
        ("The meeting will take place at 11:00 a.m. Tuesday.", "test_dot_splitting"),
    ];

    for (text, name) in texts {
        let tokenizer = MosesTokenizer::new(Language::En);
        let tkns = tokenizer.tokenize(text, Option::None);
        black_box(tkns.tokens());

        c.bench_function(name, |b| {
            b.iter(|| {
                let tkn = tokenizer.tokenize(text, Option::None);
                black_box(tkn.tokens());
            })
        });
    }
}

criterion_group!(benches, tokenizer_benchmark);
criterion_main!(benches);
