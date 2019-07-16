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
    1.202_056_903_159_594_2,
    -1.644_934_066_848_226_4,
    0.75,
    0.083_333_333_333_333_33,
    -0.003_472_222_222_222_222,
    0.0,
    0.000_011_574_074_074_074_073,
    0.0,
    -9.841_899_722_852_104e-8,
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
        1.201_628_547_556_303_3,
        -1.613_876_378_574_197_4,
        1.992_460_076_256_715,
        -4.608_434_740_279_359_5,
        14.136_112_478_063_298,
        -37.847_196_344_850_07,
        75.917_522_385_341_07,
        -104.338_332_625_214_93,
        87.064_786_639_238_02,
        -33.185_246_714_654_81,
    ],
    [
        1.201_355_793_584_498,
        -1.605_198_905_586_597_8,
        1.869_247_626_871_025_4,
        -3.583_578_399_020_174,
        8.632_959_051_212_216,
        -18.064_877_754_105_275,
        28.315_033_994_398_984,
        -30.406_230_069_031_66,
        19.823_264_920_887_198,
        -5.902_853_840_644_861,
    ],
    [
        1.200_930_372_320_894_2,
        -1.594_570_885_062_688_3,
        1.750_782_176_938_148_6,
        -2.810_296_962_523_067,
        5.375_510_547_123_608,
        -8.881_873_970_191_34,
        10.991_855_101_713_918,
        -9.321_381_981_267_75,
        4.799_950_575_193_392,
        -1.129_137_671_710_779,
    ],
    [
        1.200_304_400_345_066,
        -1.582_118_825_066_125,
        1.640_320_339_488_977_6,
        -2.236_747_152_428_470_6,
        3.454_583_970_530_248_6,
        -4.578_468_661_856_534,
        4.543_419_848_428_598,
        -3.089_576_371_493_760_3,
        1.275_776_374_659_842_5,
        -0.240_666_497_997_802_74,
    ],
    [
        1.199_369_716_769_313_6,
        -1.567_153_386_317_111_4,
        1.533_487_442_371_964_9,
        -1.790_461_326_780_165_7,
        2.252_301_645_534_085,
        -2.412_425_533_506_78,
        1.933_809_666_035_951_4,
        -1.062_305_585_171_082,
        0.354_381_926_369_201_74,
        -0.054_011_296_601_736_127,
    ],
    [
        1.198_026_224_681_786,
        -1.549_677_008_537_735_8,
        1.432_161_407_440_679,
        -1.446_788_687_274_887_6,
        1.500_823_307_031_072_6,
        -1.313_876_306_024_861_7,
        0.860_220_655_736_170_1,
        -0.385_979_281_993_356_1,
        0.105_180_844_365_202_42,
        -0.013_095_735_074_946_244,
    ],
    [
        1.196_157_144_929_189_7,
        -1.529_692_032_898_114_7,
        1.336_950_120_570_690_2,
        -1.181_522_014_610_323_8,
        1.024_523_423_564_342,
        -0.742_308_416_730_577_9,
        0.401_833_978_038_736_4,
        -0.149_081_135_456_604_65,
        0.033_593_193_627_595_52,
        -0.003_458_836_824_732_855_6,
    ],
    [
        1.193_629_015_389_840_6,
        -1.507_200_331_660_786_5,
        1.247_821_011_211_956_3,
        -0.975_035_747_094_337_8,
        0.716_323_907_620_832_4,
        -0.434_960_876_320_982_85,
        0.197_060_061_107_739,
        -0.061_187_155_202_560_83,
        0.011_540_118_361_536_728,
        -0.000_994_579_203_040_892_3,
    ],
    [
        1.190_206_838_133_806_2,
        -1.481_614_893_619_197_3,
        1.162_633_765_082_819_8,
        -0.809_249_611_070_596_6,
        0.508_493_234_921_956_9,
        -0.260_921_158_404_262_17,
        0.099_705_413_493_167_82,
        -0.026_109_560_373_691_59,
        0.004_153_309_852_946_963,
        -0.000_301_914_967_673_213_9,
    ],
    [
        1.185_765_765_629_293,
        -1.453_448_968_570_910_9,
        1.083_102_145_105_459_4,
        -0.678_019_217_115_069_1,
        0.369_048_201_760_097_56,
        -0.161_966_732_554_144_4,
        0.052_810_685_885_363_105,
        -0.011_798_688_338_672_226,
        0.001_601_474_269_917_563_6,
        -0.000_099_346_672_044_168_82,
    ],
    [
        1.180_259_630_237_092_2,
        -1.423_444_976_962_555_5,
        1.010_329_712_593_077_7,
        -0.574_907_019_226_792_5,
        0.274_988_230_724_238_3,
        -0.104_681_507_509_172_31,
        0.029_518_035_846_592_745,
        -0.005_701_446_873_313_074,
        0.000_669_120_041_415_416_1,
        -0.000_035_893_495_149_683_49,
    ],
    [
        1.173_256_828_508_576_5,
        -1.390_342_100_065_097,
        0.940_689_253_731_647_9,
        -0.489_329_577_123_905_7,
        0.207_293_384_465_220_05,
        -0.068_934_258_893_900_31,
        0.016_916_692_670_395_12,
        -0.002_842_018_651_551_579,
        0.000_290_131_585_299_070_23,
        -0.000_013_539_732_121_325_341,
    ],
    [
        1.164_159_355_564_407_9,
        -1.352_887_714_339_140_7,
        0.872_066_247_288_997_4,
        -0.415_891_395_234_157_7,
        0.156_704_365_490_561_6,
        -0.045_671_290_120_816_15,
        0.009_775_916_608_003_197,
        -0.001_431_111_437_142_206_9,
        0.000_127_306_605_405_880_58,
        -5.177_789_969_024_283e-6,
    ],
    [
        1.152_518_457_179_454_7,
        -1.311_039_882_793_061_4,
        0.805_121_297_655_190_6,
        -0.353_342_350_166_347_6,
        0.119_087_866_702_095_99,
        -0.030_571_074_893_307_85,
        0.005_729_861_021_903_609,
        -0.000_733_319_443_853_482_7,
        0.000_057_021_620_567_669_024,
        -2.027_606_013_420_976_6e-6,
    ],
];

/// Taylor expansion for \\(\Li_{3} e\^{-x}\\) around \(x = +\infty\).  This is
/// valid until \\(x \approx 2.85 \\) at which point the floating point error
/// becomes larger.  Note that the polynomial is in \\(e\^{-x}\\) as opposed to
/// \\(x\\).
const BOSE_EINSTEIN_INFINITY: [f64; 11] = [
    0.0,
    1.0,
    0.125,
    0.037_037_037_037_037_035,
    0.015_625,
    0.008,
    0.004_629_629_629_629_629,
    0.002_915_451_895_043_731_7,
    0.001_953_125,
    0.001_371_742_112_482_853_1,
    0.001,
];

/// Taylor expansion for \\(-\Li_{3} -e\^{x}\\) around \(x = -\infty\).  This is
/// valid until \\(x \approx -2.6 \\) at which point the floating point error
/// becomes larger.  Note that the polynomial is in \\(e\^{x}\\) as opposed to
/// \\(x\\).
const FERMI_DIRAC_NEG_INFINITY: [f64; 12] = [
    0.0,
    1.0,
    -0.125,
    0.037_037_037_037_037_035,
    -0.015_625,
    0.008,
    -0.004_629_629_629_629_629,
    0.002_915_451_895_043_731_7,
    -0.001_953_125,
    0.001_371_742_112_482_853_1,
    -0.001,
    0.000_751_314_800_901_577_8,
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
        0.901_548_732_990_022_8,
        0.822_419_769_924_370_5,
        0.346_309_050_740_792_16,
        0.082_784_178_779_805_57,
        0.009_771_807_557_769_036,
        -0.000_482_348_722_046_307_4,
        -0.000_411_404_201_780_193_3,
        -0.000_075_135_927_716_277_57,
        -6.820_372_258_827_072_5e-6,
        -2.640_018_648_052_557_7e-7,
    ],
    [
        0.901_503_058_094_498_2,
        0.822_246_890_753_007_3,
        0.346_020_578_510_343_57,
        0.082_506_276_340_257_98,
        0.009_602_002_551_589_543,
        -0.000_550_284_528_127_892_9,
        -0.000_429_076_938_953_908_3,
        -0.000_077_985_365_258_309_95,
        -7.073_343_025_067_766e-6,
        -2.730_017_709_660_523e-7,
    ],
    [
        0.901_536_535_646_938,
        0.822_424_392_349_900_6,
        0.346_440_720_566_563_2,
        0.083_089_020_252_824_04,
        0.010_124_006_475_236_46,
        -0.000_237_105_533_924_753_55,
        -0.000_303_232_782_437_027_7,
        -0.000_045_327_776_394_798_3,
        -2.107_230_858_171_591e-6,
        6.411_885_495_585_187e-8,
    ],
    [
        0.901_542_520_661_083,
        0.822_465_508_212_429_1,
        0.346_566_981_334_894_6,
        0.083_316_612_926_613_02,
        0.010_389_518_131_662_721,
        -0.000_029_122_420_604_746_842,
        -0.000_193_797_967_267_937_92,
        -8.019_784_729_888_557e-6,
        5.372_040_431_469_726e-6,
        7.359_692_885_053_077e-7,
    ],
    [
        0.901_542_677_484_553,
        0.822_467_035_854_616_1,
        0.346_573_613_267_966_8,
        0.083_333_461_554_499_26,
        0.010_417_135_022_197_963,
        1.175_270_021_008_520_7e-6,
        -0.000_171_547_179_590_128_6,
        2.527_881_990_338_24e-6,
        8.299_232_582_079_652e-6,
        1.097_965_483_309_461_7e-6,
    ],
    [
        0.901_542_677_369_695_6,
        0.822_467_033_424_110_4,
        0.346_573_590_280_028_27,
        0.083_333_333_335_061_97,
        0.010_416_666_659_813_657,
        -2.938_756_009_353_559_5e-10,
        -0.000_173_612_005_616_834_68,
        1.258_039_810_613_248_4e-8,
        6.304_272_302_193_683e-6,
        2.921_259_915_319_22e-7,
    ],
    [
        0.901_542_677_370_627_4,
        0.822_467_033_385_379_6,
        0.346_573_590_971_784_6,
        0.083_333_326_332_682_52,
        0.010_416_711_231_074_35,
        -1.871_624_631_855_812e-7,
        -0.000_173_083_346_464_316_53,
        -9.931_349_873_127_676e-7,
        7.405_756_503_231_595_5e-6,
        -8.672_361_758_883_497e-7,
    ],
    [
        0.901_542_665_575_511,
        0.822_467_171_496_523_4,
        0.346_572_879_102_676_65,
        0.083_335_434_485_518_44,
        0.010_412_801_147_992_194,
        4.414_853_398_905_329_6e-6,
        -0.000_176_327_659_017_971_62,
        8.277_459_381_884_172e-8,
        7.474_360_215_077_891e-6,
        -9.833_247_103_813_855e-7,
    ],
    [
        0.901_539_386_113_096_7,
        0.822_491_486_159_532_5,
        0.346_492_233_987_360_84,
        0.083_492_651_807_469_63,
        0.010_214_038_765_093_63,
        0.000_173_611_829_084_646_42,
        -0.000_273_421_250_403_937_7,
        0.000_036_341_881_914_419_53,
        -5.293_977_893_178_692e-7,
        -1.870_839_390_209_571_3e-7,
    ],
    [
        0.901_508_750_653_860_4,
        0.822_659_638_668_424_5,
        0.346_079_741_582_401_57,
        0.084_086_295_214_422_12,
        0.009_661_615_178_942_463,
        0.000_518_332_980_053_707_8,
        -0.000_417_667_838_726_450_6,
        0.000_075_368_246_188_467_7,
        -6.723_388_510_773_663e-6,
        2.522_183_204_547_332e-7,
    ],
    [
        0.901_510_046_800_408,
        0.822_664_288_204_647_1,
        0.346_050_683_446_701_1,
        0.084_142_007_357_900_5,
        0.009_604_289_718_218_535,
        0.000_554_410_557_403_665,
        -0.000_432_062_898_022_454_83,
        0.000_078_941_848_921_002_5,
        -7.229_235_944_383_016_6e-6,
        2.835_164_855_977_137e-7,
    ],
];

/// Taylor expansion for \\(-\Li_{3} -e\^{x}\\) around \(x = +\infty\).  This is
/// valid until \\(x \approx -2.6 \\) at which point the floating point error
/// becomes larger.  Note that the polynomial is in \\(e\^{x}\\) as opposed to
/// \\(x\\).  There is an addition factor of:
///
/// ```ignore
/// 0.16666666666666666667 * x.powi(3) + 1.644_934_066_848_226_4 * x
/// ```
/// which must be added for this to be accurate.
const FERMI_DIRAC_INFINITY: [f64; 11] = [
    0.0,
    1.0,
    -0.125,
    0.037_037_037_037_037_035,
    -0.015_625,
    0.008,
    -0.004_629_629_629_629_629,
    0.002_915_451_895_043_731_7,
    -0.001_953_125,
    0.001_371_742_112_482_853_1,
    -0.001,
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
        log::debug!("Using exact expression at x = 0.");
        ZETA_3
    } else if x < 0.22 {
        log::debug!("Using Taylor series around x = 0.");
        polynomial(x, &BOSE_EINSTEIN_ZERO) - 0.5 * x.powi(2) * x.ln()
    } else if x > 2.85 {
        log::debug!("Using Taylor series around x = ∞.");
        let ex = (-x).exp();
        polynomial(ex, &BOSE_EINSTEIN_INFINITY)
    } else {
        let (i, x_lim) = BOSE_EINSTEIN_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between 0.22 and 2.85.");
        log::debug!("Using minimax polynomial for x < {}.", x_lim);
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
        log::debug!("Using Taylor series around x = -∞.");
        let ex = x.exp();
        polynomial(ex, &FERMI_DIRAC_NEG_INFINITY)
    } else if x > 2.5 {
        log::debug!("Using Taylor series around x = ∞.");
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
        log::debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(x, &FERMI_DIRAC_MINIMAX_COEFFICIENTS[i])
    }
}

#[cfg(test)]
mod test {
    use crate::utilities::test::*;
    use csv;

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
    use crate::utilities::test::*;
    use csv;
    use test::Bencher;

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
