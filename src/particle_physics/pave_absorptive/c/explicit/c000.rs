use super::{log_diff, Parameters};

pub(crate) fn c000(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-1)
            * log_diff(
                param.m0_2 * (param.s1 + param.s12 - param.s2)
                    + param.m1_2 * (param.s1 + param.s2 - param.s12)
                    + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                param.lambda_m01_sqrt * param.lambda_s12_sqrt,
            )
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-1)
            * log_diff(
                (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                    + param.m2_2 * (param.s1 + param.s2 - param.s12)
                    + param.m0_2 * (param.s12 + param.s2 - param.s1),
                param.lambda_m02_sqrt * param.lambda_s12_sqrt,
            )
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-1)
            * log_diff(
                param.m2_2 * (param.s1 + param.s12 - param.s2)
                    + param.s12 * (-2. * param.m0_2 + param.s1 + param.s2 - param.s12)
                    + param.m1_2 * (param.s12 + param.s2 - param.s1),
                param.lambda_m12_sqrt * param.lambda_s12_sqrt,
            )
    } else {
        0.0
    })
}
