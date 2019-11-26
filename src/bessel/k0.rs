#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-0)
        * polynomial(
            x2,
            &[
                0.1159315156584124,
                0.2789828789146031,
                0.02524892993216269,
                0.000846035090708223,
                0.00001491471929926043,
                1.627105610481598e-7,
            ],
        )
        + x.powi(0)
            * x.ln()
            * polynomial(
                x2,
                &[
                    -1.,
                    -0.25,
                    -0.015625,
                    -0.0004340277777777778,
                    -6.781684027777778e-6,
                    -6.781684027777778e-8,
                ],
            )
}

pub fn upper(x: f64) -> f64 {
    (-x).exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                1.2533141373155,
                -0.1566642671644375,
                0.08812365027999611,
                -0.0917954690416626,
                0.1405618119700459,
                -0.2846376692393429,
                0.7175241245408436,
                -2.165385304417903,
                7.61268271084419,
                -30.55646254769404,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 4] = [
    &[
        -0.2085722183787364,
        -0.6466234373411494,
        -0.06818858239273789,
        -0.006776619049836259,
        -0.0005648532057290292,
        -0.00003870426039175932,
        -2.177482279505991e-6,
        -1.01796546979261e-7,
        -4.120691475130881e-9,
        -1.569517494678906e-10,
        -5.899867225122668e-12,
        -1.871437115665544e-13,
        -2.833812674927421e-15,
        4.703793047913263e-17,
        -4.60808850165433e-18,
        -8.483001738166144e-19,
        -2.175956684309925e-20,
        2.734365003703626e-21,
        1.97035521601881e-22,
        -4.824920297624573e-24,
        -1.088322034264007e-24,
        -2.167494507994685e-26,
        4.093535718104437e-27,
        2.671155937682729e-28,
        -7.716198677280879e-30,
        -1.547754548546159e-30,
        -2.952431067515539e-32,
        5.865147539142937e-33,
        3.843710147402933e-34,
        -1.063570906459021e-35,
        -2.217876982425348e-36,
    ],
    &[
        -2.467441907190908,
        -1.754519053456803,
        -0.2419543837236125,
        -0.02616987795781264,
        -0.002198919868164031,
        -0.0001473626999277215,
        -8.154120704192495e-6,
        -3.88458124320244e-7,
        -1.645633508833329e-8,
        -6.150148199573163e-10,
        -1.966732561381868e-11,
        -5.886075240267398e-13,
        -2.076346637454755e-14,
        -5.76383735679904e-16,
        7.524414697926271e-18,
        3.871570956790986e-19,
        -9.92699968068361e-20,
        -3.330279068213404e-21,
        4.859571349709267e-22,
        1.782724571223284e-23,
        -2.506315962671042e-24,
        -1.029056289681279e-25,
        1.271968081824586e-26,
        6.045867512515982e-28,
        -6.343995934595752e-29,
        -3.561942088625405e-30,
        3.088827658765004e-31,
        2.079944806505929e-32,
        -1.457121647312464e-33,
        -1.188071347251053e-34,
        6.59549196297243e-36,
    ],
    &[
        -9.53435522080024,
        -5.861303723397037,
        -0.912175982699729,
        -0.1006600285344433,
        -0.008403448121880131,
        -0.0005606133846439977,
        -0.00003121152437960403,
        -1.493429897865665e-6,
        -6.24065340165602e-8,
        -2.314731212015925e-9,
        -7.798651692626943e-11,
        -2.376699794910962e-12,
        -6.395652745062551e-14,
        -1.739440979497802e-15,
        -4.888178595755486e-17,
        -3.326513018674579e-19,
        -2.735428148323062e-21,
        -4.047037267488822e-21,
        6.350244100363885e-23,
        1.619762925573782e-23,
        -1.026826829031997e-24,
        -4.623447202997478e-26,
        7.187917384115923e-27,
        -2.14410891765012e-29,
        -3.743491316926318e-29,
        1.434705614506876e-30,
        1.496215501925613e-31,
        -1.263856826868574e-32,
        -3.795866091755067e-34,
        7.855633828114734e-35,
        -4.542103259603949e-37,
    ],
    &[
        -34.77648132317171,
        -21.48396893729513,
        -3.479958708451811,
        -0.3850360004122241,
        -0.03208479875472865,
        -0.002141960223281649,
        -0.0001193253957951028,
        -5.700964198303499e-6,
        -2.383945654517671e-7,
        -8.867847180369254e-9,
        -2.968019891774466e-10,
        -9.02987804727078e-12,
        -2.525076698528572e-13,
        -6.482849046971511e-15,
        -1.546564507217406e-16,
        -3.573201723906707e-18,
        -6.792730209173642e-20,
        -1.33326610617161e-21,
        -5.454608700737067e-23,
        1.14094743585138e-24,
        5.435876220271201e-28,
        -8.055474594124891e-27,
        5.268440618863259e-28,
        -8.423730135613104e-31,
        -2.406722733070753e-30,
        1.713485295272069e-31,
        -4.698332523766234e-34,
        -7.799421598832119e-34,
        5.496761159063232e-35,
        6.521568543603345e-38,
        -2.667028314096517e-37,
    ],
];

pub const SPLITS: [f64; 5] = [
    -1.295219660504773,
    0.04520109037926667,
    1.385621841263306,
    2.726042592147345,
    4.066463343031385,
];