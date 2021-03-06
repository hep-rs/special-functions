use super::{log_diff, Parameters};

pub(crate) fn c021(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.1666666666666667
            * std::f64::consts::PI
            * param.s1.powi(-2)
            * param.lambda_s12_sqrt.powi(-7)
            * ((param.m0_2.powi(2)
                * (param.s12.powi(4)
                    + 6. * param.s12.powi(2) * param.s2 * (2. * param.s1 + param.s2)
                    + -2. * param.s12.powi(3) * (5. * param.s1 + 2. * param.s2)
                    + param.s12
                        * (26. * param.s1.powi(3)
                            + -60. * param.s1.powi(2) * param.s2
                            + 6. * param.s1 * param.s2.powi(2)
                            + -4. * param.s2.powi(3))
                    - (param.s1 - param.s2).powi(2)
                        * (17. * param.s1.powi(2) + 6. * param.s1 * param.s2 - param.s2.powi(2)))
                + param.m1_2.powi(2)
                    * (param.s1.powi(4)
                        + param.s12.powi(4)
                        + -14. * param.s1.powi(3) * param.s2
                        + -94. * param.s1.powi(2) * param.s2.powi(2)
                        + -14. * param.s1 * param.s2.powi(3)
                        + param.s2.powi(4)
                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                        + 6. * param.s12.powi(2)
                            * (param.s1.powi(2) + param.s2.powi(2) - param.s1 * param.s2)
                        + -4.
                            * param.s12
                            * (param.s1.powi(3)
                                + -6. * param.s1.powi(2) * param.s2
                                + -6. * param.s1 * param.s2.powi(2)
                                + param.s2.powi(3)))
                + param.s1.powi(2)
                    * (param.s12.powi(4)
                        + 6. * param.s1 * param.s12.powi(2) * (param.s1 + 2. * param.s2)
                        + -2. * param.s12.powi(3) * (2. * param.s1 + 5. * param.s2)
                        + (param.s1 - param.s2).powi(2)
                            * (param.s1.powi(2)
                                + -6. * param.s1 * param.s2
                                + -17. * param.s2.powi(2))
                        + param.s12
                            * (-4. * param.s1.powi(3)
                                + 6. * param.s1.powi(2) * param.s2
                                + -60. * param.s1 * param.s2.powi(2)
                                + 26. * param.s2.powi(3))
                        + -24.
                            * param.m2_2.powi(2)
                            * (param.s1.powi(2)
                                + param.s12.powi(2)
                                + 3. * param.s1 * param.s2
                                + param.s2.powi(2)
                                + -2. * param.s12 * (param.s1 + param.s2))
                        + 6. * param.m2_2
                            * (param.s12.powi(3)
                                + -11. * param.s1.powi(2) * param.s2
                                + 5. * param.s1 * param.s2.powi(2)
                                + 7. * param.s2.powi(3)
                                + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 6. * param.s1 * param.s2
                                        + -13. * param.s2.powi(2))
                                - param.s1.powi(3)))
                + -2.
                    * param.m1_2
                    * param.s1
                    * (param.s1.powi(4)
                        + param.s12.powi(4)
                        + -11. * param.s1.powi(3) * param.s2
                        + -37. * param.s1.powi(2) * param.s2.powi(2)
                        + 43. * param.s1 * param.s2.powi(3)
                        + 4. * param.s2.powi(4)
                        + 3. * param.s12.powi(2)
                            * (2. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + 5. * param.s2.powi(2))
                        + 3. * param.m2_2
                            * (param.s12.powi(3)
                                + -19. * param.s1.powi(2) * param.s2
                                + -19. * param.s1 * param.s2.powi(2)
                                + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 22. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                - param.s2.powi(3)
                                - param.s1.powi(3))
                        - param.s12
                            * (4. * param.s1.powi(3)
                                + -15. * param.s1.powi(2) * param.s2
                                + 42. * param.s1 * param.s2.powi(2)
                                + 13. * param.s2.powi(3))
                        - param.s12.powi(3) * (4. * param.s1 + 7. * param.s2))
                + -2.
                    * param.m0_2
                    * (param.m1_2
                        * (4. * param.s1.powi(4)
                            + param.s12.powi(4)
                            + 43. * param.s1.powi(3) * param.s2
                            + -37. * param.s1.powi(2) * param.s2.powi(2)
                            + -11. * param.s1 * param.s2.powi(3)
                            + param.s2.powi(4)
                            + 3. * param.s12.powi(2)
                                * (5. * param.s1.powi(2)
                                    + param.s1 * param.s2
                                    + 2. * param.s2.powi(2))
                            - param.s12
                                * (13. * param.s1.powi(3)
                                    + 42. * param.s1.powi(2) * param.s2
                                    + -15. * param.s1 * param.s2.powi(2)
                                    + 4. * param.s2.powi(3))
                            - param.s12.powi(3) * (7. * param.s1 + 4. * param.s2))
                        - param.s1
                            * (param.s12.powi(3) * (param.s1 + param.s2)
                                + 3. * param.s12.powi(2)
                                    * (param.s1.powi(2)
                                        + -12. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + 2. * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + 9. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12
                                    * (-5. * param.s1.powi(3)
                                        + 21. * param.s1.powi(2) * param.s2
                                        + 21. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3))
                                + 3. * param.m2_2
                                    * (7. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                        + 5. * param.s1.powi(2) * param.s2
                                        + -11. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-13. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                - param.s12.powi(4))))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 6. * param.s1.powi(2)
                    * (-10.
                        * param.m1_2.powi(3)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(3)
                            * (3. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2)))
                        + 6. * param.m1_2.powi(2)
                            * param.s2
                            * (param.s2
                                * (-3. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -4. * param.s2)
                                    + param.s1 * param.s2
                                    + 2. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2)
                                        + -2. * param.s12 * (param.s1 + param.s2)))
                        + param.m0_2.powi(2)
                            * (param.s1.powi(3) * param.s12
                                + 3. * param.s1.powi(2) * param.s12.powi(2)
                                + -5. * param.s1 * param.s12.powi(3)
                                + 2. * param.s12.powi(4)
                                + param.s1.powi(3) * param.s2
                                + -16. * param.s1.powi(2) * param.s12 * param.s2
                                + 11. * param.s1 * param.s12.powi(2) * param.s2
                                + 4. * param.s12.powi(3) * param.s2
                                + 3. * param.s1.powi(2) * param.s2.powi(2)
                                + 11. * param.s1 * param.s12 * param.s2.powi(2)
                                + -12. * param.s12.powi(2) * param.s2.powi(2)
                                + -5. * param.s1 * param.s2.powi(3)
                                + 4. * param.s12 * param.s2.powi(3)
                                + 2. * param.s2.powi(4)
                                + -3.
                                    * param.m1_2
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2))
                                + -6.
                                    * param.m2_2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.s1.powi(4))
                        + param.s1
                            * (-4.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s1.powi(3))
                                + -6.
                                    * param.m2_2
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                + param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + -3.
                            * param.m1_2
                            * (param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + -9. * param.s1.powi(2) * param.s2
                                    + -9. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3))
                                + -2.
                                    * param.m2_2
                                    * param.s2
                                    * (3. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                - param.s2.powi(2)
                                    * ((param.s1 - param.s2).powi(2)
                                        * (3. * param.s1 + param.s2)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - param.s12.powi(3)))
                        + param.m0_2
                            * (6.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -3. * param.s2.powi(2)
                                    + param.s12 * (-4. * param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (3. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                + 2. * param.m2_2
                                    * (param.s12.powi(3) * (param.s1 + param.s2)
                                        + 2. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -16. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (-5. * param.s1.powi(3)
                                                + 11. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3))
                                        - param.s12.powi(4))
                                + param.s2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (4. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-12. * param.s1.powi(2)
                                                + 11. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (4. * param.s1.powi(3)
                                                + 11. * param.s1.powi(2) * param.s2
                                                + -16. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 6. * param.m1_2
                                    * (param.m2_2
                                        * (param.s12.powi(3)
                                            + -5. * param.s1.powi(2) * param.s2
                                            + 3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3)
                                            + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                            + param.s12
                                                * (3. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            - param.s1.powi(3))
                                        + 2. * param.s2
                                            * (param.s12.powi(2) * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12.powi(3)))))
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.3333333333333333
            * std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-7)
            * ((param.m2_2.powi(2)
                * (param.s1.powi(3)
                    + 29. * param.s1.powi(2) * param.s2
                    + 29. * param.s1 * param.s2.powi(2)
                    + param.s2.powi(3)
                    + 3. * param.s12.powi(2) * (param.s1 + param.s2)
                    - param.s12
                        * (3. * param.s1.powi(2)
                            + 32. * param.s1 * param.s2
                            + 3. * param.s2.powi(2))
                    - param.s12.powi(3))
                + param.s2.powi(2)
                    * (-30. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1)
                        + 3. * param.s12.powi(2) * (-6. * param.s1 + param.s2)
                        + (param.s1 - param.s2).powi(2) * (10. * param.s1 + param.s2)
                        + param.s12
                            * (9. * param.s1.powi(2)
                                + 10. * param.s1 * param.s2
                                + -3. * param.s2.powi(2))
                        + 3. * param.m1_2
                            * (-13. * param.s1.powi(2)
                                + 6. * param.s1 * param.s12
                                + 7. * param.s12.powi(2)
                                + 6. * param.s1 * param.s2
                                + -14. * param.s12 * param.s2
                                + 7. * param.s2.powi(2))
                        - param.s12.powi(3))
                + param.m0_2.powi(2)
                    * (3. * param.s12.powi(2) * (param.s1 + -6. * param.s2)
                        + (param.s1 - param.s2).powi(2) * (param.s1 + 10. * param.s2)
                        + param.s12
                            * (-3. * param.s1.powi(2)
                                + 10. * param.s1 * param.s2
                                + 9. * param.s2.powi(2))
                        - param.s12.powi(3))
                + param.m2_2
                    * param.s2
                    * (19. * param.s1.powi(3)
                        + 2. * param.s12.powi(3)
                        + 3. * param.s12.powi(2) * (5. * param.s1 + -2. * param.s2)
                        + 20. * param.s1.powi(2) * param.s2
                        + -37. * param.s1 * param.s2.powi(2)
                        + -2. * param.s2.powi(3)
                        + param.s12
                            * (-36. * param.s1.powi(2)
                                + 22. * param.s1 * param.s2
                                + 6. * param.s2.powi(2))
                        + -3.
                            * param.m1_2
                            * (7. * param.s1.powi(2)
                                + 7. * param.s12.powi(2)
                                + 26. * param.s1 * param.s2
                                + 7. * param.s2.powi(2)
                                + -14. * param.s12 * (param.s1 + param.s2)))
                + param.m0_2
                    * (param.m2_2
                        * (-2. * param.s1.powi(3)
                            + 2. * param.s12.powi(3)
                            + -37. * param.s1.powi(2) * param.s2
                            + 20. * param.s1 * param.s2.powi(2)
                            + 19. * param.s2.powi(3)
                            + param.s12.powi(2) * (-6. * param.s1 + 15. * param.s2)
                            + param.s12
                                * (6. * param.s1.powi(2)
                                    + 22. * param.s1 * param.s2
                                    + -36. * param.s2.powi(2)))
                        + param.s2
                            * (-10. * param.s12.powi(3)
                                + 9. * param.s12.powi(2) * (param.s1 + param.s2)
                                + -11. * (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                + 3. * param.m1_2
                                    * (7. * param.s1.powi(2)
                                        + -14. * param.s1 * param.s12
                                        + 7. * param.s12.powi(2)
                                        + 6. * param.s1 * param.s2
                                        + 6. * param.s12 * param.s2
                                        + -13. * param.s2.powi(2))
                                + 4. * param.s12
                                    * (3. * param.s1.powi(2)
                                        + -14. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)))))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 3. * param.s2
                    * (-10.
                        * param.m1_2.powi(3)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(3)
                            * (3. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2)))
                        + 6. * param.m1_2.powi(2)
                            * param.s2
                            * (param.s2
                                * (-3. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -4. * param.s2)
                                    + param.s1 * param.s2
                                    + 2. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2)
                                        + -2. * param.s12 * (param.s1 + param.s2)))
                        + param.m0_2.powi(2)
                            * (param.s1.powi(3) * param.s12
                                + 3. * param.s1.powi(2) * param.s12.powi(2)
                                + -5. * param.s1 * param.s12.powi(3)
                                + 2. * param.s12.powi(4)
                                + param.s1.powi(3) * param.s2
                                + -16. * param.s1.powi(2) * param.s12 * param.s2
                                + 11. * param.s1 * param.s12.powi(2) * param.s2
                                + 4. * param.s12.powi(3) * param.s2
                                + 3. * param.s1.powi(2) * param.s2.powi(2)
                                + 11. * param.s1 * param.s12 * param.s2.powi(2)
                                + -12. * param.s12.powi(2) * param.s2.powi(2)
                                + -5. * param.s1 * param.s2.powi(3)
                                + 4. * param.s12 * param.s2.powi(3)
                                + 2. * param.s2.powi(4)
                                + -3.
                                    * param.m1_2
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2))
                                + -6.
                                    * param.m2_2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.s1.powi(4))
                        + param.s1
                            * (-4.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s1.powi(3))
                                + -6.
                                    * param.m2_2
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                + param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + -3.
                            * param.m1_2
                            * (param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + -9. * param.s1.powi(2) * param.s2
                                    + -9. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3))
                                + -2.
                                    * param.m2_2
                                    * param.s2
                                    * (3. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                - param.s2.powi(2)
                                    * ((param.s1 - param.s2).powi(2)
                                        * (3. * param.s1 + param.s2)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - param.s12.powi(3)))
                        + param.m0_2
                            * (6.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -3. * param.s2.powi(2)
                                    + param.s12 * (-4. * param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (3. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                + 2. * param.m2_2
                                    * (param.s12.powi(3) * (param.s1 + param.s2)
                                        + 2. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -16. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (-5. * param.s1.powi(3)
                                                + 11. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3))
                                        - param.s12.powi(4))
                                + param.s2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (4. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-12. * param.s1.powi(2)
                                                + 11. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (4. * param.s1.powi(3)
                                                + 11. * param.s1.powi(2) * param.s2
                                                + -16. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 6. * param.m1_2
                                    * (param.m2_2
                                        * (param.s12.powi(3)
                                            + -5. * param.s1.powi(2) * param.s2
                                            + 3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3)
                                            + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                            + param.s12
                                                * (3. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            - param.s1.powi(3))
                                        + 2. * param.s2
                                            * (param.s12.powi(2) * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12.powi(3)))))
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
            / param.s2
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.1666666666666667
            * std::f64::consts::PI
            * param.s12.powi(-3)
            * param.lambda_s12_sqrt.powi(-7)
            * ((2. * param.m2_2.powi(2) * param.s1.powi(5)
                + -14. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12
                + 48. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(2)
                + 10. * param.m2_2 * param.s1.powi(4) * param.s12.powi(2)
                + -56. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(3)
                + -24. * param.m2_2 * param.s1.powi(3) * param.s12.powi(3)
                + 4. * param.s1.powi(4) * param.s12.powi(3)
                + 14. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4)
                + 22. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4)
                + -6. * param.s1.powi(3) * param.s12.powi(4)
                + 6. * param.m2_2.powi(2) * param.s12.powi(5)
                + -7. * param.m2_2 * param.s1 * param.s12.powi(5)
                + 4. * param.s1.powi(2) * param.s12.powi(5)
                + -10. * param.m2_2.powi(2) * param.s1.powi(4) * param.s2
                + 30. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12 * param.s2
                + 2. * param.m2_2 * param.s1.powi(4) * param.s12 * param.s2
                + 6. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(2) * param.s2
                + 12. * param.m2_2 * param.s1.powi(3) * param.s12.powi(2) * param.s2
                + 8. * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + 22. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2
                + 60. * param.m2_2 * param.s1.powi(2) * param.s12.powi(3) * param.s2
                + -6. * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + -24. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2
                + -68. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2
                + -12. * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + -6. * param.m2_2 * param.s12.powi(5) * param.s2
                + 10. * param.s1 * param.s12.powi(5) * param.s2
                + 20. * param.m2_2.powi(2) * param.s1.powi(3) * param.s2.powi(2)
                + -6. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                + 2. * param.m2_2 * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                + -24. * param.m2_2.powi(2) * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                + -42. * param.m2_2 * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(2)
                + -16. * param.s1.powi(3) * param.s12.powi(2) * param.s2.powi(2)
                + 38. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(2)
                + 60. * param.m2_2 * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                + 42. * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(2)
                + 20. * param.m2_2 * param.s12.powi(4) * param.s2.powi(2)
                + 36. * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + 2. * param.s12.powi(5) * param.s2.powi(2)
                + -20. * param.m2_2.powi(2) * param.s1.powi(2) * param.s2.powi(3)
                + -22. * param.m2_2.powi(2) * param.s1 * param.s12 * param.s2.powi(3)
                + -8. * param.m2_2 * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                + -30. * param.m2_2.powi(2) * param.s12.powi(2) * param.s2.powi(3)
                + 8. * param.m2_2 * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                + 10. * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(3)
                + -24. * param.m2_2 * param.s12.powi(3) * param.s2.powi(3)
                + -46. * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + -6. * param.s12.powi(4) * param.s2.powi(3)
                + 10. * param.m2_2.powi(2) * param.s1 * param.s2.powi(4)
                + 12. * param.m2_2.powi(2) * param.s12 * param.s2.powi(4)
                + 7. * param.m2_2 * param.s1 * param.s12 * param.s2.powi(4)
                + 12. * param.m2_2 * param.s12.powi(2) * param.s2.powi(4)
                + param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + 6. * param.s12.powi(3) * param.s2.powi(4)
                + -2. * param.m2_2.powi(2) * param.s2.powi(5)
                + -2. * param.m2_2 * param.s12 * param.s2.powi(5)
                + -2. * param.s12.powi(2) * param.s2.powi(5)
                + 6. * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (3. * param.s12.powi(3)
                        + (param.s1 - param.s2).powi(3)
                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                        + param.s12
                            * (param.s1.powi(2)
                                + 4. * param.s1 * param.s2
                                + -5. * param.s2.powi(2)))
                + param.m1_2.powi(2)
                    * (-3. * param.s12.powi(5)
                        + 2. * (param.s1 - param.s2).powi(5)
                        + 2. * param.s12.powi(4) * (7. * param.s1 + 15. * param.s2)
                        + param.s12.powi(3)
                            * (-26. * param.s1.powi(2)
                                + -62. * param.s1 * param.s2
                                + 20. * param.s2.powi(2))
                        + 12.
                            * param.s12.powi(2)
                            * (2. * param.s1.powi(3)
                                + 2. * param.s1.powi(2) * param.s2
                                + param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3))
                        - param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (11. * param.s1 + 15. * param.s2))
                + param.m1_2
                    * (param.m2_2
                        * (15. * param.s12.powi(5)
                            + -4. * (param.s1 - param.s2).powi(5)
                            + param.s12.powi(4) * (-58. * param.s1 + 12. * param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (25. * param.s1 + 27. * param.s2)
                            + 8. * param.s12.powi(3)
                                * (11. * param.s1.powi(2)
                                    + 8. * param.s1 * param.s2
                                    + -11. * param.s2.powi(2))
                            + -6.
                                * param.s12.powi(2)
                                * (11. * param.s1.powi(3)
                                    + 8. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + -14. * param.s2.powi(3)))
                        + param.s12
                            * (3. * param.s12.powi(5)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (7. * param.s1.powi(2)
                                        + -22. * param.s1 * param.s2
                                        + -39. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (22. * param.s1.powi(2)
                                        + 28. * param.s1 * param.s2
                                        + 8. * param.s2.powi(2))
                                + -6.
                                    * param.s12.powi(2)
                                    * (3. * param.s1.powi(3)
                                        + -4. * param.s1.powi(2) * param.s2
                                        + 21. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))
                                - param.s12.powi(4) * (13. * param.s1 + 24. * param.s2)
                                - (param.s1 + -4. * param.s2) * (param.s1 - param.s2).powi(4)))
                + 3. * param.m0_2
                    * param.s12
                    * (param.m2_2
                        * (-9. * param.s12.powi(4)
                            + (param.s1 - param.s2).powi(4)
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (5. * param.s1 + 4. * param.s2)
                            + 2. * param.s12.powi(3) * (5. * param.s1 + 6. * param.s2)
                            + 4. * param.s12.powi(2)
                                * (2. * param.s1.powi(2)
                                    + -9. * param.s1 * param.s2
                                    + param.s2.powi(2)))
                        + param.s12
                            * (param.s12.powi(4)
                                + -2. * param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                + 16. * param.s12.powi(2) * (param.s1 - param.s2) * param.s2
                                + 2. * param.s12
                                    * (param.s1.powi(3)
                                        + -12. * param.s1.powi(2) * param.s2
                                        + 9. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3) * (param.s1 + 3. * param.s2))
                        - param.m1_2
                            * (3. * param.s12.powi(4)
                                + (param.s1 - param.s2).powi(4)
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1 + 2. * param.s2)
                                + param.s12.powi(3) * (-10. * param.s1 + 24. * param.s2)
                                + 4. * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + -5. * param.s1 * param.s2
                                        + -4. * param.s2.powi(2))))
                - param.s1 * param.s12.powi(6)
                - param.s1.powi(5) * param.s12.powi(2)
                - param.m2_2 * param.s1.powi(5) * param.s12)
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 6. * param.s12.powi(3)
                    * (-10.
                        * param.m1_2.powi(3)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(3)
                            * (3. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2)))
                        + 6. * param.m1_2.powi(2)
                            * param.s2
                            * (param.s2
                                * (-3. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -4. * param.s2)
                                    + param.s1 * param.s2
                                    + 2. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2)
                                        + -2. * param.s12 * (param.s1 + param.s2)))
                        + param.m0_2.powi(2)
                            * (param.s1.powi(3) * param.s12
                                + 3. * param.s1.powi(2) * param.s12.powi(2)
                                + -5. * param.s1 * param.s12.powi(3)
                                + 2. * param.s12.powi(4)
                                + param.s1.powi(3) * param.s2
                                + -16. * param.s1.powi(2) * param.s12 * param.s2
                                + 11. * param.s1 * param.s12.powi(2) * param.s2
                                + 4. * param.s12.powi(3) * param.s2
                                + 3. * param.s1.powi(2) * param.s2.powi(2)
                                + 11. * param.s1 * param.s12 * param.s2.powi(2)
                                + -12. * param.s12.powi(2) * param.s2.powi(2)
                                + -5. * param.s1 * param.s2.powi(3)
                                + 4. * param.s12 * param.s2.powi(3)
                                + 2. * param.s2.powi(4)
                                + -3.
                                    * param.m1_2
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2))
                                + -6.
                                    * param.m2_2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.s1.powi(4))
                        + param.s1
                            * (-4.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s1.powi(3))
                                + -6.
                                    * param.m2_2
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                + param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + -3.
                            * param.m1_2
                            * (param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + -9. * param.s1.powi(2) * param.s2
                                    + -9. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3))
                                + -2.
                                    * param.m2_2
                                    * param.s2
                                    * (3. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                - param.s2.powi(2)
                                    * ((param.s1 - param.s2).powi(2)
                                        * (3. * param.s1 + param.s2)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - param.s12.powi(3)))
                        + param.m0_2
                            * (6.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -3. * param.s2.powi(2)
                                    + param.s12 * (-4. * param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (3. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3))
                                + 2. * param.m2_2
                                    * (param.s12.powi(3) * (param.s1 + param.s2)
                                        + 2. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -16. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (-5. * param.s1.powi(3)
                                                + 11. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3))
                                        - param.s12.powi(4))
                                + param.s2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (4. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-12. * param.s1.powi(2)
                                                + 11. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (4. * param.s1.powi(3)
                                                + 11. * param.s1.powi(2) * param.s2
                                                + -16. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 6. * param.m1_2
                                    * (param.m2_2
                                        * (param.s12.powi(3)
                                            + -5. * param.s1.powi(2) * param.s2
                                            + 3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3)
                                            + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                            + param.s12
                                                * (3. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            - param.s1.powi(3))
                                        + 2. * param.s2
                                            * (param.s12.powi(2) * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12.powi(3)))))
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
