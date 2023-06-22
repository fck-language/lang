use std::io::Read;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lang_macros::languages;
use lang_inner::compress::UStream;
use lang_inner::{LanguageRaw, Table};

languages!(en);

macro_rules! samples {
    ($($f:ident),*$(,)?) => {$(
        fn $f(c: &mut Criterion) {
            let buf = Vec::new();
            let l = &en::LANG;
            let m: (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>) = (&en::MAP.0, &en::MAP.1, &en::MAP.2);
            c.bench_function(
                stringify!($f),
                |b| b.iter(
                    || lang::tokenize(include_str!(concat!("../tests/sample scripts/", stringify!($f), ".fck")).bytes(), l, &buf, m)
                )
            );
        }
    )*
    criterion_group!(benches, $($f),*);
    criterion_main!(benches);
    };
}

samples!{test1, test2, test3}
