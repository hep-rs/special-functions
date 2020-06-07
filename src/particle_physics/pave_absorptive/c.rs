use crate::{
    other::binomial,
    particle_physics::{kallen_lambda_sqrt, pave_absorptive as pave},
};

/// Internal parameters shared across the evaluation of the C coefficient
/// function.
struct Parameters {
    s1: f64,
    s12: f64,
    s2: f64,
    p1p2: f64,
    m0: f64,
    m0_2: f64,
    m1: f64,
    m1_2: f64,
    m2: f64,
    m2_2: f64,
    lambda_s_sqrt: f64,
    lambda_s1_sqrt: f64,
    lambda_s12_sqrt: f64,
    lambda_s2_sqrt: f64,
    f: [f64; 2],
    z: [[f64; 2]; 2],
    det_z: f64,
    coz: [[f64; 2]; 2],
    x: [f64; 2],
}

impl Parameters {
    fn new(s1: f64, s12: f64, s2: f64, m0: f64, m1: f64, m2: f64) -> Self {
        let m0_2 = m0.powi(2);
        let m1_2 = m1.powi(2);
        let m2_2 = m2.powi(2);
        let p1p2 = (s12 - s1 - s2) / 2.0;
        let f = [s1 - m1_2 + m0_2, s2 - m2_2 + m0_2];
        Self {
            s1,
            s12,
            s2,
            p1p2,
            m0,
            m0_2,
            m1,
            m1_2,
            m2,
            m2_2,
            lambda_s_sqrt: kallen_lambda_sqrt(s1, s12, s2),
            lambda_s1_sqrt: kallen_lambda_sqrt(s1, m0_2, m1_2),
            lambda_s12_sqrt: kallen_lambda_sqrt(s12, m1_2, m1_2),
            lambda_s2_sqrt: kallen_lambda_sqrt(s2, m0_2, m2_2),
            f,
            z: [[s1, p1p2], [p1p2, s2]],
            det_z: s1 * s2 - p1p2 * p1p2,
            coz: [[s2, -p1p2], [-p1p2, s1]],
            x: [s2 * f[0] - p1p2 * f[1], -p1p2 * f[0] + s1 * f[1]],
        }
    }
}

/// Absrptive part of the Passarin-Veltman coefficient function
///
/// \\begin{equation}
///   \boldsymbol{C}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}\underbrace{2\dots2}_{n_2}}(s_1, s_{12}, s_2; m0, m1, m_2)
/// \\end{equation}
#[allow(clippy::too_many_arguments, non_snake_case)]
pub fn c(r: i32, n1: i32, n2: i32, s1: f64, s12: f64, s2: f64, m0: f64, m1: f64, m2: f64) -> f64 {
    debug_assert!(n1 >= 0 && n2 >= 0, "n1 and n2 must be non-negative.");
    debug_assert!(
        m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0,
        "masses must be non-negative."
    );

    if s1 <= (m0 + m1).powi(2) && s2 <= (m0 + m2).powi(2) && s12 <= (m1 + m2).powi(2) {
        return 0.0;
    }

    // Always have `n1 >= n2`
    if n2 > n1 {
        let param = Parameters::new(s2, s12, s1, m0, m2, m1);
        c_internal(r, n2, n1, &param)
    } else {
        let param = Parameters::new(s1, s12, s2, m0, m1, m2);
        c_internal(r, n1, n2, &param)
    }
}

fn c_internal(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    let zero_threshold = f64::MIN_POSITIVE.sqrt();
    if r == 0 && n1 == 0 && n2 == 0 {
        scalar(param)
    } else if param.det_z > zero_threshold {
        if (param.s1 - param.m0_2) / param.s1 < f64::EPSILON
            && (param.s2 - param.m2_2) / param.s2 < f64::EPSILON
            && param.m1 < zero_threshold
        {
            c_case2(r, n1, n2, param)
        } else {
            c_case1(r, n1, n2, param)
        }
    } else if param.x[0] > zero_threshold && param.x[1] > zero_threshold {
        c_case3(r, n1, n2, param)
    } else if param.f[0] > zero_threshold && param.f[1] > zero_threshold {
        if param.z[0][0] > zero_threshold
            && param.z[0][1] > zero_threshold
            && param.z[1][0] > zero_threshold
            && param.z[1][1] > zero_threshold
        {
            c_case5(r, n1, n2, param)
        } else {
            c_case4(r, n1, n2, param)
        }
    } else {
        c_case6(r, n1, n2, param)
    }
}

fn scalar(param: &Parameters) -> f64 {
    let mut result = 0.0;

    if param.s1 > (param.m0 + param.m1).powi(2) {
        let lambda = param.lambda_s1_sqrt * param.lambda_s_sqrt;
        let m0_factor = param.m0_2 * (param.s1 + param.s12 - param.s2);
        let m1_factor = param.m1_2 * (param.s1 - param.s12 + param.s2);
        let s_factor = param.s1 * (-2.0 * param.m2_2 - param.s1 + param.s12 + param.s2);

        let numerator = lambda + m0_factor + m1_factor + s_factor;
        let denominator = -lambda + m0_factor + m1_factor + s_factor;

        result += (numerator / denominator).ln();
    }

    if param.s12 > (param.m1 + param.m2).powi(2) {
        let lambda = param.lambda_s12_sqrt * param.lambda_s_sqrt;
        let m1_factor = param.m1_2 * (-param.s1 + param.s12 + param.s2);
        let m2_factor = param.m2_2 * (param.s1 + param.s12 - param.s2);
        let s_factor = param.s12 * (-2.0 * param.m0_2 + param.s1 - param.s12 + param.s2);

        let numerator = lambda + m1_factor + m2_factor + s_factor;
        let denominator = -lambda + m1_factor + m2_factor + s_factor;

        result += (numerator / denominator).ln();
    }

    if param.s2 > (param.m0 + param.m2).powi(2) {
        let lambda = param.lambda_s2_sqrt * param.lambda_s_sqrt;
        let m0_factor = param.m0_2 * (-param.s1 + param.s12 + param.s2);
        let m2_factor = param.m2_2 * (param.s1 - param.s12 + param.s2);
        let s_factor = param.s2 * (-2.0 * param.m1_2 + param.s1 + param.s12 - param.s2);

        let numerator = lambda + m0_factor + m2_factor + s_factor;
        let denominator = -lambda + m0_factor + m2_factor + s_factor;

        result += (numerator / denominator).ln();
    }

    result / param.lambda_s_sqrt
}

fn c_case1(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    unimplemented!()
}

fn c_case2(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    unimplemented!()
}

fn c_case3(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    unimplemented!()
}

fn c_case4(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    unimplemented!()
}
fn c_case5(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    unimplemented!()
}

fn c_case6(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    unimplemented!()
}

fn neg_power(n: i32) -> f64 {
    if n % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}
