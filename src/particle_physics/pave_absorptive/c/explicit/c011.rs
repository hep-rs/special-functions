use super::{log_diff, Parameters};

pub(crate) fn c011(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.5 * std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-5)
            * (2.
                * param.s1
                * (3. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1) * param.s2
                    + param.m0_2.powi(2)
                        * (2. * param.s12.powi(2)
                            - param.s12 * (param.s1 + param.s2)
                            - (param.s1 - param.s2).powi(2))
                    + 2. * param.m1_2
                        * (param.m2_2
                            * (param.s1.powi(2)
                                + param.s12.powi(2)
                                + 4. * param.s1 * param.s2
                                + param.s2.powi(2)
                                + -2. * param.s12 * (param.s1 + param.s2))
                            - param.s2
                                * (-2. * param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + param.s12 * (param.s1 + -2. * param.s2)
                                    + param.s1 * param.s2
                                    + param.s2.powi(2)))
                    - param.s1
                        * (3. * param.m2_2.powi(2) * (param.s1 + param.s2 - param.s12)
                            + 2. * param.m2_2
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -2. * param.s2.powi(2)
                                    + param.s12 * (-2. * param.s1 + param.s2))
                            + param.s2
                                * (-2. * param.s12.powi(2)
                                    + (param.s1 - param.s2).powi(2)
                                    + param.s12 * (param.s1 + param.s2)))
                    - param.m0_2
                        * (param.s1.powi(2) * param.s12
                            + param.s1 * param.s12.powi(2)
                            + param.s1.powi(2) * param.s2
                            + -6. * param.s1 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s2
                            + param.s1 * param.s2.powi(2)
                            + param.s12 * param.s2.powi(2)
                            + 2. * param.m2_2
                                * (-2. * param.s1.powi(2)
                                    + param.s1 * param.s12
                                    + param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -2. * param.s12 * param.s2
                                    + param.s2.powi(2))
                            + 2. * param.m1_2
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -2. * param.s2.powi(2)
                                    + param.s12 * (-2. * param.s1 + param.s2))
                            - param.s2.powi(3)
                            - param.s12.powi(3)
                            - param.s1.powi(3)))
                * log_diff(
                    param.m0_2 * (param.s1 + param.s12 - param.s2)
                        + param.m1_2 * (param.s1 + param.s2 - param.s12)
                        + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                    param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m0_2
                    * (-5. * param.s1.powi(2)
                        + 4. * param.s1 * param.s12
                        + param.s12.powi(2)
                        + 4. * param.s1 * param.s2
                        + -2. * param.s12 * param.s2
                        + param.s2.powi(2))
                    + param.s1
                        * (param.s1.powi(2)
                            + -2. * param.s1 * param.s12
                            + param.s12.powi(2)
                            + 4. * param.s1 * param.s2
                            + 4. * param.s12 * param.s2
                            + -5. * param.s2.powi(2)
                            + 6. * param.m2_2 * (param.s1 + param.s2 - param.s12))
                    - param.m1_2
                        * (param.s1.powi(2)
                            + param.s12.powi(2)
                            + 10. * param.s1 * param.s2
                            + param.s2.powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2)))
                    * param.lambda_m01_sqrt
                    * param.lambda_s12_sqrt)
            / param.s1
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.5 * std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-5)
            * (2.
                * param.s2
                * (3. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1) * param.s2
                    + param.m0_2.powi(2)
                        * (2. * param.s12.powi(2)
                            - param.s12 * (param.s1 + param.s2)
                            - (param.s1 - param.s2).powi(2))
                    + 2. * param.m1_2
                        * (param.m2_2
                            * (param.s1.powi(2)
                                + param.s12.powi(2)
                                + 4. * param.s1 * param.s2
                                + param.s2.powi(2)
                                + -2. * param.s12 * (param.s1 + param.s2))
                            - param.s2
                                * (-2. * param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + param.s12 * (param.s1 + -2. * param.s2)
                                    + param.s1 * param.s2
                                    + param.s2.powi(2)))
                    - param.s1
                        * (3. * param.m2_2.powi(2) * (param.s1 + param.s2 - param.s12)
                            + 2. * param.m2_2
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -2. * param.s2.powi(2)
                                    + param.s12 * (-2. * param.s1 + param.s2))
                            + param.s2
                                * (-2. * param.s12.powi(2)
                                    + (param.s1 - param.s2).powi(2)
                                    + param.s12 * (param.s1 + param.s2)))
                    - param.m0_2
                        * (param.s1.powi(2) * param.s12
                            + param.s1 * param.s12.powi(2)
                            + param.s1.powi(2) * param.s2
                            + -6. * param.s1 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s2
                            + param.s1 * param.s2.powi(2)
                            + param.s12 * param.s2.powi(2)
                            + 2. * param.m2_2
                                * (-2. * param.s1.powi(2)
                                    + param.s1 * param.s12
                                    + param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -2. * param.s12 * param.s2
                                    + param.s2.powi(2))
                            + 2. * param.m1_2
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -2. * param.s2.powi(2)
                                    + param.s12 * (-2. * param.s1 + param.s2))
                            - param.s2.powi(3)
                            - param.s12.powi(3)
                            - param.s1.powi(3)))
                * log_diff(
                    (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1),
                    param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m0_2
                    * (param.s1.powi(2)
                        + -2. * param.s1 * param.s12
                        + param.s12.powi(2)
                        + 4. * param.s1 * param.s2
                        + 4. * param.s12 * param.s2
                        + -5. * param.s2.powi(2))
                    + param.s2
                        * (-5. * param.s1.powi(2)
                            + 4. * param.s1 * param.s12
                            + param.s12.powi(2)
                            + 4. * param.s1 * param.s2
                            + -2. * param.s12 * param.s2
                            + param.s2.powi(2)
                            + 6. * param.m1_2 * (param.s1 + param.s2 - param.s12))
                    - param.m2_2
                        * (param.s1.powi(2)
                            + param.s12.powi(2)
                            + 10. * param.s1 * param.s2
                            + param.s2.powi(2)
                            + -2. * param.s12 * (param.s1 + param.s2)))
                    * param.lambda_m02_sqrt
                    * param.lambda_s12_sqrt)
            / param.s2
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.5 * std::f64::consts::PI
            * param.s12.powi(-2)
            * param.lambda_s12_sqrt.powi(-5)
            * ((6. * param.m2_2 * param.s1.powi(2) * param.s12
                + param.s1.powi(3) * param.s12
                + -3. * param.m2_2 * param.s1 * param.s12.powi(2)
                + -2. * param.s1.powi(2) * param.s12.powi(2)
                + -2. * param.m2_2 * param.s12.powi(3)
                + param.s1 * param.s12.powi(3)
                + 3. * param.m2_2 * param.s1.powi(2) * param.s2
                + -2. * param.m2_2 * param.s1 * param.s12 * param.s2
                + 5. * param.m2_2 * param.s12.powi(2) * param.s2
                + 8. * param.s1 * param.s12.powi(2) * param.s2
                + param.s12.powi(3) * param.s2
                + -3. * param.m2_2 * param.s1 * param.s2.powi(2)
                + -4. * param.m2_2 * param.s12 * param.s2.powi(2)
                + -2. * param.s12.powi(2) * param.s2.powi(2)
                + param.m2_2 * param.s2.powi(3)
                + param.s12 * param.s2.powi(3)
                + 2. * param.m0_2
                    * param.s12
                    * (2. * param.s12.powi(2)
                        - param.s12 * (param.s1 + param.s2)
                        - (param.s1 - param.s2).powi(2))
                - param.m1_2
                    * (2. * param.s12.powi(3)
                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                        + 2. * param.s12
                            * (2. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + -3. * param.s2.powi(2))
                        - (param.s1 - param.s2).powi(3))
                - param.s1 * param.s12 * param.s2.powi(2)
                - param.s1.powi(2) * param.s12 * param.s2
                - param.m2_2 * param.s1.powi(3))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 2. * param.s12.powi(2)
                    * (3. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1) * param.s2
                        + param.m0_2.powi(2)
                            * (2. * param.s12.powi(2)
                                - param.s12 * (param.s1 + param.s2)
                                - (param.s1 - param.s2).powi(2))
                        + 2. * param.m1_2
                            * (param.m2_2
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 4. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                - param.s2
                                    * (-2. * param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + param.s12 * (param.s1 + -2. * param.s2)
                                        + param.s1 * param.s2
                                        + param.s2.powi(2)))
                        - param.s1
                            * (3. * param.m2_2.powi(2) * (param.s1 + param.s2 - param.s12)
                                + 2. * param.m2_2
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -2. * param.s2.powi(2)
                                        + param.s12 * (-2. * param.s1 + param.s2))
                                + param.s2
                                    * (-2. * param.s12.powi(2)
                                        + (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2)))
                        - param.m0_2
                            * (param.s1.powi(2) * param.s12
                                + param.s1 * param.s12.powi(2)
                                + param.s1.powi(2) * param.s2
                                + -6. * param.s1 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s2
                                + param.s1 * param.s2.powi(2)
                                + param.s12 * param.s2.powi(2)
                                + 2. * param.m2_2
                                    * (-2. * param.s1.powi(2)
                                        + param.s1 * param.s12
                                        + param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -2. * param.s12 * param.s2
                                        + param.s2.powi(2))
                                + 2. * param.m1_2
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -2. * param.s2.powi(2)
                                        + param.s12 * (-2. * param.s1 + param.s2))
                                - param.s2.powi(3)
                                - param.s12.powi(3)
                                - param.s1.powi(3)))
                    * log_diff(
                        param.m2_2 * (param.s1 + param.s12 - param.s2)
                            + param.s12 * (-2. * param.m0_2 + param.s1 + param.s2 - param.s12)
                            + param.m1_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m12_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    })
}
