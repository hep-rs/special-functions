//! Polylogarithms functions

use approx_fn;

mod li2;
mod li3;
mod li4;
mod li5;
mod li6;
mod li7;
mod li8;
mod li9;

/// Approximation of polylogarithm function \\(Li_0(x)\\) for all \\(x < 1\\).
pub fn li0(x: f64) -> f64 {
    x / (1.0 - x)
}

/// Approximation of polylogarithm function \\(Li_1(x)\\) for all \\(x < 1\\).
pub fn li1(x: f64) -> f64 {
    -(-x).ln_1p()
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_2(x)\\) for all \\(x < 1\\)."]
    (pub) fn li2(mod = li2, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_3(x)\\) for all \\(x < 1\\)."]
    (pub) fn li3(mod = li3, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_4(x)\\) for all \\(x < 1\\)."]
    (pub) fn li4(mod = li4, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_5(x)\\) for all \\(x < 1\\)."]
    (pub) fn li5(mod = li5, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_6(x)\\) for all \\(x < 1\\)."]
    (pub) fn li6(mod = li6, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_7(x)\\) for all \\(x < 1\\)."]
    (pub) fn li7(mod = li7, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_8(x)\\) for all \\(x < 1\\)."]
    (pub) fn li8(mod = li8, type = chebyshev);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_9(x)\\) for all \\(x < 1\\)."]
    (pub) fn li9(mod = li9, type = chebyshev);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    fn li0() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li0.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li0(x);
                approx_eq(n, v, 9.0, 0.0);
            }
        }
    }

    #[test]
    fn li1() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li1.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li1(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li2() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li2.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li2(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li3() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li3.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li3(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li4() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li4.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li4(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li5() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li5.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li5(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li6() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li6.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li6(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li7() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li7.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li7(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li8() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li8.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li8(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }

    #[test]
    fn li9() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li9.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::li9(x);
                approx_eq(n, v, 9.0, 10f64.powi(-10));
            }
        }
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod benches {
    use test::Bencher;

    #[bench]
    fn li0(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li0.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li0(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li1(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li1.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li1(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li2(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li2.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li2(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li3(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li3.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li3(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li4(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li4.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li4(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li5(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li5.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li5(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li6(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li6.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li6(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li7(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li7.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li7(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li8(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li8.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li8(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn li9(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li9.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li9(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }
}
