use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prepona::storage::edge::{
    ArrKUniformHyperedge, CheckedFixedSizeMutEdgeDescriptor, EdgeDescriptor,
    FixedSizeMutEdgeDescriptor,
};

// --- ArrKUniformHyperedge - EdgeDescriptor
pub fn edge_descriptor_arr_k_uniform_hyperedge_get_sources(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function("edge_descriptor_arr_k_uniform_hyperedge_get_sources", |b| {
        b.iter(|| edge.get_sources())
    });
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_get_destinations(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_get_destinations",
        |b| b.iter(|| edge.get_destinations()),
    );
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_is_source_success(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_is_source_success",
        |b| b.iter(|| edge.is_source(black_box(&0))),
    );
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_is_source_fail(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_is_source_fail",
        |b| b.iter(|| edge.is_source(black_box(&100))),
    );
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_is_destination_success(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_is_destination_success",
        |b| b.iter(|| edge.is_destination(black_box(&0))),
    );
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_is_destination_fail(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_is_destination_fail",
        |b| b.iter(|| edge.is_destination(black_box(&100))),
    );
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_sources_count(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_sources_count",
        |b| b.iter(|| edge.sources_count()),
    );
}

pub fn edge_descriptor_arr_k_uniform_hyperedge_destinations_count(c: &mut Criterion) {
    let edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "edge_descriptor_arr_k_uniform_hyperedge_destinations_count",
        |b| b.iter(|| edge.destinations_count()),
    );
}

// --- ArrKUniformHyperedge - FixedEdgeDescriptor
pub fn fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src(c: &mut Criterion) {
    let mut edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src",
        |b| b.iter(|| edge.replace_src(black_box(&0), black_box(100))),
    );
}

pub fn fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst(c: &mut Criterion) {
    let mut edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst",
        |b| b.iter(|| edge.replace_dst(black_box(&1), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src_success(
    c: &mut Criterion,
) {
    let mut edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src_success",
        |b| b.iter(|| edge.replace_src_checked(black_box(&0), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src_fail(c: &mut Criterion) {
    let mut edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src_fail",
        |b| b.iter(|| edge.replace_src_checked(black_box(&100), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst_success(
    c: &mut Criterion,
) {
    let mut edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst_success",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&1), black_box(100))),
    );
}

pub fn checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst_fail(c: &mut Criterion) {
    let mut edge = ArrKUniformHyperedge::<i32, 100>::try_init(0..100).unwrap();

    c.bench_function(
        "checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst_fail",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&100), black_box(100))),
    );
}

criterion_group!(
    benches,
    // ArrKUniformHyperedge edge
    edge_descriptor_arr_k_uniform_hyperedge_get_sources,
    edge_descriptor_arr_k_uniform_hyperedge_get_destinations,
    edge_descriptor_arr_k_uniform_hyperedge_is_source_success,
    edge_descriptor_arr_k_uniform_hyperedge_is_source_fail,
    edge_descriptor_arr_k_uniform_hyperedge_is_destination_success,
    edge_descriptor_arr_k_uniform_hyperedge_is_destination_fail,
    edge_descriptor_arr_k_uniform_hyperedge_sources_count,
    edge_descriptor_arr_k_uniform_hyperedge_destinations_count,
    // ArrKUniformHyperedge - FixedEdgeDescriptor
    fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src,
    fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst,
    checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src_success,
    checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_src_fail,
    checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst_success,
    checked_fixed_edge_descriptor_arr_k_uniform_hyperedge_replace_dst_fail,
);

criterion_main!(benches);
