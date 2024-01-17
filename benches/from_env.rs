use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[derive(serde::Deserialize)]
pub struct MyStruct {
    first_var: String,
    second_var: String,
}

fn from_env_bench(c: &mut Criterion) {
    temp_env::with_vars(
        [("FIRST_VAR", Some("Hello")), ("SECOND_VAR", Some("World!"))],
        || {
            c.bench_function("from_env", |b| {
                b.iter(|| black_box(serde_env::from_env::<MyStruct>()))
            });
        },
    );
}
criterion_group!(bench_from_env, from_env_bench);
criterion_main!(bench_from_env);
