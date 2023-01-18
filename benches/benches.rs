use criterion::{criterion_group, criterion_main, Criterion};
use dispatchers::*;

fn native(c: &mut Criterion) {
    c.bench_function("native", |b| b.iter(|| native::run()));
}

fn treewalk(c: &mut Criterion) {
    let code = treewalk::code();
    c.bench_function("treewalk", |b| b.iter(|| treewalk::run(&code)));
}

fn compact_treewalk_dtable(c: &mut Criterion) {
    let code = compact_treewalk_dtable::code();
    c.bench_function("compact treewalk (dtable)", |b| {
        b.iter(|| compact_treewalk_dtable::run(&code))
    });
}

fn compact_treewalk_switch(c: &mut Criterion) {
    let code = compact_treewalk_switch::code();
    c.bench_function("compact treewalk (switch)", |b| {
        b.iter(|| compact_treewalk_switch::run(&code))
    });
}

fn stack_dtable(c: &mut Criterion) {
    let code = stack_dtable::code();
    c.bench_function("stack (dtable)", |b| b.iter(|| stack_dtable::run(&code)));
}

fn stack_switch(c: &mut Criterion) {
    let code = stack_switch::code();
    c.bench_function("stack (switch)", |b| b.iter(|| stack_switch::run(&code)));
}

fn register_dtable(c: &mut Criterion) {
    let code = register_dtable::code();
    c.bench_function("register (dtable)", |b| {
        b.iter(|| register_dtable::run(&code))
    });
}

fn register_switch(c: &mut Criterion) {
    let code = register_switch::code();
    c.bench_function("register (switch)", |b| {
        b.iter(|| register_switch::run(&code))
    });
}

criterion_group!(
    benches,
    native,
    treewalk,
    compact_treewalk_dtable,
    compact_treewalk_switch,
    stack_dtable,
    stack_switch,
    register_dtable,
    register_switch,
);
criterion_main!(benches);
