#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(8)
        * polynomial(
            x2,
            &[
                9.68812003968254e-8,
                2.691144455467372e-9,
                3.363930569334215e-11,
                2.548432249495618e-13,
                1.327308463278967e-15,
                5.105032551072952e-18,
            ],
        )
}

pub fn upper(x: f64) -> f64 {
    x.exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                0.3989422804014327,
                -12.71628518779567,
                196.3076525865956,
                -1889.461156145983,
                12222.45185381933,
                -53473.22686045955,
                150393.4505450425,
                -233646.9678110481,
                113172.7500334764,
                51870.84376534337,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 2] = [
    &[
        -9.25931577390976,
        11.53207643170594,
        0.3978581884628116,
        0.1488171427307726,
        0.04186429603153622,
        0.00883785891709169,
        0.001279580756617064,
        0.00006719044545994653,
        -0.0000244285224855827,
        -7.86075442824618e-6,
        -6.862315694346824e-7,
        2.673737184301796e-7,
        1.243911002496491e-7,
        2.240947592405693e-8,
        -8.693106631700991e-10,
        -1.792346081618321e-9,
        -5.430443681304954e-10,
        -5.956593761102665e-11,
        1.821509668869122e-11,
        1.07035362267525e-11,
        2.446400359854643e-12,
        7.67625838747917e-14,
        -1.562265970320004e-13,
        -6.241947502906515e-14,
        -1.082441463146753e-14,
        8.308903169099554e-16,
        1.14343872473842e-15,
        3.587613973571507e-16,
        4.386691975690092e-17,
        -1.372929526850448e-17,
        -7.845797442066239e-18,
    ],
    &[
        45.29547401006699,
        55.86126887986134,
        16.72691640704637,
        3.820320659513853,
        0.6023457636489939,
        0.08063490302986129,
        0.00959648769249457,
        0.0007222824123873631,
        0.00008019842725200897,
        0.00001132437656738164,
        -2.17032730113096e-6,
        4.257425486560218e-7,
        6.186356815305833e-8,
        -5.461770625430814e-8,
        1.412420021208264e-8,
        -1.220075137485006e-10,
        -1.285770844490915e-9,
        4.856533327512936e-10,
        -5.116791604325438e-11,
        -3.12016694499465e-11,
        1.724986420752359e-11,
        -3.140114788147767e-12,
        -7.366961398971593e-13,
        6.432520960403017e-13,
        -1.608776040130834e-13,
        -1.511865463379082e-14,
        2.548975737080987e-14,
        -7.961248971325472e-15,
        -1.203587238951111e-16,
        1.093631067032107e-15,
        -3.939532705713392e-16,
    ],
];

pub const SPLITS: [f64; 3] = [-0.5459583160862256, 2.145115473764598, 4.836189263615421];
