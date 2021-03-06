use super::{log_diff, Parameters};

pub(crate) fn d0001(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.5 * std::f64::consts::PI
            * ((param.m2_2 * param.s1 * param.s12
                + param.m0_2 * param.s1 * param.s2
                + param.m2_2 * param.s1 * param.s2
                + param.m0_2 * param.s12 * param.s2
                + -2. * param.s1 * param.s12 * param.s2
                + param.m3_2
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + -2. * param.s12 * (param.s1 + param.s2))
                + param.m2_2 * param.s1 * param.s23
                + param.m0_2 * param.s12 * param.s23
                + param.m2_2 * param.s12 * param.s23
                + param.s1 * param.s12 * param.s23
                + param.m0_2 * param.s2 * param.s23
                + param.s12 * param.s2 * param.s23
                + param.m0_2 * param.s1 * param.s3
                + -2. * param.m2_2 * param.s1 * param.s3
                + param.s1 * param.s12 * param.s3
                + param.m0_2 * param.s2 * param.s3
                + param.s1 * param.s2 * param.s3
                + param.m2_2 * param.s1 * param.s4
                + -2. * param.m0_2 * param.s2 * param.s4
                + param.m2_2 * param.s2 * param.s4
                + param.s1 * param.s2 * param.s4
                + param.s12 * param.s2 * param.s4
                + param.m1_2
                    * ((param.s1 - param.s2) * (param.s3 - param.s4)
                        + param.s12
                            * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                        - param.s12.powi(2))
                - param.s2.powi(2) * param.s4
                - param.m2_2 * param.s12 * param.s4
                - param.m0_2 * param.s12 * param.s3
                - param.s1.powi(2) * param.s3
                - param.m2_2 * param.s2 * param.s23
                - param.s12.powi(2) * param.s23
                - param.m0_2 * param.s1 * param.s23
                - param.m0_2 * param.s2.powi(2)
                - param.m2_2 * param.s1.powi(2))
                * (param.m2_2.powi(2) * param.s1.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                    + param.m0_2.powi(2) * param.s2.powi(2)
                    + param.m3_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2))
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                    + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                    + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                    + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                    + param.m0_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                    + param.m2_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                    + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                    + param.s12.powi(2) * param.s23.powi(2)
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                    + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                    + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                    + -2. * param.s1 * param.s12 * param.s23 * param.s3
                    + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                    + param.m0_2.powi(2) * param.s3.powi(2)
                    + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                    + param.s1.powi(2) * param.s3.powi(2)
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                    + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                    + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                    + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                    + -2. * param.s12 * param.s2 * param.s23 * param.s4
                    + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                    + -2. * param.s1 * param.s2 * param.s3 * param.s4
                    + param.m2_2.powi(2) * param.s4.powi(2)
                    + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                    + param.s2.powi(2) * param.s4.powi(2)
                    + param.m1_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s3 - param.s4).powi(2)
                            + -2. * param.s12 * (param.s3 + param.s4))
                    + 2. * param.m3_2
                        * (-2. * param.s1 * param.s12 * param.s2
                            + param.s1 * param.s12 * param.s23
                            + param.s12 * param.s2 * param.s23
                            + param.s1 * param.s12 * param.s3
                            + param.s1 * param.s2 * param.s3
                            + param.m0_2
                                * (param.s12 * (param.s2 + param.s23 - param.s3)
                                    + param.s1 * (param.s2 + param.s3 - param.s23)
                                    + param.s2
                                        * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                            + param.s1 * param.s2 * param.s4
                            + param.s12 * param.s2 * param.s4
                            + param.m2_2
                                * (param.s12 * (param.s1 + param.s23 - param.s4)
                                    + param.s2 * (param.s4 - param.s23)
                                    + param.s1
                                        * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                                    - param.s1.powi(2))
                            - param.s2.powi(2) * param.s4
                            - param.s1.powi(2) * param.s3
                            - param.s12.powi(2) * param.s23)
                    + -2.
                        * param.m1_2
                        * (param.m0_2 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s23
                            + 2. * param.m0_2 * param.s1 * param.s3
                            + param.m0_2 * param.s3.powi(2)
                            + param.s1 * param.s3.powi(2)
                            + param.m0_2 * param.s23 * param.s4
                            + 2. * param.s12 * param.s3 * param.s4
                            + param.s2 * param.s4.powi(2)
                            + param.m3_2
                                * (param.s12.powi(2)
                                    - param.s12
                                        * (param.s1
                                            + param.s2
                                            + -2. * param.s23
                                            + param.s3
                                            + param.s4)
                                    - (param.s1 - param.s2) * (param.s3 - param.s4))
                            - param.m2_2
                                * (-2. * param.s2 * param.s4
                                    + param.s23 * param.s4
                                    + param.s3 * param.s4
                                    + param.s12 * (param.s23 + param.s4 - param.s1)
                                    + param.s1 * (param.s3 + param.s4)
                                    - param.s4.powi(2)
                                    - param.s23 * param.s3)
                            - param.s2 * param.s3 * param.s4
                            - param.s1 * param.s3 * param.s4
                            - param.m0_2 * param.s3 * param.s4
                            - param.s12 * param.s23 * param.s4
                            - param.s12 * param.s2 * param.s4
                            - param.m0_2 * param.s2 * param.s4
                            - param.s12 * param.s23 * param.s3
                            - param.m0_2 * param.s23 * param.s3
                            - param.m0_2 * param.s2 * param.s3
                            - param.s1 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s23))
                    .sqrt()
                    .powi(-1)
                * log_diff(
                    param.m1_2
                        * (param.s1 * param.s12
                            + param.m3_2 * (param.s1 + param.s12 - param.s2)
                            + param.s12 * param.s23
                            + -2. * param.s1 * param.s3
                            + param.s1 * param.s4
                            + -2. * param.s12 * param.s4
                            + param.s2 * param.s4
                            + param.m2_2 * (param.s1 + param.s4 - param.s23))
                        + param.s1
                            * (param.m3_2 * (param.s12 + param.s2 - param.s1)
                                + param.s1 * param.s3
                                + param.m2_2
                                    * (-2. * param.m3_2 + param.s23 + param.s4 - param.s1)
                                - param.s2 * param.s4
                                - param.s12 * param.s23)
                        + param.m0_2
                            * (param.m3_2 * param.s1
                                + param.m3_2 * param.s2
                                + param.s1 * param.s2
                                + param.s1 * param.s23
                                + param.s12 * param.s23
                                + -2. * param.s2 * param.s23
                                + -2. * param.s1 * param.s3
                                + param.m2_2 * (param.s1 + param.s23 - param.s4)
                                + param.s2 * param.s4
                                + param.m1_2
                                    * (-2. * param.s1
                                        + param.s12
                                        + param.s2
                                        + param.s23
                                        + -2. * param.s3
                                        + param.s4)
                                - param.m3_2 * param.s12)
                        - param.m1_2.powi(2) * (param.s12 + param.s4 - param.s3)
                        - param.m0_2.powi(2) * (param.s2 + param.s23 - param.s3),
                    -1. * (param.m2_2.powi(2) * param.s1.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                        + param.m0_2.powi(2) * param.s2.powi(2)
                        + param.m3_2.powi(2)
                            * (param.s1.powi(2)
                                + (param.s12 - param.s2).powi(2)
                                + -2. * param.s1 * (param.s12 + param.s2))
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                        + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                        + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                        + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                        + param.m0_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                        + param.m2_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                        + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                        + param.s12.powi(2) * param.s23.powi(2)
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                        + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                        + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                        + -2. * param.s1 * param.s12 * param.s23 * param.s3
                        + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                        + param.m0_2.powi(2) * param.s3.powi(2)
                        + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                        + param.s1.powi(2) * param.s3.powi(2)
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                        + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                        + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                        + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                        + -2. * param.s12 * param.s2 * param.s23 * param.s4
                        + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                        + -2. * param.s1 * param.s2 * param.s3 * param.s4
                        + param.m2_2.powi(2) * param.s4.powi(2)
                        + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                        + param.s2.powi(2) * param.s4.powi(2)
                        + param.m1_2.powi(2)
                            * (param.s12.powi(2)
                                + (param.s3 - param.s4).powi(2)
                                + -2. * param.s12 * (param.s3 + param.s4))
                        + 2. * param.m3_2
                            * (-2. * param.s1 * param.s12 * param.s2
                                + param.s1 * param.s12 * param.s23
                                + param.s12 * param.s2 * param.s23
                                + param.s1 * param.s12 * param.s3
                                + param.s1 * param.s2 * param.s3
                                + param.m0_2
                                    * (param.s12 * (param.s2 + param.s23 - param.s3)
                                        + param.s1 * (param.s2 + param.s3 - param.s23)
                                        + param.s2
                                            * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                                + param.s1 * param.s2 * param.s4
                                + param.s12 * param.s2 * param.s4
                                + param.m2_2
                                    * ((param.s12 - param.s2) * (param.s23 - param.s4)
                                        + param.s1
                                            * (param.s12
                                                + param.s2
                                                + param.s23
                                                + -2. * param.s3
                                                + param.s4)
                                        - param.s1.powi(2))
                                - param.s2.powi(2) * param.s4
                                - param.s1.powi(2) * param.s3
                                - param.s12.powi(2) * param.s23)
                        + -2.
                            * param.m1_2
                            * (param.m0_2 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s23
                                + 2. * param.m0_2 * param.s1 * param.s3
                                + param.m0_2 * param.s3.powi(2)
                                + param.s1 * param.s3.powi(2)
                                + param.m0_2 * param.s23 * param.s4
                                + 2. * param.s12 * param.s3 * param.s4
                                + param.s2 * param.s4.powi(2)
                                + param.m3_2
                                    * (param.s12.powi(2)
                                        - param.s12
                                            * (param.s1
                                                + param.s2
                                                + -2. * param.s23
                                                + param.s3
                                                + param.s4)
                                        - (param.s1 - param.s2) * (param.s3 - param.s4))
                                - param.m2_2
                                    * (-2. * param.s2 * param.s4
                                        + param.s23 * param.s4
                                        + param.s3 * param.s4
                                        + param.s12 * (param.s23 + param.s4)
                                        + param.s1 * (param.s3 + param.s4 - param.s12)
                                        - param.s4.powi(2)
                                        - param.s23 * param.s3)
                                - param.s2 * param.s3 * param.s4
                                - param.s1 * param.s3 * param.s4
                                - param.m0_2 * param.s3 * param.s4
                                - param.s12 * param.s23 * param.s4
                                - param.s12 * param.s2 * param.s4
                                - param.m0_2 * param.s2 * param.s4
                                - param.s12 * param.s23 * param.s3
                                - param.m0_2 * param.s23 * param.s3
                                - param.m0_2 * param.s2 * param.s3
                                - param.s1 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s23))
                        .sqrt()
                        * param.lambda_m01_sqrt,
                )
                + param.lambda_s12_sqrt
                    * log_diff(
                        param.m1_2 * (param.s1 + param.s12 - param.s2)
                            + param.m0_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    )
                + param.s12
                    * (param.s1 + param.s23 - param.s4)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s23 - param.s4)
                            + param.m1_2 * (param.s1 + param.s4 - param.s23)
                            + param.s1 * (-2. * param.m3_2 + param.s23 + param.s4 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s14_sqrt,
                    )
                + param.s2
                    * (param.s4 - param.s23)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s23 - param.s4)
                            + param.m1_2 * (param.s1 + param.s4 - param.s23)
                            + param.s1 * (-2. * param.m3_2 + param.s23 + param.s4 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s14_sqrt,
                    )
                + param.s1
                    * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s23 - param.s4)
                            + param.m1_2 * (param.s1 + param.s4 - param.s23)
                            + param.s1 * (-2. * param.m3_2 + param.s23 + param.s4 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s14_sqrt,
                    )
                - param.s1.powi(2)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s23 - param.s4)
                            + param.m1_2 * (param.s1 + param.s4 - param.s23)
                            + param.s1 * (-2. * param.m3_2 + param.s23 + param.s4 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s14_sqrt,
                    ))
            / (param.s12.powi(2) * param.s23
                + param.s1.powi(2) * param.s3
                + param.s2
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s4 * (param.s2 + param.s4 - param.s3))
                - param.s12
                    * (param.s1 * (param.s23 + param.s3 - param.s2)
                        + (param.s2 - param.s3) * param.s4
                        + param.s23 * (param.s2 + param.s3 + param.s4)
                        - param.s23.powi(2))
                - param.s1
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s3 * (param.s4 - param.s3)
                        + param.s2 * (param.s3 + param.s4)))
    } else {
        0.0
    }) + (if param.s12 > (param.m0 + param.m2).powi(2) {
        0.5 * std::f64::consts::PI
            * ((param.m2_2 * param.s1 * param.s12
                + param.m0_2 * param.s1 * param.s2
                + param.m2_2 * param.s1 * param.s2
                + param.m0_2 * param.s12 * param.s2
                + -2. * param.s1 * param.s12 * param.s2
                + param.m3_2
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + -2. * param.s12 * (param.s1 + param.s2))
                + param.m2_2 * param.s1 * param.s23
                + param.m0_2 * param.s12 * param.s23
                + param.m2_2 * param.s12 * param.s23
                + param.s1 * param.s12 * param.s23
                + param.m0_2 * param.s2 * param.s23
                + param.s12 * param.s2 * param.s23
                + param.m0_2 * param.s1 * param.s3
                + -2. * param.m2_2 * param.s1 * param.s3
                + param.s1 * param.s12 * param.s3
                + param.m0_2 * param.s2 * param.s3
                + param.s1 * param.s2 * param.s3
                + param.m2_2 * param.s1 * param.s4
                + -2. * param.m0_2 * param.s2 * param.s4
                + param.m2_2 * param.s2 * param.s4
                + param.s1 * param.s2 * param.s4
                + param.s12 * param.s2 * param.s4
                + param.m1_2
                    * ((param.s1 - param.s2) * (param.s3 - param.s4)
                        + param.s12
                            * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                        - param.s12.powi(2))
                - param.s2.powi(2) * param.s4
                - param.m2_2 * param.s12 * param.s4
                - param.m0_2 * param.s12 * param.s3
                - param.s1.powi(2) * param.s3
                - param.m2_2 * param.s2 * param.s23
                - param.s12.powi(2) * param.s23
                - param.m0_2 * param.s1 * param.s23
                - param.m0_2 * param.s2.powi(2)
                - param.m2_2 * param.s1.powi(2))
                * (param.m2_2.powi(2) * param.s1.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                    + param.m0_2.powi(2) * param.s2.powi(2)
                    + param.m3_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2))
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                    + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                    + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                    + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                    + param.m0_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                    + param.m2_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                    + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                    + param.s12.powi(2) * param.s23.powi(2)
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                    + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                    + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                    + -2. * param.s1 * param.s12 * param.s23 * param.s3
                    + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                    + param.m0_2.powi(2) * param.s3.powi(2)
                    + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                    + param.s1.powi(2) * param.s3.powi(2)
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                    + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                    + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                    + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                    + -2. * param.s12 * param.s2 * param.s23 * param.s4
                    + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                    + -2. * param.s1 * param.s2 * param.s3 * param.s4
                    + param.m2_2.powi(2) * param.s4.powi(2)
                    + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                    + param.s2.powi(2) * param.s4.powi(2)
                    + param.m1_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s3 - param.s4).powi(2)
                            + -2. * param.s12 * (param.s3 + param.s4))
                    + 2. * param.m3_2
                        * (-2. * param.s1 * param.s12 * param.s2
                            + param.s1 * param.s12 * param.s23
                            + param.s12 * param.s2 * param.s23
                            + param.s1 * param.s12 * param.s3
                            + param.s1 * param.s2 * param.s3
                            + param.m0_2
                                * (param.s12 * (param.s2 + param.s23 - param.s3)
                                    + param.s1 * (param.s2 + param.s3 - param.s23)
                                    + param.s2
                                        * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                            + param.s1 * param.s2 * param.s4
                            + param.s12 * param.s2 * param.s4
                            + param.m2_2
                                * (param.s12 * (param.s1 + param.s23 - param.s4)
                                    + param.s2 * (param.s4 - param.s23)
                                    + param.s1
                                        * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                                    - param.s1.powi(2))
                            - param.s2.powi(2) * param.s4
                            - param.s1.powi(2) * param.s3
                            - param.s12.powi(2) * param.s23)
                    + -2.
                        * param.m1_2
                        * (param.m0_2 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s23
                            + 2. * param.m0_2 * param.s1 * param.s3
                            + param.m0_2 * param.s3.powi(2)
                            + param.s1 * param.s3.powi(2)
                            + param.m0_2 * param.s23 * param.s4
                            + 2. * param.s12 * param.s3 * param.s4
                            + param.s2 * param.s4.powi(2)
                            + param.m3_2
                                * (param.s12.powi(2)
                                    - param.s12
                                        * (param.s1
                                            + param.s2
                                            + -2. * param.s23
                                            + param.s3
                                            + param.s4)
                                    - (param.s1 - param.s2) * (param.s3 - param.s4))
                            - param.m2_2
                                * (-2. * param.s2 * param.s4
                                    + param.s23 * param.s4
                                    + param.s3 * param.s4
                                    + param.s12 * (param.s23 + param.s4 - param.s1)
                                    + param.s1 * (param.s3 + param.s4)
                                    - param.s4.powi(2)
                                    - param.s23 * param.s3)
                            - param.s2 * param.s3 * param.s4
                            - param.s1 * param.s3 * param.s4
                            - param.m0_2 * param.s3 * param.s4
                            - param.s12 * param.s23 * param.s4
                            - param.s12 * param.s2 * param.s4
                            - param.m0_2 * param.s2 * param.s4
                            - param.s12 * param.s23 * param.s3
                            - param.m0_2 * param.s23 * param.s3
                            - param.m0_2 * param.s2 * param.s3
                            - param.s1 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s23))
                    .sqrt()
                    .powi(-1)
                * log_diff(
                    param.m2_2
                        * (param.s1 * param.s12
                            + param.m3_2 * (param.s1 + param.s12 - param.s2)
                            + -2. * param.s12 * param.s23
                            + param.s1 * param.s3
                            + -2. * param.s1 * param.s4
                            + param.s12 * param.s4
                            + param.s2 * param.s4
                            + param.m1_2 * (param.s12 + param.s4 - param.s3))
                        + param.s12
                            * (param.m3_2 * (param.s1 + param.s2 - param.s12)
                                + param.s12 * param.s23
                                + param.m1_2
                                    * (-2. * param.m3_2 + param.s3 + param.s4 - param.s12)
                                - param.s2 * param.s4
                                - param.s1 * param.s3)
                        + param.m0_2
                            * (param.m1_2 * param.s12
                                + param.s12 * param.s2
                                + param.m3_2 * (param.s12 + param.s2 - param.s1)
                                + -2. * param.s12 * param.s23
                                + param.m1_2 * param.s3
                                + param.s1 * param.s3
                                + param.s12 * param.s3
                                + -2. * param.s2 * param.s3
                                + param.s2 * param.s4
                                + param.m2_2
                                    * (param.s1
                                        + -2. * param.s12
                                        + param.s2
                                        + -2. * param.s23
                                        + param.s3
                                        + param.s4)
                                - param.m1_2 * param.s4)
                        - param.m2_2.powi(2) * (param.s1 + param.s4 - param.s23)
                        - param.m0_2.powi(2) * (param.s2 + param.s3 - param.s23),
                    -1. * (param.m2_2.powi(2) * param.s1.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                        + param.m0_2.powi(2) * param.s2.powi(2)
                        + param.m3_2.powi(2)
                            * (param.s1.powi(2)
                                + (param.s12 - param.s2).powi(2)
                                + -2. * param.s1 * (param.s12 + param.s2))
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                        + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                        + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                        + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                        + param.m0_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                        + param.m2_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                        + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                        + param.s12.powi(2) * param.s23.powi(2)
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                        + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                        + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                        + -2. * param.s1 * param.s12 * param.s23 * param.s3
                        + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                        + param.m0_2.powi(2) * param.s3.powi(2)
                        + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                        + param.s1.powi(2) * param.s3.powi(2)
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                        + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                        + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                        + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                        + -2. * param.s12 * param.s2 * param.s23 * param.s4
                        + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                        + -2. * param.s1 * param.s2 * param.s3 * param.s4
                        + param.m2_2.powi(2) * param.s4.powi(2)
                        + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                        + param.s2.powi(2) * param.s4.powi(2)
                        + param.m1_2.powi(2)
                            * (param.s12.powi(2)
                                + (param.s3 - param.s4).powi(2)
                                + -2. * param.s12 * (param.s3 + param.s4))
                        + 2. * param.m3_2
                            * (-2. * param.s1 * param.s12 * param.s2
                                + param.s1 * param.s12 * param.s23
                                + param.s12 * param.s2 * param.s23
                                + param.s1 * param.s12 * param.s3
                                + param.s1 * param.s2 * param.s3
                                + param.m0_2
                                    * (param.s12 * (param.s2 + param.s23 - param.s3)
                                        + param.s1 * (param.s2 + param.s3 - param.s23)
                                        + param.s2
                                            * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                                + param.s1 * param.s2 * param.s4
                                + param.s12 * param.s2 * param.s4
                                + param.m2_2
                                    * ((param.s12 - param.s2) * (param.s23 - param.s4)
                                        + param.s1
                                            * (param.s12
                                                + param.s2
                                                + param.s23
                                                + -2. * param.s3
                                                + param.s4)
                                        - param.s1.powi(2))
                                - param.s2.powi(2) * param.s4
                                - param.s1.powi(2) * param.s3
                                - param.s12.powi(2) * param.s23)
                        + -2.
                            * param.m1_2
                            * (param.m0_2 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s23
                                + 2. * param.m0_2 * param.s1 * param.s3
                                + param.m0_2 * param.s3.powi(2)
                                + param.s1 * param.s3.powi(2)
                                + param.m0_2 * param.s23 * param.s4
                                + 2. * param.s12 * param.s3 * param.s4
                                + param.s2 * param.s4.powi(2)
                                + param.m3_2
                                    * (param.s12.powi(2)
                                        - param.s12
                                            * (param.s1
                                                + param.s2
                                                + -2. * param.s23
                                                + param.s3
                                                + param.s4)
                                        - (param.s1 - param.s2) * (param.s3 - param.s4))
                                - param.m2_2
                                    * (-2. * param.s2 * param.s4
                                        + param.s23 * param.s4
                                        + param.s3 * param.s4
                                        + param.s12 * (param.s23 + param.s4)
                                        + param.s1 * (param.s3 + param.s4 - param.s12)
                                        - param.s4.powi(2)
                                        - param.s23 * param.s3)
                                - param.s2 * param.s3 * param.s4
                                - param.s1 * param.s3 * param.s4
                                - param.m0_2 * param.s3 * param.s4
                                - param.s12 * param.s23 * param.s4
                                - param.s12 * param.s2 * param.s4
                                - param.m0_2 * param.s2 * param.s4
                                - param.s12 * param.s23 * param.s3
                                - param.m0_2 * param.s23 * param.s3
                                - param.m0_2 * param.s2 * param.s3
                                - param.s1 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s23))
                        .sqrt()
                        * param.lambda_m02_sqrt,
                )
                + param.lambda_s12_sqrt
                    * log_diff(
                        param.m2_2 * (param.s1 + param.s12 - param.s2)
                            + param.s12 * (-2. * param.m1_2 + param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    )
                + (param.s1 - param.s2)
                    * (param.s3 - param.s4)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s12 + param.s3 - param.s4)
                            + param.m2_2 * (param.s12 + param.s4 - param.s3)
                            + param.s12 * (-2. * param.m3_2 + param.s3 + param.s4 - param.s12),
                        param.lambda_m02_sqrt * param.lambda_s34_sqrt,
                    )
                + param.s12
                    * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s12 + param.s3 - param.s4)
                            + param.m2_2 * (param.s12 + param.s4 - param.s3)
                            + param.s12 * (-2. * param.m3_2 + param.s3 + param.s4 - param.s12),
                        param.lambda_m02_sqrt * param.lambda_s34_sqrt,
                    )
                - param.s12.powi(2)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        param.m0_2 * (param.s12 + param.s3 - param.s4)
                            + param.m2_2 * (param.s12 + param.s4 - param.s3)
                            + param.s12 * (-2. * param.m3_2 + param.s3 + param.s4 - param.s12),
                        param.lambda_m02_sqrt * param.lambda_s34_sqrt,
                    ))
            / (param.s12.powi(2) * param.s23
                + param.s1.powi(2) * param.s3
                + param.s2
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s4 * (param.s2 + param.s4 - param.s3))
                - param.s12
                    * (param.s1 * (param.s23 + param.s3 - param.s2)
                        + (param.s2 - param.s3) * param.s4
                        + param.s23 * (param.s2 + param.s3 + param.s4)
                        - param.s23.powi(2))
                - param.s1
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s3 * (param.s4 - param.s3)
                        + param.s2 * (param.s3 + param.s4)))
    } else {
        0.0
    }) + (if param.s4 > (param.m0 + param.m3).powi(2) {
        0.5 * std::f64::consts::PI
            * ((param.m2_2 * param.s1 * param.s12
                + param.m0_2 * param.s1 * param.s2
                + param.m2_2 * param.s1 * param.s2
                + param.m0_2 * param.s12 * param.s2
                + -2. * param.s1 * param.s12 * param.s2
                + param.m3_2
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + -2. * param.s12 * (param.s1 + param.s2))
                + param.m2_2 * param.s1 * param.s23
                + param.m0_2 * param.s12 * param.s23
                + param.m2_2 * param.s12 * param.s23
                + param.s1 * param.s12 * param.s23
                + param.m0_2 * param.s2 * param.s23
                + param.s12 * param.s2 * param.s23
                + param.m0_2 * param.s1 * param.s3
                + -2. * param.m2_2 * param.s1 * param.s3
                + param.s1 * param.s12 * param.s3
                + param.m0_2 * param.s2 * param.s3
                + param.s1 * param.s2 * param.s3
                + param.m2_2 * param.s1 * param.s4
                + -2. * param.m0_2 * param.s2 * param.s4
                + param.m2_2 * param.s2 * param.s4
                + param.s1 * param.s2 * param.s4
                + param.s12 * param.s2 * param.s4
                + param.m1_2
                    * ((param.s1 - param.s2) * (param.s3 - param.s4)
                        + param.s12
                            * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                        - param.s12.powi(2))
                - param.s2.powi(2) * param.s4
                - param.m2_2 * param.s12 * param.s4
                - param.m0_2 * param.s12 * param.s3
                - param.s1.powi(2) * param.s3
                - param.m2_2 * param.s2 * param.s23
                - param.s12.powi(2) * param.s23
                - param.m0_2 * param.s1 * param.s23
                - param.m0_2 * param.s2.powi(2)
                - param.m2_2 * param.s1.powi(2))
                * (param.m2_2.powi(2) * param.s1.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                    + param.m0_2.powi(2) * param.s2.powi(2)
                    + param.m3_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2))
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                    + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                    + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                    + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                    + param.m0_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                    + param.m2_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                    + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                    + param.s12.powi(2) * param.s23.powi(2)
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                    + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                    + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                    + -2. * param.s1 * param.s12 * param.s23 * param.s3
                    + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                    + param.m0_2.powi(2) * param.s3.powi(2)
                    + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                    + param.s1.powi(2) * param.s3.powi(2)
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                    + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                    + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                    + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                    + -2. * param.s12 * param.s2 * param.s23 * param.s4
                    + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                    + -2. * param.s1 * param.s2 * param.s3 * param.s4
                    + param.m2_2.powi(2) * param.s4.powi(2)
                    + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                    + param.s2.powi(2) * param.s4.powi(2)
                    + param.m1_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s3 - param.s4).powi(2)
                            + -2. * param.s12 * (param.s3 + param.s4))
                    + 2. * param.m3_2
                        * (-2. * param.s1 * param.s12 * param.s2
                            + param.s1 * param.s12 * param.s23
                            + param.s12 * param.s2 * param.s23
                            + param.s1 * param.s12 * param.s3
                            + param.s1 * param.s2 * param.s3
                            + param.m0_2
                                * (param.s12 * (param.s2 + param.s23 - param.s3)
                                    + param.s1 * (param.s2 + param.s3 - param.s23)
                                    + param.s2
                                        * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                            + param.s1 * param.s2 * param.s4
                            + param.s12 * param.s2 * param.s4
                            + param.m2_2
                                * (param.s12 * (param.s1 + param.s23 - param.s4)
                                    + param.s2 * (param.s4 - param.s23)
                                    + param.s1
                                        * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                                    - param.s1.powi(2))
                            - param.s2.powi(2) * param.s4
                            - param.s1.powi(2) * param.s3
                            - param.s12.powi(2) * param.s23)
                    + -2.
                        * param.m1_2
                        * (param.m0_2 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s23
                            + 2. * param.m0_2 * param.s1 * param.s3
                            + param.m0_2 * param.s3.powi(2)
                            + param.s1 * param.s3.powi(2)
                            + param.m0_2 * param.s23 * param.s4
                            + 2. * param.s12 * param.s3 * param.s4
                            + param.s2 * param.s4.powi(2)
                            + param.m3_2
                                * (param.s12.powi(2)
                                    - param.s12
                                        * (param.s1
                                            + param.s2
                                            + -2. * param.s23
                                            + param.s3
                                            + param.s4)
                                    - (param.s1 - param.s2) * (param.s3 - param.s4))
                            - param.m2_2
                                * (-2. * param.s2 * param.s4
                                    + param.s23 * param.s4
                                    + param.s3 * param.s4
                                    + param.s12 * (param.s23 + param.s4 - param.s1)
                                    + param.s1 * (param.s3 + param.s4)
                                    - param.s4.powi(2)
                                    - param.s23 * param.s3)
                            - param.s2 * param.s3 * param.s4
                            - param.s1 * param.s3 * param.s4
                            - param.m0_2 * param.s3 * param.s4
                            - param.s12 * param.s23 * param.s4
                            - param.s12 * param.s2 * param.s4
                            - param.m0_2 * param.s2 * param.s4
                            - param.s12 * param.s23 * param.s3
                            - param.m0_2 * param.s23 * param.s3
                            - param.m0_2 * param.s2 * param.s3
                            - param.s1 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s23))
                    .sqrt()
                    .powi(-1)
                * log_diff(
                    param.m1_2 * param.m3_2 * param.s12
                        + -2. * param.m3_2 * param.s1 * param.s12
                        + param.m3_2.powi(2) * param.s2
                        + param.m3_2 * param.s12 * param.s23
                        + param.m0_2.powi(2) * (param.s2 - param.s3 - param.s23)
                        + param.m3_2 * param.s1 * param.s3
                        + param.m1_2 * param.m3_2 * param.s4
                        + param.m3_2 * param.s1 * param.s4
                        + param.m1_2 * param.s12 * param.s4
                        + param.m3_2 * param.s12 * param.s4
                        + -2. * param.m3_2 * param.s2 * param.s4
                        + param.m1_2 * param.s3 * param.s4
                        + param.s2 * param.s4.powi(2)
                        + param.m2_2
                            * ((-2. * param.m1_2 + param.s1 + param.s23 - param.s4) * param.s4
                                + param.m3_2 * (param.s1 + param.s4 - param.s23))
                        + param.m0_2
                            * (param.s12 * param.s23
                                + param.m1_2 * param.s3
                                + param.s1 * param.s3
                                + -2. * param.s23 * param.s3
                                + param.m3_2
                                    * (param.s1
                                        + param.s12
                                        + -2. * param.s2
                                        + param.s23
                                        + param.s3
                                        + -2. * param.s4)
                                + param.m1_2 * param.s4
                                + -2. * param.s2 * param.s4
                                + param.s23 * param.s4
                                + param.s3 * param.s4
                                + param.m2_2 * (param.s23 + param.s4 - param.s1)
                                - param.m1_2 * param.s12)
                        - param.m1_2 * param.s4.powi(2)
                        - param.s1 * param.s3 * param.s4
                        - param.s12 * param.s23 * param.s4
                        - param.m1_2 * param.m3_2 * param.s3
                        - param.m3_2.powi(2) * param.s12
                        - param.m3_2.powi(2) * param.s1,
                    -1. * (param.m2_2.powi(2) * param.s1.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                        + param.m0_2.powi(2) * param.s2.powi(2)
                        + param.m3_2.powi(2)
                            * (param.s1.powi(2)
                                + (param.s12 - param.s2).powi(2)
                                + -2. * param.s1 * (param.s12 + param.s2))
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                        + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                        + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                        + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                        + param.m0_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                        + param.m2_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                        + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                        + param.s12.powi(2) * param.s23.powi(2)
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                        + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                        + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                        + -2. * param.s1 * param.s12 * param.s23 * param.s3
                        + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                        + param.m0_2.powi(2) * param.s3.powi(2)
                        + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                        + param.s1.powi(2) * param.s3.powi(2)
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                        + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                        + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                        + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                        + -2. * param.s12 * param.s2 * param.s23 * param.s4
                        + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                        + -2. * param.s1 * param.s2 * param.s3 * param.s4
                        + param.m2_2.powi(2) * param.s4.powi(2)
                        + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                        + param.s2.powi(2) * param.s4.powi(2)
                        + param.m1_2.powi(2)
                            * (param.s12.powi(2)
                                + (param.s3 - param.s4).powi(2)
                                + -2. * param.s12 * (param.s3 + param.s4))
                        + 2. * param.m3_2
                            * (-2. * param.s1 * param.s12 * param.s2
                                + param.s1 * param.s12 * param.s23
                                + param.s12 * param.s2 * param.s23
                                + param.s1 * param.s12 * param.s3
                                + param.s1 * param.s2 * param.s3
                                + param.m0_2
                                    * (param.s12 * (param.s2 + param.s23 - param.s3)
                                        + param.s1 * (param.s2 + param.s3 - param.s23)
                                        + param.s2
                                            * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                                + param.s1 * param.s2 * param.s4
                                + param.s12 * param.s2 * param.s4
                                + param.m2_2
                                    * ((param.s12 - param.s2) * (param.s23 - param.s4)
                                        + param.s1
                                            * (param.s12
                                                + param.s2
                                                + param.s23
                                                + -2. * param.s3
                                                + param.s4)
                                        - param.s1.powi(2))
                                - param.s2.powi(2) * param.s4
                                - param.s1.powi(2) * param.s3
                                - param.s12.powi(2) * param.s23)
                        + -2.
                            * param.m1_2
                            * (param.m0_2 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s23
                                + 2. * param.m0_2 * param.s1 * param.s3
                                + param.m0_2 * param.s3.powi(2)
                                + param.s1 * param.s3.powi(2)
                                + param.m0_2 * param.s23 * param.s4
                                + 2. * param.s12 * param.s3 * param.s4
                                + param.s2 * param.s4.powi(2)
                                + param.m3_2
                                    * (param.s12.powi(2)
                                        - param.s12
                                            * (param.s1
                                                + param.s2
                                                + -2. * param.s23
                                                + param.s3
                                                + param.s4)
                                        - (param.s1 - param.s2) * (param.s3 - param.s4))
                                - param.m2_2
                                    * (-2. * param.s2 * param.s4
                                        + param.s23 * param.s4
                                        + param.s3 * param.s4
                                        + param.s12 * (param.s23 + param.s4)
                                        + param.s1 * (param.s3 + param.s4 - param.s12)
                                        - param.s4.powi(2)
                                        - param.s23 * param.s3)
                                - param.s2 * param.s3 * param.s4
                                - param.s1 * param.s3 * param.s4
                                - param.m0_2 * param.s3 * param.s4
                                - param.s12 * param.s23 * param.s4
                                - param.s12 * param.s2 * param.s4
                                - param.m0_2 * param.s2 * param.s4
                                - param.s12 * param.s23 * param.s3
                                - param.m0_2 * param.s23 * param.s3
                                - param.m0_2 * param.s2 * param.s3
                                - param.s1 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s23))
                        .sqrt()
                        * param.lambda_m03_sqrt,
                )
                + (param.s12 * (param.s1 + param.s23 - param.s4)
                    + param.s2 * (param.s4 - param.s23)
                    + param.s1 * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                    - param.s1.powi(2))
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s23 - param.s4) * param.s4
                            + param.m3_2 * (param.s1 + param.s4 - param.s23)
                            + param.m0_2 * (param.s23 + param.s4 - param.s1),
                        param.lambda_m03_sqrt * param.lambda_s14_sqrt,
                    )
                + (param.s1 - param.s2)
                    * (param.s3 - param.s4)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        (-2. * param.m2_2 + param.s12 + param.s3 - param.s4) * param.s4
                            + param.m3_2 * (param.s12 + param.s4 - param.s3)
                            + param.m0_2 * (param.s3 + param.s4 - param.s12),
                        param.lambda_m03_sqrt * param.lambda_s34_sqrt,
                    )
                + param.s12
                    * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        (-2. * param.m2_2 + param.s12 + param.s3 - param.s4) * param.s4
                            + param.m3_2 * (param.s12 + param.s4 - param.s3)
                            + param.m0_2 * (param.s3 + param.s4 - param.s12),
                        param.lambda_m03_sqrt * param.lambda_s34_sqrt,
                    )
                - param.s12.powi(2)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        (-2. * param.m2_2 + param.s12 + param.s3 - param.s4) * param.s4
                            + param.m3_2 * (param.s12 + param.s4 - param.s3)
                            + param.m0_2 * (param.s3 + param.s4 - param.s12),
                        param.lambda_m03_sqrt * param.lambda_s34_sqrt,
                    ))
            / (param.s12.powi(2) * param.s23
                + param.s1.powi(2) * param.s3
                + param.s2
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s4 * (param.s2 + param.s4 - param.s3))
                - param.s12
                    * (param.s1 * (param.s23 + param.s3 - param.s2)
                        + (param.s2 - param.s3) * param.s4
                        + param.s23 * (param.s2 + param.s3 + param.s4)
                        - param.s23.powi(2))
                - param.s1
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s3 * (param.s4 - param.s3)
                        + param.s2 * (param.s3 + param.s4)))
    } else {
        0.0
    }) + (if param.s2 > (param.m1 + param.m2).powi(2) {
        0.5 * std::f64::consts::PI
            * ((param.m2_2 * param.s1 * param.s12
                + param.m0_2 * param.s1 * param.s2
                + param.m2_2 * param.s1 * param.s2
                + param.m0_2 * param.s12 * param.s2
                + -2. * param.s1 * param.s12 * param.s2
                + param.m3_2
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + -2. * param.s12 * (param.s1 + param.s2))
                + param.m2_2 * param.s1 * param.s23
                + param.m0_2 * param.s12 * param.s23
                + param.m2_2 * param.s12 * param.s23
                + param.s1 * param.s12 * param.s23
                + param.m0_2 * param.s2 * param.s23
                + param.s12 * param.s2 * param.s23
                + param.m0_2 * param.s1 * param.s3
                + -2. * param.m2_2 * param.s1 * param.s3
                + param.s1 * param.s12 * param.s3
                + param.m0_2 * param.s2 * param.s3
                + param.s1 * param.s2 * param.s3
                + param.m2_2 * param.s1 * param.s4
                + -2. * param.m0_2 * param.s2 * param.s4
                + param.m2_2 * param.s2 * param.s4
                + param.s1 * param.s2 * param.s4
                + param.s12 * param.s2 * param.s4
                + param.m1_2
                    * ((param.s1 - param.s2) * (param.s3 - param.s4)
                        + param.s12
                            * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                        - param.s12.powi(2))
                - param.s2.powi(2) * param.s4
                - param.m2_2 * param.s12 * param.s4
                - param.m0_2 * param.s12 * param.s3
                - param.s1.powi(2) * param.s3
                - param.m2_2 * param.s2 * param.s23
                - param.s12.powi(2) * param.s23
                - param.m0_2 * param.s1 * param.s23
                - param.m0_2 * param.s2.powi(2)
                - param.m2_2 * param.s1.powi(2))
                * (param.m2_2.powi(2) * param.s1.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                    + param.m0_2.powi(2) * param.s2.powi(2)
                    + param.m3_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2))
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                    + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                    + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                    + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                    + param.m0_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                    + param.m2_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                    + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                    + param.s12.powi(2) * param.s23.powi(2)
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                    + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                    + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                    + -2. * param.s1 * param.s12 * param.s23 * param.s3
                    + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                    + param.m0_2.powi(2) * param.s3.powi(2)
                    + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                    + param.s1.powi(2) * param.s3.powi(2)
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                    + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                    + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                    + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                    + -2. * param.s12 * param.s2 * param.s23 * param.s4
                    + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                    + -2. * param.s1 * param.s2 * param.s3 * param.s4
                    + param.m2_2.powi(2) * param.s4.powi(2)
                    + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                    + param.s2.powi(2) * param.s4.powi(2)
                    + param.m1_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s3 - param.s4).powi(2)
                            + -2. * param.s12 * (param.s3 + param.s4))
                    + 2. * param.m3_2
                        * (-2. * param.s1 * param.s12 * param.s2
                            + param.s1 * param.s12 * param.s23
                            + param.s12 * param.s2 * param.s23
                            + param.s1 * param.s12 * param.s3
                            + param.s1 * param.s2 * param.s3
                            + param.m0_2
                                * (param.s12 * (param.s2 + param.s23 - param.s3)
                                    + param.s1 * (param.s2 + param.s3 - param.s23)
                                    + param.s2
                                        * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                            + param.s1 * param.s2 * param.s4
                            + param.s12 * param.s2 * param.s4
                            + param.m2_2
                                * (param.s12 * (param.s1 + param.s23 - param.s4)
                                    + param.s2 * (param.s4 - param.s23)
                                    + param.s1
                                        * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                                    - param.s1.powi(2))
                            - param.s2.powi(2) * param.s4
                            - param.s1.powi(2) * param.s3
                            - param.s12.powi(2) * param.s23)
                    + -2.
                        * param.m1_2
                        * (param.m0_2 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s23
                            + 2. * param.m0_2 * param.s1 * param.s3
                            + param.m0_2 * param.s3.powi(2)
                            + param.s1 * param.s3.powi(2)
                            + param.m0_2 * param.s23 * param.s4
                            + 2. * param.s12 * param.s3 * param.s4
                            + param.s2 * param.s4.powi(2)
                            + param.m3_2
                                * (param.s12.powi(2)
                                    - param.s12
                                        * (param.s1
                                            + param.s2
                                            + -2. * param.s23
                                            + param.s3
                                            + param.s4)
                                    - (param.s1 - param.s2) * (param.s3 - param.s4))
                            - param.m2_2
                                * (-2. * param.s2 * param.s4
                                    + param.s23 * param.s4
                                    + param.s3 * param.s4
                                    + param.s12 * (param.s23 + param.s4 - param.s1)
                                    + param.s1 * (param.s3 + param.s4)
                                    - param.s4.powi(2)
                                    - param.s23 * param.s3)
                            - param.s2 * param.s3 * param.s4
                            - param.s1 * param.s3 * param.s4
                            - param.m0_2 * param.s3 * param.s4
                            - param.s12 * param.s23 * param.s4
                            - param.s12 * param.s2 * param.s4
                            - param.m0_2 * param.s2 * param.s4
                            - param.s12 * param.s23 * param.s3
                            - param.m0_2 * param.s23 * param.s3
                            - param.m0_2 * param.s2 * param.s3
                            - param.s1 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s23))
                    .sqrt()
                    .powi(-1)
                * log_diff(
                    param.m2_2
                        * (param.s1 * param.s2
                            + param.m3_2 * (param.s1 + param.s2 - param.s12)
                            + -2. * param.s1 * param.s23
                            + param.s12 * param.s23
                            + param.s2 * param.s23
                            + param.m0_2 * (param.s2 + param.s23 - param.s3)
                            + param.s1 * param.s3
                            + -2. * param.s2 * param.s4)
                        + param.m1_2
                            * (param.m0_2 * param.s2
                                + param.s12 * param.s2
                                + param.m3_2 * (param.s12 + param.s2 - param.s1)
                                + param.s12 * param.s23
                                + param.m0_2 * param.s3
                                + param.s1 * param.s3
                                + -2. * param.s12 * param.s3
                                + param.s2 * param.s3
                                + param.m2_2
                                    * (param.s1
                                        + param.s12
                                        + -2. * param.s2
                                        + param.s23
                                        + param.s3
                                        + -2. * param.s4)
                                + -2. * param.s2 * param.s4
                                - param.m0_2 * param.s23)
                        + param.s2
                            * (param.m3_2 * (param.s1 + param.s12 - param.s2)
                                + param.m0_2
                                    * (-2. * param.m3_2 + param.s23 + param.s3 - param.s2)
                                + param.s2 * param.s4
                                - param.s1 * param.s3
                                - param.s12 * param.s23)
                        - param.m1_2.powi(2) * (param.s12 + param.s3 - param.s4)
                        - param.m2_2.powi(2) * (param.s1 + param.s23 - param.s4),
                    -1. * (param.m2_2.powi(2) * param.s1.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                        + param.m0_2.powi(2) * param.s2.powi(2)
                        + param.m3_2.powi(2)
                            * (param.s1.powi(2)
                                + (param.s12 - param.s2).powi(2)
                                + -2. * param.s1 * (param.s12 + param.s2))
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                        + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                        + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                        + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                        + param.m0_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                        + param.m2_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                        + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                        + param.s12.powi(2) * param.s23.powi(2)
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                        + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                        + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                        + -2. * param.s1 * param.s12 * param.s23 * param.s3
                        + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                        + param.m0_2.powi(2) * param.s3.powi(2)
                        + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                        + param.s1.powi(2) * param.s3.powi(2)
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                        + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                        + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                        + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                        + -2. * param.s12 * param.s2 * param.s23 * param.s4
                        + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                        + -2. * param.s1 * param.s2 * param.s3 * param.s4
                        + param.m2_2.powi(2) * param.s4.powi(2)
                        + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                        + param.s2.powi(2) * param.s4.powi(2)
                        + param.m1_2.powi(2)
                            * (param.s12.powi(2)
                                + (param.s3 - param.s4).powi(2)
                                + -2. * param.s12 * (param.s3 + param.s4))
                        + 2. * param.m3_2
                            * (-2. * param.s1 * param.s12 * param.s2
                                + param.s1 * param.s12 * param.s23
                                + param.s12 * param.s2 * param.s23
                                + param.s1 * param.s12 * param.s3
                                + param.s1 * param.s2 * param.s3
                                + param.m0_2
                                    * (param.s12 * (param.s2 + param.s23 - param.s3)
                                        + param.s1 * (param.s2 + param.s3 - param.s23)
                                        + param.s2
                                            * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                                + param.s1 * param.s2 * param.s4
                                + param.s12 * param.s2 * param.s4
                                + param.m2_2
                                    * ((param.s12 - param.s2) * (param.s23 - param.s4)
                                        + param.s1
                                            * (param.s12
                                                + param.s2
                                                + param.s23
                                                + -2. * param.s3
                                                + param.s4)
                                        - param.s1.powi(2))
                                - param.s2.powi(2) * param.s4
                                - param.s1.powi(2) * param.s3
                                - param.s12.powi(2) * param.s23)
                        + -2.
                            * param.m1_2
                            * (param.m0_2 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s23
                                + 2. * param.m0_2 * param.s1 * param.s3
                                + param.m0_2 * param.s3.powi(2)
                                + param.s1 * param.s3.powi(2)
                                + param.m0_2 * param.s23 * param.s4
                                + 2. * param.s12 * param.s3 * param.s4
                                + param.s2 * param.s4.powi(2)
                                + param.m3_2
                                    * (param.s12.powi(2)
                                        - param.s12
                                            * (param.s1
                                                + param.s2
                                                + -2. * param.s23
                                                + param.s3
                                                + param.s4)
                                        - (param.s1 - param.s2) * (param.s3 - param.s4))
                                - param.m2_2
                                    * (-2. * param.s2 * param.s4
                                        + param.s23 * param.s4
                                        + param.s3 * param.s4
                                        + param.s12 * (param.s23 + param.s4)
                                        + param.s1 * (param.s3 + param.s4 - param.s12)
                                        - param.s4.powi(2)
                                        - param.s23 * param.s3)
                                - param.s2 * param.s3 * param.s4
                                - param.s1 * param.s3 * param.s4
                                - param.m0_2 * param.s3 * param.s4
                                - param.s12 * param.s23 * param.s4
                                - param.s12 * param.s2 * param.s4
                                - param.m0_2 * param.s2 * param.s4
                                - param.s12 * param.s23 * param.s3
                                - param.m0_2 * param.s23 * param.s3
                                - param.m0_2 * param.s2 * param.s3
                                - param.s1 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s23))
                        .sqrt()
                        * param.lambda_m12_sqrt,
                )
                + param.lambda_s12_sqrt
                    * log_diff(
                        (-2. * param.m0_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m1_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m12_sqrt * param.lambda_s12_sqrt,
                    )
                + param.s12
                    * (param.s2 + param.s23 - param.s3)
                    * param.lambda_s23_sqrt.powi(-1)
                    * log_diff(
                        param.m2_2 * (param.s2 + param.s23 - param.s3)
                            + param.m1_2 * (param.s2 + param.s3 - param.s23)
                            + param.s2 * (-2. * param.m3_2 + param.s23 + param.s3 - param.s2),
                        param.lambda_m12_sqrt * param.lambda_s23_sqrt,
                    )
                + param.s1
                    * (param.s2 + param.s3 - param.s23)
                    * param.lambda_s23_sqrt.powi(-1)
                    * log_diff(
                        param.m2_2 * (param.s2 + param.s23 - param.s3)
                            + param.m1_2 * (param.s2 + param.s3 - param.s23)
                            + param.s2 * (-2. * param.m3_2 + param.s23 + param.s3 - param.s2),
                        param.lambda_m12_sqrt * param.lambda_s23_sqrt,
                    )
                + param.s2
                    * (param.s23 + param.s3 + -2. * param.s4 - param.s2)
                    * param.lambda_s23_sqrt.powi(-1)
                    * log_diff(
                        param.m2_2 * (param.s2 + param.s23 - param.s3)
                            + param.m1_2 * (param.s2 + param.s3 - param.s23)
                            + param.s2 * (-2. * param.m3_2 + param.s23 + param.s3 - param.s2),
                        param.lambda_m12_sqrt * param.lambda_s23_sqrt,
                    ))
            / (param.s12.powi(2) * param.s23
                + param.s1.powi(2) * param.s3
                + param.s2
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s4 * (param.s2 + param.s4 - param.s3))
                - param.s12
                    * (param.s1 * (param.s23 + param.s3 - param.s2)
                        + (param.s2 - param.s3) * param.s4
                        + param.s23 * (param.s2 + param.s3 + param.s4)
                        - param.s23.powi(2))
                - param.s1
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s3 * (param.s4 - param.s3)
                        + param.s2 * (param.s3 + param.s4)))
    } else {
        0.0
    }) + (if param.s23 > (param.m1 + param.m3).powi(2) {
        0.5 * std::f64::consts::PI
            * ((param.m2_2 * param.s1 * param.s12
                + param.m0_2 * param.s1 * param.s2
                + param.m2_2 * param.s1 * param.s2
                + param.m0_2 * param.s12 * param.s2
                + -2. * param.s1 * param.s12 * param.s2
                + param.m3_2
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + -2. * param.s12 * (param.s1 + param.s2))
                + param.m2_2 * param.s1 * param.s23
                + param.m0_2 * param.s12 * param.s23
                + param.m2_2 * param.s12 * param.s23
                + param.s1 * param.s12 * param.s23
                + param.m0_2 * param.s2 * param.s23
                + param.s12 * param.s2 * param.s23
                + param.m0_2 * param.s1 * param.s3
                + -2. * param.m2_2 * param.s1 * param.s3
                + param.s1 * param.s12 * param.s3
                + param.m0_2 * param.s2 * param.s3
                + param.s1 * param.s2 * param.s3
                + param.m2_2 * param.s1 * param.s4
                + -2. * param.m0_2 * param.s2 * param.s4
                + param.m2_2 * param.s2 * param.s4
                + param.s1 * param.s2 * param.s4
                + param.s12 * param.s2 * param.s4
                + param.m1_2
                    * ((param.s1 - param.s2) * (param.s3 - param.s4)
                        + param.s12
                            * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                        - param.s12.powi(2))
                - param.s2.powi(2) * param.s4
                - param.m2_2 * param.s12 * param.s4
                - param.m0_2 * param.s12 * param.s3
                - param.s1.powi(2) * param.s3
                - param.m2_2 * param.s2 * param.s23
                - param.s12.powi(2) * param.s23
                - param.m0_2 * param.s1 * param.s23
                - param.m0_2 * param.s2.powi(2)
                - param.m2_2 * param.s1.powi(2))
                * (param.m2_2.powi(2) * param.s1.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                    + param.m0_2.powi(2) * param.s2.powi(2)
                    + param.m3_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2))
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                    + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                    + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                    + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                    + param.m0_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                    + param.m2_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                    + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                    + param.s12.powi(2) * param.s23.powi(2)
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                    + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                    + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                    + -2. * param.s1 * param.s12 * param.s23 * param.s3
                    + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                    + param.m0_2.powi(2) * param.s3.powi(2)
                    + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                    + param.s1.powi(2) * param.s3.powi(2)
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                    + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                    + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                    + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                    + -2. * param.s12 * param.s2 * param.s23 * param.s4
                    + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                    + -2. * param.s1 * param.s2 * param.s3 * param.s4
                    + param.m2_2.powi(2) * param.s4.powi(2)
                    + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                    + param.s2.powi(2) * param.s4.powi(2)
                    + param.m1_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s3 - param.s4).powi(2)
                            + -2. * param.s12 * (param.s3 + param.s4))
                    + 2. * param.m3_2
                        * (-2. * param.s1 * param.s12 * param.s2
                            + param.s1 * param.s12 * param.s23
                            + param.s12 * param.s2 * param.s23
                            + param.s1 * param.s12 * param.s3
                            + param.s1 * param.s2 * param.s3
                            + param.m0_2
                                * (param.s12 * (param.s2 + param.s23 - param.s3)
                                    + param.s1 * (param.s2 + param.s3 - param.s23)
                                    + param.s2
                                        * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                            + param.s1 * param.s2 * param.s4
                            + param.s12 * param.s2 * param.s4
                            + param.m2_2
                                * (param.s12 * (param.s1 + param.s23 - param.s4)
                                    + param.s2 * (param.s4 - param.s23)
                                    + param.s1
                                        * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                                    - param.s1.powi(2))
                            - param.s2.powi(2) * param.s4
                            - param.s1.powi(2) * param.s3
                            - param.s12.powi(2) * param.s23)
                    + -2.
                        * param.m1_2
                        * (param.m0_2 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s23
                            + 2. * param.m0_2 * param.s1 * param.s3
                            + param.m0_2 * param.s3.powi(2)
                            + param.s1 * param.s3.powi(2)
                            + param.m0_2 * param.s23 * param.s4
                            + 2. * param.s12 * param.s3 * param.s4
                            + param.s2 * param.s4.powi(2)
                            + param.m3_2
                                * (param.s12.powi(2)
                                    - param.s12
                                        * (param.s1
                                            + param.s2
                                            + -2. * param.s23
                                            + param.s3
                                            + param.s4)
                                    - (param.s1 - param.s2) * (param.s3 - param.s4))
                            - param.m2_2
                                * (-2. * param.s2 * param.s4
                                    + param.s23 * param.s4
                                    + param.s3 * param.s4
                                    + param.s12 * (param.s23 + param.s4 - param.s1)
                                    + param.s1 * (param.s3 + param.s4)
                                    - param.s4.powi(2)
                                    - param.s23 * param.s3)
                            - param.s2 * param.s3 * param.s4
                            - param.s1 * param.s3 * param.s4
                            - param.m0_2 * param.s3 * param.s4
                            - param.s12 * param.s23 * param.s4
                            - param.s12 * param.s2 * param.s4
                            - param.m0_2 * param.s2 * param.s4
                            - param.s12 * param.s23 * param.s3
                            - param.m0_2 * param.s23 * param.s3
                            - param.m0_2 * param.s2 * param.s3
                            - param.s1 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s23))
                    .sqrt()
                    .powi(-1)
                * log_diff(
                    param.m3_2.powi(2) * param.s12
                        + param.m0_2 * param.m3_2 * param.s2
                        + -2. * param.m3_2 * param.s1 * param.s2
                        + param.m0_2 * param.m3_2 * param.s23
                        + param.m3_2 * param.s1 * param.s23
                        + -2. * param.m3_2 * param.s12 * param.s23
                        + param.m0_2 * param.s2 * param.s23
                        + param.m3_2 * param.s2 * param.s23
                        + param.s12 * param.s23.powi(2)
                        + param.m3_2 * param.s1 * param.s3
                        + param.m0_2 * param.s23 * param.s3
                        + param.m1_2.powi(2) * (param.s12 - param.s4 - param.s3)
                        + param.m3_2 * param.s2 * param.s4
                        + param.m2_2
                            * (param.m3_2 * (param.s1 + param.s23 - param.s4)
                                + param.s23 * (-2. * param.m0_2 + param.s1 + param.s4 - param.s23))
                        + param.m1_2
                            * (param.m0_2 * param.s23
                                + -2. * param.s12 * param.s23
                                + param.m0_2 * param.s3
                                + param.s1 * param.s3
                                + param.s23 * param.s3
                                + param.s2 * param.s4
                                + param.s23 * param.s4
                                + -2. * param.s3 * param.s4
                                + param.m2_2 * (param.s23 + param.s4 - param.s1)
                                + param.m3_2
                                    * (param.s1
                                        + -2. * param.s12
                                        + param.s2
                                        + -2. * param.s23
                                        + param.s3
                                        + param.s4)
                                - param.m0_2 * param.s2)
                        - param.s2 * param.s23 * param.s4
                        - param.s1 * param.s23 * param.s3
                        - param.m0_2 * param.m3_2 * param.s3
                        - param.m0_2 * param.s23.powi(2)
                        - param.m3_2.powi(2) * param.s2
                        - param.m3_2.powi(2) * param.s1,
                    -1. * (param.m2_2.powi(2) * param.s1.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                        + param.m0_2.powi(2) * param.s2.powi(2)
                        + param.m3_2.powi(2)
                            * (param.s1.powi(2)
                                + (param.s12 - param.s2).powi(2)
                                + -2. * param.s1 * (param.s12 + param.s2))
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                        + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                        + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                        + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                        + param.m0_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                        + param.m2_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                        + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                        + param.s12.powi(2) * param.s23.powi(2)
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                        + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                        + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                        + -2. * param.s1 * param.s12 * param.s23 * param.s3
                        + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                        + param.m0_2.powi(2) * param.s3.powi(2)
                        + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                        + param.s1.powi(2) * param.s3.powi(2)
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                        + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                        + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                        + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                        + -2. * param.s12 * param.s2 * param.s23 * param.s4
                        + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                        + -2. * param.s1 * param.s2 * param.s3 * param.s4
                        + param.m2_2.powi(2) * param.s4.powi(2)
                        + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                        + param.s2.powi(2) * param.s4.powi(2)
                        + param.m1_2.powi(2)
                            * (param.s12.powi(2)
                                + (param.s3 - param.s4).powi(2)
                                + -2. * param.s12 * (param.s3 + param.s4))
                        + 2. * param.m3_2
                            * (-2. * param.s1 * param.s12 * param.s2
                                + param.s1 * param.s12 * param.s23
                                + param.s12 * param.s2 * param.s23
                                + param.s1 * param.s12 * param.s3
                                + param.s1 * param.s2 * param.s3
                                + param.m0_2
                                    * (param.s12 * (param.s2 + param.s23 - param.s3)
                                        + param.s1 * (param.s2 + param.s3 - param.s23)
                                        + param.s2
                                            * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                                + param.s1 * param.s2 * param.s4
                                + param.s12 * param.s2 * param.s4
                                + param.m2_2
                                    * ((param.s12 - param.s2) * (param.s23 - param.s4)
                                        + param.s1
                                            * (param.s12
                                                + param.s2
                                                + param.s23
                                                + -2. * param.s3
                                                + param.s4)
                                        - param.s1.powi(2))
                                - param.s2.powi(2) * param.s4
                                - param.s1.powi(2) * param.s3
                                - param.s12.powi(2) * param.s23)
                        + -2.
                            * param.m1_2
                            * (param.m0_2 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s23
                                + 2. * param.m0_2 * param.s1 * param.s3
                                + param.m0_2 * param.s3.powi(2)
                                + param.s1 * param.s3.powi(2)
                                + param.m0_2 * param.s23 * param.s4
                                + 2. * param.s12 * param.s3 * param.s4
                                + param.s2 * param.s4.powi(2)
                                + param.m3_2
                                    * (param.s12.powi(2)
                                        - param.s12
                                            * (param.s1
                                                + param.s2
                                                + -2. * param.s23
                                                + param.s3
                                                + param.s4)
                                        - (param.s1 - param.s2) * (param.s3 - param.s4))
                                - param.m2_2
                                    * (-2. * param.s2 * param.s4
                                        + param.s23 * param.s4
                                        + param.s3 * param.s4
                                        + param.s12 * (param.s23 + param.s4)
                                        + param.s1 * (param.s3 + param.s4 - param.s12)
                                        - param.s4.powi(2)
                                        - param.s23 * param.s3)
                                - param.s2 * param.s3 * param.s4
                                - param.s1 * param.s3 * param.s4
                                - param.m0_2 * param.s3 * param.s4
                                - param.s12 * param.s23 * param.s4
                                - param.s12 * param.s2 * param.s4
                                - param.m0_2 * param.s2 * param.s4
                                - param.s12 * param.s23 * param.s3
                                - param.m0_2 * param.s23 * param.s3
                                - param.m0_2 * param.s2 * param.s3
                                - param.s1 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s23))
                        .sqrt()
                        * param.lambda_m13_sqrt,
                )
                + param.s12
                    * (param.s1 + param.s23 - param.s4)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s1 + param.s23 - param.s4)
                            + param.s23 * (-2. * param.m0_2 + param.s1 + param.s4 - param.s23)
                            + param.m1_2 * (param.s23 + param.s4 - param.s1),
                        param.lambda_m13_sqrt * param.lambda_s14_sqrt,
                    )
                + param.s2
                    * (param.s4 - param.s23)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s1 + param.s23 - param.s4)
                            + param.s23 * (-2. * param.m0_2 + param.s1 + param.s4 - param.s23)
                            + param.m1_2 * (param.s23 + param.s4 - param.s1),
                        param.lambda_m13_sqrt * param.lambda_s14_sqrt,
                    )
                + param.s1
                    * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s1 + param.s23 - param.s4)
                            + param.s23 * (-2. * param.m0_2 + param.s1 + param.s4 - param.s23)
                            + param.m1_2 * (param.s23 + param.s4 - param.s1),
                        param.lambda_m13_sqrt * param.lambda_s14_sqrt,
                    )
                + (param.s12 * (param.s2 + param.s23 - param.s3)
                    + param.s1 * (param.s2 + param.s3 - param.s23)
                    + param.s2 * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                    * param.lambda_s23_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s2 + param.s23 - param.s3)
                            + param.s23 * (-2. * param.m2_2 + param.s2 + param.s3 - param.s23)
                            + param.m1_2 * (param.s23 + param.s3 - param.s2),
                        param.lambda_m13_sqrt * param.lambda_s23_sqrt,
                    )
                - param.s1.powi(2)
                    * param.lambda_s14_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s1 + param.s23 - param.s4)
                            + param.s23 * (-2. * param.m0_2 + param.s1 + param.s4 - param.s23)
                            + param.m1_2 * (param.s23 + param.s4 - param.s1),
                        param.lambda_m13_sqrt * param.lambda_s14_sqrt,
                    ))
            / (param.s12.powi(2) * param.s23
                + param.s1.powi(2) * param.s3
                + param.s2
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s4 * (param.s2 + param.s4 - param.s3))
                - param.s12
                    * (param.s1 * (param.s23 + param.s3 - param.s2)
                        + (param.s2 - param.s3) * param.s4
                        + param.s23 * (param.s2 + param.s3 + param.s4)
                        - param.s23.powi(2))
                - param.s1
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s3 * (param.s4 - param.s3)
                        + param.s2 * (param.s3 + param.s4)))
    } else {
        0.0
    }) + (if param.s3 > (param.m2 + param.m3).powi(2) {
        0.5 * std::f64::consts::PI
            * ((param.m2_2 * param.s1 * param.s12
                + param.m0_2 * param.s1 * param.s2
                + param.m2_2 * param.s1 * param.s2
                + param.m0_2 * param.s12 * param.s2
                + -2. * param.s1 * param.s12 * param.s2
                + param.m3_2
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + -2. * param.s12 * (param.s1 + param.s2))
                + param.m2_2 * param.s1 * param.s23
                + param.m0_2 * param.s12 * param.s23
                + param.m2_2 * param.s12 * param.s23
                + param.s1 * param.s12 * param.s23
                + param.m0_2 * param.s2 * param.s23
                + param.s12 * param.s2 * param.s23
                + param.m0_2 * param.s1 * param.s3
                + -2. * param.m2_2 * param.s1 * param.s3
                + param.s1 * param.s12 * param.s3
                + param.m0_2 * param.s2 * param.s3
                + param.s1 * param.s2 * param.s3
                + param.m2_2 * param.s1 * param.s4
                + -2. * param.m0_2 * param.s2 * param.s4
                + param.m2_2 * param.s2 * param.s4
                + param.s1 * param.s2 * param.s4
                + param.s12 * param.s2 * param.s4
                + param.m1_2
                    * ((param.s1 - param.s2) * (param.s3 - param.s4)
                        + param.s12
                            * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                        - param.s12.powi(2))
                - param.s2.powi(2) * param.s4
                - param.m2_2 * param.s12 * param.s4
                - param.m0_2 * param.s12 * param.s3
                - param.s1.powi(2) * param.s3
                - param.m2_2 * param.s2 * param.s23
                - param.s12.powi(2) * param.s23
                - param.m0_2 * param.s1 * param.s23
                - param.m0_2 * param.s2.powi(2)
                - param.m2_2 * param.s1.powi(2))
                * (param.m2_2.powi(2) * param.s1.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                    + param.m0_2.powi(2) * param.s2.powi(2)
                    + param.m3_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2))
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                    + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                    + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                    + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                    + param.m0_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                    + param.m2_2.powi(2) * param.s23.powi(2)
                    + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                    + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                    + param.s12.powi(2) * param.s23.powi(2)
                    + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                    + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                    + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                    + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                    + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                    + -2. * param.s1 * param.s12 * param.s23 * param.s3
                    + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                    + param.m0_2.powi(2) * param.s3.powi(2)
                    + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                    + param.s1.powi(2) * param.s3.powi(2)
                    + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                    + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                    + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                    + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                    + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                    + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                    + -2. * param.s12 * param.s2 * param.s23 * param.s4
                    + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                    + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                    + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                    + -2. * param.s1 * param.s2 * param.s3 * param.s4
                    + param.m2_2.powi(2) * param.s4.powi(2)
                    + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                    + param.s2.powi(2) * param.s4.powi(2)
                    + param.m1_2.powi(2)
                        * (param.s12.powi(2)
                            + (param.s3 - param.s4).powi(2)
                            + -2. * param.s12 * (param.s3 + param.s4))
                    + 2. * param.m3_2
                        * (-2. * param.s1 * param.s12 * param.s2
                            + param.s1 * param.s12 * param.s23
                            + param.s12 * param.s2 * param.s23
                            + param.s1 * param.s12 * param.s3
                            + param.s1 * param.s2 * param.s3
                            + param.m0_2
                                * (param.s12 * (param.s2 + param.s23 - param.s3)
                                    + param.s1 * (param.s2 + param.s3 - param.s23)
                                    + param.s2
                                        * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                            + param.s1 * param.s2 * param.s4
                            + param.s12 * param.s2 * param.s4
                            + param.m2_2
                                * (param.s12 * (param.s1 + param.s23 - param.s4)
                                    + param.s2 * (param.s4 - param.s23)
                                    + param.s1
                                        * (param.s2 + param.s23 + -2. * param.s3 + param.s4)
                                    - param.s1.powi(2))
                            - param.s2.powi(2) * param.s4
                            - param.s1.powi(2) * param.s3
                            - param.s12.powi(2) * param.s23)
                    + -2.
                        * param.m1_2
                        * (param.m0_2 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s23
                            + 2. * param.m0_2 * param.s1 * param.s3
                            + param.m0_2 * param.s3.powi(2)
                            + param.s1 * param.s3.powi(2)
                            + param.m0_2 * param.s23 * param.s4
                            + 2. * param.s12 * param.s3 * param.s4
                            + param.s2 * param.s4.powi(2)
                            + param.m3_2
                                * (param.s12.powi(2)
                                    - param.s12
                                        * (param.s1
                                            + param.s2
                                            + -2. * param.s23
                                            + param.s3
                                            + param.s4)
                                    - (param.s1 - param.s2) * (param.s3 - param.s4))
                            - param.m2_2
                                * (-2. * param.s2 * param.s4
                                    + param.s23 * param.s4
                                    + param.s3 * param.s4
                                    + param.s12 * (param.s23 + param.s4 - param.s1)
                                    + param.s1 * (param.s3 + param.s4)
                                    - param.s4.powi(2)
                                    - param.s23 * param.s3)
                            - param.s2 * param.s3 * param.s4
                            - param.s1 * param.s3 * param.s4
                            - param.m0_2 * param.s3 * param.s4
                            - param.s12 * param.s23 * param.s4
                            - param.s12 * param.s2 * param.s4
                            - param.m0_2 * param.s2 * param.s4
                            - param.s12 * param.s23 * param.s3
                            - param.m0_2 * param.s23 * param.s3
                            - param.m0_2 * param.s2 * param.s3
                            - param.s1 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s3
                            - param.m0_2 * param.s12 * param.s23))
                    .sqrt()
                    .powi(-1)
                * log_diff(
                    param.m3_2.powi(2) * (param.s1 - param.s2 - param.s12)
                        + param.m2_2.powi(2) * (param.s1 - param.s4 - param.s23)
                        + param.m3_2
                            * (-2. * param.s12 * param.s2
                                + param.s12 * param.s23
                                + -2. * param.s1 * param.s3
                                + param.s12 * param.s3
                                + param.s2 * param.s3
                                + param.m0_2 * (param.s2 + param.s3 - param.s23)
                                + param.m1_2 * (param.s12 + param.s3 - param.s4)
                                + param.s2 * param.s4)
                        + param.s3
                            * (param.m0_2 * (-2. * param.m1_2 + param.s2 + param.s23 - param.s3)
                                + param.s1 * param.s3
                                + param.m1_2 * (param.s12 + param.s4 - param.s3)
                                - param.s2 * param.s4
                                - param.s12 * param.s23)
                        + param.m2_2
                            * (param.m0_2 * param.s23
                                + param.s12 * param.s23
                                + param.m0_2 * param.s3
                                + -2. * param.s1 * param.s3
                                + param.s23 * param.s3
                                + param.s2 * param.s4
                                + -2. * param.s23 * param.s4
                                + param.s3 * param.s4
                                + param.m3_2
                                    * (-2. * param.s1
                                        + param.s12
                                        + param.s2
                                        + param.s23
                                        + -2. * param.s3
                                        + param.s4)
                                + param.m1_2 * (param.s3 + param.s4 - param.s12)
                                - param.m0_2 * param.s2),
                    -1. * (param.m2_2.powi(2) * param.s1.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s1 * param.s2
                        + param.m0_2.powi(2) * param.s2.powi(2)
                        + param.m3_2.powi(2)
                            * (param.s1.powi(2)
                                + (param.s12 - param.s2).powi(2)
                                + -2. * param.s1 * (param.s12 + param.s2))
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s23
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s23
                        + -4. * param.m0_2 * param.m2_2 * param.s12 * param.s23
                        + 2. * param.m2_2 * param.s1 * param.s12 * param.s23
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s23
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s23
                        + 2. * param.m0_2 * param.s12 * param.s2 * param.s23
                        + param.m0_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.m2_2 * param.s23.powi(2)
                        + param.m2_2.powi(2) * param.s23.powi(2)
                        + -2. * param.m0_2 * param.s12 * param.s23.powi(2)
                        + -2. * param.m2_2 * param.s12 * param.s23.powi(2)
                        + param.s12.powi(2) * param.s23.powi(2)
                        + 2. * param.m0_2 * param.m2_2 * param.s1 * param.s3
                        + -2. * param.m2_2 * param.s1.powi(2) * param.s3
                        + -2. * param.m0_2.powi(2) * param.s2 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s2 * param.s3
                        + -2. * param.m0_2.powi(2) * param.s23 * param.s3
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m2_2 * param.s1 * param.s23 * param.s3
                        + 2. * param.m0_2 * param.s12 * param.s23 * param.s3
                        + -2. * param.s1 * param.s12 * param.s23 * param.s3
                        + -4. * param.m0_2 * param.s2 * param.s23 * param.s3
                        + param.m0_2.powi(2) * param.s3.powi(2)
                        + -2. * param.m0_2 * param.s1 * param.s3.powi(2)
                        + param.s1.powi(2) * param.s3.powi(2)
                        + -2. * param.m2_2.powi(2) * param.s1 * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s2 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s2 * param.s4
                        + -2. * param.m0_2 * param.s2.powi(2) * param.s4
                        + 2. * param.m0_2 * param.m2_2 * param.s23 * param.s4
                        + -2. * param.m2_2.powi(2) * param.s23 * param.s4
                        + -4. * param.m2_2 * param.s1 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s12 * param.s23 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s23 * param.s4
                        + 2. * param.m2_2 * param.s2 * param.s23 * param.s4
                        + -2. * param.s12 * param.s2 * param.s23 * param.s4
                        + -2. * param.m0_2 * param.m2_2 * param.s3 * param.s4
                        + 2. * param.m2_2 * param.s1 * param.s3 * param.s4
                        + 2. * param.m0_2 * param.s2 * param.s3 * param.s4
                        + -2. * param.s1 * param.s2 * param.s3 * param.s4
                        + param.m2_2.powi(2) * param.s4.powi(2)
                        + -2. * param.m2_2 * param.s2 * param.s4.powi(2)
                        + param.s2.powi(2) * param.s4.powi(2)
                        + param.m1_2.powi(2)
                            * (param.s12.powi(2)
                                + (param.s3 - param.s4).powi(2)
                                + -2. * param.s12 * (param.s3 + param.s4))
                        + 2. * param.m3_2
                            * (-2. * param.s1 * param.s12 * param.s2
                                + param.s1 * param.s12 * param.s23
                                + param.s12 * param.s2 * param.s23
                                + param.s1 * param.s12 * param.s3
                                + param.s1 * param.s2 * param.s3
                                + param.m0_2
                                    * (param.s12 * (param.s2 + param.s23 - param.s3)
                                        + param.s1 * (param.s2 + param.s3 - param.s23)
                                        + param.s2
                                            * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                                + param.s1 * param.s2 * param.s4
                                + param.s12 * param.s2 * param.s4
                                + param.m2_2
                                    * ((param.s12 - param.s2) * (param.s23 - param.s4)
                                        + param.s1
                                            * (param.s12
                                                + param.s2
                                                + param.s23
                                                + -2. * param.s3
                                                + param.s4)
                                        - param.s1.powi(2))
                                - param.s2.powi(2) * param.s4
                                - param.s1.powi(2) * param.s3
                                - param.s12.powi(2) * param.s23)
                        + -2.
                            * param.m1_2
                            * (param.m0_2 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s23
                                + 2. * param.m0_2 * param.s1 * param.s3
                                + param.m0_2 * param.s3.powi(2)
                                + param.s1 * param.s3.powi(2)
                                + param.m0_2 * param.s23 * param.s4
                                + 2. * param.s12 * param.s3 * param.s4
                                + param.s2 * param.s4.powi(2)
                                + param.m3_2
                                    * (param.s12.powi(2)
                                        - param.s12
                                            * (param.s1
                                                + param.s2
                                                + -2. * param.s23
                                                + param.s3
                                                + param.s4)
                                        - (param.s1 - param.s2) * (param.s3 - param.s4))
                                - param.m2_2
                                    * (-2. * param.s2 * param.s4
                                        + param.s23 * param.s4
                                        + param.s3 * param.s4
                                        + param.s12 * (param.s23 + param.s4)
                                        + param.s1 * (param.s3 + param.s4 - param.s12)
                                        - param.s4.powi(2)
                                        - param.s23 * param.s3)
                                - param.s2 * param.s3 * param.s4
                                - param.s1 * param.s3 * param.s4
                                - param.m0_2 * param.s3 * param.s4
                                - param.s12 * param.s23 * param.s4
                                - param.s12 * param.s2 * param.s4
                                - param.m0_2 * param.s2 * param.s4
                                - param.s12 * param.s23 * param.s3
                                - param.m0_2 * param.s23 * param.s3
                                - param.m0_2 * param.s2 * param.s3
                                - param.s1 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s3
                                - param.m0_2 * param.s12 * param.s23))
                        .sqrt()
                        * param.lambda_m23_sqrt,
                )
                + (param.s1 - param.s2)
                    * (param.s3 - param.s4)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s12 + param.s3 - param.s4)
                            + param.s3 * (-2. * param.m0_2 + param.s12 + param.s4 - param.s3)
                            + param.m2_2 * (param.s3 + param.s4 - param.s12),
                        param.lambda_m23_sqrt * param.lambda_s34_sqrt,
                    )
                + param.s12
                    * (param.s1 + param.s2 + -2. * param.s23 + param.s3 + param.s4)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s12 + param.s3 - param.s4)
                            + param.s3 * (-2. * param.m0_2 + param.s12 + param.s4 - param.s3)
                            + param.m2_2 * (param.s3 + param.s4 - param.s12),
                        param.lambda_m23_sqrt * param.lambda_s34_sqrt,
                    )
                + (param.s12 * (param.s2 + param.s23 - param.s3)
                    + param.s1 * (param.s2 + param.s3 - param.s23)
                    + param.s2 * (param.s23 + param.s3 + -2. * param.s4 - param.s2))
                    * param.lambda_s23_sqrt.powi(-1)
                    * log_diff(
                        (-2. * param.m1_2 + param.s2 + param.s23 - param.s3) * param.s3
                            + param.m3_2 * (param.s2 + param.s3 - param.s23)
                            + param.m2_2 * (param.s23 + param.s3 - param.s2),
                        param.lambda_m23_sqrt * param.lambda_s23_sqrt,
                    )
                - param.s12.powi(2)
                    * param.lambda_s34_sqrt.powi(-1)
                    * log_diff(
                        param.m3_2 * (param.s12 + param.s3 - param.s4)
                            + param.s3 * (-2. * param.m0_2 + param.s12 + param.s4 - param.s3)
                            + param.m2_2 * (param.s3 + param.s4 - param.s12),
                        param.lambda_m23_sqrt * param.lambda_s34_sqrt,
                    ))
            / (param.s12.powi(2) * param.s23
                + param.s1.powi(2) * param.s3
                + param.s2
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s4 * (param.s2 + param.s4 - param.s3))
                - param.s12
                    * (param.s1 * (param.s23 + param.s3 - param.s2)
                        + (param.s2 - param.s3) * param.s4
                        + param.s23 * (param.s2 + param.s3 + param.s4)
                        - param.s23.powi(2))
                - param.s1
                    * (param.s23 * (param.s3 - param.s4)
                        + param.s3 * (param.s4 - param.s3)
                        + param.s2 * (param.s3 + param.s4)))
    } else {
        0.0
    })
}
