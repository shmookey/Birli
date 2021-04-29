use birli::{context_to_baseline_imgsets, cxx_aoflagger_new, flag_imgsets};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glob::glob;
use mwalib::CorrelatorContext;
use std::env;
use std::path::Path;

// fn get_context_mwax_full_1247842824() -> CorrelatorContext {
//     let test_dir = env::var("BIRLI_TEST_DIR").unwrap();
//     let test_path = Path::new(&test_dir);
//     let vis_path = test_path.join("1247842824_vis");
//     let metafits_path = vis_path
//         .join("1247842824.metafits")
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let gpufits_glob = vis_path
//         .join("1247842824_*gpubox*.fits")
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let gpufits_files: Vec<String> = glob(gpufits_glob.as_str())
//         .unwrap()
//         .filter_map(Result::ok)
//         .map(|path| path.to_str().unwrap().to_owned())
//         .collect();
//     CorrelatorContext::new(&metafits_path, &gpufits_files).unwrap()
// }

fn get_context_mwax_half_1247842824() -> CorrelatorContext {
    let test_dir = env::var("BIRLI_TEST_DIR").unwrap();
    let test_path = Path::new(&test_dir);
    let vis_path = test_path.join("1247842824_vis");
    let metafits_path = vis_path
        .join("1247842824.metafits")
        .to_str()
        .unwrap()
        .to_owned();
    let gpufits_glob = vis_path
        .join("1247842824_*gpubox*_00.fits")
        .to_str()
        .unwrap()
        .to_owned();
    let gpufits_files: Vec<String> = glob(gpufits_glob.as_str())
        .unwrap()
        .filter_map(Result::ok)
        .map(|path| path.to_str().unwrap().to_owned())
        .collect();
    CorrelatorContext::new(&metafits_path, &gpufits_files).unwrap()
}

// fn get_context_ord_full_1196175296() -> CorrelatorContext {
//     let test_dir = env::var("BIRLI_TEST_DIR").unwrap();
//     let test_path = Path::new(&test_dir);
//     let vis_path = test_path.join("1196175296_vis");
//     let metafits_path = vis_path
//         .join("1196175296.metafits")
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let gpufits_glob = vis_path
//         .join("1196175296_*gpubox*.fits")
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let gpufits_files: Vec<String> = glob(gpufits_glob.as_str())
//         .unwrap()
//         .filter_map(Result::ok)
//         .map(|path| path.to_str().unwrap().to_owned())
//         .collect();
//     CorrelatorContext::new(&metafits_path, &gpufits_files).unwrap()
// }

// fn get_context_ord_half_1196175296() -> CorrelatorContext {
//     let test_dir = env::var("BIRLI_TEST_DIR").unwrap();
//     let test_path = Path::new(&test_dir);
//     let vis_path = test_path.join("1196175296_vis");
//     let metafits_path = vis_path
//         .join("1196175296.metafits")
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let gpufits_glob = vis_path
//         .join("1196175296_*gpubox*_00.fits")
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let gpufits_files: Vec<String> = glob(gpufits_glob.as_str())
//         .unwrap()
//         .filter_map(Result::ok)
//         .map(|path| path.to_str().unwrap().to_owned())
//         .collect();
//     CorrelatorContext::new(&metafits_path, &gpufits_files).unwrap()
// }

// fn bench_context_to_baseline_imgsets_ord_half_1196175296(c: &mut Criterion) {
//     let aoflagger = unsafe { cxx_aoflagger_new() };
//     let context = get_context_ord_half_1196175296();
//     c.bench_function("context_to_baseline_imgsets", |b| {
//         b.iter(|| context_to_baseline_imgsets(black_box(&aoflagger), black_box(&context)))
//     });
// }

fn bench_context_to_baseline_imgsets_mwax_half_1247842824(c: &mut Criterion) {
    let aoflagger = unsafe { cxx_aoflagger_new() };
    let context = get_context_mwax_half_1247842824();
    c.bench_function("context_to_baseline_imgsets - mwax_half_1247842824", |b| {
        b.iter(|| context_to_baseline_imgsets(black_box(&aoflagger), black_box(&context)))
    });
}

fn bench_flag_imgsets_mwax_half_1247842824(c: &mut Criterion) {
    let aoflagger = unsafe { cxx_aoflagger_new() };
    let context = get_context_mwax_half_1247842824();
    let strategy_filename = &aoflagger.FindStrategyFileMWA();
    c.bench_function("flag_imgsets - mwax_half_1247842824", |b| {
        b.iter(|| {
            let baseline_imgsets = context_to_baseline_imgsets(&aoflagger, &context);
            flag_imgsets(
                black_box(&aoflagger),
                black_box(&strategy_filename),
                baseline_imgsets,
            )
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_context_to_baseline_imgsets_mwax_half_1247842824,
        bench_flag_imgsets_mwax_half_1247842824
);
criterion_main!(benches);
