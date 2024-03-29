#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-1)
        * polynomial(
            x2,
            &[
                1.,
                -0.3079657578292062,
                -0.08537071972865078,
                -0.00464218276647156,
                -0.0001125360703663057,
                -1.559288770203821e-6,
            ],
        )
        + x.powi(1)
            * x.ln()
            * polynomial(
                x2,
                &[
                    0.5,
                    0.0625,
                    0.002604166666666667,
                    0.00005425347222222222,
                    6.781684027777778e-7,
                ],
            )
}

pub fn upper(x: f64) -> f64 {
    (-x).exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                1.2533141373155,
                0.4699928014933126,
                -0.1468727504666602,
                0.1285136566583277,
                -0.1807223296757733,
                0.3478904846258636,
                -0.8479830562755424,
                2.49852150509758,
                -8.627707072290082,
                34.15134049448157,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 2] = [
    &[
        -0.7148263489151712,
        -2.560161954750035,
        -0.4521524409058577,
        -0.1149195779013302,
        -0.02026761062127773,
        -0.002648856207434807,
        -0.0002930256913008474,
        -0.00003230149309352294,
        -3.157537436179572e-6,
        -1.671187356691462e-7,
        -2.720777063679845e-9,
        -2.241406462698717e-9,
        -3.668423189494817e-10,
        3.763202645155196e-11,
        8.060535198480392e-12,
        -1.263269741287805e-12,
        -2.145472743450889e-13,
        3.900831076737965e-14,
        5.529794953962252e-15,
        -1.214233402406799e-15,
        -1.402849322328854e-16,
        3.780184851449894e-17,
        3.466800284202729e-18,
        -1.175428767612738e-18,
        -8.225441780301431e-20,
        3.644681964382177e-20,
        1.821375160975717e-21,
        -1.125523033278816e-21,
        -3.518821007782122e-23,
        3.353174159877224e-23,
        4.640574677180552e-25,
    ],
    &[
        -23.59222532546936,
        -26.48591641404397,
        -8.363345325875404,
        -1.887101733591633,
        -0.3227061696977166,
        -0.0446474303891523,
        -0.005165354046883223,
        -0.0005127233223440709,
        -0.00004482017594586142,
        -3.473922083976412e-6,
        -2.412328643900733e-7,
        -1.565450735701652e-8,
        -8.942453369178765e-10,
        -4.525954748632959e-11,
        -3.055134013660533e-12,
        -7.744626436422918e-14,
        1.380363333555574e-15,
        -1.83100053869134e-15,
        1.19072007146297e-16,
        9.86194620388337e-18,
        -3.668170562589307e-18,
        3.569259246167537e-19,
        6.780660203627973e-21,
        -6.326078387490272e-21,
        8.353436826049313e-22,
        -4.550945888688958e-23,
        -3.706671123581502e-24,
        1.644625680460585e-24,
        -3.248175427154551e-25,
        2.952767555037981e-26,
        3.013481930056739e-27,
    ],
];

pub const SPLITS: [f64; 3] = [-1.541328429516419, 1.26722142814209, 4.075771285800598];
