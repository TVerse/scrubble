use criterion::{black_box, criterion_group, criterion_main, Criterion};
use game::bitboard::Bitboard;
use game::bitboard::BitboardImpl;

pub fn count_ones(c: &mut Criterion) {
    c.bench_function("count_ones", |b| {
        b.iter(|| black_box(BitboardImpl::full()).count_ones())
    });
}

pub fn shift_up(c: &mut Criterion) {
    c.bench_function("shift_up", |b| {
        b.iter(|| black_box(BitboardImpl::full()).up(1))
    });
}

pub fn shift_down(c: &mut Criterion) {
    c.bench_function("shift_down", |b| {
        b.iter(|| black_box(BitboardImpl::full()).down(1))
    });
}

pub fn shift_left(c: &mut Criterion) {
    c.bench_function("shift_left", |b| {
        b.iter(|| black_box(BitboardImpl::full()).left(1))
    });
}

pub fn shift_right(c: &mut Criterion) {
    c.bench_function("shift_right", |b| {
        b.iter(|| black_box(BitboardImpl::full()).right(1))
    });
}

pub fn and(c: &mut Criterion) {
    c.bench_function("and", |b| {
        b.iter(|| black_box(BitboardImpl::full()) & black_box(BitboardImpl::full()))
    });
}

pub fn or(c: &mut Criterion) {
    c.bench_function("or", |b| {
        b.iter(|| black_box(BitboardImpl::full()) | black_box(BitboardImpl::full()))
    });
}

pub fn not(c: &mut Criterion) {
    c.bench_function("not", |b| b.iter(|| !black_box(BitboardImpl::full())));
}

criterion_group!(
    bitboard,
    count_ones,
    shift_up,
    shift_down,
    shift_left,
    shift_right,
    and,
    or,
    not,
);
criterion_main!(bitboard);
