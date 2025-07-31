use criterion::{criterion_group, criterion_main, Criterion};
use std::{collections::BTreeMap, hint::black_box};
use unleash_types::{
    client_metrics::{ImpactMetric, ImpactMetricEnv, MetricSample, MetricType},
    MergeMut,
};

fn generate_labels(start: usize, end: usize) -> BTreeMap<String, String> {
    (start..end)
        .map(|i| (format!("label_{}", i), format!("value_{}", i)))
        .collect()
}

pub fn bench_merge(c: &mut Criterion) {
    let first = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric {
            name: "test_metric".to_string(),
            help: "something".to_string(),
            r#type: MetricType::Counter,
            samples: vec![
                MetricSample {
                    value: 1.0,
                    labels: Some(generate_labels(0, 100)),
                },
                MetricSample {
                    value: 2.0,
                    labels: Some(generate_labels(100, 200)),
                },
            ],
        },
    };

    let second = ImpactMetricEnv {
        app_name: "test_app".to_string(),
        environment: "development".to_string(),
        impact_metric: ImpactMetric {
            name: "test_metric".to_string(),
            help: "something".to_string(),
            r#type: MetricType::Counter,
            samples: vec![
                MetricSample {
                    value: 3.0,
                    labels: Some(generate_labels(50, 150)),
                },
                MetricSample {
                    value: 4.0,
                    labels: Some(generate_labels(250, 350)),
                },
            ],
        },
    };

    c.bench_function("labels_to_key", |b| {
        b.iter(|| {
            let mut first_clone = first.clone();
            let second_clone = second.clone();
            first_clone.merge(second_clone);
            black_box(());
        })
    });
}

criterion_group!(benches, bench_merge);
criterion_main!(benches);
