pub fn conv_rs(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut res = vec![];
    for i in 0..=sample.len() - coeff.len() {
        let tmp = sample[i..].iter().zip(coeff).map(|(&s, &c)| s * c).sum();
        res.push(tmp);
    }
    res
}

pub fn conv_rs_noiter(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
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

// ---- C module with FFI -----

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
