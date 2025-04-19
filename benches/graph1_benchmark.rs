use criterion::{criterion_group, criterion_main, Criterion};
use graph1::core::context::GraphContext;
use graph1::draw::tools::fill;
use graph1::test::mock_contexts::{get_mock_graph_context, MockUserData};
use graph1::utils::color::adapters;


fn rgba_to_0rgb_unsafe_bench(ctx_dst: &mut GraphContext<MockUserData>, ctx_src: &mut GraphContext<MockUserData>) {
    adapters::rgba_to_0rgb_unsafe(
        &mut ctx_dst.frame_buf,
        &mut ctx_src.frame_buf,
        false,
    );
}
fn rgba_to_0rgb_bench(ctx_dst: &mut GraphContext<MockUserData>, ctx_src: &mut GraphContext<MockUserData>) {
    adapters::rgba_to_0rgb(
        &mut ctx_dst.frame_buf,
        &mut ctx_src.frame_buf,
        1,
        false,
    );
}
fn benchmark_comparison(c: &mut Criterion) {
    // Test set-up
    let mut ctx_dst: GraphContext<MockUserData> = get_mock_graph_context(640, 480);
    let mut ctx_src: GraphContext<MockUserData> = get_mock_graph_context(640, 480);
    fill::buffer(&mut ctx_src.frame_buf, 0x33_44_55_ff,1);

    // Create a benchmark group
    let mut group = c.benchmark_group("Function Comparison");


    group.bench_function("rgba_to_0rgb_unsafe_640x480", |b| {
        b.iter(|| rgba_to_0rgb_unsafe_bench(&mut ctx_dst, &mut ctx_src));
    });

    group.bench_function("rgba_to_0rgb_640x480", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut ctx_dst, &mut ctx_src));
    });




    ctx_dst.resize(800, 600);
    ctx_src.resize(800, 600);
    group.bench_function("rgba_to_0rgb_unsafe_800x600", |b| {
        b.iter(|| rgba_to_0rgb_unsafe_bench(&mut ctx_dst, &mut ctx_src));
    });
    group.bench_function("rgba_to_0rgb_800x600", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut ctx_dst, &mut ctx_src));
    });



    ctx_dst.resize(1024, 768);
    ctx_src.resize(1024, 768);
    group.bench_function("rgba_to_0rgb_unsafe_1024x768", |b| {
        b.iter(|| rgba_to_0rgb_unsafe_bench(&mut ctx_dst, &mut ctx_src));
    });
    group.bench_function("rgba_to_0rgb_1024x768", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut ctx_dst, &mut ctx_src));
    });



    ctx_dst.resize(2048, 1536);
    ctx_src.resize(2048, 1536);
    group.bench_function("rgba_to_0rgb_unsafe_2048x1536);", |b| {
        b.iter(|| rgba_to_0rgb_unsafe_bench(&mut ctx_dst, &mut ctx_src));
    });
    group.bench_function("rgba_to_0rgb_2048x1536);", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut ctx_dst, &mut ctx_src));
    });


    ctx_dst.resize(4096, 3072);
    ctx_src.resize(4096, 3072);
    group.bench_function("rgba_to_0rgb_unsafe_4096x3072);", |b| {
        b.iter(|| rgba_to_0rgb_unsafe_bench(&mut ctx_dst, &mut ctx_src));
    });
    group.bench_function("rgba_to_0rgb_4096x3072);", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut ctx_dst, &mut ctx_src));
    });

    /*
    group.bench_function("rgba_to_0rgb", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut ctx_dst, &mut ctx_src));
    });
*/
    group.finish();

}

criterion_group!(benches, benchmark_comparison);
criterion_main!(benches);
