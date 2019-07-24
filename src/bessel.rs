//! Bessel functions

use approx_fn;

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

approx_fn! {
   #[doc = "Approximation of modified Bessel function \\(K_0(x)\\) for all \\(x > 0\\)."]
   (pub) fn k0(mod = k0, type = chebyshev);
}

approx_fn! {
   #[doc = "Approximation of modified Bessel function \\(K_1(x)\\) for all \\(x > 0\\)."]
   (pub) fn k1(mod = k1, type = chebyshev);
}

approx_fn! {    #[doc = "Approximation of modified Bessel function \\(K_2(x)\\) for all \\(x > 0\\)."]
    (pub) fn k2(mod = k2, type = chebyshev);
}

approx_fn! {     #[doc = "Approximation of modified Bessel function \\(K_3(x)\\) for all \\(x > 0\\)."]
   (pub) fn k3(mod = k3, type = chebyshev);
}

approx_fn! { #[doc = "Approximation of modified Bessel function \\(K_4(x)\\) for all \\(x > 0\\)."]
   (pub) fn k4(mod = k4, type = chebyshev);
}

approx_fn! { #[doc = "Approximation of modified Bessel function \\(K_5(x)\\) for all \\(x > 0\\)."]
   (pub) fn k5(mod = k5, type = chebyshev);
}

approx_fn! {     #[doc = "Approximation of modified Bessel function \\(K_6(x)\\) for all \\(x > 0\\)."]
   (pub) fn k6(mod = k6, type = chebyshev);
}

approx_fn! {
   #[doc = "Approximation of modified Bessel function \\(K_7(x)\\) for all \\(x > 0\\)."]
   (pub) fn k7(mod = k7, type = chebyshev);
}

approx_fn! {
   #[doc = "Approximation of modified Bessel function \\(K_8(x)\\) for all \\(x > 0\\)."]
   (pub) fn k8(mod = k8, type = chebyshev);
}

approx_fn! {
   #[doc = "Approximation of modified Bessel function \\(K_9(x)\\) for all \\(x > 0\\)."]
   (pub) fn k9(mod = k9, type = chebyshev);
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
    use test::Bencher;

    #[bench]
    fn k0(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel/k0.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k0(x);
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
                    test::black_box((x, v, n));
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
    //                 test::black_box((x, v, n));
    //             }
    //         }
    //     });
    // }
}
