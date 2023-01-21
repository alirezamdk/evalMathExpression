use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use evalmath::calculate::calc_postfix;
use evalmath::calculate::infix_to_postfix;
use evalmath::parse::parser::parse;

const BENCH_TEST_INPUT: &str = "1+2*3-4/5^2";

fn bench_parse(c: &mut Criterion) 
{
    let input = BENCH_TEST_INPUT;
    c.bench_function("parse", move |b| 
    {
        b.iter(|| 
        {
            parse(black_box(input));
        });
    });
}


fn bench_calculate(c: &mut Criterion) 
{
    let input = BENCH_TEST_INPUT;
    let parse = parse(input);
    let postfix = infix_to_postfix(&parse.unwrap());

    c.bench_function("postfix calculate", move |b| 
    {
        b.iter(|| 
        {
            calc_postfix(black_box(&postfix));
        });
    });
}

criterion_group!(benches, bench_parse, bench_calculate);
criterion_main!(benches);
