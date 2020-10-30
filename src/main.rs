use rand::Rng;
use std::time::Instant;
mod tests;
mod conv;
use conv::*;


fn tests(
    testfn: &dyn Fn(&[f32], &[f32]) -> Vec<f32>,
    sample: &[f32],
    coeff: &[f32],
    testname: &str,
    ref0: Option<f32>,
) -> Vec<f32> {
    let start_time = Instant::now();
    let res = testfn(&sample, &coeff);
    let elapsed_time = start_time.elapsed();
    let milliseconds = (elapsed_time.as_secs() as f32 * 1000.0)
        + (elapsed_time.subsec_nanos() as f32 / 1_000_000.0);
    print!("{:22} elapsed: {:6.1} ms          ", testname, milliseconds);

    if let Some(sum0) = ref0 {
        //println!("len & sum ellenőrzés ...");
        assert_eq!(sample.len() - coeff.len() + 1, res.len());
        println!("diff: {}", sum0 - res.iter().sum::<f32>());
    } else {
        println!();
    }
    res
}

fn main() {
    let mut rng = rand::thread_rng();
    const COEFF_LEN: i32 = 256;
    let sample: Vec<_> = (0..4096 * 1000 + COEFF_LEN - 1)
        .map(|_| rng.gen::<f32>())
        .collect();
    let coeff: Vec<_> = (0..COEFF_LEN).map(|_| rng.gen::<f32>()).collect();

    println!("---");

    let res0 = tests(&tests::single_rs, &sample, &coeff, "test_single_rs", None);
    let ref0 = Some(res0.iter().sum());
    tests(
        &tests::single_rs_noiter,
        &sample,
        &coeff,
        "test_single_rs_noiter",
        ref0,
    );
    tests(
        &tests::splitted_rs,
        &sample,
        &coeff,
        "test_splitted_rs",
        ref0,
    );
    tests(&tests::rayontest_rs, &sample, &coeff, "test_rayon_rs", ref0);
    println!();
    tests(&tests::single_c, &sample, &coeff, "test_single_c", ref0);
    tests(&tests::splitted_c, &sample, &coeff, "test_splitted_c", ref0);
    tests(&tests::rayontest_c, &sample, &coeff, "test_rayon_c", ref0);
}
