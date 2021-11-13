use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prepona::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, DirectedEdge, EdgeDescriptor, FixedSizeMutEdgeDescriptor,
    UndirectedEdge,
};

// --- Undirected edge - EdgeDescriptor
pub fn edge_descriptor_edge_undirected_get_sources(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_get_sources", |b| {
        b.iter(|| edge.get_sources())
    });
}

pub fn edge_descriptor_edge_undirected_get_destinations(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_get_destinations", |b| {
        b.iter(|| edge.get_destinations())
    });
}

pub fn edge_descriptor_edge_undirected_is_source_success(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_is_source_success", |b| {
        b.iter(|| edge.is_source(black_box(&0)))
    });
}

pub fn edge_descriptor_edge_undirected_is_source_fail(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_is_source_fail", |b| {
        b.iter(|| edge.is_source(black_box(&2)))
    });
}

pub fn edge_descriptor_edge_undirected_is_destination_success(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function(
        "edge_descriptor_edge_undirected_is_destination_success",
        |b| b.iter(|| edge.is_destination(black_box(&0))),
    );
}

pub fn edge_descriptor_edge_undirected_is_destination_fail(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_is_destination_fail", |b| {
        b.iter(|| edge.is_destination(black_box(&2)))
    });
}

pub fn edge_descriptor_edge_undirected_sources_count(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_sources_count", |b| {
        b.iter(|| edge.sources_count())
    });
}

pub fn edge_descriptor_edge_undirected_destinations_count(c: &mut Criterion) {
    let edge = UndirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_undirected_destinations_count", |b| {
        b.iter(|| edge.destinations_count())
    });
}

// --- Undirected edge - Fixed edge descriptor
pub fn fixed_edge_descriptor_edge_undirected_replace_src(c: &mut Criterion) {
    let mut edge = UndirectedEdge::init(0, 1);

    c.bench_function("fixed_edge_descriptor_edge_undirected_replace_src", |b| {
        b.iter(|| edge.replace_src(black_box(&0), black_box(2)))
    });
}

pub fn fixed_edge_descriptor_edge_undirected_replace_dst(c: &mut Criterion) {
    let mut edge = UndirectedEdge::init(0, 1);

    c.bench_function("fixed_edge_descriptor_edge_undirected_replace_dst", |b| {
        b.iter(|| edge.replace_dst(black_box(&1), black_box(2)))
    });
}

pub fn checked_fixed_edge_descriptor_edge_undirected_replace_src_success(c: &mut Criterion) {
    let mut edge = UndirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_undirected_replace_src_success",
        |b| b.iter(|| edge.replace_src_checked(black_box(&0), black_box(2))),
    );
}

pub fn checked_fixed_edge_descriptor_edge_undirected_replace_src_fail(c: &mut Criterion) {
    let mut edge = UndirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_undirected_replace_src_fail",
        |b| b.iter(|| edge.replace_src_checked(black_box(&3), black_box(2))),
    );
}

pub fn checked_fixed_edge_descriptor_edge_undirected_replace_dst_success(c: &mut Criterion) {
    let mut edge = UndirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_undirected_replace_dst_success",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&1), black_box(2))),
    );
}

pub fn checked_fixed_edge_descriptor_edge_undirected_replace_dst_fail(c: &mut Criterion) {
    let mut edge = UndirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_undirected_replace_dst_fail",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&3), black_box(2))),
    );
}

// --- Directed edge
pub fn edge_descriptor_edge_directed_get_sources(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_get_sources", |b| {
        b.iter(|| edge.get_sources())
    });
}

pub fn edge_descriptor_edge_directed_get_destinations(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_get_destinations", |b| {
        b.iter(|| edge.get_destinations())
    });
}

pub fn edge_descriptor_edge_directed_is_source_success(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_is_source_success", |b| {
        b.iter(|| edge.is_source(black_box(&0)))
    });
}

pub fn edge_descriptor_edge_directed_is_source_fail(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_is_source_fail", |b| {
        b.iter(|| edge.is_source(black_box(&1)))
    });
}

pub fn edge_descriptor_edge_directed_is_destination_success(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function(
        "edge_descriptor_edge_directed_is_destination_success",
        |b| b.iter(|| edge.is_destination(black_box(&1))),
    );
}

pub fn edge_descriptor_edge_directed_is_destination_fail(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_is_destination_fail", |b| {
        b.iter(|| edge.is_destination(black_box(&0)))
    });
}

pub fn edge_descriptor_edge_directed_sources_count(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_sources_count", |b| {
        b.iter(|| edge.sources_count())
    });
}

pub fn edge_descriptor_edge_directed_destinations_count(c: &mut Criterion) {
    let edge = DirectedEdge::init(0, 1);

    c.bench_function("edge_descriptor_edge_directed_destinations_count", |b| {
        b.iter(|| edge.destinations_count())
    });
}

// Directed edge - FixedSizeEdgeDescriptor
pub fn fixed_edge_descriptor_edge_directed_replace_src(c: &mut Criterion) {
    let mut edge = DirectedEdge::init(0, 1);

    c.bench_function("fixed_edge_descriptor_edge_directed_replace_src", |b| {
        b.iter(|| edge.replace_src(black_box(&0), black_box(2)))
    });
}

pub fn fixed_edge_descriptor_edge_directed_replace_dst(c: &mut Criterion) {
    let mut edge = DirectedEdge::init(0, 1);

    c.bench_function("fixed_edge_descriptor_edge_directed_replace_dst", |b| {
        b.iter(|| edge.replace_dst(black_box(&1), black_box(2)))
    });
}

pub fn checked_fixed_edge_descriptor_edge_directed_replace_src_success(c: &mut Criterion) {
    let mut edge = DirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_directed_replace_src_success",
        |b| b.iter(|| edge.replace_src_checked(black_box(&0), black_box(2))),
    );
}

pub fn checked_fixed_edge_descriptor_edge_directed_replace_src_fail(c: &mut Criterion) {
    let mut edge = DirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_directed_replace_src_fail",
        |b| b.iter(|| edge.replace_src_checked(black_box(&3), black_box(2))),
    );
}

pub fn checked_fixed_edge_descriptor_edge_directed_replace_dst_success(c: &mut Criterion) {
    let mut edge = DirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_directed_replace_dst_success",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&1), black_box(2))),
    );
}

pub fn checked_fixed_edge_descriptor_edge_directed_replace_dst_fail(c: &mut Criterion) {
    let mut edge = DirectedEdge::init(0, 1);

    c.bench_function(
        "checked_fixed_edge_descriptor_edge_directed_replace_dst_fail",
        |b| b.iter(|| edge.replace_dst_checked(black_box(&3), black_box(2))),
    );
}

criterion_group!(
    benches,
    // Undirected edge
    edge_descriptor_edge_undirected_get_sources,
    edge_descriptor_edge_undirected_get_destinations,
    edge_descriptor_edge_undirected_is_source_success,
    edge_descriptor_edge_undirected_is_source_fail,
    edge_descriptor_edge_undirected_is_destination_success,
    edge_descriptor_edge_undirected_is_destination_fail,
    edge_descriptor_edge_undirected_sources_count,
    edge_descriptor_edge_undirected_destinations_count,
    // Undirected edge - FixedEdgeDescriptor
    fixed_edge_descriptor_edge_undirected_replace_src,
    fixed_edge_descriptor_edge_undirected_replace_dst,
    checked_fixed_edge_descriptor_edge_undirected_replace_src_success,
    checked_fixed_edge_descriptor_edge_undirected_replace_src_fail,
    checked_fixed_edge_descriptor_edge_undirected_replace_dst_success,
    checked_fixed_edge_descriptor_edge_undirected_replace_dst_fail,
    // Directed edge
    edge_descriptor_edge_directed_get_sources,
    edge_descriptor_edge_directed_get_destinations,
    edge_descriptor_edge_directed_is_source_success,
    edge_descriptor_edge_directed_is_source_fail,
    edge_descriptor_edge_directed_is_destination_success,
    edge_descriptor_edge_directed_is_destination_fail,
    edge_descriptor_edge_directed_sources_count,
    edge_descriptor_edge_directed_destinations_count,
    // Directed edge - FixedEdgeDescriptor
    fixed_edge_descriptor_edge_directed_replace_src,
    fixed_edge_descriptor_edge_directed_replace_dst,
    checked_fixed_edge_descriptor_edge_directed_replace_src_success,
    checked_fixed_edge_descriptor_edge_directed_replace_src_fail,
    checked_fixed_edge_descriptor_edge_directed_replace_dst_success,
    checked_fixed_edge_descriptor_edge_directed_replace_dst_fail,
);

criterion_main!(benches);
