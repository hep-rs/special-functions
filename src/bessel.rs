//! Bessel functions

use crate::polynomial::approximation;
use std::f64;

mod k0;
mod k1;
mod k2;
mod k3;
mod k4;
mod k5;
mod k6;
mod k7;
mod k8;
mod k9;

/// Approximation of modified Bessel function \\(K_0(x)\\) for all \\(x >
/// 0\\).
pub fn k0(x: f64) -> f64 {
    approximation(x, &k0::NUMERATORS, &k0::DENOMINATORS, &k0::SPLITS)
        + if x < k0::SPLITS[1] {
            k0::lower(x)
        } else if x > k0::SPLITS[k0::SPLITS.len() - 2] {
            k0::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_1(x)\\) for all \\(x >
/// 0\\).
pub fn k1(x: f64) -> f64 {
    approximation(x, &k1::NUMERATORS, &k1::DENOMINATORS, &k1::SPLITS)
        + if x < k1::SPLITS[1] {
            k1::lower(x)
        } else if x > k1::SPLITS[k1::SPLITS.len() - 2] {
            k1::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_2(x)\\) for all \\(x >
/// 0\\).
pub fn k2(x: f64) -> f64 {
    approximation(x, &k2::NUMERATORS, &k2::DENOMINATORS, &k2::SPLITS)
        + if x < k2::SPLITS[1] {
            k2::lower(x)
        } else if x > k2::SPLITS[k2::SPLITS.len() - 2] {
            k2::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_3(x)\\) for all \\(x >
/// 0\\).
pub fn k3(x: f64) -> f64 {
    approximation(x, &k3::NUMERATORS, &k3::DENOMINATORS, &k3::SPLITS)
        + if x < k3::SPLITS[1] {
            k3::lower(x)
        } else if x > k3::SPLITS[k3::SPLITS.len() - 2] {
            k3::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_4(x)\\) for all \\(x >
/// 0\\).
pub fn k4(x: f64) -> f64 {
    approximation(x, &k4::NUMERATORS, &k4::DENOMINATORS, &k4::SPLITS)
        + if x < k4::SPLITS[1] {
            k4::lower(x)
        } else if x > k4::SPLITS[k4::SPLITS.len() - 2] {
            k4::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_5(x)\\) for all \\(x >
/// 0\\).
pub fn k5(x: f64) -> f64 {
    approximation(x, &k5::NUMERATORS, &k5::DENOMINATORS, &k5::SPLITS)
        + if x < k5::SPLITS[1] {
            k5::lower(x)
        } else if x > k5::SPLITS[k5::SPLITS.len() - 2] {
            k5::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_6(x)\\) for all \\(x >
/// 0\\).
pub fn k6(x: f64) -> f64 {
    approximation(x, &k6::NUMERATORS, &k6::DENOMINATORS, &k6::SPLITS)
        + if x < k6::SPLITS[1] {
            k6::lower(x)
        } else if x > k6::SPLITS[k6::SPLITS.len() - 2] {
            k6::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_7(x)\\) for all \\(x >
/// 0\\).
pub fn k7(x: f64) -> f64 {
    approximation(x, &k7::NUMERATORS, &k7::DENOMINATORS, &k7::SPLITS)
        + if x < k7::SPLITS[1] {
            k7::lower(x)
        } else if x > k7::SPLITS[k7::SPLITS.len() - 2] {
            k7::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_8(x)\\) for all \\(x >
/// 0\\).
pub fn k8(x: f64) -> f64 {
    approximation(x, &k8::NUMERATORS, &k8::DENOMINATORS, &k8::SPLITS)
        + if x < k8::SPLITS[1] {
            k8::lower(x)
        } else if x > k8::SPLITS[k8::SPLITS.len() - 2] {
            k8::upper(x)
        } else {
            0.0
        }
}

/// Approximation of modified Bessel function \\(K_9(x)\\) for all \\(x >
/// 0\\).
pub fn k9(x: f64) -> f64 {
    approximation(x, &k9::NUMERATORS, &k9::DENOMINATORS, &k9::SPLITS)
        + if x < k9::SPLITS[1] {
            k9::lower(x)
        } else if x > k9::SPLITS[k9::SPLITS.len() - 2] {
            k9::upper(x)
        } else {
            0.0
        }
}

#[cfg(test)]
mod test {
    use crate::utilities::test::*;
    use csv;
    use std::f64;

    #[test]
    fn k0() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k0.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                println!("x = {}", x);
                let n = super::k0(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k1() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k1.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k1(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k2() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k2.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k2(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k3() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k3.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k3(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k4() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k4.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k4(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k5() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k5.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k5(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k6() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k6.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k6(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k7() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k7.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k7(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k8() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k8.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k8(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    #[test]
    fn k9() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k9.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k9(x);
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }

    // #[test]
    // fn k1_on_k2() {
    //     let mut rdr = csv::Reader::from_path("tests/data/bessel_k1_on_k2.csv").unwrap();

    //     for result in rdr.deserialize() {
    //         let (x, v): (f64, f64) = result.unwrap();
    //         println!("x = {:.2e}, v = {:.2e}", x, v);

    //         if !v.is_nan() {
    //             let n = super::k1_on_k2(x);
    //             approx_eq(n, v, 11.0, 0.0);
    //         }
    //     }

    //     assert_eq!(super::k1_on_k2(0.0), 0.0);
    // }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use crate::utilities::test::*;
    use csv;
    use test::Bencher;

    #[bench]
    fn k0(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k0.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k0(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k1(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k1.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k1(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k2(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k2.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k2(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k3(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k3.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k3(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k4(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k4.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k4(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k5(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k5.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k5(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k6(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k6.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k6(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k7(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k7.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k7(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k8(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k8.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k8(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k9(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k9.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k9(x);
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }

    // #[bench]
    // fn k1_on_k2(b: &mut Bencher) {
    //     let rdr = csv::Reader::from_path("tests/data/bessel_k1_on_k2.csv").unwrap();
    //     let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

    //     b.iter(|| {
    //         for &(x, v) in &data {
    //             if !v.is_nan() {
    //                 let n = super::k1_on_k2(x);
    //                 approx_eq(n, v, 11.0, 0.0);
    //             }
    //         }
    //     });
    // }
}
