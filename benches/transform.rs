#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate criterion;

use core::time::Duration;
use criterion::{black_box, BatchSize, BenchmarkGroup, Criterion, Throughput};
use rand::prelude::*;
use std::fmt;

const LEN: usize = 1_000_000;
const WARM_UP_TIME: Duration = Duration::from_secs(1);
const MEASUREMENT_TIME: Duration = Duration::from_secs(5);

use bevy::prelude::{GlobalTransform, Mat3, Mat4, Quat, Transform, Vec3, Vec4};

fn cmp(c: &mut Criterion) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[0]);

    let mut rng = rand::thread_rng();
    let trs = Transform {
        translation: Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        rotation: Quat::from_rotation_ypr(rng.gen(), rng.gen(), rng.gen()),
        scale: Vec3::new(rng.gen(), rng.gen(), rng.gen()),
    };

    let mat = trs.compute_matrix();

    let mut group = c.benchmark_group("conversion");
    group.bench_with_input("to_mat4", &trs, |b, data| {
        b.iter(|| black_box(data.compute_matrix()))
    });
    group.bench_with_input("from_mat4", &mat, |b, data| {
        b.iter(|| black_box(Transform::from_matrix(*data)))
    });
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();

    let mut group = c.benchmark_group("transform_point");
    group.bench_with_input("trs", &trs, |b, data| {
        b.iter(|| black_box(data.mul_vec3(Vec3::ONE)))
    });
    group.bench_with_input("mat4", &mat, |b, data| {
        b.iter(|| black_box(data.mul_vec4(Vec4::ONE)))
    });
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();

    let mut group = c.benchmark_group("inverse");
    group.bench_with_input("trs", &trs, |b, data| {
        b.iter(|| {
            black_box(
                Transform {
                    translation: -data.translation,
                    rotation: data.rotation.inverse(),
                    scale: data.scale.recip(),
                }
                .compute_matrix(),
            )
        })
    });
    group.bench_with_input("mat4", &mat, |b, data| b.iter(|| black_box(data.inverse())));
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();

    let mut group = c.benchmark_group("transform_propagation");
    group.bench_with_input("trs", &trs, |b, data| {
        b.iter(|| black_box(data.mul_transform(*data)))
    });
    group.bench_with_input("mat4", &mat, |b, data| {
        b.iter(|| black_box((*data) * (*data)))
    });
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();

    let mut group = c.benchmark_group("right_up_forward");
    group.bench_with_input("trs", &trs, |b, data| {
        b.iter(|| black_box(data.rotation * Vec3::X))
    });
    group.bench_with_input("mat4", &mat, |b, data| {
        b.iter(|| black_box(Vec3::from(data.x_axis).normalize()))
    });
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();

    let mut group = c.benchmark_group("any_direction");
    group.bench_with_input("trs", &trs, |b, data| {
        b.iter(|| black_box(data.rotation * Vec3::X))
    });
    group.bench_with_input("mat4", &mat, |b, data| {
        b.iter(|| black_box(data.transform_vector3(Vec3::X)))
    });
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.finish();
}

criterion_group!(benches, cmp);
criterion_main!(benches);
