//! Polylogarithms
//!
//! At this stage, this only implements to special version of the polylogarithms:
//! - \\(\Li_3(e\^x)\\) for \\(x \leq 0\\) which appears in the expression for
//!   the number densities of Bose--Einstein species; and
//! - \\(-\Li_3(-e\^x)\\) for \\(x \in \mathbb{R}\\) which appears in the
//!   expression for the number densities of Fermi--Dirac species.

use std::f64;

use crate::polynomial::polynomial;

const ZETA_3: f64 = 1.202_056_903_159_594_2;

/// Taylor series expansion for \\(\Li_{3} e\^{-x}\\) around \\(x = 0\\) (for
/// \\(x \geq 0\\)).  This is applicable until \\(x \approx 0.22\\) at which point the
/// floating point error becomes larger.
const BOSE_EINSTEIN_ZERO: [f64; 9] = [
    1.2020569031595942854,
    -1.6449340668482264365,
    0.75000000000000000000,
    0.083333333333333333333,
    -0.0034722222222222222222,
    0.0,
    0.000011574074074074074074,
    0.0,
    -9.8418997228521038045e-8,
];

/// Subdivision for the Bose--Einstein function.  For each subdivision, a
/// degree-9 minimax polynomial ensures that the error is less than the floating
/// point error.
const BOSE_EINSTEIN_INTERVALS: [f64; 14] = [
    0.28, 0.36, 0.45, 0.56, 0.69, 0.84, 1.01, 1.20, 1.42, 1.65, 1.9, 2.18, 2.51, 2.85,
];

/// Minimax polynomial correspond for each interval in `BOSE_EINSTEIN_INTERVALS`.
const BOSE_EINSTEIN_MINIMAX_COEFFICIENTS: [[f64; 10]; 14] = [
    [
        1.2016285475563033060,
        -1.6138763785741975169,
        1.9924600762567151638,
        -4.6084347402793591257,
        14.136112478063298568,
        -37.847196344850070830,
        75.917522385341070394,
        -104.33833262521493766,
        87.064786639238023781,
        -33.185246714654806005,
    ],
    [
        1.2013557935844981173,
        -1.6051989055865978492,
        1.8692476268710253858,
        -3.5835783990201740742,
        8.6329590512122155556,
        -18.064877754105276587,
        28.315033994398982401,
        -30.406230069031660120,
        19.823264920887196748,
        -5.9028538406448606674,
    ],
    [
        1.2009303723208941758,
        -1.5945708850626883183,
        1.7507821769381485937,
        -2.8102969625230673441,
        5.3755105471236083966,
        -8.8818739701913394650,
        10.991855101713918225,
        -9.3213819812677504188,
        4.7999505751933925006,
        -1.1291376717107789505,
    ],
    [
        1.2003044003450659380,
        -1.5821188250661249268,
        1.6403203394889774860,
        -2.2367471524284705370,
        3.4545839705302484824,
        -4.5784686618565336248,
        4.5434198484285980851,
        -3.0895763714937605048,
        1.2757763746598426128,
        -0.24066649799780272505,
    ],
    [
        1.1993697167693136908,
        -1.5671533863171113677,
        1.5334874423719649813,
        -1.7904613267801657434,
        2.2523016455340850905,
        -2.4124255335067800560,
        1.9338096660359514772,
        -1.0623055851710819945,
        0.35438192636920172377,
        -0.054011296601736124930,
    ],
    [
        1.1980262246817860197,
        -1.5496770085377359179,
        1.4321614074406789370,
        -1.4467886872748876100,
        1.5008233070310725187,
        -1.3138763060248617691,
        0.86022065573617010500,
        -0.38597928199335615346,
        0.10518084436520242550,
        -0.013095735074946244908,
    ],
    [
        1.1961571449291896162,
        -1.5296920328981146336,
        1.3369501205706900977,
        -1.1815220146103237218,
        1.0245234235643418515,
        -0.74230841673057787405,
        0.40183397803873638571,
        -0.14908113545660465151,
        0.033593193627595517017,
        -0.0034588368247328554320,
    ],
    [
        1.1936290153898406562,
        -1.5072003316607865904,
        1.2478210112119562151,
        -0.97503574709433785382,
        0.71632390762083237508,
        -0.43496087632098287424,
        0.19706006110773898572,
        -0.061187155202560830873,
        0.011540118361536727040,
        -0.00099457920304089225536,
    ],
    [
        1.1902068381338062383,
        -1.4816148936191973610,
        1.1626337650828197186,
        -0.80924961107059651098,
        0.50849323492195689799,
        -0.26092115840426217711,
        0.099705413493167823217,
        -0.026109560373691588070,
        0.0041533098529469631781,
        -0.00030191496767321388065,
    ],
    [
        1.1857657656292930806,
        -1.4534489685709108511,
        1.0831021451054594119,
        -0.67801921711506909864,
        0.36904820176009757533,
        -0.16196673255414440789,
        0.052810685885363104173,
        -0.011798688338672225208,
        0.0016014742699175635594,
        -0.000099346672044168810591,
    ],
    [
        1.1802596302370923495,
        -1.4234449769625553646,
        1.0103297125930776331,
        -0.57490701922679254737,
        0.27498823072423826994,
        -0.10468150750917232176,
        0.029518035846592746628,
        -0.0057014468733130740001,
        0.00066912004141541601171,
        -0.000035893495149683489989,
    ],
    [
        1.1732568285085765188,
        -1.3903421000650971125,
        0.94068925373164790330,
        -0.48932957712390570631,
        0.20729338446522006350,
        -0.068934258893900316776,
        0.016916692670395117917,
        -0.0028420186515515793220,
        0.00029013158529907024506,
        -0.000013539732121325340655,
    ],
    [
        1.1641593555644077733,
        -1.3528877143391406568,
        0.87206624728899736906,
        -0.41589139523415768147,
        0.15670436549056159658,
        -0.045671290120816148712,
        0.0097759166080031969059,
        -0.0014311114371422069387,
        0.00012730660540588058612,
        -5.1777899690242828959e-6,
    ],
    [
        1.1525184571794547788,
        -1.3110398827930614838,
        0.80512129765519056868,
        -0.35334235016634759436,
        0.11908786670209599446,
        -0.030571074893307851711,
        0.0057298610219036096513,
        -0.00073331944385348268010,
        0.000057021620567669020812,
        -2.0276060134209767904e-6,
    ],
];

/// Taylor expansion for \\(\Li_{3} e\^{-x}\\) around \(x = +\infty\).  This is
/// valid until \\(x \approx 2.85 \\) at which point the floating point error
/// becomes larger.  Note that the polynomial is in \\(e\^{-x}\\) as opposed to
/// \\(x\\).
const BOSE_EINSTEIN_INFINITY: [f64; 11] = [
    0.0,
    1.0000000000000000000,
    0.12500000000000000000,
    0.037037037037037037037,
    0.015625000000000000000,
    0.0080000000000000000000,
    0.0046296296296296296296,
    0.0029154518950437317784,
    0.0019531250000000000000,
    0.0013717421124828532236,
    0.0010000000000000000000,
];

/// Taylor expansion for \\(-\Li_{3} -e\^{x}\\) around \(x = -\infty\).  This is
/// valid until \\(x \approx -2.6 \\) at which point the floating point error
/// becomes larger.  Note that the polynomial is in \\(e\^{x}\\) as opposed to
/// \\(x\\).
const FERMI_DIRAC_NEG_INFINITY: [f64; 12] = [
    0.0,
    1.0000000000000000000,
    -0.12500000000000000000,
    0.037037037037037037037,
    -0.015625000000000000000,
    0.0080000000000000000000,
    -0.0046296296296296296296,
    0.0029154518950437317784,
    -0.0019531250000000000000,
    0.0013717421124828532236,
    -0.0010000000000000000000,
    0.00075131480090157776108,
];

/// Subdivision for the Fermi--Dirac function.  For each subdivision, a degree-9
/// minimax polynomial ensures that the error is less than the floating point
/// error.
const FERMI_DIRAC_INTERVALS: [f64; 11] = [
    -2.15, -1.67, -1.25, -0.8, -0.3, 0.1, 0.6, 1.1, 1.65, 2.05, 2.5,
];

/// Minimax polynomial correspond for each interval in `FERMI_DIRAC_INTERVALS`.
const FERMI_DIRAC_MINIMAX_COEFFICIENTS: [[f64; 10]; 11] = [
    [
        0.90154873299002283863,
        0.82241976992437044379,
        0.34630905074079215765,
        0.082784178779805571846,
        0.0097718075577690367863,
        -0.00048234872204630743208,
        -0.00041140420178019327540,
        -0.000075135927716277565826,
        -6.8203722588270727123e-6,
        -2.6400186480525577614e-7,
    ],
    [
        0.90150305809449814300,
        0.82224689075300725356,
        0.34602057851034359464,
        0.082506276340257977531,
        0.0096020025515895438662,
        -0.00055028452812789288550,
        -0.00042907693895390832661,
        -0.000077985365258309945371,
        -7.0733430250677661352e-6,
        -2.7300177096605227639e-7,
    ],
    [
        0.90153653564693806385,
        0.82242439234990056966,
        0.34644072056656324764,
        0.083089020252824040060,
        0.010124006475236458914,
        -0.00023710553392475353427,
        -0.00030323278243702773904,
        -0.000045327776394798303374,
        -2.1072308581715910160e-6,
        6.4118854955851868921e-8,
    ],
    [
        0.90154252066108299514,
        0.82246550821242916523,
        0.34656698133489459368,
        0.083316612926613024282,
        0.010389518131662721199,
        -0.000029122420604746842756,
        -0.00019379796726793792668,
        -8.0197847298885573207e-6,
        5.3720404314697261160e-6,
        7.3596928850530768573e-7,
    ],
    [
        0.90154267748455301202,
        0.82246703585461608843,
        0.34657361326796679716,
        0.083333461554499268209,
        0.010417135022197963669,
        1.1752700210085207381e-6,
        -0.00017154717959012859597,
        2.5278819903382398720e-6,
        8.2992325820796509880e-6,
        1.0979654833094616613e-6,
    ],
    [
        0.90154267736969567585,
        0.82246703342411043027,
        0.34657359028002825482,
        0.083333333335061969301,
        0.010416666659813657397,
        -2.9387560093535594017e-10,
        -0.00017361200561683468617,
        1.2580398106132483797e-8,
        6.3042723021936834218e-6,
        2.9212599153192200589e-7,
    ],
    [
        0.90154267737062737965,
        0.82246703338537967925,
        0.34657359097178456161,
        0.083333326332682521884,
        0.010416711231074348971,
        -1.8716246318558118090e-7,
        -0.00017308334646431652280,
        -9.9313498731276757106e-7,
        7.4057565032315955493e-6,
        -8.6723617588834972661e-7,
    ],
    [
        0.90154266557551098408,
        0.82246717149652335189,
        0.34657287910267667045,
        0.083335434485518440009,
        0.010412801147992194333,
        4.4148533989053296315e-6,
        -0.00017632765901797160732,
        8.2774593818841718042e-8,
        7.4743602150778911600e-6,
        -9.8332471038138542550e-7,
    ],
    [
        0.90153938611309674751,
        0.82249148615953246258,
        0.34649223398736081604,
        0.083492651807469628173,
        0.010214038765093629910,
        0.00017361182908464641845,
        -0.00027342125040393769415,
        0.000036341881914419530466,
        -5.2939778931786919505e-7,
        -1.8708393902095712238e-7,
    ],
    [
        0.90150875065386042279,
        0.82265963866842450075,
        0.34607974158240154523,
        0.084086295214422129111,
        0.0096616151789424639647,
        0.00051833298005370779098,
        -0.00041766783872645057798,
        0.000075368246188467698555,
        -6.7233885107736624711e-6,
        2.5221832045473321723e-7,
    ],
    [
        0.90151004680040807225,
        0.82266428820464705394,
        0.34605068344670107390,
        0.084142007357900507368,
        0.0096042897182185351471,
        0.00055441055740366496741,
        -0.00043206289802245482864,
        0.000078941848921002502329,
        -7.2292359443830163745e-6,
        2.8351648559771369830e-7,
    ],
];

/// Taylor expansion for \\(-\Li_{3} -e\^{x}\\) around \(x = +\infty\).  This is
/// valid until \\(x \approx -2.6 \\) at which point the floating point error
/// becomes larger.  Note that the polynomial is in \\(e\^{x}\\) as opposed to
/// \\(x\\).  There is an addition factor of:
///
/// ```ignore
/// 0.16666666666666666667 * x.powi(3) + 1.6449340668482264365 * x
/// ```
/// which must be added for this to be accurate.
const FERMI_DIRAC_INFINITY: [f64; 11] = [
    0.0,
    1.0000000000000000000,
    -0.12500000000000000000,
    0.037037037037037037037,
    -0.015625000000000000000,
    0.0080000000000000000000,
    -0.0046296296296296296296,
    0.0029154518950437317784,
    -0.0019531250000000000000,
    0.0013717421124828532236,
    -0.0010000000000000000000,
];

/// Approximation of polylogarithm appearing in the Bose–Einstein statistics.
/// Specifically, this approximates the function \\(\Li_{3} e\^x\\) for \\(x
/// \leq 0\\).
///
/// The approximation is split in three regimes:
///
/// - \\(|x| \ll 1\\), where the Taylor series expansion around \\(x = 0\\) is
///   used;
/// - \\(|x| \approx 1\\), where a mini-max polynomial is used which ensures the
///   error is spread over the whole interval and overall bias is minimized.
///   (This is unlike a Taylor series where the error is minimized at a single
///   point and increases (sometimes rapidly) everywhere else);
/// - \\(|x| \gg 1\\), where the Taylor series expansion around \\(x = \infty\\)
///   is used.
pub fn bose_einstein(x: f64) -> f64 {
    debug_assert!(x <= 0.0, "Argument must be negative");

    let x = -x;

    if x == 0.0 {
        debug!("Using exact expression at x = 0.");
        ZETA_3
    } else if x < 0.22 {
        debug!("Using Taylor series around x = 0.");
        polynomial(x, &BOSE_EINSTEIN_ZERO) - 0.5 * x.powi(2) * x.ln()
    } else if x > 2.85 {
        debug!("Using Taylor series around x = ∞.");
        let ex = (-x).exp();
        polynomial(ex, &BOSE_EINSTEIN_INFINITY)
    } else {
        let (i, x_lim) = BOSE_EINSTEIN_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between 0.22 and 2.85.");
        debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(x, &BOSE_EINSTEIN_MINIMAX_COEFFICIENTS[i])
    }
}

/// Approximation of polylogarithm appearing in the Fermi–Dirac statistics.
/// Specifically, this approximates the function \\(-\Li_{3} (-e\^x)\\) for all
/// values of \\(x\\).
///
/// The approximation is split in three regimes:
///
/// - \\(|x| \ll 1\\), where the Taylor series expansion around \\(x = 0\\) is
///   used;
/// - \\(|x| \approx 1\\), where a mini-max polynomial is used which ensures the
///   error is spread over the whole interval and overall bias is minimized.
///   (This is unlike a Taylor series where the error is minimized at a single
///   point and increases (sometimes rapidly) everywhere else);
/// - \\(|x| \gg 1\\), where the Taylor series expansion around \\(x = \infty\\)
///   is used.
pub fn fermi_dirac(x: f64) -> f64 {
    if x < -2.6 {
        debug!("Using Taylor series around x = -∞.");
        let ex = x.exp();
        polynomial(ex, &FERMI_DIRAC_NEG_INFINITY)
    } else if x > 2.5 {
        debug!("Using Taylor series around x = ∞.");
        let ex = (-x).exp();
        polynomial(ex, &FERMI_DIRAC_INFINITY)
            + 0.166_666_666_666_666_66 * x.powi(3)
            + 1.644_934_066_848_226_4 * x
    } else {
        let (i, x_lim) = FERMI_DIRAC_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between -2.6 and 2.5.");
        debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(x, &FERMI_DIRAC_MINIMAX_COEFFICIENTS[i])
    }
}

#[cfg(test)]
mod test {
    use csv;
    use crate::utilities::test::*;

    #[test]
    fn bose_einstein() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog_bose_einstein.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::bose_einstein(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }
    }

    #[test]
    fn fermi_dirac() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog_fermi_dirac.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::fermi_dirac(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use csv;
    use test::Bencher;
    use utilities::test::*;

    #[bench]
    fn bose_einstein(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog_bose_einstein.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::bose_einstein(x);
                    approx_eq(n, v, 12.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn fermi_dirac(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog_fermi_dirac.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::fermi_dirac(x);
                    approx_eq(n, v, 12.0, 0.0);
                }
            }
        });
    }
}
