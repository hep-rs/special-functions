#![allow(clippy::all)]

use crate::approximations::polynomial;
use std::convert::identity;

approx_fn! {
    fn _lower(mod = lower, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    fn _upper(mod = upper, type = chebyshev, outer = identity, inner = identity);
}

#[inline]
pub(crate) fn eval(x: f64) -> f64 {
    if x < 0.0 {
        _lower(x)
    } else {
        _upper(x)
    }
}

pub(crate) mod lower {
    use super::*;

    pub fn lower(x: f64) -> f64 {
        polynomial(
            x.recip(),
            &[
                0.0,
                1.,
                0.0078125,
                0.0004572473708276177,
                0.00006103515625,
                0.0000128,
                3.572245084590764e-6,
                1.214265678902012e-6,
                4.76837158203125e-7,
                2.09075158128769e-7,
                1e-7,
            ],
        ) + polynomial(
            (-x).ln(),
            &[
                0.0,
                -1.97110218259487,
                0.0,
                -0.3156776098324153,
                0.0,
                -0.01370778389040189,
                0.0,
                -0.0001984126984126984,
            ],
        )
    }

    pub fn upper(x: f64) -> f64 {
        polynomial(
            x,
            &[
                0.0,
                1.,
                0.0078125,
                0.0004572473708276177,
                0.00006103515625,
                0.0000128,
                3.572245084590764e-6,
                1.214265678902012e-6,
                4.76837158203125e-7,
                2.09075158128769e-7,
                1e-7,
            ],
        )
    }

    pub const COEFFICIENTS: [&[f64]; 1] = [&[
        -2.485910672190817,
        2.322060295776381,
        0.01642838928068531,
        0.0006719315685050858,
        0.00005317087939773115,
        5.960978216093643e-6,
        8.304581804618238e-7,
        1.343479723729436e-7,
        2.425306524401931e-8,
        4.762747079319627e-9,
        9.99925299498195e-10,
        2.216761917172446e-10,
        5.141996923196633e-11,
        1.239288238263605e-11,
        3.086542880256977e-12,
        7.909432275376223e-13,
        2.078082735096488e-13,
        5.581721875314155e-14,
        1.529026298547541e-14,
        4.263065584918883e-15,
        1.207647352579972e-15,
        3.470776189167142e-16,
        1.010708180393544e-16,
        2.978885824522514e-17,
        8.877428093129953e-18,
        2.672729396053507e-18,
        8.123750886123699e-19,
        2.49290130622988e-19,
        7.773791553592283e-20,
        2.635318373507581e-20,
        7.521713913711639e-21,
    ]];

    pub const SPLITS: [f64; 2] = [-4.947453297099445, -0.1468569999423405];
}

pub(crate) mod upper {
    use super::*;

    pub fn lower(x: f64) -> f64 {
        polynomial(
            x,
            &[
                0.0,
                1.,
                0.0078125,
                0.0004572473708276177,
                0.00006103515625,
                0.0000128,
                3.572245084590764e-6,
                1.214265678902012e-6,
                4.76837158203125e-7,
                2.09075158128769e-7,
                1e-7,
            ],
        )
    }

    pub fn upper(x: f64) -> f64 {
        if x == 1.0 {
            return 1.008349277381923;
        }

        let xm1 = x - 1.0;
        let ln = (-xm1).ln();
        polynomial(
            xm1,
            &[
                1.008349277381923,
                1.017343061984449,
                0.00979234657946039,
                0.001037682041654449,
                0.000427684815130812,
                0.0006293662133366944,
                -0.002131756038909045,
                0.00336131229740577,
                -0.004218834990524859,
                0.004763720769847113,
                -0.005076465741826112,
            ],
        ) + ln
            * polynomial(
                xm1,
                &[
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    -0.001388888888888889,
                    0.004166666666666667,
                    -0.007986111111111111,
                    0.0125,
                    -0.01743634259259259,
                ],
            )
    }

    pub const COEFFICIENTS: [&[f64]; 1] = [&[
        0.5513976160376317,
        0.407357330026047,
        0.0007107613778083627,
        0.00001079216653727532,
        4.312866565248302e-7,
        3.055158551305538e-8,
        3.176794894145374e-9,
        4.353695343230703e-10,
        7.345267027655042e-11,
        1.456694396024918e-11,
        3.286262520757532e-12,
        8.232906253196475e-13,
        2.249226079435186e-13,
        6.607831744310014e-14,
        2.064678201961452e-14,
        6.801425893478727e-15,
        2.345396949741464e-15,
        8.417333051411811e-16,
        3.128831478647694e-16,
        1.199748067980033e-16,
        4.729581593711969e-17,
        1.911286520198914e-17,
        7.898156196371941e-18,
        3.33043282047171e-18,
        1.430438391211414e-18,
        6.24933113480364e-19,
        2.776229912066288e-19,
        1.258803843514307e-19,
        5.948509749467298e-20,
        3.191334091096419e-20,
        1.219938937282821e-20,
    ]];

    pub const SPLITS: [f64; 2] = [0.1445759509888041, 0.951938711503051];
}
