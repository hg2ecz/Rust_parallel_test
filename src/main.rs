use rand::Rng;
use std::time::Instant;
mod tests;

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

fn conv_rs(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut res = vec![];
    for i in 0..=sample.len() - coeff.len() {
        let mut tmp = 0.;
        let window = &sample[i..i + coeff.len()];
        for j in 0..coeff.len() {
            // tmp += sample[i + j] * coeff[j]; // 3x slower
            tmp += window[j] * coeff[j];
        }
        res.push(tmp);
    }
    res
}

#[link(name = "conv", kind = "static")]
extern "C" {
    fn conv_cc(
        output: *mut f32,
        outlen: *mut i32,
        sample: *const f32,
        samplelen: i32,
        coeff: *const f32,
        coefflen: i32,
    );
}

pub fn conv_c(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut output: Vec<f32> = Vec::with_capacity(sample.len());
    let mut outlen: i32 = 0;
    unsafe {
        conv_cc(
            output.as_mut_ptr(),
            &mut outlen,
            sample.as_ptr(),
            sample.len() as i32,
            coeff.as_ptr(),
            coeff.len() as i32,
        );
        output.set_len(outlen as usize);
    }
    output
}

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
    print!("{:20} elapsed: {:6.1} ms          ", testname, milliseconds);

    if let Some(sum0) = ref0 {
        //println!("len & sum ellenőrzés ...");
        assert_eq!(sample.len() - coeff.len() + 1, res.len());
        println!("diff: {}", sum0 - res.iter().sum::<f32>());
    } else {
        println!();
    }
    res
}
