// reference
pub fn single_rs(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    super::conv_rs(sample, coeff)
}

pub fn single_rs_noiter(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    super::conv_rs_noiter(sample, coeff)
}

// single C
pub fn single_c(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    super::conv_c(sample, coeff)
}

// splitted, single thread
pub fn splitted_rs(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut blocklist = vec![];
    for i in (0..=sample.len() - coeff.len()).step_by(4096) {
        blocklist.push(&sample[i..i + 4096 + coeff.len() - 1]);
    }

    let mut res = vec![];
    res.extend(blocklist.iter().flat_map(|s| super::conv_rs(s, &coeff)));
    res
}

pub fn splitted_c(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut blocklist = vec![];
    for i in (0..=sample.len() - coeff.len()).step_by(4096) {
        blocklist.push(&sample[i..i + 4096 + coeff.len() - 1]);
    }

    let mut res = vec![];
    res.extend(blocklist.iter().flat_map(|s| super::conv_c(s, &coeff)));
    res
}

// splitted, parallelize with rayon
use rayon::prelude::*;

pub fn rayontest_rs(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut blocklist = vec![];
    for i in (0..=sample.len() - coeff.len()).step_by(4096) {
        blocklist.push(&sample[i..i + 4096 + coeff.len() - 1]);
    }

    let mut res = vec![];
    res.extend(
        blocklist
            .par_iter()
            .flat_map(|s| super::conv_rs(s, &coeff))
            .collect::<Vec<_>>(),
    );
    res
}

pub fn rayontest_c(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut blocklist = vec![];
    for i in (0..=sample.len() - coeff.len()).step_by(4096) {
        blocklist.push(&sample[i..i + 4096 + coeff.len() - 1]);
    }

    let mut res = vec![];
    res.extend(
        blocklist
            .par_iter()
            .flat_map(|s| super::conv_c(s, &coeff))
            .collect::<Vec<_>>(),
    );
    res
}
