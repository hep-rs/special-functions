use super::{log_diff, Parameters};

pub(crate) fn c111(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.02083333333333333
            * std::f64::consts::PI
            * param.s1.powi(-2)
            * param.lambda_s12_sqrt.powi(-7)
            * ((param.m0_2.powi(3)
                * (param.s12.powi(4)
                    + (param.s1 - param.s2).powi(3) * (3. * param.s1 - param.s2)
                    + -4. * param.s12.powi(3) * (3. * param.s1 + param.s2)
                    + 6. * param.s12.powi(2)
                        * (-6. * param.s1.powi(2) + 3. * param.s1 * param.s2 + param.s2.powi(2))
                    + param.s12
                        * (44. * param.s1.powi(3)
                            + -40. * param.s1.powi(2) * param.s2
                            + -4. * param.s2.powi(3)))
                + param.m1_2
                    * param.s1.powi(2)
                    * (-3. * param.s12.powi(4)
                        + -6. * param.s1 * param.s12.powi(2) * (3. * param.s1 + 5. * param.s2)
                        + 4. * param.s12.powi(3) * (3. * param.s1 + 7. * param.s2)
                        + 4. * param.s12
                            * (3. * param.s1.powi(3)
                                + -6. * param.s1.powi(2) * param.s2
                                + 47. * param.s1 * param.s2.powi(2)
                                + -18. * param.s2.powi(3))
                        + 6. * param.m2_2.powi(2)
                            * (11. * param.s1.powi(2)
                                + 11. * param.s12.powi(2)
                                + 38. * param.s1 * param.s2
                                + 11. * param.s2.powi(2)
                                + -22. * param.s12 * (param.s1 + param.s2))
                        + -4.
                            * param.m2_2
                            * (-4. * param.s1.powi(3)
                                + 4. * param.s12.powi(3)
                                + -3. * param.s12.powi(2) * (4. * param.s1 + -7. * param.s2)
                                + -53. * param.s1.powi(2) * param.s2
                                + 28. * param.s1 * param.s2.powi(2)
                                + 29. * param.s2.powi(3)
                                + 2. * param.s12
                                    * (6. * param.s1.powi(2)
                                        + 16. * param.s1 * param.s2
                                        + -27. * param.s2.powi(2)))
                        - (param.s1 - param.s2).powi(2)
                            * (3. * param.s1.powi(2)
                                + -20. * param.s1 * param.s2
                                + -47. * param.s2.powi(2)))
                + param.s1.powi(3)
                    * (param.s12.powi(4)
                        + (param.s1 + -3. * param.s2) * (param.s1 - param.s2).powi(3)
                        + 60. * param.m2_2.powi(3) * (param.s12 - param.s2 - param.s1)
                        + -4. * param.s12.powi(3) * (param.s1 + 3. * param.s2)
                        + -6.
                            * param.m2_2.powi(2)
                            * (11. * param.s1.powi(2)
                                + -22. * param.s1 * param.s12
                                + 11. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 8. * param.s12 * param.s2
                                + -19. * param.s2.powi(2))
                        + 6. * param.s12.powi(2)
                            * (param.s1.powi(2)
                                + 3. * param.s1 * param.s2
                                + -6. * param.s2.powi(2))
                        + -4.
                            * param.s12
                            * (param.s1.powi(3)
                                + 10. * param.s1 * param.s2.powi(2)
                                + -11. * param.s2.powi(3))
                        + 4. * param.m2_2
                            * (2. * param.s12.powi(3)
                                + -2.
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1 + 7. * param.s2)
                                + param.s12.powi(2) * (-6. * param.s1 + 27. * param.s2)
                                + param.s12
                                    * (6. * param.s1.powi(2)
                                        + -17. * param.s1 * param.s2
                                        + -15. * param.s2.powi(2))))
                + param.m1_2.powi(2)
                    * param.s1
                    * (3. * param.s1.powi(4)
                        + 3. * param.s12.powi(4)
                        + -34. * param.s1.powi(3) * param.s2
                        + -110. * param.s1.powi(2) * param.s2.powi(2)
                        + 130. * param.s1 * param.s2.powi(3)
                        + 11. * param.s2.powi(4)
                        + -4. * param.s12.powi(3) * (3. * param.s1 + 5. * param.s2)
                        + 6. * param.s12.powi(2)
                            * (3. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + 7. * param.s2.powi(2))
                        + -4.
                            * param.s12
                            * (3. * param.s1.powi(3)
                                + -12. * param.s1.powi(2) * param.s2
                                + 31. * param.s1 * param.s2.powi(2)
                                + 9. * param.s2.powi(3))
                        + 4. * param.m2_2
                            * (-2. * param.s1.powi(3)
                                + 2. * param.s12.powi(3)
                                + -43. * param.s1.powi(2) * param.s2
                                + -43. * param.s1 * param.s2.powi(2)
                                + -2. * param.s2.powi(3)
                                + -6. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (6. * param.s1.powi(2)
                                        + 49. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))))
                + param.m0_2
                    * (param.m1_2.powi(2)
                        * (11. * param.s1.powi(4)
                            + 3. * param.s12.powi(4)
                            + 130. * param.s1.powi(3) * param.s2
                            + -110. * param.s1.powi(2) * param.s2.powi(2)
                            + -34. * param.s1 * param.s2.powi(3)
                            + 3. * param.s2.powi(4)
                            + -4. * param.s12.powi(3) * (5. * param.s1 + 3. * param.s2)
                            + 6. * param.s12.powi(2)
                                * (7. * param.s1.powi(2)
                                    + param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                            + -4.
                                * param.s12
                                * (9. * param.s1.powi(3)
                                    + 31. * param.s1.powi(2) * param.s2
                                    + -12. * param.s1 * param.s2.powi(2)
                                    + 3. * param.s2.powi(3)))
                        + param.s1.powi(2)
                            * (-7. * param.s12.powi(4)
                                + 20. * param.s12.powi(3) * (param.s1 + -2. * param.s2)
                                + (param.s1 - param.s2).powi(3) * (param.s1 + 5. * param.s2)
                                + -6.
                                    * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 11. * param.s1 * param.s2
                                        + -16. * param.s2.powi(2))
                                + -6.
                                    * param.m2_2.powi(2)
                                    * (-19. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s12
                                        + 11. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + -22. * param.s12 * param.s2
                                        + 11. * param.s2.powi(2))
                                + 4. * param.s12
                                    * (param.s1.powi(3)
                                        + 26. * param.s1.powi(2) * param.s2
                                        + -16. * param.s1 * param.s2.powi(2)
                                        + -11. * param.s2.powi(3))
                                + 4. * param.m2_2
                                    * (17. * param.s12.powi(3)
                                        + -18. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 16.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + param.s12
                                            * (-15. * param.s1.powi(2)
                                                + 82. * param.s1 * param.s2
                                                + -15. * param.s2.powi(2))))
                        + -4.
                            * param.m1_2
                            * param.s1
                            * (6.
                                * param.s12.powi(2)
                                * (param.s1.powi(2)
                                    + -9. * param.s1 * param.s2
                                    + param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 26. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + param.s12
                                    * (-8. * param.s1.powi(3)
                                        + 34. * param.s1.powi(2) * param.s2
                                        + 34. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))
                                + param.m2_2
                                    * (29. * param.s1.powi(3)
                                        + 4. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (7. * param.s1 + -4. * param.s2)
                                        + 28. * param.s1.powi(2) * param.s2
                                        + -53. * param.s1 * param.s2.powi(2)
                                        + -4. * param.s2.powi(3)
                                        + param.s12
                                            * (-54. * param.s1.powi(2)
                                                + 32. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2)))
                                - param.s12.powi(4)))
                - param.m0_2.powi(2)
                    * (param.m1_2
                        * (3. * param.s12.powi(4)
                            + 6. * param.s12.powi(2)
                                * param.s2
                                * (5. * param.s1 + 3. * param.s2)
                            + -4. * param.s12.powi(3) * (7. * param.s1 + 3. * param.s2)
                            + 4. * param.s12
                                * (18. * param.s1.powi(3)
                                    + -47. * param.s1.powi(2) * param.s2
                                    + 6. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s2.powi(3))
                            - (param.s1 - param.s2).powi(2)
                                * (47. * param.s1.powi(2)
                                    + 20. * param.s1 * param.s2
                                    + -3. * param.s2.powi(2)))
                        + param.s1
                            * (7. * param.s12.powi(4)
                                + 20. * param.s12.powi(3) * (2. * param.s1 - param.s2)
                                + (param.s1 - param.s2).powi(3) * (5. * param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (-96. * param.s1.powi(2)
                                        + 66. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + 4. * param.s12
                                    * (11. * param.s1.powi(3)
                                        + 16. * param.s1.powi(2) * param.s2
                                        + -26. * param.s1 * param.s2.powi(2)
                                        - param.s2.powi(3))
                                + -4.
                                    * param.m2_2
                                    * (2. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (9. * param.s1 + -2. * param.s2)
                                        + -2.
                                            * (param.s1 - param.s2).powi(2)
                                            * (7. * param.s1 + param.s2)
                                        + param.s12
                                            * (-15. * param.s1.powi(2)
                                                + -17. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2)))))
                - param.m1_2.powi(3)
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
                                + param.s2.powi(3))))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s1.powi(2)
                    * (5.
                        * param.m1_2.powi(4)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(4)
                            * param.s12
                            * (3. * param.s12.powi(2) + -2. * (param.s1 - param.s2).powi(2)
                                - param.s12 * (param.s1 + param.s2))
                        + -4.
                            * param.m1_2.powi(3)
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
                        + -2.
                            * param.m0_2.powi(3)
                            * (param.m1_2
                                * (3. * param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(3)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2)))
                                + 2. * param.s12
                                    * (param.s12.powi(2) * (param.s1 + param.s2)
                                        + param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12.powi(3))
                                + param.m2_2
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + 2. * param.m1_2
                            * param.s1
                            * (4.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                + -3.
                                    * param.m2_2.powi(2)
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
                                + 6. * param.m2_2
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + 3. * param.m1_2.powi(2)
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
                        + param.m0_2.powi(2)
                            * (3.
                                * param.m1_2.powi(2)
                                * (param.s12.powi(3)
                                    + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2) * (param.s1 + 3. * param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                        + param.s12
                                            * (-3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1 + param.s2))
                                + -2.
                                    * param.m2_2
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
                                + param.s12
                                    * (param.s12.powi(4)
                                        + param.s12.powi(2)
                                            * (-3. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + -2.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (5. * param.s1.powi(3)
                                                + -11. * param.s1.powi(2) * param.s2
                                                + -11. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        - param.s12.powi(3) * (param.s1 + param.s2))
                                + 2. * param.m1_2
                                    * (-2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (5. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1 + 2. * param.s2)
                                        + param.s12.powi(2)
                                            * (-3. * param.s1.powi(2)
                                                + -11. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + 6. * param.m2_2
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        - param.s12
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))))
                        + -2.
                            * param.m0_2
                            * (2.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -3. * param.s2.powi(2)
                                    + param.s12 * (-4. * param.s1 + param.s2))
                                + 3. * param.m1_2.powi(2)
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
                                                - param.s12.powi(3)))
                                + param.m1_2
                                    * (3.
                                        * param.m2_2.powi(2)
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
                                                + param.s12.powi(3)
                                                    * (4. * param.s1 + -5. * param.s2)
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
                                                        + param.s2.powi(3))))
                                + param.s1
                                    * (2.
                                        * param.m2_2.powi(3)
                                        * (-3. * param.s1.powi(2)
                                            + 2. * param.s12.powi(2)
                                            + param.s12 * (param.s1 + -4. * param.s2)
                                            + param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                        + -6.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        + 2. * param.s12
                                            * param.s2
                                            * (param.s12.powi(2) * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12.powi(3))
                                        + param.m2_2
                                            * (2. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-5. * param.s1 + 4. * param.s2)
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
                                                    * (param.s1 + 2. * param.s2))))
                        - param.s1.powi(2)
                            * (5. * param.m2_2.powi(4) * (param.s1 + param.s2 - param.s12)
                                + 4. * param.m2_2.powi(3)
                                    * (2. * param.s1.powi(2)
                                        + 2. * param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -3. * param.s2.powi(2)
                                        + param.s12 * (-4. * param.s1 + param.s2))
                                + param.s12
                                    * param.s2.powi(2)
                                    * (-3. * param.s12.powi(2)
                                        + 2. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (3. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2)))
                                + -3.
                                    * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2))))
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.02083333333333333
            * std::f64::consts::PI
            * param.s2.powi(-2)
            * param.lambda_s12_sqrt.powi(-7)
            * ((param.m0_2.powi(3)
                * (param.s12.powi(4)
                    + (param.s1 + -3. * param.s2) * (param.s1 - param.s2).powi(3)
                    + -4. * param.s12.powi(3) * (param.s1 + 3. * param.s2)
                    + 6. * param.s12.powi(2)
                        * (param.s1.powi(2) + 3. * param.s1 * param.s2 + -6. * param.s2.powi(2))
                    + -4.
                        * param.s12
                        * (param.s1.powi(3)
                            + 10. * param.s1 * param.s2.powi(2)
                            + -11. * param.s2.powi(3)))
                + param.s2.powi(3)
                    * (param.s12.powi(4)
                        + (param.s1 - param.s2).powi(3) * (3. * param.s1 - param.s2)
                        + 60. * param.m1_2.powi(3) * (param.s12 - param.s2 - param.s1)
                        + -4. * param.s12.powi(3) * (3. * param.s1 + param.s2)
                        + 6. * param.s12.powi(2)
                            * (-6. * param.s1.powi(2)
                                + 3. * param.s1 * param.s2
                                + param.s2.powi(2))
                        + -6.
                            * param.m1_2.powi(2)
                            * (-19. * param.s1.powi(2)
                                + 8. * param.s1 * param.s12
                                + 11. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + -22. * param.s12 * param.s2
                                + 11. * param.s2.powi(2))
                        + param.s12
                            * (44. * param.s1.powi(3)
                                + -40. * param.s1.powi(2) * param.s2
                                + -4. * param.s2.powi(3))
                        + 4. * param.m1_2
                            * (2. * param.s12.powi(3)
                                + 3. * param.s12.powi(2) * (9. * param.s1 + -2. * param.s2)
                                + -2.
                                    * (param.s1 - param.s2).powi(2)
                                    * (7. * param.s1 + param.s2)
                                + param.s12
                                    * (-15. * param.s1.powi(2)
                                        + -17. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))))
                + param.m2_2.powi(2)
                    * param.s2
                    * (11. * param.s1.powi(4)
                        + 3. * param.s12.powi(4)
                        + 130. * param.s1.powi(3) * param.s2
                        + -110. * param.s1.powi(2) * param.s2.powi(2)
                        + -34. * param.s1 * param.s2.powi(3)
                        + 3. * param.s2.powi(4)
                        + -4. * param.s12.powi(3) * (5. * param.s1 + 3. * param.s2)
                        + 6. * param.s12.powi(2)
                            * (7. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + 3. * param.s2.powi(2))
                        + -4.
                            * param.s12
                            * (9. * param.s1.powi(3)
                                + 31. * param.s1.powi(2) * param.s2
                                + -12. * param.s1 * param.s2.powi(2)
                                + 3. * param.s2.powi(3))
                        + 4. * param.m1_2
                            * (-2. * param.s1.powi(3)
                                + 2. * param.s12.powi(3)
                                + -43. * param.s1.powi(2) * param.s2
                                + -43. * param.s1 * param.s2.powi(2)
                                + -2. * param.s2.powi(3)
                                + -6. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (6. * param.s1.powi(2)
                                        + 49. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))))
                + param.m2_2
                    * param.s2.powi(2)
                    * (-3. * param.s12.powi(4)
                        + -6. * param.s12.powi(2) * param.s2 * (5. * param.s1 + 3. * param.s2)
                        + 4. * param.s12.powi(3) * (7. * param.s1 + 3. * param.s2)
                        + (param.s1 - param.s2).powi(2)
                            * (47. * param.s1.powi(2)
                                + 20. * param.s1 * param.s2
                                + -3. * param.s2.powi(2))
                        + -4.
                            * param.s12
                            * (18. * param.s1.powi(3)
                                + -47. * param.s1.powi(2) * param.s2
                                + 6. * param.s1 * param.s2.powi(2)
                                + -3. * param.s2.powi(3))
                        + 6. * param.m1_2.powi(2)
                            * (11. * param.s1.powi(2)
                                + 11. * param.s12.powi(2)
                                + 38. * param.s1 * param.s2
                                + 11. * param.s2.powi(2)
                                + -22. * param.s12 * (param.s1 + param.s2))
                        + -4.
                            * param.m1_2
                            * (29. * param.s1.powi(3)
                                + 4. * param.s12.powi(3)
                                + 3. * param.s12.powi(2) * (7. * param.s1 + -4. * param.s2)
                                + 28. * param.s1.powi(2) * param.s2
                                + -53. * param.s1 * param.s2.powi(2)
                                + -4. * param.s2.powi(3)
                                + param.s12
                                    * (-54. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2))))
                + param.m0_2
                    * (param.m2_2.powi(2)
                        * (3. * param.s1.powi(4)
                            + 3. * param.s12.powi(4)
                            + -34. * param.s1.powi(3) * param.s2
                            + -110. * param.s1.powi(2) * param.s2.powi(2)
                            + 130. * param.s1 * param.s2.powi(3)
                            + 11. * param.s2.powi(4)
                            + -4. * param.s12.powi(3) * (3. * param.s1 + 5. * param.s2)
                            + 6. * param.s12.powi(2)
                                * (3. * param.s1.powi(2)
                                    + param.s1 * param.s2
                                    + 7. * param.s2.powi(2))
                            + -4.
                                * param.s12
                                * (3. * param.s1.powi(3)
                                    + -12. * param.s1.powi(2) * param.s2
                                    + 31. * param.s1 * param.s2.powi(2)
                                    + 9. * param.s2.powi(3)))
                        + -4.
                            * param.m2_2
                            * param.s2
                            * (6.
                                * param.s12.powi(2)
                                * (param.s1.powi(2)
                                    + -9. * param.s1 * param.s2
                                    + param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 26. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + param.s12
                                    * (-8. * param.s1.powi(3)
                                        + 34. * param.s1.powi(2) * param.s2
                                        + 34. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))
                                + param.m1_2
                                    * (-4. * param.s1.powi(3)
                                        + 4. * param.s12.powi(3)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (4. * param.s1 + -7. * param.s2)
                                        + -53. * param.s1.powi(2) * param.s2
                                        + 28. * param.s1 * param.s2.powi(2)
                                        + 29. * param.s2.powi(3)
                                        + 2. * param.s12
                                            * (6. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -27. * param.s2.powi(2)))
                                - param.s12.powi(4))
                        + param.s2.powi(2)
                            * (-7. * param.s12.powi(4)
                                + 20. * param.s12.powi(3) * (-2. * param.s1 + param.s2)
                                + -6.
                                    * param.m1_2.powi(2)
                                    * (11. * param.s1.powi(2)
                                        + -22. * param.s1 * param.s12
                                        + 11. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 8. * param.s12 * param.s2
                                        + -19. * param.s2.powi(2))
                                + 6. * param.s12.powi(2)
                                    * (16. * param.s1.powi(2)
                                        + -11. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + 4. * param.s12
                                    * (-11. * param.s1.powi(3)
                                        + -16. * param.s1.powi(2) * param.s2
                                        + 26. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 4. * param.m1_2
                                    * (17. * param.s12.powi(3)
                                        + -18. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 16.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + param.s12
                                            * (-15. * param.s1.powi(2)
                                                + 82. * param.s1 * param.s2
                                                + -15. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(3) * (5. * param.s1 + param.s2)))
                - param.m0_2.powi(2)
                    * (param.m2_2
                        * (3. * param.s12.powi(4)
                            + 6. * param.s1
                                * param.s12.powi(2)
                                * (3. * param.s1 + 5. * param.s2)
                            + -4. * param.s12.powi(3) * (3. * param.s1 + 7. * param.s2)
                            + (param.s1 - param.s2).powi(2)
                                * (3. * param.s1.powi(2)
                                    + -20. * param.s1 * param.s2
                                    + -47. * param.s2.powi(2))
                            + -4.
                                * param.s12
                                * (3. * param.s1.powi(3)
                                    + -6. * param.s1.powi(2) * param.s2
                                    + 47. * param.s1 * param.s2.powi(2)
                                    + -18. * param.s2.powi(3)))
                        + param.s2
                            * (7. * param.s12.powi(4)
                                + -20. * param.s12.powi(3) * (param.s1 + -2. * param.s2)
                                + 6. * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 11. * param.s1 * param.s2
                                        + -16. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (param.s1.powi(3)
                                        + 26. * param.s1.powi(2) * param.s2
                                        + -16. * param.s1 * param.s2.powi(2)
                                        + -11. * param.s2.powi(3))
                                + -4.
                                    * param.m1_2
                                    * (2. * param.s12.powi(3)
                                        + -2.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 7. * param.s2)
                                        + param.s12.powi(2) * (-6. * param.s1 + 27. * param.s2)
                                        + param.s12
                                            * (6. * param.s1.powi(2)
                                                + -17. * param.s1 * param.s2
                                                + -15. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(3) * (param.s1 + 5. * param.s2)))
                - param.m2_2.powi(3)
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
                                + param.s2.powi(3))))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s2.powi(2)
                    * (5.
                        * param.m1_2.powi(4)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(4)
                            * param.s12
                            * (3. * param.s12.powi(2) + -2. * (param.s1 - param.s2).powi(2)
                                - param.s12 * (param.s1 + param.s2))
                        + -4.
                            * param.m1_2.powi(3)
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
                        + -2.
                            * param.m0_2.powi(3)
                            * (param.m1_2
                                * (3. * param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(3)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2)))
                                + 2. * param.s12
                                    * (param.s12.powi(2) * (param.s1 + param.s2)
                                        + param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12.powi(3))
                                + param.m2_2
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + 2. * param.m1_2
                            * param.s1
                            * (4.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                + -3.
                                    * param.m2_2.powi(2)
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
                                + 6. * param.m2_2
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + 3. * param.m1_2.powi(2)
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
                        + param.m0_2.powi(2)
                            * (3.
                                * param.m1_2.powi(2)
                                * (param.s12.powi(3)
                                    + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2) * (param.s1 + 3. * param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                        + param.s12
                                            * (-3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1 + param.s2))
                                + -2.
                                    * param.m2_2
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
                                + param.s12
                                    * (param.s12.powi(4)
                                        + param.s12.powi(2)
                                            * (-3. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + -2.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (5. * param.s1.powi(3)
                                                + -11. * param.s1.powi(2) * param.s2
                                                + -11. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        - param.s12.powi(3) * (param.s1 + param.s2))
                                + 2. * param.m1_2
                                    * (-2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (5. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1 + 2. * param.s2)
                                        + param.s12.powi(2)
                                            * (-3. * param.s1.powi(2)
                                                + -11. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + 6. * param.m2_2
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        - param.s12
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))))
                        + -2.
                            * param.m0_2
                            * (2.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -3. * param.s2.powi(2)
                                    + param.s12 * (-4. * param.s1 + param.s2))
                                + 3. * param.m1_2.powi(2)
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
                                                - param.s12.powi(3)))
                                + param.m1_2
                                    * (3.
                                        * param.m2_2.powi(2)
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
                                                + param.s12.powi(3)
                                                    * (4. * param.s1 + -5. * param.s2)
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
                                                        + param.s2.powi(3))))
                                + param.s1
                                    * (2.
                                        * param.m2_2.powi(3)
                                        * (-3. * param.s1.powi(2)
                                            + 2. * param.s12.powi(2)
                                            + param.s12 * (param.s1 + -4. * param.s2)
                                            + param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                        + -6.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        + 2. * param.s12
                                            * param.s2
                                            * (param.s12.powi(2) * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12.powi(3))
                                        + param.m2_2
                                            * (2. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-5. * param.s1 + 4. * param.s2)
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
                                                    * (param.s1 + 2. * param.s2))))
                        - param.s1.powi(2)
                            * (5. * param.m2_2.powi(4) * (param.s1 + param.s2 - param.s12)
                                + 4. * param.m2_2.powi(3)
                                    * (2. * param.s1.powi(2)
                                        + 2. * param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -3. * param.s2.powi(2)
                                        + param.s12 * (-4. * param.s1 + param.s2))
                                + param.s12
                                    * param.s2.powi(2)
                                    * (-3. * param.s12.powi(2)
                                        + 2. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (3. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2)))
                                + -3.
                                    * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2))))
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.02083333333333333
            * std::f64::consts::PI
            * param.s12.powi(-3)
            * param.lambda_s12_sqrt.powi(-7)
            * ((2. * param.m2_2.powi(3) * param.s1.powi(5)
                + -15. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12
                + -6. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12
                + 60. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(2)
                + 57. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(2)
                + 6. * param.m2_2 * param.s1.powi(5) * param.s12.powi(2)
                + -20. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(3)
                + -64. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(3)
                + -15. * param.m2_2 * param.s1.powi(4) * param.s12.powi(3)
                + -2. * param.s1.powi(5) * param.s12.powi(3)
                + -30. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4)
                + -24. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(4)
                + 8. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4)
                + 9. * param.s1.powi(4) * param.s12.powi(4)
                + 3. * param.m2_2.powi(3) * param.s12.powi(5)
                + 42. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5)
                + 6. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5)
                + -16. * param.s1.powi(3) * param.s12.powi(5)
                + -5. * param.m2_2.powi(2) * param.s12.powi(6)
                + -6. * param.m2_2 * param.s1 * param.s12.powi(6)
                + 14. * param.s1.powi(2) * param.s12.powi(6)
                + param.m2_2 * param.s12.powi(7)
                + -6. * param.s1 * param.s12.powi(7)
                + param.s12.powi(8)
                + -10. * param.m2_2.powi(3) * param.s1.powi(4) * param.s2
                + 34. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12 * param.s2
                + 26. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12 * param.s2
                + -12. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(2) * param.s2
                + -86. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(2) * param.s2
                + -10. * param.m2_2 * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + 62. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2
                + 192. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(3) * param.s2
                + 142. * param.m2_2 * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + 18. * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + -14. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2
                + -58. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2
                + -108. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + -30. * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + 22. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2
                + -22. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2
                + -2. * param.m2_2 * param.s12.powi(6) * param.s2
                + 18. * param.s1 * param.s12.powi(6) * param.s2
                + -6. * param.s12.powi(7) * param.s2
                + 20. * param.m2_2.powi(3) * param.s1.powi(3) * param.s2.powi(2)
                + -12. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                + -44. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                + -24. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                + -12.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -8. * param.m2_2 * param.s1.powi(3) * param.s12.powi(2) * param.s2.powi(2)
                + 26. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(2)
                + -24. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                + -114. * param.m2_2 * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(2)
                + -16. * param.s1.powi(3) * param.s12.powi(3) * param.s2.powi(2)
                + -38. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(2)
                + 48. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + 66. * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(2)
                + -2. * param.m2_2 * param.s12.powi(5) * param.s2.powi(2)
                + 14. * param.s12.powi(6) * param.s2.powi(2)
                + -20. * param.m2_2.powi(3) * param.s1.powi(2) * param.s2.powi(3)
                + -18. * param.m2_2.powi(3) * param.s1 * param.s12 * param.s2.powi(3)
                + 36. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                + -24. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(3)
                + 54. * param.m2_2.powi(2) * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                + 24. * param.m2_2 * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(3)
                + 32. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(3)
                + -6. * param.m2_2 * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + -16. * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(3)
                + 8. * param.m2_2 * param.s12.powi(4) * param.s2.powi(3)
                + -30. * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + -16. * param.s12.powi(5) * param.s2.powi(3)
                + 10. * param.m2_2.powi(3) * param.s1 * param.s2.powi(4)
                + 11. * param.m2_2.powi(3) * param.s12 * param.s2.powi(4)
                + -14. * param.m2_2.powi(2) * param.s1 * param.s12 * param.s2.powi(4)
                + -13. * param.m2_2.powi(2) * param.s12.powi(2) * param.s2.powi(4)
                + -14. * param.m2_2 * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + -7. * param.m2_2 * param.s12.powi(3) * param.s2.powi(4)
                + 18. * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + 9. * param.s12.powi(4) * param.s2.powi(4)
                + -2. * param.m2_2.powi(3) * param.s2.powi(5)
                + 2. * param.m2_2.powi(2) * param.s12 * param.s2.powi(5)
                + 2. * param.m2_2 * param.s12.powi(2) * param.s2.powi(5)
                + -2. * param.s12.powi(3) * param.s2.powi(5)
                + 12.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (3. * param.s12.powi(2) + -2. * (param.s1 - param.s2).powi(2)
                        - param.s12 * (param.s1 + param.s2))
                + param.m1_2.powi(3)
                    * (3. * param.s12.powi(5)
                        + -2. * (param.s1 - param.s2).powi(5)
                        + -2. * param.s12.powi(4) * (7. * param.s1 + 15. * param.s2)
                        + param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (11. * param.s1 + 15. * param.s2)
                        + param.s12.powi(3)
                            * (26. * param.s1.powi(2)
                                + 62. * param.s1 * param.s2
                                + -20. * param.s2.powi(2))
                        + -12.
                            * param.s12.powi(2)
                            * (2. * param.s1.powi(3)
                                + 2. * param.s1.powi(2) * param.s2
                                + param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3)))
                + -6.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m1_2
                        * (9. * param.s12.powi(3)
                            + -2. * param.s12.powi(2) * (8. * param.s1 + -5. * param.s2)
                            + 2. * (param.s1 - param.s2).powi(3)
                            + param.s12
                                * (5. * param.s1.powi(2)
                                    + 12. * param.s1 * param.s2
                                    + -17. * param.s2.powi(2)))
                        + param.m2_2
                            * (9. * param.s12.powi(3)
                                + 2. * param.s12.powi(2) * (5. * param.s1 + -8. * param.s2)
                                + -2. * (param.s1 - param.s2).powi(3)
                                + param.s12
                                    * (-17. * param.s1.powi(2)
                                        + 12. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2)))
                        + param.s12
                            * (-5. * param.s12.powi(3)
                                + 4. * param.s12.powi(2) * (param.s1 + param.s2)
                                + -6. * (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (7. * param.s1.powi(2)
                                        + -26. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2))))
                + param.m1_2
                    * (param.m2_2.powi(2)
                        * (-21. * param.s12.powi(5)
                            + -6. * (param.s1 - param.s2).powi(5)
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (41. * param.s1 + 37. * param.s2)
                            + param.s12.powi(4) * (-26. * param.s1 + 82. * param.s2)
                            + 2. * param.s12.powi(3)
                                * (72. * param.s1.powi(2)
                                    + -43. * param.s1 * param.s2
                                    + -63. * param.s2.powi(2))
                            + -12.
                                * param.s12.powi(2)
                                * (11. * param.s1.powi(3)
                                    + 3. * param.s1.powi(2) * param.s2
                                    + -6. * param.s1 * param.s2.powi(2)
                                    + -8. * param.s2.powi(3)))
                        + 4. * param.m2_2
                            * param.s12
                            * (param.s12.powi(5)
                                + param.s12.powi(4) * (param.s1 + param.s2)
                                + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + -29. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (7. * param.s1.powi(3)
                                        + -24. * param.s1.powi(2) * param.s2
                                        + -24. * param.s1 * param.s2.powi(2)
                                        + 7. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (7. * param.s1.powi(2)
                                        + 22. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (param.s12.powi(5)
                                + -2. * param.s12.powi(4) * (param.s1 + 3. * param.s2)
                                + 2. * (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(2)
                                        + -4. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(3)
                                    * (param.s1.powi(2)
                                        + 11. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + 4. * param.s12.powi(2)
                                    * (2. * param.s1.powi(3)
                                        + 12. * param.s1.powi(2) * param.s2
                                        + -27. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3))
                                - param.s12
                                    * (7. * param.s1.powi(4)
                                        + 6. * param.s1.powi(3) * param.s2
                                        + 114. * param.s1.powi(2) * param.s2.powi(2)
                                        + -142. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4))))
                + -4.
                    * param.m0_2
                    * param.s12
                    * (param.m2_2
                        * param.s12
                        * (3. * param.s12.powi(4)
                            + param.s12.powi(3) * (19. * param.s1 + -8. * param.s2)
                            + (param.s1 - param.s2).powi(3) * (5. * param.s1 + param.s2)
                            + 3. * param.s1
                                * param.s12
                                * (5. * param.s1.powi(2)
                                    + 12. * param.s1 * param.s2
                                    + -17. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-42. * param.s1.powi(2)
                                    + 34. * param.s1 * param.s2
                                    + 6. * param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (-2. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 2. * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + 7. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (6. * param.s1.powi(2)
                                        + -23. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))
                                + -3.
                                    * param.s12
                                    * (2. * param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)))
                        + param.m1_2
                            * (param.s12
                                * (3. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-8. * param.s1 + 19. * param.s2)
                                    + param.s12.powi(2)
                                        * (6. * param.s1.powi(2)
                                            + 34. * param.s1 * param.s2
                                            + -42. * param.s2.powi(2))
                                    + 3. * param.s12
                                        * param.s2
                                        * (-17. * param.s1.powi(2)
                                            + 12. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1 + 5. * param.s2))
                                + param.m2_2
                                    * (-21. * param.s12.powi(4)
                                        + 2. * (param.s1 - param.s2).powi(4)
                                        + 28. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + -18.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -86. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))))
                        - param.m2_2.powi(2)
                            * (3. * param.s12.powi(4)
                                + param.s12.powi(3) * (29. * param.s1 + -10. * param.s2)
                                + (param.s1 - param.s2).powi(4)
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (2. * param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (-21. * param.s1.powi(2)
                                        + -25. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2)))
                        - param.m1_2.powi(2)
                            * (3. * param.s12.powi(4)
                                + (param.s1 - param.s2).powi(4)
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1 + 2. * param.s2)
                                + param.s12.powi(3) * (-10. * param.s1 + 29. * param.s2)
                                + param.s12.powi(2)
                                    * (12. * param.s1.powi(2)
                                        + -25. * param.s1 * param.s2
                                        + -21. * param.s2.powi(2))))
                - param.m1_2.powi(2)
                    * (param.m2_2
                        * (21. * param.s12.powi(5)
                            + -6. * (param.s1 - param.s2).powi(5)
                            + param.s12.powi(4) * (-82. * param.s1 + 26. * param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (37. * param.s1 + 41. * param.s2)
                            + 2. * param.s12.powi(3)
                                * (63. * param.s1.powi(2)
                                    + 43. * param.s1 * param.s2
                                    + -72. * param.s2.powi(2))
                            + -12.
                                * param.s12.powi(2)
                                * (8. * param.s1.powi(3)
                                    + 6. * param.s1.powi(2) * param.s2
                                    + -3. * param.s1 * param.s2.powi(2)
                                    + -11. * param.s2.powi(3)))
                        + param.s12
                            * (5. * param.s12.powi(5)
                                + -2.
                                    * (param.s1 + -3. * param.s2)
                                    * (param.s1 - param.s2).powi(4)
                                + -2. * param.s12.powi(4) * (11. * param.s1 + 21. * param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (13. * param.s1.powi(2)
                                        + -28. * param.s1 * param.s2
                                        + -57. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (38. * param.s1.powi(2)
                                        + 58. * param.s1 * param.s2
                                        + 24. * param.s2.powi(2))
                                + -8.
                                    * param.s12.powi(2)
                                    * (4. * param.s1.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 24. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3)))))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s12.powi(3)
                    * (5.
                        * param.m1_2.powi(4)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(4)
                            * param.s12
                            * (3. * param.s12.powi(2) + -2. * (param.s1 - param.s2).powi(2)
                                - param.s12 * (param.s1 + param.s2))
                        + -4.
                            * param.m1_2.powi(3)
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
                        + -2.
                            * param.m0_2.powi(3)
                            * (param.m1_2
                                * (3. * param.s12.powi(3)
                                    + (param.s1 - param.s2).powi(3)
                                    + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                    + param.s12
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2)))
                                + 2. * param.s12
                                    * (param.s12.powi(2) * (param.s1 + param.s2)
                                        + param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12.powi(3))
                                + param.m2_2
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + 2. * param.m1_2
                            * param.s1
                            * (4.
                                * param.m2_2.powi(3)
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                + -3.
                                    * param.m2_2.powi(2)
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
                                + 6. * param.m2_2
                                    * param.s2
                                    * (param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + param.s2))
                                - param.s2.powi(2)
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)))
                        + 3. * param.m1_2.powi(2)
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
                        + param.m0_2.powi(2)
                            * (3.
                                * param.m1_2.powi(2)
                                * (param.s12.powi(3)
                                    + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2) * (param.s1 + 3. * param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                        + param.s12
                                            * (-3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1 + param.s2))
                                + -2.
                                    * param.m2_2
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
                                + param.s12
                                    * (param.s12.powi(4)
                                        + param.s12.powi(2)
                                            * (-3. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + -2.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (5. * param.s1.powi(3)
                                                + -11. * param.s1.powi(2) * param.s2
                                                + -11. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        - param.s12.powi(3) * (param.s1 + param.s2))
                                + 2. * param.m1_2
                                    * (-2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (5. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1 + 2. * param.s2)
                                        + param.s12.powi(2)
                                            * (-3. * param.s1.powi(2)
                                                + -11. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + 6. * param.m2_2
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        - param.s12
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + 11. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))))
                        + -2.
                            * param.m0_2
                            * (2.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -3. * param.s2.powi(2)
                                    + param.s12 * (-4. * param.s1 + param.s2))
                                + 3. * param.m1_2.powi(2)
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
                                                - param.s12.powi(3)))
                                + param.m1_2
                                    * (3.
                                        * param.m2_2.powi(2)
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
                                                + param.s12.powi(3)
                                                    * (4. * param.s1 + -5. * param.s2)
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
                                                        + param.s2.powi(3))))
                                + param.s1
                                    * (2.
                                        * param.m2_2.powi(3)
                                        * (-3. * param.s1.powi(2)
                                            + 2. * param.s12.powi(2)
                                            + param.s12 * (param.s1 + -4. * param.s2)
                                            + param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                        + -6.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(3)
                                                + (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2))
                                        + 2. * param.s12
                                            * param.s2
                                            * (param.s12.powi(2) * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1.powi(2)
                                                        + -4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                - param.s12.powi(3))
                                        + param.m2_2
                                            * (2. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-5. * param.s1 + 4. * param.s2)
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
                                                    * (param.s1 + 2. * param.s2))))
                        - param.s1.powi(2)
                            * (5. * param.m2_2.powi(4) * (param.s1 + param.s2 - param.s12)
                                + 4. * param.m2_2.powi(3)
                                    * (2. * param.s1.powi(2)
                                        + 2. * param.s12.powi(2)
                                        + param.s1 * param.s2
                                        + -3. * param.s2.powi(2)
                                        + param.s12 * (-4. * param.s1 + param.s2))
                                + param.s12
                                    * param.s2.powi(2)
                                    * (-3. * param.s12.powi(2)
                                        + 2. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (3. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2) * (-5. * param.s1 + 3. * param.s2)
                                        + param.s12
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2)))
                                + -3.
                                    * param.m2_2.powi(2)
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1 + 3. * param.s2))))
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
