//! Polylogarithms functions

use crate::polynomial::approximation;
use std::f64;

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
    -(1.0 - x).ln()
}

/// Approximation of polylogarithm function \\(Li_2(x)\\) for all \\(x < 1\\).
pub fn li2(x: f64) -> f64 {
    approximation(x, &li2::NUMERATORS, &li2::DENOMINATORS, &li2::SPLITS)
        + if x < li2::SPLITS[1] {
            li2::lower(x)
        } else if x > li2::SPLITS[li2::SPLITS.len() - 2] {
            li2::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_3(x)\\) for all \\(x < 1\\).
pub fn li3(x: f64) -> f64 {
    approximation(x, &li3::NUMERATORS, &li3::DENOMINATORS, &li3::SPLITS)
        + if x < li3::SPLITS[1] {
            li3::lower(x)
        } else if x > li3::SPLITS[li3::SPLITS.len() - 2] {
            li3::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_4(x)\\) for all \\(x < 1\\).
pub fn li4(x: f64) -> f64 {
    approximation(x, &li4::NUMERATORS, &li4::DENOMINATORS, &li4::SPLITS)
        + if x < li4::SPLITS[1] {
            li4::lower(x)
        } else if x > li4::SPLITS[li4::SPLITS.len() - 2] {
            li4::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_5(x)\\) for all \\(x < 1\\).
pub fn li5(x: f64) -> f64 {
    approximation(x, &li5::NUMERATORS, &li5::DENOMINATORS, &li5::SPLITS)
        + if x < li5::SPLITS[1] {
            li5::lower(x)
        } else if x > li5::SPLITS[li5::SPLITS.len() - 2] {
            li5::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_6(x)\\) for all \\(x < 1\\).
pub fn li6(x: f64) -> f64 {
    approximation(x, &li6::NUMERATORS, &li6::DENOMINATORS, &li6::SPLITS)
        + if x < li6::SPLITS[1] {
            li6::lower(x)
        } else if x > li6::SPLITS[li6::SPLITS.len() - 2] {
            li6::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_7(x)\\) for all \\(x < 1\\).
pub fn li7(x: f64) -> f64 {
    approximation(x, &li7::NUMERATORS, &li7::DENOMINATORS, &li7::SPLITS)
        + if x < li7::SPLITS[1] {
            li7::lower(x)
        } else if x > li7::SPLITS[li7::SPLITS.len() - 2] {
            li7::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_8(x)\\) for all \\(x < 1\\).
pub fn li8(x: f64) -> f64 {
    approximation(x, &li8::NUMERATORS, &li8::DENOMINATORS, &li8::SPLITS)
        + if x < li8::SPLITS[1] {
            li8::lower(x)
        } else if x > li8::SPLITS[li8::SPLITS.len() - 2] {
            li8::upper(x)
        } else {
            0.0
        }
}

/// Approximation of polylogarithm function \\(Li_9(x)\\) for all \\(x < 1\\).
pub fn li9(x: f64) -> f64 {
    approximation(x, &li9::NUMERATORS, &li9::DENOMINATORS, &li9::SPLITS)
        + if x < li9::SPLITS[1] {
            li9::lower(x)
        } else if x > li9::SPLITS[li9::SPLITS.len() - 2] {
            li9::upper(x)
        } else {
            0.0
        }
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use csv;
    use std::f64;

    #[test]
    fn li0() {
        let mut rdr = csv::Reader::from_path("tests/data/polylog/li0.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                println!("x = {}", x);
                let n = super::li0(x);
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
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
                approx_eq(n, v, 10.0, 0.0);
            }
        }
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod benches {
    use crate::utilities::test::*;
    use csv;
    use test::Bencher;

    #[bench]
    fn li0(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/polylog/li0.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::li0(x);
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
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
                    approx_eq(n, v, 10.0, 0.0);
                }
            }
        });
    }
}
