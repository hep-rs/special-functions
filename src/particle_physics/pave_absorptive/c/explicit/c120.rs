use super::{log_diff, Parameters};

pub(crate) fn c120(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.04166666666666667
            * std::f64::consts::PI
            * param.s1.powi(-3)
            * param.lambda_s12_sqrt.powi(-7)
            * (12.
                * param.s1.powi(3)
                * (5. * param.m1_2.powi(4) * param.s2.powi(3)
                    + 10.
                        * param.m1_2.powi(3)
                        * param.s2.powi(2)
                        * (param.m2_2 * (param.s12 - param.s2 - param.s1)
                            + param.s2 * (param.s2 - param.s12 - param.s1))
                    + param.m0_2.powi(4)
                        * param.s12
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + param.s12 * (-2. * param.s1 + 3. * param.s2))
                    + 6. * param.m1_2.powi(2)
                        * param.s2
                        * ((param.s12.powi(2)
                            + param.s12 * (3. * param.s1 + -2. * param.s2)
                            + (param.s1 - param.s2).powi(2))
                            * param.s2.powi(2)
                            + param.m2_2.powi(2)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                            - param.m2_2
                                * param.s2
                                * (-3. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -4. * param.s2)
                                    + param.s1 * param.s2
                                    + 2. * param.s2.powi(2)))
                    + param.s1
                        * (param.s12
                            * (param.s12.powi(2)
                                + param.s12 * (3. * param.s1 + -2. * param.s2)
                                + (param.s1 - param.s2).powi(2))
                            * param.s2.powi(3)
                            + param.m2_2.powi(4)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                            + 3. * param.m2_2.powi(2)
                                * param.s2
                                * (param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                    - param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + param.s2))
                            - param.m2_2
                                * param.s2.powi(2)
                                * (3. * param.s12.powi(3)
                                    + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                    + param.s12
                                        * (-5. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3))
                            - param.m2_2.powi(3)
                                * (param.s12.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + 3. * param.s1 * param.s2.powi(2)
                                    + 3. * param.s2.powi(3)
                                    + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    - param.s1.powi(3)))
                    + param.m1_2
                        * (-3.
                            * param.m2_2
                            * param.s2.powi(2)
                            * ((param.s1 - param.s2).powi(2) * (3. * param.s1 + param.s2)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                - param.s12.powi(3))
                            + param.s2.powi(3)
                                * (param.s12.powi(2) * (-9. * param.s1 + 3. * param.s2)
                                    + -3.
                                        * param.s12
                                        * (3. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3)
                                    - param.s12.powi(3))
                            + param.m2_2.powi(3)
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
                            + -3.
                                * param.m2_2.powi(2)
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
                                    - param.s2.powi(3)))
                    + param.m0_2.powi(2)
                        * (6.
                            * param.m1_2.powi(2)
                            * param.s2
                            * (param.s12.powi(2)
                                + (param.s1 - param.s2).powi(2)
                                + param.s12 * (-2. * param.s1 + 3. * param.s2))
                            + 3. * param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                    - param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + param.s2))
                            + 3. * param.s12
                                * param.s2
                                * (param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                    - param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + param.s2))
                            + 3. * param.m1_2
                                * (param.s2
                                    * (-3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3))
                                    + param.m2_2
                                        * (param.s12.powi(3)
                                            + param.s12.powi(2)
                                                * (-3. * param.s1 + 5. * param.s2)
                                            + param.s12
                                                * (3. * param.s1.powi(2)
                                                    + -4. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(2)
                                                * (param.s1 + 3. * param.s2)))
                            - param.m2_2
                                * (2. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-5. * param.s1 + 4. * param.s2)
                                    + param.s12.powi(2)
                                        * (3. * param.s1.powi(2)
                                            + 11. * param.s1 * param.s2
                                            + -12. * param.s2.powi(2))
                                    + param.s12
                                        * (param.s1.powi(3)
                                            + -16. * param.s1.powi(2) * param.s2
                                            + 11. * param.s1 * param.s2.powi(2)
                                            + 4. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(3) * (param.s1 + 2. * param.s2)))
                    - param.m0_2
                        * (10.
                            * param.m1_2.powi(3)
                            * param.s2.powi(2)
                            * (param.s12 + param.s2 - param.s1)
                            + param.s12
                                * param.s2.powi(2)
                                * ((param.s1 - param.s2).powi(2) * (3. * param.s1 + param.s2)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - param.s12.powi(3))
                            + param.m2_2.powi(3)
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
                            + param.m2_2.powi(2)
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
                            + param.m2_2
                                * param.s2
                                * (2. * param.s12.powi(4)
                                    + param.s12.powi(3) * (4. * param.s1 + -5. * param.s2)
                                    + (param.s1 - param.s2).powi(3) * (2. * param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (-12. * param.s1.powi(2)
                                            + 11. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + param.s12
                                        * (4. * param.s1.powi(3)
                                            + 11. * param.s1.powi(2) * param.s2
                                            + -16. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + 6. * param.m1_2.powi(2)
                                * param.s2
                                * (param.m2_2
                                    * (2. * param.s1.powi(2)
                                        + 2. * param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -3. * param.s2.powi(2)
                                        + param.s12 * (-4. * param.s1 + param.s2))
                                    + param.s2
                                        * (-3. * param.s12.powi(2)
                                            + 2. * (param.s1 - param.s2).powi(2)
                                            + param.s12 * (param.s1 + param.s2)))
                            + 3. * param.m1_2
                                * (param.m2_2.powi(2)
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
                                    + -4.
                                        * param.m2_2
                                        * param.s2
                                        * (param.s12.powi(3)
                                            + (param.s1 - param.s2).powi(2)
                                                * (param.s1 + param.s2)
                                            - param.s12
                                                * (param.s1.powi(2)
                                                    + -4. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            - param.s12.powi(2) * (param.s1 + param.s2))
                                    + param.s2.powi(2)
                                        * (3. * param.s12.powi(3)
                                            + param.s12.powi(2)
                                                * (3. * param.s1 + -5. * param.s2)
                                            + param.s12
                                                * (-5. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(3))))
                    - param.m0_2.powi(3)
                        * (param.m2_2
                            * (3. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2)))
                            + param.m1_2
                                * (param.s12.powi(3)
                                    + -3. * param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3))
                            + param.s12
                                * (param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (-3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - param.s12.powi(3))))
                * log_diff(
                    param.m0_2 * (param.s1 + param.s12 - param.s2)
                        + param.m1_2 * (param.s1 + param.s2 - param.s12)
                        + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                    param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m1_2.powi(3)
                    * (param.s1.powi(5)
                        + -8. * param.s1.powi(4) * param.s2
                        + 37. * param.s1.powi(3) * param.s2.powi(2)
                        + 37. * param.s1.powi(2) * param.s2.powi(3)
                        + -8. * param.s1 * param.s2.powi(4)
                        + param.s2.powi(5)
                        + 5. * param.s12.powi(4) * (param.s1 + param.s2)
                        + param.s12.powi(2)
                            * (10. * param.s1.powi(3)
                                + -9. * param.s1.powi(2) * param.s2
                                + -9. * param.s1 * param.s2.powi(2)
                                + 10. * param.s2.powi(3))
                        + param.s12
                            * (-5. * param.s1.powi(4)
                                + 19. * param.s1.powi(3) * param.s2
                                + -18. * param.s1.powi(2) * param.s2.powi(2)
                                + 19. * param.s1 * param.s2.powi(3)
                                + -5. * param.s2.powi(4))
                        - param.s12.powi(3)
                            * (10. * param.s1.powi(2)
                                + 7. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        - param.s12.powi(5))
                    + param.m0_2.powi(3)
                        * (param.s12.powi(5)
                            + (param.s1 - param.s2).powi(3)
                                * (3. * param.s1.powi(2)
                                    + -3. * param.s1 * param.s2
                                    + param.s2.powi(2))
                            + param.s12.powi(3)
                                * (24. * param.s1.powi(2)
                                    + 15. * param.s1 * param.s2
                                    + 10. * param.s2.powi(2))
                            + param.s12
                                * (7. * param.s1.powi(4)
                                    + 11. * param.s1.powi(3) * param.s2
                                    + -12. * param.s1.powi(2) * param.s2.powi(2)
                                    + -11. * param.s1 * param.s2.powi(3)
                                    + 5. * param.s2.powi(4))
                            - param.s12.powi(2)
                                * (28. * param.s1.powi(3)
                                    + -3. * param.s1.powi(2) * param.s2
                                    + 3. * param.s1 * param.s2.powi(2)
                                    + 10. * param.s2.powi(3))
                            - param.s12.powi(4) * (7. * param.s1 + 5. * param.s2))
                    + param.m1_2.powi(2)
                        * param.s1
                        * (-3. * param.s1.powi(5)
                            + 3. * param.s12.powi(5)
                            + 22. * param.s1.powi(4) * param.s2
                            + -83. * param.s1.powi(3) * param.s2.powi(2)
                            + 17. * param.s1.powi(2) * param.s2.powi(3)
                            + 52. * param.s1 * param.s2.powi(4)
                            + -5. * param.s2.powi(5)
                            + param.s12.powi(3)
                                * (30. * param.s1.powi(2)
                                    + 29. * param.s1 * param.s2
                                    + 38. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-30. * param.s1.powi(3)
                                    + 15. * param.s1.powi(2) * param.s2
                                    + 39. * param.s1 * param.s2.powi(2)
                                    + -42. * param.s2.powi(3))
                            + param.s12
                                * (15. * param.s1.powi(4)
                                    + -49. * param.s1.powi(3) * param.s2
                                    + 6. * param.s1.powi(2) * param.s2.powi(2)
                                    + -105. * param.s1 * param.s2.powi(3)
                                    + 23. * param.s2.powi(4))
                            + 2. * param.m2_2
                                * (param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + -14. * param.s1.powi(3) * param.s2
                                    + -64. * param.s1.powi(2) * param.s2.powi(2)
                                    + -14. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (param.s1.powi(2) + param.s2.powi(2)
                                            - param.s1 * param.s2)
                                    + -4.
                                        * param.s12
                                        * (param.s1.powi(3)
                                            + -6. * param.s1.powi(2) * param.s2
                                            + -6. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            - param.s12.powi(4) * (15. * param.s1 + 17. * param.s2))
                    + param.m1_2
                        * param.s1.powi(2)
                        * (-3. * param.s12.powi(5)
                            + param.s12.powi(4) * (15. * param.s1 + 19. * param.s2)
                            + (param.s1 - param.s2).powi(2)
                                * (3. * param.s1.powi(3)
                                    + -14. * param.s1.powi(2) * param.s2
                                    + 30. * param.s1 * param.s2.powi(2)
                                    + 13. * param.s2.powi(3))
                            + param.s12.powi(2)
                                * (30. * param.s1.powi(3)
                                    + -3. * param.s1.powi(2) * param.s2
                                    + -33. * param.s1 * param.s2.powi(2)
                                    + 72. * param.s2.powi(3))
                            + param.s12
                                * (-15. * param.s1.powi(4)
                                    + 41. * param.s1.powi(3) * param.s2
                                    + 24. * param.s1.powi(2) * param.s2.powi(2)
                                    + 51. * param.s1 * param.s2.powi(3)
                                    + -49. * param.s2.powi(4))
                            + -6.
                                * param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + -14. * param.s1.powi(2) * param.s2
                                    + -14. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 17. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3))
                            + -4.
                                * param.m2_2
                                * (param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + -11. * param.s1.powi(3) * param.s2
                                    + -22. * param.s1.powi(2) * param.s2.powi(2)
                                    + 28. * param.s1 * param.s2.powi(3)
                                    + 4. * param.s2.powi(4)
                                    + 3. * param.s12.powi(2)
                                        * (2. * param.s1.powi(2)
                                            + param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    - param.s12
                                        * (4. * param.s1.powi(3)
                                            + -15. * param.s1.powi(2) * param.s2
                                            + 27. * param.s1 * param.s2.powi(2)
                                            + 13. * param.s2.powi(3))
                                    - param.s12.powi(3) * (4. * param.s1 + 7. * param.s2))
                            - param.s12.powi(3)
                                * (30. * param.s1.powi(2)
                                    + 37. * param.s1 * param.s2
                                    + 52. * param.s2.powi(2)))
                    + param.s1.powi(3)
                        * (param.s12.powi(5)
                            + param.s12.powi(3)
                                * (10. * param.s1.powi(2)
                                    + 15. * param.s1 * param.s2
                                    + 24. * param.s2.powi(2))
                            + param.s12
                                * (5. * param.s1.powi(4)
                                    + -11. * param.s1.powi(3) * param.s2
                                    + -12. * param.s1.powi(2) * param.s2.powi(2)
                                    + 11. * param.s1 * param.s2.powi(3)
                                    + 7. * param.s2.powi(4))
                            + -12.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                            + 6. * param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + -3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + -8. * param.s1.powi(2) * param.s2
                                    + 4. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 5. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2))
                                    - param.s1.powi(3))
                            + 2. * param.m2_2
                                * (param.s12.powi(4)
                                    + 6. * param.s12.powi(2) * (param.s1 + param.s2).powi(2)
                                    + -2. * param.s12.powi(3) * (2. * param.s1 + 5. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + -6. * param.s1 * param.s2
                                            + -11. * param.s2.powi(2))
                                    + param.s12
                                        * (-4. * param.s1.powi(3)
                                            + 6. * param.s1.powi(2) * param.s2
                                            + -42. * param.s1 * param.s2.powi(2)
                                            + 14. * param.s2.powi(3)))
                            - param.s12.powi(2)
                                * (10. * param.s1.powi(3)
                                    + 3. * param.s1.powi(2) * param.s2
                                    + -3. * param.s1 * param.s2.powi(2)
                                    + 28. * param.s2.powi(3))
                            - (param.s1 - param.s2).powi(3)
                                * (param.s1.powi(2)
                                    + -3. * param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                            - param.s12.powi(4) * (5. * param.s1 + 7. * param.s2))
                    + param.m0_2.powi(2)
                        * (param.m1_2
                            * (-3. * param.s12.powi(5)
                                + param.s12.powi(4) * (19. * param.s1 + 15. * param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (13. * param.s1.powi(3)
                                        + 30. * param.s1.powi(2) * param.s2
                                        + -14. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (72. * param.s1.powi(3)
                                        + -33. * param.s1.powi(2) * param.s2
                                        + -3. * param.s1 * param.s2.powi(2)
                                        + 30. * param.s2.powi(3))
                                + param.s12
                                    * (-49. * param.s1.powi(4)
                                        + 51. * param.s1.powi(3) * param.s2
                                        + 24. * param.s1.powi(2) * param.s2.powi(2)
                                        + 41. * param.s1 * param.s2.powi(3)
                                        + -15. * param.s2.powi(4))
                                - param.s12.powi(3)
                                    * (52. * param.s1.powi(2)
                                        + 37. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2)))
                            - param.s1
                                * (param.s12.powi(5)
                                    + -3. * param.s12.powi(4) * (3. * param.s1 + param.s2)
                                    + (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2) + 3. * param.s1 * param.s2
                                            - param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (20. * param.s1.powi(2)
                                            + -3. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-16. * param.s1.powi(3)
                                            + -51. * param.s1.powi(2) * param.s2
                                            + 27. * param.s1 * param.s2.powi(2)
                                            + 2. * param.s2.powi(3))
                                    + 3. * param.s12
                                        * (param.s1.powi(4)
                                            + 19. * param.s1.powi(3) * param.s2
                                            + -16. * param.s1.powi(2) * param.s2.powi(2)
                                            + -3. * param.s1 * param.s2.powi(3)
                                            - param.s2.powi(4))
                                    + -2.
                                        * param.m2_2
                                        * (param.s12.powi(4)
                                            + 6. * param.s12.powi(2)
                                                * (param.s1 + param.s2).powi(2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (5. * param.s1 + 2. * param.s2)
                                            + 2. * param.s12
                                                * (7. * param.s1.powi(3)
                                                    + -21. * param.s1.powi(2) * param.s2
                                                    + 3. * param.s1 * param.s2.powi(2)
                                                    + -2. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(2)
                                                * (11. * param.s1.powi(2)
                                                    + 6. * param.s1 * param.s2
                                                    - param.s2.powi(2)))))
                    + param.m0_2
                        * (param.m1_2.powi(2)
                            * (-5. * param.s1.powi(5)
                                + 3. * param.s12.powi(5)
                                + 52. * param.s1.powi(4) * param.s2
                                + 17. * param.s1.powi(3) * param.s2.powi(2)
                                + -83. * param.s1.powi(2) * param.s2.powi(3)
                                + 22. * param.s1 * param.s2.powi(4)
                                + -3. * param.s2.powi(5)
                                + param.s12.powi(3)
                                    * (38. * param.s1.powi(2)
                                        + 29. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-42. * param.s1.powi(3)
                                        + 39. * param.s1.powi(2) * param.s2
                                        + 15. * param.s1 * param.s2.powi(2)
                                        + -30. * param.s2.powi(3))
                                + param.s12
                                    * (23. * param.s1.powi(4)
                                        + -105. * param.s1.powi(3) * param.s2
                                        + 6. * param.s1.powi(2) * param.s2.powi(2)
                                        + -49. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4))
                                - param.s12.powi(4) * (17. * param.s1 + 15. * param.s2))
                            + 2. * param.m1_2
                                * param.s1
                                * (7. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (9. * param.s1.powi(2)
                                            + 16. * param.s1 * param.s2
                                            + 9. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(3)
                                            + -19. * param.s1.powi(2) * param.s2
                                            + -19. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3))
                                    + 2. * param.s12.powi(2)
                                        * (11. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + 11. * param.s2.powi(3))
                                    + param.s12
                                        * (-13. * param.s1.powi(4)
                                            + 32. * param.s1.powi(3) * param.s2
                                            + -90. * param.s1.powi(2) * param.s2.powi(2)
                                            + 32. * param.s1 * param.s2.powi(3)
                                            + -13. * param.s2.powi(4))
                                    + -2.
                                        * param.m2_2
                                        * (4. * param.s1.powi(4)
                                            + param.s12.powi(4)
                                            + 28. * param.s1.powi(3) * param.s2
                                            + -22. * param.s1.powi(2) * param.s2.powi(2)
                                            + -11. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4)
                                            + 3. * param.s12.powi(2)
                                                * (5. * param.s1.powi(2)
                                                    + param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            - param.s12
                                                * (13. * param.s1.powi(3)
                                                    + 27. * param.s1.powi(2) * param.s2
                                                    + -15. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12.powi(3)
                                                * (7. * param.s1 + 4. * param.s2))
                                    - param.s12.powi(5))
                            + param.s1.powi(2)
                                * (3. * param.s12.powi(4) * (param.s1 + 3. * param.s2)
                                    + param.s12.powi(3)
                                        * (-2. * param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + -20. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-2. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + 51. * param.s1 * param.s2.powi(2)
                                            + 16. * param.s2.powi(3))
                                    + 3. * param.s12
                                        * (param.s1.powi(4)
                                            + 3. * param.s1.powi(3) * param.s2
                                            + 16. * param.s1.powi(2) * param.s2.powi(2)
                                            + -19. * param.s1 * param.s2.powi(3)
                                            - param.s2.powi(4))
                                    + 6. * param.m2_2.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 3. * param.s12.powi(2) * (param.s1 - param.s2)
                                            + 4. * param.s1.powi(2) * param.s2
                                            + -8. * param.s1 * param.s2.powi(2)
                                            + param.s12
                                                * (-9. * param.s1.powi(2)
                                                    + 5. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            - param.s2.powi(3))
                                    + -4.
                                        * param.m2_2
                                        * (param.s12.powi(4)
                                            + -3.
                                                * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -9. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + -2.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (5. * param.s1.powi(3)
                                                    + -18. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            - param.s12.powi(3) * (param.s1 + param.s2))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2) + -3. * param.s1 * param.s2
                                            - param.s2.powi(2))
                                    - param.s12.powi(5))))
                    * param.lambda_m01_sqrt
                    * param.lambda_s12_sqrt)
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.04166666666666667
            * std::f64::consts::PI
            * param.lambda_s12_sqrt.powi(-7)
            * (12.
                * param.s2
                * (5. * param.m1_2.powi(4) * param.s2.powi(3)
                    + 10.
                        * param.m1_2.powi(3)
                        * param.s2.powi(2)
                        * (param.m2_2 * (param.s12 - param.s2 - param.s1)
                            + param.s2 * (param.s2 - param.s12 - param.s1))
                    + param.m0_2.powi(4)
                        * param.s12
                        * (param.s12.powi(2)
                            + (param.s1 - param.s2).powi(2)
                            + param.s12 * (-2. * param.s1 + 3. * param.s2))
                    + 6. * param.m1_2.powi(2)
                        * param.s2
                        * ((param.s12.powi(2)
                            + param.s12 * (3. * param.s1 + -2. * param.s2)
                            + (param.s1 - param.s2).powi(2))
                            * param.s2.powi(2)
                            + param.m2_2.powi(2)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                            - param.m2_2
                                * param.s2
                                * (-3. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -4. * param.s2)
                                    + param.s1 * param.s2
                                    + 2. * param.s2.powi(2)))
                    + param.s1
                        * (param.s12
                            * (param.s12.powi(2)
                                + param.s12 * (3. * param.s1 + -2. * param.s2)
                                + (param.s1 - param.s2).powi(2))
                            * param.s2.powi(3)
                            + param.m2_2.powi(4)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                            + 3. * param.m2_2.powi(2)
                                * param.s2
                                * (param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                    - param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + param.s2))
                            - param.m2_2
                                * param.s2.powi(2)
                                * (3. * param.s12.powi(3)
                                    + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                    + param.s12
                                        * (-5. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3))
                            - param.m2_2.powi(3)
                                * (param.s12.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + 3. * param.s1 * param.s2.powi(2)
                                    + 3. * param.s2.powi(3)
                                    + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    - param.s1.powi(3)))
                    + param.m1_2
                        * (-3.
                            * param.m2_2
                            * param.s2.powi(2)
                            * ((param.s1 - param.s2).powi(2) * (3. * param.s1 + param.s2)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                - param.s12.powi(3))
                            + param.s2.powi(3)
                                * (param.s12.powi(2) * (-9. * param.s1 + 3. * param.s2)
                                    + -3.
                                        * param.s12
                                        * (3. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3)
                                    - param.s12.powi(3))
                            + param.m2_2.powi(3)
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
                            + -3.
                                * param.m2_2.powi(2)
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
                                    - param.s2.powi(3)))
                    + param.m0_2.powi(2)
                        * (6.
                            * param.m1_2.powi(2)
                            * param.s2
                            * (param.s12.powi(2)
                                + (param.s1 - param.s2).powi(2)
                                + param.s12 * (-2. * param.s1 + 3. * param.s2))
                            + 3. * param.m2_2.powi(2)
                                * (param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                    - param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + param.s2))
                            + 3. * param.s12
                                * param.s2
                                * (param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                    - param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + param.s2))
                            + 3. * param.m1_2
                                * (param.s2
                                    * (-3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3))
                                    + param.m2_2
                                        * (param.s12.powi(3)
                                            + param.s12.powi(2)
                                                * (-3. * param.s1 + 5. * param.s2)
                                            + param.s12
                                                * (3. * param.s1.powi(2)
                                                    + -4. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(2)
                                                * (param.s1 + 3. * param.s2)))
                            - param.m2_2
                                * (2. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-5. * param.s1 + 4. * param.s2)
                                    + param.s12.powi(2)
                                        * (3. * param.s1.powi(2)
                                            + 11. * param.s1 * param.s2
                                            + -12. * param.s2.powi(2))
                                    + param.s12
                                        * (param.s1.powi(3)
                                            + -16. * param.s1.powi(2) * param.s2
                                            + 11. * param.s1 * param.s2.powi(2)
                                            + 4. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(3) * (param.s1 + 2. * param.s2)))
                    - param.m0_2
                        * (10.
                            * param.m1_2.powi(3)
                            * param.s2.powi(2)
                            * (param.s12 + param.s2 - param.s1)
                            + param.s12
                                * param.s2.powi(2)
                                * ((param.s1 - param.s2).powi(2) * (3. * param.s1 + param.s2)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - param.s12.powi(3))
                            + param.m2_2.powi(3)
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
                            + param.m2_2.powi(2)
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
                            + param.m2_2
                                * param.s2
                                * (2. * param.s12.powi(4)
                                    + param.s12.powi(3) * (4. * param.s1 + -5. * param.s2)
                                    + (param.s1 - param.s2).powi(3) * (2. * param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (-12. * param.s1.powi(2)
                                            + 11. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + param.s12
                                        * (4. * param.s1.powi(3)
                                            + 11. * param.s1.powi(2) * param.s2
                                            + -16. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + 6. * param.m1_2.powi(2)
                                * param.s2
                                * (param.m2_2
                                    * (2. * param.s1.powi(2)
                                        + 2. * param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -3. * param.s2.powi(2)
                                        + param.s12 * (-4. * param.s1 + param.s2))
                                    + param.s2
                                        * (-3. * param.s12.powi(2)
                                            + 2. * (param.s1 - param.s2).powi(2)
                                            + param.s12 * (param.s1 + param.s2)))
                            + 3. * param.m1_2
                                * (param.m2_2.powi(2)
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
                                    + -4.
                                        * param.m2_2
                                        * param.s2
                                        * (param.s12.powi(3)
                                            + (param.s1 - param.s2).powi(2)
                                                * (param.s1 + param.s2)
                                            - param.s12
                                                * (param.s1.powi(2)
                                                    + -4. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            - param.s12.powi(2) * (param.s1 + param.s2))
                                    + param.s2.powi(2)
                                        * (3. * param.s12.powi(3)
                                            + param.s12.powi(2)
                                                * (3. * param.s1 + -5. * param.s2)
                                            + param.s12
                                                * (-5. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(3))))
                    - param.m0_2.powi(3)
                        * (param.m2_2
                            * (3. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                + param.s12
                                    * (param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2)))
                            + param.m1_2
                                * (param.s12.powi(3)
                                    + -3. * param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3))
                            + param.s12
                                * (param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                    + (param.s1 - param.s2).powi(2) * (param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (-3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - param.s12.powi(3))))
                * log_diff(
                    (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1),
                    param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                )
                - (param.s2.powi(3)
                    * (-60. * param.m1_2.powi(3)
                        + param.s12.powi(3)
                        + -4.
                            * param.m1_2
                            * (8. * param.s12.powi(2)
                                + param.s12 * (29. * param.s1 + -16. * param.s2)
                                + 8. * (param.s1 - param.s2).powi(2))
                        + param.s12.powi(2) * (29. * param.s1 + -3. * param.s2)
                        + (param.s1 - param.s2).powi(3)
                        + 90. * param.m1_2.powi(2) * (param.s1 + param.s12 - param.s2)
                        + param.s12
                            * (29. * param.s1.powi(2)
                                + -32. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)))
                    + param.m2_2.powi(3)
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
                    + param.m2_2
                        * param.s2.powi(2)
                        * (-3. * param.s12.powi(3)
                            + -90. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1)
                            + (param.s1 - param.s2).powi(2) * (29. * param.s1 + 3. * param.s2)
                            + param.s12.powi(2) * (-55. * param.s1 + 9. * param.s2)
                            + param.s12
                                * (29. * param.s1.powi(2)
                                    + 32. * param.s1 * param.s2
                                    + -9. * param.s2.powi(2))
                            + 4. * param.m1_2
                                * (-29. * param.s1.powi(2)
                                    + 13. * param.s1 * param.s12
                                    + 16. * param.s12.powi(2)
                                    + 13. * param.s1 * param.s2
                                    + -32. * param.s12 * param.s2
                                    + 16. * param.s2.powi(2)))
                    + param.m0_2.powi(3)
                        * (param.s12.powi(3)
                            + param.s12.powi(2) * (-3. * param.s1 + 29. * param.s2)
                            + param.s12
                                * (3. * param.s1.powi(2)
                                    + -32. * param.s1 * param.s2
                                    + 29. * param.s2.powi(2))
                            - (param.s1 - param.s2).powi(3))
                    + param.m2_2.powi(2)
                        * param.s2
                        * (29. * param.s1.powi(3)
                            + 3. * param.s12.powi(3)
                            + param.s12.powi(2) * (23. * param.s1 + -9. * param.s2)
                            + 29. * param.s1.powi(2) * param.s2
                            + -55. * param.s1 * param.s2.powi(2)
                            + -3. * param.s2.powi(3)
                            + param.s12
                                * (-55. * param.s1.powi(2)
                                    + 32. * param.s1 * param.s2
                                    + 9. * param.s2.powi(2))
                            + -4.
                                * param.m1_2
                                * (8. * param.s1.powi(2)
                                    + 8. * param.s12.powi(2)
                                    + 29. * param.s1 * param.s2
                                    + 8. * param.s2.powi(2)
                                    + -16. * param.s12 * (param.s1 + param.s2)))
                    + param.m0_2
                        * (param.m2_2.powi(2)
                            * (-3. * param.s1.powi(3)
                                + 3. * param.s12.powi(3)
                                + -55. * param.s1.powi(2) * param.s2
                                + 29. * param.s1 * param.s2.powi(2)
                                + 29. * param.s2.powi(3)
                                + param.s12.powi(2) * (-9. * param.s1 + 23. * param.s2)
                                + param.s12
                                    * (9. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + -55. * param.s2.powi(2)))
                            + 4. * param.m2_2
                                * param.s2
                                * (-8. * param.s12.powi(3)
                                    + 8. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + -8.
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + param.s2)
                                    + param.m1_2
                                        * (16. * param.s1.powi(2)
                                            + -32. * param.s1 * param.s12
                                            + 16. * param.s12.powi(2)
                                            + 13. * param.s1 * param.s2
                                            + 13. * param.s12 * param.s2
                                            + -29. * param.s2.powi(2))
                                    + param.s12
                                        * (8. * param.s1.powi(2)
                                            + -42. * param.s1 * param.s2
                                            + 8. * param.s2.powi(2)))
                            + param.s2.powi(2)
                                * (29. * param.s12.powi(3)
                                    + param.s12.powi(2) * (29. * param.s1 + -55. * param.s2)
                                    + -3. * (param.s1 - param.s2).powi(3)
                                    + 90.
                                        * param.m1_2.powi(2)
                                        * (param.s12 + param.s2 - param.s1)
                                    + param.s12
                                        * (-55. * param.s1.powi(2)
                                            + 32. * param.s1 * param.s2
                                            + 23. * param.s2.powi(2))
                                    + param.m1_2
                                        * (-116. * param.s12.powi(2)
                                            + 64. * (param.s1 - param.s2).powi(2)
                                            + 52. * param.s12 * (param.s1 + param.s2))))
                    + param.m0_2.powi(2)
                        * (param.m2_2
                            * (-3. * param.s12.powi(3)
                                + param.s12.powi(2) * (9. * param.s1 + -55. * param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1 + 29. * param.s2)
                                + param.s12
                                    * (-9. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + 29. * param.s2.powi(2)))
                            + param.s2
                                * (29. * param.s12.powi(3)
                                    + 3. * (param.s1 - param.s2).powi(3)
                                    + param.s12.powi(2) * (-55. * param.s1 + 29. * param.s2)
                                    + param.s12
                                        * (23. * param.s1.powi(2)
                                            + 32. * param.s1 * param.s2
                                            + -55. * param.s2.powi(2))
                                    + -4.
                                        * param.m1_2
                                        * (8. * param.s12.powi(2)
                                            + 8. * (param.s1 - param.s2).powi(2)
                                            + param.s12 * (-16. * param.s1 + 29. * param.s2)))))
                    * param.lambda_m02_sqrt
                    * param.lambda_s12_sqrt)
            / param.s2
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.04166666666666667
            * std::f64::consts::PI
            * param.s12.powi(-3)
            * param.lambda_s12_sqrt.powi(-7)
            * ((7. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12
                + param.m2_2.powi(2) * param.s1.powi(5) * param.s12
                + -24. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(2)
                + -9. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(2)
                + param.m2_2 * param.s1.powi(5) * param.s12.powi(2)
                + 28. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(3)
                + 20. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(3)
                + -3. * param.m2_2 * param.s1.powi(4) * param.s12.powi(3)
                + -7. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4)
                + -16. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(4)
                + 2. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4)
                + 5. * param.s1.powi(4) * param.s12.powi(4)
                + -3. * param.m2_2.powi(3) * param.s12.powi(5)
                + 3. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5)
                + 2. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5)
                + -10. * param.s1.powi(3) * param.s12.powi(5)
                + param.m2_2.powi(2) * param.s12.powi(6)
                + -3. * param.m2_2 * param.s1 * param.s12.powi(6)
                + 10. * param.s1.powi(2) * param.s12.powi(6)
                + param.m2_2 * param.s12.powi(7)
                + -5. * param.s1 * param.s12.powi(7)
                + param.s12.powi(8)
                + 5. * param.m2_2.powi(3) * param.s1.powi(4) * param.s2
                + -15. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12 * param.s2
                + -3. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12 * param.s2
                + -3. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(2) * param.s2
                + -3. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(2) * param.s2
                + -9. * param.m2_2 * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + -11. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2
                + -51. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(3) * param.s2
                + -3. * param.m2_2 * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + 7. * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + 12. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2
                + 57. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2
                + 27. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + -15. * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + -9. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2
                + 3. * param.s1.powi(2) * param.s12.powi(5) * param.s2
                + -6. * param.m2_2 * param.s12.powi(6) * param.s2
                + 11. * param.s1 * param.s12.powi(6) * param.s2
                + -6. * param.s12.powi(7) * param.s2
                + -10. * param.m2_2.powi(3) * param.s1.powi(3) * param.s2.powi(2)
                + 3. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                + 2. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                + 12. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                + 27.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 20. * param.m2_2 * param.s1.powi(3) * param.s12.powi(2) * param.s2.powi(2)
                + -19. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(2)
                + -48. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                + -51. * param.m2_2 * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(2)
                + -24. * param.s1.powi(3) * param.s12.powi(3) * param.s2.powi(2)
                + -7. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(2)
                + -48. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + -3. * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(2)
                + 11. * param.m2_2 * param.s12.powi(5) * param.s2.powi(2)
                + 12. * param.s1 * param.s12.powi(5) * param.s2.powi(2)
                + 15. * param.s12.powi(6) * param.s2.powi(2)
                + 10. * param.m2_2.powi(3) * param.s1.powi(2) * param.s2.powi(3)
                + 11. * param.m2_2.powi(3) * param.s1 * param.s12 * param.s2.powi(3)
                + 2. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                + 15. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(3)
                + -9. * param.m2_2.powi(2) * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                + -16. * param.m2_2 * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(3)
                + 11. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(3)
                + 57. * param.m2_2 * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + 28. * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(3)
                + -7. * param.m2_2 * param.s12.powi(4) * param.s2.powi(3)
                + -11. * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + -19. * param.s12.powi(5) * param.s2.powi(3)
                + -5. * param.m2_2.powi(3) * param.s1 * param.s2.powi(4)
                + -6. * param.m2_2.powi(3) * param.s12 * param.s2.powi(4)
                + -3. * param.m2_2.powi(2) * param.s1 * param.s12 * param.s2.powi(4)
                + -6. * param.m2_2.powi(2) * param.s12.powi(2) * param.s2.powi(4)
                + 3. * param.m2_2 * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + -7. * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + 12. * param.s12.powi(4) * param.s2.powi(4)
                + param.m2_2.powi(3) * param.s2.powi(5)
                + param.m2_2.powi(2) * param.s12 * param.s2.powi(5)
                + param.m2_2 * param.s12.powi(2) * param.s2.powi(5)
                + -3. * param.s12.powi(3) * param.s2.powi(5)
                + 12.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (param.s12.powi(2)
                        + (param.s1 - param.s2).powi(2)
                        + param.s12 * (-2. * param.s1 + 3. * param.s2))
                + param.m1_2.powi(3)
                    * ((param.s1 - param.s2).powi(5)
                        + param.s12.powi(4) * (5. * param.s1 + 8. * param.s2)
                        + param.s12.powi(2)
                            * (10. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + 18. * param.s1 * param.s2.powi(2)
                                + -37. * param.s2.powi(3))
                        - param.s12.powi(3)
                            * (10. * param.s1.powi(2)
                                + 19. * param.s1 * param.s2
                                + 37. * param.s2.powi(2))
                        - param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (5. * param.s1 + 8. * param.s2)
                        - param.s12.powi(5))
                + -6.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m2_2
                        * (5. * param.s12.powi(3)
                            + (param.s1 - param.s2).powi(3)
                            + param.s12.powi(2) * (-9. * param.s1 + 4. * param.s2)
                            + param.s12
                                * (3. * param.s1.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + -8. * param.s2.powi(2)))
                        + param.s12
                            * (param.s12.powi(2) * (3. * param.s1 + -8. * param.s2)
                                + (param.s1 - param.s2).powi(2) * (param.s1 + 5. * param.s2)
                                + param.s12
                                    * (-3. * param.s1.powi(2)
                                        + 5. * param.s1 * param.s2
                                        + 4. * param.s2.powi(2))
                                - param.s12.powi(3))
                        + param.m1_2
                            * (param.s12.powi(3)
                                + param.s12.powi(2) * (-3. * param.s1 + 14. * param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + -17. * param.s1 * param.s2
                                        + 14. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(3)))
                + param.m1_2.powi(2)
                    * (param.m2_2
                        * (5. * param.s12.powi(5)
                            + -3. * (param.s1 - param.s2).powi(5)
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (17. * param.s1 + 22. * param.s2)
                            + param.s12.powi(3)
                                * (42. * param.s1.powi(2)
                                    + 105. * param.s1 * param.s2
                                    + -17. * param.s2.powi(2))
                            - param.s12.powi(2)
                                * (38. * param.s1.powi(3)
                                    + 39. * param.s1.powi(2) * param.s2
                                    + 6. * param.s1 * param.s2.powi(2)
                                    + -83. * param.s2.powi(3))
                            - param.s12.powi(4) * (23. * param.s1 + 52. * param.s2))
                        + param.s12
                            * (3. * param.s12.powi(5)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (15. * param.s1.powi(2)
                                        + param.s1 * param.s2
                                        + -52. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (30. * param.s1.powi(2)
                                        + 49. * param.s1 * param.s2
                                        + 83. * param.s2.powi(2))
                                - param.s12.powi(2)
                                    * (30. * param.s1.powi(3)
                                        + 15. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + 17. * param.s2.powi(3))
                                - param.s12.powi(4) * (15. * param.s1 + 22. * param.s2)
                                - (3. * param.s1 + -5. * param.s2)
                                    * (param.s1 - param.s2).powi(4)))
                + -2.
                    * param.m0_2
                    * param.s12
                    * (param.m1_2.powi(2)
                        * (param.s12.powi(4)
                            + (param.s1 - param.s2).powi(4)
                            + -2. * param.s12.powi(3) * (2. * param.s1 + 7. * param.s2)
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (2. * param.s1 + 7. * param.s2)
                            + param.s12.powi(2)
                                * (6. * param.s1.powi(2)
                                    + 24. * param.s1 * param.s2
                                    + -64. * param.s2.powi(2)))
                        + param.m2_2.powi(2)
                            * (-11. * param.s12.powi(4)
                                + (param.s1 - param.s2).powi(4)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (5. * param.s1 + 4. * param.s2)
                                + 2. * param.s12.powi(3) * (7. * param.s1 + 8. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + -21. * param.s1 * param.s2
                                        + param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (param.s12.powi(4)
                                + -4. * param.s12.powi(3) * (param.s1 + 2. * param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + -11. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (2. * param.s1.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + 21. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3)))
                        + 2. * param.m2_2
                            * param.s12
                            * (2. * param.s12.powi(4)
                                + param.s12.powi(3) * (-5. * param.s1 + 8. * param.s2)
                                + param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 18. * param.s1 * param.s2
                                        + -20. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1.powi(3)
                                        + -27. * param.s1.powi(2) * param.s2
                                        + 18. * param.s1 * param.s2.powi(2)
                                        + 8. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3) * (param.s1 + 2. * param.s2))
                        + -2.
                            * param.m1_2
                            * (param.m2_2
                                * (4. * param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12.powi(3) * (-13. * param.s1 + 28. * param.s2)
                                    + param.s12.powi(2)
                                        * (15. * param.s1.powi(2)
                                            + -27. * param.s1 * param.s2
                                            + -22. * param.s2.powi(2))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (7. * param.s1 + 11. * param.s2))
                                + param.s12
                                    * (param.s12.powi(4)
                                        + (param.s1 + -4. * param.s2)
                                            * (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + -22. * param.s2.powi(2))
                                        + param.s12
                                            * (-4. * param.s1.powi(3)
                                                + 3. * param.s1.powi(2) * param.s2
                                                + -27. * param.s1 * param.s2.powi(2)
                                                + 28. * param.s2.powi(3))
                                        - param.s12.powi(3) * (4. * param.s1 + 11. * param.s2))))
                - param.m1_2
                    * (2.
                        * param.m2_2
                        * param.s12
                        * (3. * param.s12.powi(5)
                            + param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (7. * param.s1.powi(2)
                                    + -18. * param.s1 * param.s2
                                    + -25. * param.s2.powi(2))
                            + param.s12.powi(3)
                                * (22. * param.s1.powi(2)
                                    + 32. * param.s1 * param.s2
                                    + 22. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (9. * param.s1.powi(3)
                                    + -9. * param.s1.powi(2) * param.s2
                                    + 45. * param.s1 * param.s2.powi(2)
                                    + -11. * param.s2.powi(3))
                            - param.s12.powi(4) * (13. * param.s1 + 25. * param.s2)
                            - (param.s1 + -3. * param.s2) * (param.s1 - param.s2).powi(4))
                        + param.m2_2.powi(2)
                            * (13. * param.s12.powi(5)
                                + -3. * (param.s1 - param.s2).powi(5)
                                + param.s12.powi(4) * (-49. * param.s1 + 4. * param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (19. * param.s1 + 20. * param.s2)
                                + param.s12.powi(3)
                                    * (72. * param.s1.powi(2)
                                        + 51. * param.s1 * param.s2
                                        + -61. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-52. * param.s1.powi(3)
                                        + -33. * param.s1.powi(2) * param.s2
                                        + 24. * param.s1 * param.s2.powi(2)
                                        + 61. * param.s2.powi(3)))
                        + param.s12.powi(2)
                            * (3. * param.s12.powi(5)
                                + -5. * param.s12.powi(4) * (3. * param.s1 + 4. * param.s2)
                                + param.s12.powi(3)
                                    * (30. * param.s1.powi(2)
                                        + 41. * param.s1 * param.s2
                                        + 61. * param.s2.powi(2))
                                + param.s12
                                    * (15. * param.s1.powi(4)
                                        + -37. * param.s1.powi(3) * param.s2
                                        + -33. * param.s1.powi(2) * param.s2.powi(2)
                                        + 51. * param.s1 * param.s2.powi(3)
                                        + 4. * param.s2.powi(4))
                                - param.s12.powi(2)
                                    * (30. * param.s1.powi(3)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + -24. * param.s1 * param.s2.powi(2)
                                        + 61. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3)
                                    * (3. * param.s1.powi(2)
                                        + -10. * param.s1 * param.s2
                                        + 13. * param.s2.powi(2))))
                - param.s1.powi(5) * param.s12.powi(3)
                - param.m2_2.powi(3) * param.s1.powi(5))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s12.powi(3)
                    * (5. * param.m1_2.powi(4) * param.s2.powi(3)
                        + 10.
                            * param.m1_2.powi(3)
                            * param.s2.powi(2)
                            * (param.m2_2 * (param.s12 - param.s2 - param.s1)
                                + param.s2 * (param.s2 - param.s12 - param.s1))
                        + param.m0_2.powi(4)
                            * param.s12
                            * (param.s12.powi(2)
                                + (param.s1 - param.s2).powi(2)
                                + param.s12 * (-2. * param.s1 + 3. * param.s2))
                        + 6. * param.m1_2.powi(2)
                            * param.s2
                            * ((param.s12.powi(2)
                                + param.s12 * (3. * param.s1 + -2. * param.s2)
                                + (param.s1 - param.s2).powi(2))
                                * param.s2.powi(2)
                                + param.m2_2.powi(2)
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2)
                                        + -2. * param.s12 * (param.s1 + param.s2))
                                - param.m2_2
                                    * param.s2
                                    * (-3. * param.s1.powi(2)
                                        + 2. * param.s12.powi(2)
                                        + param.s12 * (param.s1 + -4. * param.s2)
                                        + param.s1 * param.s2
                                        + 2. * param.s2.powi(2)))
                        + param.s1
                            * (param.s12
                                * (param.s12.powi(2)
                                    + param.s12 * (3. * param.s1 + -2. * param.s2)
                                    + (param.s1 - param.s2).powi(2))
                                * param.s2.powi(3)
                                + param.m2_2.powi(4)
                                    * (param.s1.powi(2)
                                        + param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2)
                                        + -2. * param.s12 * (param.s1 + param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.m2_2
                                    * param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3))
                                - param.m2_2.powi(3)
                                    * (param.s12.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s1.powi(3)))
                        + param.m1_2
                            * (-3.
                                * param.m2_2
                                * param.s2.powi(2)
                                * ((param.s1 - param.s2).powi(2) * (3. * param.s1 + param.s2)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - param.s12.powi(3))
                                + param.s2.powi(3)
                                    * (param.s12.powi(2) * (-9. * param.s1 + 3. * param.s2)
                                        + -3.
                                            * param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)
                                        - param.s12.powi(3))
                                + param.m2_2.powi(3)
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
                                + -3.
                                    * param.m2_2.powi(2)
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
                                        - param.s2.powi(3)))
                        + param.m0_2.powi(2)
                            * (6.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (param.s12.powi(2)
                                    + (param.s1 - param.s2).powi(2)
                                    + param.s12 * (-2. * param.s1 + 3. * param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                + 3. * param.s12
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                + 3. * param.m1_2
                                    * (param.s2
                                        * (-3. * param.s12.powi(3)
                                            + param.s12.powi(2)
                                                * (5. * param.s1 + -3. * param.s2)
                                            - param.s12
                                                * (param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(3))
                                        + param.m2_2
                                            * (param.s12.powi(3)
                                                + param.s12.powi(2)
                                                    * (-3. * param.s1 + 5. * param.s2)
                                                + param.s12
                                                    * (3. * param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + -3. * param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + 3. * param.s2)))
                                - param.m2_2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-5. * param.s1 + 4. * param.s2)
                                        + param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + 11. * param.s1 * param.s2
                                                + -12. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1 + 2. * param.s2)))
                        - param.m0_2
                            * (10.
                                * param.m1_2.powi(3)
                                * param.s2.powi(2)
                                * (param.s12 + param.s2 - param.s1)
                                + param.s12
                                    * param.s2.powi(2)
                                    * ((param.s1 - param.s2).powi(2)
                                        * (3. * param.s1 + param.s2)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - param.s12.powi(3))
                                + param.m2_2.powi(3)
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
                                + param.m2_2.powi(2)
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
                                + param.m2_2
                                    * param.s2
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
                                + 6. * param.m1_2.powi(2)
                                    * param.s2
                                    * (param.m2_2
                                        * (2. * param.s1.powi(2)
                                            + 2. * param.s12.powi(2)
                                            + param.s1 * param.s2
                                            + -3. * param.s2.powi(2)
                                            + param.s12 * (-4. * param.s1 + param.s2))
                                        + param.s2
                                            * (-3. * param.s12.powi(2)
                                                + 2. * (param.s1 - param.s2).powi(2)
                                                + param.s12 * (param.s1 + param.s2)))
                                + 3. * param.m1_2
                                    * (param.m2_2.powi(2)
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
                                        + -4.
                                            * param.m2_2
                                            * param.s2
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        + param.s2.powi(2)
                                            * (3. * param.s12.powi(3)
                                                + param.s12.powi(2)
                                                    * (3. * param.s1 + -5. * param.s2)
                                                + param.s12
                                                    * (-5. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(3))))
                        - param.m0_2.powi(3)
                            * (param.m2_2
                                * (3. * param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(3)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2)))
                                + param.m1_2
                                    * (param.s12.powi(3)
                                        + -3. * param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3))
                                + param.s12
                                    * (param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (-3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s12.powi(3))))
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
