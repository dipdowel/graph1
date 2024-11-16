use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use graph1::core::context::GraphContext;
use graph1::draw::tools::fill;
use graph1::test::mock_contexts::{get_mock_graph_context, MockUserData};
use graph1::utils::color::adapters;
use std::fmt;



struct BenchContexts {
    ctx_src: GraphContext<MockUserData>,
    ctx_dst: GraphContext<MockUserData>,
}

impl fmt::Display for BenchContexts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the desired string representation to the formatter
        write!(
            f,
            "ctx_dst: {:?}, ctx_src: {:?}",
            self.ctx_dst, self.ctx_dst
        )
    }
}

fn rgba_to_0rgb_unsafe_bench(data: &mut BenchContexts) {
    adapters::rgba_to_0rgb_unsafe(
        &mut data.ctx_dst.frame_buf,
        &mut data.ctx_src.frame_buf,
        false,
    );
}
fn rgba_to_0rgb_bench(data: &mut BenchContexts) {
    adapters::rgba_to_0rgb(
        &mut data.ctx_dst.frame_buf,
        &mut data.ctx_src.frame_buf,
        false,
    );
}
fn benchmark_comparison(c: &mut Criterion) {
    // Test set-up
    let mut bench_contexts: BenchContexts = BenchContexts {
        ctx_src: get_mock_graph_context(1024,768),
        ctx_dst: get_mock_graph_context(1024,768),
    };
    fill::buffer(&mut bench_contexts.ctx_src.frame_buf, 0x33_44_55_ff);

    // Create a benchmark group
    let mut group = c.benchmark_group("Function Comparison");


    group.bench_function("rgba_to_0rgb_unsafe", |b| {
        b.iter(|| rgba_to_0rgb_unsafe_bench(&mut bench_contexts));
    });

    group.bench_function("rgba_to_0rgb", |b| {
        b.iter(|| rgba_to_0rgb_bench(&mut bench_contexts));
    });

    group.finish();

}

criterion_group!(benches, benchmark_comparison);
criterion_main!(benches);
