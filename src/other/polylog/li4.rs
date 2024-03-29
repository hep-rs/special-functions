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
                -1.894065658994492,
                -1.,
                -0.0625,
                -0.01234567901234568,
                -0.00390625,
                -0.0016,
                -0.0007716049382716049,
                -0.0004164931278633903,
                -0.000244140625,
                -0.0001524157902758726,
                -0.0001,
            ],
        ) + polynomial(
            (-x).ln(),
            &[0.0, 0.0, -0.8224670334241132, 0.0, -0.04166666666666667],
        )
    }

    pub fn upper(x: f64) -> f64 {
        polynomial(
            x,
            &[
                0.0,
                1.,
                0.0625,
                0.01234567901234568,
                0.00390625,
                0.0016,
                0.0007716049382716049,
                0.0004164931278633903,
                0.000244140625,
                0.0001524157902758726,
                0.0001,
            ],
        )
    }

    pub const COEFFICIENTS: [&[f64]; 2] = [
        &[
            -5.395288653011002,
            1.502960933375927,
            0.03192611840065708,
            0.001544539861903513,
            0.0001024764285740781,
            8.058202543052015e-6,
            7.053138168950754e-7,
            6.648749903208133e-8,
            6.619406612131135e-9,
            6.87305539153187e-10,
            7.378914558289289e-11,
            8.140879438084686e-12,
            9.18761342890836e-13,
            1.056995555682293e-13,
            1.236232708317281e-14,
            1.466701052531868e-15,
            1.762112096704983e-16,
            2.14066115155274e-17,
            2.626404930526667e-18,
            3.251129507299212e-19,
            4.056869291435997e-20,
            5.099298899136262e-21,
            6.452301566327857e-22,
            8.21412848320789e-23,
            1.051571430967189e-23,
            1.353192039842693e-24,
            1.749676116775674e-25,
            2.272416255862577e-26,
            2.964432272723836e-27,
            3.947069029699817e-28,
            5.097908319796336e-29,
        ],
        &[
            -2.0392242457935,
            1.886247375360667,
            0.07321008390723963,
            0.007417137095512286,
            0.00109949339675774,
            0.0002006953362902186,
            0.00004182508494299749,
            9.55923519897081e-6,
            2.339036502813697e-6,
            6.032450332272305e-7,
            1.622294116886175e-7,
            4.51434005489095e-8,
            1.292398456374588e-8,
            3.789927930272704e-9,
            1.134506686613055e-9,
            3.457287143719234e-10,
            1.070167069263524e-10,
            3.358628913776004e-11,
            1.0671021052562e-11,
            3.427891818380148e-12,
            1.112122143685511e-12,
            3.640630725343135e-13,
            1.201570292019527e-13,
            3.995447235409017e-14,
            1.337701521132132e-14,
            4.507158695615002e-15,
            1.527738948278152e-15,
            5.213414671163263e-16,
            1.807885671099829e-16,
            6.867357886423779e-17,
            2.119049199522589e-17,
        ],
    ];

    pub const SPLITS: [f64; 3] = [-9.2338976446268, -4.652602989794619, -0.07130833496243414];
}

pub(crate) mod upper {
    use super::*;

    pub fn lower(x: f64) -> f64 {
        polynomial(
            x,
            &[
                0.0,
                1.,
                0.0625,
                0.01234567901234568,
                0.00390625,
                0.0016,
                0.0007716049382716049,
                0.0004164931278633903,
                0.000244140625,
                0.0001524157902758726,
                0.0001,
            ],
        )
    }

    pub fn upper(x: f64) -> f64 {
        if x == 1.0 {
            return 1.082323233711138;
        }

        let xm1 = x - 1.0;
        let ln = (-xm1).ln();
        polynomial(
            xm1,
            &[
                1.082323233711138,
                1.202056903159594,
                0.2214385818443161,
                -0.1162258434820262,
                0.05758055484887188,
                -0.02900559166595327,
                0.01418764713508712,
                -0.006028318183603879,
                0.001332628810720929,
                0.001448207632398665,
                -0.003117737499542576,
            ],
        ) + ln
            * polynomial(
                xm1,
                &[
                    0.0,
                    0.0,
                    0.0,
                    -0.1666666666666667,
                    0.25,
                    -0.2916666666666667,
                    0.3125,
                    -0.3222222222222222,
                    0.3256944444444444,
                    -0.3255180776014109,
                    0.3231646825396825,
                ],
            )
    }

    pub const COEFFICIENTS: [&[f64]; 3] = [
        &[
            0.3012279570825021,
            0.2320978254121053,
            0.001896296529418625,
            0.00005298281798228912,
            2.478872675405455e-6,
            1.541391492057523e-7,
            1.147334092302746e-8,
            9.66489419393507e-10,
            8.909576894805852e-11,
            8.795888324261883e-12,
            9.16370396939813e-13,
            9.96979268681493e-14,
            1.124039100344422e-14,
            1.305651464291556e-15,
            1.555480108449749e-16,
            1.893848954869789e-17,
            2.349775493995971e-18,
            2.964105321482065e-19,
            3.794108109766509e-20,
            4.920112702084226e-21,
            6.455047051263192e-22,
            8.558156091984467e-23,
            1.145476056527247e-23,
            1.546478218117216e-24,
            2.104402688068167e-25,
            2.884408301042486e-26,
            3.979944945487208e-27,
            5.525497157216015e-28,
            7.718015100740538e-29,
            1.104560363218029e-29,
            1.527547163351313e-30,
        ],
        &[
            0.6571969162294497,
            0.1225451769110808,
            0.0006387165378246329,
            0.00001322503441048736,
            5.015839635660978e-7,
            2.663844151976843e-8,
            1.747383537017943e-9,
            1.322883279648683e-10,
            1.110192293532851e-11,
            1.006602196131147e-12,
            9.69152124462609e-14,
            9.78872191384417e-15,
            1.02806932636403e-15,
            1.115338336655296e-16,
            1.24357582988093e-17,
            1.419352523628599e-18,
            1.653032128600952e-19,
            1.959421344581029e-20,
            2.358914526926134e-21,
            2.879199516653619e-22,
            3.557672460229351e-23,
            4.444794745156184e-24,
            5.608730819969315e-25,
            7.141746656474514e-26,
            9.16904260118292e-27,
            1.186096470886048e-27,
            1.544991898228236e-28,
            2.025394880203806e-29,
            2.671731171327885e-30,
            3.604659081821975e-31,
            4.719794877720262e-32,
        ],
        &[
            0.908039444123583,
            0.1284941355535858,
            0.0008780484201304673,
            0.00003096556396975171,
            2.434319102049322e-6,
            3.027776993031417e-7,
            5.000287111596956e-8,
            9.95699580130346e-9,
            2.259666922600635e-9,
            5.643467445934289e-10,
            1.515926665249239e-10,
            4.311361567336056e-11,
            1.283780051150918e-11,
            3.969428129894914e-12,
            1.266547464213866e-12,
            4.150300442332862e-13,
            1.39140259126131e-13,
            4.757936247793642e-14,
            1.655390957059183e-14,
            5.848053607252189e-15,
            2.09416564068159e-15,
            7.590605222353548e-16,
            2.781491681963421e-16,
            1.029346411824263e-16,
            3.843606743851417e-17,
            1.447072821734965e-17,
            5.491085587985299e-18,
            2.103319060010614e-18,
            8.241498830218125e-19,
            3.587438797664443e-19,
            1.210648063084051e-19,
        ],
    ];

    pub const SPLITS: [f64; 4] = [
        0.07065927920878669,
        0.5165426636898918,
        0.7394843559304444,
        0.962426048170997,
    ];
}
