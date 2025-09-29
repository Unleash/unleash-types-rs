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

fn generate_buckets(count: usize, multiplier: u64) -> Vec<Bucket> {
    (0..count)
        .map(|i| Bucket {
            le: (i as f64) * 0.1,
            count: i as u64 * multiplier,
        })
        .collect()
}

pub fn bench_merge(c: &mut Criterion) {
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

    let bucket_count = 100;
    let buckets1 = generate_buckets(bucket_count, 10);
    let buckets2 = generate_buckets(bucket_count, 5);
    let common_labels = generate_labels(0, 100);

    let first_histogram = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric::Histogram {
            name: "test_histogram".to_string(),
            help: "histogram metric".to_string(),
            samples: vec![BucketMetricSample {
                labels: Some(common_labels.clone()),
                count: 1000,
                sum: 2500.0,
                buckets: buckets1,
            }],
        },
    };

    let second_histogram = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric::Histogram {
            name: "test_histogram".to_string(),
            help: "histogram metric".to_string(),
            samples: vec![BucketMetricSample {
                labels: Some(common_labels),
                count: 500,
                sum: 1250.0,
                buckets: buckets2,
            }],
        },
    };

    c.bench_function("counter_merge", |b| {
        b.iter(|| {
            let mut first_clone = first_counter.clone();
            let second_clone = second_counter.clone();
            black_box(first_clone.merge(second_clone));
        })
    });

    c.bench_function("histogram_merge_many_buckets_fast_path", |b| {
        b.iter(|| {
            let mut first_clone = first_histogram.clone();
            let second_clone = second_histogram.clone();
            black_box(first_clone.merge(second_clone));
        })
    });
}

criterion_group!(benches, bench_merge);
criterion_main!(benches);
