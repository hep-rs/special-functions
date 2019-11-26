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

pub const COEFFICIENTS: [&[f64]; 7] = [
    &[
        7.85751154448083,
        -1.5860760755966,
        -0.002863242918637818,
        -0.0002911760903084006,
        -0.00002187477168817379,
        -1.269331847727281e-6,
        -5.687715007882085e-8,
        -1.816867932135455e-9,
        -2.474982856860108e-11,
        1.468635146692057e-12,
        1.268780385967284e-13,
        4.305125608081737e-15,
        -3.450676541457636e-17,
        -1.321506088077229e-17,
        -7.900004117102386e-19,
        -1.756299032914105e-20,
        9.78063891761199e-22,
        1.148712092253844e-22,
        5.006935878282659e-24,
        1.120253609938614e-26,
        -1.363573668980506e-26,
        -9.53906900990357e-28,
        -2.473279706175855e-29,
        1.119414711350016e-30,
        1.480101103029911e-31,
        6.677713416714713e-33,
        1.316913750964583e-35,
        -1.88375702670583e-35,
        -1.305743979609802e-36,
        -3.177083659370198e-38,
        1.722836612560361e-39,
    ],
    &[
        4.645559163855608,
        -1.631408673458018,
        -0.00958430757533792,
        -0.000936312931844164,
        -0.00006513771346328813,
        -3.243911113605911e-6,
        -1.025330745742495e-7,
        -5.934312799472193e-10,
        1.323675175034908e-10,
        5.814136876847253e-12,
        -1.385412171975928e-13,
        -2.624582155141517e-14,
        -9.86363169545544e-16,
        3.300257283180673e-17,
        5.1550064060969e-18,
        1.686794467645004e-19,
        -8.898757399514712e-21,
        -1.093467873091354e-21,
        -2.810995860803372e-23,
        2.438885318735574e-24,
        2.377089667724473e-25,
        4.079060958777127e-27,
        -6.588893224838355e-28,
        -5.151792082232968e-29,
        -3.674977450253288e-31,
        1.735053662923468e-31,
        1.094254531656944e-32,
        -5.333877691945099e-35,
        -4.442187864126646e-35,
        -2.235395323012524e-36,
        4.52028308391493e-38,
    ],
    &[
        1.254190787415838,
        -1.776530027293202,
        -0.02963837498182352,
        -0.002626685472527414,
        -0.0001530195081300059,
        -5.33380529056008e-6,
        -5.076456345460183e-8,
        3.426024375757324e-9,
        -4.824847288258094e-11,
        -1.984047749292274e-11,
        -5.143576604652448e-13,
        5.694713690537313e-14,
        3.535694303070131e-15,
        -1.265452603924704e-16,
        -1.790952958373763e-17,
        1.46975414241795e-20,
        7.539113730926518e-20,
        2.072884306634874e-21,
        -2.641453580080223e-22,
        -1.642724278483629e-23,
        6.849046068353481e-25,
        9.13060378639321e-26,
        -4.358695643457871e-28,
        -4.164076103515167e-28,
        -9.83034805229662e-30,
        1.580220379253509e-30,
        8.869491768704289e-32,
        -4.589614763207821e-33,
        -5.294407646281554e-34,
        5.788215041484044e-36,
        2.559850645066114e-36,
    ],
    &[
        -2.670712022584629,
        -2.189757890771796,
        -0.07952066485818134,
        -0.005973781017452602,
        -0.0002678414900117682,
        -6.225679926407898e-6,
        -7.660001025706056e-8,
        -7.432272244970711e-9,
        -4.090022615701721e-10,
        1.297753445476368e-11,
        1.086597460360667e-12,
        -6.536114820970487e-14,
        -3.91304740343414e-15,
        3.011091045573191e-16,
        1.387057438338897e-17,
        -1.409912894915428e-18,
        -4.789139521511177e-20,
        6.622829712472163e-21,
        1.535492427161468e-22,
        -3.105755895029667e-23,
        -4.116980653102597e-25,
        1.449008349658582e-25,
        5.535673518365928e-28,
        -6.708723616524602e-28,
        3.648021231580531e-30,
        3.075550585089305e-30,
        -4.559752124156574e-32,
        -1.393074438277848e-32,
        3.438450219301077e-34,
        6.191651651554423e-35,
        -2.202873698797001e-36,
    ],
    &[
        -7.971733311305428,
        -3.196064265609391,
        -0.1815931600636394,
        -0.01144543102390353,
        -0.000432844037186562,
        -0.00001147280331227431,
        -3.760763627066544e-7,
        -1.11432966036309e-8,
        7.376742198892039e-11,
        -5.565928911636265e-13,
        -8.725973996851162e-13,
        2.585890867095798e-14,
        1.894326646535513e-15,
        -1.758987515413625e-16,
        1.433304780670939e-19,
        6.666724127656718e-19,
        -2.851914735407934e-20,
        -1.418636935919297e-21,
        1.731569476627253e-22,
        -1.657433383007233e-24,
        -6.283881229769943e-25,
        3.350492949034153e-26,
        1.137650933612475e-27,
        -1.863167561058831e-28,
        3.289281004943479e-30,
        6.310445375947636e-31,
        -4.081496270654305e-32,
        -8.772920079021301e-34,
        2.087463782692634e-34,
        -5.274448622643293e-36,
        -6.489389567389465e-37,
    ],
    &[
        -16.34845535391535,
        -5.337062387026985,
        -0.3699345453607086,
        -0.02080034082985122,
        -0.0007771908730369832,
        -0.00002416764925393526,
        -6.902316405587109e-7,
        -1.31217161175683e-8,
        -2.474967193093829e-10,
        -1.014210190428115e-11,
        1.323913118354108e-13,
        1.036627595789746e-15,
        -7.406621112294678e-16,
        3.710705657602558e-17,
        -6.853578362289935e-20,
        -1.101283870365327e-19,
        6.920611314300749e-21,
        -7.704473897365144e-23,
        -1.72816981943114e-23,
        1.311091496341053e-24,
        -2.499931475831697e-26,
        -2.724041696331658e-27,
        2.506422266681016e-28,
        -6.580509394446907e-30,
        -4.204056688671865e-31,
        4.802524844184332e-32,
        -1.590140285875055e-33,
        -6.152345929557648e-35,
        9.17969343707443e-36,
        -3.649974258450784e-37,
        -8.033566990003905e-39,
    ],
    &[
        -45.24552787067365,
        -26.80133391543663,
        -4.025302162995088,
        -0.4230785668496123,
        -0.03258924570992196,
        -0.00205452026813869,
        -0.0001069415522424024,
        -4.715205131194087e-6,
        -1.921040187840242e-7,
        -6.204736232867769e-9,
        -2.033961776795517e-10,
        -7.877748919358558e-12,
        1.18461864987437e-13,
        -2.300674771103801e-14,
        3.536448647778605e-16,
        9.61304774557155e-17,
        -1.668623350288384e-17,
        1.42585605460334e-18,
        -5.265759308648301e-20,
        -4.966245913669245e-21,
        1.12327506031658e-21,
        -1.098931643782355e-22,
        5.229178210635249e-24,
        2.444578963463143e-25,
        -7.917860573329067e-26,
        8.676892257025678e-27,
        -4.886783825961435e-28,
        -9.91812000183574e-30,
        5.70336340093132e-30,
        -6.944421181829981e-31,
        4.444763347669774e-32,
    ],
];

pub const SPLITS: [f64; 8] = [
    -0.701138118448943,
    -0.07420546506858809,
    0.5527271883117668,
    1.179659841692122,
    1.806592495072477,
    2.433525148452832,
    3.060457801833187,
    4.314323108593896,
];