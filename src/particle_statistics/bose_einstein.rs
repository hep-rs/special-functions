#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    polynomial(
        x,
        &[1.2020569031595942, -1.6449340668482262, 0.75, 0.08333333333333333, -0.003472222222222222, 0., 0.000011574074074074073, 0., -9.841899722852104e-8, 0., 1.1482216343327454e-9],
    )
    - 0.5 * x.powi(2) * x.ln()
}

pub fn upper(x: f64) -> f64 {
    polynomial(
        (-x).exp(),
        &[0., 1., 0.125, 0.037037037037037035, 0.015625, 0.008, 0.004629629629629629, 0.0029154518950437317, 0.001953125, 0.0013717421124828531, 0.001, 0.0007513148009015778]
    )
}

pub const COEFFICIENTS: [&[f64]; 2] = [
    &[0.45227266968963964, -0.25705557870550316, 0.04034875529291428, -0.004928241484344002, 0.0005875079079072601, -0.00007986263061371321, 0.000012871623323569996, -2.371449985928576e-6, 4.793965232169481e-7, -1.0357583884941594e-7, 2.3529245681989094e-8, -5.559140912151847e-9, 1.3555645261219942e-9, -3.3923696898999666e-10, 8.675906427001404e-11, -2.2601208068667815e-11, 5.981776700941113e-12, -1.6051231382723122e-12, 4.3594393141484023e-13, -1.1967115554720538e-13, 3.316478671408503e-14, -9.269690121490736e-15, 2.610888849377544e-15, -7.405137139584545e-16, 2.1136254865799955e-16, -6.067877717481667e-17, 1.751266810764571e-17, -5.079161045182683e-18, 1.4797696642229958e-18, -4.329271446526713e-19, 1.2715265579352852e-19, -3.7480943961336015e-20, 1.1085774926165943e-20, -3.2892585335477852e-21, 9.78861133836472e-22, -2.921175685260838e-22, 8.740514037331587e-23, -2.621767577084209e-23, 7.882607256697458e-24, -2.3752506621575758e-24, 7.172370459829179e-25, -2.1701284912823823e-25, 6.578987898738397e-26, -1.9994301976786675e-26, 6.1304277429521136e-27, -2.0246362228235602e-27, 5.651753518332331e-28],
    &[0.14189347862459759, -0.0771357928692625, 0.010951304027219309, -0.0010824390886692433, 0.00008637530886303146, -6.259123142748734e-6, 4.5681150357294767e-7, -3.588128838643293e-8, 3.07264376141117e-9, -2.8190511403417364e-10, 2.719505820025826e-11, -2.7257673876528977e-12, 2.8174181238098e-13, -2.987100845125721e-14, 3.2351745375163693e-15, -3.567813481110027e-16, 3.9963585833221354e-17, -4.537356500310285e-18, 5.213126890881635e-19, -6.052796156928953e-20, 7.093800451058055e-21, -8.383918968405501e-22, 9.983949296199161e-23, -1.1971186526435619e-23, 1.4443924987961293e-24, -1.7527272168572214e-25, 2.1380654520552438e-26, -2.6207511165604514e-27, 3.2267822639768235e-28, -3.989431886504129e-29, 4.951346799645782e-30, -6.167268466472435e-31, 7.707563737625854e-32, -9.66281133001938e-33, 1.2149765765602199e-33, -1.53191201499761e-34, 1.936562012131677e-35, -2.45412534878702e-36, 3.1172466635267815e-37, -3.968265708899438e-38, 5.062158463034696e-39, -6.470386154856782e-40, 8.285937177386252e-41, -1.0629981203716302e-41, 1.366408325096222e-42, -1.7875809181410562e-43, 2.2667725971526927e-44],
];

pub const SPLITS: [f64; 3] = [0.3850679021450391, 1.494332005536929, 2.6035961089288184];
