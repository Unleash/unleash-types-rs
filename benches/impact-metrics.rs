use criterion::{criterion_group, criterion_main, Criterion};
use std::{collections::BTreeMap, hint::black_box};
use unleash_types::{
    client_metrics::{
        Bucket, BucketMetricSample, ImpactMetric, ImpactMetricEnv, NumericMetricSample,
    },
    MergeMut,
};

fn generate_labels(start: usize, end: usize) -> BTreeMap<String, String> {
    (start..end)
        .map(|i| (format!("label_{}", i), format!("value_{}", i)))
        .collect()
}

pub fn bench_merge(c: &mut Criterion) {
    // Test counter merging with different labels
    let first_counter = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric::Counter {
            name: "test_metric".to_string(),
            help: "something".to_string(),
            samples: vec![
                NumericMetricSample {
                    value: 1.0,
                    labels: Some(generate_labels(0, 100)),
                },
                NumericMetricSample {
                    value: 2.0,
                    labels: Some(generate_labels(100, 200)),
                },
            ],
        },
    };

    let second_counter = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric::Counter {
            name: "test_metric".to_string(),
            help: "something".to_string(),
            samples: vec![
                NumericMetricSample {
                    value: 3.0,
                    labels: Some(generate_labels(50, 150)),
                },
                NumericMetricSample {
                    value: 4.0,
                    labels: Some(generate_labels(250, 350)),
                },
            ],
        },
    };

    // Test histogram merging with matching buckets (fast path) and many labels
    // Using the same labels for both histograms to test the fast path
    let common_labels = generate_labels(0, 100);
    
    let first_histogram = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric::Histogram {
            name: "test_histogram".to_string(),
            help: "histogram metric".to_string(),
            samples: vec![
                BucketMetricSample {
                    labels: Some(common_labels.clone()),
                    count: 100,
                    sum: 250.0,
                    buckets: vec![
                        Bucket { le: 0.1, count: 10 },
                        Bucket { le: 0.5, count: 30 },
                        Bucket { le: 1.0, count: 60 },
                        Bucket { le: 5.0, count: 90 },
                        Bucket { le: f64::INFINITY, count: 100 },
                    ],
                },
            ],
        },
    };

    let second_histogram = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric::Histogram {
            name: "test_histogram".to_string(),
            help: "histogram metric".to_string(),
            samples: vec![
                BucketMetricSample {
                    labels: Some(common_labels.clone()),
                    count: 50,
                    sum: 125.0,
                    buckets: vec![
                        Bucket { le: 0.1, count: 5 },
                        Bucket { le: 0.5, count: 15 },
                        Bucket { le: 1.0, count: 30 },
                        Bucket { le: 5.0, count: 45 },
                        Bucket { le: f64::INFINITY, count: 50 },
                    ],
                },
            ],
        },
    };

    c.bench_function("counter_merge", |b| {
        b.iter(|| {
            let mut first_clone = first_counter.clone();
            let second_clone = second_counter.clone();
            black_box(first_clone.merge(second_clone));
        })
    });

    c.bench_function("histogram_merge_fast_path", |b| {
        b.iter(|| {
            let mut first_clone = first_histogram.clone();
            let second_clone = second_histogram.clone();
            black_box(first_clone.merge(second_clone));
        })
    });
}

criterion_group!(benches, bench_merge);
criterion_main!(benches);
