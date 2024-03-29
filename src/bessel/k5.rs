#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-5)
        * polynomial(
            x2,
            &[
                384.,
                -24.,
                1.,
                -0.04166666666666667,
                0.002604166666666667,
                -0.000327499526647156,
                -0.00001997538536955743,
                -4.189840818543418e-7,
            ],
        )
        + x.powi(5)
            * x.ln()
            * polynomial(
                x2,
                &[
                    0.0002604166666666667,
                    0.00001085069444444444,
                    1.937624007936508e-7,
                ],
            )
}

pub fn upper(x: f64) -> f64 {
    (-x).exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                1.2533141373155,
                15.50976244927932,
                88.21177393027611,
                275.6617935321128,
                439.3359834418048,
                208.6845921348573,
                -91.2995090590001,
                112.4940379476965,
                -219.7149178665947,
                576.7516593998112,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 2] = [
    &[
        2.653949528151962,
        -7.065729762339219,
        -0.377765963772093,
        -0.1235306365227639,
        -0.02848481365398826,
        -0.004377725329446185,
        -0.0003251072124247367,
        0.00002284482976094575,
        5.0489176736014e-6,
        -1.414362483389041e-6,
        -5.254717537432162e-7,
        -1.195357332999875e-8,
        2.470697217376365e-8,
        4.222400518728825e-9,
        -7.122258676610543e-10,
        -3.378404582433083e-10,
        -1.028130826863637e-11,
        1.762873831752836e-11,
        3.211260920054488e-12,
        -5.428998320759901e-13,
        -2.682356409507971e-13,
        -8.012398580025083e-15,
        1.466830857624654e-14,
        2.659663467506383e-15,
        -4.799377483032844e-16,
        -2.309148412376862e-16,
        -5.510634360833764e-18,
        1.312029892586726e-17,
        2.300551504076886e-18,
        -4.583316079049645e-19,
        -2.071306945655496e-19,
    ],
    &[
        -31.11344918649669,
        -33.78819010028696,
        -9.32661188959141,
        -1.966815120969425,
        -0.2941107494908688,
        -0.03655109032196109,
        -0.003919943751582862,
        -0.000321113200926505,
        -0.00002656766280017397,
        -2.372835957851889e-6,
        1.610631035547977e-8,
        -1.658228728900069e-8,
        -3.000670111918611e-9,
        1.017770420946947e-9,
        -1.402323960300425e-10,
        -9.26467302034961e-12,
        8.109602294301936e-12,
        -1.578925251501665e-12,
        3.549445619921859e-14,
        6.006596018142589e-14,
        -1.629933706596304e-14,
        1.352843523013898e-15,
        3.918856228680636e-16,
        -1.572801670813844e-16,
        2.126964876042159e-17,
        1.836083283221554e-18,
        -1.419578223754708e-18,
        2.688894422100111e-19,
        -2.106647619235266e-21,
        -1.201780257315089e-20,
        3.032347510746066e-21,
    ],
];

pub const SPLITS: [f64; 3] = [-0.701138118448943, 1.806592495072477, 4.314323108593896];
