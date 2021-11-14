use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prepona::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, EdgeDescriptor, FixedSizeMutEdgeDescriptor, HashedHyperedge,
    MutEdgeDescriptor,
};

// --- HashedHyperedge - EdgeDescriptor
pub fn edge_descriptor_hashed_hyperedge_get_sources(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function("edge_descriptor_hashed_hyperedge_get_sources", |b| {
        b.iter(|| edge.get_sources())
    });
}

pub fn edge_descriptor_hashed_hyperedge_get_destinations(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function("edge_descriptor_hashed_hyperedge_get_destinations", |b| {
        b.iter(|| edge.get_destinations())
    });
}

pub fn edge_descriptor_hashed_hyperedge_is_source_success(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function("edge_descriptor_hashed_hyperedge_is_source_success", |b| {
        b.iter(|| edge.is_source(black_box(&0)))
    });
}

pub fn edge_descriptor_hashed_hyperedge_is_source_fail(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function("edge_descriptor_hashed_hyperedge_is_source_fail", |b| {
        b.iter(|| edge.is_source(black_box(&100)))
    });
}

pub fn edge_descriptor_hashed_hyperedge_is_destination_success(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "edge_descriptor_hashed_hyperedge_is_destination_success",
        |b| b.iter(|| edge.is_destination(black_box(&0))),
    );
}

pub fn edge_descriptor_hashed_hyperedge_is_destination_fail(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "edge_descriptor_hashed_hyperedge_is_destination_fail",
        |b| b.iter(|| edge.is_destination(black_box(&100))),
    );
}

pub fn edge_descriptor_hashed_hyperedge_sources_count(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function("edge_descriptor_hashed_hyperedge_sources_count", |b| {
        b.iter(|| edge.sources_count())
    });
}

pub fn edge_descriptor_hashed_hyperedge_destinations_count(c: &mut Criterion) {
    let edge = HashedHyperedge::init(0..100);

    c.bench_function("edge_descriptor_hashed_hyperedge_destinations_count", |b| {
        b.iter(|| edge.destinations_count())
    });
}

// --- HashedHyperedge - FixedEdgeDescriptor
pub fn fixed_edge_descriptor_hashed_hyperedge_replace_src(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("fixed_edge_descriptor_hashed_hyperedge_replace_src", |b| {
        b.iter(|| edge.replace_src(black_box(&0), black_box(100)))
    });
}

pub fn fixed_edge_descriptor_hashed_hyperedge_replace_dst(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("fixed_edge_descriptor_hashed_hyperedge_replace_dst", |b| {
        b.iter(|| edge.replace_dst(black_box(&1), black_box(100)))
    });
}

pub fn checked_fixed_edge_descriptor_hashed_hyperedge_replace_src_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_fixed_edge_descriptor_hashed_hyperedge_replace_src_success",
        |b| b.iter(|| edge.replace_src_checked(black_box(&0), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_hashed_hyperedge_replace_src_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_fixed_edge_descriptor_hashed_hyperedge_replace_src_fail",
        |b| b.iter(|| edge.replace_src_checked(black_box(&100), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_hashed_hyperedge_replace_dst_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_fixed_edge_descriptor_hashed_hyperedge_replace_dst_success",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&1), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_hashed_hyperedge_replace_dst_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_fixed_edge_descriptor_hashed_hyperedge_replace_dst_fail",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&100), black_box(100))),
    );
}

// HashedHyperedge - MutEdgeDescriptor
pub fn mut_edge_descriptor_hashed_hyperedge_add_loop(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add_loop", |b| {
        b.iter(|| edge.add(100, 100))
    });
}

pub fn mut_edge_descriptor_hashed_hyperedge_add(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add", |b| {
        b.iter(|| edge.add(100, 101))
    });
}

pub fn mut_edge_descriptor_hashed_hyperedge_add_src(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add_src", |b| {
        b.iter(|| edge.add_src(100))
    });
}

pub fn mut_edge_descriptor_hashed_hyperedge_add_dst(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add_dst", |b| {
        b.iter(|| edge.add_dst(100))
    });
}

pub fn mut_edge_descriptor_hashed_hyperedge_remove(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_remove", |b| {
        b.iter(|| edge.remove(&0))
    });
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_loop_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_mut_edge_descriptor_hashed_hyperedge_add_loop_success",
        |b| b.iter(|| edge.add(100, 100)),
    );
}
pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_loop_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_mut_edge_descriptor_hashed_hyperedge_add_loop_fail",
        |b| b.iter(|| edge.add(0, 0)),
    );
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "checked_mut_edge_descriptor_hashed_hyperedge_add_success",
        |b| b.iter(|| edge.add(100, 101)),
    );
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add", |b| {
        b.iter(|| edge.add(0, 1))
    });
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_src_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "mut_edge_descriptor_hashed_hyperedge_add_src_success",
        |b| b.iter(|| edge.add_src(100)),
    );
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_src_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add_src_fail", |b| {
        b.iter(|| edge.add_src(0))
    });
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_dst_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function(
        "mut_edge_descriptor_hashed_hyperedge_add_dst_success",
        |b| b.iter(|| edge.add_dst(100)),
    );
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_add_dst_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_add_dst_fail", |b| {
        b.iter(|| edge.add_dst(0))
    });
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_remove_success(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_remove_success", |b| {
        b.iter(|| edge.remove(&0))
    });
}

pub fn checked_mut_edge_descriptor_hashed_hyperedge_remove_fail(c: &mut Criterion) {
    let mut edge = HashedHyperedge::init(0..100);

    c.bench_function("mut_edge_descriptor_hashed_hyperedge_remove_fail", |b| {
        b.iter(|| edge.remove(&100))
    });
}

criterion_group!(
    benches,
    // HashedHyperedge edge
    edge_descriptor_hashed_hyperedge_get_sources,
    edge_descriptor_hashed_hyperedge_get_destinations,
    edge_descriptor_hashed_hyperedge_is_source_success,
    edge_descriptor_hashed_hyperedge_is_source_fail,
    edge_descriptor_hashed_hyperedge_is_destination_success,
    edge_descriptor_hashed_hyperedge_is_destination_fail,
    edge_descriptor_hashed_hyperedge_sources_count,
    edge_descriptor_hashed_hyperedge_destinations_count,
    // HashedHyperedge - FixedEdgeDescriptor
    fixed_edge_descriptor_hashed_hyperedge_replace_src,
    fixed_edge_descriptor_hashed_hyperedge_replace_dst,
    checked_fixed_edge_descriptor_hashed_hyperedge_replace_src_success,
    checked_fixed_edge_descriptor_hashed_hyperedge_replace_src_fail,
    checked_fixed_edge_descriptor_hashed_hyperedge_replace_dst_success,
    checked_fixed_edge_descriptor_hashed_hyperedge_replace_dst_fail,
    // HashedHyperedge - MutEdgeDescriptor
    mut_edge_descriptor_hashed_hyperedge_add_loop,
    mut_edge_descriptor_hashed_hyperedge_add,
    mut_edge_descriptor_hashed_hyperedge_add_src,
    mut_edge_descriptor_hashed_hyperedge_add_dst,
    mut_edge_descriptor_hashed_hyperedge_remove,
    checked_mut_edge_descriptor_hashed_hyperedge_add_loop_success,
    checked_mut_edge_descriptor_hashed_hyperedge_add_loop_fail,
    checked_mut_edge_descriptor_hashed_hyperedge_add_success,
    checked_mut_edge_descriptor_hashed_hyperedge_add_fail,
    checked_mut_edge_descriptor_hashed_hyperedge_add_src_success,
    checked_mut_edge_descriptor_hashed_hyperedge_add_src_fail,
    checked_mut_edge_descriptor_hashed_hyperedge_add_dst_success,
    checked_mut_edge_descriptor_hashed_hyperedge_add_dst_fail,
    checked_mut_edge_descriptor_hashed_hyperedge_remove_success,
    checked_mut_edge_descriptor_hashed_hyperedge_remove_fail
);

criterion_main!(benches);
