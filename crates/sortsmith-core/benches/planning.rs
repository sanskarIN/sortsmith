use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use sortsmith_core::{
    Rule, RuleAction, RuleCriterion, ScanOptions, find_duplicates, preview_organization,
};
use std::fs;
use std::hint::black_box;
use tempfile::TempDir;
use uuid::Uuid;

fn organization_rule() -> Rule {
    Rule {
        id: Uuid::new_v4(),
        name: "Text files".into(),
        enabled: true,
        match_all: true,
        criteria: vec![RuleCriterion::Extension {
            values: vec!["txt".into()],
        }],
        action: RuleAction::MoveTo {
            subdirectory: "Text".into(),
        },
    }
}

fn seed_planning_tree(count: usize) -> TempDir {
    let dir = tempfile::tempdir().expect("create benchmark directory");
    for index in 0..count {
        let extension = if index % 4 == 0 { "bin" } else { "txt" };
        fs::write(
            dir.path().join(format!("file-{index:06}.{extension}")),
            b"sortsmith benchmark",
        )
        .expect("seed benchmark file");
    }
    dir
}

fn seed_duplicate_tree(count: usize) -> TempDir {
    let dir = tempfile::tempdir().expect("create benchmark directory");
    for index in 0..count {
        let bucket = index % 25;
        let payload = format!("duplicate benchmark bucket {bucket:02}\n").repeat(128);
        fs::write(
            dir.path().join(format!("candidate-{index:06}.dat")),
            payload.as_bytes(),
        )
        .expect("seed duplicate benchmark file");
    }
    dir
}

fn benchmark_planning(c: &mut Criterion) {
    let options = ScanOptions {
        recursive: false,
        include_hidden: false,
        follow_links: false,
        max_depth: Some(32),
    };
    let rules = vec![organization_rule()];
    let mut group = c.benchmark_group("organization_planning");

    for count in [100usize, 1_000, 5_000] {
        let dir = seed_planning_tree(count);
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |bencher, _| {
            bencher.iter(|| {
                let preview = preview_organization(
                    black_box(dir.path()),
                    black_box(&rules),
                    black_box(&options),
                )
                .expect("planning benchmark should succeed");
                black_box(preview);
            });
        });
    }
    group.finish();
}

fn benchmark_duplicate_hashing(c: &mut Criterion) {
    let options = ScanOptions {
        recursive: false,
        include_hidden: false,
        follow_links: false,
        max_depth: Some(32),
    };
    let mut group = c.benchmark_group("duplicate_hashing");

    for count in [100usize, 1_000] {
        let dir = seed_duplicate_tree(count);
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |bencher, _| {
            bencher.iter(|| {
                let groups = find_duplicates(black_box(dir.path()), black_box(&options))
                    .expect("duplicate benchmark should succeed");
                black_box(groups);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_planning, benchmark_duplicate_hashing);
criterion_main!(benches);
