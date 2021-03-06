use super::{log_diff, Parameters};

pub(crate) fn c122(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.002777777777777778
            * std::f64::consts::PI
            * param.s1.powi(-3)
            * param.lambda_s12_sqrt.powi(-11)
            * (60.
                * param.s1.powi(3)
                * (14.
                    * param.m1_2.powi(6)
                    * param.s2.powi(3)
                    * (2. * param.s1.powi(2)
                        + 2. * param.s12.powi(2)
                        + 5. * param.s1 * param.s2
                        + 2. * param.s2.powi(2)
                        + -4. * param.s12 * (param.s1 + param.s2))
                    + param.m0_2.powi(6)
                        * param.s12
                        * (10. * param.s12.powi(4)
                            + 3. * (param.s1 - param.s2).powi(4)
                            + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                            + 9. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (param.s1 + param.s2)
                            + param.s12.powi(2)
                                * (-17. * param.s1.powi(2)
                                    + 40. * param.s1 * param.s2
                                    + -17. * param.s2.powi(2)))
                    + 21.
                        * param.m1_2.powi(5)
                        * param.s2.powi(2)
                        * (param.s2
                            * (-5. * param.s1.powi(3)
                                + -3. * param.s12.powi(3)
                                + -5. * param.s1.powi(2) * param.s2
                                + 7. * param.s1 * param.s2.powi(2)
                                + 3. * param.s2.powi(3)
                                + param.s12.powi(2) * (param.s1 + 9. * param.s2)
                                + param.s12
                                    * (7. * param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2)))
                            + 3. * param.m2_2
                                * (param.s12.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3)))
                    + 15.
                        * param.m1_2.powi(4)
                        * param.s2
                        * (-3.
                            * param.m2_2
                            * param.s2
                            * (-5. * param.s1.powi(4)
                                + 2. * param.s12.powi(4)
                                + -15. * param.s1.powi(3) * param.s2
                                + 5. * param.s1.powi(2) * param.s2.powi(2)
                                + 13. * param.s1 * param.s2.powi(3)
                                + 2. * param.s2.powi(4)
                                + param.s12.powi(2)
                                    * (-9. * param.s1.powi(2)
                                        + 15. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2))
                                + param.s12
                                    * (13. * param.s1.powi(3)
                                        + 8. * param.s1.powi(2) * param.s2
                                        + -27. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))
                                - param.s12.powi(3) * (param.s1 + 8. * param.s2))
                            + 3. * param.m2_2.powi(2)
                                * (param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + 10. * param.s1.powi(3) * param.s2
                                    + 20. * param.s1.powi(2) * param.s2.powi(2)
                                    + 10. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (param.s1.powi(3)
                                            + 6. * param.s1.powi(2) * param.s2
                                            + 6. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + param.s2.powi(2)
                                * (3. * param.s12.powi(4)
                                    + 3. * param.s12.powi(3) * (3. * param.s1 + -4. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (10. * param.s1.powi(2)
                                            + 15. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-17. * param.s1.powi(2)
                                            + -9. * param.s1 * param.s2
                                            + 18. * param.s2.powi(2))
                                    - param.s12
                                        * (5. * param.s1.powi(3)
                                            + -40. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + 12. * param.s2.powi(3))))
                    + -3.
                        * param.m0_2.powi(5)
                        * (3.
                            * param.s12
                            * (-2. * param.s12.powi(5)
                                + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + 10. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + -16. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-7. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + 9. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3)))
                            + param.m1_2
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (-2. * param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + -35. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(3)
                                            + -63. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (7. * param.s1 + 13. * param.s2)
                                    - (param.s1 - param.s2).powi(5))
                            + param.m2_2
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (param.s1 + -2. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (13. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(3)
                                        * (-35. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 45. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + 17. * param.s2.powi(3))))
                    + 10.
                        * param.m1_2.powi(3)
                        * (-3.
                            * param.m2_2
                            * param.s2.powi(2)
                            * (param.s12.powi(4) * (-7. * param.s1 + 5. * param.s2)
                                + param.s12.powi(3)
                                    * (17. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + -10. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (10. * param.s1.powi(3)
                                        + 30. * param.s1.powi(2) * param.s2
                                        + 15. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (param.s1.powi(3)
                                        + -63. * param.s1.powi(2) * param.s2
                                        + 18. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3))
                                + param.s12
                                    * (-20. * param.s1.powi(4)
                                        + 40. * param.s1.powi(3) * param.s2
                                        + 45. * param.s1.powi(2) * param.s2.powi(2)
                                        + -32. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                - param.s12.powi(5))
                            + param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + -25. * param.s1.powi(4) * param.s2
                                    + -100. * param.s1.powi(3) * param.s2.powi(2)
                                    + -100. * param.s1.powi(2) * param.s2.powi(3)
                                    + -25. * param.s1 * param.s2.powi(4)
                                    + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + 10.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -10.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + 5. * param.s12
                                        * (param.s1.powi(4)
                                            + 16. * param.s1.powi(3) * param.s2
                                            + 36. * param.s1.powi(2) * param.s2.powi(2)
                                            + 16. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4))
                                    - param.s2.powi(5)
                                    - param.s1.powi(5))
                            + param.s2.powi(3)
                                * (param.s12.powi(4) * (-13. * param.s1 + 5. * param.s2)
                                    + param.s12.powi(2)
                                        * (35. * param.s1.powi(3)
                                            + -45. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12
                                        * (10. * param.s1.powi(4)
                                            + 40. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3)
                                        * (10. * param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(5))
                            + -3.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (5. * param.s1.powi(5)
                                    + param.s12.powi(5)
                                    + param.s12.powi(4) * (param.s1 + -5. * param.s2)
                                    + 35. * param.s1.powi(4) * param.s2
                                    + 20. * param.s1.powi(3) * param.s2.powi(2)
                                    + -40. * param.s1.powi(2) * param.s2.powi(3)
                                    + -19. * param.s1 * param.s2.powi(4)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (7. * param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (13. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -27. * param.s1 * param.s2.powi(2)
                                            + -5. * param.s2.powi(3))
                                    + param.s12
                                        * (-19. * param.s1.powi(4)
                                            + -64. * param.s1.powi(3) * param.s2
                                            + 36. * param.s1.powi(2) * param.s2.powi(2)
                                            + 56. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s2.powi(5)))
                    + 15.
                        * param.m1_2.powi(2)
                        * param.s1
                        * (3.
                            * param.m2_2.powi(4)
                            * (param.s1.powi(4)
                                + param.s12.powi(4)
                                + 10. * param.s1.powi(3) * param.s2
                                + 20. * param.s1.powi(2) * param.s2.powi(2)
                                + 10. * param.s1 * param.s2.powi(3)
                                + param.s2.powi(4)
                                + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 6. * param.s12.powi(2)
                                    * (param.s1.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (param.s1.powi(3)
                                        + 6. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)))
                            + param.s2.powi(3)
                                * (2. * param.s12.powi(5)
                                    + param.s12.powi(4) * (8. * param.s1 + -7. * param.s2)
                                    + (param.s1 - param.s2).powi(4) * (2. * param.s1 + param.s2)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + 2. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (4. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                            - param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + -18. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + -2.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + -19. * param.s1.powi(4) * param.s2
                                    + -40. * param.s1.powi(3) * param.s2.powi(2)
                                    + 20. * param.s1.powi(2) * param.s2.powi(3)
                                    + 35. * param.s1 * param.s2.powi(4)
                                    + 5. * param.s2.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + 27. * param.s1.powi(2) * param.s2
                                            + -9. * param.s1 * param.s2.powi(2)
                                            + -13. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 56. * param.s1.powi(3) * param.s2
                                            + 36. * param.s1.powi(2) * param.s2.powi(2)
                                            + -64. * param.s1 * param.s2.powi(3)
                                            + -19. * param.s2.powi(4))
                                    - param.s1.powi(5))
                            + -6.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                    + param.s12.powi(3)
                                        * (-7. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 2. * param.s2.powi(3))
                                    + param.s12
                                        * (2. * param.s1.powi(4)
                                            + -16. * param.s1.powi(3) * param.s2
                                            + 9. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + -3. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (2. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2)))
                            + 6. * param.m2_2.powi(2)
                                * param.s2
                                * (param.s12.powi(5)
                                    + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + 2. * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(3)
                                            + 6. * param.s1.powi(2) * param.s2
                                            + 6. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + 2. * param.s12.powi(2)
                                        * (4. * param.s1.powi(3)
                                            + -9. * param.s1.powi(2) * param.s2
                                            + -9. * param.s1 * param.s2.powi(2)
                                            + 4. * param.s2.powi(3))
                                    - param.s12
                                        * (7. * param.s1.powi(4)
                                            + 4. * param.s1.powi(3) * param.s2
                                            + -36. * param.s1.powi(2) * param.s2.powi(2)
                                            + 4. * param.s1 * param.s2.powi(3)
                                            + 7. * param.s2.powi(4))))
                    + param.s1.powi(3)
                        * (14.
                            * param.m2_2.powi(6)
                            * (2. * param.s1.powi(2)
                                + 2. * param.s12.powi(2)
                                + 5. * param.s1 * param.s2
                                + 2. * param.s2.powi(2)
                                + -4. * param.s12 * (param.s1 + param.s2))
                            + param.s12
                                * param.s2.powi(3)
                                * (10. * param.s12.powi(4)
                                    + 3. * (param.s1 - param.s2).powi(4)
                                    + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 9. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (-17. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2)))
                            + -21.
                                * param.m2_2.powi(5)
                                * (-3. * param.s1.powi(3)
                                    + 3. * param.s12.powi(3)
                                    + -7. * param.s1.powi(2) * param.s2
                                    + 5. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3)
                                    + param.s12
                                        * (9. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    - param.s12.powi(2) * (9. * param.s1 + param.s2))
                            + -3.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (-2. * param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + -35. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(3)
                                            + -63. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (7. * param.s1 + 13. * param.s2)
                                    - (param.s1 - param.s2).powi(5))
                            + 15.
                                * param.m2_2.powi(4)
                                * (3. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-12. * param.s1 + 9. * param.s2)
                                    + param.s12.powi(2)
                                        * (18. * param.s1.powi(2)
                                            + -9. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(2)
                                            + 15. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    - param.s12
                                        * (12. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -40. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                            + 15.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (2. * param.s12.powi(5)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + 2. * param.s2)
                                    + param.s12.powi(4) * (-7. * param.s1 + 8. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (4. * param.s1.powi(2)
                                            + -2. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + -6. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                            + -10.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + 13. * param.s2)
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-10. * param.s1.powi(3)
                                            + 18. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 8. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 40. * param.s1 * param.s2.powi(3)
                                            + 10. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))))
                    + -3.
                        * param.m1_2
                        * param.s1.powi(2)
                        * (-21.
                            * param.m2_2.powi(5)
                            * (param.s12.powi(3)
                                + -5. * param.s1.powi(2) * param.s2
                                + -5. * param.s1 * param.s2.powi(2)
                                + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                - param.s2.powi(3)
                                - param.s1.powi(3))
                            + 15.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (-2. * param.s12.powi(5)
                                    + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + -16. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-7. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -7. * param.s2.powi(3)))
                            + 15.
                                * param.m2_2.powi(4)
                                * (2. * param.s1.powi(4)
                                    + 2. * param.s12.powi(4)
                                    + 13. * param.s1.powi(3) * param.s2
                                    + 5. * param.s1.powi(2) * param.s2.powi(2)
                                    + -15. * param.s1 * param.s2.powi(3)
                                    + -5. * param.s2.powi(4)
                                    + 3. * param.s12.powi(2)
                                        * (4. * param.s1.powi(2)
                                            + 5. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    + param.s12
                                        * (-8. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + 8. * param.s1 * param.s2.powi(2)
                                            + 13. * param.s2.powi(3))
                                    - param.s12.powi(3) * (8. * param.s1 + param.s2))
                            + param.s2.powi(3)
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (param.s1 + -2. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (13. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(3)
                                        * (-35. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 45. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + 17. * param.s2.powi(3)))
                            + 30.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (2. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (2. * param.s1.powi(3)
                                            + -18. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3))
                                    + param.s12
                                        * (-3. * param.s1.powi(4)
                                            + 8. * param.s1.powi(3) * param.s2
                                            + 9. * param.s1.powi(2) * param.s2.powi(2)
                                            + -16. * param.s1 * param.s2.powi(3)
                                            + 2. * param.s2.powi(4)))
                            + -10.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 32. * param.s1.powi(3) * param.s2
                                            + -45. * param.s1.powi(2) * param.s2.powi(2)
                                            + -40. * param.s1 * param.s2.powi(3)
                                            + 20. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(3)
                                            + 15. * param.s1.powi(2) * param.s2
                                            + 30. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12.powi(2)
                                        * (10. * param.s1.powi(3)
                                            + 18. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))))
                    + 3. * param.m0_2.powi(4)
                        * (5.
                            * param.m2_2.powi(2)
                            * (2. * param.s12.powi(5)
                                + param.s12.powi(4) * (8. * param.s1 + -7. * param.s2)
                                + (param.s1 - param.s2).powi(4) * (2. * param.s1 + param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 2. * param.s1 * param.s2
                                        + -4. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (4. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                        - param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -18. * param.s1.powi(2) * param.s2
                                        + 9. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)))
                            + 5. * param.m1_2.powi(2)
                                * (2. * param.s12.powi(5)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + 2. * param.s2)
                                    + param.s12.powi(4) * (-7. * param.s1 + 8. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (4. * param.s1.powi(2)
                                            + -2. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + -6. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                            + 3. * param.s12
                                * (param.s12.powi(6)
                                    + param.s12.powi(4)
                                        * (-5. * param.s1.powi(2)
                                            + 18. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(3)
                                            + -17. * param.s1.powi(2) * param.s2
                                            + -17. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + 17. * param.s1.powi(3) * param.s2
                                            + -54. * param.s1.powi(2) * param.s2.powi(2)
                                            + 17. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(3)
                                            + -16. * param.s1.powi(2) * param.s2
                                            + -16. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    - param.s12.powi(5) * (param.s1 + param.s2))
                            + param.m1_2
                                * (-12. * param.s12.powi(6)
                                    + 2. * param.s12.powi(5) * (16. * param.s1 + -9. * param.s2)
                                    + -2.
                                        * param.s12.powi(4)
                                        * (5. * param.s1.powi(2)
                                            + 53. * param.s1 * param.s2
                                            + -45. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (40. * param.s1.powi(4)
                                            + -81. * param.s1.powi(3) * param.s2
                                            + -153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 239. * param.s1 * param.s2.powi(3)
                                            + -45. * param.s2.powi(4))
                                    + 15.
                                        * param.m2_2
                                        * (2. * param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (-5. * param.s1.powi(2)
                                                    + 16. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (7. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 10. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(4)
                                                * (param.s1 + param.s2))
                                    - param.s12.powi(3)
                                        * (40. * param.s1.powi(3)
                                            + -239. * param.s1.powi(2) * param.s2
                                            + 136. * param.s1 * param.s2.powi(2)
                                            + 45. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (8. * param.s1.powi(2)
                                            + 65. * param.s1 * param.s2
                                            + 27. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(5)
                                        * (2. * param.s1 + 3. * param.s2))
                            - param.m2_2
                                * (12. * param.s12.powi(6)
                                    + 2. * param.s12.powi(5) * (9. * param.s1 + -16. * param.s2)
                                    + param.s12.powi(4)
                                        * (-90. * param.s1.powi(2)
                                            + 106. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (45. * param.s1.powi(3)
                                            + 136. * param.s1.powi(2) * param.s2
                                            + -239. * param.s1 * param.s2.powi(2)
                                            + 40. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + -239. * param.s1.powi(3) * param.s2
                                            + 153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 81. * param.s1 * param.s2.powi(3)
                                            + -40. * param.s2.powi(4))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (27. * param.s1.powi(2)
                                            + 65. * param.s1 * param.s2
                                            + 8. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(5)
                                        * (3. * param.s1 + 2. * param.s2)))
                    + 3. * param.m0_2.powi(2)
                        * (5.
                            * param.m1_2.powi(4)
                            * param.s2
                            * (3. * param.s12.powi(4)
                                + param.s12.powi(3) * (-12. * param.s1 + 9. * param.s2)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + -9. * param.s1 * param.s2
                                        + -17. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 15. * param.s1 * param.s2
                                        + 10. * param.s2.powi(2))
                                - param.s12
                                    * (12. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + -40. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)))
                            + 10.
                                * param.m1_2.powi(3)
                                * (-3.
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (2. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (2. * param.s1.powi(3)
                                                + -18. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + param.s12
                                            * (-3. * param.s1.powi(4)
                                                + 8. * param.s1.powi(3) * param.s2
                                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                + -16. * param.s1 * param.s2.powi(3)
                                                + 2. * param.s2.powi(4)))
                                    + param.m2_2
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1 + 7. * param.s2)
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + -17. * param.s2.powi(2))
                                            + param.s12
                                                * (5. * param.s1.powi(4)
                                                    + 32. * param.s1.powi(3) * param.s2
                                                    + -45.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -40. * param.s1 * param.s2.powi(3)
                                                    + 20. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 15. * param.s1.powi(2) * param.s2
                                                    + 30. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (10. * param.s1.powi(3)
                                                    + 18. * param.s1.powi(2) * param.s2
                                                    + -63. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))))
                            + param.s1
                                * (5.
                                    * param.m2_2.powi(4)
                                    * (3. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3)
                                            * (3. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (10. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-17. * param.s1.powi(2)
                                                + -9. * param.s1 * param.s2
                                                + 18. * param.s2.powi(2))
                                        - param.s12
                                            * (5. * param.s1.powi(3)
                                                + -40. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 12. * param.s2.powi(3)))
                                    + -30.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3))
                                            + param.s12
                                                * (2. * param.s1.powi(4)
                                                    + -16. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + 8. * param.s1 * param.s2.powi(3)
                                                    + -3. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + 18.
                                        * param.m2_2.powi(2)
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    + 3. * param.s12
                                        * param.s2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    - param.m2_2
                                        * (3. * param.s12.powi(7)
                                            + param.s12.powi(6)
                                                * (-9. * param.s1 + 15. * param.s2)
                                            + param.s12.powi(5)
                                                * (param.s1.powi(2)
                                                    + 72. * param.s1 * param.s2
                                                    + -63. * param.s2.powi(2))
                                            + param.s12.powi(4)
                                                * (25. * param.s1.powi(3)
                                                    + -239. * param.s1.powi(2) * param.s2
                                                    + 153. * param.s1 * param.s2.powi(2)
                                                    + 45. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (-35. * param.s1.powi(4)
                                                    + 136. * param.s1.powi(3) * param.s2
                                                    + 298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -432. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (17. * param.s1.powi(5)
                                                    + 81. * param.s1.powi(4) * param.s2
                                                    + -486.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 153. * param.s1 * param.s2.powi(4)
                                                    + -63. * param.s2.powi(5))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(3)
                                                    + 67. * param.s1.powi(2) * param.s2
                                                    + 117. * param.s1 * param.s2.powi(2)
                                                    + 15. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(5)
                                                * (param.s1.powi(2)
                                                    + 6. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))))
                            + 6. * param.m1_2.powi(2)
                                * (3.
                                    * param.s2
                                    * (param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (-5. * param.s1.powi(2)
                                                + 18. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(3)
                                                + -17. * param.s1.powi(2) * param.s2
                                                + -17. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (5. * param.s1.powi(4)
                                                + 17. * param.s1.powi(3) * param.s2
                                                + -54. * param.s1.powi(2) * param.s2.powi(2)
                                                + 17. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + -16. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - param.s12.powi(5) * (param.s1 + param.s2))
                                    + 5. * param.m2_2.powi(2)
                                        * (param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 6. * param.s1.powi(2) * param.s2
                                                    + 6. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + 2. * param.s12.powi(2)
                                                * (4. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12
                                                * (7. * param.s1.powi(4)
                                                    + 4. * param.s1.powi(3) * param.s2
                                                    + -36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 4. * param.s1 * param.s2.powi(3)
                                                    + 7. * param.s2.powi(4)))
                                    - param.m2_2
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-7. * param.s1 + 8. * param.s2)
                                            + param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + -40. * param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 18. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -144. * param.s1.powi(2) * param.s2
                                                    + 81. * param.s1 * param.s2.powi(2)
                                                    + 40. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-20. * param.s1.powi(4)
                                                    + 106. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -239. * param.s1 * param.s2.powi(3)
                                                    + 10. * param.s2.powi(4))
                                            + param.s12
                                                * (13. * param.s1.powi(5)
                                                    + 16. * param.s1.powi(4) * param.s2
                                                    + -239.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 136.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 106. * param.s1 * param.s2.powi(4)
                                                    + -32. * param.s2.powi(5))))
                            + param.m1_2
                                * (10.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (7. * param.s1 + -5. * param.s2)
                                        + param.s12.powi(3)
                                            * (-17. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        + param.s12
                                            * (20. * param.s1.powi(4)
                                                + -40. * param.s1.powi(3) * param.s2
                                                + -45. * param.s1.powi(2) * param.s2.powi(2)
                                                + 32. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -63. * param.s1.powi(2) * param.s2
                                                + 18. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (10. * param.s1.powi(3)
                                                + 30. * param.s1.powi(2) * param.s2
                                                + 15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                    + param.s2
                                        * (-3. * param.s12.powi(7)
                                            + param.s12.powi(6)
                                                * (-15. * param.s1 + 9. * param.s2)
                                            + param.s12.powi(5)
                                                * (63. * param.s1.powi(2)
                                                    + -72. * param.s1 * param.s2
                                                    - param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (-45. * param.s1.powi(4)
                                                    + 432. * param.s1.powi(3) * param.s2
                                                    + -298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -136. * param.s1 * param.s2.powi(3)
                                                    + 35. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (63. * param.s1.powi(5)
                                                    + -153. * param.s1.powi(4) * param.s2
                                                    + -298.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 486.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -81. * param.s1 * param.s2.powi(4)
                                                    + -17. * param.s2.powi(5))
                                            - param.s12.powi(4)
                                                * (45. * param.s1.powi(3)
                                                    + 153. * param.s1.powi(2) * param.s2
                                                    + -239. * param.s1 * param.s2.powi(2)
                                                    + 25. * param.s2.powi(3))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (15. * param.s1.powi(3)
                                                    + 117. * param.s1.powi(2) * param.s2
                                                    + 67. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(5)
                                                * (3. * param.s1.powi(2)
                                                    + 6. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + 3. * param.m2_2
                                        * (param.s12.powi(7)
                                            + param.s12.powi(6) * (param.s1 + param.s2)
                                            + param.s12.powi(5)
                                                * (-17. * param.s1.powi(2)
                                                    + 64. * param.s1 * param.s2
                                                    + -17. * param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + param.s12.powi(4)
                                                * (35. * param.s1.powi(3)
                                                    + -81. * param.s1.powi(2) * param.s2
                                                    + -81. * param.s1 * param.s2.powi(2)
                                                    + 35. * param.s2.powi(3))
                                            + 9. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(4)
                                                    + -6. * param.s1.powi(3) * param.s2
                                                    + -30.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -6. * param.s1 * param.s2.powi(3)
                                                    + param.s2.powi(4))
                                            - param.s12.powi(2)
                                                * (param.s1.powi(5)
                                                    + -239. * param.s1.powi(4) * param.s2
                                                    + 298.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -239. * param.s1 * param.s2.powi(4)
                                                    + param.s2.powi(5))
                                            - param.s12.powi(3)
                                                * (25. * param.s1.powi(4)
                                                    + 136. * param.s1.powi(3) * param.s2
                                                    + -486.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 136. * param.s1 * param.s2.powi(3)
                                                    + 25. * param.s2.powi(4)))
                                    + -6.
                                        * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (8. * param.s1 + -7. * param.s2)
                                            + param.s12.powi(4)
                                                * (-40. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(3)
                                                * (4. * param.s1.powi(3)
                                                    + 18. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (40. * param.s1.powi(3)
                                                    + 81. * param.s1.powi(2) * param.s2
                                                    + -144. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (10. * param.s1.powi(4)
                                                    + -239. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 106. * param.s1 * param.s2.powi(3)
                                                    + -20. * param.s2.powi(4))
                                            + param.s12
                                                * (-32. * param.s1.powi(5)
                                                    + 106. * param.s1.powi(4) * param.s2
                                                    + 136.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -239.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 16. * param.s1 * param.s2.powi(4)
                                                    + 13. * param.s2.powi(5)))))
                    + -3.
                        * param.m0_2
                        * (7.
                            * param.m1_2.powi(5)
                            * param.s2.powi(2)
                            * (-3. * param.s1.powi(3)
                                + 3. * param.s12.powi(3)
                                + -7. * param.s1.powi(2) * param.s2
                                + 5. * param.s1 * param.s2.powi(2)
                                + 5. * param.s2.powi(3)
                                + param.s12
                                    * (9. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + -7. * param.s2.powi(2))
                                - param.s12.powi(2) * (9. * param.s1 + param.s2))
                            + 5. * param.m1_2.powi(4)
                                * param.s2
                                * (param.s2
                                    * (-9. * param.s12.powi(4)
                                        + 15. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 4. * (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -64. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + param.s12
                                            * (-27. * param.s1.powi(3)
                                                + 41. * param.s1.powi(2) * param.s2
                                                + 41. * param.s1 * param.s2.powi(2)
                                                + -27. * param.s2.powi(3)))
                                    + 3. * param.m2_2
                                        * (2. * param.s1.powi(4)
                                            + 2. * param.s12.powi(4)
                                            + 13. * param.s1.powi(3) * param.s2
                                            + 5. * param.s1.powi(2) * param.s2.powi(2)
                                            + -15. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4)
                                            + 3. * param.s12.powi(2)
                                                * (4. * param.s1.powi(2)
                                                    + 5. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2))
                                            + param.s12
                                                * (-8. * param.s1.powi(3)
                                                    + -27. * param.s1.powi(2) * param.s2
                                                    + 8. * param.s1 * param.s2.powi(2)
                                                    + 13. * param.s2.powi(3))
                                            - param.s12.powi(3) * (8. * param.s1 + param.s2)))
                            + 10.
                                * param.m1_2.powi(3)
                                * (param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + -19. * param.s1.powi(4) * param.s2
                                        + -40. * param.s1.powi(3) * param.s2.powi(2)
                                        + 20. * param.s1.powi(2) * param.s2.powi(3)
                                        + 35. * param.s1 * param.s2.powi(4)
                                        + 5. * param.s2.powi(5)
                                        + param.s12.powi(4) * (-5. * param.s1 + param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 27. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + -13. * param.s2.powi(3))
                                        + param.s12
                                            * (5. * param.s1.powi(4)
                                                + 56. * param.s1.powi(3) * param.s2
                                                + 36. * param.s1.powi(2) * param.s2.powi(2)
                                                + -64. * param.s1 * param.s2.powi(3)
                                                + -19. * param.s2.powi(4))
                                        - param.s1.powi(5))
                                    + 3. * param.s2.powi(2)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3))
                                            + param.s12
                                                * (2. * param.s1.powi(4)
                                                    + -16. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + 8. * param.s1 * param.s2.powi(3)
                                                    + -3. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + -4.
                                        * param.m2_2
                                        * param.s2
                                        * (param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 6. * param.s1.powi(2) * param.s2
                                                    + 6. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + 2. * param.s12.powi(2)
                                                * (4. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12
                                                * (7. * param.s1.powi(4)
                                                    + 4. * param.s1.powi(3) * param.s2
                                                    + -36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 4. * param.s1 * param.s2.powi(3)
                                                    + 7. * param.s2.powi(4))))
                            + param.s1.powi(2)
                                * (7.
                                    * param.m2_2.powi(5)
                                    * (5. * param.s1.powi(3)
                                        + 3. * param.s12.powi(3)
                                        + 5. * param.s1.powi(2) * param.s2
                                        + -7. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s2.powi(3)
                                        + param.s12
                                            * (-7. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + 9. * param.s2))
                                    + 3. * param.s12
                                        * param.s2.powi(2)
                                        * (-2. * param.s12.powi(5)
                                            + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1 + param.s2)
                                            + param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 10. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (5. * param.s1.powi(2)
                                                    + -16. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (-7. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + -7. * param.s2.powi(3)))
                                    + -5.
                                        * param.m2_2.powi(4)
                                        * (9. * param.s12.powi(4)
                                            + -15. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + param.s12.powi(2)
                                                * (-9. * param.s1.powi(2)
                                                    + 64. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + -4.
                                                * (param.s1 - param.s2).powi(2)
                                                * (3. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + param.s12
                                                * (27. * param.s1.powi(3)
                                                    + -41. * param.s1.powi(2) * param.s2
                                                    + -41. * param.s1 * param.s2.powi(2)
                                                    + 27. * param.s2.powi(3)))
                                    + 2. * param.m2_2.powi(2)
                                        * (-3. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (13. * param.s1 + -27. * param.s2)
                                            + 2. * (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + param.s12.powi(4)
                                                * (-20. * param.s1.powi(2)
                                                    + 16. * param.s1 * param.s2
                                                    + 45. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + 106. * param.s1.powi(2) * param.s2
                                                    + -239. * param.s1 * param.s2.powi(2)
                                                    + 45. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + -144. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 136. * param.s1 * param.s2.powi(3)
                                                    + -90. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (7. * param.s1.powi(3)
                                                    + -27. * param.s1.powi(2) * param.s2
                                                    + -142. * param.s1 * param.s2.powi(2)
                                                    + -18. * param.s2.powi(3)))
                                    + 30.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + -7. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (2. * param.s1.powi(3)
                                                    + -18. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12
                                                * (-3. * param.s1.powi(4)
                                                    + 8. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + -16. * param.s1 * param.s2.powi(3)
                                                    + 2. * param.s2.powi(4)))
                                    + param.m2_2
                                        * param.s2
                                        * (12. * param.s12.powi(6)
                                            + (param.s1 - param.s2).powi(5)
                                                * (2. * param.s1 + 3. * param.s2)
                                            + param.s12.powi(5)
                                                * (-32. * param.s1 + 18. * param.s2)
                                            + 2. * param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + 53. * param.s1 * param.s2
                                                    + -45. * param.s2.powi(2))
                                            + param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (8. * param.s1.powi(2)
                                                    + 65. * param.s1 * param.s2
                                                    + 27. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (40. * param.s1.powi(3)
                                                    + -239. * param.s1.powi(2) * param.s2
                                                    + 136. * param.s1 * param.s2.powi(2)
                                                    + 45. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-40. * param.s1.powi(4)
                                                    + 81. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -239. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))))
                            + param.m1_2
                                * param.s1
                                * (15.
                                    * param.m2_2.powi(4)
                                    * (-5. * param.s1.powi(4)
                                        + 2. * param.s12.powi(4)
                                        + -15. * param.s1.powi(3) * param.s2
                                        + 5. * param.s1.powi(2) * param.s2.powi(2)
                                        + 13. * param.s1 * param.s2.powi(3)
                                        + 2. * param.s2.powi(4)
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + param.s12
                                            * (13. * param.s1.powi(3)
                                                + 8. * param.s1.powi(2) * param.s2
                                                + -27. * param.s1 * param.s2.powi(2)
                                                + -8. * param.s2.powi(3))
                                        - param.s12.powi(3) * (param.s1 + 8. * param.s2))
                                    + param.s2.powi(2)
                                        * (12. * param.s12.powi(6)
                                            + 2. * param.s12.powi(5)
                                                * (9. * param.s1 + -16. * param.s2)
                                            + param.s12.powi(4)
                                                * (-90. * param.s1.powi(2)
                                                    + 106. * param.s1 * param.s2
                                                    + 10. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (45. * param.s1.powi(3)
                                                    + 136. * param.s1.powi(2) * param.s2
                                                    + -239. * param.s1 * param.s2.powi(2)
                                                    + 40. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (45. * param.s1.powi(4)
                                                    + -239. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 81. * param.s1 * param.s2.powi(3)
                                                    + -40. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (27. * param.s1.powi(2)
                                                    + 65. * param.s1 * param.s2
                                                    + 8. * param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(5)
                                                * (3. * param.s1 + 2. * param.s2))
                                    + -24.
                                        * param.m2_2
                                        * param.s2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    + -40.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 6. * param.s1.powi(2) * param.s2
                                                    + 6. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + 2. * param.s12.powi(2)
                                                * (4. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12
                                                * (7. * param.s1.powi(4)
                                                    + 4. * param.s1.powi(3) * param.s2
                                                    + -36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 4. * param.s1 * param.s2.powi(3)
                                                    + 7. * param.s2.powi(4)))
                                    + 6. * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-7. * param.s1 + 8. * param.s2)
                                            + param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + -40. * param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 18. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -144. * param.s1.powi(2) * param.s2
                                                    + 81. * param.s1 * param.s2.powi(2)
                                                    + 40. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-20. * param.s1.powi(4)
                                                    + 106. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -239. * param.s1 * param.s2.powi(3)
                                                    + 10. * param.s2.powi(4))
                                            + param.s12
                                                * (13. * param.s1.powi(5)
                                                    + 16. * param.s1.powi(4) * param.s2
                                                    + -239.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 136.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 106. * param.s1 * param.s2.powi(4)
                                                    + -32. * param.s2.powi(5))))
                            + 2. * param.m1_2.powi(2)
                                * (param.s2.powi(2)
                                    * (-3. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-27. * param.s1 + 13. * param.s2)
                                        + param.s12.powi(4)
                                            * (45. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -20. * param.s2.powi(2))
                                        + 2. * (param.s1 - param.s2).powi(4)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (18. * param.s1.powi(3)
                                                + 142. * param.s1.powi(2) * param.s2
                                                + 27. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (45. * param.s1.powi(3)
                                                + -239. * param.s1.powi(2) * param.s2
                                                + 106. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-90. * param.s1.powi(4)
                                                + 136. * param.s1.powi(3) * param.s2
                                                + 153. * param.s1.powi(2) * param.s2.powi(2)
                                                + -144. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4)))
                                    + 5. * param.m2_2.powi(3)
                                        * (5. * param.s1.powi(5)
                                            + param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -5. * param.s2)
                                            + 35. * param.s1.powi(4) * param.s2
                                            + 20. * param.s1.powi(3) * param.s2.powi(2)
                                            + -40. * param.s1.powi(2) * param.s2.powi(3)
                                            + -19. * param.s1 * param.s2.powi(4)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (7. * param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + 2. * param.s12.powi(2)
                                                * (13. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -27. * param.s1 * param.s2.powi(2)
                                                    + -5. * param.s2.powi(3))
                                            + param.s12
                                                * (-19. * param.s1.powi(4)
                                                    + -64. * param.s1.powi(3) * param.s2
                                                    + 36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 56. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s2.powi(5))
                                    + 3. * param.m2_2
                                        * param.s2
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (8. * param.s1 + -7. * param.s2)
                                            + param.s12.powi(4)
                                                * (-40. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(3)
                                                * (4. * param.s1.powi(3)
                                                    + 18. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (40. * param.s1.powi(3)
                                                    + 81. * param.s1.powi(2) * param.s2
                                                    + -144. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (10. * param.s1.powi(4)
                                                    + -239. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 106. * param.s1 * param.s2.powi(3)
                                                    + -20. * param.s2.powi(4))
                                            + param.s12
                                                * (-32. * param.s1.powi(5)
                                                    + 106. * param.s1.powi(4) * param.s2
                                                    + 136.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -239.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 16. * param.s1 * param.s2.powi(4)
                                                    + 13. * param.s2.powi(5)))
                                    + -3.
                                        * param.m2_2.powi(2)
                                        * (param.s12.powi(6)
                                            + -2.
                                                * param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + -24. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (30. * param.s1.powi(3)
                                                    + -82. * param.s1.powi(2) * param.s2
                                                    + -82. * param.s1 * param.s2.powi(2)
                                                    + 30. * param.s2.powi(3))
                                            + -4.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(4)
                                                    + 16. * param.s1.powi(3) * param.s2
                                                    + 36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 16. * param.s1 * param.s2.powi(3)
                                                    + param.s2.powi(4))
                                            + param.s12
                                                * (19. * param.s1.powi(5)
                                                    + 123. * param.s1.powi(4) * param.s2
                                                    + -212.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -212.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 123. * param.s1 * param.s2.powi(4)
                                                    + 19. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (35. * param.s1.powi(4)
                                                    + 32. * param.s1.powi(3) * param.s2
                                                    + -324.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 32. * param.s1 * param.s2.powi(3)
                                                    + 35. * param.s2.powi(4))
                                            - param.s12.powi(5) * (param.s1 + param.s2))))
                    - param.m0_2.powi(3)
                        * (-6.
                            * param.m2_2.powi(2)
                            * (3. * param.s12.powi(6)
                                + param.s12.powi(5) * (27. * param.s1 + -13. * param.s2)
                                + -2.
                                    * (param.s1 - param.s2).powi(4)
                                    * (6. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-45. * param.s1.powi(2)
                                        + -16. * param.s1 * param.s2
                                        + 20. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (90. * param.s1.powi(4)
                                        + -136. * param.s1.powi(3) * param.s2
                                        + -153. * param.s1.powi(2) * param.s2.powi(2)
                                        + 144. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                - param.s12.powi(3)
                                    * (45. * param.s1.powi(3)
                                        + -239. * param.s1.powi(2) * param.s2
                                        + 106. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (18. * param.s1.powi(3)
                                        + 142. * param.s1.powi(2) * param.s2
                                        + 27. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3)))
                            + 10.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (13. * param.s1 + -5. * param.s2)
                                    + (param.s1 - param.s2).powi(3)
                                        * (10. * param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-35. * param.s1.powi(3)
                                            + 45. * param.s1.powi(2) * param.s2
                                            + 18. * param.s1 * param.s2.powi(2)
                                            + -10. * param.s2.powi(3))
                                    + param.s12
                                        * (10. * param.s1.powi(4)
                                            + 40. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4)))
                            + 10.
                                * param.m1_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + 13. * param.s2)
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-10. * param.s1.powi(3)
                                            + 18. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 8. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 40. * param.s1 * param.s2.powi(3)
                                            + 10. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2)))
                            + param.s12
                                * (param.s12.powi(5)
                                    * (17. * param.s1.powi(2)
                                        + -64. * param.s1 * param.s2
                                        + 17. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-35. * param.s1.powi(3)
                                            + 81. * param.s1.powi(2) * param.s2
                                            + 81. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + 3. * (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + -9.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(4)
                                            + -6. * param.s1.powi(3) * param.s2
                                            + -30. * param.s1.powi(2) * param.s2.powi(2)
                                            + -6. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4))
                                    + param.s12.powi(3)
                                        * (25. * param.s1.powi(4)
                                            + 136. * param.s1.powi(3) * param.s2
                                            + -486. * param.s1.powi(2) * param.s2.powi(2)
                                            + 136. * param.s1 * param.s2.powi(3)
                                            + 25. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(5)
                                            + -239. * param.s1.powi(4) * param.s2
                                            + 298. * param.s1.powi(3) * param.s2.powi(2)
                                            + 298. * param.s1.powi(2) * param.s2.powi(3)
                                            + -239. * param.s1 * param.s2.powi(4)
                                            + param.s2.powi(5))
                                    - param.s12.powi(6) * (param.s1 + param.s2)
                                    - param.s12.powi(7))
                            + 3. * param.m2_2
                                * (3. * param.s12.powi(7)
                                    + 3. * param.s12.powi(6) * (5. * param.s1 + -3. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                        * (3. * param.s1.powi(2)
                                            + 6. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (-63. * param.s1.powi(2)
                                            + 72. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (15. * param.s1.powi(3)
                                            + 117. * param.s1.powi(2) * param.s2
                                            + 67. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + param.s12.powi(4)
                                        * (45. * param.s1.powi(3)
                                            + 153. * param.s1.powi(2) * param.s2
                                            + -239. * param.s1 * param.s2.powi(2)
                                            + 25. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (45. * param.s1.powi(4)
                                            + -432. * param.s1.powi(3) * param.s2
                                            + 298. * param.s1.powi(2) * param.s2.powi(2)
                                            + 136. * param.s1 * param.s2.powi(3)
                                            + -35. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (-63. * param.s1.powi(5)
                                            + 153. * param.s1.powi(4) * param.s2
                                            + 298. * param.s1.powi(3) * param.s2.powi(2)
                                            + -486. * param.s1.powi(2) * param.s2.powi(3)
                                            + 81. * param.s1 * param.s2.powi(4)
                                            + 17. * param.s2.powi(5)))
                            + 6. * param.m1_2.powi(2)
                                * (-3. * param.s12.powi(6)
                                    + param.s12.powi(5) * (13. * param.s1 + -27. * param.s2)
                                    + 2. * (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-20. * param.s1.powi(2)
                                            + 16. * param.s1 * param.s2
                                            + 45. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(3)
                                            + 106. * param.s1.powi(2) * param.s2
                                            + -239. * param.s1 * param.s2.powi(2)
                                            + 45. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + -144. * param.s1.powi(3) * param.s2
                                            + 153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 136. * param.s1 * param.s2.powi(3)
                                            + -90. * param.s2.powi(4))
                                    + 15.
                                        * param.m2_2
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + -7. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (2. * param.s1.powi(3)
                                                    + -18. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12
                                                * (-3. * param.s1.powi(4)
                                                    + 8. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + -16. * param.s1 * param.s2.powi(3)
                                                    + 2. * param.s2.powi(4)))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (7. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + -142. * param.s1 * param.s2.powi(2)
                                            + -18. * param.s2.powi(3)))
                            + 3. * param.m1_2
                                * (3. * param.s12.powi(7)
                                    + param.s12.powi(6) * (-9. * param.s1 + 15. * param.s2)
                                    + param.s12.powi(5)
                                        * (param.s1.powi(2)
                                            + 72. * param.s1 * param.s2
                                            + -63. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (25. * param.s1.powi(3)
                                            + -239. * param.s1.powi(2) * param.s2
                                            + 153. * param.s1 * param.s2.powi(2)
                                            + 45. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (-35. * param.s1.powi(4)
                                            + 136. * param.s1.powi(3) * param.s2
                                            + 298. * param.s1.powi(2) * param.s2.powi(2)
                                            + -432. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(5)
                                            + 81. * param.s1.powi(4) * param.s2
                                            + -486. * param.s1.powi(3) * param.s2.powi(2)
                                            + 298. * param.s1.powi(2) * param.s2.powi(3)
                                            + 153. * param.s1 * param.s2.powi(4)
                                            + -63. * param.s2.powi(5))
                                    + 30.
                                        * param.m2_2.powi(2)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3))
                                            + param.s12
                                                * (2. * param.s1.powi(4)
                                                    + -16. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + 8. * param.s1 * param.s2.powi(3)
                                                    + -3. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + -24.
                                        * param.m2_2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(3)
                                            + 67. * param.s1.powi(2) * param.s2
                                            + 117. * param.s1 * param.s2.powi(2)
                                            + 15. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(5)
                                        * (param.s1.powi(2)
                                            + 6. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2)))))
                * log_diff(
                    param.m0_2 * (param.s1 + param.s12 - param.s2)
                        + param.m1_2 * (param.s1 + param.s2 - param.s12)
                        + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                    param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m0_2.powi(5)
                    * (param.s12.powi(7)
                        + (param.s1 - param.s2).powi(5)
                            * (10. * param.s1.powi(2)
                                + -5. * param.s1 * param.s2
                                + param.s2.powi(2))
                        + param.s12.powi(5)
                            * (165. * param.s1.powi(2)
                                + 70. * param.s1 * param.s2
                                + 21. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (150. * param.s1.powi(3)
                                + -67. * param.s1.powi(2) * param.s2
                                + -22. * param.s1 * param.s2.powi(2)
                                + -7. * param.s2.powi(3))
                        + param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (491. * param.s1.powi(3)
                                + 85. * param.s1.powi(2) * param.s2
                                + 13. * param.s1 * param.s2.powi(2)
                                + -7. * param.s2.powi(3))
                        + 5. * param.s12.powi(3)
                            * (-285. * param.s1.powi(4)
                                + 218. * param.s1.powi(3) * param.s2
                                + 12. * param.s1.powi(2) * param.s2.powi(2)
                                + 12. * param.s1 * param.s2.powi(3)
                                + 7. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (24. * param.s1.powi(5)
                                + 1465. * param.s1.powi(4) * param.s2
                                + -1668. * param.s1.powi(3) * param.s2.powi(2)
                                + 180. * param.s1.powi(2) * param.s2.powi(3)
                                + 20. * param.s1 * param.s2.powi(4)
                                + -21. * param.s2.powi(5))
                        - param.s12.powi(6) * (16. * param.s1 + 7. * param.s2))
                    + param.m1_2.powi(5)
                        * (param.s1.powi(7)
                            + -19. * param.s1.powi(6) * param.s2
                            + 261. * param.s1.powi(5) * param.s2.powi(2)
                            + 3537. * param.s1.powi(4) * param.s2.powi(3)
                            + 3537. * param.s1.powi(3) * param.s2.powi(4)
                            + 261. * param.s1.powi(2) * param.s2.powi(5)
                            + -19. * param.s1 * param.s2.powi(6)
                            + param.s2.powi(7)
                            + 7. * param.s12.powi(6) * (param.s1 + param.s2)
                            + 5. * param.s12.powi(4)
                                * (7. * param.s1.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + 7. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (7. * param.s1.powi(4)
                                    + -24. * param.s1.powi(3) * param.s2
                                    + 12. * param.s1.powi(2) * param.s2.powi(2)
                                    + -24. * param.s1 * param.s2.powi(3)
                                    + 7. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (21. * param.s1.powi(5)
                                    + -155. * param.s1.powi(4) * param.s2
                                    + 540. * param.s1.powi(3) * param.s2.powi(2)
                                    + 540. * param.s1.powi(2) * param.s2.powi(3)
                                    + -155. * param.s1 * param.s2.powi(4)
                                    + 21. * param.s2.powi(5))
                            - param.s12
                                * (7. * param.s1.powi(6)
                                    + -88. * param.s1.powi(5) * param.s2
                                    + 695. * param.s1.powi(4) * param.s2.powi(2)
                                    + 4232. * param.s1.powi(3) * param.s2.powi(3)
                                    + 695. * param.s1.powi(2) * param.s2.powi(4)
                                    + -88. * param.s1 * param.s2.powi(5)
                                    + 7. * param.s2.powi(6))
                            - param.s12.powi(5)
                                * (21. * param.s1.powi(2)
                                    + 16. * param.s1 * param.s2
                                    + 21. * param.s2.powi(2))
                            - param.s12.powi(7))
                    + param.s1.powi(5)
                        * (param.s12.powi(7)
                            + param.s12.powi(5)
                                * (21. * param.s1.powi(2)
                                    + 70. * param.s1 * param.s2
                                    + 165. * param.s2.powi(2))
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (7. * param.s1.powi(3)
                                    + -13. * param.s1.powi(2) * param.s2
                                    + -85. * param.s1 * param.s2.powi(2)
                                    + -491. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(4)
                                * (7. * param.s1.powi(3)
                                    + 22. * param.s1.powi(2) * param.s2
                                    + 67. * param.s1 * param.s2.powi(2)
                                    + -150. * param.s2.powi(3))
                            + 5. * param.s12.powi(3)
                                * (7. * param.s1.powi(4)
                                    + 12. * param.s1.powi(3) * param.s2
                                    + 12. * param.s1.powi(2) * param.s2.powi(2)
                                    + 218. * param.s1 * param.s2.powi(3)
                                    + -285. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-21. * param.s1.powi(5)
                                    + 20. * param.s1.powi(4) * param.s2
                                    + 180. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1668. * param.s1.powi(2) * param.s2.powi(3)
                                    + 1465. * param.s1 * param.s2.powi(4)
                                    + 24. * param.s2.powi(5))
                            + -840.
                                * param.m2_2.powi(5)
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + 2. * param.s2.powi(2)
                                    + -4. * param.s12 * (param.s1 + param.s2))
                            + 420.
                                * param.m2_2.powi(4)
                                * (-7. * param.s1.powi(3)
                                    + 7. * param.s12.powi(3)
                                    + -18. * param.s1.powi(2) * param.s2
                                    + 12. * param.s1 * param.s2.powi(2)
                                    + 13. * param.s2.powi(3)
                                    + param.s12
                                        * (21. * param.s1.powi(2)
                                            + 19. * param.s1 * param.s2
                                            + -19. * param.s2.powi(2))
                                    - param.s12.powi(2) * (21. * param.s1 + param.s2))
                            + -10.
                                * param.m2_2.powi(3)
                                * (137. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-548. * param.s1 + 628. * param.s2)
                                    + 2. * param.s12.powi(2)
                                        * (411. * param.s1.powi(2)
                                            + -335. * param.s1 * param.s2
                                            + -513. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (137. * param.s1.powi(2)
                                            + 860. * param.s1 * param.s2
                                            + 641. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (137. * param.s1.powi(3)
                                            + 136. * param.s1.powi(2) * param.s2
                                            + -620. * param.s1 * param.s2.powi(2)
                                            + 95. * param.s2.powi(3)))
                            + 30.
                                * param.m2_2.powi(2)
                                * (3. * param.s12.powi(5)
                                    + param.s12.powi(4) * (-15. * param.s1 + 122. * param.s2)
                                    + 5. * param.s12.powi(3)
                                        * (6. * param.s1.powi(2)
                                            + -59. * param.s1 * param.s2
                                            + 14. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-30. * param.s1.powi(3)
                                            + 153. * param.s1.powi(2) * param.s2
                                            + 425. * param.s1 * param.s2.powi(2)
                                            + -412. * param.s2.powi(3))
                                    + param.s12
                                        * (15. * param.s1.powi(4)
                                            + 91. * param.s1.powi(3) * param.s2
                                            + -620. * param.s1.powi(2) * param.s2.powi(2)
                                            + 403. * param.s1 * param.s2.powi(3)
                                            + 111. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (3. * param.s1.powi(2)
                                            + 80. * param.s1 * param.s2
                                            + 106. * param.s2.powi(2)))
                            + 3. * param.m2_2
                                * (3. * param.s12.powi(6)
                                    + -6. * param.s12.powi(5) * (3. * param.s1 + 13. * param.s2)
                                    + 5. * param.s12.powi(4)
                                        * (9. * param.s1.powi(2)
                                            + 54. * param.s1 * param.s2
                                            + -205. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(4)
                                        * (3. * param.s1.powi(2)
                                            + -30. * param.s1 * param.s2
                                            + -187. * param.s2.powi(2))
                                    + -60.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(3)
                                            + 5. * param.s1.powi(2) * param.s2
                                            + -12. * param.s1 * param.s2.powi(2)
                                            + -15. * param.s2.powi(3))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (9. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + 553. * param.s1 * param.s2.powi(2)
                                            + 499. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + 60. * param.s1.powi(3) * param.s2
                                            + 1586. * param.s1.powi(2) * param.s2.powi(2)
                                            + -3980. * param.s1 * param.s2.powi(3)
                                            + 1385. * param.s2.powi(4)))
                            - (param.s1 - param.s2).powi(5)
                                * (param.s1.powi(2)
                                    + -5. * param.s1 * param.s2
                                    + 10. * param.s2.powi(2))
                            - param.s12.powi(6) * (7. * param.s1 + 16. * param.s2))
                    + 2. * param.m1_2.powi(3)
                        * param.s1.powi(2)
                        * (-5. * param.s12.powi(7)
                            + param.s12.powi(6) * (35. * param.s1 + 53. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (35. * param.s1.powi(3)
                                    + 29. * param.s1.powi(2) * param.s2
                                    + -16. * param.s1 * param.s2.powi(2)
                                    + 134. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (35. * param.s1.powi(4)
                                    + -48. * param.s1.powi(3) * param.s2
                                    + -210. * param.s1.powi(2) * param.s2.powi(2)
                                    + 279. * param.s1 * param.s2.powi(3)
                                    + 197. * param.s2.powi(4))
                            + (param.s1 - param.s2).powi(2)
                                * (5. * param.s1.powi(5)
                                    + -67. * param.s1.powi(4) * param.s2
                                    + 599. * param.s1.powi(3) * param.s2.powi(2)
                                    + 5036. * param.s1.powi(2) * param.s2.powi(3)
                                    + 2549. * param.s1 * param.s2.powi(4)
                                    + 68. * param.s2.powi(5))
                            + param.s12.powi(2)
                                * (105. * param.s1.powi(5)
                                    + -505. * param.s1.powi(4) * param.s2
                                    + -90. * param.s1.powi(3) * param.s2.powi(2)
                                    + -8709. * param.s1.powi(2) * param.s2.powi(3)
                                    + 6020. * param.s1 * param.s2.powi(4)
                                    + 825. * param.s2.powi(5))
                            + param.s12
                                * (-35. * param.s1.powi(6)
                                    + 332. * param.s1.powi(5) * param.s2
                                    + -1360. * param.s1.powi(4) * param.s2.powi(2)
                                    + 5663. * param.s1.powi(3) * param.s2.powi(3)
                                    + 7613. * param.s1.powi(2) * param.s2.powi(4)
                                    + -6805. * param.s1 * param.s2.powi(5)
                                    + -368. * param.s2.powi(6))
                            + -15.
                                * param.m2_2.powi(2)
                                * (-3. * param.s1.powi(5)
                                    + 3. * param.s12.powi(5)
                                    + -208. * param.s1.powi(4) * param.s2
                                    + -1049. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1049. * param.s1.powi(2) * param.s2.powi(3)
                                    + -208. * param.s1 * param.s2.powi(4)
                                    + -3. * param.s2.powi(5)
                                    + -15. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (30. * param.s1.powi(2)
                                            + 253. * param.s1 * param.s2
                                            + 30. * param.s2.powi(2))
                                    + -3.
                                        * param.s12.powi(2)
                                        * (10. * param.s1.powi(3)
                                            + 223. * param.s1.powi(2) * param.s2
                                            + 223. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    + param.s12
                                        * (15. * param.s1.powi(4)
                                            + 639. * param.s1.powi(3) * param.s2
                                            + 1688. * param.s1.powi(2) * param.s2.powi(2)
                                            + 639. * param.s1 * param.s2.powi(3)
                                            + 15. * param.s2.powi(4)))
                            + -6.
                                * param.m2_2
                                * (3. * param.s1.powi(6)
                                    + 3. * param.s12.powi(6)
                                    + -87. * param.s1.powi(5) * param.s2
                                    + -1799. * param.s1.powi(4) * param.s2.powi(2)
                                    + -1479. * param.s1.powi(3) * param.s2.powi(3)
                                    + 2406. * param.s1.powi(2) * param.s2.powi(4)
                                    + 938. * param.s1 * param.s2.powi(5)
                                    + 18. * param.s2.powi(6)
                                    + -3. * param.s12.powi(5) * (6. * param.s1 + 11. * param.s2)
                                    + 15.
                                        * param.s12.powi(4)
                                        * (3. * param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + 8. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (12. * param.s1.powi(3)
                                            + -30. * param.s1.powi(2) * param.s2
                                            + 193. * param.s1 * param.s2.powi(2)
                                            + 42. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + -390. * param.s1.powi(3) * param.s2
                                            + -229. * param.s1.powi(2) * param.s2.powi(2)
                                            + 2805. * param.s1 * param.s2.powi(3)
                                            + 195. * param.s2.powi(4))
                                    - param.s12
                                        * (18. * param.s1.powi(5)
                                            + -315. * param.s1.powi(4) * param.s2
                                            + -2873. * param.s1.powi(3) * param.s2.powi(2)
                                            + 2372. * param.s1.powi(2) * param.s2.powi(3)
                                            + 2805. * param.s1 * param.s2.powi(4)
                                            + 93. * param.s2.powi(5)))
                            - param.s12.powi(5)
                                * (105. * param.s1.powi(2)
                                    + 188. * param.s1 * param.s2
                                    + 258. * param.s2.powi(2)))
                    + param.m1_2
                        * param.s1.powi(4)
                        * (-5. * param.s12.powi(7)
                            + param.s12.powi(6) * (35. * param.s1 + 71. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (35. * param.s1.powi(3)
                                    + 83. * param.s1.powi(2) * param.s2
                                    + 173. * param.s1 * param.s2.powi(2)
                                    + -135. * param.s2.powi(3))
                            + (param.s1 - param.s2).powi(4)
                                * (5. * param.s1.powi(3)
                                    + -39. * param.s1.powi(2) * param.s2
                                    + 165. * param.s1 * param.s2.powi(2)
                                    + 511. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (35. * param.s1.powi(4)
                                    + 24. * param.s1.powi(3) * param.s2
                                    + -120. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1522. * param.s1 * param.s2.powi(3)
                                    + -885. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (105. * param.s1.powi(5)
                                    + -235. * param.s1.powi(4) * param.s2
                                    + -1080. * param.s1.powi(3) * param.s2.powi(2)
                                    + 3582. * param.s1.powi(2) * param.s2.powi(3)
                                    + 4615. * param.s1 * param.s2.powi(4)
                                    + -4275. * param.s2.powi(5))
                            + -420.
                                * param.m2_2.powi(4)
                                * (-7. * param.s1.powi(3)
                                    + 7. * param.s12.powi(3)
                                    + -38. * param.s1.powi(2) * param.s2
                                    + -38. * param.s1 * param.s2.powi(2)
                                    + -7. * param.s2.powi(3)
                                    + -21. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (21. * param.s1.powi(2)
                                            + 59. * param.s1 * param.s2
                                            + 21. * param.s2.powi(2)))
                            + 20.
                                * param.m2_2.powi(3)
                                * (137. * param.s1.powi(4)
                                    + 137. * param.s12.powi(4)
                                    + 1174. * param.s1.powi(3) * param.s2
                                    + 570. * param.s1.powi(2) * param.s2.powi(2)
                                    + -1430. * param.s1 * param.s2.powi(3)
                                    + -451. * param.s2.powi(4)
                                    + param.s12.powi(3) * (-548. * param.s1 + 40. * param.s2)
                                    + 2. * param.s12.powi(2)
                                        * (411. * param.s1.powi(2)
                                            + 547. * param.s1 * param.s2
                                            + -471. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (137. * param.s1.powi(3)
                                            + 577. * param.s1.powi(2) * param.s2
                                            + -221. * param.s1 * param.s2.powi(2)
                                            + -304. * param.s2.powi(3)))
                            + -30.
                                * param.m2_2.powi(2)
                                * (9. * param.s12.powi(5)
                                    + param.s12.powi(4) * (-45. * param.s1 + 229. * param.s2)
                                    + param.s12.powi(3)
                                        * (90. * param.s1.powi(2)
                                            + -337. * param.s1 * param.s2
                                            + -418. * param.s2.powi(2))
                                    + param.s12
                                        * (45. * param.s1.powi(4)
                                            + 821. * param.s1.powi(3) * param.s2
                                            + -1316. * param.s1.powi(2) * param.s2.powi(2)
                                            + -1271. * param.s1 * param.s2.powi(3)
                                            + 713. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(2)
                                        * (9. * param.s1.powi(3)
                                            + 368. * param.s1.powi(2) * param.s2
                                            + 938. * param.s1 * param.s2.powi(2)
                                            + 323. * param.s2.powi(3))
                                    - param.s12.powi(2)
                                        * (90. * param.s1.powi(3)
                                            + 363. * param.s1.powi(2) * param.s2
                                            + -1945. * param.s1 * param.s2.powi(2)
                                            + 210. * param.s2.powi(3)))
                            + -12.
                                * param.m2_2
                                * (3. * param.s12.powi(6)
                                    + -9. * param.s12.powi(5) * (2. * param.s1 + 7. * param.s2)
                                    + 5. * param.s12.powi(4)
                                        * (9. * param.s1.powi(2)
                                            + 39. * param.s1 * param.s2
                                            + -83. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(3)
                                        * (3. * param.s1.powi(3)
                                            + -48. * param.s1.powi(2) * param.s2
                                            + -557. * param.s1 * param.s2.powi(2)
                                            + -343. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (12. * param.s1.powi(3)
                                            + 30. * param.s1.powi(2) * param.s2
                                            + 151. * param.s1 * param.s2.powi(2)
                                            + -250. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + -90. * param.s1.powi(3) * param.s2
                                            + 2351. * param.s1.powi(2) * param.s2.powi(2)
                                            + -1855. * param.s1 * param.s2.powi(3)
                                            + -675. * param.s2.powi(4))
                                    - param.s12
                                        * (18. * param.s1.powi(5)
                                            + -165. * param.s1.powi(4) * param.s2
                                            + 777. * param.s1.powi(3) * param.s2.powi(2)
                                            + 1832. * param.s1.powi(2) * param.s2.powi(3)
                                            + -2905. * param.s1 * param.s2.powi(4)
                                            + 443. * param.s2.powi(5)))
                            - param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (35. * param.s1.powi(4)
                                    + -154. * param.s1.powi(3) * param.s2
                                    + -198. * param.s1.powi(2) * param.s2.powi(2)
                                    + -5348. * param.s1 * param.s2.powi(3)
                                    + -539. * param.s2.powi(4))
                            - param.s12.powi(5)
                                * (105. * param.s1.powi(2)
                                    + 296. * param.s1 * param.s2
                                    + 591. * param.s2.powi(2)))
                    + -2.
                        * param.m1_2.powi(2)
                        * param.s1.powi(3)
                        * (-5. * param.s12.powi(7)
                            + param.s12.powi(6) * (35. * param.s1 + 62. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (35. * param.s1.powi(3)
                                    + 56. * param.s1.powi(2) * param.s2
                                    + 56. * param.s1 * param.s2.powi(2)
                                    + 114. * param.s2.powi(3))
                            + (param.s1 - param.s2).powi(3)
                                * (5. * param.s1.powi(4)
                                    + -53. * param.s1.powi(3) * param.s2
                                    + 348. * param.s1.powi(2) * param.s2.powi(2)
                                    + 2017. * param.s1 * param.s2.powi(3)
                                    + 518. * param.s2.powi(4))
                            + param.s12.powi(3)
                                * (-175. * param.s1.powi(4)
                                    + 60. * param.s1.powi(3) * param.s2
                                    + 1050. * param.s1.powi(2) * param.s2.powi(2)
                                    + -5345. * param.s1 * param.s2.powi(3)
                                    + 675. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (105. * param.s1.powi(5)
                                    + -370. * param.s1.powi(4) * param.s2
                                    + -810. * param.s1.powi(3) * param.s2.powi(2)
                                    + -3471. * param.s1.powi(2) * param.s2.powi(3)
                                    + 10180. * param.s1 * param.s2.powi(4)
                                    + -2250. * param.s2.powi(5))
                            + param.s12
                                * (-35. * param.s1.powi(6)
                                    + 278. * param.s1.powi(5) * param.s2
                                    + -640. * param.s1.powi(4) * param.s2.powi(2)
                                    + 7437. * param.s1.powi(3) * param.s2.powi(3)
                                    + -4463. * param.s1.powi(2) * param.s2.powi(4)
                                    + -4445. * param.s1 * param.s2.powi(5)
                                    + 1868. * param.s2.powi(6))
                            + 5. * param.m2_2.powi(3)
                                * (137. * param.s1.powi(4)
                                    + 137. * param.s12.powi(4)
                                    + 1762. * param.s1.powi(3) * param.s2
                                    + 3762. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1762. * param.s1 * param.s2.powi(3)
                                    + 137. * param.s2.powi(4)
                                    + -548. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (822. * param.s1.powi(2)
                                            + 2858. * param.s1 * param.s2
                                            + 822. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (137. * param.s1.powi(3)
                                            + 1018. * param.s1.powi(2) * param.s2
                                            + 1018. * param.s1 * param.s2.powi(2)
                                            + 137. * param.s2.powi(3)))
                            + -15.
                                * param.m2_2.powi(2)
                                * (-9. * param.s1.powi(5)
                                    + 9. * param.s12.powi(5)
                                    + -487. * param.s1.powi(4) * param.s2
                                    + -1385. * param.s1.powi(3) * param.s2.powi(2)
                                    + 615. * param.s1.powi(2) * param.s2.powi(3)
                                    + 1138. * param.s1 * param.s2.powi(4)
                                    + 128. * param.s2.powi(5)
                                    + param.s12.powi(4) * (-45. * param.s1 + 92. * param.s2)
                                    + param.s12.powi(3)
                                        * (90. * param.s1.powi(2)
                                            + 211. * param.s1 * param.s2
                                            + -458. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-90. * param.s1.powi(3)
                                            + -1185. * param.s1.powi(2) * param.s2
                                            + 851. * param.s1 * param.s2.powi(2)
                                            + 732. * param.s2.powi(3))
                                    + param.s12
                                        * (45. * param.s1.powi(4)
                                            + 1369. * param.s1.powi(3) * param.s2
                                            + 992. * param.s1.powi(2) * param.s2.powi(2)
                                            + -2155. * param.s1 * param.s2.powi(3)
                                            + -503. * param.s2.powi(4)))
                            + -3.
                                * param.m2_2
                                * (9. * param.s12.powi(6)
                                    + -18. * param.s12.powi(5) * (3. * param.s1 + 8. * param.s2)
                                    + 5. * param.s12.powi(4)
                                        * (27. * param.s1.powi(2)
                                            + 72. * param.s1 * param.s2
                                            + -20. * param.s2.powi(2))
                                    + -10.
                                        * param.s12.powi(3)
                                        * (18. * param.s1.powi(3)
                                            + 395. * param.s1 * param.s2.powi(2)
                                            + -166. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (135. * param.s1.powi(4)
                                            + -720. * param.s1.powi(3) * param.s2
                                            + 5238. * param.s1.powi(2) * param.s2.powi(2)
                                            + 4160. * param.s1 * param.s2.powi(3)
                                            + -3075. * param.s2.powi(4))
                                    + (param.s1 - param.s2).powi(2)
                                        * (9. * param.s1.powi(4)
                                            + -198. * param.s1.powi(3) * param.s2
                                            + -3367. * param.s1.powi(2) * param.s2.powi(2)
                                            + -4048. * param.s1 * param.s2.powi(3)
                                            + -586. * param.s2.powi(4))
                                    + param.s12
                                        * (-54. * param.s1.powi(5)
                                            + 720. * param.s1.powi(4) * param.s2
                                            + 1774. * param.s1.powi(3) * param.s2.powi(2)
                                            + -12076. * param.s1.powi(2) * param.s2.powi(3)
                                            + 2360. * param.s1 * param.s2.powi(4)
                                            + 2236. * param.s2.powi(5)))
                            - param.s12.powi(5)
                                * (105. * param.s1.powi(2)
                                    + 242. * param.s1 * param.s2
                                    + 402. * param.s2.powi(2)))
                    + param.m0_2.powi(4)
                        * (param.m1_2
                            * (-5. * param.s12.powi(7)
                                + param.s12.powi(6) * (71. * param.s1 + 35. * param.s2)
                                + (param.s1 - param.s2).powi(4)
                                    * (511. * param.s1.powi(3)
                                        + 165. * param.s1.powi(2) * param.s2
                                        + -39. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (-675. * param.s1.powi(3)
                                        + 865. * param.s1.powi(2) * param.s2
                                        + 415. * param.s1 * param.s2.powi(2)
                                        + 175. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (885. * param.s1.powi(4)
                                        + -1522. * param.s1.powi(3) * param.s2
                                        + 120. * param.s1.powi(2) * param.s2.powi(2)
                                        + -24. * param.s1 * param.s2.powi(3)
                                        + -35. * param.s2.powi(4))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (539. * param.s1.powi(4)
                                        + 5348. * param.s1.powi(3) * param.s2
                                        + 198. * param.s1.powi(2) * param.s2.powi(2)
                                        + 154. * param.s1 * param.s2.powi(3)
                                        + -35. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-4275. * param.s1.powi(5)
                                        + 4615. * param.s1.powi(4) * param.s2
                                        + 3582. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1080. * param.s1.powi(2) * param.s2.powi(3)
                                        + -235. * param.s1 * param.s2.powi(4)
                                        + 105. * param.s2.powi(5))
                                - param.s12.powi(5)
                                    * (591. * param.s1.powi(2)
                                        + 296. * param.s1 * param.s2
                                        + 105. * param.s2.powi(2)))
                            + param.s1
                                * (-8. * param.s12.powi(7)
                                    + param.s12.powi(6) * (182. * param.s1 + 47. * param.s2)
                                    + 2. * param.s12.powi(5)
                                        * (588. * param.s1.powi(2)
                                            + -280. * param.s1 * param.s2
                                            + -57. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(4)
                                        * (690. * param.s1.powi(3)
                                            + -461. * param.s1.powi(2) * param.s2
                                            + -86. * param.s1 * param.s2.powi(2)
                                            + -29. * param.s2.powi(3))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (506. * param.s1.powi(3)
                                            + 889. * param.s1.powi(2) * param.s2
                                            + 61. * param.s1 * param.s2.powi(2)
                                            - param.s2.powi(3))
                                    + 10.
                                        * param.s12.powi(3)
                                        * (162. * param.s1.powi(4)
                                            + 553. * param.s1.powi(3) * param.s2
                                            + -675. * param.s1.powi(2) * param.s2.powi(2)
                                            + 24. * param.s1 * param.s2.powi(3)
                                            + -10. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (1518. * param.s1.powi(5)
                                            + -8705. * param.s1.powi(4) * param.s2
                                            + 5712. * param.s1.powi(3) * param.s2.powi(2)
                                            + 1872. * param.s1.powi(2) * param.s2.powi(3)
                                            + -430. * param.s1 * param.s2.powi(4)
                                            + 33. * param.s2.powi(5))
                                    + 3. * param.m2_2
                                        * (3. * param.s12.powi(6)
                                            + -6.
                                                * param.s12.powi(5)
                                                * (13. * param.s1 + 3. * param.s2)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (205. * param.s1.powi(2)
                                                    + -54. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + 60.
                                                * param.s12.powi(3)
                                                * (15. * param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + -5. * param.s1 * param.s2.powi(2)
                                                    - param.s2.powi(3))
                                            + -2.
                                                * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (499. * param.s1.powi(3)
                                                    + 553. * param.s1.powi(2) * param.s2
                                                    + -27. * param.s1 * param.s2.powi(2)
                                                    + 9. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (1385. * param.s1.powi(4)
                                                    + -3980. * param.s1.powi(3) * param.s2
                                                    + 1586.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 60. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(4)
                                                * (187. * param.s1.powi(2)
                                                    + 30. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2)))
                                    - (param.s1 - param.s2).powi(5)
                                        * (26. * param.s1.powi(2) + 5. * param.s1 * param.s2
                                            - param.s2.powi(2))))
                    + param.m0_2.powi(3)
                        * (2.
                            * param.m1_2.powi(2)
                            * (5. * param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (402. * param.s1.powi(2)
                                        + 242. * param.s1 * param.s2
                                        + 105. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (114. * param.s1.powi(3)
                                        + 56. * param.s1.powi(2) * param.s2
                                        + 56. * param.s1 * param.s2.powi(2)
                                        + 35. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(3)
                                    * (135. * param.s1.powi(4)
                                        + -1069. * param.s1.powi(3) * param.s2
                                        + 210. * param.s1.powi(2) * param.s2.powi(2)
                                        + 12. * param.s1 * param.s2.powi(3)
                                        + -35. * param.s2.powi(4))
                                + (param.s1 - param.s2).powi(3)
                                    * (518. * param.s1.powi(4)
                                        + 2017. * param.s1.powi(3) * param.s2
                                        + 348. * param.s1.powi(2) * param.s2.powi(2)
                                        + -53. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (2250. * param.s1.powi(5)
                                        + -10180. * param.s1.powi(4) * param.s2
                                        + 3471. * param.s1.powi(3) * param.s2.powi(2)
                                        + 810. * param.s1.powi(2) * param.s2.powi(3)
                                        + 370. * param.s1 * param.s2.powi(4)
                                        + -105. * param.s2.powi(5))
                                + param.s12
                                    * (-1868. * param.s1.powi(6)
                                        + 4445. * param.s1.powi(5) * param.s2
                                        + 4463. * param.s1.powi(4) * param.s2.powi(2)
                                        + -7437. * param.s1.powi(3) * param.s2.powi(3)
                                        + 640. * param.s1.powi(2) * param.s2.powi(4)
                                        + -278. * param.s1 * param.s2.powi(5)
                                        + 35. * param.s2.powi(6))
                                - param.s12.powi(6) * (62. * param.s1 + 35. * param.s2))
                            + param.s1.powi(2)
                                * (37. * param.s12.powi(7)
                                    + param.s12.powi(6) * (527. * param.s1 + -133. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                        * (19. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (-1839. * param.s1.powi(2)
                                            + 1870. * param.s1 * param.s2
                                            + 111. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (273. * param.s1.powi(3)
                                            + 1157. * param.s1.powi(2) * param.s2
                                            + -1399. * param.s1 * param.s2.powi(2)
                                            + 29. * param.s2.powi(3))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (563. * param.s1.powi(3)
                                            + 3397. * param.s1.powi(2) * param.s2
                                            + 1813. * param.s1 * param.s2.powi(2)
                                            + 47. * param.s2.powi(3))
                                    + 5. * param.s12.powi(3)
                                        * (219. * param.s1.powi(4)
                                            + -3010. * param.s1.powi(3) * param.s2
                                            + 1878. * param.s1.powi(2) * param.s2.powi(2)
                                            + 906. * param.s1 * param.s2.powi(3)
                                            + -65. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (-1767. * param.s1.powi(5)
                                            + 5875. * param.s1.powi(4) * param.s2
                                            + 9312. * param.s1.powi(3) * param.s2.powi(2)
                                            + -15408. * param.s1.powi(2) * param.s2.powi(3)
                                            + 1775. * param.s1 * param.s2.powi(4)
                                            + 213. * param.s2.powi(5))
                                    + 30.
                                        * param.m2_2.powi(2)
                                        * (3. * param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (122. * param.s1 + -15. * param.s2)
                                            + (param.s1 - param.s2).powi(3)
                                                * (106. * param.s1.powi(2)
                                                    + 80. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + 5. * param.s12.powi(3)
                                                * (14. * param.s1.powi(2)
                                                    + -59. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (-412. * param.s1.powi(3)
                                                    + 425. * param.s1.powi(2) * param.s2
                                                    + 153. * param.s1 * param.s2.powi(2)
                                                    + -30. * param.s2.powi(3))
                                            + param.s12
                                                * (111. * param.s1.powi(4)
                                                    + 403. * param.s1.powi(3) * param.s2
                                                    + -620.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 91. * param.s1 * param.s2.powi(3)
                                                    + 15. * param.s2.powi(4)))
                                    + -6.
                                        * param.m2_2
                                        * (21. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (574. * param.s1 + -96. * param.s2)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (149. * param.s1.powi(2)
                                                    + 110. * param.s1 * param.s2
                                                    + -33. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (253. * param.s1.powi(3)
                                                    + 1541. * param.s1.powi(2) * param.s2
                                                    + 286. * param.s1 * param.s2.powi(2)
                                                    + -12. * param.s2.powi(3))
                                            + -10.
                                                * param.s12.powi(3)
                                                * (122. * param.s1.powi(3)
                                                    + -523. * param.s1.powi(2) * param.s2
                                                    + 161. * param.s1 * param.s2.powi(2)
                                                    + 12. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (2075. * param.s1.powi(4)
                                                    + -3090. * param.s1.powi(3) * param.s2
                                                    + -3198.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2390. * param.s1 * param.s2.powi(3)
                                                    + 15. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(4)
                                                * (199. * param.s1.powi(2)
                                                    + 220. * param.s1 * param.s2
                                                    + 9. * param.s2.powi(2))))
                            + param.m1_2
                                * param.s1
                                * (19. * param.s12.powi(7)
                                    + param.s12.powi(5)
                                        * (-675. * param.s1.powi(2)
                                            + 574. * param.s1 * param.s2
                                            + 183. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (1041. * param.s1.powi(3)
                                            + -2485. * param.s1.powi(2) * param.s2
                                            + 185. * param.s1 * param.s2.powi(2)
                                            + -25. * param.s2.powi(3))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (539. * param.s1.powi(4)
                                            + -13840. * param.s1.powi(3) * param.s2
                                            + -11520. * param.s1.powi(2) * param.s2.powi(2)
                                            + 88. * param.s1 * param.s2.powi(3)
                                            + -83. * param.s2.powi(4))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (1551. * param.s1.powi(4)
                                            + -1976. * param.s1.powi(3) * param.s2
                                            + -2670. * param.s1.powi(2) * param.s2.powi(2)
                                            + 492. * param.s1 * param.s2.powi(3)
                                            + 11. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (3621. * param.s1.powi(5)
                                            + 16345. * param.s1.powi(4) * param.s2
                                            + -44682. * param.s1.powi(3) * param.s2.powi(2)
                                            + 12402. * param.s1.powi(2) * param.s2.powi(3)
                                            + 1325. * param.s1 * param.s2.powi(4)
                                            + 141. * param.s2.powi(5))
                                    + -12.
                                        * param.m2_2
                                        * (3. * param.s12.powi(6)
                                            + -9.
                                                * param.s12.powi(5)
                                                * (7. * param.s1 + 2. * param.s2)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (83. * param.s1.powi(2)
                                                    + -39. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + 5. * param.s12.powi(3)
                                                * (250. * param.s1.powi(3)
                                                    + -151. * param.s1.powi(2) * param.s2
                                                    + -30. * param.s1 * param.s2.powi(2)
                                                    + -12. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (343. * param.s1.powi(3)
                                                    + 557. * param.s1.powi(2) * param.s2
                                                    + 48. * param.s1 * param.s2.powi(2)
                                                    + -3. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-675. * param.s1.powi(4)
                                                    + -1855. * param.s1.powi(3) * param.s2
                                                    + 2351.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -90. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))
                                            - param.s12
                                                * (443. * param.s1.powi(5)
                                                    + -2905. * param.s1.powi(4) * param.s2
                                                    + 1832.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 777.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -165. * param.s1 * param.s2.powi(4)
                                                    + 18. * param.s2.powi(5)))
                                    - (param.s1 - param.s2).powi(4)
                                        * (605. * param.s1.powi(3)
                                            + 1779. * param.s1.powi(2) * param.s2
                                            + 201. * param.s1 * param.s2.powi(2)
                                            + -17. * param.s2.powi(3))
                                    - param.s12.powi(6) * (349. * param.s1 + 97. * param.s2)))
                    + param.m0_2.powi(2)
                        * (-2.
                            * param.m1_2.powi(3)
                            * (5. * param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (258. * param.s1.powi(2)
                                        + 188. * param.s1 * param.s2
                                        + 105. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (134. * param.s1.powi(3)
                                        + -16. * param.s1.powi(2) * param.s2
                                        + 29. * param.s1 * param.s2.powi(2)
                                        + 35. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (197. * param.s1.powi(4)
                                        + 279. * param.s1.powi(3) * param.s2
                                        + -210. * param.s1.powi(2) * param.s2.powi(2)
                                        + -48. * param.s1 * param.s2.powi(3)
                                        + 35. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-825. * param.s1.powi(5)
                                        + -6020. * param.s1.powi(4) * param.s2
                                        + 8709. * param.s1.powi(3) * param.s2.powi(2)
                                        + 90. * param.s1.powi(2) * param.s2.powi(3)
                                        + 505. * param.s1 * param.s2.powi(4)
                                        + -105. * param.s2.powi(5))
                                + param.s12
                                    * (368. * param.s1.powi(6)
                                        + 6805. * param.s1.powi(5) * param.s2
                                        + -7613. * param.s1.powi(4) * param.s2.powi(2)
                                        + -5663. * param.s1.powi(3) * param.s2.powi(3)
                                        + 1360. * param.s1.powi(2) * param.s2.powi(4)
                                        + -332. * param.s1 * param.s2.powi(5)
                                        + 35. * param.s2.powi(6))
                                - (param.s1 - param.s2).powi(2)
                                    * (68. * param.s1.powi(5)
                                        + 2549. * param.s1.powi(4) * param.s2
                                        + 5036. * param.s1.powi(3) * param.s2.powi(2)
                                        + 599. * param.s1.powi(2) * param.s2.powi(3)
                                        + -67. * param.s1 * param.s2.powi(4)
                                        + 5. * param.s2.powi(5))
                                - param.s12.powi(6) * (53. * param.s1 + 35. * param.s2))
                            + param.s1.powi(3)
                                * (37. * param.s12.powi(7)
                                    + param.s12.powi(6) * (-133. * param.s1 + 527. * param.s2)
                                    + param.s12.powi(5)
                                        * (111. * param.s1.powi(2)
                                            + 1870. * param.s1 * param.s2
                                            + -1839. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (29. * param.s1.powi(3)
                                            + -1399. * param.s1.powi(2) * param.s2
                                            + 1157. * param.s1 * param.s2.powi(2)
                                            + 273. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (65. * param.s1.powi(4)
                                            + -906. * param.s1.powi(3) * param.s2
                                            + -1878. * param.s1.powi(2) * param.s2.powi(2)
                                            + 3010. * param.s1 * param.s2.powi(3)
                                            + -219. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (213. * param.s1.powi(5)
                                            + 1775. * param.s1.powi(4) * param.s2
                                            + -15408. * param.s1.powi(3) * param.s2.powi(2)
                                            + 9312. * param.s1.powi(2) * param.s2.powi(3)
                                            + 5875. * param.s1 * param.s2.powi(4)
                                            + -1767. * param.s2.powi(5))
                                    + -10.
                                        * param.m2_2.powi(3)
                                        * (137. * param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (628. * param.s1 + -548. * param.s2)
                                            + -2.
                                                * param.s12.powi(2)
                                                * (513. * param.s1.powi(2)
                                                    + 335. * param.s1 * param.s2
                                                    + -411. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(2)
                                                * (641. * param.s1.powi(2)
                                                    + 860. * param.s1 * param.s2
                                                    + 137. * param.s2.powi(2))
                                            + -4.
                                                * param.s12
                                                * (95. * param.s1.powi(3)
                                                    + -620. * param.s1.powi(2) * param.s2
                                                    + 136. * param.s1 * param.s2.powi(2)
                                                    + 137. * param.s2.powi(3)))
                                    + 30.
                                        * param.m2_2.powi(2)
                                        * (70. * param.s12.powi(5)
                                            + 3. * param.s12.powi(4)
                                                * (46. * param.s1 + -71. * param.s2)
                                            + param.s12.powi(3)
                                                * (-664. * param.s1.powi(2)
                                                    + 671. * param.s1 * param.s2
                                                    + 152. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (464. * param.s1.powi(3)
                                                    + 905. * param.s1.powi(2) * param.s2
                                                    + -1627. * param.s1 * param.s2.powi(2)
                                                    + 122. * param.s2.powi(3))
                                            + param.s12
                                                * (162. * param.s1.powi(4)
                                                    + -1543. * param.s1.powi(3) * param.s2
                                                    + 890.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 689. * param.s1 * param.s2.powi(3)
                                                    + -198. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (170. * param.s1.powi(2)
                                                    + 330. * param.s1 * param.s2
                                                    + 67. * param.s2.powi(2)))
                                    + -6.
                                        * param.m2_2
                                        * (131. * param.s12.powi(6)
                                            + -86. * param.s12.powi(5) * (param.s1 + param.s2)
                                            + 2. * (param.s1 - param.s2).powi(4)
                                                * (58. * param.s1.powi(2)
                                                    + 205. * param.s1 * param.s2
                                                    + 58. * param.s2.powi(2))
                                            + -10.
                                                * param.s12.powi(4)
                                                * (85. * param.s1.powi(2)
                                                    + -331. * param.s1 * param.s2
                                                    + 85. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (13. * param.s1.powi(3)
                                                    + -1564. * param.s1.powi(2) * param.s2
                                                    + -1564. * param.s1 * param.s2.powi(2)
                                                    + 13. * param.s2.powi(3))
                                            + 10.
                                                * param.s12.powi(3)
                                                * (164. * param.s1.powi(3)
                                                    + -329. * param.s1.powi(2) * param.s2
                                                    + -329. * param.s1 * param.s2.powi(2)
                                                    + 164. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (925. * param.s1.powi(4)
                                                    + 3060. * param.s1.powi(3) * param.s2
                                                    + -10682.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 3060. * param.s1 * param.s2.powi(3)
                                                    + 925. * param.s2.powi(4)))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (47. * param.s1.powi(3)
                                            + 1813. * param.s1.powi(2) * param.s2
                                            + 3397. * param.s1 * param.s2.powi(2)
                                            + 563. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(5)
                                        * (param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + 19. * param.s2.powi(2)))
                            + 6. * param.m1_2
                                * param.s1.powi(2)
                                * (-5.
                                    * param.m2_2.powi(2)
                                    * (9. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (229. * param.s1 + -45. * param.s2)
                                        + param.s12.powi(3)
                                            * (-418. * param.s1.powi(2)
                                                + -337. * param.s1 * param.s2
                                                + 90. * param.s2.powi(2))
                                        + param.s12
                                            * (713. * param.s1.powi(4)
                                                + -1271. * param.s1.powi(3) * param.s2
                                                + -1316. * param.s1.powi(2) * param.s2.powi(2)
                                                + 821. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        - param.s12.powi(2)
                                            * (210. * param.s1.powi(3)
                                                + -1945. * param.s1.powi(2) * param.s2
                                                + 363. * param.s1 * param.s2.powi(2)
                                                + 90. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (323. * param.s1.powi(3)
                                                + 938. * param.s1.powi(2) * param.s2
                                                + 368. * param.s1 * param.s2.powi(2)
                                                + 9. * param.s2.powi(3)))
                                    + -3.
                                        * (param.s12.powi(7)
                                            + 5. * param.s12.powi(6) * (param.s1 + param.s2)
                                            + -4.
                                                * param.s12.powi(5)
                                                * (9. * param.s1.powi(2)
                                                    + -73. * param.s1 * param.s2
                                                    + 9. * param.s2.powi(2))
                                            + 10.
                                                * param.s12.powi(4)
                                                * (7. * param.s1.powi(3)
                                                    + -34. * param.s1.powi(2) * param.s2
                                                    + -34. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3))
                                            + param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (10. * param.s1.powi(4)
                                                    + -285. * param.s1.powi(3) * param.s2
                                                    + -1518.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -285. * param.s1 * param.s2.powi(3)
                                                    + 10. * param.s2.powi(4))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (11. * param.s1.powi(4)
                                                    + 139. * param.s1.powi(3) * param.s2
                                                    + -524.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 139. * param.s1 * param.s2.powi(3)
                                                    + 11. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (9. * param.s1.powi(5)
                                                    + 1130. * param.s1.powi(4) * param.s2
                                                    + -1591.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -1591.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 1130. * param.s1 * param.s2.powi(4)
                                                    + 9. * param.s2.powi(5))
                                            - (param.s1 - param.s2).powi(4)
                                                * (4. * param.s1.powi(3)
                                                    + 103. * param.s1.powi(2) * param.s2
                                                    + 103. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3)))
                                    + 2. * param.m2_2
                                        * (18. * param.s12.powi(6)
                                            + 7. * param.s12.powi(5)
                                                * (41. * param.s1 + -9. * param.s2)
                                            + -15.
                                                * param.s12.powi(4)
                                                * (68. * param.s1.powi(2)
                                                    + -62. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2))
                                            + 5. * param.s12.powi(3)
                                                * (170. * param.s1.powi(3)
                                                    + 596. * param.s1.powi(2) * param.s2
                                                    + -739. * param.s1 * param.s2.powi(2)
                                                    + 18. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (308. * param.s1.powi(3)
                                                    + 1602. * param.s1.powi(2) * param.s2
                                                    + 898. * param.s1 * param.s2.powi(2)
                                                    + 27. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (430. * param.s1.powi(4)
                                                    + -7820. * param.s1.powi(3) * param.s2
                                                    + 4711.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2635. * param.s1 * param.s2.powi(3)
                                                    + -180. * param.s2.powi(4))
                                            + param.s12
                                                * (-873. * param.s1.powi(5)
                                                    + 3295. * param.s1.powi(4) * param.s2
                                                    + 4483.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -7682.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 660. * param.s1 * param.s2.powi(4)
                                                    + 117. * param.s2.powi(5))))
                            + 3. * param.m1_2.powi(2)
                                * param.s1
                                * (2.
                                    * param.m2_2
                                    * (9. * param.s12.powi(6)
                                        + -18.
                                            * param.s12.powi(5)
                                            * (8. * param.s1 + 3. * param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (20. * param.s1.powi(2)
                                                + -72. * param.s1 * param.s2
                                                + -27. * param.s2.powi(2))
                                        + 10.
                                            * param.s12.powi(3)
                                            * (166. * param.s1.powi(3)
                                                + -395. * param.s1.powi(2) * param.s2
                                                + -18. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-3075. * param.s1.powi(4)
                                                + 4160. * param.s1.powi(3) * param.s2
                                                + 5238. * param.s1.powi(2) * param.s2.powi(2)
                                                + -720. * param.s1 * param.s2.powi(3)
                                                + 135. * param.s2.powi(4))
                                        + 2. * param.s12
                                            * (1118. * param.s1.powi(5)
                                                + 1180. * param.s1.powi(4) * param.s2
                                                + -6038. * param.s1.powi(3) * param.s2.powi(2)
                                                + 887. * param.s1.powi(2) * param.s2.powi(3)
                                                + 360. * param.s1 * param.s2.powi(4)
                                                + -27. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(2)
                                            * (586. * param.s1.powi(4)
                                                + 4048. * param.s1.powi(3) * param.s2
                                                + 3367. * param.s1.powi(2) * param.s2.powi(2)
                                                + 198. * param.s1 * param.s2.powi(3)
                                                + -9. * param.s2.powi(4)))
                                    + -3.
                                        * (param.s12.powi(7)
                                            + param.s12.powi(5)
                                                * (27. * param.s1.powi(2)
                                                    + -74. * param.s1 * param.s2
                                                    + -15. * param.s2.powi(2))
                                            + 5. * param.s12.powi(4)
                                                * (5. * param.s1.powi(3)
                                                    + -195. * param.s1.powi(2) * param.s2
                                                    + 65. * param.s1 * param.s2.powi(2)
                                                    + 11. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (23. * param.s1.powi(4)
                                                    + 718. * param.s1.powi(3) * param.s2
                                                    + 1068.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 86. * param.s1 * param.s2.powi(3)
                                                    + -5. * param.s2.powi(4))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (29. * param.s1.powi(4)
                                                    + -546. * param.s1.powi(3) * param.s2
                                                    + 250.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 60. * param.s1 * param.s2.powi(3)
                                                    + 17. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (189. * param.s1.powi(5)
                                                    + -1585. * param.s1.powi(4) * param.s2
                                                    + -3832.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 4522.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -115. * param.s1 * param.s2.powi(4)
                                                    + 69. * param.s2.powi(5))
                                            - param.s12
                                                * (107. * param.s1.powi(6)
                                                    + 744. * param.s1.powi(5) * param.s2
                                                    + -5789.
                                                        * param.s1.powi(4)
                                                        * param.s2.powi(2)
                                                    + 3658.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(3)
                                                    + 1529.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(4)
                                                    + -278. * param.s1 * param.s2.powi(5)
                                                    + 29. * param.s2.powi(6))
                                            - param.s12.powi(6) * (13. * param.s1 + param.s2))))
                    + param.m0_2
                        * (param.m1_2.powi(4)
                            * (-14. * param.s1.powi(7)
                                + 5. * param.s12.powi(7)
                                + 401. * param.s1.powi(6) * param.s2
                                + 7212. * param.s1.powi(5) * param.s2.powi(2)
                                + 2487. * param.s1.powi(4) * param.s2.powi(3)
                                + -9168. * param.s1.powi(3) * param.s2.powi(4)
                                + -999. * param.s1.powi(2) * param.s2.powi(5)
                                + 86. * param.s1 * param.s2.powi(6)
                                + -5. * param.s2.powi(7)
                                + param.s12.powi(5)
                                    * (159. * param.s1.powi(2)
                                        + 134. * param.s1 * param.s2
                                        + 105. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (62. * param.s1.powi(3)
                                        + -43. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 35. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (71. * param.s1.powi(4)
                                        + -300. * param.s1.powi(3) * param.s2
                                        + -120. * param.s1.powi(2) * param.s2.powi(2)
                                        + -84. * param.s1 * param.s2.powi(3)
                                        + 35. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-240. * param.s1.powi(5)
                                        + 2395. * param.s1.powi(4) * param.s2
                                        + 8022. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1080. * param.s1.powi(2) * param.s2.powi(3)
                                        + 640. * param.s1 * param.s2.powi(4)
                                        + -105. * param.s2.powi(5))
                                + param.s12
                                    * (89. * param.s1.powi(6)
                                        + -1610. * param.s1.powi(5) * param.s2
                                        + -14729. * param.s1.powi(4) * param.s2.powi(2)
                                        + 2956. * param.s1.powi(3) * param.s2.powi(3)
                                        + 2305. * param.s1.powi(2) * param.s2.powi(4)
                                        + -386. * param.s1 * param.s2.powi(5)
                                        + 35. * param.s2.powi(6))
                                - param.s12.powi(6) * (44. * param.s1 + 35. * param.s2))
                            + param.s1.powi(4)
                                * (-8. * param.s12.powi(7)
                                    + param.s12.powi(6) * (47. * param.s1 + 182. * param.s2)
                                    + -2.
                                        * param.s12.powi(5)
                                        * (57. * param.s1.powi(2)
                                            + 280. * param.s1 * param.s2
                                            + -588. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (29. * param.s1.powi(3)
                                            + 86. * param.s1.powi(2) * param.s2
                                            + 461. * param.s1 * param.s2.powi(2)
                                            + -690. * param.s2.powi(3))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(3)
                                            + -61. * param.s1.powi(2) * param.s2
                                            + -889. * param.s1 * param.s2.powi(2)
                                            + -506. * param.s2.powi(3))
                                    + -10.
                                        * param.s12.powi(3)
                                        * (10. * param.s1.powi(4)
                                            + -24. * param.s1.powi(3) * param.s2
                                            + 675. * param.s1.powi(2) * param.s2.powi(2)
                                            + -553. * param.s1 * param.s2.powi(3)
                                            + -162. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (33. * param.s1.powi(5)
                                            + -430. * param.s1.powi(4) * param.s2
                                            + 1872. * param.s1.powi(3) * param.s2.powi(2)
                                            + 5712. * param.s1.powi(2) * param.s2.powi(3)
                                            + -8705. * param.s1 * param.s2.powi(4)
                                            + 1518. * param.s2.powi(5))
                                    + 420.
                                        * param.m2_2.powi(4)
                                        * (13. * param.s1.powi(3)
                                            + 7. * param.s12.powi(3)
                                            + 12. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + -7. * param.s2.powi(3)
                                            + param.s12
                                                * (-19. * param.s1.powi(2)
                                                    + 19. * param.s1 * param.s2
                                                    + 21. * param.s2.powi(2))
                                            - param.s12.powi(2) * (param.s1 + 21. * param.s2))
                                    + -40.
                                        * param.m2_2.powi(3)
                                        * (122. * param.s12.powi(4)
                                            + -194. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + param.s12.powi(2)
                                                * (-150. * param.s1.powi(2)
                                                    + 971. * param.s1 * param.s2
                                                    + -150. * param.s2.powi(2))
                                            + param.s12
                                                * (394. * param.s1.powi(3)
                                                    + -646. * param.s1.powi(2) * param.s2
                                                    + -646. * param.s1 * param.s2.powi(2)
                                                    + 394. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(2)
                                                * (172. * param.s1.powi(2)
                                                    + 475. * param.s1 * param.s2
                                                    + 172. * param.s2.powi(2)))
                                    + 30.
                                        * param.m2_2.powi(2)
                                        * (70. * param.s12.powi(5)
                                            + -3.
                                                * param.s12.powi(4)
                                                * (71. * param.s1 + -46. * param.s2)
                                            + param.s12.powi(3)
                                                * (152. * param.s1.powi(2)
                                                    + 671. * param.s1 * param.s2
                                                    + -664. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (67. * param.s1.powi(2)
                                                    + 330. * param.s1 * param.s2
                                                    + 170. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (122. * param.s1.powi(3)
                                                    + -1627. * param.s1.powi(2) * param.s2
                                                    + 905. * param.s1 * param.s2.powi(2)
                                                    + 464. * param.s2.powi(3))
                                            + param.s12
                                                * (-198. * param.s1.powi(4)
                                                    + 689. * param.s1.powi(3) * param.s2
                                                    + 890.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -1543. * param.s1 * param.s2.powi(3)
                                                    + 162. * param.s2.powi(4)))
                                    + -6.
                                        * param.m2_2
                                        * (21. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-96. * param.s1 + 574. * param.s2)
                                            + 5. * param.s12.powi(4)
                                                * (33. * param.s1.powi(2)
                                                    + -110. * param.s1 * param.s2
                                                    + -149. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (12. * param.s1.powi(3)
                                                    + -286. * param.s1.powi(2) * param.s2
                                                    + -1541. * param.s1 * param.s2.powi(2)
                                                    + -253. * param.s2.powi(3))
                                            + -10.
                                                * param.s12.powi(3)
                                                * (12. * param.s1.powi(3)
                                                    + 161. * param.s1.powi(2) * param.s2
                                                    + -523. * param.s1 * param.s2.powi(2)
                                                    + 122. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (15. * param.s1.powi(4)
                                                    + 2390. * param.s1.powi(3) * param.s2
                                                    + -3198.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -3090. * param.s1 * param.s2.powi(3)
                                                    + 2075. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(4)
                                                * (9. * param.s1.powi(2)
                                                    + 220. * param.s1 * param.s2
                                                    + 199. * param.s2.powi(2)))
                                    - (param.s1 - param.s2).powi(5)
                                        * (param.s1.powi(2)
                                            + -5. * param.s1 * param.s2
                                            + -26. * param.s2.powi(2)))
                            + param.m1_2
                                * param.s1.powi(3)
                                * (19. * param.s12.powi(7)
                                    + param.s12.powi(5)
                                        * (183. * param.s1.powi(2)
                                            + 574. * param.s1 * param.s2
                                            + -675. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(4)
                                        * (25. * param.s1.powi(3)
                                            + -185. * param.s1.powi(2) * param.s2
                                            + 2485. * param.s1 * param.s2.powi(2)
                                            + -1041. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(4)
                                        * (17. * param.s1.powi(3)
                                            + -201. * param.s1.powi(2) * param.s2
                                            + -1779. * param.s1 * param.s2.powi(2)
                                            + -605. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (11. * param.s1.powi(4)
                                            + 492. * param.s1.powi(3) * param.s2
                                            + -2670. * param.s1.powi(2) * param.s2.powi(2)
                                            + -1976. * param.s1 * param.s2.powi(3)
                                            + 1551. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (141. * param.s1.powi(5)
                                            + 1325. * param.s1.powi(4) * param.s2
                                            + 12402. * param.s1.powi(3) * param.s2.powi(2)
                                            + -44682. * param.s1.powi(2) * param.s2.powi(3)
                                            + 16345. * param.s1 * param.s2.powi(4)
                                            + 3621. * param.s2.powi(5))
                                    + 20.
                                        * param.m2_2.powi(3)
                                        * (-451. * param.s1.powi(4)
                                            + 137. * param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (40. * param.s1 + -548. * param.s2)
                                            + -1430. * param.s1.powi(3) * param.s2
                                            + 570. * param.s1.powi(2) * param.s2.powi(2)
                                            + 1174. * param.s1 * param.s2.powi(3)
                                            + 137. * param.s2.powi(4)
                                            + param.s12.powi(2)
                                                * (-942. * param.s1.powi(2)
                                                    + 1094. * param.s1 * param.s2
                                                    + 822. * param.s2.powi(2))
                                            + 4. * param.s12
                                                * (304. * param.s1.powi(3)
                                                    + 221. * param.s1.powi(2) * param.s2
                                                    + -577. * param.s1 * param.s2.powi(2)
                                                    + -137. * param.s2.powi(3)))
                                    + -30.
                                        * param.m2_2.powi(2)
                                        * (79. * param.s12.powi(5)
                                            + -121. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (153. * param.s1.powi(2)
                                                    + -869. * param.s1 * param.s2
                                                    + 153. * param.s2.powi(2))
                                            + 39.
                                                * (param.s1 - param.s2).powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 37. * param.s1.powi(2) * param.s2
                                                    + 37. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (854. * param.s1.powi(3)
                                                    + -2060. * param.s1.powi(2) * param.s2
                                                    + -2060. * param.s1 * param.s2.powi(2)
                                                    + 854. * param.s2.powi(3))
                                            - param.s12
                                                * (701. * param.s1.powi(4)
                                                    + 610. * param.s1.powi(3) * param.s2
                                                    + -4638.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 610. * param.s1 * param.s2.powi(3)
                                                    + 701. * param.s2.powi(4)))
                                    + 12.
                                        * param.m2_2
                                        * (18. * param.s12.powi(6)
                                            + -7.
                                                * param.s12.powi(5)
                                                * (9. * param.s1 + -41. * param.s2)
                                            + 15.
                                                * param.s12.powi(4)
                                                * (3. * param.s1.powi(2)
                                                    + 62. * param.s1 * param.s2
                                                    + -68. * param.s2.powi(2))
                                            + 5. * param.s12.powi(3)
                                                * (18. * param.s1.powi(3)
                                                    + -739. * param.s1.powi(2) * param.s2
                                                    + 596. * param.s1 * param.s2.powi(2)
                                                    + 170. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-180. * param.s1.powi(4)
                                                    + 2635. * param.s1.powi(3) * param.s2
                                                    + 4711.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -7820. * param.s1 * param.s2.powi(3)
                                                    + 430. * param.s2.powi(4))
                                            + param.s12
                                                * (117. * param.s1.powi(5)
                                                    + 660. * param.s1.powi(4) * param.s2
                                                    + -7682.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 4483.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 3295. * param.s1 * param.s2.powi(4)
                                                    + -873. * param.s2.powi(5))
                                            - (param.s1 - param.s2).powi(3)
                                                * (27. * param.s1.powi(3)
                                                    + 898. * param.s1.powi(2) * param.s2
                                                    + 1602. * param.s1 * param.s2.powi(2)
                                                    + 308. * param.s2.powi(3)))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (83. * param.s1.powi(4)
                                            + -88. * param.s1.powi(3) * param.s2
                                            + 11520. * param.s1.powi(2) * param.s2.powi(2)
                                            + 13840. * param.s1 * param.s2.powi(3)
                                            + -539. * param.s2.powi(4))
                                    - param.s12.powi(6) * (97. * param.s1 + 349. * param.s2))
                            + param.m1_2.powi(3)
                                * param.s1
                                * (-7. * param.s12.powi(7)
                                    + 85. * param.s12.powi(6) * (param.s1 + param.s2)
                                    + -11.
                                        * param.s12.powi(5)
                                        * (33. * param.s1.powi(2)
                                            + 74. * param.s1 * param.s2
                                            + 33. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (157. * param.s1.powi(3)
                                            + 271. * param.s1.powi(2) * param.s2
                                            + 271. * param.s1 * param.s2.powi(2)
                                            + 157. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (193. * param.s1.powi(4)
                                            + -204. * param.s1.powi(3) * param.s2
                                            + 3450. * param.s1.powi(2) * param.s2.powi(2)
                                            + -204. * param.s1 * param.s2.powi(3)
                                            + 193. * param.s2.powi(4))
                                    + (param.s1 - param.s2).powi(2)
                                        * (43. * param.s1.powi(5)
                                            + -965. * param.s1.powi(4) * param.s2
                                            + -15458. * param.s1.powi(3) * param.s2.powi(2)
                                            + -15458. * param.s1.powi(2) * param.s2.powi(3)
                                            + -965. * param.s1 * param.s2.powi(4)
                                            + 43. * param.s2.powi(5))
                                    + param.s12.powi(2)
                                        * (687. * param.s1.powi(5)
                                            + -4325. * param.s1.powi(4) * param.s2
                                            + 18402. * param.s1.powi(3) * param.s2.powi(2)
                                            + 18402. * param.s1.powi(2) * param.s2.powi(3)
                                            + -4325. * param.s1 * param.s2.powi(4)
                                            + 687. * param.s2.powi(5))
                                    + param.s12
                                        * (-265. * param.s1.powi(6)
                                            + 3730. * param.s1.powi(5) * param.s2
                                            + 11341. * param.s1.powi(4) * param.s2.powi(2)
                                            + -49772. * param.s1.powi(3) * param.s2.powi(3)
                                            + 11341. * param.s1.powi(2) * param.s2.powi(4)
                                            + 3730. * param.s1 * param.s2.powi(5)
                                            + -265. * param.s2.powi(6))
                                    + -12.
                                        * param.m2_2
                                        * (18. * param.s1.powi(6)
                                            + 3. * param.s12.powi(6)
                                            + 938. * param.s1.powi(5) * param.s2
                                            + 2406. * param.s1.powi(4) * param.s2.powi(2)
                                            + -1479. * param.s1.powi(3) * param.s2.powi(3)
                                            + -1799. * param.s1.powi(2) * param.s2.powi(4)
                                            + -87. * param.s1 * param.s2.powi(5)
                                            + 3. * param.s2.powi(6)
                                            + -3.
                                                * param.s12.powi(5)
                                                * (11. * param.s1 + 6. * param.s2)
                                            + 15.
                                                * param.s12.powi(4)
                                                * (8. * param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (42. * param.s1.powi(3)
                                                    + 193. * param.s1.powi(2) * param.s2
                                                    + -30. * param.s1 * param.s2.powi(2)
                                                    + 12. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (195. * param.s1.powi(4)
                                                    + 2805. * param.s1.powi(3) * param.s2
                                                    + -229.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -390. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))
                                            - param.s12
                                                * (93. * param.s1.powi(5)
                                                    + 2805. * param.s1.powi(4) * param.s2
                                                    + 2372.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -2873.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -315. * param.s1 * param.s2.powi(4)
                                                    + 18. * param.s2.powi(5))))
                            + 3. * param.m1_2.powi(2)
                                * param.s1.powi(2)
                                * (10.
                                    * param.m2_2.powi(2)
                                    * (128. * param.s1.powi(5)
                                        + 9. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (92. * param.s1 + -45. * param.s2)
                                        + 1138. * param.s1.powi(4) * param.s2
                                        + 615. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1385. * param.s1.powi(2) * param.s2.powi(3)
                                        + -487. * param.s1 * param.s2.powi(4)
                                        + -9. * param.s2.powi(5)
                                        + param.s12.powi(3)
                                            * (-458. * param.s1.powi(2)
                                                + 211. * param.s1 * param.s2
                                                + 90. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (732. * param.s1.powi(3)
                                                + 851. * param.s1.powi(2) * param.s2
                                                + -1185. * param.s1 * param.s2.powi(2)
                                                + -90. * param.s2.powi(3))
                                        + param.s12
                                            * (-503. * param.s1.powi(4)
                                                + -2155. * param.s1.powi(3) * param.s2
                                                + 992. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1369. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4)))
                                    + -2.
                                        * param.m2_2
                                        * (9. * param.s12.powi(6)
                                            + 36. * param.s12.powi(5) * (param.s1 + param.s2)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (63. * param.s1.powi(2)
                                                    + -464. * param.s1 * param.s2
                                                    + 63. * param.s2.powi(2))
                                            + 10.
                                                * param.s12.powi(3)
                                                * (72. * param.s1.powi(3)
                                                    + -385. * param.s1.powi(2) * param.s2
                                                    + -385. * param.s1 * param.s2.powi(2)
                                                    + 72. * param.s2.powi(3))
                                            + 2. * param.s12
                                                * (198. * param.s1.powi(5)
                                                    + 3775. * param.s1.powi(4) * param.s2
                                                    + -6493.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -6493.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 3775. * param.s1 * param.s2.powi(4)
                                                    + 198. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (765. * param.s1.powi(4)
                                                    + 2730. * param.s1.powi(3) * param.s2
                                                    + -19498.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2730. * param.s1 * param.s2.powi(3)
                                                    + 765. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(2)
                                                * (81. * param.s1.powi(4)
                                                    + 3488. * param.s1.powi(3) * param.s2
                                                    + 9242.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 3488. * param.s1 * param.s2.powi(3)
                                                    + 81. * param.s2.powi(4)))
                                    + -3.
                                        * (param.s12.powi(7)
                                            + param.s12.powi(5)
                                                * (-15. * param.s1.powi(2)
                                                    + -74. * param.s1 * param.s2
                                                    + 27. * param.s2.powi(2))
                                            + 5. * param.s12.powi(4)
                                                * (11. * param.s1.powi(3)
                                                    + 65. * param.s1.powi(2) * param.s2
                                                    + -195. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (5. * param.s1.powi(4)
                                                    + -86. * param.s1.powi(3) * param.s2
                                                    + -1068.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -718. * param.s1 * param.s2.powi(3)
                                                    + -23. * param.s2.powi(4))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (17. * param.s1.powi(4)
                                                    + 60. * param.s1.powi(3) * param.s2
                                                    + 250.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -546. * param.s1 * param.s2.powi(3)
                                                    + 29. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (69. * param.s1.powi(5)
                                                    + -115. * param.s1.powi(4) * param.s2
                                                    + 4522.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -3832.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -1585. * param.s1 * param.s2.powi(4)
                                                    + 189. * param.s2.powi(5))
                                            - param.s12
                                                * (29. * param.s1.powi(6)
                                                    + -278. * param.s1.powi(5) * param.s2
                                                    + 1529.
                                                        * param.s1.powi(4)
                                                        * param.s2.powi(2)
                                                    + 3658.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(3)
                                                    + -5789.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(4)
                                                    + 744. * param.s1 * param.s2.powi(5)
                                                    + 107. * param.s2.powi(6))
                                            - param.s12.powi(6) * (param.s1 + 13. * param.s2))))
                    - param.m1_2.powi(4)
                        * param.s1
                        * (5. * param.s1.powi(7)
                            + -5. * param.s12.powi(7)
                            + -86. * param.s1.powi(6) * param.s2
                            + 999. * param.s1.powi(5) * param.s2.powi(2)
                            + 9168. * param.s1.powi(4) * param.s2.powi(3)
                            + -2487. * param.s1.powi(3) * param.s2.powi(4)
                            + -7212. * param.s1.powi(2) * param.s2.powi(5)
                            + -401. * param.s1 * param.s2.powi(6)
                            + 14. * param.s2.powi(7)
                            + param.s12.powi(6) * (35. * param.s1 + 44. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (35. * param.s1.powi(3)
                                    + 2. * param.s1.powi(2) * param.s2
                                    + -43. * param.s1 * param.s2.powi(2)
                                    + 62. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (35. * param.s1.powi(4)
                                    + -84. * param.s1.powi(3) * param.s2
                                    + -120. * param.s1.powi(2) * param.s2.powi(2)
                                    + -300. * param.s1 * param.s2.powi(3)
                                    + 71. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (105. * param.s1.powi(5)
                                    + -640. * param.s1.powi(4) * param.s2
                                    + 1080. * param.s1.powi(3) * param.s2.powi(2)
                                    + -8022. * param.s1.powi(2) * param.s2.powi(3)
                                    + -2395. * param.s1 * param.s2.powi(4)
                                    + 240. * param.s2.powi(5))
                            + param.s12
                                * (-35. * param.s1.powi(6)
                                    + 386. * param.s1.powi(5) * param.s2
                                    + -2305. * param.s1.powi(4) * param.s2.powi(2)
                                    + -2956. * param.s1.powi(3) * param.s2.powi(3)
                                    + 14729. * param.s1.powi(2) * param.s2.powi(4)
                                    + 1610. * param.s1 * param.s2.powi(5)
                                    + -89. * param.s2.powi(6))
                            + -3.
                                * param.m2_2
                                * (3. * param.s1.powi(6)
                                    + 3. * param.s12.powi(6)
                                    + -102. * param.s1.powi(5) * param.s2
                                    + -2839. * param.s1.powi(4) * param.s2.powi(2)
                                    + -6724. * param.s1.powi(3) * param.s2.powi(3)
                                    + -2839. * param.s1.powi(2) * param.s2.powi(4)
                                    + -102. * param.s1 * param.s2.powi(5)
                                    + 3. * param.s2.powi(6)
                                    + -18. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + 15.
                                        * param.s12.powi(4)
                                        * (3. * param.s1.powi(2)
                                            + -2. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + -60.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(3)
                                            + -5. * param.s1.powi(2) * param.s2
                                            + -5. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + -540. * param.s1.powi(3) * param.s2
                                            + -3574. * param.s1.powi(2) * param.s2.powi(2)
                                            + -540. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    + param.s12
                                        * (-18. * param.s1.powi(5)
                                            + 390. * param.s1.powi(4) * param.s2
                                            + 6068. * param.s1.powi(3) * param.s2.powi(2)
                                            + 6068. * param.s1.powi(2) * param.s2.powi(3)
                                            + 390. * param.s1 * param.s2.powi(4)
                                            + -18. * param.s2.powi(5)))
                            - param.s12.powi(5)
                                * (105. * param.s1.powi(2)
                                    + 134. * param.s1 * param.s2
                                    + 159. * param.s2.powi(2))))
                    * param.lambda_m01_sqrt
                    * param.lambda_s12_sqrt)
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.002777777777777778
            * std::f64::consts::PI
            * param.s2.powi(-3)
            * param.lambda_s12_sqrt.powi(-11)
            * (60.
                * param.s2.powi(3)
                * (14.
                    * param.m1_2.powi(6)
                    * param.s2.powi(3)
                    * (2. * param.s1.powi(2)
                        + 2. * param.s12.powi(2)
                        + 5. * param.s1 * param.s2
                        + 2. * param.s2.powi(2)
                        + -4. * param.s12 * (param.s1 + param.s2))
                    + param.m0_2.powi(6)
                        * param.s12
                        * (10. * param.s12.powi(4)
                            + 3. * (param.s1 - param.s2).powi(4)
                            + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                            + 9. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (param.s1 + param.s2)
                            + param.s12.powi(2)
                                * (-17. * param.s1.powi(2)
                                    + 40. * param.s1 * param.s2
                                    + -17. * param.s2.powi(2)))
                    + 21.
                        * param.m1_2.powi(5)
                        * param.s2.powi(2)
                        * (param.s2
                            * (-5. * param.s1.powi(3)
                                + -3. * param.s12.powi(3)
                                + -5. * param.s1.powi(2) * param.s2
                                + 7. * param.s1 * param.s2.powi(2)
                                + 3. * param.s2.powi(3)
                                + param.s12.powi(2) * (param.s1 + 9. * param.s2)
                                + param.s12
                                    * (7. * param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2)))
                            + 3. * param.m2_2
                                * (param.s12.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3)))
                    + 15.
                        * param.m1_2.powi(4)
                        * param.s2
                        * (-3.
                            * param.m2_2
                            * param.s2
                            * (-5. * param.s1.powi(4)
                                + 2. * param.s12.powi(4)
                                + -15. * param.s1.powi(3) * param.s2
                                + 5. * param.s1.powi(2) * param.s2.powi(2)
                                + 13. * param.s1 * param.s2.powi(3)
                                + 2. * param.s2.powi(4)
                                + param.s12.powi(2)
                                    * (-9. * param.s1.powi(2)
                                        + 15. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2))
                                + param.s12
                                    * (13. * param.s1.powi(3)
                                        + 8. * param.s1.powi(2) * param.s2
                                        + -27. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))
                                - param.s12.powi(3) * (param.s1 + 8. * param.s2))
                            + 3. * param.m2_2.powi(2)
                                * (param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + 10. * param.s1.powi(3) * param.s2
                                    + 20. * param.s1.powi(2) * param.s2.powi(2)
                                    + 10. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (param.s1.powi(3)
                                            + 6. * param.s1.powi(2) * param.s2
                                            + 6. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + param.s2.powi(2)
                                * (3. * param.s12.powi(4)
                                    + 3. * param.s12.powi(3) * (3. * param.s1 + -4. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (10. * param.s1.powi(2)
                                            + 15. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-17. * param.s1.powi(2)
                                            + -9. * param.s1 * param.s2
                                            + 18. * param.s2.powi(2))
                                    - param.s12
                                        * (5. * param.s1.powi(3)
                                            + -40. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + 12. * param.s2.powi(3))))
                    + -3.
                        * param.m0_2.powi(5)
                        * (3.
                            * param.s12
                            * (-2. * param.s12.powi(5)
                                + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + 10. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + -16. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-7. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + 9. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3)))
                            + param.m1_2
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (-2. * param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + -35. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(3)
                                            + -63. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (7. * param.s1 + 13. * param.s2)
                                    - (param.s1 - param.s2).powi(5))
                            + param.m2_2
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (param.s1 + -2. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (13. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(3)
                                        * (-35. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 45. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + 17. * param.s2.powi(3))))
                    + 10.
                        * param.m1_2.powi(3)
                        * (-3.
                            * param.m2_2
                            * param.s2.powi(2)
                            * (param.s12.powi(4) * (-7. * param.s1 + 5. * param.s2)
                                + param.s12.powi(3)
                                    * (17. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + -10. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (10. * param.s1.powi(3)
                                        + 30. * param.s1.powi(2) * param.s2
                                        + 15. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (param.s1.powi(3)
                                        + -63. * param.s1.powi(2) * param.s2
                                        + 18. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3))
                                + param.s12
                                    * (-20. * param.s1.powi(4)
                                        + 40. * param.s1.powi(3) * param.s2
                                        + 45. * param.s1.powi(2) * param.s2.powi(2)
                                        + -32. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                - param.s12.powi(5))
                            + param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + -25. * param.s1.powi(4) * param.s2
                                    + -100. * param.s1.powi(3) * param.s2.powi(2)
                                    + -100. * param.s1.powi(2) * param.s2.powi(3)
                                    + -25. * param.s1 * param.s2.powi(4)
                                    + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + 10.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -10.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + 5. * param.s12
                                        * (param.s1.powi(4)
                                            + 16. * param.s1.powi(3) * param.s2
                                            + 36. * param.s1.powi(2) * param.s2.powi(2)
                                            + 16. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4))
                                    - param.s2.powi(5)
                                    - param.s1.powi(5))
                            + param.s2.powi(3)
                                * (param.s12.powi(4) * (-13. * param.s1 + 5. * param.s2)
                                    + param.s12.powi(2)
                                        * (35. * param.s1.powi(3)
                                            + -45. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12
                                        * (10. * param.s1.powi(4)
                                            + 40. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3)
                                        * (10. * param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - param.s12.powi(5))
                            + -3.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (5. * param.s1.powi(5)
                                    + param.s12.powi(5)
                                    + param.s12.powi(4) * (param.s1 + -5. * param.s2)
                                    + 35. * param.s1.powi(4) * param.s2
                                    + 20. * param.s1.powi(3) * param.s2.powi(2)
                                    + -40. * param.s1.powi(2) * param.s2.powi(3)
                                    + -19. * param.s1 * param.s2.powi(4)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (7. * param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (13. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -27. * param.s1 * param.s2.powi(2)
                                            + -5. * param.s2.powi(3))
                                    + param.s12
                                        * (-19. * param.s1.powi(4)
                                            + -64. * param.s1.powi(3) * param.s2
                                            + 36. * param.s1.powi(2) * param.s2.powi(2)
                                            + 56. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s2.powi(5)))
                    + 15.
                        * param.m1_2.powi(2)
                        * param.s1
                        * (3.
                            * param.m2_2.powi(4)
                            * (param.s1.powi(4)
                                + param.s12.powi(4)
                                + 10. * param.s1.powi(3) * param.s2
                                + 20. * param.s1.powi(2) * param.s2.powi(2)
                                + 10. * param.s1 * param.s2.powi(3)
                                + param.s2.powi(4)
                                + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 6. * param.s12.powi(2)
                                    * (param.s1.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (param.s1.powi(3)
                                        + 6. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)))
                            + param.s2.powi(3)
                                * (2. * param.s12.powi(5)
                                    + param.s12.powi(4) * (8. * param.s1 + -7. * param.s2)
                                    + (param.s1 - param.s2).powi(4) * (2. * param.s1 + param.s2)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + 2. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (4. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                            - param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + -18. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + -2.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + -19. * param.s1.powi(4) * param.s2
                                    + -40. * param.s1.powi(3) * param.s2.powi(2)
                                    + 20. * param.s1.powi(2) * param.s2.powi(3)
                                    + 35. * param.s1 * param.s2.powi(4)
                                    + 5. * param.s2.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + 27. * param.s1.powi(2) * param.s2
                                            + -9. * param.s1 * param.s2.powi(2)
                                            + -13. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 56. * param.s1.powi(3) * param.s2
                                            + 36. * param.s1.powi(2) * param.s2.powi(2)
                                            + -64. * param.s1 * param.s2.powi(3)
                                            + -19. * param.s2.powi(4))
                                    - param.s1.powi(5))
                            + -6.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                    + param.s12.powi(3)
                                        * (-7. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 2. * param.s2.powi(3))
                                    + param.s12
                                        * (2. * param.s1.powi(4)
                                            + -16. * param.s1.powi(3) * param.s2
                                            + 9. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + -3. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (2. * param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + param.s2.powi(2)))
                            + 6. * param.m2_2.powi(2)
                                * param.s2
                                * (param.s12.powi(5)
                                    + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + 2. * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(3)
                                            + 6. * param.s1.powi(2) * param.s2
                                            + 6. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + 2. * param.s12.powi(2)
                                        * (4. * param.s1.powi(3)
                                            + -9. * param.s1.powi(2) * param.s2
                                            + -9. * param.s1 * param.s2.powi(2)
                                            + 4. * param.s2.powi(3))
                                    - param.s12
                                        * (7. * param.s1.powi(4)
                                            + 4. * param.s1.powi(3) * param.s2
                                            + -36. * param.s1.powi(2) * param.s2.powi(2)
                                            + 4. * param.s1 * param.s2.powi(3)
                                            + 7. * param.s2.powi(4))))
                    + param.s1.powi(3)
                        * (14.
                            * param.m2_2.powi(6)
                            * (2. * param.s1.powi(2)
                                + 2. * param.s12.powi(2)
                                + 5. * param.s1 * param.s2
                                + 2. * param.s2.powi(2)
                                + -4. * param.s12 * (param.s1 + param.s2))
                            + param.s12
                                * param.s2.powi(3)
                                * (10. * param.s12.powi(4)
                                    + 3. * (param.s1 - param.s2).powi(4)
                                    + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 9. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (-17. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2)))
                            + -21.
                                * param.m2_2.powi(5)
                                * (-3. * param.s1.powi(3)
                                    + 3. * param.s12.powi(3)
                                    + -7. * param.s1.powi(2) * param.s2
                                    + 5. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3)
                                    + param.s12
                                        * (9. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    - param.s12.powi(2) * (9. * param.s1 + param.s2))
                            + -3.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (-2. * param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + -35. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(3)
                                            + -63. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (7. * param.s1 + 13. * param.s2)
                                    - (param.s1 - param.s2).powi(5))
                            + 15.
                                * param.m2_2.powi(4)
                                * (3. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-12. * param.s1 + 9. * param.s2)
                                    + param.s12.powi(2)
                                        * (18. * param.s1.powi(2)
                                            + -9. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(2)
                                            + 15. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    - param.s12
                                        * (12. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -40. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                            + 15.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (2. * param.s12.powi(5)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + 2. * param.s2)
                                    + param.s12.powi(4) * (-7. * param.s1 + 8. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (4. * param.s1.powi(2)
                                            + -2. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + -6. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                            + -10.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + 13. * param.s2)
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-10. * param.s1.powi(3)
                                            + 18. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 8. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 40. * param.s1 * param.s2.powi(3)
                                            + 10. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))))
                    + -3.
                        * param.m1_2
                        * param.s1.powi(2)
                        * (-21.
                            * param.m2_2.powi(5)
                            * (param.s12.powi(3)
                                + -5. * param.s1.powi(2) * param.s2
                                + -5. * param.s1 * param.s2.powi(2)
                                + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                - param.s2.powi(3)
                                - param.s1.powi(3))
                            + 15.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (-2. * param.s12.powi(5)
                                    + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + -16. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-7. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -7. * param.s2.powi(3)))
                            + 15.
                                * param.m2_2.powi(4)
                                * (2. * param.s1.powi(4)
                                    + 2. * param.s12.powi(4)
                                    + 13. * param.s1.powi(3) * param.s2
                                    + 5. * param.s1.powi(2) * param.s2.powi(2)
                                    + -15. * param.s1 * param.s2.powi(3)
                                    + -5. * param.s2.powi(4)
                                    + 3. * param.s12.powi(2)
                                        * (4. * param.s1.powi(2)
                                            + 5. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    + param.s12
                                        * (-8. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + 8. * param.s1 * param.s2.powi(2)
                                            + 13. * param.s2.powi(3))
                                    - param.s12.powi(3) * (8. * param.s1 + param.s2))
                            + param.s2.powi(3)
                                * (10. * param.s12.powi(5)
                                    + 10. * param.s12.powi(4) * (param.s1 + -2. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (13. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(3)
                                        * (-35. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 45. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + 17. * param.s2.powi(3)))
                            + 30.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (2. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2)
                                            + 4. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (2. * param.s1.powi(3)
                                            + -18. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3))
                                    + param.s12
                                        * (-3. * param.s1.powi(4)
                                            + 8. * param.s1.powi(3) * param.s2
                                            + 9. * param.s1.powi(2) * param.s2.powi(2)
                                            + -16. * param.s1 * param.s2.powi(3)
                                            + 2. * param.s2.powi(4)))
                            + -10.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 32. * param.s1.powi(3) * param.s2
                                            + -45. * param.s1.powi(2) * param.s2.powi(2)
                                            + -40. * param.s1 * param.s2.powi(3)
                                            + 20. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(3)
                                            + 15. * param.s1.powi(2) * param.s2
                                            + 30. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12.powi(2)
                                        * (10. * param.s1.powi(3)
                                            + 18. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))))
                    + 3. * param.m0_2.powi(4)
                        * (5.
                            * param.m2_2.powi(2)
                            * (2. * param.s12.powi(5)
                                + param.s12.powi(4) * (8. * param.s1 + -7. * param.s2)
                                + (param.s1 - param.s2).powi(4) * (2. * param.s1 + param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 2. * param.s1 * param.s2
                                        + -4. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (4. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                        - param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -18. * param.s1.powi(2) * param.s2
                                        + 9. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)))
                            + 5. * param.m1_2.powi(2)
                                * (2. * param.s12.powi(5)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + 2. * param.s2)
                                    + param.s12.powi(4) * (-7. * param.s1 + 8. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (4. * param.s1.powi(2)
                                            + -2. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + -6. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                            + 3. * param.s12
                                * (param.s12.powi(6)
                                    + param.s12.powi(4)
                                        * (-5. * param.s1.powi(2)
                                            + 18. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(3)
                                            + -17. * param.s1.powi(2) * param.s2
                                            + -17. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + 17. * param.s1.powi(3) * param.s2
                                            + -54. * param.s1.powi(2) * param.s2.powi(2)
                                            + 17. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(3)
                                            + -16. * param.s1.powi(2) * param.s2
                                            + -16. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    - param.s12.powi(5) * (param.s1 + param.s2))
                            + param.m1_2
                                * (-12. * param.s12.powi(6)
                                    + 2. * param.s12.powi(5) * (16. * param.s1 + -9. * param.s2)
                                    + -2.
                                        * param.s12.powi(4)
                                        * (5. * param.s1.powi(2)
                                            + 53. * param.s1 * param.s2
                                            + -45. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (40. * param.s1.powi(4)
                                            + -81. * param.s1.powi(3) * param.s2
                                            + -153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 239. * param.s1 * param.s2.powi(3)
                                            + -45. * param.s2.powi(4))
                                    + 15.
                                        * param.m2_2
                                        * (2. * param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (-5. * param.s1.powi(2)
                                                    + 16. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (7. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 10. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(4)
                                                * (param.s1 + param.s2))
                                    - param.s12.powi(3)
                                        * (40. * param.s1.powi(3)
                                            + -239. * param.s1.powi(2) * param.s2
                                            + 136. * param.s1 * param.s2.powi(2)
                                            + 45. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (8. * param.s1.powi(2)
                                            + 65. * param.s1 * param.s2
                                            + 27. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(5)
                                        * (2. * param.s1 + 3. * param.s2))
                            - param.m2_2
                                * (12. * param.s12.powi(6)
                                    + 2. * param.s12.powi(5) * (9. * param.s1 + -16. * param.s2)
                                    + param.s12.powi(4)
                                        * (-90. * param.s1.powi(2)
                                            + 106. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (45. * param.s1.powi(3)
                                            + 136. * param.s1.powi(2) * param.s2
                                            + -239. * param.s1 * param.s2.powi(2)
                                            + 40. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + -239. * param.s1.powi(3) * param.s2
                                            + 153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 81. * param.s1 * param.s2.powi(3)
                                            + -40. * param.s2.powi(4))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (27. * param.s1.powi(2)
                                            + 65. * param.s1 * param.s2
                                            + 8. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(5)
                                        * (3. * param.s1 + 2. * param.s2)))
                    + 3. * param.m0_2.powi(2)
                        * (5.
                            * param.m1_2.powi(4)
                            * param.s2
                            * (3. * param.s12.powi(4)
                                + param.s12.powi(3) * (-12. * param.s1 + 9. * param.s2)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + -9. * param.s1 * param.s2
                                        + -17. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 15. * param.s1 * param.s2
                                        + 10. * param.s2.powi(2))
                                - param.s12
                                    * (12. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + -40. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)))
                            + 10.
                                * param.m1_2.powi(3)
                                * (-3.
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (2. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (2. * param.s1.powi(3)
                                                + -18. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + param.s12
                                            * (-3. * param.s1.powi(4)
                                                + 8. * param.s1.powi(3) * param.s2
                                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                + -16. * param.s1 * param.s2.powi(3)
                                                + 2. * param.s2.powi(4)))
                                    + param.m2_2
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1 + 7. * param.s2)
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + -17. * param.s2.powi(2))
                                            + param.s12
                                                * (5. * param.s1.powi(4)
                                                    + 32. * param.s1.powi(3) * param.s2
                                                    + -45.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -40. * param.s1 * param.s2.powi(3)
                                                    + 20. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 15. * param.s1.powi(2) * param.s2
                                                    + 30. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (10. * param.s1.powi(3)
                                                    + 18. * param.s1.powi(2) * param.s2
                                                    + -63. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))))
                            + param.s1
                                * (5.
                                    * param.m2_2.powi(4)
                                    * (3. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3)
                                            * (3. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (10. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-17. * param.s1.powi(2)
                                                + -9. * param.s1 * param.s2
                                                + 18. * param.s2.powi(2))
                                        - param.s12
                                            * (5. * param.s1.powi(3)
                                                + -40. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 12. * param.s2.powi(3)))
                                    + -30.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3))
                                            + param.s12
                                                * (2. * param.s1.powi(4)
                                                    + -16. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + 8. * param.s1 * param.s2.powi(3)
                                                    + -3. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + 18.
                                        * param.m2_2.powi(2)
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    + 3. * param.s12
                                        * param.s2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    - param.m2_2
                                        * (3. * param.s12.powi(7)
                                            + param.s12.powi(6)
                                                * (-9. * param.s1 + 15. * param.s2)
                                            + param.s12.powi(5)
                                                * (param.s1.powi(2)
                                                    + 72. * param.s1 * param.s2
                                                    + -63. * param.s2.powi(2))
                                            + param.s12.powi(4)
                                                * (25. * param.s1.powi(3)
                                                    + -239. * param.s1.powi(2) * param.s2
                                                    + 153. * param.s1 * param.s2.powi(2)
                                                    + 45. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (-35. * param.s1.powi(4)
                                                    + 136. * param.s1.powi(3) * param.s2
                                                    + 298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -432. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (17. * param.s1.powi(5)
                                                    + 81. * param.s1.powi(4) * param.s2
                                                    + -486.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 153. * param.s1 * param.s2.powi(4)
                                                    + -63. * param.s2.powi(5))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(3)
                                                    + 67. * param.s1.powi(2) * param.s2
                                                    + 117. * param.s1 * param.s2.powi(2)
                                                    + 15. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(5)
                                                * (param.s1.powi(2)
                                                    + 6. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))))
                            + 6. * param.m1_2.powi(2)
                                * (3.
                                    * param.s2
                                    * (param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (-5. * param.s1.powi(2)
                                                + 18. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(3)
                                                + -17. * param.s1.powi(2) * param.s2
                                                + -17. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (5. * param.s1.powi(4)
                                                + 17. * param.s1.powi(3) * param.s2
                                                + -54. * param.s1.powi(2) * param.s2.powi(2)
                                                + 17. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + -16. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - param.s12.powi(5) * (param.s1 + param.s2))
                                    + 5. * param.m2_2.powi(2)
                                        * (param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 6. * param.s1.powi(2) * param.s2
                                                    + 6. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + 2. * param.s12.powi(2)
                                                * (4. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12
                                                * (7. * param.s1.powi(4)
                                                    + 4. * param.s1.powi(3) * param.s2
                                                    + -36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 4. * param.s1 * param.s2.powi(3)
                                                    + 7. * param.s2.powi(4)))
                                    - param.m2_2
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-7. * param.s1 + 8. * param.s2)
                                            + param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + -40. * param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 18. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -144. * param.s1.powi(2) * param.s2
                                                    + 81. * param.s1 * param.s2.powi(2)
                                                    + 40. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-20. * param.s1.powi(4)
                                                    + 106. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -239. * param.s1 * param.s2.powi(3)
                                                    + 10. * param.s2.powi(4))
                                            + param.s12
                                                * (13. * param.s1.powi(5)
                                                    + 16. * param.s1.powi(4) * param.s2
                                                    + -239.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 136.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 106. * param.s1 * param.s2.powi(4)
                                                    + -32. * param.s2.powi(5))))
                            + param.m1_2
                                * (10.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (7. * param.s1 + -5. * param.s2)
                                        + param.s12.powi(3)
                                            * (-17. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        + param.s12
                                            * (20. * param.s1.powi(4)
                                                + -40. * param.s1.powi(3) * param.s2
                                                + -45. * param.s1.powi(2) * param.s2.powi(2)
                                                + 32. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -63. * param.s1.powi(2) * param.s2
                                                + 18. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (10. * param.s1.powi(3)
                                                + 30. * param.s1.powi(2) * param.s2
                                                + 15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                    + param.s2
                                        * (-3. * param.s12.powi(7)
                                            + param.s12.powi(6)
                                                * (-15. * param.s1 + 9. * param.s2)
                                            + param.s12.powi(5)
                                                * (63. * param.s1.powi(2)
                                                    + -72. * param.s1 * param.s2
                                                    - param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (-45. * param.s1.powi(4)
                                                    + 432. * param.s1.powi(3) * param.s2
                                                    + -298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -136. * param.s1 * param.s2.powi(3)
                                                    + 35. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (63. * param.s1.powi(5)
                                                    + -153. * param.s1.powi(4) * param.s2
                                                    + -298.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 486.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -81. * param.s1 * param.s2.powi(4)
                                                    + -17. * param.s2.powi(5))
                                            - param.s12.powi(4)
                                                * (45. * param.s1.powi(3)
                                                    + 153. * param.s1.powi(2) * param.s2
                                                    + -239. * param.s1 * param.s2.powi(2)
                                                    + 25. * param.s2.powi(3))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (15. * param.s1.powi(3)
                                                    + 117. * param.s1.powi(2) * param.s2
                                                    + 67. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(5)
                                                * (3. * param.s1.powi(2)
                                                    + 6. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + 3. * param.m2_2
                                        * (param.s12.powi(7)
                                            + param.s12.powi(6) * (param.s1 + param.s2)
                                            + param.s12.powi(5)
                                                * (-17. * param.s1.powi(2)
                                                    + 64. * param.s1 * param.s2
                                                    + -17. * param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + param.s12.powi(4)
                                                * (35. * param.s1.powi(3)
                                                    + -81. * param.s1.powi(2) * param.s2
                                                    + -81. * param.s1 * param.s2.powi(2)
                                                    + 35. * param.s2.powi(3))
                                            + 9. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(4)
                                                    + -6. * param.s1.powi(3) * param.s2
                                                    + -30.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -6. * param.s1 * param.s2.powi(3)
                                                    + param.s2.powi(4))
                                            - param.s12.powi(2)
                                                * (param.s1.powi(5)
                                                    + -239. * param.s1.powi(4) * param.s2
                                                    + 298.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 298.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -239. * param.s1 * param.s2.powi(4)
                                                    + param.s2.powi(5))
                                            - param.s12.powi(3)
                                                * (25. * param.s1.powi(4)
                                                    + 136. * param.s1.powi(3) * param.s2
                                                    + -486.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 136. * param.s1 * param.s2.powi(3)
                                                    + 25. * param.s2.powi(4)))
                                    + -6.
                                        * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (8. * param.s1 + -7. * param.s2)
                                            + param.s12.powi(4)
                                                * (-40. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(3)
                                                * (4. * param.s1.powi(3)
                                                    + 18. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (40. * param.s1.powi(3)
                                                    + 81. * param.s1.powi(2) * param.s2
                                                    + -144. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (10. * param.s1.powi(4)
                                                    + -239. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 106. * param.s1 * param.s2.powi(3)
                                                    + -20. * param.s2.powi(4))
                                            + param.s12
                                                * (-32. * param.s1.powi(5)
                                                    + 106. * param.s1.powi(4) * param.s2
                                                    + 136.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -239.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 16. * param.s1 * param.s2.powi(4)
                                                    + 13. * param.s2.powi(5)))))
                    + -3.
                        * param.m0_2
                        * (7.
                            * param.m1_2.powi(5)
                            * param.s2.powi(2)
                            * (-3. * param.s1.powi(3)
                                + 3. * param.s12.powi(3)
                                + -7. * param.s1.powi(2) * param.s2
                                + 5. * param.s1 * param.s2.powi(2)
                                + 5. * param.s2.powi(3)
                                + param.s12
                                    * (9. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + -7. * param.s2.powi(2))
                                - param.s12.powi(2) * (9. * param.s1 + param.s2))
                            + 5. * param.m1_2.powi(4)
                                * param.s2
                                * (param.s2
                                    * (-9. * param.s12.powi(4)
                                        + 15. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 4. * (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -64. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + param.s12
                                            * (-27. * param.s1.powi(3)
                                                + 41. * param.s1.powi(2) * param.s2
                                                + 41. * param.s1 * param.s2.powi(2)
                                                + -27. * param.s2.powi(3)))
                                    + 3. * param.m2_2
                                        * (2. * param.s1.powi(4)
                                            + 2. * param.s12.powi(4)
                                            + 13. * param.s1.powi(3) * param.s2
                                            + 5. * param.s1.powi(2) * param.s2.powi(2)
                                            + -15. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4)
                                            + 3. * param.s12.powi(2)
                                                * (4. * param.s1.powi(2)
                                                    + 5. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2))
                                            + param.s12
                                                * (-8. * param.s1.powi(3)
                                                    + -27. * param.s1.powi(2) * param.s2
                                                    + 8. * param.s1 * param.s2.powi(2)
                                                    + 13. * param.s2.powi(3))
                                            - param.s12.powi(3) * (8. * param.s1 + param.s2)))
                            + 10.
                                * param.m1_2.powi(3)
                                * (param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + -19. * param.s1.powi(4) * param.s2
                                        + -40. * param.s1.powi(3) * param.s2.powi(2)
                                        + 20. * param.s1.powi(2) * param.s2.powi(3)
                                        + 35. * param.s1 * param.s2.powi(4)
                                        + 5. * param.s2.powi(5)
                                        + param.s12.powi(4) * (-5. * param.s1 + param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 27. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + -13. * param.s2.powi(3))
                                        + param.s12
                                            * (5. * param.s1.powi(4)
                                                + 56. * param.s1.powi(3) * param.s2
                                                + 36. * param.s1.powi(2) * param.s2.powi(2)
                                                + -64. * param.s1 * param.s2.powi(3)
                                                + -19. * param.s2.powi(4))
                                        - param.s1.powi(5))
                                    + 3. * param.s2.powi(2)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3))
                                            + param.s12
                                                * (2. * param.s1.powi(4)
                                                    + -16. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + 8. * param.s1 * param.s2.powi(3)
                                                    + -3. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + -4.
                                        * param.m2_2
                                        * param.s2
                                        * (param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 6. * param.s1.powi(2) * param.s2
                                                    + 6. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + 2. * param.s12.powi(2)
                                                * (4. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12
                                                * (7. * param.s1.powi(4)
                                                    + 4. * param.s1.powi(3) * param.s2
                                                    + -36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 4. * param.s1 * param.s2.powi(3)
                                                    + 7. * param.s2.powi(4))))
                            + param.s1.powi(2)
                                * (7.
                                    * param.m2_2.powi(5)
                                    * (5. * param.s1.powi(3)
                                        + 3. * param.s12.powi(3)
                                        + 5. * param.s1.powi(2) * param.s2
                                        + -7. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s2.powi(3)
                                        + param.s12
                                            * (-7. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        - param.s12.powi(2) * (param.s1 + 9. * param.s2))
                                    + 3. * param.s12
                                        * param.s2.powi(2)
                                        * (-2. * param.s12.powi(5)
                                            + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1 + param.s2)
                                            + param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 10. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (5. * param.s1.powi(2)
                                                    + -16. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (-7. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + -7. * param.s2.powi(3)))
                                    + -5.
                                        * param.m2_2.powi(4)
                                        * (9. * param.s12.powi(4)
                                            + -15. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + param.s12.powi(2)
                                                * (-9. * param.s1.powi(2)
                                                    + 64. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + -4.
                                                * (param.s1 - param.s2).powi(2)
                                                * (3. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + param.s12
                                                * (27. * param.s1.powi(3)
                                                    + -41. * param.s1.powi(2) * param.s2
                                                    + -41. * param.s1 * param.s2.powi(2)
                                                    + 27. * param.s2.powi(3)))
                                    + 2. * param.m2_2.powi(2)
                                        * (-3. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (13. * param.s1 + -27. * param.s2)
                                            + 2. * (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + param.s12.powi(4)
                                                * (-20. * param.s1.powi(2)
                                                    + 16. * param.s1 * param.s2
                                                    + 45. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + 106. * param.s1.powi(2) * param.s2
                                                    + -239. * param.s1 * param.s2.powi(2)
                                                    + 45. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + -144. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 136. * param.s1 * param.s2.powi(3)
                                                    + -90. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (7. * param.s1.powi(3)
                                                    + -27. * param.s1.powi(2) * param.s2
                                                    + -142. * param.s1 * param.s2.powi(2)
                                                    + -18. * param.s2.powi(3)))
                                    + 30.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + -7. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (2. * param.s1.powi(3)
                                                    + -18. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12
                                                * (-3. * param.s1.powi(4)
                                                    + 8. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + -16. * param.s1 * param.s2.powi(3)
                                                    + 2. * param.s2.powi(4)))
                                    + param.m2_2
                                        * param.s2
                                        * (12. * param.s12.powi(6)
                                            + (param.s1 - param.s2).powi(5)
                                                * (2. * param.s1 + 3. * param.s2)
                                            + param.s12.powi(5)
                                                * (-32. * param.s1 + 18. * param.s2)
                                            + 2. * param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + 53. * param.s1 * param.s2
                                                    + -45. * param.s2.powi(2))
                                            + param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (8. * param.s1.powi(2)
                                                    + 65. * param.s1 * param.s2
                                                    + 27. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (40. * param.s1.powi(3)
                                                    + -239. * param.s1.powi(2) * param.s2
                                                    + 136. * param.s1 * param.s2.powi(2)
                                                    + 45. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-40. * param.s1.powi(4)
                                                    + 81. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -239. * param.s1 * param.s2.powi(3)
                                                    + 45. * param.s2.powi(4))))
                            + param.m1_2
                                * param.s1
                                * (15.
                                    * param.m2_2.powi(4)
                                    * (-5. * param.s1.powi(4)
                                        + 2. * param.s12.powi(4)
                                        + -15. * param.s1.powi(3) * param.s2
                                        + 5. * param.s1.powi(2) * param.s2.powi(2)
                                        + 13. * param.s1 * param.s2.powi(3)
                                        + 2. * param.s2.powi(4)
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + param.s12
                                            * (13. * param.s1.powi(3)
                                                + 8. * param.s1.powi(2) * param.s2
                                                + -27. * param.s1 * param.s2.powi(2)
                                                + -8. * param.s2.powi(3))
                                        - param.s12.powi(3) * (param.s1 + 8. * param.s2))
                                    + param.s2.powi(2)
                                        * (12. * param.s12.powi(6)
                                            + 2. * param.s12.powi(5)
                                                * (9. * param.s1 + -16. * param.s2)
                                            + param.s12.powi(4)
                                                * (-90. * param.s1.powi(2)
                                                    + 106. * param.s1 * param.s2
                                                    + 10. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (45. * param.s1.powi(3)
                                                    + 136. * param.s1.powi(2) * param.s2
                                                    + -239. * param.s1 * param.s2.powi(2)
                                                    + 40. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (45. * param.s1.powi(4)
                                                    + -239. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 81. * param.s1 * param.s2.powi(3)
                                                    + -40. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(3)
                                                * (27. * param.s1.powi(2)
                                                    + 65. * param.s1 * param.s2
                                                    + 8. * param.s2.powi(2))
                                            - (param.s1 - param.s2).powi(5)
                                                * (3. * param.s1 + 2. * param.s2))
                                    + -24.
                                        * param.m2_2
                                        * param.s2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    + -40.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + 6. * param.s1.powi(2) * param.s2
                                                    + 6. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + 2. * param.s12.powi(2)
                                                * (4. * param.s1.powi(3)
                                                    + -9. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            - param.s12
                                                * (7. * param.s1.powi(4)
                                                    + 4. * param.s1.powi(3) * param.s2
                                                    + -36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 4. * param.s1 * param.s2.powi(3)
                                                    + 7. * param.s2.powi(4)))
                                    + 6. * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-7. * param.s1 + 8. * param.s2)
                                            + param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + -40. * param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 18. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -144. * param.s1.powi(2) * param.s2
                                                    + 81. * param.s1 * param.s2.powi(2)
                                                    + 40. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-20. * param.s1.powi(4)
                                                    + 106. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -239. * param.s1 * param.s2.powi(3)
                                                    + 10. * param.s2.powi(4))
                                            + param.s12
                                                * (13. * param.s1.powi(5)
                                                    + 16. * param.s1.powi(4) * param.s2
                                                    + -239.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 136.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 106. * param.s1 * param.s2.powi(4)
                                                    + -32. * param.s2.powi(5))))
                            + 2. * param.m1_2.powi(2)
                                * (param.s2.powi(2)
                                    * (-3. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-27. * param.s1 + 13. * param.s2)
                                        + param.s12.powi(4)
                                            * (45. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -20. * param.s2.powi(2))
                                        + 2. * (param.s1 - param.s2).powi(4)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (18. * param.s1.powi(3)
                                                + 142. * param.s1.powi(2) * param.s2
                                                + 27. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (45. * param.s1.powi(3)
                                                + -239. * param.s1.powi(2) * param.s2
                                                + 106. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-90. * param.s1.powi(4)
                                                + 136. * param.s1.powi(3) * param.s2
                                                + 153. * param.s1.powi(2) * param.s2.powi(2)
                                                + -144. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4)))
                                    + 5. * param.m2_2.powi(3)
                                        * (5. * param.s1.powi(5)
                                            + param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -5. * param.s2)
                                            + 35. * param.s1.powi(4) * param.s2
                                            + 20. * param.s1.powi(3) * param.s2.powi(2)
                                            + -40. * param.s1.powi(2) * param.s2.powi(3)
                                            + -19. * param.s1 * param.s2.powi(4)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (7. * param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + 2. * param.s12.powi(2)
                                                * (13. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -27. * param.s1 * param.s2.powi(2)
                                                    + -5. * param.s2.powi(3))
                                            + param.s12
                                                * (-19. * param.s1.powi(4)
                                                    + -64. * param.s1.powi(3) * param.s2
                                                    + 36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 56. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s2.powi(5))
                                    + 3. * param.m2_2
                                        * param.s2
                                        * (2. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (8. * param.s1 + -7. * param.s2)
                                            + param.s12.powi(4)
                                                * (-40. * param.s1.powi(2)
                                                    + 41. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(3)
                                                * (4. * param.s1.powi(3)
                                                    + 18. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (40. * param.s1.powi(3)
                                                    + 81. * param.s1.powi(2) * param.s2
                                                    + -144. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (10. * param.s1.powi(4)
                                                    + -239. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 106. * param.s1 * param.s2.powi(3)
                                                    + -20. * param.s2.powi(4))
                                            + param.s12
                                                * (-32. * param.s1.powi(5)
                                                    + 106. * param.s1.powi(4) * param.s2
                                                    + 136.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -239.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 16. * param.s1 * param.s2.powi(4)
                                                    + 13. * param.s2.powi(5)))
                                    + -3.
                                        * param.m2_2.powi(2)
                                        * (param.s12.powi(6)
                                            + -2.
                                                * param.s12.powi(4)
                                                * (5. * param.s1.powi(2)
                                                    + -24. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (30. * param.s1.powi(3)
                                                    + -82. * param.s1.powi(2) * param.s2
                                                    + -82. * param.s1 * param.s2.powi(2)
                                                    + 30. * param.s2.powi(3))
                                            + -4.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(4)
                                                    + 16. * param.s1.powi(3) * param.s2
                                                    + 36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 16. * param.s1 * param.s2.powi(3)
                                                    + param.s2.powi(4))
                                            + param.s12
                                                * (19. * param.s1.powi(5)
                                                    + 123. * param.s1.powi(4) * param.s2
                                                    + -212.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -212.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 123. * param.s1 * param.s2.powi(4)
                                                    + 19. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (35. * param.s1.powi(4)
                                                    + 32. * param.s1.powi(3) * param.s2
                                                    + -324.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 32. * param.s1 * param.s2.powi(3)
                                                    + 35. * param.s2.powi(4))
                                            - param.s12.powi(5) * (param.s1 + param.s2))))
                    - param.m0_2.powi(3)
                        * (-6.
                            * param.m2_2.powi(2)
                            * (3. * param.s12.powi(6)
                                + param.s12.powi(5) * (27. * param.s1 + -13. * param.s2)
                                + -2.
                                    * (param.s1 - param.s2).powi(4)
                                    * (6. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-45. * param.s1.powi(2)
                                        + -16. * param.s1 * param.s2
                                        + 20. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (90. * param.s1.powi(4)
                                        + -136. * param.s1.powi(3) * param.s2
                                        + -153. * param.s1.powi(2) * param.s2.powi(2)
                                        + 144. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                - param.s12.powi(3)
                                    * (45. * param.s1.powi(3)
                                        + -239. * param.s1.powi(2) * param.s2
                                        + 106. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (18. * param.s1.powi(3)
                                        + 142. * param.s1.powi(2) * param.s2
                                        + 27. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3)))
                            + 10.
                                * param.m2_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (13. * param.s1 + -5. * param.s2)
                                    + (param.s1 - param.s2).powi(3)
                                        * (10. * param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-35. * param.s1.powi(3)
                                            + 45. * param.s1.powi(2) * param.s2
                                            + 18. * param.s1 * param.s2.powi(2)
                                            + -10. * param.s2.powi(3))
                                    + param.s12
                                        * (10. * param.s1.powi(4)
                                            + 40. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4)))
                            + 10.
                                * param.m1_2.powi(3)
                                * (param.s12.powi(5)
                                    + param.s12.powi(4) * (-5. * param.s1 + 13. * param.s2)
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + -32. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-10. * param.s1.powi(3)
                                            + 18. * param.s1.powi(2) * param.s2
                                            + 45. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + 8. * param.s1.powi(3) * param.s2
                                            + -63. * param.s1.powi(2) * param.s2.powi(2)
                                            + 40. * param.s1 * param.s2.powi(3)
                                            + 10. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2)))
                            + param.s12
                                * (param.s12.powi(5)
                                    * (17. * param.s1.powi(2)
                                        + -64. * param.s1 * param.s2
                                        + 17. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-35. * param.s1.powi(3)
                                            + 81. * param.s1.powi(2) * param.s2
                                            + 81. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + 3. * (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + -9.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(4)
                                            + -6. * param.s1.powi(3) * param.s2
                                            + -30. * param.s1.powi(2) * param.s2.powi(2)
                                            + -6. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4))
                                    + param.s12.powi(3)
                                        * (25. * param.s1.powi(4)
                                            + 136. * param.s1.powi(3) * param.s2
                                            + -486. * param.s1.powi(2) * param.s2.powi(2)
                                            + 136. * param.s1 * param.s2.powi(3)
                                            + 25. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(5)
                                            + -239. * param.s1.powi(4) * param.s2
                                            + 298. * param.s1.powi(3) * param.s2.powi(2)
                                            + 298. * param.s1.powi(2) * param.s2.powi(3)
                                            + -239. * param.s1 * param.s2.powi(4)
                                            + param.s2.powi(5))
                                    - param.s12.powi(6) * (param.s1 + param.s2)
                                    - param.s12.powi(7))
                            + 3. * param.m2_2
                                * (3. * param.s12.powi(7)
                                    + 3. * param.s12.powi(6) * (5. * param.s1 + -3. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                        * (3. * param.s1.powi(2)
                                            + 6. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (-63. * param.s1.powi(2)
                                            + 72. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (15. * param.s1.powi(3)
                                            + 117. * param.s1.powi(2) * param.s2
                                            + 67. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + param.s12.powi(4)
                                        * (45. * param.s1.powi(3)
                                            + 153. * param.s1.powi(2) * param.s2
                                            + -239. * param.s1 * param.s2.powi(2)
                                            + 25. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (45. * param.s1.powi(4)
                                            + -432. * param.s1.powi(3) * param.s2
                                            + 298. * param.s1.powi(2) * param.s2.powi(2)
                                            + 136. * param.s1 * param.s2.powi(3)
                                            + -35. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (-63. * param.s1.powi(5)
                                            + 153. * param.s1.powi(4) * param.s2
                                            + 298. * param.s1.powi(3) * param.s2.powi(2)
                                            + -486. * param.s1.powi(2) * param.s2.powi(3)
                                            + 81. * param.s1 * param.s2.powi(4)
                                            + 17. * param.s2.powi(5)))
                            + 6. * param.m1_2.powi(2)
                                * (-3. * param.s12.powi(6)
                                    + param.s12.powi(5) * (13. * param.s1 + -27. * param.s2)
                                    + 2. * (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-20. * param.s1.powi(2)
                                            + 16. * param.s1 * param.s2
                                            + 45. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (10. * param.s1.powi(3)
                                            + 106. * param.s1.powi(2) * param.s2
                                            + -239. * param.s1 * param.s2.powi(2)
                                            + 45. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + -144. * param.s1.powi(3) * param.s2
                                            + 153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 136. * param.s1 * param.s2.powi(3)
                                            + -90. * param.s2.powi(4))
                                    + 15.
                                        * param.m2_2
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + -7. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (2. * param.s1.powi(3)
                                                    + -18. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12
                                                * (-3. * param.s1.powi(4)
                                                    + 8. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + -16. * param.s1 * param.s2.powi(3)
                                                    + 2. * param.s2.powi(4)))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (7. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + -142. * param.s1 * param.s2.powi(2)
                                            + -18. * param.s2.powi(3)))
                            + 3. * param.m1_2
                                * (3. * param.s12.powi(7)
                                    + param.s12.powi(6) * (-9. * param.s1 + 15. * param.s2)
                                    + param.s12.powi(5)
                                        * (param.s1.powi(2)
                                            + 72. * param.s1 * param.s2
                                            + -63. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (25. * param.s1.powi(3)
                                            + -239. * param.s1.powi(2) * param.s2
                                            + 153. * param.s1 * param.s2.powi(2)
                                            + 45. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (-35. * param.s1.powi(4)
                                            + 136. * param.s1.powi(3) * param.s2
                                            + 298. * param.s1.powi(2) * param.s2.powi(2)
                                            + -432. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(5)
                                            + 81. * param.s1.powi(4) * param.s2
                                            + -486. * param.s1.powi(3) * param.s2.powi(2)
                                            + 298. * param.s1.powi(2) * param.s2.powi(3)
                                            + 153. * param.s1 * param.s2.powi(4)
                                            + -63. * param.s2.powi(5))
                                    + 30.
                                        * param.m2_2.powi(2)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -18. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3))
                                            + param.s12
                                                * (2. * param.s1.powi(4)
                                                    + -16. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + 8. * param.s1 * param.s2.powi(3)
                                                    + -3. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + param.s2.powi(2)))
                                    + -24.
                                        * param.m2_2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(3)
                                            + 67. * param.s1.powi(2) * param.s2
                                            + 117. * param.s1 * param.s2.powi(2)
                                            + 15. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(5)
                                        * (param.s1.powi(2)
                                            + 6. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2)))))
                * log_diff(
                    (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1),
                    param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m0_2.powi(5)
                    * (param.s12.powi(7)
                        + param.s12.powi(5)
                            * (21. * param.s1.powi(2)
                                + 70. * param.s1 * param.s2
                                + 165. * param.s2.powi(2))
                        + param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (7. * param.s1.powi(3)
                                + -13. * param.s1.powi(2) * param.s2
                                + -85. * param.s1 * param.s2.powi(2)
                                + -491. * param.s2.powi(3))
                        + -5.
                            * param.s12.powi(4)
                            * (7. * param.s1.powi(3)
                                + 22. * param.s1.powi(2) * param.s2
                                + 67. * param.s1 * param.s2.powi(2)
                                + -150. * param.s2.powi(3))
                        + 5. * param.s12.powi(3)
                            * (7. * param.s1.powi(4)
                                + 12. * param.s1.powi(3) * param.s2
                                + 12. * param.s1.powi(2) * param.s2.powi(2)
                                + 218. * param.s1 * param.s2.powi(3)
                                + -285. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (-21. * param.s1.powi(5)
                                + 20. * param.s1.powi(4) * param.s2
                                + 180. * param.s1.powi(3) * param.s2.powi(2)
                                + -1668. * param.s1.powi(2) * param.s2.powi(3)
                                + 1465. * param.s1 * param.s2.powi(4)
                                + 24. * param.s2.powi(5))
                        - (param.s1 - param.s2).powi(5)
                            * (param.s1.powi(2)
                                + -5. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        - param.s12.powi(6) * (7. * param.s1 + 16. * param.s2))
                    + param.m2_2.powi(5)
                        * (param.s1.powi(7)
                            + -19. * param.s1.powi(6) * param.s2
                            + 261. * param.s1.powi(5) * param.s2.powi(2)
                            + 3537. * param.s1.powi(4) * param.s2.powi(3)
                            + 3537. * param.s1.powi(3) * param.s2.powi(4)
                            + 261. * param.s1.powi(2) * param.s2.powi(5)
                            + -19. * param.s1 * param.s2.powi(6)
                            + param.s2.powi(7)
                            + 7. * param.s12.powi(6) * (param.s1 + param.s2)
                            + 5. * param.s12.powi(4)
                                * (7. * param.s1.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + 7. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (7. * param.s1.powi(4)
                                    + -24. * param.s1.powi(3) * param.s2
                                    + 12. * param.s1.powi(2) * param.s2.powi(2)
                                    + -24. * param.s1 * param.s2.powi(3)
                                    + 7. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (21. * param.s1.powi(5)
                                    + -155. * param.s1.powi(4) * param.s2
                                    + 540. * param.s1.powi(3) * param.s2.powi(2)
                                    + 540. * param.s1.powi(2) * param.s2.powi(3)
                                    + -155. * param.s1 * param.s2.powi(4)
                                    + 21. * param.s2.powi(5))
                            - param.s12
                                * (7. * param.s1.powi(6)
                                    + -88. * param.s1.powi(5) * param.s2
                                    + 695. * param.s1.powi(4) * param.s2.powi(2)
                                    + 4232. * param.s1.powi(3) * param.s2.powi(3)
                                    + 695. * param.s1.powi(2) * param.s2.powi(4)
                                    + -88. * param.s1 * param.s2.powi(5)
                                    + 7. * param.s2.powi(6))
                            - param.s12.powi(5)
                                * (21. * param.s1.powi(2)
                                    + 16. * param.s1 * param.s2
                                    + 21. * param.s2.powi(2))
                            - param.s12.powi(7))
                    + param.s2.powi(5)
                        * (param.s12.powi(7)
                            + (param.s1 - param.s2).powi(5)
                                * (10. * param.s1.powi(2)
                                    + -5. * param.s1 * param.s2
                                    + param.s2.powi(2))
                            + param.s12.powi(5)
                                * (165. * param.s1.powi(2)
                                    + 70. * param.s1 * param.s2
                                    + 21. * param.s2.powi(2))
                            + 5. * param.s12.powi(4)
                                * (150. * param.s1.powi(3)
                                    + -67. * param.s1.powi(2) * param.s2
                                    + -22. * param.s1 * param.s2.powi(2)
                                    + -7. * param.s2.powi(3))
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (491. * param.s1.powi(3)
                                    + 85. * param.s1.powi(2) * param.s2
                                    + 13. * param.s1 * param.s2.powi(2)
                                    + -7. * param.s2.powi(3))
                            + 5. * param.s12.powi(3)
                                * (-285. * param.s1.powi(4)
                                    + 218. * param.s1.powi(3) * param.s2
                                    + 12. * param.s1.powi(2) * param.s2.powi(2)
                                    + 12. * param.s1 * param.s2.powi(3)
                                    + 7. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (24. * param.s1.powi(5)
                                    + 1465. * param.s1.powi(4) * param.s2
                                    + -1668. * param.s1.powi(3) * param.s2.powi(2)
                                    + 180. * param.s1.powi(2) * param.s2.powi(3)
                                    + 20. * param.s1 * param.s2.powi(4)
                                    + -21. * param.s2.powi(5))
                            + -840.
                                * param.m1_2.powi(5)
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + 2. * param.s2.powi(2)
                                    + -4. * param.s12 * (param.s1 + param.s2))
                            + 420.
                                * param.m1_2.powi(4)
                                * (13. * param.s1.powi(3)
                                    + 7. * param.s12.powi(3)
                                    + 12. * param.s1.powi(2) * param.s2
                                    + -18. * param.s1 * param.s2.powi(2)
                                    + -7. * param.s2.powi(3)
                                    + param.s12
                                        * (-19. * param.s1.powi(2)
                                            + 19. * param.s1 * param.s2
                                            + 21. * param.s2.powi(2))
                                    - param.s12.powi(2) * (param.s1 + 21. * param.s2))
                            + -10.
                                * param.m1_2.powi(3)
                                * (137. * param.s12.powi(4)
                                    + param.s12.powi(3) * (628. * param.s1 + -548. * param.s2)
                                    + -2.
                                        * param.s12.powi(2)
                                        * (513. * param.s1.powi(2)
                                            + 335. * param.s1 * param.s2
                                            + -411. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (641. * param.s1.powi(2)
                                            + 860. * param.s1 * param.s2
                                            + 137. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (95. * param.s1.powi(3)
                                            + -620. * param.s1.powi(2) * param.s2
                                            + 136. * param.s1 * param.s2.powi(2)
                                            + 137. * param.s2.powi(3)))
                            + 30.
                                * param.m1_2.powi(2)
                                * (3. * param.s12.powi(5)
                                    + param.s12.powi(4) * (122. * param.s1 + -15. * param.s2)
                                    + (param.s1 - param.s2).powi(3)
                                        * (106. * param.s1.powi(2)
                                            + 80. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + 5. * param.s12.powi(3)
                                        * (14. * param.s1.powi(2)
                                            + -59. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-412. * param.s1.powi(3)
                                            + 425. * param.s1.powi(2) * param.s2
                                            + 153. * param.s1 * param.s2.powi(2)
                                            + -30. * param.s2.powi(3))
                                    + param.s12
                                        * (111. * param.s1.powi(4)
                                            + 403. * param.s1.powi(3) * param.s2
                                            + -620. * param.s1.powi(2) * param.s2.powi(2)
                                            + 91. * param.s1 * param.s2.powi(3)
                                            + 15. * param.s2.powi(4)))
                            + 3. * param.m1_2
                                * (3. * param.s12.powi(6)
                                    + -6. * param.s12.powi(5) * (13. * param.s1 + 3. * param.s2)
                                    + -5.
                                        * param.s12.powi(4)
                                        * (205. * param.s1.powi(2)
                                            + -54. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2))
                                    + 60.
                                        * param.s12.powi(3)
                                        * (15. * param.s1.powi(3)
                                            + 12. * param.s1.powi(2) * param.s2
                                            + -5. * param.s1 * param.s2.powi(2)
                                            - param.s2.powi(3))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (499. * param.s1.powi(3)
                                            + 553. * param.s1.powi(2) * param.s2
                                            + -27. * param.s1 * param.s2.powi(2)
                                            + 9. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (1385. * param.s1.powi(4)
                                            + -3980. * param.s1.powi(3) * param.s2
                                            + 1586. * param.s1.powi(2) * param.s2.powi(2)
                                            + 60. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(4)
                                        * (187. * param.s1.powi(2)
                                            + 30. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2)))
                            - param.s12.powi(6) * (16. * param.s1 + 7. * param.s2))
                    + -2.
                        * param.m2_2.powi(2)
                        * param.s2.powi(3)
                        * (-5. * param.s12.powi(7)
                            + param.s12.powi(6) * (62. * param.s1 + 35. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (114. * param.s1.powi(3)
                                    + 56. * param.s1.powi(2) * param.s2
                                    + 56. * param.s1 * param.s2.powi(2)
                                    + 35. * param.s2.powi(3))
                            + 5. * param.s12.powi(3)
                                * (135. * param.s1.powi(4)
                                    + -1069. * param.s1.powi(3) * param.s2
                                    + 210. * param.s1.powi(2) * param.s2.powi(2)
                                    + 12. * param.s1 * param.s2.powi(3)
                                    + -35. * param.s2.powi(4))
                            + param.s12
                                * (1868. * param.s1.powi(6)
                                    + -4445. * param.s1.powi(5) * param.s2
                                    + -4463. * param.s1.powi(4) * param.s2.powi(2)
                                    + 7437. * param.s1.powi(3) * param.s2.powi(3)
                                    + -640. * param.s1.powi(2) * param.s2.powi(4)
                                    + 278. * param.s1 * param.s2.powi(5)
                                    + -35. * param.s2.powi(6))
                            + 5. * param.m1_2.powi(3)
                                * (137. * param.s1.powi(4)
                                    + 137. * param.s12.powi(4)
                                    + 1762. * param.s1.powi(3) * param.s2
                                    + 3762. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1762. * param.s1 * param.s2.powi(3)
                                    + 137. * param.s2.powi(4)
                                    + -548. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (822. * param.s1.powi(2)
                                            + 2858. * param.s1 * param.s2
                                            + 822. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (137. * param.s1.powi(3)
                                            + 1018. * param.s1.powi(2) * param.s2
                                            + 1018. * param.s1 * param.s2.powi(2)
                                            + 137. * param.s2.powi(3)))
                            + -15.
                                * param.m1_2.powi(2)
                                * (128. * param.s1.powi(5)
                                    + 9. * param.s12.powi(5)
                                    + param.s12.powi(4) * (92. * param.s1 + -45. * param.s2)
                                    + 1138. * param.s1.powi(4) * param.s2
                                    + 615. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1385. * param.s1.powi(2) * param.s2.powi(3)
                                    + -487. * param.s1 * param.s2.powi(4)
                                    + -9. * param.s2.powi(5)
                                    + param.s12.powi(3)
                                        * (-458. * param.s1.powi(2)
                                            + 211. * param.s1 * param.s2
                                            + 90. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (732. * param.s1.powi(3)
                                            + 851. * param.s1.powi(2) * param.s2
                                            + -1185. * param.s1 * param.s2.powi(2)
                                            + -90. * param.s2.powi(3))
                                    + param.s12
                                        * (-503. * param.s1.powi(4)
                                            + -2155. * param.s1.powi(3) * param.s2
                                            + 992. * param.s1.powi(2) * param.s2.powi(2)
                                            + 1369. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4)))
                            + -3.
                                * param.m1_2
                                * (9. * param.s12.powi(6)
                                    + -18. * param.s12.powi(5) * (8. * param.s1 + 3. * param.s2)
                                    + -5.
                                        * param.s12.powi(4)
                                        * (20. * param.s1.powi(2)
                                            + -72. * param.s1 * param.s2
                                            + -27. * param.s2.powi(2))
                                    + 10.
                                        * param.s12.powi(3)
                                        * (166. * param.s1.powi(3)
                                            + -395. * param.s1.powi(2) * param.s2
                                            + -18. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (-3075. * param.s1.powi(4)
                                            + 4160. * param.s1.powi(3) * param.s2
                                            + 5238. * param.s1.powi(2) * param.s2.powi(2)
                                            + -720. * param.s1 * param.s2.powi(3)
                                            + 135. * param.s2.powi(4))
                                    + 2. * param.s12
                                        * (1118. * param.s1.powi(5)
                                            + 1180. * param.s1.powi(4) * param.s2
                                            + -6038. * param.s1.powi(3) * param.s2.powi(2)
                                            + 887. * param.s1.powi(2) * param.s2.powi(3)
                                            + 360. * param.s1 * param.s2.powi(4)
                                            + -27. * param.s2.powi(5))
                                    - (param.s1 - param.s2).powi(2)
                                        * (586. * param.s1.powi(4)
                                            + 4048. * param.s1.powi(3) * param.s2
                                            + 3367. * param.s1.powi(2) * param.s2.powi(2)
                                            + 198. * param.s1 * param.s2.powi(3)
                                            + -9. * param.s2.powi(4)))
                            - param.s12.powi(2)
                                * (2250. * param.s1.powi(5)
                                    + -10180. * param.s1.powi(4) * param.s2
                                    + 3471. * param.s1.powi(3) * param.s2.powi(2)
                                    + 810. * param.s1.powi(2) * param.s2.powi(3)
                                    + 370. * param.s1 * param.s2.powi(4)
                                    + -105. * param.s2.powi(5))
                            - (param.s1 - param.s2).powi(3)
                                * (518. * param.s1.powi(4)
                                    + 2017. * param.s1.powi(3) * param.s2
                                    + 348. * param.s1.powi(2) * param.s2.powi(2)
                                    + -53. * param.s1 * param.s2.powi(3)
                                    + 5. * param.s2.powi(4))
                            - param.s12.powi(5)
                                * (402. * param.s1.powi(2)
                                    + 242. * param.s1 * param.s2
                                    + 105. * param.s2.powi(2)))
                    + 2. * param.m2_2.powi(3)
                        * param.s2.powi(2)
                        * (-5. * param.s12.powi(7)
                            + param.s12.powi(6) * (53. * param.s1 + 35. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (134. * param.s1.powi(3)
                                    + -16. * param.s1.powi(2) * param.s2
                                    + 29. * param.s1 * param.s2.powi(2)
                                    + 35. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (197. * param.s1.powi(4)
                                    + 279. * param.s1.powi(3) * param.s2
                                    + -210. * param.s1.powi(2) * param.s2.powi(2)
                                    + -48. * param.s1 * param.s2.powi(3)
                                    + 35. * param.s2.powi(4))
                            + (param.s1 - param.s2).powi(2)
                                * (68. * param.s1.powi(5)
                                    + 2549. * param.s1.powi(4) * param.s2
                                    + 5036. * param.s1.powi(3) * param.s2.powi(2)
                                    + 599. * param.s1.powi(2) * param.s2.powi(3)
                                    + -67. * param.s1 * param.s2.powi(4)
                                    + 5. * param.s2.powi(5))
                            + param.s12.powi(2)
                                * (825. * param.s1.powi(5)
                                    + 6020. * param.s1.powi(4) * param.s2
                                    + -8709. * param.s1.powi(3) * param.s2.powi(2)
                                    + -90. * param.s1.powi(2) * param.s2.powi(3)
                                    + -505. * param.s1 * param.s2.powi(4)
                                    + 105. * param.s2.powi(5))
                            + param.s12
                                * (-368. * param.s1.powi(6)
                                    + -6805. * param.s1.powi(5) * param.s2
                                    + 7613. * param.s1.powi(4) * param.s2.powi(2)
                                    + 5663. * param.s1.powi(3) * param.s2.powi(3)
                                    + -1360. * param.s1.powi(2) * param.s2.powi(4)
                                    + 332. * param.s1 * param.s2.powi(5)
                                    + -35. * param.s2.powi(6))
                            + -15.
                                * param.m1_2.powi(2)
                                * (-3. * param.s1.powi(5)
                                    + 3. * param.s12.powi(5)
                                    + -208. * param.s1.powi(4) * param.s2
                                    + -1049. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1049. * param.s1.powi(2) * param.s2.powi(3)
                                    + -208. * param.s1 * param.s2.powi(4)
                                    + -3. * param.s2.powi(5)
                                    + -15. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + param.s12.powi(3)
                                        * (30. * param.s1.powi(2)
                                            + 253. * param.s1 * param.s2
                                            + 30. * param.s2.powi(2))
                                    + -3.
                                        * param.s12.powi(2)
                                        * (10. * param.s1.powi(3)
                                            + 223. * param.s1.powi(2) * param.s2
                                            + 223. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    + param.s12
                                        * (15. * param.s1.powi(4)
                                            + 639. * param.s1.powi(3) * param.s2
                                            + 1688. * param.s1.powi(2) * param.s2.powi(2)
                                            + 639. * param.s1 * param.s2.powi(3)
                                            + 15. * param.s2.powi(4)))
                            + -6.
                                * param.m1_2
                                * (18. * param.s1.powi(6)
                                    + 3. * param.s12.powi(6)
                                    + 938. * param.s1.powi(5) * param.s2
                                    + 2406. * param.s1.powi(4) * param.s2.powi(2)
                                    + -1479. * param.s1.powi(3) * param.s2.powi(3)
                                    + -1799. * param.s1.powi(2) * param.s2.powi(4)
                                    + -87. * param.s1 * param.s2.powi(5)
                                    + 3. * param.s2.powi(6)
                                    + -3. * param.s12.powi(5) * (11. * param.s1 + 6. * param.s2)
                                    + 15.
                                        * param.s12.powi(4)
                                        * (8. * param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (42. * param.s1.powi(3)
                                            + 193. * param.s1.powi(2) * param.s2
                                            + -30. * param.s1 * param.s2.powi(2)
                                            + 12. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (195. * param.s1.powi(4)
                                            + 2805. * param.s1.powi(3) * param.s2
                                            + -229. * param.s1.powi(2) * param.s2.powi(2)
                                            + -390. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    - param.s12
                                        * (93. * param.s1.powi(5)
                                            + 2805. * param.s1.powi(4) * param.s2
                                            + 2372. * param.s1.powi(3) * param.s2.powi(2)
                                            + -2873. * param.s1.powi(2) * param.s2.powi(3)
                                            + -315. * param.s1 * param.s2.powi(4)
                                            + 18. * param.s2.powi(5)))
                            - param.s12.powi(5)
                                * (258. * param.s1.powi(2)
                                    + 188. * param.s1 * param.s2
                                    + 105. * param.s2.powi(2)))
                    + param.m2_2
                        * param.s2.powi(4)
                        * (-5. * param.s12.powi(7)
                            + param.s12.powi(6) * (71. * param.s1 + 35. * param.s2)
                            + (param.s1 - param.s2).powi(4)
                                * (511. * param.s1.powi(3)
                                    + 165. * param.s1.powi(2) * param.s2
                                    + -39. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3))
                            + param.s12.powi(4)
                                * (-675. * param.s1.powi(3)
                                    + 865. * param.s1.powi(2) * param.s2
                                    + 415. * param.s1 * param.s2.powi(2)
                                    + 175. * param.s2.powi(3))
                            + 5. * param.s12.powi(3)
                                * (885. * param.s1.powi(4)
                                    + -1522. * param.s1.powi(3) * param.s2
                                    + 120. * param.s1.powi(2) * param.s2.powi(2)
                                    + -24. * param.s1 * param.s2.powi(3)
                                    + -35. * param.s2.powi(4))
                            + param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (539. * param.s1.powi(4)
                                    + 5348. * param.s1.powi(3) * param.s2
                                    + 198. * param.s1.powi(2) * param.s2.powi(2)
                                    + 154. * param.s1 * param.s2.powi(3)
                                    + -35. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-4275. * param.s1.powi(5)
                                    + 4615. * param.s1.powi(4) * param.s2
                                    + 3582. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1080. * param.s1.powi(2) * param.s2.powi(3)
                                    + -235. * param.s1 * param.s2.powi(4)
                                    + 105. * param.s2.powi(5))
                            + -420.
                                * param.m1_2.powi(4)
                                * (-7. * param.s1.powi(3)
                                    + 7. * param.s12.powi(3)
                                    + -38. * param.s1.powi(2) * param.s2
                                    + -38. * param.s1 * param.s2.powi(2)
                                    + -7. * param.s2.powi(3)
                                    + -21. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (21. * param.s1.powi(2)
                                            + 59. * param.s1 * param.s2
                                            + 21. * param.s2.powi(2)))
                            + 20.
                                * param.m1_2.powi(3)
                                * (-451. * param.s1.powi(4)
                                    + 137. * param.s12.powi(4)
                                    + param.s12.powi(3) * (40. * param.s1 + -548. * param.s2)
                                    + -1430. * param.s1.powi(3) * param.s2
                                    + 570. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1174. * param.s1 * param.s2.powi(3)
                                    + 137. * param.s2.powi(4)
                                    + param.s12.powi(2)
                                        * (-942. * param.s1.powi(2)
                                            + 1094. * param.s1 * param.s2
                                            + 822. * param.s2.powi(2))
                                    + 4. * param.s12
                                        * (304. * param.s1.powi(3)
                                            + 221. * param.s1.powi(2) * param.s2
                                            + -577. * param.s1 * param.s2.powi(2)
                                            + -137. * param.s2.powi(3)))
                            + -30.
                                * param.m1_2.powi(2)
                                * (9. * param.s12.powi(5)
                                    + param.s12.powi(4) * (229. * param.s1 + -45. * param.s2)
                                    + param.s12.powi(3)
                                        * (-418. * param.s1.powi(2)
                                            + -337. * param.s1 * param.s2
                                            + 90. * param.s2.powi(2))
                                    + param.s12
                                        * (713. * param.s1.powi(4)
                                            + -1271. * param.s1.powi(3) * param.s2
                                            + -1316. * param.s1.powi(2) * param.s2.powi(2)
                                            + 821. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    - param.s12.powi(2)
                                        * (210. * param.s1.powi(3)
                                            + -1945. * param.s1.powi(2) * param.s2
                                            + 363. * param.s1 * param.s2.powi(2)
                                            + 90. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(2)
                                        * (323. * param.s1.powi(3)
                                            + 938. * param.s1.powi(2) * param.s2
                                            + 368. * param.s1 * param.s2.powi(2)
                                            + 9. * param.s2.powi(3)))
                            + -12.
                                * param.m1_2
                                * (3. * param.s12.powi(6)
                                    + -9. * param.s12.powi(5) * (7. * param.s1 + 2. * param.s2)
                                    + -5.
                                        * param.s12.powi(4)
                                        * (83. * param.s1.powi(2)
                                            + -39. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2))
                                    + 5. * param.s12.powi(3)
                                        * (250. * param.s1.powi(3)
                                            + -151. * param.s1.powi(2) * param.s2
                                            + -30. * param.s1 * param.s2.powi(2)
                                            + -12. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(3)
                                        * (343. * param.s1.powi(3)
                                            + 557. * param.s1.powi(2) * param.s2
                                            + 48. * param.s1 * param.s2.powi(2)
                                            + -3. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (-675. * param.s1.powi(4)
                                            + -1855. * param.s1.powi(3) * param.s2
                                            + 2351. * param.s1.powi(2) * param.s2.powi(2)
                                            + -90. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    - param.s12
                                        * (443. * param.s1.powi(5)
                                            + -2905. * param.s1.powi(4) * param.s2
                                            + 1832. * param.s1.powi(3) * param.s2.powi(2)
                                            + 777. * param.s1.powi(2) * param.s2.powi(3)
                                            + -165. * param.s1 * param.s2.powi(4)
                                            + 18. * param.s2.powi(5)))
                            - param.s12.powi(5)
                                * (591. * param.s1.powi(2)
                                    + 296. * param.s1 * param.s2
                                    + 105. * param.s2.powi(2)))
                    + param.m0_2.powi(4)
                        * (param.m2_2
                            * (-5. * param.s12.powi(7)
                                + param.s12.powi(6) * (35. * param.s1 + 71. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (35. * param.s1.powi(3)
                                        + 83. * param.s1.powi(2) * param.s2
                                        + 173. * param.s1 * param.s2.powi(2)
                                        + -135. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(4)
                                    * (5. * param.s1.powi(3)
                                        + -39. * param.s1.powi(2) * param.s2
                                        + 165. * param.s1 * param.s2.powi(2)
                                        + 511. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + 24. * param.s1.powi(3) * param.s2
                                        + -120. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1522. * param.s1 * param.s2.powi(3)
                                        + -885. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (105. * param.s1.powi(5)
                                        + -235. * param.s1.powi(4) * param.s2
                                        + -1080. * param.s1.powi(3) * param.s2.powi(2)
                                        + 3582. * param.s1.powi(2) * param.s2.powi(3)
                                        + 4615. * param.s1 * param.s2.powi(4)
                                        + -4275. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (35. * param.s1.powi(4)
                                        + -154. * param.s1.powi(3) * param.s2
                                        + -198. * param.s1.powi(2) * param.s2.powi(2)
                                        + -5348. * param.s1 * param.s2.powi(3)
                                        + -539. * param.s2.powi(4))
                                - param.s12.powi(5)
                                    * (105. * param.s1.powi(2)
                                        + 296. * param.s1 * param.s2
                                        + 591. * param.s2.powi(2)))
                            + param.s2
                                * (-8. * param.s12.powi(7)
                                    + param.s12.powi(6) * (47. * param.s1 + 182. * param.s2)
                                    + -2.
                                        * param.s12.powi(5)
                                        * (57. * param.s1.powi(2)
                                            + 280. * param.s1 * param.s2
                                            + -588. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (29. * param.s1.powi(3)
                                            + 86. * param.s1.powi(2) * param.s2
                                            + 461. * param.s1 * param.s2.powi(2)
                                            + -690. * param.s2.powi(3))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (param.s1.powi(3)
                                            + -61. * param.s1.powi(2) * param.s2
                                            + -889. * param.s1 * param.s2.powi(2)
                                            + -506. * param.s2.powi(3))
                                    + -10.
                                        * param.s12.powi(3)
                                        * (10. * param.s1.powi(4)
                                            + -24. * param.s1.powi(3) * param.s2
                                            + 675. * param.s1.powi(2) * param.s2.powi(2)
                                            + -553. * param.s1 * param.s2.powi(3)
                                            + -162. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (33. * param.s1.powi(5)
                                            + -430. * param.s1.powi(4) * param.s2
                                            + 1872. * param.s1.powi(3) * param.s2.powi(2)
                                            + 5712. * param.s1.powi(2) * param.s2.powi(3)
                                            + -8705. * param.s1 * param.s2.powi(4)
                                            + 1518. * param.s2.powi(5))
                                    + 3. * param.m1_2
                                        * (3. * param.s12.powi(6)
                                            + -6.
                                                * param.s12.powi(5)
                                                * (3. * param.s1 + 13. * param.s2)
                                            + 5. * param.s12.powi(4)
                                                * (9. * param.s1.powi(2)
                                                    + 54. * param.s1 * param.s2
                                                    + -205. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (3. * param.s1.powi(2)
                                                    + -30. * param.s1 * param.s2
                                                    + -187. * param.s2.powi(2))
                                            + -60.
                                                * param.s12.powi(3)
                                                * (param.s1.powi(3)
                                                    + 5. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + -15. * param.s2.powi(3))
                                            + -2.
                                                * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (9. * param.s1.powi(3)
                                                    + -27. * param.s1.powi(2) * param.s2
                                                    + 553. * param.s1 * param.s2.powi(2)
                                                    + 499. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (45. * param.s1.powi(4)
                                                    + 60. * param.s1.powi(3) * param.s2
                                                    + 1586.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -3980. * param.s1 * param.s2.powi(3)
                                                    + 1385. * param.s2.powi(4)))
                                    - (param.s1 - param.s2).powi(5)
                                        * (param.s1.powi(2)
                                            + -5. * param.s1 * param.s2
                                            + -26. * param.s2.powi(2))))
                    + param.m0_2.powi(3)
                        * (2.
                            * param.m2_2.powi(2)
                            * (5. * param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (105. * param.s1.powi(2)
                                        + 242. * param.s1 * param.s2
                                        + 402. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (35. * param.s1.powi(3)
                                        + 56. * param.s1.powi(2) * param.s2
                                        + 56. * param.s1 * param.s2.powi(2)
                                        + 114. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + -12. * param.s1.powi(3) * param.s2
                                        + -210. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1069. * param.s1 * param.s2.powi(3)
                                        + -135. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-105. * param.s1.powi(5)
                                        + 370. * param.s1.powi(4) * param.s2
                                        + 810. * param.s1.powi(3) * param.s2.powi(2)
                                        + 3471. * param.s1.powi(2) * param.s2.powi(3)
                                        + -10180. * param.s1 * param.s2.powi(4)
                                        + 2250. * param.s2.powi(5))
                                + param.s12
                                    * (35. * param.s1.powi(6)
                                        + -278. * param.s1.powi(5) * param.s2
                                        + 640. * param.s1.powi(4) * param.s2.powi(2)
                                        + -7437. * param.s1.powi(3) * param.s2.powi(3)
                                        + 4463. * param.s1.powi(2) * param.s2.powi(4)
                                        + 4445. * param.s1 * param.s2.powi(5)
                                        + -1868. * param.s2.powi(6))
                                - (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1.powi(4)
                                        + -53. * param.s1.powi(3) * param.s2
                                        + 348. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2017. * param.s1 * param.s2.powi(3)
                                        + 518. * param.s2.powi(4))
                                - param.s12.powi(6) * (35. * param.s1 + 62. * param.s2))
                            + param.s2.powi(2)
                                * (37. * param.s12.powi(7)
                                    + param.s12.powi(6) * (-133. * param.s1 + 527. * param.s2)
                                    + param.s12.powi(5)
                                        * (111. * param.s1.powi(2)
                                            + 1870. * param.s1 * param.s2
                                            + -1839. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (29. * param.s1.powi(3)
                                            + -1399. * param.s1.powi(2) * param.s2
                                            + 1157. * param.s1 * param.s2.powi(2)
                                            + 273. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (65. * param.s1.powi(4)
                                            + -906. * param.s1.powi(3) * param.s2
                                            + -1878. * param.s1.powi(2) * param.s2.powi(2)
                                            + 3010. * param.s1 * param.s2.powi(3)
                                            + -219. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (213. * param.s1.powi(5)
                                            + 1775. * param.s1.powi(4) * param.s2
                                            + -15408. * param.s1.powi(3) * param.s2.powi(2)
                                            + 9312. * param.s1.powi(2) * param.s2.powi(3)
                                            + 5875. * param.s1 * param.s2.powi(4)
                                            + -1767. * param.s2.powi(5))
                                    + 30.
                                        * param.m1_2.powi(2)
                                        * (3. * param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (-15. * param.s1 + 122. * param.s2)
                                            + 5. * param.s12.powi(3)
                                                * (6. * param.s1.powi(2)
                                                    + -59. * param.s1 * param.s2
                                                    + 14. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (-30. * param.s1.powi(3)
                                                    + 153. * param.s1.powi(2) * param.s2
                                                    + 425. * param.s1 * param.s2.powi(2)
                                                    + -412. * param.s2.powi(3))
                                            + param.s12
                                                * (15. * param.s1.powi(4)
                                                    + 91. * param.s1.powi(3) * param.s2
                                                    + -620.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 403. * param.s1 * param.s2.powi(3)
                                                    + 111. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (3. * param.s1.powi(2)
                                                    + 80. * param.s1 * param.s2
                                                    + 106. * param.s2.powi(2)))
                                    + -6.
                                        * param.m1_2
                                        * (21. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-96. * param.s1 + 574. * param.s2)
                                            + 5. * param.s12.powi(4)
                                                * (33. * param.s1.powi(2)
                                                    + -110. * param.s1 * param.s2
                                                    + -149. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (12. * param.s1.powi(3)
                                                    + -286. * param.s1.powi(2) * param.s2
                                                    + -1541. * param.s1 * param.s2.powi(2)
                                                    + -253. * param.s2.powi(3))
                                            + -10.
                                                * param.s12.powi(3)
                                                * (12. * param.s1.powi(3)
                                                    + 161. * param.s1.powi(2) * param.s2
                                                    + -523. * param.s1 * param.s2.powi(2)
                                                    + 122. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (15. * param.s1.powi(4)
                                                    + 2390. * param.s1.powi(3) * param.s2
                                                    + -3198.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -3090. * param.s1 * param.s2.powi(3)
                                                    + 2075. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(4)
                                                * (9. * param.s1.powi(2)
                                                    + 220. * param.s1 * param.s2
                                                    + 199. * param.s2.powi(2)))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (47. * param.s1.powi(3)
                                            + 1813. * param.s1.powi(2) * param.s2
                                            + 3397. * param.s1 * param.s2.powi(2)
                                            + 563. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(5)
                                        * (param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + 19. * param.s2.powi(2)))
                            + param.m2_2
                                * param.s2
                                * (19. * param.s12.powi(7)
                                    + param.s12.powi(5)
                                        * (183. * param.s1.powi(2)
                                            + 574. * param.s1 * param.s2
                                            + -675. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(4)
                                        * (25. * param.s1.powi(3)
                                            + -185. * param.s1.powi(2) * param.s2
                                            + 2485. * param.s1 * param.s2.powi(2)
                                            + -1041. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(4)
                                        * (17. * param.s1.powi(3)
                                            + -201. * param.s1.powi(2) * param.s2
                                            + -1779. * param.s1 * param.s2.powi(2)
                                            + -605. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (11. * param.s1.powi(4)
                                            + 492. * param.s1.powi(3) * param.s2
                                            + -2670. * param.s1.powi(2) * param.s2.powi(2)
                                            + -1976. * param.s1 * param.s2.powi(3)
                                            + 1551. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (141. * param.s1.powi(5)
                                            + 1325. * param.s1.powi(4) * param.s2
                                            + 12402. * param.s1.powi(3) * param.s2.powi(2)
                                            + -44682. * param.s1.powi(2) * param.s2.powi(3)
                                            + 16345. * param.s1 * param.s2.powi(4)
                                            + 3621. * param.s2.powi(5))
                                    + -12.
                                        * param.m1_2
                                        * (3. * param.s12.powi(6)
                                            + -9.
                                                * param.s12.powi(5)
                                                * (2. * param.s1 + 7. * param.s2)
                                            + 5. * param.s12.powi(4)
                                                * (9. * param.s1.powi(2)
                                                    + 39. * param.s1 * param.s2
                                                    + -83. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (3. * param.s1.powi(3)
                                                    + -48. * param.s1.powi(2) * param.s2
                                                    + -557. * param.s1 * param.s2.powi(2)
                                                    + -343. * param.s2.powi(3))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (12. * param.s1.powi(3)
                                                    + 30. * param.s1.powi(2) * param.s2
                                                    + 151. * param.s1 * param.s2.powi(2)
                                                    + -250. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (45. * param.s1.powi(4)
                                                    + -90. * param.s1.powi(3) * param.s2
                                                    + 2351.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -1855. * param.s1 * param.s2.powi(3)
                                                    + -675. * param.s2.powi(4))
                                            - param.s12
                                                * (18. * param.s1.powi(5)
                                                    + -165. * param.s1.powi(4) * param.s2
                                                    + 777.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 1832.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -2905. * param.s1 * param.s2.powi(4)
                                                    + 443. * param.s2.powi(5)))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (83. * param.s1.powi(4)
                                            + -88. * param.s1.powi(3) * param.s2
                                            + 11520. * param.s1.powi(2) * param.s2.powi(2)
                                            + 13840. * param.s1 * param.s2.powi(3)
                                            + -539. * param.s2.powi(4))
                                    - param.s12.powi(6) * (97. * param.s1 + 349. * param.s2)))
                    + param.m0_2
                        * (param.m2_2.powi(4)
                            * (-5. * param.s1.powi(7)
                                + 5. * param.s12.powi(7)
                                + 86. * param.s1.powi(6) * param.s2
                                + -999. * param.s1.powi(5) * param.s2.powi(2)
                                + -9168. * param.s1.powi(4) * param.s2.powi(3)
                                + 2487. * param.s1.powi(3) * param.s2.powi(4)
                                + 7212. * param.s1.powi(2) * param.s2.powi(5)
                                + 401. * param.s1 * param.s2.powi(6)
                                + -14. * param.s2.powi(7)
                                + param.s12.powi(5)
                                    * (105. * param.s1.powi(2)
                                        + 134. * param.s1 * param.s2
                                        + 159. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (35. * param.s1.powi(3)
                                        + 2. * param.s1.powi(2) * param.s2
                                        + -43. * param.s1 * param.s2.powi(2)
                                        + 62. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + -84. * param.s1.powi(3) * param.s2
                                        + -120. * param.s1.powi(2) * param.s2.powi(2)
                                        + -300. * param.s1 * param.s2.powi(3)
                                        + 71. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-105. * param.s1.powi(5)
                                        + 640. * param.s1.powi(4) * param.s2
                                        + -1080. * param.s1.powi(3) * param.s2.powi(2)
                                        + 8022. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2395. * param.s1 * param.s2.powi(4)
                                        + -240. * param.s2.powi(5))
                                + param.s12
                                    * (35. * param.s1.powi(6)
                                        + -386. * param.s1.powi(5) * param.s2
                                        + 2305. * param.s1.powi(4) * param.s2.powi(2)
                                        + 2956. * param.s1.powi(3) * param.s2.powi(3)
                                        + -14729. * param.s1.powi(2) * param.s2.powi(4)
                                        + -1610. * param.s1 * param.s2.powi(5)
                                        + 89. * param.s2.powi(6))
                                - param.s12.powi(6) * (35. * param.s1 + 44. * param.s2))
                            + param.s2.powi(4)
                                * (-8. * param.s12.powi(7)
                                    + param.s12.powi(6) * (182. * param.s1 + 47. * param.s2)
                                    + 2. * param.s12.powi(5)
                                        * (588. * param.s1.powi(2)
                                            + -280. * param.s1 * param.s2
                                            + -57. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(4)
                                        * (690. * param.s1.powi(3)
                                            + -461. * param.s1.powi(2) * param.s2
                                            + -86. * param.s1 * param.s2.powi(2)
                                            + -29. * param.s2.powi(3))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (506. * param.s1.powi(3)
                                            + 889. * param.s1.powi(2) * param.s2
                                            + 61. * param.s1 * param.s2.powi(2)
                                            - param.s2.powi(3))
                                    + 10.
                                        * param.s12.powi(3)
                                        * (162. * param.s1.powi(4)
                                            + 553. * param.s1.powi(3) * param.s2
                                            + -675. * param.s1.powi(2) * param.s2.powi(2)
                                            + 24. * param.s1 * param.s2.powi(3)
                                            + -10. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (1518. * param.s1.powi(5)
                                            + -8705. * param.s1.powi(4) * param.s2
                                            + 5712. * param.s1.powi(3) * param.s2.powi(2)
                                            + 1872. * param.s1.powi(2) * param.s2.powi(3)
                                            + -430. * param.s1 * param.s2.powi(4)
                                            + 33. * param.s2.powi(5))
                                    + 420.
                                        * param.m1_2.powi(4)
                                        * (-7. * param.s1.powi(3)
                                            + 7. * param.s12.powi(3)
                                            + -18. * param.s1.powi(2) * param.s2
                                            + 12. * param.s1 * param.s2.powi(2)
                                            + 13. * param.s2.powi(3)
                                            + param.s12
                                                * (21. * param.s1.powi(2)
                                                    + 19. * param.s1 * param.s2
                                                    + -19. * param.s2.powi(2))
                                            - param.s12.powi(2) * (21. * param.s1 + param.s2))
                                    + -40.
                                        * param.m1_2.powi(3)
                                        * (122. * param.s12.powi(4)
                                            + -194. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + param.s12.powi(2)
                                                * (-150. * param.s1.powi(2)
                                                    + 971. * param.s1 * param.s2
                                                    + -150. * param.s2.powi(2))
                                            + param.s12
                                                * (394. * param.s1.powi(3)
                                                    + -646. * param.s1.powi(2) * param.s2
                                                    + -646. * param.s1 * param.s2.powi(2)
                                                    + 394. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(2)
                                                * (172. * param.s1.powi(2)
                                                    + 475. * param.s1 * param.s2
                                                    + 172. * param.s2.powi(2)))
                                    + 30.
                                        * param.m1_2.powi(2)
                                        * (70. * param.s12.powi(5)
                                            + 3. * param.s12.powi(4)
                                                * (46. * param.s1 + -71. * param.s2)
                                            + param.s12.powi(3)
                                                * (-664. * param.s1.powi(2)
                                                    + 671. * param.s1 * param.s2
                                                    + 152. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (464. * param.s1.powi(3)
                                                    + 905. * param.s1.powi(2) * param.s2
                                                    + -1627. * param.s1 * param.s2.powi(2)
                                                    + 122. * param.s2.powi(3))
                                            + param.s12
                                                * (162. * param.s1.powi(4)
                                                    + -1543. * param.s1.powi(3) * param.s2
                                                    + 890.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 689. * param.s1 * param.s2.powi(3)
                                                    + -198. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (170. * param.s1.powi(2)
                                                    + 330. * param.s1 * param.s2
                                                    + 67. * param.s2.powi(2)))
                                    + -6.
                                        * param.m1_2
                                        * (21. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (574. * param.s1 + -96. * param.s2)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (149. * param.s1.powi(2)
                                                    + 110. * param.s1 * param.s2
                                                    + -33. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (253. * param.s1.powi(3)
                                                    + 1541. * param.s1.powi(2) * param.s2
                                                    + 286. * param.s1 * param.s2.powi(2)
                                                    + -12. * param.s2.powi(3))
                                            + -10.
                                                * param.s12.powi(3)
                                                * (122. * param.s1.powi(3)
                                                    + -523. * param.s1.powi(2) * param.s2
                                                    + 161. * param.s1 * param.s2.powi(2)
                                                    + 12. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (2075. * param.s1.powi(4)
                                                    + -3090. * param.s1.powi(3) * param.s2
                                                    + -3198.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2390. * param.s1 * param.s2.powi(3)
                                                    + 15. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(4)
                                                * (199. * param.s1.powi(2)
                                                    + 220. * param.s1 * param.s2
                                                    + 9. * param.s2.powi(2)))
                                    - (param.s1 - param.s2).powi(5)
                                        * (26. * param.s1.powi(2) + 5. * param.s1 * param.s2
                                            - param.s2.powi(2)))
                            + param.m2_2.powi(3)
                                * param.s2
                                * (-7. * param.s12.powi(7)
                                    + 85. * param.s12.powi(6) * (param.s1 + param.s2)
                                    + -11.
                                        * param.s12.powi(5)
                                        * (33. * param.s1.powi(2)
                                            + 74. * param.s1 * param.s2
                                            + 33. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (157. * param.s1.powi(3)
                                            + 271. * param.s1.powi(2) * param.s2
                                            + 271. * param.s1 * param.s2.powi(2)
                                            + 157. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (193. * param.s1.powi(4)
                                            + -204. * param.s1.powi(3) * param.s2
                                            + 3450. * param.s1.powi(2) * param.s2.powi(2)
                                            + -204. * param.s1 * param.s2.powi(3)
                                            + 193. * param.s2.powi(4))
                                    + (param.s1 - param.s2).powi(2)
                                        * (43. * param.s1.powi(5)
                                            + -965. * param.s1.powi(4) * param.s2
                                            + -15458. * param.s1.powi(3) * param.s2.powi(2)
                                            + -15458. * param.s1.powi(2) * param.s2.powi(3)
                                            + -965. * param.s1 * param.s2.powi(4)
                                            + 43. * param.s2.powi(5))
                                    + param.s12.powi(2)
                                        * (687. * param.s1.powi(5)
                                            + -4325. * param.s1.powi(4) * param.s2
                                            + 18402. * param.s1.powi(3) * param.s2.powi(2)
                                            + 18402. * param.s1.powi(2) * param.s2.powi(3)
                                            + -4325. * param.s1 * param.s2.powi(4)
                                            + 687. * param.s2.powi(5))
                                    + param.s12
                                        * (-265. * param.s1.powi(6)
                                            + 3730. * param.s1.powi(5) * param.s2
                                            + 11341. * param.s1.powi(4) * param.s2.powi(2)
                                            + -49772. * param.s1.powi(3) * param.s2.powi(3)
                                            + 11341. * param.s1.powi(2) * param.s2.powi(4)
                                            + 3730. * param.s1 * param.s2.powi(5)
                                            + -265. * param.s2.powi(6))
                                    + -12.
                                        * param.m1_2
                                        * (3. * param.s1.powi(6)
                                            + 3. * param.s12.powi(6)
                                            + -87. * param.s1.powi(5) * param.s2
                                            + -1799. * param.s1.powi(4) * param.s2.powi(2)
                                            + -1479. * param.s1.powi(3) * param.s2.powi(3)
                                            + 2406. * param.s1.powi(2) * param.s2.powi(4)
                                            + 938. * param.s1 * param.s2.powi(5)
                                            + 18. * param.s2.powi(6)
                                            + -3.
                                                * param.s12.powi(5)
                                                * (6. * param.s1 + 11. * param.s2)
                                            + 15.
                                                * param.s12.powi(4)
                                                * (3. * param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + 8. * param.s2.powi(2))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (12. * param.s1.powi(3)
                                                    + -30. * param.s1.powi(2) * param.s2
                                                    + 193. * param.s1 * param.s2.powi(2)
                                                    + 42. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (45. * param.s1.powi(4)
                                                    + -390. * param.s1.powi(3) * param.s2
                                                    + -229.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2805. * param.s1 * param.s2.powi(3)
                                                    + 195. * param.s2.powi(4))
                                            - param.s12
                                                * (18. * param.s1.powi(5)
                                                    + -315. * param.s1.powi(4) * param.s2
                                                    + -2873.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 2372.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 2805. * param.s1 * param.s2.powi(4)
                                                    + 93. * param.s2.powi(5))))
                            + param.m2_2
                                * param.s2.powi(3)
                                * (19. * param.s12.powi(7)
                                    + param.s12.powi(5)
                                        * (-675. * param.s1.powi(2)
                                            + 574. * param.s1 * param.s2
                                            + 183. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (1041. * param.s1.powi(3)
                                            + -2485. * param.s1.powi(2) * param.s2
                                            + 185. * param.s1 * param.s2.powi(2)
                                            + -25. * param.s2.powi(3))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (539. * param.s1.powi(4)
                                            + -13840. * param.s1.powi(3) * param.s2
                                            + -11520. * param.s1.powi(2) * param.s2.powi(2)
                                            + 88. * param.s1 * param.s2.powi(3)
                                            + -83. * param.s2.powi(4))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (1551. * param.s1.powi(4)
                                            + -1976. * param.s1.powi(3) * param.s2
                                            + -2670. * param.s1.powi(2) * param.s2.powi(2)
                                            + 492. * param.s1 * param.s2.powi(3)
                                            + 11. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (3621. * param.s1.powi(5)
                                            + 16345. * param.s1.powi(4) * param.s2
                                            + -44682. * param.s1.powi(3) * param.s2.powi(2)
                                            + 12402. * param.s1.powi(2) * param.s2.powi(3)
                                            + 1325. * param.s1 * param.s2.powi(4)
                                            + 141. * param.s2.powi(5))
                                    + 20.
                                        * param.m1_2.powi(3)
                                        * (137. * param.s1.powi(4)
                                            + 137. * param.s12.powi(4)
                                            + 1174. * param.s1.powi(3) * param.s2
                                            + 570. * param.s1.powi(2) * param.s2.powi(2)
                                            + -1430. * param.s1 * param.s2.powi(3)
                                            + -451. * param.s2.powi(4)
                                            + param.s12.powi(3)
                                                * (-548. * param.s1 + 40. * param.s2)
                                            + 2. * param.s12.powi(2)
                                                * (411. * param.s1.powi(2)
                                                    + 547. * param.s1 * param.s2
                                                    + -471. * param.s2.powi(2))
                                            + -4.
                                                * param.s12
                                                * (137. * param.s1.powi(3)
                                                    + 577. * param.s1.powi(2) * param.s2
                                                    + -221. * param.s1 * param.s2.powi(2)
                                                    + -304. * param.s2.powi(3)))
                                    + -30.
                                        * param.m1_2.powi(2)
                                        * (79. * param.s12.powi(5)
                                            + -121. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12.powi(3)
                                                * (153. * param.s1.powi(2)
                                                    + -869. * param.s1 * param.s2
                                                    + 153. * param.s2.powi(2))
                                            + 39.
                                                * (param.s1 - param.s2).powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 37. * param.s1.powi(2) * param.s2
                                                    + 37. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (854. * param.s1.powi(3)
                                                    + -2060. * param.s1.powi(2) * param.s2
                                                    + -2060. * param.s1 * param.s2.powi(2)
                                                    + 854. * param.s2.powi(3))
                                            - param.s12
                                                * (701. * param.s1.powi(4)
                                                    + 610. * param.s1.powi(3) * param.s2
                                                    + -4638.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 610. * param.s1 * param.s2.powi(3)
                                                    + 701. * param.s2.powi(4)))
                                    + 12.
                                        * param.m1_2
                                        * (18. * param.s12.powi(6)
                                            + 7. * param.s12.powi(5)
                                                * (41. * param.s1 + -9. * param.s2)
                                            + -15.
                                                * param.s12.powi(4)
                                                * (68. * param.s1.powi(2)
                                                    + -62. * param.s1 * param.s2
                                                    + -3. * param.s2.powi(2))
                                            + 5. * param.s12.powi(3)
                                                * (170. * param.s1.powi(3)
                                                    + 596. * param.s1.powi(2) * param.s2
                                                    + -739. * param.s1 * param.s2.powi(2)
                                                    + 18. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (308. * param.s1.powi(3)
                                                    + 1602. * param.s1.powi(2) * param.s2
                                                    + 898. * param.s1 * param.s2.powi(2)
                                                    + 27. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (430. * param.s1.powi(4)
                                                    + -7820. * param.s1.powi(3) * param.s2
                                                    + 4711.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2635. * param.s1 * param.s2.powi(3)
                                                    + -180. * param.s2.powi(4))
                                            + param.s12
                                                * (-873. * param.s1.powi(5)
                                                    + 3295. * param.s1.powi(4) * param.s2
                                                    + 4483.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -7682.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 660. * param.s1 * param.s2.powi(4)
                                                    + 117. * param.s2.powi(5)))
                                    - (param.s1 - param.s2).powi(4)
                                        * (605. * param.s1.powi(3)
                                            + 1779. * param.s1.powi(2) * param.s2
                                            + 201. * param.s1 * param.s2.powi(2)
                                            + -17. * param.s2.powi(3))
                                    - param.s12.powi(6) * (349. * param.s1 + 97. * param.s2))
                            + 3. * param.m2_2.powi(2)
                                * param.s2.powi(2)
                                * (10.
                                    * param.m1_2.powi(2)
                                    * (-9. * param.s1.powi(5)
                                        + 9. * param.s12.powi(5)
                                        + -487. * param.s1.powi(4) * param.s2
                                        + -1385. * param.s1.powi(3) * param.s2.powi(2)
                                        + 615. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1138. * param.s1 * param.s2.powi(4)
                                        + 128. * param.s2.powi(5)
                                        + param.s12.powi(4)
                                            * (-45. * param.s1 + 92. * param.s2)
                                        + param.s12.powi(3)
                                            * (90. * param.s1.powi(2)
                                                + 211. * param.s1 * param.s2
                                                + -458. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-90. * param.s1.powi(3)
                                                + -1185. * param.s1.powi(2) * param.s2
                                                + 851. * param.s1 * param.s2.powi(2)
                                                + 732. * param.s2.powi(3))
                                        + param.s12
                                            * (45. * param.s1.powi(4)
                                                + 1369. * param.s1.powi(3) * param.s2
                                                + 992. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2155. * param.s1 * param.s2.powi(3)
                                                + -503. * param.s2.powi(4)))
                                    + -2.
                                        * param.m1_2
                                        * (9. * param.s12.powi(6)
                                            + 36. * param.s12.powi(5) * (param.s1 + param.s2)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (63. * param.s1.powi(2)
                                                    + -464. * param.s1 * param.s2
                                                    + 63. * param.s2.powi(2))
                                            + 10.
                                                * param.s12.powi(3)
                                                * (72. * param.s1.powi(3)
                                                    + -385. * param.s1.powi(2) * param.s2
                                                    + -385. * param.s1 * param.s2.powi(2)
                                                    + 72. * param.s2.powi(3))
                                            + 2. * param.s12
                                                * (198. * param.s1.powi(5)
                                                    + 3775. * param.s1.powi(4) * param.s2
                                                    + -6493.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -6493.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 3775. * param.s1 * param.s2.powi(4)
                                                    + 198. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (765. * param.s1.powi(4)
                                                    + 2730. * param.s1.powi(3) * param.s2
                                                    + -19498.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 2730. * param.s1 * param.s2.powi(3)
                                                    + 765. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(2)
                                                * (81. * param.s1.powi(4)
                                                    + 3488. * param.s1.powi(3) * param.s2
                                                    + 9242.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 3488. * param.s1 * param.s2.powi(3)
                                                    + 81. * param.s2.powi(4)))
                                    + -3.
                                        * (param.s12.powi(7)
                                            + param.s12.powi(5)
                                                * (27. * param.s1.powi(2)
                                                    + -74. * param.s1 * param.s2
                                                    + -15. * param.s2.powi(2))
                                            + 5. * param.s12.powi(4)
                                                * (5. * param.s1.powi(3)
                                                    + -195. * param.s1.powi(2) * param.s2
                                                    + 65. * param.s1 * param.s2.powi(2)
                                                    + 11. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (23. * param.s1.powi(4)
                                                    + 718. * param.s1.powi(3) * param.s2
                                                    + 1068.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 86. * param.s1 * param.s2.powi(3)
                                                    + -5. * param.s2.powi(4))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (29. * param.s1.powi(4)
                                                    + -546. * param.s1.powi(3) * param.s2
                                                    + 250.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 60. * param.s1 * param.s2.powi(3)
                                                    + 17. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (189. * param.s1.powi(5)
                                                    + -1585. * param.s1.powi(4) * param.s2
                                                    + -3832.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 4522.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -115. * param.s1 * param.s2.powi(4)
                                                    + 69. * param.s2.powi(5))
                                            - param.s12
                                                * (107. * param.s1.powi(6)
                                                    + 744. * param.s1.powi(5) * param.s2
                                                    + -5789.
                                                        * param.s1.powi(4)
                                                        * param.s2.powi(2)
                                                    + 3658.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(3)
                                                    + 1529.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(4)
                                                    + -278. * param.s1 * param.s2.powi(5)
                                                    + 29. * param.s2.powi(6))
                                            - param.s12.powi(6) * (13. * param.s1 + param.s2))))
                    + param.m0_2.powi(2)
                        * (-2.
                            * param.m2_2.powi(3)
                            * (5. * param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (105. * param.s1.powi(2)
                                        + 188. * param.s1 * param.s2
                                        + 258. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (35. * param.s1.powi(3)
                                        + 29. * param.s1.powi(2) * param.s2
                                        + -16. * param.s1 * param.s2.powi(2)
                                        + 134. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + -48. * param.s1.powi(3) * param.s2
                                        + -210. * param.s1.powi(2) * param.s2.powi(2)
                                        + 279. * param.s1 * param.s2.powi(3)
                                        + 197. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-105. * param.s1.powi(5)
                                        + 505. * param.s1.powi(4) * param.s2
                                        + 90. * param.s1.powi(3) * param.s2.powi(2)
                                        + 8709. * param.s1.powi(2) * param.s2.powi(3)
                                        + -6020. * param.s1 * param.s2.powi(4)
                                        + -825. * param.s2.powi(5))
                                + param.s12
                                    * (35. * param.s1.powi(6)
                                        + -332. * param.s1.powi(5) * param.s2
                                        + 1360. * param.s1.powi(4) * param.s2.powi(2)
                                        + -5663. * param.s1.powi(3) * param.s2.powi(3)
                                        + -7613. * param.s1.powi(2) * param.s2.powi(4)
                                        + 6805. * param.s1 * param.s2.powi(5)
                                        + 368. * param.s2.powi(6))
                                - (param.s1 - param.s2).powi(2)
                                    * (5. * param.s1.powi(5)
                                        + -67. * param.s1.powi(4) * param.s2
                                        + 599. * param.s1.powi(3) * param.s2.powi(2)
                                        + 5036. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2549. * param.s1 * param.s2.powi(4)
                                        + 68. * param.s2.powi(5))
                                - param.s12.powi(6) * (35. * param.s1 + 53. * param.s2))
                            + param.s2.powi(3)
                                * (37. * param.s12.powi(7)
                                    + param.s12.powi(6) * (527. * param.s1 + -133. * param.s2)
                                    + (param.s1 - param.s2).powi(5)
                                        * (19. * param.s1.powi(2)
                                            + 40. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (-1839. * param.s1.powi(2)
                                            + 1870. * param.s1 * param.s2
                                            + 111. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (273. * param.s1.powi(3)
                                            + 1157. * param.s1.powi(2) * param.s2
                                            + -1399. * param.s1 * param.s2.powi(2)
                                            + 29. * param.s2.powi(3))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (563. * param.s1.powi(3)
                                            + 3397. * param.s1.powi(2) * param.s2
                                            + 1813. * param.s1 * param.s2.powi(2)
                                            + 47. * param.s2.powi(3))
                                    + 5. * param.s12.powi(3)
                                        * (219. * param.s1.powi(4)
                                            + -3010. * param.s1.powi(3) * param.s2
                                            + 1878. * param.s1.powi(2) * param.s2.powi(2)
                                            + 906. * param.s1 * param.s2.powi(3)
                                            + -65. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (-1767. * param.s1.powi(5)
                                            + 5875. * param.s1.powi(4) * param.s2
                                            + 9312. * param.s1.powi(3) * param.s2.powi(2)
                                            + -15408. * param.s1.powi(2) * param.s2.powi(3)
                                            + 1775. * param.s1 * param.s2.powi(4)
                                            + 213. * param.s2.powi(5))
                                    + -10.
                                        * param.m1_2.powi(3)
                                        * (137. * param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (-548. * param.s1 + 628. * param.s2)
                                            + 2. * param.s12.powi(2)
                                                * (411. * param.s1.powi(2)
                                                    + -335. * param.s1 * param.s2
                                                    + -513. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(2)
                                                * (137. * param.s1.powi(2)
                                                    + 860. * param.s1 * param.s2
                                                    + 641. * param.s2.powi(2))
                                            + -4.
                                                * param.s12
                                                * (137. * param.s1.powi(3)
                                                    + 136. * param.s1.powi(2) * param.s2
                                                    + -620. * param.s1 * param.s2.powi(2)
                                                    + 95. * param.s2.powi(3)))
                                    + 30.
                                        * param.m1_2.powi(2)
                                        * (70. * param.s12.powi(5)
                                            + -3.
                                                * param.s12.powi(4)
                                                * (71. * param.s1 + -46. * param.s2)
                                            + param.s12.powi(3)
                                                * (152. * param.s1.powi(2)
                                                    + 671. * param.s1 * param.s2
                                                    + -664. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (67. * param.s1.powi(2)
                                                    + 330. * param.s1 * param.s2
                                                    + 170. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (122. * param.s1.powi(3)
                                                    + -1627. * param.s1.powi(2) * param.s2
                                                    + 905. * param.s1 * param.s2.powi(2)
                                                    + 464. * param.s2.powi(3))
                                            + param.s12
                                                * (-198. * param.s1.powi(4)
                                                    + 689. * param.s1.powi(3) * param.s2
                                                    + 890.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -1543. * param.s1 * param.s2.powi(3)
                                                    + 162. * param.s2.powi(4)))
                                    + -6.
                                        * param.m1_2
                                        * (131. * param.s12.powi(6)
                                            + -86. * param.s12.powi(5) * (param.s1 + param.s2)
                                            + 2. * (param.s1 - param.s2).powi(4)
                                                * (58. * param.s1.powi(2)
                                                    + 205. * param.s1 * param.s2
                                                    + 58. * param.s2.powi(2))
                                            + -10.
                                                * param.s12.powi(4)
                                                * (85. * param.s1.powi(2)
                                                    + -331. * param.s1 * param.s2
                                                    + 85. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (13. * param.s1.powi(3)
                                                    + -1564. * param.s1.powi(2) * param.s2
                                                    + -1564. * param.s1 * param.s2.powi(2)
                                                    + 13. * param.s2.powi(3))
                                            + 10.
                                                * param.s12.powi(3)
                                                * (164. * param.s1.powi(3)
                                                    + -329. * param.s1.powi(2) * param.s2
                                                    + -329. * param.s1 * param.s2.powi(2)
                                                    + 164. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (925. * param.s1.powi(4)
                                                    + 3060. * param.s1.powi(3) * param.s2
                                                    + -10682.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 3060. * param.s1 * param.s2.powi(3)
                                                    + 925. * param.s2.powi(4))))
                            + 6. * param.m2_2
                                * param.s2.powi(2)
                                * (-5.
                                    * param.m1_2.powi(2)
                                    * (9. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-45. * param.s1 + 229. * param.s2)
                                        + param.s12.powi(3)
                                            * (90. * param.s1.powi(2)
                                                + -337. * param.s1 * param.s2
                                                + -418. * param.s2.powi(2))
                                        + param.s12
                                            * (45. * param.s1.powi(4)
                                                + 821. * param.s1.powi(3) * param.s2
                                                + -1316. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1271. * param.s1 * param.s2.powi(3)
                                                + 713. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(2)
                                            * (9. * param.s1.powi(3)
                                                + 368. * param.s1.powi(2) * param.s2
                                                + 938. * param.s1 * param.s2.powi(2)
                                                + 323. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (90. * param.s1.powi(3)
                                                + 363. * param.s1.powi(2) * param.s2
                                                + -1945. * param.s1 * param.s2.powi(2)
                                                + 210. * param.s2.powi(3)))
                                    + 2. * param.m1_2
                                        * (18. * param.s12.powi(6)
                                            + -7.
                                                * param.s12.powi(5)
                                                * (9. * param.s1 + -41. * param.s2)
                                            + 15.
                                                * param.s12.powi(4)
                                                * (3. * param.s1.powi(2)
                                                    + 62. * param.s1 * param.s2
                                                    + -68. * param.s2.powi(2))
                                            + 5. * param.s12.powi(3)
                                                * (18. * param.s1.powi(3)
                                                    + -739. * param.s1.powi(2) * param.s2
                                                    + 596. * param.s1 * param.s2.powi(2)
                                                    + 170. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-180. * param.s1.powi(4)
                                                    + 2635. * param.s1.powi(3) * param.s2
                                                    + 4711.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -7820. * param.s1 * param.s2.powi(3)
                                                    + 430. * param.s2.powi(4))
                                            + param.s12
                                                * (117. * param.s1.powi(5)
                                                    + 660. * param.s1.powi(4) * param.s2
                                                    + -7682.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 4483.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 3295. * param.s1 * param.s2.powi(4)
                                                    + -873. * param.s2.powi(5))
                                            - (param.s1 - param.s2).powi(3)
                                                * (27. * param.s1.powi(3)
                                                    + 898. * param.s1.powi(2) * param.s2
                                                    + 1602. * param.s1 * param.s2.powi(2)
                                                    + 308. * param.s2.powi(3)))
                                    + -3.
                                        * (param.s12.powi(7)
                                            + 5. * param.s12.powi(6) * (param.s1 + param.s2)
                                            + -4.
                                                * param.s12.powi(5)
                                                * (9. * param.s1.powi(2)
                                                    + -73. * param.s1 * param.s2
                                                    + 9. * param.s2.powi(2))
                                            + 10.
                                                * param.s12.powi(4)
                                                * (7. * param.s1.powi(3)
                                                    + -34. * param.s1.powi(2) * param.s2
                                                    + -34. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3))
                                            + param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (10. * param.s1.powi(4)
                                                    + -285. * param.s1.powi(3) * param.s2
                                                    + -1518.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -285. * param.s1 * param.s2.powi(3)
                                                    + 10. * param.s2.powi(4))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (11. * param.s1.powi(4)
                                                    + 139. * param.s1.powi(3) * param.s2
                                                    + -524.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 139. * param.s1 * param.s2.powi(3)
                                                    + 11. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (9. * param.s1.powi(5)
                                                    + 1130. * param.s1.powi(4) * param.s2
                                                    + -1591.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -1591.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + 1130. * param.s1 * param.s2.powi(4)
                                                    + 9. * param.s2.powi(5))
                                            - (param.s1 - param.s2).powi(4)
                                                * (4. * param.s1.powi(3)
                                                    + 103. * param.s1.powi(2) * param.s2
                                                    + 103. * param.s1 * param.s2.powi(2)
                                                    + 4. * param.s2.powi(3))))
                            + 3. * param.m2_2.powi(2)
                                * param.s2
                                * (2.
                                    * param.m1_2
                                    * (9. * param.s12.powi(6)
                                        + -18.
                                            * param.s12.powi(5)
                                            * (3. * param.s1 + 8. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (27. * param.s1.powi(2)
                                                + 72. * param.s1 * param.s2
                                                + -20. * param.s2.powi(2))
                                        + -10.
                                            * param.s12.powi(3)
                                            * (18. * param.s1.powi(3)
                                                + 395. * param.s1 * param.s2.powi(2)
                                                + -166. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (135. * param.s1.powi(4)
                                                + -720. * param.s1.powi(3) * param.s2
                                                + 5238. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4160. * param.s1 * param.s2.powi(3)
                                                + -3075. * param.s2.powi(4))
                                        + (param.s1 - param.s2).powi(2)
                                            * (9. * param.s1.powi(4)
                                                + -198. * param.s1.powi(3) * param.s2
                                                + -3367. * param.s1.powi(2) * param.s2.powi(2)
                                                + -4048. * param.s1 * param.s2.powi(3)
                                                + -586. * param.s2.powi(4))
                                        + param.s12
                                            * (-54. * param.s1.powi(5)
                                                + 720. * param.s1.powi(4) * param.s2
                                                + 1774. * param.s1.powi(3) * param.s2.powi(2)
                                                + -12076.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(3)
                                                + 2360. * param.s1 * param.s2.powi(4)
                                                + 2236. * param.s2.powi(5)))
                                    + -3.
                                        * (param.s12.powi(7)
                                            + param.s12.powi(5)
                                                * (-15. * param.s1.powi(2)
                                                    + -74. * param.s1 * param.s2
                                                    + 27. * param.s2.powi(2))
                                            + 5. * param.s12.powi(4)
                                                * (11. * param.s1.powi(3)
                                                    + 65. * param.s1.powi(2) * param.s2
                                                    + -195. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(3)
                                                * (5. * param.s1.powi(4)
                                                    + -86. * param.s1.powi(3) * param.s2
                                                    + -1068.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -718. * param.s1 * param.s2.powi(3)
                                                    + -23. * param.s2.powi(4))
                                            + -5.
                                                * param.s12.powi(3)
                                                * (17. * param.s1.powi(4)
                                                    + 60. * param.s1.powi(3) * param.s2
                                                    + 250.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -546. * param.s1 * param.s2.powi(3)
                                                    + 29. * param.s2.powi(4))
                                            + param.s12.powi(2)
                                                * (69. * param.s1.powi(5)
                                                    + -115. * param.s1.powi(4) * param.s2
                                                    + 4522.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -3832.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -1585. * param.s1 * param.s2.powi(4)
                                                    + 189. * param.s2.powi(5))
                                            - param.s12
                                                * (29. * param.s1.powi(6)
                                                    + -278. * param.s1.powi(5) * param.s2
                                                    + 1529.
                                                        * param.s1.powi(4)
                                                        * param.s2.powi(2)
                                                    + 3658.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(3)
                                                    + -5789.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(4)
                                                    + 744. * param.s1 * param.s2.powi(5)
                                                    + 107. * param.s2.powi(6))
                                            - param.s12.powi(6) * (param.s1 + 13. * param.s2))))
                    - param.m2_2.powi(4)
                        * param.s2
                        * (14. * param.s1.powi(7)
                            + -5. * param.s12.powi(7)
                            + -401. * param.s1.powi(6) * param.s2
                            + -7212. * param.s1.powi(5) * param.s2.powi(2)
                            + -2487. * param.s1.powi(4) * param.s2.powi(3)
                            + 9168. * param.s1.powi(3) * param.s2.powi(4)
                            + 999. * param.s1.powi(2) * param.s2.powi(5)
                            + -86. * param.s1 * param.s2.powi(6)
                            + 5. * param.s2.powi(7)
                            + param.s12.powi(6) * (44. * param.s1 + 35. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (62. * param.s1.powi(3)
                                    + -43. * param.s1.powi(2) * param.s2
                                    + 2. * param.s1 * param.s2.powi(2)
                                    + 35. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (71. * param.s1.powi(4)
                                    + -300. * param.s1.powi(3) * param.s2
                                    + -120. * param.s1.powi(2) * param.s2.powi(2)
                                    + -84. * param.s1 * param.s2.powi(3)
                                    + 35. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (240. * param.s1.powi(5)
                                    + -2395. * param.s1.powi(4) * param.s2
                                    + -8022. * param.s1.powi(3) * param.s2.powi(2)
                                    + 1080. * param.s1.powi(2) * param.s2.powi(3)
                                    + -640. * param.s1 * param.s2.powi(4)
                                    + 105. * param.s2.powi(5))
                            + param.s12
                                * (-89. * param.s1.powi(6)
                                    + 1610. * param.s1.powi(5) * param.s2
                                    + 14729. * param.s1.powi(4) * param.s2.powi(2)
                                    + -2956. * param.s1.powi(3) * param.s2.powi(3)
                                    + -2305. * param.s1.powi(2) * param.s2.powi(4)
                                    + 386. * param.s1 * param.s2.powi(5)
                                    + -35. * param.s2.powi(6))
                            + -3.
                                * param.m1_2
                                * (3. * param.s1.powi(6)
                                    + 3. * param.s12.powi(6)
                                    + -102. * param.s1.powi(5) * param.s2
                                    + -2839. * param.s1.powi(4) * param.s2.powi(2)
                                    + -6724. * param.s1.powi(3) * param.s2.powi(3)
                                    + -2839. * param.s1.powi(2) * param.s2.powi(4)
                                    + -102. * param.s1 * param.s2.powi(5)
                                    + 3. * param.s2.powi(6)
                                    + -18. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + 15.
                                        * param.s12.powi(4)
                                        * (3. * param.s1.powi(2)
                                            + -2. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + -60.
                                        * param.s12.powi(3)
                                        * (param.s1.powi(3)
                                            + -5. * param.s1.powi(2) * param.s2
                                            + -5. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + -540. * param.s1.powi(3) * param.s2
                                            + -3574. * param.s1.powi(2) * param.s2.powi(2)
                                            + -540. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    + param.s12
                                        * (-18. * param.s1.powi(5)
                                            + 390. * param.s1.powi(4) * param.s2
                                            + 6068. * param.s1.powi(3) * param.s2.powi(2)
                                            + 6068. * param.s1.powi(2) * param.s2.powi(3)
                                            + 390. * param.s1 * param.s2.powi(4)
                                            + -18. * param.s2.powi(5)))
                            - param.s12.powi(5)
                                * (159. * param.s1.powi(2)
                                    + 134. * param.s1 * param.s2
                                    + 105. * param.s2.powi(2))))
                    * param.lambda_m02_sqrt
                    * param.lambda_s12_sqrt)
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.002777777777777778
            * std::f64::consts::PI
            * param.s12.powi(-5)
            * param.lambda_s12_sqrt.powi(-11)
            * ((-6. * param.m2_2.powi(5) * param.s1.powi(9)
                + 66. * param.m2_2.powi(5) * param.s1.powi(8) * param.s12
                + 21. * param.m2_2.powi(4) * param.s1.powi(9) * param.s12
                + -340. * param.m2_2.powi(5) * param.s1.powi(7) * param.s12.powi(2)
                + -240. * param.m2_2.powi(4) * param.s1.powi(8) * param.s12.powi(2)
                + -24. * param.m2_2.powi(3) * param.s1.powi(9) * param.s12.powi(2)
                + 1120. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12.powi(3)
                + 1325. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12.powi(3)
                + 300. * param.m2_2.powi(3) * param.s1.powi(8) * param.s12.powi(3)
                + 6. * param.m2_2.powi(2) * param.s1.powi(9) * param.s12.powi(3)
                + -2940. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12.powi(4)
                + -5180. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12.powi(4)
                + -2050. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12.powi(4)
                + -120. * param.m2_2.powi(2) * param.s1.powi(8) * param.s12.powi(4)
                + 6. * param.m2_2 * param.s1.powi(9) * param.s12.powi(4)
                + 2856. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(5)
                + 8379. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(5)
                + 5095. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(5)
                + 500. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(5)
                + -30. * param.m2_2 * param.s1.powi(8) * param.s12.powi(5)
                + -3. * param.s1.powi(9) * param.s12.powi(5)
                + 84. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(6)
                + -3906. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(6)
                + -4896. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(6)
                + -905. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(6)
                + 50. * param.m2_2 * param.s1.powi(7) * param.s12.powi(6)
                + 24. * param.s1.powi(8) * param.s12.powi(6)
                + -960. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(7)
                + -2205. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(7)
                + 459. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(7)
                + 804. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(7)
                + -5. * param.m2_2 * param.s1.powi(6) * param.s12.powi(7)
                + -85. * param.s1.powi(7) * param.s12.powi(7)
                + 130. * param.m2_2.powi(5) * param.s1 * param.s12.powi(8)
                + 2100. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(8)
                + 2190. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(8)
                + -301. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(8)
                + -96. * param.m2_2 * param.s1.powi(5) * param.s12.powi(8)
                + 175. * param.s1.powi(6) * param.s12.powi(8)
                + -10. * param.m2_2.powi(5) * param.s12.powi(9)
                + -320. * param.m2_2.powi(4) * param.s1 * param.s12.powi(9)
                + -1275. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(9)
                + -20. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(9)
                + 149. * param.m2_2 * param.s1.powi(4) * param.s12.powi(9)
                + -231. * param.s1.powi(5) * param.s12.powi(9)
                + 26. * param.m2_2.powi(4) * param.s12.powi(10)
                + 220. * param.m2_2.powi(3) * param.s1 * param.s12.powi(10)
                + 45. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(10)
                + -110. * param.m2_2 * param.s1.powi(3) * param.s12.powi(10)
                + 203. * param.s1.powi(4) * param.s12.powi(10)
                + -19. * param.m2_2.powi(3) * param.s12.powi(11)
                + -10. * param.m2_2.powi(2) * param.s1 * param.s12.powi(11)
                + 45. * param.m2_2 * param.s1.powi(2) * param.s12.powi(11)
                + -119. * param.s1.powi(3) * param.s12.powi(11)
                + param.m2_2.powi(2) * param.s12.powi(12)
                + -10. * param.m2_2 * param.s1 * param.s12.powi(12)
                + 45. * param.s1.powi(2) * param.s12.powi(12)
                + param.m2_2 * param.s12.powi(13)
                + -10. * param.s1 * param.s12.powi(13)
                + param.s12.powi(14)
                + 54. * param.m2_2.powi(5) * param.s1.powi(8) * param.s2
                + -405. * param.m2_2.powi(5) * param.s1.powi(7) * param.s12 * param.s2
                + -180. * param.m2_2.powi(4) * param.s1.powi(8) * param.s12 * param.s2
                + 1255. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12.powi(2) * param.s2
                + 1365. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12.powi(2) * param.s2
                + 180. * param.m2_2.powi(3) * param.s1.powi(8) * param.s12.powi(2) * param.s2
                + -1825. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12.powi(3) * param.s2
                + -4205. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12.powi(3) * param.s2
                + -1335. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12.powi(3) * param.s2
                + -565. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + 4475. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(4) * param.s2
                + 2995. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(4) * param.s2
                + -435. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(4) * param.s2
                + -90. * param.m2_2 * param.s1.powi(8) * param.s12.powi(4) * param.s2
                + -1909. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + -10855. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(5) * param.s2
                + -14425. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(5) * param.s2
                + -3305. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(5) * param.s2
                + 90. * param.m2_2 * param.s1.powi(7) * param.s12.powi(5) * param.s2
                + 36. * param.s1.powi(8) * param.s12.powi(5) * param.s2
                + 3215. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + 13505. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(6) * param.s2
                + 24365. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(6) * param.s2
                + 11675. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(6) * param.s2
                + 520. * param.m2_2 * param.s1.powi(6) * param.s12.powi(6) * param.s2
                + -180. * param.s1.powi(7) * param.s12.powi(6) * param.s2
                + -745. * param.m2_2.powi(5) * param.s1 * param.s12.powi(7) * param.s2
                + -5605. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(7) * param.s2
                + -12535. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(7) * param.s2
                + -10735. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(7) * param.s2
                + -1150. * param.m2_2 * param.s1.powi(5) * param.s12.powi(7) * param.s2
                + 340. * param.s1.powi(6) * param.s12.powi(7) * param.s2
                + 85. * param.m2_2.powi(5) * param.s12.powi(8) * param.s2
                + 1715. * param.m2_2.powi(4) * param.s1 * param.s12.powi(8) * param.s2
                + 1595. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(8) * param.s2
                + 1725. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(8) * param.s2
                + 740. * param.m2_2 * param.s1.powi(4) * param.s12.powi(8) * param.s2
                + -250. * param.s1.powi(5) * param.s12.powi(8) * param.s2
                + -215. * param.m2_2.powi(4) * param.s12.powi(9) * param.s2
                + -985. * param.m2_2.powi(3) * param.s1 * param.s12.powi(9) * param.s2
                + 1195. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(9) * param.s2
                + 30. * param.m2_2 * param.s1.powi(3) * param.s12.powi(9) * param.s2
                + -70. * param.s1.powi(4) * param.s12.powi(9) * param.s2
                + 145. * param.m2_2.powi(3) * param.s12.powi(10) * param.s2
                + -125. * param.m2_2.powi(2) * param.s1 * param.s12.powi(10) * param.s2
                + -200. * param.m2_2 * param.s1.powi(2) * param.s12.powi(10) * param.s2
                + 264. * param.s1.powi(3) * param.s12.powi(10) * param.s2
                + 5. * param.m2_2.powi(2) * param.s12.powi(11) * param.s2
                + 70. * param.m2_2 * param.s1 * param.s12.powi(11) * param.s2
                + -200. * param.s1.powi(2) * param.s12.powi(11) * param.s2
                + -10. * param.m2_2 * param.s12.powi(12) * param.s2
                + 70. * param.s1 * param.s12.powi(12) * param.s2
                + -10. * param.s12.powi(13) * param.s2
                + -216. * param.m2_2.powi(5) * param.s1.powi(7) * param.s2.powi(2)
                + 987. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12 * param.s2.powi(2)
                + 684. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12 * param.s2.powi(2)
                + -1416.
                    * param.m2_2.powi(5)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -3033.
                    * param.m2_2.powi(4)
                    * param.s1.powi(6)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -591.
                    * param.m2_2.powi(3)
                    * param.s1.powi(7)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -25.
                    * param.m2_2.powi(5)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 3549.
                    * param.m2_2.powi(4)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 2067.
                    * param.m2_2.powi(3)
                    * param.s1.powi(6)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -141.
                    * param.m2_2.powi(2)
                    * param.s1.powi(7)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 960.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 4265.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 2649.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 2667.
                    * param.m2_2.powi(2)
                    * param.s1.powi(6)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 309. * param.m2_2 * param.s1.powi(7) * param.s12.powi(4) * param.s2.powi(2)
                + -3501.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + -8550.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + -6400.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + -5241.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + -2628. * param.m2_2 * param.s1.powi(6) * param.s12.powi(5) * param.s2.powi(2)
                + -225. * param.s1.powi(7) * param.s12.powi(5) * param.s2.powi(2)
                + 1712. * param.m2_2.powi(5) * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + 3219.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + -6195.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + -10900.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + -621. * param.m2_2 * param.s1.powi(5) * param.s12.powi(6) * param.s2.powi(2)
                + 480. * param.s1.powi(6) * param.s12.powi(6) * param.s2.powi(2)
                + -321. * param.m2_2.powi(5) * param.s12.powi(7) * param.s2.powi(2)
                + -3583. * param.m2_2.powi(4) * param.s1 * param.s12.powi(7) * param.s2.powi(2)
                + 2319.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(7)
                    * param.s2.powi(2)
                + 14895.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(7)
                    * param.s2.powi(2)
                + 7325. * param.m2_2 * param.s1.powi(4) * param.s12.powi(7) * param.s2.powi(2)
                + -60. * param.s1.powi(5) * param.s12.powi(7) * param.s2.powi(2)
                + 789. * param.m2_2.powi(4) * param.s12.powi(8) * param.s2.powi(2)
                + 1517. * param.m2_2.powi(3) * param.s1 * param.s12.powi(8) * param.s2.powi(2)
                + -1881.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(8)
                    * param.s2.powi(2)
                + -3855. * param.m2_2 * param.s1.powi(3) * param.s12.powi(8) * param.s2.powi(2)
                + -385. * param.s1.powi(4) * param.s12.powi(8) * param.s2.powi(2)
                + -486. * param.m2_2.powi(3) * param.s12.powi(9) * param.s2.powi(2)
                + 667. * param.m2_2.powi(2) * param.s1 * param.s12.powi(9) * param.s2.powi(2)
                + -456. * param.m2_2 * param.s1.powi(2) * param.s12.powi(9) * param.s2.powi(2)
                + 45. * param.s1.powi(3) * param.s12.powi(9) * param.s2.powi(2)
                + -66. * param.m2_2.powi(2) * param.s12.powi(10) * param.s2.powi(2)
                + -113. * param.m2_2 * param.s1 * param.s12.powi(10) * param.s2.powi(2)
                + 300. * param.s1.powi(2) * param.s12.powi(10) * param.s2.powi(2)
                + 39. * param.m2_2 * param.s12.powi(11) * param.s2.powi(2)
                + -200. * param.s1 * param.s12.powi(11) * param.s2.powi(2)
                + 45. * param.s12.powi(12) * param.s2.powi(2)
                + 504. * param.m2_2.powi(5) * param.s1.powi(6) * param.s2.powi(3)
                + -1113. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12 * param.s2.powi(3)
                + -1512. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12 * param.s2.powi(3)
                + 155.
                    * param.m2_2.powi(5)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 2973.
                    * param.m2_2.powi(4)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 1113.
                    * param.m2_2.powi(3)
                    * param.s1.powi(6)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 420.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 845.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -777.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 513.
                    * param.m2_2.powi(2)
                    * param.s1.powi(6)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 900.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + -840.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + -4930.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + -3777.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + -387. * param.m2_2 * param.s1.powi(6) * param.s12.powi(4) * param.s2.powi(3)
                + -1881. * param.m2_2.powi(5) * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + 2520.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + 15600.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + 17550.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + 7158. * param.m2_2 * param.s1.powi(5) * param.s12.powi(5) * param.s2.powi(3)
                + 489. * param.s1.powi(6) * param.s12.powi(5) * param.s2.powi(3)
                + 707. * param.m2_2.powi(5) * param.s12.powi(6) * param.s2.powi(3)
                + 3273. * param.m2_2.powi(4) * param.s1 * param.s12.powi(6) * param.s2.powi(3)
                + -3255.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(3)
                + -8200.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(6)
                    * param.s2.powi(3)
                + -6765. * param.m2_2 * param.s1.powi(4) * param.s12.powi(6) * param.s2.powi(3)
                + -1764. * param.s1.powi(5) * param.s12.powi(6) * param.s2.powi(3)
                + -1687. * param.m2_2.powi(4) * param.s12.powi(7) * param.s2.powi(3)
                + -477. * param.m2_2.powi(3) * param.s1 * param.s12.powi(7) * param.s2.powi(3)
                + -1455.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(7)
                    * param.s2.powi(3)
                + -4840. * param.m2_2 * param.s1.powi(3) * param.s12.powi(7) * param.s2.powi(3)
                + -375. * param.s1.powi(4) * param.s12.powi(7) * param.s2.powi(3)
                + 938. * param.m2_2.powi(3) * param.s12.powi(8) * param.s2.powi(3)
                + -1077. * param.m2_2.powi(2) * param.s1 * param.s12.powi(8) * param.s2.powi(3)
                + 1245. * param.m2_2 * param.s1.powi(2) * param.s12.powi(8) * param.s2.powi(3)
                + 1460. * param.s1.powi(3) * param.s12.powi(8) * param.s2.powi(3)
                + 238. * param.m2_2.powi(2) * param.s12.powi(9) * param.s2.powi(3)
                + -102. * param.m2_2 * param.s1 * param.s12.powi(9) * param.s2.powi(3)
                + 45. * param.s1.powi(2) * param.s12.powi(9) * param.s2.powi(3)
                + -77. * param.m2_2 * param.s12.powi(10) * param.s2.powi(3)
                + 264. * param.s1 * param.s12.powi(10) * param.s2.powi(3)
                + -119. * param.s12.powi(11) * param.s2.powi(3)
                + -756. * param.m2_2.powi(5) * param.s1.powi(5) * param.s2.powi(4)
                + 315. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12 * param.s2.powi(4)
                + 2142. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12 * param.s2.powi(4)
                + 340.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -345.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -1323.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + 340.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -1145.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -1395.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -873.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + 700. * param.m2_2.powi(5) * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + -1820.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + -470.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + 1305.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + 27. * param.m2_2 * param.s1.powi(5) * param.s12.powi(4) * param.s2.powi(4)
                + -1001. * param.m2_2.powi(5) * param.s12.powi(5) * param.s2.powi(4)
                + -245. * param.m2_2.powi(4) * param.s1 * param.s12.powi(5) * param.s2.powi(4)
                + -1145.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(4)
                + -9060.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(4)
                + -6180. * param.m2_2 * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(4)
                + -297. * param.s1.powi(5) * param.s12.powi(5) * param.s2.powi(4)
                + 2317. * param.m2_2.powi(4) * param.s12.powi(6) * param.s2.powi(4)
                + -1295. * param.m2_2.powi(3) * param.s1 * param.s12.powi(6) * param.s2.powi(4)
                + 2855.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(4)
                + 8010. * param.m2_2 * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(4)
                + 2880. * param.s1.powi(4) * param.s12.powi(6) * param.s2.powi(4)
                + -1148. * param.m2_2.powi(3) * param.s12.powi(7) * param.s2.powi(4)
                + 455. * param.m2_2.powi(2) * param.s1 * param.s12.powi(7) * param.s2.powi(4)
                + 155. * param.m2_2 * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(4)
                + -375. * param.s1.powi(3) * param.s12.powi(7) * param.s2.powi(4)
                + -448. * param.m2_2.powi(2) * param.s12.powi(8) * param.s2.powi(4)
                + 455. * param.m2_2 * param.s1 * param.s12.powi(8) * param.s2.powi(4)
                + -385. * param.s1.powi(2) * param.s12.powi(8) * param.s2.powi(4)
                + 77. * param.m2_2 * param.s12.powi(9) * param.s2.powi(4)
                + -70. * param.s1 * param.s12.powi(9) * param.s2.powi(4)
                + 203. * param.s12.powi(10) * param.s2.powi(4)
                + 756. * param.m2_2.powi(5) * param.s1.powi(4) * param.s2.powi(5)
                + 609. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12 * param.s2.powi(5)
                + -2016. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12 * param.s2.powi(5)
                + 525.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + -2025.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + 1029.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + 565. * param.m2_2.powi(5) * param.s1 * param.s12.powi(3) * param.s2.powi(5)
                + -1875.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + 1935.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + 849.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + 945. * param.m2_2.powi(5) * param.s12.powi(4) * param.s2.powi(5)
                + -2195. * param.m2_2.powi(4) * param.s1 * param.s12.powi(4) * param.s2.powi(5)
                + 2625.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(5)
                + 1155.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(5)
                + 399. * param.m2_2 * param.s1.powi(4) * param.s12.powi(4) * param.s2.powi(5)
                + -2121. * param.m2_2.powi(4) * param.s12.powi(5) * param.s2.powi(5)
                + 1765. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5) * param.s2.powi(5)
                + 105.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(5)
                + 1170. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(5)
                + -297. * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(5)
                + 924. * param.m2_2.powi(3) * param.s12.powi(6) * param.s2.powi(5)
                + 545. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6) * param.s2.powi(5)
                + -1320. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(5)
                + -1764. * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(5)
                + 504. * param.m2_2.powi(2) * param.s12.powi(7) * param.s2.powi(5)
                + -430. * param.m2_2 * param.s1 * param.s12.powi(7) * param.s2.powi(5)
                + -60. * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(5)
                + -21. * param.m2_2 * param.s12.powi(8) * param.s2.powi(5)
                + -250. * param.s1 * param.s12.powi(8) * param.s2.powi(5)
                + -231. * param.s12.powi(9) * param.s2.powi(5)
                + -504. * param.m2_2.powi(5) * param.s1.powi(3) * param.s2.powi(6)
                + -735. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12 * param.s2.powi(6)
                + 1260. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12 * param.s2.powi(6)
                + -760. * param.m2_2.powi(5) * param.s1 * param.s12.powi(2) * param.s2.powi(6)
                + 1965.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(6)
                + -525.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(6)
                + -595. * param.m2_2.powi(5) * param.s12.powi(3) * param.s2.powi(6)
                + 2015. * param.m2_2.powi(4) * param.s1 * param.s12.powi(3) * param.s2.powi(6)
                + -1035.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(6)
                + -495.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(6)
                + 1295. * param.m2_2.powi(4) * param.s12.powi(4) * param.s2.powi(6)
                + -985. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2.powi(6)
                + -1035.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(6)
                + -405. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(6)
                + -490. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2.powi(6)
                + -695. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2.powi(6)
                + 360. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(6)
                + 489. * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(6)
                + -350. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2.powi(6)
                + 85. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2.powi(6)
                + 480. * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(6)
                + -35. * param.m2_2 * param.s12.powi(7) * param.s2.powi(6)
                + 340. * param.s1 * param.s12.powi(7) * param.s2.powi(6)
                + 175. * param.s12.powi(8) * param.s2.powi(6)
                + 216. * param.m2_2.powi(5) * param.s1.powi(2) * param.s2.powi(7)
                + 333. * param.m2_2.powi(5) * param.s1 * param.s12 * param.s2.powi(7)
                + -504. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12 * param.s2.powi(7)
                + 241. * param.m2_2.powi(5) * param.s12.powi(2) * param.s2.powi(7)
                + -777. * param.m2_2.powi(4) * param.s1 * param.s12.powi(2) * param.s2.powi(7)
                + 171.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(7)
                + -509. * param.m2_2.powi(4) * param.s12.powi(3) * param.s2.powi(7)
                + 273. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(7)
                + 171.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(7)
                + 166. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(7)
                + 273. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(7)
                + 171. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(7)
                + 146. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(7)
                + 78. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(7)
                + -225. * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(7)
                + 41. * param.m2_2 * param.s12.powi(6) * param.s2.powi(7)
                + -180. * param.s1 * param.s12.powi(6) * param.s2.powi(7)
                + -85. * param.s12.powi(7) * param.s2.powi(7)
                + -54. * param.m2_2.powi(5) * param.s1 * param.s2.powi(8)
                + -57. * param.m2_2.powi(5) * param.s12 * param.s2.powi(8)
                + 117. * param.m2_2.powi(4) * param.s1 * param.s12 * param.s2.powi(8)
                + 117. * param.m2_2.powi(4) * param.s12.powi(2) * param.s2.powi(8)
                + -33. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(8)
                + -33. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(8)
                + -33. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(8)
                + -33. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(8)
                + -33. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(8)
                + -18. * param.m2_2 * param.s12.powi(5) * param.s2.powi(8)
                + 36. * param.s1 * param.s12.powi(5) * param.s2.powi(8)
                + 24. * param.s12.powi(6) * param.s2.powi(8)
                + 6. * param.m2_2.powi(5) * param.s2.powi(9)
                + -12. * param.m2_2.powi(4) * param.s12 * param.s2.powi(9)
                + 3. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(9)
                + 3. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(9)
                + 3. * param.m2_2 * param.s12.powi(4) * param.s2.powi(9)
                + -3. * param.s12.powi(5) * param.s2.powi(9)
                + 60.
                    * param.m0_2.powi(5)
                    * param.s12.powi(5)
                    * (10. * param.s12.powi(4)
                        + 3. * (param.s1 - param.s2).powi(4)
                        + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                        + 9. * param.s12 * (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                        + param.s12.powi(2)
                            * (-17. * param.s1.powi(2)
                                + 40. * param.s1 * param.s2
                                + -17. * param.s2.powi(2)))
                + param.m1_2.powi(5)
                    * (-10. * param.s12.powi(9)
                        + 6. * (param.s1 - param.s2).powi(9)
                        + -3.
                            * param.s12
                            * (param.s1 - param.s2).powi(7)
                            * (19. * param.s1 + 22. * param.s2)
                        + 5. * param.s12.powi(8) * (17. * param.s1 + 26. * param.s2)
                        + param.s12.powi(2)
                            * (param.s1 - param.s2).powi(5)
                            * (241. * param.s1.powi(2)
                                + 445. * param.s1 * param.s2
                                + 340. * param.s2.powi(2))
                        + param.s12.powi(6)
                            * (707. * param.s1.powi(3)
                                + 1712. * param.s1.powi(2) * param.s2
                                + 3215. * param.s1 * param.s2.powi(2)
                                + 84. * param.s2.powi(3))
                        + -5.
                            * param.s12.powi(3)
                            * (param.s1 - param.s2).powi(3)
                            * (119. * param.s1.powi(3)
                                + 244. * param.s1.powi(2) * param.s2
                                + 307. * param.s1 * param.s2.powi(2)
                                + 224. * param.s2.powi(3))
                        + 5. * param.s12.powi(4)
                            * (189. * param.s1.powi(5)
                                + 140. * param.s1.powi(4) * param.s2
                                + 180. * param.s1.powi(3) * param.s2.powi(2)
                                + 192. * param.s1.powi(2) * param.s2.powi(3)
                                + -113. * param.s1 * param.s2.powi(4)
                                + -588. * param.s2.powi(5))
                        - param.s12.powi(5)
                            * (1001. * param.s1.powi(4)
                                + 1881. * param.s1.powi(3) * param.s2
                                + 3501. * param.s1.powi(2) * param.s2.powi(2)
                                + 1909. * param.s1 * param.s2.powi(3)
                                + -2856. * param.s2.powi(4))
                        - param.s12.powi(7)
                            * (321. * param.s1.powi(2)
                                + 745. * param.s1 * param.s2
                                + 960. * param.s2.powi(2)))
                + -30.
                    * param.m0_2.powi(4)
                    * param.s12.powi(4)
                    * (param.m1_2
                        * (50. * param.s12.powi(5)
                            + -3. * (param.s1 - param.s2).powi(5)
                            + -6.
                                * param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (6. * param.s1 + 11. * param.s2)
                            + param.s12.powi(4) * (-105. * param.s1 + 55. * param.s2)
                            + 2. * param.s12.powi(3)
                                * (9. * param.s1.powi(2)
                                    + 100. * param.s1 * param.s2
                                    + -94. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (38. * param.s1.powi(3)
                                    + -156. * param.s1.powi(2) * param.s2
                                    + 111. * param.s1 * param.s2.powi(2)
                                    + 7. * param.s2.powi(3)))
                        + param.s12
                            * (-26. * param.s12.powi(5)
                                + 21. * param.s12.powi(4) * (param.s1 + param.s2)
                                + 15. * (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + 12.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + 13. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (78. * param.s1.powi(2)
                                        + -238. * param.s1 * param.s2
                                        + 78. * param.s2.powi(2))
                                + -10.
                                    * param.s12.powi(2)
                                    * (10. * param.s1.powi(3)
                                        + -13. * param.s1.powi(2) * param.s2
                                        + -13. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3)))
                        + param.m2_2
                            * (50. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (11. * param.s1 + -21. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(5)
                                + 6. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (11. * param.s1 + 6. * param.s2)
                                + param.s12.powi(3)
                                    * (-188. * param.s1.powi(2)
                                        + 200. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (7. * param.s1.powi(3)
                                        + 111. * param.s1.powi(2) * param.s2
                                        + -156. * param.s1 * param.s2.powi(2)
                                        + 38. * param.s2.powi(3))))
                + param.m1_2.powi(4)
                    * (param.s12
                        * (26. * param.s12.powi(9)
                            + -3.
                                * (4. * param.s1 + -7. * param.s2)
                                * (param.s1 - param.s2).powi(8)
                            + -5. * param.s12.powi(8) * (43. * param.s1 + 64. * param.s2)
                            + 3. * param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (39. * param.s1.powi(2)
                                    + -25. * param.s1 * param.s2
                                    + -80. * param.s2.powi(2))
                            + param.s12.powi(7)
                                * (789. * param.s1.powi(2)
                                    + 1715. * param.s1 * param.s2
                                    + 2100. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (2317. * param.s1.powi(4)
                                    + 3273. * param.s1.powi(3) * param.s2
                                    + 3219. * param.s1.powi(2) * param.s2.powi(2)
                                    + 13505. * param.s1 * param.s2.powi(3)
                                    + -3906. * param.s2.powi(4))
                            + 5. * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (259. * param.s1.powi(4)
                                    + 79. * param.s1.powi(3) * param.s2
                                    + -465. * param.s1.powi(2) * param.s2.powi(2)
                                    + -1177. * param.s1 * param.s2.powi(3)
                                    + -1036. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (2121. * param.s1.powi(5)
                                    + 245. * param.s1.powi(4) * param.s2
                                    + -2520. * param.s1.powi(3) * param.s2.powi(2)
                                    + 8550. * param.s1.powi(2) * param.s2.powi(3)
                                    + 10855. * param.s1 * param.s2.powi(4)
                                    + -8379. * param.s2.powi(5))
                            - param.s12.powi(6)
                                * (1687. * param.s1.powi(3)
                                    + 3583. * param.s1.powi(2) * param.s2
                                    + 5605. * param.s1 * param.s2.powi(2)
                                    + 2205. * param.s2.powi(3))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (509. * param.s1.powi(3)
                                    + 21. * param.s1.powi(2) * param.s2
                                    + -1095. * param.s1 * param.s2.powi(2)
                                    + -1325. * param.s2.powi(3)))
                        + param.m2_2
                            * (80. * param.s12.powi(9)
                                + -30. * (param.s1 - param.s2).powi(9)
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(7)
                                    * (98. * param.s1 + 107. * param.s2)
                                + -5. * param.s12.powi(8) * (127. * param.s1 + 262. * param.s2)
                                + param.s12.powi(7)
                                    * (2244. * param.s1.powi(2)
                                        + 6635. * param.s1 * param.s2
                                        + 369. * param.s2.powi(2))
                                + 5. * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(3)
                                    * (664. * param.s1.powi(3)
                                        + 1313. * param.s1.powi(2) * param.s2
                                        + 1520. * param.s1 * param.s2.powi(2)
                                        + 973. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (6190. * param.s1.powi(4)
                                        + 13185. * param.s1.powi(3) * param.s2
                                        + 16503. * param.s1.powi(2) * param.s2.powi(2)
                                        + -5395. * param.s1 * param.s2.powi(3)
                                        + -14175. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(4)
                                    * (1107. * param.s1.powi(5)
                                        + 868. * param.s1.powi(4) * param.s2
                                        + 1008. * param.s1.powi(3) * param.s2.powi(2)
                                        + 348. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1147. * param.s1 * param.s2.powi(4)
                                        + -2184. * param.s2.powi(5))
                                - param.s12.powi(6)
                                    * (4639. * param.s1.powi(3)
                                        + 13498. * param.s1.powi(2) * param.s2
                                        + 9343. * param.s1 * param.s2.powi(2)
                                        + -7686. * param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(5)
                                    * (1289. * param.s1.powi(2)
                                        + 2255. * param.s1 * param.s2
                                        + 1586. * param.s2.powi(2))))
                + param.m1_2.powi(2)
                    * (param.m2_2.powi(3)
                        * (-370. * param.s12.powi(9)
                            + -5. * param.s12.powi(8) * (97. * param.s1 + -503. * param.s2)
                            + -60. * (param.s1 - param.s2).powi(9)
                            + 6. * param.s12
                                * (param.s1 - param.s2).powi(7)
                                * (104. * param.s1 + 101. * param.s2)
                            + param.s12.powi(7)
                                * (7344. * param.s1.powi(2)
                                    + -7090. * param.s1 * param.s2
                                    + -7656. * param.s2.powi(2))
                            + 20.
                                * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(3)
                                * (425. * param.s1.powi(3)
                                    + 736. * param.s1.powi(2) * param.s2
                                    + 700. * param.s1 * param.s2.powi(2)
                                    + 374. * param.s2.powi(3))
                            + param.s12.powi(6)
                                * (-18189. * param.s1.powi(3)
                                    + -8773. * param.s1.powi(2) * param.s2
                                    + 27227. * param.s1 * param.s2.powi(2)
                                    + 13811. * param.s2.powi(3))
                            + 2. * param.s12.powi(5)
                                * (11175. * param.s1.powi(4)
                                    + 14680. * param.s1.powi(3) * param.s2
                                    + -6771. * param.s1.powi(2) * param.s2.powi(2)
                                    + -16320. * param.s1 * param.s2.powi(3)
                                    + -8200. * param.s2.powi(4))
                            + -5.
                                * param.s12.powi(4)
                                * (3351. * param.s1.powi(5)
                                    + 2453. * param.s1.powi(4) * param.s2
                                    + 708. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1692. * param.s1.powi(2) * param.s2.powi(3)
                                    + -2147. * param.s1 * param.s2.powi(4)
                                    + -2673. * param.s2.powi(5))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(5)
                                * (2959. * param.s1.powi(2)
                                    + 4540. * param.s1 * param.s2
                                    + 2761. * param.s2.powi(2)))
                        + 3. * param.m2_2.powi(2)
                            * param.s12
                            * (12. * param.s12.powi(9)
                                + 95. * param.s12.powi(8) * (param.s1 + param.s2)
                                + 9. * (param.s1 - param.s2).powi(8) * (param.s1 + param.s2)
                                + param.s12.powi(7)
                                    * (-773. * param.s1.powi(2)
                                        + 3050. * param.s1 * param.s2
                                        + -773. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(4)
                                    * (229. * param.s1.powi(3)
                                        + 716. * param.s1.powi(2) * param.s2
                                        + 716. * param.s1 * param.s2.powi(2)
                                        + 229. * param.s2.powi(3))
                                + param.s12.powi(6)
                                    * (2070. * param.s1.powi(3)
                                        + -6934. * param.s1.powi(2) * param.s2
                                        + -6934. * param.s1 * param.s2.powi(2)
                                        + 2070. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (-2929. * param.s1.powi(4)
                                        + 1238. * param.s1.powi(3) * param.s2
                                        + 27334. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1238. * param.s1 * param.s2.powi(3)
                                        + -2929. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(2)
                                    * (267. * param.s1.powi(4)
                                        + 1162. * param.s1.powi(3) * param.s2
                                        + 1822. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1162. * param.s1 * param.s2.powi(3)
                                        + 267. * param.s2.powi(4))
                                + 2. * param.s12.powi(4)
                                    * (1244. * param.s1.powi(5)
                                        + 2895. * param.s1.powi(4) * param.s2
                                        + -9575. * param.s1.powi(3) * param.s2.powi(2)
                                        + -9575. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2895. * param.s1 * param.s2.powi(4)
                                        + 1244. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(6)
                                    * (95. * param.s1.powi(2)
                                        + 206. * param.s1 * param.s2
                                        + 95. * param.s2.powi(2)))
                        + param.s12.powi(3)
                            * (param.s12.powi(9)
                                + 5. * param.s12.powi(8) * (param.s1 + -2. * param.s2)
                                + param.s12.powi(7)
                                    * (-66. * param.s1.powi(2)
                                        + -125. * param.s1 * param.s2
                                        + 45. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (238. * param.s1.powi(3)
                                        + 667. * param.s1.powi(2) * param.s2
                                        + 1195. * param.s1 * param.s2.powi(2)
                                        + -20. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(6)
                                    * (param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 12. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3))
                                + -3.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (11. * param.s1.powi(4)
                                        + -47. * param.s1.powi(3) * param.s2
                                        + 91. * param.s1.powi(2) * param.s2.powi(2)
                                        + 305. * param.s1 * param.s2.powi(3)
                                        + 40. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (146. * param.s1.powi(5)
                                        + -403. * param.s1.powi(4) * param.s2
                                        + -847. * param.s1.powi(3) * param.s2.powi(2)
                                        + -10351. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2305. * param.s1 * param.s2.powi(4)
                                        + 500. * param.s2.powi(5))
                                + param.s12.powi(4)
                                    * (504. * param.s1.powi(5)
                                        + 455. * param.s1.powi(4) * param.s2
                                        + -1455. * param.s1.powi(3) * param.s2.powi(2)
                                        + 14895. * param.s1.powi(2) * param.s2.powi(3)
                                        + -10735. * param.s1 * param.s2.powi(4)
                                        + 804. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(3)
                                    * (70. * param.s1.powi(6)
                                        + -109. * param.s1.powi(5) * param.s2
                                        + -571. * param.s1.powi(4) * param.s2.powi(2)
                                        + 1640. * param.s1.powi(3) * param.s2.powi(3)
                                        + 2180. * param.s1.powi(2) * param.s2.powi(4)
                                        + -2335. * param.s1 * param.s2.powi(5)
                                        + 181. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (448. * param.s1.powi(4)
                                        + 1077. * param.s1.powi(3) * param.s2
                                        + 1881. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1725. * param.s1 * param.s2.powi(3)
                                        + 301. * param.s2.powi(4)))
                        + 3. * param.m2_2
                            * param.s12.powi(2)
                            * (3. * param.s12.powi(9)
                                + 5. * param.s12.powi(8) * (param.s1 + -6. * param.s2)
                                + (param.s1 - param.s2).powi(7)
                                    * (2. * param.s1.powi(2)
                                        + -12. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (347. * param.s1.powi(3)
                                        + 1606. * param.s1.powi(2) * param.s2
                                        + -3046. * param.s1 * param.s2.powi(2)
                                        + 389. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (113. * param.s1.powi(4)
                                        + -591. * param.s1.powi(3) * param.s2
                                        + -2564. * param.s1.powi(2) * param.s2.powi(2)
                                        + -2181. * param.s1 * param.s2.powi(3)
                                        + -327. * param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (557. * param.s1.powi(5)
                                        + -140. * param.s1.powi(4) * param.s2
                                        + 15505. * param.s1.powi(3) * param.s2.powi(2)
                                        + -12015. * param.s1.powi(2) * param.s2.powi(3)
                                        + -8480. * param.s1 * param.s2.powi(4)
                                        + 1357. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(3)
                                    * (65. * param.s1.powi(6)
                                        + -287. * param.s1.powi(5) * param.s2
                                        + 1548. * param.s1.powi(4) * param.s2.powi(2)
                                        + 1952. * param.s1.powi(3) * param.s2.powi(3)
                                        + -3367. * param.s1.powi(2) * param.s2.powi(4)
                                        + -93. * param.s1 * param.s2.powi(5)
                                        + 182. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (577. * param.s1.powi(4)
                                        + 1749. * param.s1.powi(3) * param.s2
                                        + 3805. * param.s1.powi(2) * param.s2.powi(2)
                                        + -10213. * param.s1 * param.s2.powi(3)
                                        + 1080. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (22. * param.s1.powi(3)
                                        + -119. * param.s1.powi(2) * param.s2
                                        + -234. * param.s1 * param.s2.powi(2)
                                        + -59. * param.s2.powi(3))
                                - param.s12.powi(7)
                                    * (103. * param.s1.powi(2)
                                        + 395. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2))))
                + param.m1_2
                    * (param.m2_2.powi(4)
                        * (80. * param.s12.powi(9)
                            + 30. * (param.s1 - param.s2).powi(9)
                            + -3.
                                * param.s12
                                * (param.s1 - param.s2).powi(7)
                                * (107. * param.s1 + 98. * param.s2)
                            + -5. * param.s12.powi(8) * (262. * param.s1 + 127. * param.s2)
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(5)
                                * (1586. * param.s1.powi(2)
                                    + 2255. * param.s1 * param.s2
                                    + 1289. * param.s2.powi(2))
                            + param.s12.powi(7)
                                * (369. * param.s1.powi(2)
                                    + 6635. * param.s1 * param.s2
                                    + 2244. * param.s2.powi(2))
                            + param.s12.powi(6)
                                * (7686. * param.s1.powi(3)
                                    + -9343. * param.s1.powi(2) * param.s2
                                    + -13498. * param.s1 * param.s2.powi(2)
                                    + -4639. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(3)
                                * (973. * param.s1.powi(3)
                                    + 1520. * param.s1.powi(2) * param.s2
                                    + 1313. * param.s1 * param.s2.powi(2)
                                    + 664. * param.s2.powi(3))
                            + param.s12.powi(5)
                                * (-14175. * param.s1.powi(4)
                                    + -5395. * param.s1.powi(3) * param.s2
                                    + 16503. * param.s1.powi(2) * param.s2.powi(2)
                                    + 13185. * param.s1 * param.s2.powi(3)
                                    + 6190. * param.s2.powi(4))
                            + 5. * param.s12.powi(4)
                                * (2184. * param.s1.powi(5)
                                    + 1147. * param.s1.powi(4) * param.s2
                                    + -348. * param.s1.powi(3) * param.s2.powi(2)
                                    + -1008. * param.s1.powi(2) * param.s2.powi(3)
                                    + -868. * param.s1 * param.s2.powi(4)
                                    + -1107. * param.s2.powi(5)))
                        + param.m2_2.powi(3)
                            * param.s12
                            * (-94. * param.s12.powi(9)
                                + -3.
                                    * (17. * param.s1 + -5. * param.s2)
                                    * (param.s1 - param.s2).powi(8)
                                + 10. * param.s12.powi(8) * (133. * param.s1 + 67. * param.s2)
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(6)
                                    * (187. * param.s1.powi(2)
                                        + 128. * param.s1 * param.s2
                                        + -51. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-6021. * param.s1.powi(3)
                                        + 25607. * param.s1.powi(2) * param.s2
                                        + 5243. * param.s1 * param.s2.powi(2)
                                        + 3863. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (17541. * param.s1.powi(4)
                                        + -31498. * param.s1.powi(3) * param.s2
                                        + -37260. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1446. * param.s1 * param.s2.powi(3)
                                        + -4565. * param.s2.powi(4))
                                + 5. * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(2)
                                    * (1949. * param.s1.powi(4)
                                        + 3842. * param.s1.powi(3) * param.s2
                                        + 3126. * param.s1.powi(2) * param.s2.powi(2)
                                        + 838. * param.s1 * param.s2.powi(3)
                                        + -395. * param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (-18471. * param.s1.powi(5)
                                        + 5195. * param.s1.powi(4) * param.s2
                                        + 56310. * param.s1.powi(3) * param.s2.powi(2)
                                        + 4950. * param.s1.powi(2) * param.s2.powi(3)
                                        + -8135. * param.s1 * param.s2.powi(4)
                                        + 3639. * param.s2.powi(5))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(4)
                                    * (2899. * param.s1.powi(3)
                                        + 3891. * param.s1.powi(2) * param.s2
                                        + 1479. * param.s1 * param.s2.powi(2)
                                        + -709. * param.s2.powi(3))
                                - param.s12.powi(7)
                                    * (1641. * param.s1.powi(2)
                                        + 4840. * param.s1 * param.s2
                                        + 2109. * param.s2.powi(2)))
                        + 3. * param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (3. * param.s12.powi(9)
                                + 5. * param.s12.powi(8) * (-6. * param.s1 + param.s2)
                                + (param.s1 - param.s2).powi(7)
                                    * (5. * param.s1.powi(2)
                                        + 12. * param.s1 * param.s2
                                        + -2. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (389. * param.s1.powi(3)
                                        + -3046. * param.s1.powi(2) * param.s2
                                        + 1606. * param.s1 * param.s2.powi(2)
                                        + 347. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (327. * param.s1.powi(4)
                                        + 2181. * param.s1.powi(3) * param.s2
                                        + 2564. * param.s1.powi(2) * param.s2.powi(2)
                                        + 591. * param.s1 * param.s2.powi(3)
                                        + -113. * param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (1357. * param.s1.powi(5)
                                        + -8480. * param.s1.powi(4) * param.s2
                                        + -12015. * param.s1.powi(3) * param.s2.powi(2)
                                        + 15505. * param.s1.powi(2) * param.s2.powi(3)
                                        + -140. * param.s1 * param.s2.powi(4)
                                        + 557. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(3)
                                    * (182. * param.s1.powi(6)
                                        + -93. * param.s1.powi(5) * param.s2
                                        + -3367. * param.s1.powi(4) * param.s2.powi(2)
                                        + 1952. * param.s1.powi(3) * param.s2.powi(3)
                                        + 1548. * param.s1.powi(2) * param.s2.powi(4)
                                        + -287. * param.s1 * param.s2.powi(5)
                                        + 65. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (1080. * param.s1.powi(4)
                                        + -10213. * param.s1.powi(3) * param.s2
                                        + 3805. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1749. * param.s1 * param.s2.powi(3)
                                        + 577. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (59. * param.s1.powi(3)
                                        + 234. * param.s1.powi(2) * param.s2
                                        + 119. * param.s1 * param.s2.powi(2)
                                        + -22. * param.s2.powi(3))
                                - param.s12.powi(7)
                                    * (2. * param.s1.powi(2)
                                        + 395. * param.s1 * param.s2
                                        + 103. * param.s2.powi(2)))
                        + param.m2_2
                            * param.s12.powi(3)
                            * (4. * param.s12.powi(9)
                                + -40. * param.s12.powi(8) * (param.s1 + param.s2)
                                + param.s12.powi(7)
                                    * (129. * param.s1.powi(2)
                                        + 280. * param.s1 * param.s2
                                        + 129. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-155. * param.s1.powi(3)
                                        + 277. * param.s1.powi(2) * param.s2
                                        + 277. * param.s1 * param.s2.powi(2)
                                        + -155. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(6)
                                    * (param.s1.powi(3)
                                        + -11. * param.s1.powi(2) * param.s2
                                        + -11. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + -3.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (15. * param.s1.powi(4)
                                        + -146. * param.s1.powi(3) * param.s2
                                        + -538. * param.s1.powi(2) * param.s2.powi(2)
                                        + -146. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (191. * param.s1.powi(5)
                                        + -583. * param.s1.powi(4) * param.s2
                                        + 13652. * param.s1.powi(3) * param.s2.powi(2)
                                        + 13652. * param.s1.powi(2) * param.s2.powi(3)
                                        + -583. * param.s1 * param.s2.powi(4)
                                        + 191. * param.s2.powi(5))
                                + param.s12.powi(4)
                                    * (321. * param.s1.powi(5)
                                        + 3755. * param.s1.powi(4) * param.s2
                                        + -10920. * param.s1.powi(3) * param.s2.powi(2)
                                        + -10920. * param.s1.powi(2) * param.s2.powi(3)
                                        + 3755. * param.s1 * param.s2.powi(4)
                                        + 321. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(3)
                                    * (73. * param.s1.powi(6)
                                        + 236. * param.s1.powi(5) * param.s2
                                        + 3299. * param.s1.powi(4) * param.s2.powi(2)
                                        + -9328. * param.s1.powi(3) * param.s2.powi(3)
                                        + 3299. * param.s1.powi(2) * param.s2.powi(4)
                                        + 236. * param.s1 * param.s2.powi(5)
                                        + 73. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (43. * param.s1.powi(4)
                                        + 2694. * param.s1.powi(3) * param.s2
                                        + -12198. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2694. * param.s1 * param.s2.powi(3)
                                        + 43. * param.s2.powi(4)))
                        + param.s12.powi(4)
                            * (param.s12.powi(9)
                                + -10. * param.s12.powi(8) * (param.s1 + param.s2)
                                + param.s12.powi(7)
                                    * (39. * param.s1.powi(2)
                                        + 70. * param.s1 * param.s2
                                        + 45. * param.s2.powi(2))
                                + 3. * (param.s1 - param.s2).powi(5)
                                    * (param.s1.powi(4)
                                        + -6. * param.s1.powi(3) * param.s2
                                        + 17. * param.s1.powi(2) * param.s2.powi(2)
                                        + 20. * param.s1 * param.s2.powi(3)
                                        + -2. * param.s2.powi(4))
                                + param.s12.powi(5)
                                    * (77. * param.s1.powi(4)
                                        + -102. * param.s1.powi(3) * param.s2
                                        + -456. * param.s1.powi(2) * param.s2.powi(2)
                                        + 30. * param.s1 * param.s2.powi(3)
                                        + 149. * param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (-21. * param.s1.powi(5)
                                        + 455. * param.s1.powi(4) * param.s2
                                        + 1245. * param.s1.powi(3) * param.s2.powi(2)
                                        + -3855. * param.s1.powi(2) * param.s2.powi(3)
                                        + 740. * param.s1 * param.s2.powi(4)
                                        + -96. * param.s2.powi(5))
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (3. * param.s1.powi(5)
                                        + -4. * param.s1.powi(4) * param.s2
                                        + -81. * param.s1.powi(3) * param.s2.powi(2)
                                        + -423. * param.s1.powi(2) * param.s2.powi(3)
                                        + -5. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(3)
                                    * (7. * param.s1.powi(6)
                                        + 86. * param.s1.powi(5) * param.s2
                                        + -31. * param.s1.powi(4) * param.s2.powi(2)
                                        + 968. * param.s1.powi(3) * param.s2.powi(3)
                                        + -1465. * param.s1.powi(2) * param.s2.powi(4)
                                        + 230. * param.s1 * param.s2.powi(5)
                                        + param.s2.powi(6))
                                + param.s12.powi(2)
                                    * (41. * param.s1.powi(7)
                                        + 85. * param.s1.powi(6) * param.s2
                                        + -1320. * param.s1.powi(5) * param.s2.powi(2)
                                        + 8010. * param.s1.powi(4) * param.s2.powi(3)
                                        + -6765. * param.s1.powi(3) * param.s2.powi(4)
                                        + -621. * param.s1.powi(2) * param.s2.powi(5)
                                        + 520. * param.s1 * param.s2.powi(6)
                                        + 50. * param.s2.powi(7))
                                - param.s12.powi(6)
                                    * (77. * param.s1.powi(3)
                                        + 113. * param.s1.powi(2) * param.s2
                                        + 200. * param.s1 * param.s2.powi(2)
                                        + 110. * param.s2.powi(3))))
                + 10.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (param.m2_2.powi(2)
                        * (110. * param.s12.powi(6)
                            + 10. * param.s12.powi(5) * (55. * param.s1 + -41. * param.s2)
                            + -3. * (param.s1 - param.s2).powi(6)
                            + 6. * param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (13. * param.s1 + 7. * param.s2)
                            + 2. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (292. * param.s1.powi(2)
                                    + 385. * param.s1 * param.s2
                                    + -14. * param.s2.powi(2))
                            + param.s12.powi(4)
                                * (-699. * param.s1.powi(2)
                                    + -400. * param.s1 * param.s2
                                    + 537. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (155. * param.s1.powi(3)
                                    + -575. * param.s1.powi(2) * param.s2
                                    + 226. * param.s1 * param.s2.powi(2)
                                    + 62. * param.s2.powi(3)))
                        + param.m1_2.powi(2)
                            * (110. * param.s12.powi(6)
                                + -3. * (param.s1 - param.s2).powi(6)
                                + 6. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (7. * param.s1 + 13. * param.s2)
                                + param.s12.powi(5) * (-410. * param.s1 + 550. * param.s2)
                                + param.s12.powi(4)
                                    * (537. * param.s1.powi(2)
                                        + -400. * param.s1 * param.s2
                                        + -699. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (14. * param.s1.powi(2)
                                        + -385. * param.s1 * param.s2
                                        + -292. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (62. * param.s1.powi(3)
                                        + 226. * param.s1.powi(2) * param.s2
                                        + -575. * param.s1 * param.s2.powi(2)
                                        + 155. * param.s2.powi(3)))
                        + -2.
                            * param.m2_2
                            * param.s12
                            * (56. * param.s12.powi(6)
                                + param.s12.powi(5) * (142. * param.s1 + -149. * param.s2)
                                + -6.
                                    * (param.s1 - param.s2).powi(5)
                                    * (2. * param.s1 + param.s2)
                                + -3.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (58. * param.s1.powi(2)
                                        + 129. * param.s1 * param.s2
                                        + 17. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-552. * param.s1.powi(2)
                                        + 608. * param.s1 * param.s2
                                        + 42. * param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (128. * param.s1.powi(3)
                                        + 451. * param.s1.powi(2) * param.s2
                                        + -722. * param.s1 * param.s2.powi(2)
                                        + 101. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (142. * param.s1.powi(4)
                                        + -775. * param.s1.powi(3) * param.s2
                                        + 498. * param.s1.powi(2) * param.s2.powi(2)
                                        + 239. * param.s1 * param.s2.powi(3)
                                        + -104. * param.s2.powi(4)))
                        + param.s12.powi(2)
                            * (20. * param.s12.powi(6)
                                + 4. * param.s12.powi(5) * (param.s1 + param.s2)
                                + param.s12.powi(4)
                                    * (-183. * param.s1.powi(2)
                                        + 572. * param.s1 * param.s2
                                        + -183. * param.s2.powi(2))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (11. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 11. * param.s2.powi(2))
                                + -12.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (4. * param.s1.powi(3)
                                        + -49. * param.s1.powi(2) * param.s2
                                        + -49. * param.s1 * param.s2.powi(2)
                                        + 4. * param.s2.powi(3))
                                + 4. * param.s12.powi(3)
                                    * (73. * param.s1.powi(3)
                                        + -127. * param.s1.powi(2) * param.s2
                                        + -127. * param.s1 * param.s2.powi(2)
                                        + 73. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(2)
                                    * (59. * param.s1.powi(4)
                                        + 367. * param.s1.powi(3) * param.s2
                                        + -1032. * param.s1.powi(2) * param.s2.powi(2)
                                        + 367. * param.s1 * param.s2.powi(3)
                                        + 59. * param.s2.powi(4)))
                        + 2. * param.m1_2
                            * (param.m2_2
                                * (190. * param.s12.powi(6)
                                    + 3. * (param.s1 - param.s2).powi(6)
                                    + -220. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + -60.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (param.s1 + param.s2)
                                    + param.s12.powi(4)
                                        * (-429. * param.s1.powi(2)
                                            + 1600. * param.s1 * param.s2
                                            + -429. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (94. * param.s1.powi(2)
                                            + 475. * param.s1 * param.s2
                                            + 94. * param.s2.powi(2))
                                    + 88.
                                        * param.s12.powi(3)
                                        * (8. * param.s1.powi(3)
                                            + -11. * param.s1.powi(2) * param.s2
                                            + -11. * param.s1 * param.s2.powi(2)
                                            + 8. * param.s2.powi(3)))
                                - param.s12
                                    * (56. * param.s12.powi(6)
                                        + 6. * (param.s1 - param.s2).powi(5)
                                            * (param.s1 + 2. * param.s2)
                                        + param.s12.powi(5)
                                            * (-149. * param.s1 + 142. * param.s2)
                                        + param.s12.powi(4)
                                            * (42. * param.s1.powi(2)
                                                + 608. * param.s1 * param.s2
                                                + -552. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (17. * param.s1.powi(2)
                                                + 129. * param.s1 * param.s2
                                                + 58. * param.s2.powi(2))
                                        + 2. * param.s12.powi(3)
                                            * (101. * param.s1.powi(3)
                                                + -722. * param.s1.powi(2) * param.s2
                                                + 451. * param.s1 * param.s2.powi(2)
                                                + 128. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-208. * param.s1.powi(4)
                                                + 478. * param.s1.powi(3) * param.s2
                                                + 996. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1550. * param.s1 * param.s2.powi(3)
                                                + 284. * param.s2.powi(4)))))
                + -15.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m2_2.powi(2)
                        * param.s12
                        * (-14. * param.s12.powi(7)
                            + param.s12.powi(6) * (-353. * param.s1 + 65. * param.s2)
                            + 2. * param.s12.powi(5)
                                * (236. * param.s1.powi(2)
                                    + 166. * param.s1 * param.s2
                                    + -57. * param.s2.powi(2))
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (58. * param.s1.powi(2)
                                    + 57. * param.s1 * param.s2
                                    + 5. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (353. * param.s1.powi(3)
                                    + 1863. * param.s1.powi(2) * param.s2
                                    + 457. * param.s1 * param.s2.powi(2)
                                    + -21. * param.s2.powi(3))
                            + param.s12.powi(4)
                                * (741. * param.s1.powi(3)
                                    + -3171. * param.s1.powi(2) * param.s2
                                    + 1045. * param.s1 * param.s2.powi(2)
                                    + 85. * param.s2.powi(3))
                            + -2.
                                * param.s12.powi(3)
                                * (655. * param.s1.powi(4)
                                    + -969. * param.s1.powi(3) * param.s2
                                    + -1018. * param.s1.powi(2) * param.s2.powi(2)
                                    + 799. * param.s1 * param.s2.powi(3)
                                    + 5. * param.s2.powi(4))
                            - (param.s1 - param.s2).powi(6) * (5. * param.s1 + param.s2))
                        + param.m2_2.powi(3)
                            * (10. * param.s12.powi(7)
                                + 5. * param.s12.powi(6) * (59. * param.s1 + -11. * param.s2)
                                + (param.s1 - param.s2).powi(7)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (8. * param.s1 + 5. * param.s2)
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (33. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + 2. * param.s12.powi(5)
                                    * (26. * param.s1.powi(2)
                                        + -400. * param.s1 * param.s2
                                        + 63. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-893. * param.s1.powi(3)
                                        + 933. * param.s1.powi(2) * param.s2
                                        + 643. * param.s1 * param.s2.powi(2)
                                        + -155. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (193. * param.s1.powi(4)
                                        + 377. * param.s1.powi(3) * param.s2
                                        + -582. * param.s1.powi(2) * param.s2.powi(2)
                                        + -43. * param.s1 * param.s2.powi(3)
                                        + 55. * param.s2.powi(4)))
                        + param.m1_2.powi(3)
                            * (10. * param.s12.powi(7)
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (5. * param.s1 + 8. * param.s2)
                                + param.s12.powi(6) * (-55. * param.s1 + 295. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (63. * param.s1.powi(2)
                                        + -400. * param.s1 * param.s2
                                        + 26. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (9. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + 33. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-155. * param.s1.powi(3)
                                        + 643. * param.s1.powi(2) * param.s2
                                        + 933. * param.s1 * param.s2.powi(2)
                                        + -893. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (55. * param.s1.powi(4)
                                        + -43. * param.s1.powi(3) * param.s2
                                        + -582. * param.s1.powi(2) * param.s2.powi(2)
                                        + 377. * param.s1 * param.s2.powi(3)
                                        + 193. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(7))
                        + param.s12.powi(3)
                            * (-3. * param.s12.powi(6) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(5)
                                    * (6. * param.s1.powi(2)
                                        + -49. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))
                                + 10.
                                    * param.s1
                                    * param.s12.powi(3)
                                    * param.s2
                                    * (33. * param.s1.powi(2)
                                        + -94. * param.s1 * param.s2
                                        + 33. * param.s2.powi(2))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (param.s1.powi(3)
                                        + 19. * param.s1.powi(2) * param.s2
                                        + 19. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + -3.
                                    * param.s12.powi(4)
                                    * (5. * param.s1.powi(3)
                                        + -27. * param.s1.powi(2) * param.s2
                                        + -27. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + -12.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(4)
                                        + -6. * param.s1.powi(3) * param.s2
                                        + -50. * param.s1.powi(2) * param.s2.powi(2)
                                        + -6. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (15. * param.s1.powi(5)
                                        + -451. * param.s1.powi(4) * param.s2
                                        + 556. * param.s1.powi(3) * param.s2.powi(2)
                                        + 556. * param.s1.powi(2) * param.s2.powi(3)
                                        + -451. * param.s1 * param.s2.powi(4)
                                        + 15. * param.s2.powi(5)))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (4. * param.s12.powi(7)
                                + param.s12.powi(6) * (97. * param.s1 + -7. * param.s2)
                                + param.s12.powi(5)
                                    * (-284. * param.s1.powi(2)
                                        + 350. * param.s1 * param.s2
                                        + -24. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(5)
                                    * (13. * param.s1.powi(2)
                                        + 22. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (23. * param.s1.powi(3)
                                        + 187. * param.s1.powi(2) * param.s2
                                        + 94. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (99. * param.s1.powi(3)
                                        + 1077. * param.s1.powi(2) * param.s2
                                        + -1229. * param.s1 * param.s2.powi(2)
                                        + 85. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (182. * param.s1.powi(4)
                                        + -1395. * param.s1.powi(3) * param.s2
                                        + 934. * param.s1.powi(2) * param.s2.powi(2)
                                        + 317. * param.s1 * param.s2.powi(3)
                                        + -50. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-385. * param.s1.powi(5)
                                        + 941. * param.s1.powi(4) * param.s2
                                        + 1980. * param.s1.powi(3) * param.s2.powi(2)
                                        + -3104. * param.s1.powi(2) * param.s2.powi(3)
                                        + 517. * param.s1 * param.s2.powi(4)
                                        + 51. * param.s2.powi(5)))
                        + param.m1_2.powi(2)
                            * (param.m2_2
                                * (190. * param.s12.powi(7)
                                    + 3. * (param.s1 - param.s2).powi(7)
                                    + -6.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(5)
                                        * (6. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(6) * (-655. * param.s1 + 215. * param.s2)
                                    + 2. * param.s12.powi(5)
                                        * (348. * param.s1.powi(2)
                                            + 800. * param.s1 * param.s2
                                            + -777. * param.s2.powi(2))
                                    + 3. * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(3)
                                        * (73. * param.s1.powi(2)
                                            + 184. * param.s1 * param.s2
                                            + 113. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-31. * param.s1.powi(3)
                                            + -3737. * param.s1.powi(2) * param.s2
                                            + 1801. * param.s1 * param.s2.powi(2)
                                            + 1439. * param.s2.powi(3))
                                    + -2.
                                        * param.s12.powi(3)
                                        * (193. * param.s1.powi(4)
                                            + -955. * param.s1.powi(3) * param.s2
                                            + -762. * param.s1.powi(2) * param.s2.powi(2)
                                            + 1529. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4)))
                                + param.s12
                                    * (-14. * param.s12.powi(7)
                                        + param.s12.powi(6)
                                            * (65. * param.s1 + -353. * param.s2)
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (5. * param.s1.powi(2)
                                                + 57. * param.s1 * param.s2
                                                + 58. * param.s2.powi(2))
                                        + param.s12.powi(5)
                                            * (-114. * param.s1.powi(2)
                                                + 332. * param.s1 * param.s2
                                                + 472. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (85. * param.s1.powi(3)
                                                + 1045. * param.s1.powi(2) * param.s2
                                                + -3171. * param.s1 * param.s2.powi(2)
                                                + 741. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (5. * param.s1.powi(4)
                                                + 799. * param.s1.powi(3) * param.s2
                                                + -1018. * param.s1.powi(2) * param.s2.powi(2)
                                                + -969. * param.s1 * param.s2.powi(3)
                                                + 655. * param.s2.powi(4))
                                        - param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (21. * param.s1.powi(3)
                                                + -457. * param.s1.powi(2) * param.s2
                                                + -1863. * param.s1 * param.s2.powi(2)
                                                + -353. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(6)
                                            * (param.s1 + 5. * param.s2)))
                        + param.m1_2
                            * (param.m2_2.powi(2)
                                * (190. * param.s12.powi(7)
                                    + 5. * param.s12.powi(6)
                                        * (43. * param.s1 + -131. * param.s2)
                                    + -3. * (param.s1 - param.s2).powi(7)
                                    + 6. * param.s12
                                        * (param.s1 - param.s2).powi(5)
                                        * (7. * param.s1 + 6. * param.s2)
                                    + -3.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(3)
                                        * (113. * param.s1.powi(2)
                                            + 184. * param.s1 * param.s2
                                            + 73. * param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (-1554. * param.s1.powi(2)
                                            + 1600. * param.s1 * param.s2
                                            + 696. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (1439. * param.s1.powi(3)
                                            + 1801. * param.s1.powi(2) * param.s2
                                            + -3737. * param.s1 * param.s2.powi(2)
                                            + -31. * param.s2.powi(3))
                                    + 2. * param.s12.powi(3)
                                        * (5. * param.s1.powi(4)
                                            + -1529. * param.s1.powi(3) * param.s2
                                            + 762. * param.s1.powi(2) * param.s2.powi(2)
                                            + 955. * param.s1 * param.s2.powi(3)
                                            + -193. * param.s2.powi(4)))
                                + -2.
                                    * param.m2_2
                                    * param.s12
                                    * (54. * param.s12.powi(7)
                                        + -32. * param.s12.powi(6) * (param.s1 + param.s2)
                                        + -3.
                                            * (param.s1 - param.s2).powi(6)
                                            * (param.s1 + param.s2)
                                        + param.s12.powi(5)
                                            * (-373. * param.s1.powi(2)
                                                + 1336. * param.s1 * param.s2
                                                + -373. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (17. * param.s1.powi(2)
                                                + 46. * param.s1 * param.s2
                                                + 17. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 658. * param.s1.powi(2) * param.s2
                                                + 658. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (729. * param.s1.powi(3)
                                                + -1367. * param.s1.powi(2) * param.s2
                                                + -1367. * param.s1 * param.s2.powi(2)
                                                + 729. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (218. * param.s1.powi(4)
                                                + 591. * param.s1.powi(3) * param.s2
                                                + -2146. * param.s1.powi(2) * param.s2.powi(2)
                                                + 591. * param.s1 * param.s2.powi(3)
                                                + 218. * param.s2.powi(4)))
                                + param.s12.powi(2)
                                    * (4. * param.s12.powi(7)
                                        + param.s12.powi(6) * (-7. * param.s1 + 97. * param.s2)
                                        + param.s12.powi(5)
                                            * (-24. * param.s1.powi(2)
                                                + 350. * param.s1 * param.s2
                                                + -284. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1.powi(3)
                                                + 94. * param.s1.powi(2) * param.s2
                                                + 187. * param.s1 * param.s2.powi(2)
                                                + 23. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (85. * param.s1.powi(3)
                                                + -1229. * param.s1.powi(2) * param.s2
                                                + 1077. * param.s1 * param.s2.powi(2)
                                                + 99. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-100. * param.s1.powi(4)
                                                + 634. * param.s1.powi(3) * param.s2
                                                + 1868. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2790. * param.s1 * param.s2.powi(3)
                                                + 364. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (51. * param.s1.powi(5)
                                                + 517. * param.s1.powi(4) * param.s2
                                                + -3104. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1980. * param.s1.powi(2) * param.s2.powi(3)
                                                + 941. * param.s1 * param.s2.powi(4)
                                                + -385. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(2)
                                                + 22. * param.s1 * param.s2
                                                + 13. * param.s2.powi(2)))))
                + -3.
                    * param.m0_2
                    * param.s12
                    * (param.m1_2.powi(4)
                        * (10. * param.s12.powi(8)
                            + 3. * (param.s1 - param.s2).powi(8)
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (14. * param.s1 + 19. * param.s2)
                            + -10. * param.s12.powi(7) * (7. * param.s1 + 22. * param.s2)
                            + param.s12.powi(6)
                                * (213. * param.s1.powi(2)
                                    + 970. * param.s1 * param.s2
                                    + -1477. * param.s2.powi(2))
                            + 5. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (23. * param.s1.powi(2)
                                    + 54. * param.s1 * param.s2
                                    + 49. * param.s2.powi(2))
                            + -10.
                                * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (27. * param.s1.powi(3)
                                    + 82. * param.s1.powi(2) * param.s2
                                    + 155. * param.s1 * param.s2.powi(2)
                                    + 126. * param.s2.powi(3))
                            + param.s12.powi(5)
                                * (-368. * param.s1.powi(3)
                                    + -1646. * param.s1.powi(2) * param.s2
                                    + 2244. * param.s1 * param.s2.powi(2)
                                    + 2702. * param.s2.powi(3))
                            + param.s12.powi(4)
                                * (395. * param.s1.powi(4)
                                    + 1260. * param.s1.powi(3) * param.s2
                                    + -334. * param.s1.powi(2) * param.s2.powi(2)
                                    + -4980. * param.s1 * param.s2.powi(3)
                                    + 35. * param.s2.powi(4)))
                        + param.m2_2.powi(4)
                            * (10. * param.s12.powi(8)
                                + 3. * (param.s1 - param.s2).powi(8)
                                + -10. * param.s12.powi(7) * (22. * param.s1 + 7. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(6)
                                    * (19. * param.s1 + 14. * param.s2)
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(4)
                                    * (49. * param.s1.powi(2)
                                        + 54. * param.s1 * param.s2
                                        + 23. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-1477. * param.s1.powi(2)
                                        + 970. * param.s1 * param.s2
                                        + 213. * param.s2.powi(2))
                                + 2. * param.s12.powi(5)
                                    * (1351. * param.s1.powi(3)
                                        + 1122. * param.s1.powi(2) * param.s2
                                        + -823. * param.s1 * param.s2.powi(2)
                                        + -184. * param.s2.powi(3))
                                + -10.
                                    * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(2)
                                    * (126. * param.s1.powi(3)
                                        + 155. * param.s1.powi(2) * param.s2
                                        + 82. * param.s1 * param.s2.powi(2)
                                        + 27. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (35. * param.s1.powi(4)
                                        + -4980. * param.s1.powi(3) * param.s2
                                        + -334. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1260. * param.s1 * param.s2.powi(3)
                                        + 395. * param.s2.powi(4)))
                        + 2. * param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (5. * param.s12.powi(8)
                                + -5. * param.s12.powi(7) * (16. * param.s1 + 5. * param.s2)
                                + (param.s1 - param.s2).powi(6)
                                    * (9. * param.s1.powi(2) + 7. * param.s1 * param.s2
                                        - param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-221. * param.s1.powi(2)
                                        + 85. * param.s1 * param.s2
                                        + 44. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (1436. * param.s1.powi(3)
                                        + -3053. * param.s1.powi(2) * param.s2
                                        + 402. * param.s1 * param.s2.powi(2)
                                        + -19. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (-2100. * param.s1.powi(4)
                                        + 2595. * param.s1.powi(3) * param.s2
                                        + 3448. * param.s1.powi(2) * param.s2.powi(2)
                                        + -845. * param.s1 * param.s2.powi(3)
                                        + -40. * param.s2.powi(4))
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (31. * param.s1.powi(4)
                                        + -731. * param.s1.powi(3) * param.s2
                                        + -617. * param.s1.powi(2) * param.s2.powi(2)
                                        + -8. * param.s2.powi(4)
                                        - param.s1 * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (196. * param.s1.powi(5)
                                        + 807. * param.s1.powi(4) * param.s2
                                        + -2240. * param.s1.powi(3) * param.s2.powi(2)
                                        + 604. * param.s1.powi(2) * param.s2.powi(3)
                                        + 92. * param.s1 * param.s2.powi(4)
                                        + 13. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (184. * param.s1.powi(3)
                                        + 361. * param.s1.powi(2) * param.s2
                                        + 66. * param.s1 * param.s2.powi(2)
                                        + -11. * param.s2.powi(3)))
                        + -2.
                            * param.m2_2.powi(3)
                            * param.s12
                            * (10. * param.s12.powi(8)
                                + (param.s1 - param.s2).powi(7) * (6. * param.s1 - param.s2)
                                + -5. * param.s12.powi(7) * (38. * param.s1 + 13. * param.s2)
                                + param.s12.powi(6)
                                    * (-844. * param.s1.powi(2)
                                        + 665. * param.s1 * param.s2
                                        + 181. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (3054. * param.s1.powi(3)
                                        + -1717. * param.s1.powi(2) * param.s2
                                        + -702. * param.s1 * param.s2.powi(2)
                                        + -281. * param.s2.powi(3))
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (148. * param.s1.powi(3)
                                        + 185. * param.s1.powi(2) * param.s2
                                        + 48. * param.s1 * param.s2.powi(2)
                                        + -11. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (-2280. * param.s1.powi(4)
                                        + -3735. * param.s1.powi(3) * param.s2
                                        + 5517. * param.s1.powi(2) * param.s2.powi(2)
                                        + -55. * param.s1 * param.s2.powi(3)
                                        + 265. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(3)
                                    * (82. * param.s1.powi(5)
                                        + -1163. * param.s1.powi(4) * param.s2
                                        + 632. * param.s1.powi(3) * param.s2.powi(2)
                                        + 536. * param.s1.powi(2) * param.s2.powi(3)
                                        + -118. * param.s1 * param.s2.powi(4)
                                        + 31. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (86. * param.s1.powi(2)
                                        + 55. * param.s1 * param.s2
                                        + -11. * param.s2.powi(2)))
                        + -2.
                            * param.m2_2
                            * param.s12.powi(3)
                            * (5. * param.s12.powi(7) * param.s2
                                + (param.s1 - param.s2).powi(5)
                                    * (6. * param.s1.powi(3)
                                        + 47. * param.s1.powi(2) * param.s2
                                        + 8. * param.s1 * param.s2.powi(2)
                                        - param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (114. * param.s1.powi(3)
                                        + -787. * param.s1.powi(2) * param.s2
                                        + 248. * param.s1 * param.s2.powi(2)
                                        + 69. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (36. * param.s1.powi(5)
                                        + -135. * param.s1.powi(4) * param.s2
                                        + -736. * param.s1.powi(3) * param.s2.powi(2)
                                        + 824. * param.s1.powi(2) * param.s2.powi(3)
                                        + -72. * param.s1 * param.s2.powi(4)
                                        + 11. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(2)
                                    * (12. * param.s1.powi(6)
                                        + 259. * param.s1.powi(5) * param.s2
                                        + -1183. * param.s1.powi(4) * param.s2.powi(2)
                                        + 780. * param.s1.powi(3) * param.s2.powi(3)
                                        + 200. * param.s1.powi(2) * param.s2.powi(4)
                                        + -71. * param.s1 * param.s2.powi(5)
                                        + 3. * param.s2.powi(6))
                                - param.s12.powi(4)
                                    * (210. * param.s1.powi(4)
                                        + -2085. * param.s1.powi(3) * param.s2
                                        + 1323. * param.s1.powi(2) * param.s2.powi(2)
                                        + 55. * param.s1 * param.s2.powi(3)
                                        + 85. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (6. * param.s1.powi(4)
                                        + -727. * param.s1.powi(3) * param.s2
                                        + -1235. * param.s1.powi(2) * param.s2.powi(2)
                                        + -83. * param.s1 * param.s2.powi(3)
                                        - param.s2.powi(4))
                                - param.s12.powi(6)
                                    * (24. * param.s1.powi(2)
                                        + 95. * param.s1 * param.s2
                                        + 29. * param.s2.powi(2)))
                        + param.s12.powi(4)
                            * (3. * param.s12.powi(6) * (param.s1.powi(2) + param.s2.powi(2))
                                + -6.
                                    * param.s12.powi(5)
                                    * (3. * param.s1.powi(3)
                                        + 11. * param.s1.powi(2) * param.s2
                                        + 11. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (param.s1.powi(4)
                                        + -14. * param.s1.powi(3) * param.s2
                                        + -74. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (45. * param.s1.powi(4)
                                        + 210. * param.s1.powi(3) * param.s2
                                        + -874. * param.s1.powi(2) * param.s2.powi(2)
                                        + 210. * param.s1 * param.s2.powi(3)
                                        + 45. * param.s2.powi(4))
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(5)
                                        + -19. * param.s1.powi(4) * param.s2
                                        + 166. * param.s1.powi(3) * param.s2.powi(2)
                                        + 166. * param.s1.powi(2) * param.s2.powi(3)
                                        + -19. * param.s1 * param.s2.powi(4)
                                        + 3. * param.s2.powi(5))
                                + -20.
                                    * param.s12.powi(3)
                                    * (3. * param.s1.powi(5)
                                        + 9. * param.s1.powi(4) * param.s2
                                        + -31. * param.s1.powi(3) * param.s2.powi(2)
                                        + -31. * param.s1.powi(2) * param.s2.powi(3)
                                        + 9. * param.s1 * param.s2.powi(4)
                                        + 3. * param.s2.powi(5))
                                + 5. * param.s12.powi(2)
                                    * (9. * param.s1.powi(6)
                                        + -12. * param.s1.powi(5) * param.s2
                                        + 319. * param.s1.powi(4) * param.s2.powi(2)
                                        + -752. * param.s1.powi(3) * param.s2.powi(3)
                                        + 319. * param.s1.powi(2) * param.s2.powi(4)
                                        + -12. * param.s1 * param.s2.powi(5)
                                        + 9. * param.s2.powi(6)))
                        + -2.
                            * param.m1_2.powi(3)
                            * (param.m2_2
                                * (70. * param.s12.powi(8)
                                    + -5.
                                        * param.s12.powi(7)
                                        * (83. * param.s1 + -207. * param.s2)
                                    + 6. * (param.s1 - param.s2).powi(8)
                                    + 2. * param.s12.powi(6)
                                        * (528. * param.s1.powi(2)
                                            + -1030. * param.s1 * param.s2
                                            + -1347. * param.s2.powi(2))
                                    + 10.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (28. * param.s1.powi(2)
                                            + 57. * param.s1 * param.s2
                                            + 41. * param.s2.powi(2))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (153. * param.s1.powi(3)
                                            + 443. * param.s1.powi(2) * param.s2
                                            + 625. * param.s1 * param.s2.powi(2)
                                            + 339. * param.s2.powi(3))
                                    + param.s12.powi(5)
                                        * (-1511. * param.s1.powi(3)
                                            + -77. * param.s1.powi(2) * param.s2
                                            + 9153. * param.s1 * param.s2.powi(2)
                                            + 939. * param.s2.powi(3))
                                    + 2. * param.s12.powi(4)
                                        * (670. * param.s1.powi(4)
                                            + 1045. * param.s1.powi(3) * param.s2
                                            + -3244. * param.s1.powi(2) * param.s2.powi(2)
                                            + -3095. * param.s1 * param.s2.powi(3)
                                            + 1000. * param.s2.powi(4))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (61. * param.s1 + 71. * param.s2))
                                + param.s12
                                    * (10. * param.s12.powi(8)
                                        + (param.s1 + -6. * param.s2)
                                            * (param.s1 - param.s2).powi(7)
                                        + -5.
                                            * param.s12.powi(7)
                                            * (13. * param.s1 + 38. * param.s2)
                                        + param.s12.powi(6)
                                            * (181. * param.s1.powi(2)
                                                + 665. * param.s1 * param.s2
                                                + -844. * param.s2.powi(2))
                                        + 5. * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (11. * param.s1.powi(3)
                                                + -48. * param.s1.powi(2) * param.s2
                                                + -185. * param.s1 * param.s2.powi(2)
                                                + -148. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (265. * param.s1.powi(4)
                                                + -55. * param.s1.powi(3) * param.s2
                                                + 5517. * param.s1.powi(2) * param.s2.powi(2)
                                                + -3735. * param.s1 * param.s2.powi(3)
                                                + -2280. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (31. * param.s1.powi(5)
                                                + -118. * param.s1.powi(4) * param.s2
                                                + 536. * param.s1.powi(3) * param.s2.powi(2)
                                                + 632. * param.s1.powi(2) * param.s2.powi(3)
                                                + -1163. * param.s1 * param.s2.powi(4)
                                                + 82. * param.s2.powi(5))
                                        - param.s12.powi(5)
                                            * (281. * param.s1.powi(3)
                                                + 702. * param.s1.powi(2) * param.s2
                                                + 1717. * param.s1 * param.s2.powi(2)
                                                + -3054. * param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (11. * param.s1.powi(2)
                                                + -55. * param.s1 * param.s2
                                                + -86. * param.s2.powi(2))))
                        + -2.
                            * param.m1_2.powi(2)
                            * (param.m2_2.powi(2)
                                * (370. * param.s12.powi(8)
                                    + -9. * (param.s1 - param.s2).powi(8)
                                    + -1015. * param.s12.powi(7) * (param.s1 + param.s2)
                                    + 99.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (param.s1 + param.s2)
                                    + -30.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (17. * param.s1.powi(2)
                                            + 29. * param.s1 * param.s2
                                            + 17. * param.s2.powi(2))
                                    + 2. * param.s12.powi(6)
                                        * (78. * param.s1.powi(2)
                                            + 3545. * param.s1 * param.s2
                                            + 78. * param.s2.powi(2))
                                    + 15.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (113. * param.s1.powi(3)
                                            + 277. * param.s1.powi(2) * param.s2
                                            + 277. * param.s1 * param.s2.powi(2)
                                            + 113. * param.s2.powi(3))
                                    + param.s12.powi(5)
                                        * (2189. * param.s1.powi(3)
                                            + -9227. * param.s1.powi(2) * param.s2
                                            + -9227. * param.s1 * param.s2.powi(2)
                                            + 2189. * param.s2.powi(3))
                                    + param.s12.powi(4)
                                        * (-2975. * param.s1.powi(4)
                                            + 1640. * param.s1.powi(3) * param.s2
                                            + 13542. * param.s1.powi(2) * param.s2.powi(2)
                                            + 1640. * param.s1 * param.s2.powi(3)
                                            + -2975. * param.s2.powi(4)))
                                + param.m2_2
                                    * param.s12
                                    * (-50. * param.s12.powi(8)
                                        + 5. * param.s12.powi(7)
                                            * (38. * param.s1 + -141. * param.s2)
                                        + (param.s1 - param.s2).powi(7)
                                            * (4. * param.s1 + 11. * param.s2)
                                        + param.s12.powi(6)
                                            * (-176. * param.s1.powi(2)
                                                + -1985. * param.s1 * param.s2
                                                + 2819. * param.s2.powi(2))
                                        + 5. * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (44. * param.s1.powi(3)
                                                + 351. * param.s1.powi(2) * param.s2
                                                + 524. * param.s1 * param.s2.powi(2)
                                                + 191. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (610. * param.s1.powi(4)
                                                + -8445. * param.s1.powi(3) * param.s2
                                                + -9717. * param.s1.powi(2) * param.s2.powi(2)
                                                + 17675. * param.s1 * param.s2.powi(3)
                                                + 165. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (106. * param.s1.powi(5)
                                                + -179. * param.s1.powi(4) * param.s2
                                                + -2988. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1568. * param.s1.powi(2) * param.s2.powi(3)
                                                + 1838. * param.s1 * param.s2.powi(4)
                                                + -345. * param.s2.powi(5))
                                        - param.s12.powi(5)
                                            * (224. * param.s1.powi(3)
                                                + -9147. * param.s1.powi(2) * param.s2
                                                + 6508. * param.s1 * param.s2.powi(2)
                                                + 3129. * param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (44. * param.s1.powi(2)
                                                + 205. * param.s1 * param.s2
                                                + 141. * param.s2.powi(2)))
                                + param.s12.powi(2)
                                    * (-5. * param.s12.powi(8)
                                        + 5. * param.s12.powi(7)
                                            * (5. * param.s1 + 16. * param.s2)
                                        + (param.s1 - param.s2).powi(6)
                                            * (param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + param.s12.powi(6)
                                            * (-44. * param.s1.powi(2)
                                                + -85. * param.s1 * param.s2
                                                + 221. * param.s2.powi(2))
                                        + param.s12.powi(5)
                                            * (19. * param.s1.powi(3)
                                                + -402. * param.s1.powi(2) * param.s2
                                                + 3053. * param.s1 * param.s2.powi(2)
                                                + -1436. * param.s2.powi(3))
                                        + 5. * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (8. * param.s1.powi(4)
                                                + param.s1.powi(3) * param.s2
                                                + 617. * param.s1.powi(2) * param.s2.powi(2)
                                                + 731. * param.s1 * param.s2.powi(3)
                                                + -31. * param.s2.powi(4))
                                        + param.s12.powi(4)
                                            * (40. * param.s1.powi(4)
                                                + 845. * param.s1.powi(3) * param.s2
                                                + -3448. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2595. * param.s1 * param.s2.powi(3)
                                                + 2100. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (13. * param.s1.powi(5)
                                                + 92. * param.s1.powi(4) * param.s2
                                                + 604. * param.s1.powi(3) * param.s2.powi(2)
                                                + -2240. * param.s1.powi(2) * param.s2.powi(3)
                                                + 807. * param.s1 * param.s2.powi(4)
                                                + 196. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (11. * param.s1.powi(3)
                                                + -66. * param.s1.powi(2) * param.s2
                                                + -361. * param.s1 * param.s2.powi(2)
                                                + -184. * param.s2.powi(3))))
                        + -2.
                            * param.m1_2
                            * (param.m2_2.powi(3)
                                * (70. * param.s12.powi(8)
                                    + 5. * param.s12.powi(7)
                                        * (207. * param.s1 + -83. * param.s2)
                                    + 6. * (param.s1 - param.s2).powi(8)
                                    + -2.
                                        * param.s12.powi(6)
                                        * (1347. * param.s1.powi(2)
                                            + 1030. * param.s1 * param.s2
                                            + -528. * param.s2.powi(2))
                                    + 10.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (41. * param.s1.powi(2)
                                            + 57. * param.s1 * param.s2
                                            + 28. * param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (939. * param.s1.powi(3)
                                            + 9153. * param.s1.powi(2) * param.s2
                                            + -77. * param.s1 * param.s2.powi(2)
                                            + -1511. * param.s2.powi(3))
                                    + -5.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (339. * param.s1.powi(3)
                                            + 625. * param.s1.powi(2) * param.s2
                                            + 443. * param.s1 * param.s2.powi(2)
                                            + 153. * param.s2.powi(3))
                                    + 2. * param.s12.powi(4)
                                        * (1000. * param.s1.powi(4)
                                            + -3095. * param.s1.powi(3) * param.s2
                                            + -3244. * param.s1.powi(2) * param.s2.powi(2)
                                            + 1045. * param.s1 * param.s2.powi(3)
                                            + 670. * param.s2.powi(4))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (71. * param.s1 + 61. * param.s2))
                                + param.m2_2.powi(2)
                                    * param.s12
                                    * (-50. * param.s12.powi(8)
                                        + param.s12.powi(7)
                                            * (-705. * param.s1 + 190. * param.s2)
                                        + param.s12.powi(6)
                                            * (2819. * param.s1.powi(2)
                                                + -1985. * param.s1 * param.s2
                                                + -176. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (141. * param.s1.powi(2)
                                                + 205. * param.s1 * param.s2
                                                + 44. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (191. * param.s1.powi(3)
                                                + 524. * param.s1.powi(2) * param.s2
                                                + 351. * param.s1 * param.s2.powi(2)
                                                + 44. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (165. * param.s1.powi(4)
                                                + 17675. * param.s1.powi(3) * param.s2
                                                + -9717. * param.s1.powi(2) * param.s2.powi(2)
                                                + -8445. * param.s1 * param.s2.powi(3)
                                                + 610. * param.s2.powi(4))
                                        + 5. * param.s12.powi(3)
                                            * (345. * param.s1.powi(5)
                                                + -1838. * param.s1.powi(4) * param.s2
                                                + -1568. * param.s1.powi(3) * param.s2.powi(2)
                                                + 2988. * param.s1.powi(2) * param.s2.powi(3)
                                                + 179. * param.s1 * param.s2.powi(4)
                                                + -106. * param.s2.powi(5))
                                        - param.s12.powi(5)
                                            * (3129. * param.s1.powi(3)
                                                + 6508. * param.s1.powi(2) * param.s2
                                                + -9147. * param.s1 * param.s2.powi(2)
                                                + 224. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(7)
                                            * (11. * param.s1 + 4. * param.s2))
                                + param.m2_2
                                    * param.s12.powi(2)
                                    * (65. * param.s12.powi(7) * (param.s1 + param.s2)
                                        + 2. * (param.s1 - param.s2).powi(6)
                                            * (2. * param.s1.powi(2)
                                                + 11. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(6)
                                            * (148. * param.s1.powi(2)
                                                + -845. * param.s1 * param.s2
                                                + 148. * param.s2.powi(2))
                                        + 3. * param.s12.powi(5)
                                            * (167. * param.s1.powi(3)
                                                + -661. * param.s1.powi(2) * param.s2
                                                + -661. * param.s1 * param.s2.powi(2)
                                                + 167. * param.s2.powi(3))
                                        + 60.
                                            * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (2. * param.s1.powi(4)
                                                + -36. * param.s1.powi(3) * param.s2
                                                + -153. * param.s1.powi(2) * param.s2.powi(2)
                                                + -36. * param.s1 * param.s2.powi(3)
                                                + 2. * param.s2.powi(4))
                                        + -4.
                                            * param.s12.powi(4)
                                            * (85. * param.s1.powi(4)
                                                + 1120. * param.s1.powi(3) * param.s2
                                                + -3957. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1120. * param.s1 * param.s2.powi(3)
                                                + 85. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (param.s1.powi(5)
                                                + -1493. * param.s1.powi(4) * param.s2
                                                + 2020. * param.s1.powi(3) * param.s2.powi(2)
                                                + 2020. * param.s1.powi(2) * param.s2.powi(3)
                                                + -1493. * param.s1 * param.s2.powi(4)
                                                + param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (49. * param.s1.powi(3)
                                                + 551. * param.s1.powi(2) * param.s2
                                                + 551. * param.s1 * param.s2.powi(2)
                                                + 49. * param.s2.powi(3)))
                                + param.s12.powi(3)
                                    * (5. * param.s1 * param.s12.powi(7)
                                        + (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(3)
                                                + -8. * param.s1.powi(2) * param.s2
                                                + -47. * param.s1 * param.s2.powi(2)
                                                + -6. * param.s2.powi(3))
                                        + param.s12.powi(5)
                                            * (69. * param.s1.powi(3)
                                                + 248. * param.s1.powi(2) * param.s2
                                                + -787. * param.s1 * param.s2.powi(2)
                                                + 114. * param.s2.powi(3))
                                        + 5. * param.s12.powi(3)
                                            * (11. * param.s1.powi(5)
                                                + -72. * param.s1.powi(4) * param.s2
                                                + 824. * param.s1.powi(3) * param.s2.powi(2)
                                                + -736. * param.s1.powi(2) * param.s2.powi(3)
                                                + -135. * param.s1 * param.s2.powi(4)
                                                + 36. * param.s2.powi(5))
                                        + -5.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(6)
                                                + -71. * param.s1.powi(5) * param.s2
                                                + 200. * param.s1.powi(4) * param.s2.powi(2)
                                                + 780. * param.s1.powi(3) * param.s2.powi(3)
                                                + -1183. * param.s1.powi(2) * param.s2.powi(4)
                                                + 259. * param.s1 * param.s2.powi(5)
                                                + 12. * param.s2.powi(6))
                                        - param.s12.powi(4)
                                            * (85. * param.s1.powi(4)
                                                + 55. * param.s1.powi(3) * param.s2
                                                + 1323. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2085. * param.s1 * param.s2.powi(3)
                                                + 210. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(4)
                                                + 83. * param.s1.powi(3) * param.s2
                                                + 1235. * param.s1.powi(2) * param.s2.powi(2)
                                                + 727. * param.s1 * param.s2.powi(3)
                                                + -6. * param.s2.powi(4))
                                        - param.s12.powi(6)
                                            * (29. * param.s1.powi(2)
                                                + 95. * param.s1 * param.s2
                                                + 24. * param.s2.powi(2)))))
                - param.m1_2.powi(3)
                    * (param.m2_2
                        * param.s12
                        * (94. * param.s12.powi(9)
                            + -3.
                                * (5. * param.s1 + -17. * param.s2)
                                * (param.s1 - param.s2).powi(8)
                            + -10. * param.s12.powi(8) * (67. * param.s1 + 133. * param.s2)
                            + 3. * param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (51. * param.s1.powi(2)
                                    + -128. * param.s1 * param.s2
                                    + -187. * param.s2.powi(2))
                            + param.s12.powi(7)
                                * (2109. * param.s1.powi(2)
                                    + 4840. * param.s1 * param.s2
                                    + 1641. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (4565. * param.s1.powi(4)
                                    + -1446. * param.s1.powi(3) * param.s2
                                    + 37260. * param.s1.powi(2) * param.s2.powi(2)
                                    + 31498. * param.s1 * param.s2.powi(3)
                                    + -17541. * param.s2.powi(4))
                            + 5. * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (395. * param.s1.powi(4)
                                    + -838. * param.s1.powi(3) * param.s2
                                    + -3126. * param.s1.powi(2) * param.s2.powi(2)
                                    + -3842. * param.s1 * param.s2.powi(3)
                                    + -1949. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (3639. * param.s1.powi(5)
                                    + -8135. * param.s1.powi(4) * param.s2
                                    + 4950. * param.s1.powi(3) * param.s2.powi(2)
                                    + 56310. * param.s1.powi(2) * param.s2.powi(3)
                                    + 5195. * param.s1 * param.s2.powi(4)
                                    + -18471. * param.s2.powi(5))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (709. * param.s1.powi(3)
                                    + -1479. * param.s1.powi(2) * param.s2
                                    + -3891. * param.s1 * param.s2.powi(2)
                                    + -2899. * param.s2.powi(3))
                            - param.s12.powi(6)
                                * (3863. * param.s1.powi(3)
                                    + 5243. * param.s1.powi(2) * param.s2
                                    + 25607. * param.s1 * param.s2.powi(2)
                                    + -6021. * param.s2.powi(3)))
                        + param.m2_2.powi(2)
                            * (370. * param.s12.powi(9)
                                + -5. * param.s12.powi(8) * (503. * param.s1 + -97. * param.s2)
                                + -60. * (param.s1 - param.s2).powi(9)
                                + 6. * param.s12
                                    * (param.s1 - param.s2).powi(7)
                                    * (101. * param.s1 + 104. * param.s2)
                                + param.s12.powi(7)
                                    * (7656. * param.s1.powi(2)
                                        + 7090. * param.s1 * param.s2
                                        + -7344. * param.s2.powi(2))
                                + 20.
                                    * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(3)
                                    * (374. * param.s1.powi(3)
                                        + 700. * param.s1.powi(2) * param.s2
                                        + 736. * param.s1 * param.s2.powi(2)
                                        + 425. * param.s2.powi(3))
                                + param.s12.powi(6)
                                    * (-13811. * param.s1.powi(3)
                                        + -27227. * param.s1.powi(2) * param.s2
                                        + 8773. * param.s1 * param.s2.powi(2)
                                        + 18189. * param.s2.powi(3))
                                + 2. * param.s12.powi(5)
                                    * (8200. * param.s1.powi(4)
                                        + 16320. * param.s1.powi(3) * param.s2
                                        + 6771. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14680. * param.s1 * param.s2.powi(3)
                                        + -11175. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(4)
                                    * (2673. * param.s1.powi(5)
                                        + 2147. * param.s1.powi(4) * param.s2
                                        + 1692. * param.s1.powi(3) * param.s2.powi(2)
                                        + -708. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2453. * param.s1 * param.s2.powi(4)
                                        + -3351. * param.s2.powi(5))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(5)
                                    * (2761. * param.s1.powi(2)
                                        + 4540. * param.s1 * param.s2
                                        + 2959. * param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (19. * param.s12.powi(9)
                                + -5. * param.s12.powi(8) * (29. * param.s1 + 44. * param.s2)
                                + -3.
                                    * (param.s1 - param.s2).powi(7)
                                    * (param.s1.powi(2)
                                        + -4. * param.s1 * param.s2
                                        + 8. * param.s2.powi(2))
                                + param.s12.powi(7)
                                    * (486. * param.s1.powi(2)
                                        + 985. * param.s1 * param.s2
                                        + 1275. * param.s2.powi(2))
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (11. * param.s1.powi(3)
                                        + -36. * param.s1.powi(2) * param.s2
                                        + 55. * param.s1 * param.s2.powi(2)
                                        + 100. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (1148. * param.s1.powi(4)
                                        + 477. * param.s1.powi(3) * param.s2
                                        + -2319. * param.s1.powi(2) * param.s2.powi(2)
                                        + 12535. * param.s1 * param.s2.powi(3)
                                        + -459. * param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (-924. * param.s1.powi(5)
                                        + 1295. * param.s1.powi(4) * param.s2
                                        + 3255. * param.s1.powi(3) * param.s2.powi(2)
                                        + 6195. * param.s1.powi(2) * param.s2.powi(3)
                                        + -24365. * param.s1 * param.s2.powi(4)
                                        + 4896. * param.s2.powi(5))
                                + 5. * param.s12.powi(3)
                                    * (98. * param.s1.powi(6)
                                        + -353. * param.s1.powi(5) * param.s2
                                        + 229. * param.s1.powi(4) * param.s2.powi(2)
                                        + -3120. * param.s1.powi(3) * param.s2.powi(3)
                                        + 1280. * param.s1.powi(2) * param.s2.powi(4)
                                        + 2885. * param.s1 * param.s2.powi(5)
                                        + -1019. * param.s2.powi(6))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (166. * param.s1.powi(4)
                                        + -487. * param.s1.powi(3) * param.s2
                                        + 666. * param.s1.powi(2) * param.s2.powi(2)
                                        + 3155. * param.s1 * param.s2.powi(3)
                                        + 2050. * param.s2.powi(4))
                                - param.s12.powi(6)
                                    * (938. * param.s1.powi(3)
                                        + 1517. * param.s1.powi(2) * param.s2
                                        + 1595. * param.s1 * param.s2.powi(2)
                                        + 2190. * param.s2.powi(3)))))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s12.powi(5)
                    * (14.
                        * param.m1_2.powi(6)
                        * param.s2.powi(3)
                        * (2. * param.s1.powi(2)
                            + 2. * param.s12.powi(2)
                            + 5. * param.s1 * param.s2
                            + 2. * param.s2.powi(2)
                            + -4. * param.s12 * (param.s1 + param.s2))
                        + param.m0_2.powi(6)
                            * param.s12
                            * (10. * param.s12.powi(4)
                                + 3. * (param.s1 - param.s2).powi(4)
                                + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 9. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (-17. * param.s1.powi(2)
                                        + 40. * param.s1 * param.s2
                                        + -17. * param.s2.powi(2)))
                        + 21.
                            * param.m1_2.powi(5)
                            * param.s2.powi(2)
                            * (param.s2
                                * (-5. * param.s1.powi(3)
                                    + -3. * param.s12.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + 7. * param.s1 * param.s2.powi(2)
                                    + 3. * param.s2.powi(3)
                                    + param.s12.powi(2) * (param.s1 + 9. * param.s2)
                                    + param.s12
                                        * (7. * param.s1.powi(2)
                                            + -8. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2)))
                                + 3. * param.m2_2
                                    * (param.s12.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3)))
                        + 15.
                            * param.m1_2.powi(4)
                            * param.s2
                            * (-3.
                                * param.m2_2
                                * param.s2
                                * (-5. * param.s1.powi(4)
                                    + 2. * param.s12.powi(4)
                                    + -15. * param.s1.powi(3) * param.s2
                                    + 5. * param.s1.powi(2) * param.s2.powi(2)
                                    + 13. * param.s1 * param.s2.powi(3)
                                    + 2. * param.s2.powi(4)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 15. * param.s1 * param.s2
                                            + 12. * param.s2.powi(2))
                                    + param.s12
                                        * (13. * param.s1.powi(3)
                                            + 8. * param.s1.powi(2) * param.s2
                                            + -27. * param.s1 * param.s2.powi(2)
                                            + -8. * param.s2.powi(3))
                                    - param.s12.powi(3) * (param.s1 + 8. * param.s2))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 10. * param.s1.powi(3) * param.s2
                                        + 20. * param.s1.powi(2) * param.s2.powi(2)
                                        + 10. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1.powi(3)
                                                + 6. * param.s1.powi(2) * param.s2
                                                + 6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + param.s2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3)
                                            * (3. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (10. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-17. * param.s1.powi(2)
                                                + -9. * param.s1 * param.s2
                                                + 18. * param.s2.powi(2))
                                        - param.s12
                                            * (5. * param.s1.powi(3)
                                                + -40. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 12. * param.s2.powi(3))))
                        + -3.
                            * param.m0_2.powi(5)
                            * (3.
                                * param.s12
                                * (-2. * param.s12.powi(5)
                                    + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + 10. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + -16. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-7. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -7. * param.s2.powi(3)))
                                + param.m1_2
                                    * (10. * param.s12.powi(5)
                                        + 10. * param.s12.powi(4) * (-2. * param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (17. * param.s1.powi(3)
                                                + -63. * param.s1.powi(2) * param.s2
                                                + 45. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (7. * param.s1 + 13. * param.s2)
                                        - (param.s1 - param.s2).powi(5))
                                + param.m2_2
                                    * (10. * param.s12.powi(5)
                                        + 10. * param.s12.powi(4) * (param.s1 + -2. * param.s2)
                                        + (param.s1 - param.s2).powi(5)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (13. * param.s1 + 7. * param.s2)
                                        + param.s12.powi(3)
                                            * (-35. * param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 45. * param.s1.powi(2) * param.s2
                                                + -63. * param.s1 * param.s2.powi(2)
                                                + 17. * param.s2.powi(3))))
                        + 10.
                            * param.m1_2.powi(3)
                            * (-3.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (param.s12.powi(4) * (-7. * param.s1 + 5. * param.s2)
                                    + param.s12.powi(3)
                                        * (17. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -10. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (10. * param.s1.powi(3)
                                            + 30. * param.s1.powi(2) * param.s2
                                            + 15. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (param.s1.powi(3)
                                            + -63. * param.s1.powi(2) * param.s2
                                            + 18. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    + param.s12
                                        * (-20. * param.s1.powi(4)
                                            + 40. * param.s1.powi(3) * param.s2
                                            + 45. * param.s1.powi(2) * param.s2.powi(2)
                                            + -32. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4))
                                    - param.s12.powi(5))
                                + param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + -25. * param.s1.powi(4) * param.s2
                                        + -100. * param.s1.powi(3) * param.s2.powi(2)
                                        + -100. * param.s1.powi(2) * param.s2.powi(3)
                                        + -25. * param.s1 * param.s2.powi(4)
                                        + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + 10.
                                            * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -10.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 5. * param.s12
                                            * (param.s1.powi(4)
                                                + 16. * param.s1.powi(3) * param.s2
                                                + 36. * param.s1.powi(2) * param.s2.powi(2)
                                                + 16. * param.s1 * param.s2.powi(3)
                                                + param.s2.powi(4))
                                        - param.s2.powi(5)
                                        - param.s1.powi(5))
                                + param.s2.powi(3)
                                    * (param.s12.powi(4) * (-13. * param.s1 + 5. * param.s2)
                                        + param.s12.powi(2)
                                            * (35. * param.s1.powi(3)
                                                + -45. * param.s1.powi(2) * param.s2
                                                + -18. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - param.s12
                                            * (10. * param.s1.powi(4)
                                                + 40. * param.s1.powi(3) * param.s2
                                                + -63. * param.s1.powi(2) * param.s2.powi(2)
                                                + 8. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3)
                                            * (10. * param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(5))
                                + -3.
                                    * param.m2_2.powi(2)
                                    * param.s2
                                    * (5. * param.s1.powi(5)
                                        + param.s12.powi(5)
                                        + param.s12.powi(4) * (param.s1 + -5. * param.s2)
                                        + 35. * param.s1.powi(4) * param.s2
                                        + 20. * param.s1.powi(3) * param.s2.powi(2)
                                        + -40. * param.s1.powi(2) * param.s2.powi(3)
                                        + -19. * param.s1 * param.s2.powi(4)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (7. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (13. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -27. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3))
                                        + param.s12
                                            * (-19. * param.s1.powi(4)
                                                + -64. * param.s1.powi(3) * param.s2
                                                + 36. * param.s1.powi(2) * param.s2.powi(2)
                                                + 56. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s2.powi(5)))
                        + 15.
                            * param.m1_2.powi(2)
                            * param.s1
                            * (3.
                                * param.m2_2.powi(4)
                                * (param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + 10. * param.s1.powi(3) * param.s2
                                    + 20. * param.s1.powi(2) * param.s2.powi(2)
                                    + 10. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (param.s1.powi(3)
                                            + 6. * param.s1.powi(2) * param.s2
                                            + 6. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                                + param.s2.powi(3)
                                    * (2. * param.s12.powi(5)
                                        + param.s12.powi(4) * (8. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(4)
                                            * (2. * param.s1 + param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 2. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (4. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                                - param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (5. * param.s1.powi(3)
                                                + -18. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + -19. * param.s1.powi(4) * param.s2
                                        + -40. * param.s1.powi(3) * param.s2.powi(2)
                                        + 20. * param.s1.powi(2) * param.s2.powi(3)
                                        + 35. * param.s1 * param.s2.powi(4)
                                        + 5. * param.s2.powi(5)
                                        + param.s12.powi(4) * (-5. * param.s1 + param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 27. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + -13. * param.s2.powi(3))
                                        + param.s12
                                            * (5. * param.s1.powi(4)
                                                + 56. * param.s1.powi(3) * param.s2
                                                + 36. * param.s1.powi(2) * param.s2.powi(2)
                                                + -64. * param.s1 * param.s2.powi(3)
                                                + -19. * param.s2.powi(4))
                                        - param.s1.powi(5))
                                + -6.
                                    * param.m2_2
                                    * param.s2.powi(2)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (param.s1 + -3. * param.s2)
                                        + param.s12.powi(3)
                                            * (-7. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -18. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3))
                                        + param.s12
                                            * (2. * param.s1.powi(4)
                                                + -16. * param.s1.powi(3) * param.s2
                                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                + 8. * param.s1 * param.s2.powi(3)
                                                + -3. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + 6. * param.m2_2.powi(2)
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + -2. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + 6. * param.s1.powi(2) * param.s2
                                                + 6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + -9. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        - param.s12
                                            * (7. * param.s1.powi(4)
                                                + 4. * param.s1.powi(3) * param.s2
                                                + -36. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                + 7. * param.s2.powi(4))))
                        + param.s1.powi(3)
                            * (14.
                                * param.m2_2.powi(6)
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + 2. * param.s2.powi(2)
                                    + -4. * param.s12 * (param.s1 + param.s2))
                                + param.s12
                                    * param.s2.powi(3)
                                    * (10. * param.s12.powi(4)
                                        + 3. * (param.s1 - param.s2).powi(4)
                                        + -5. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 9. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-17. * param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + -17. * param.s2.powi(2)))
                                + -21.
                                    * param.m2_2.powi(5)
                                    * (-3. * param.s1.powi(3)
                                        + 3. * param.s12.powi(3)
                                        + -7. * param.s1.powi(2) * param.s2
                                        + 5. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)
                                        + param.s12
                                            * (9. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        - param.s12.powi(2) * (9. * param.s1 + param.s2))
                                + -3.
                                    * param.m2_2
                                    * param.s2.powi(2)
                                    * (10. * param.s12.powi(5)
                                        + 10. * param.s12.powi(4) * (-2. * param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (17. * param.s1.powi(3)
                                                + -63. * param.s1.powi(2) * param.s2
                                                + 45. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (7. * param.s1 + 13. * param.s2)
                                        - (param.s1 - param.s2).powi(5))
                                + 15.
                                    * param.m2_2.powi(4)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-12. * param.s1 + 9. * param.s2)
                                        + param.s12.powi(2)
                                            * (18. * param.s1.powi(2)
                                                + -9. * param.s1 * param.s2
                                                + -17. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(2)
                                                + 15. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        - param.s12
                                            * (12. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -40. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3)))
                                + 15.
                                    * param.m2_2.powi(2)
                                    * param.s2
                                    * (2. * param.s12.powi(5)
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 2. * param.s2)
                                        + param.s12.powi(4) * (-7. * param.s1 + 8. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (4. * param.s1.powi(2)
                                                + -2. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -18. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3)))
                                + -10.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (-5. * param.s1 + 13. * param.s2)
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-10. * param.s1.powi(3)
                                                + 18. * param.s1.powi(2) * param.s2
                                                + 45. * param.s1 * param.s2.powi(2)
                                                + -35. * param.s2.powi(3))
                                        + param.s12
                                            * (5. * param.s1.powi(4)
                                                + 8. * param.s1.powi(3) * param.s2
                                                + -63. * param.s1.powi(2) * param.s2.powi(2)
                                                + 40. * param.s1 * param.s2.powi(3)
                                                + 10. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))))
                        + -3.
                            * param.m1_2
                            * param.s1.powi(2)
                            * (-21.
                                * param.m2_2.powi(5)
                                * (param.s12.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    - param.s2.powi(3)
                                    - param.s1.powi(3))
                                + 15.
                                    * param.m2_2
                                    * param.s2.powi(2)
                                    * (-2. * param.s12.powi(5)
                                        + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (5. * param.s1.powi(2)
                                                + -16. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-7. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3)))
                                + 15.
                                    * param.m2_2.powi(4)
                                    * (2. * param.s1.powi(4)
                                        + 2. * param.s12.powi(4)
                                        + 13. * param.s1.powi(3) * param.s2
                                        + 5. * param.s1.powi(2) * param.s2.powi(2)
                                        + -15. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4)
                                        + 3. * param.s12.powi(2)
                                            * (4. * param.s1.powi(2)
                                                + 5. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + param.s12
                                            * (-8. * param.s1.powi(3)
                                                + -27. * param.s1.powi(2) * param.s2
                                                + 8. * param.s1 * param.s2.powi(2)
                                                + 13. * param.s2.powi(3))
                                        - param.s12.powi(3) * (8. * param.s1 + param.s2))
                                + param.s2.powi(3)
                                    * (10. * param.s12.powi(5)
                                        + 10. * param.s12.powi(4) * (param.s1 + -2. * param.s2)
                                        + (param.s1 - param.s2).powi(5)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (13. * param.s1 + 7. * param.s2)
                                        + param.s12.powi(3)
                                            * (-35. * param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 45. * param.s1.powi(2) * param.s2
                                                + -63. * param.s1 * param.s2.powi(2)
                                                + 17. * param.s2.powi(3)))
                                + 30.
                                    * param.m2_2.powi(2)
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (2. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + -7. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (2. * param.s1.powi(3)
                                                + -18. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + param.s12
                                            * (-3. * param.s1.powi(4)
                                                + 8. * param.s1.powi(3) * param.s2
                                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                + -16. * param.s1 * param.s2.powi(3)
                                                + 2. * param.s2.powi(4)))
                                + -10.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (-5. * param.s1 + 7. * param.s2)
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + -17. * param.s2.powi(2))
                                        + param.s12
                                            * (5. * param.s1.powi(4)
                                                + 32. * param.s1.powi(3) * param.s2
                                                + -45. * param.s1.powi(2) * param.s2.powi(2)
                                                + -40. * param.s1 * param.s2.powi(3)
                                                + 20. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + 15. * param.s1.powi(2) * param.s2
                                                + 30. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (10. * param.s1.powi(3)
                                                + 18. * param.s1.powi(2) * param.s2
                                                + -63. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))))
                        + 3. * param.m0_2.powi(4)
                            * (5.
                                * param.m2_2.powi(2)
                                * (2. * param.s12.powi(5)
                                    + param.s12.powi(4) * (8. * param.s1 + -7. * param.s2)
                                    + (param.s1 - param.s2).powi(4) * (2. * param.s1 + param.s2)
                                    + -2.
                                        * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + 2. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (4. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                            - param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + -18. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                                + 5. * param.m1_2.powi(2)
                                    * (2. * param.s12.powi(5)
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 2. * param.s2)
                                        + param.s12.powi(4) * (-7. * param.s1 + 8. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (4. * param.s1.powi(2)
                                                + -2. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -18. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3)))
                                + 3. * param.s12
                                    * (param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (-5. * param.s1.powi(2)
                                                + 18. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(3)
                                                + -17. * param.s1.powi(2) * param.s2
                                                + -17. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (5. * param.s1.powi(4)
                                                + 17. * param.s1.powi(3) * param.s2
                                                + -54. * param.s1.powi(2) * param.s2.powi(2)
                                                + 17. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + -16. * param.s1.powi(2) * param.s2
                                                + -16. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - param.s12.powi(5) * (param.s1 + param.s2))
                                + param.m1_2
                                    * (-12. * param.s12.powi(6)
                                        + 2. * param.s12.powi(5)
                                            * (16. * param.s1 + -9. * param.s2)
                                        + -2.
                                            * param.s12.powi(4)
                                            * (5. * param.s1.powi(2)
                                                + 53. * param.s1 * param.s2
                                                + -45. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (40. * param.s1.powi(4)
                                                + -81. * param.s1.powi(3) * param.s2
                                                + -153. * param.s1.powi(2) * param.s2.powi(2)
                                                + 239. * param.s1 * param.s2.powi(3)
                                                + -45. * param.s2.powi(4))
                                        + 15.
                                            * param.m2_2
                                            * (2. * param.s12.powi(5)
                                                + -2.
                                                    * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + param.s12.powi(3)
                                                    * (-5. * param.s1.powi(2)
                                                        + 16. * param.s1 * param.s2
                                                        + -5. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -9. * param.s1.powi(2) * param.s2
                                                        + -9. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 10. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2))
                                        - param.s12.powi(3)
                                            * (40. * param.s1.powi(3)
                                                + -239. * param.s1.powi(2) * param.s2
                                                + 136. * param.s1 * param.s2.powi(2)
                                                + 45. * param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (8. * param.s1.powi(2)
                                                + 65. * param.s1 * param.s2
                                                + 27. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(5)
                                            * (2. * param.s1 + 3. * param.s2))
                                - param.m2_2
                                    * (12. * param.s12.powi(6)
                                        + 2. * param.s12.powi(5)
                                            * (9. * param.s1 + -16. * param.s2)
                                        + param.s12.powi(4)
                                            * (-90. * param.s1.powi(2)
                                                + 106. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (45. * param.s1.powi(3)
                                                + 136. * param.s1.powi(2) * param.s2
                                                + -239. * param.s1 * param.s2.powi(2)
                                                + 40. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (45. * param.s1.powi(4)
                                                + -239. * param.s1.powi(3) * param.s2
                                                + 153. * param.s1.powi(2) * param.s2.powi(2)
                                                + 81. * param.s1 * param.s2.powi(3)
                                                + -40. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (27. * param.s1.powi(2)
                                                + 65. * param.s1 * param.s2
                                                + 8. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(5)
                                            * (3. * param.s1 + 2. * param.s2)))
                        + 3. * param.m0_2.powi(2)
                            * (5.
                                * param.m1_2.powi(4)
                                * param.s2
                                * (3. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-12. * param.s1 + 9. * param.s2)
                                    + param.s12.powi(2)
                                        * (18. * param.s1.powi(2)
                                            + -9. * param.s1 * param.s2
                                            + -17. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(2)
                                            + 15. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    - param.s12
                                        * (12. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -40. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3)))
                                + 10.
                                    * param.m1_2.powi(3)
                                    * (-3.
                                        * param.s2
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4) * (-3. * param.s1 + param.s2)
                                            + param.s12.powi(3)
                                                * (2. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + -7. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (param.s1.powi(2)
                                                    + 4. * param.s1 * param.s2
                                                    + 2. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (2. * param.s1.powi(3)
                                                    + -18. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + param.s12
                                                * (-3. * param.s1.powi(4)
                                                    + 8. * param.s1.powi(3) * param.s2
                                                    + 9. * param.s1.powi(2) * param.s2.powi(2)
                                                    + -16. * param.s1 * param.s2.powi(3)
                                                    + 2. * param.s2.powi(4)))
                                        + param.m2_2
                                            * (param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (-5. * param.s1 + 7. * param.s2)
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(2)
                                                        + -8. * param.s1 * param.s2
                                                        + -17. * param.s2.powi(2))
                                                + param.s12
                                                    * (5. * param.s1.powi(4)
                                                        + 32. * param.s1.powi(3) * param.s2
                                                        + -45.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -40. * param.s1 * param.s2.powi(3)
                                                        + 20. * param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 15. * param.s1.powi(2) * param.s2
                                                        + 30. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                - param.s12.powi(2)
                                                    * (10. * param.s1.powi(3)
                                                        + 18. * param.s1.powi(2) * param.s2
                                                        + -63. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))))
                                + param.s1
                                    * (5.
                                        * param.m2_2.powi(4)
                                        * (3. * param.s12.powi(4)
                                            + 3. * param.s12.powi(3)
                                                * (3. * param.s1 + -4. * param.s2)
                                            + (param.s1 - param.s2).powi(2)
                                                * (10. * param.s1.powi(2)
                                                    + 15. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (-17. * param.s1.powi(2)
                                                    + -9. * param.s1 * param.s2
                                                    + 18. * param.s2.powi(2))
                                            - param.s12
                                                * (5. * param.s1.powi(3)
                                                    + -40. * param.s1.powi(2) * param.s2
                                                    + 9. * param.s1 * param.s2.powi(2)
                                                    + 12. * param.s2.powi(3)))
                                        + -30.
                                            * param.m2_2.powi(3)
                                            * (param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (param.s1 + -3. * param.s2)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + 2. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (5. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3))
                                                + param.s12
                                                    * (2. * param.s1.powi(4)
                                                        + -16. * param.s1.powi(3) * param.s2
                                                        + 9. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 8. * param.s1 * param.s2.powi(3)
                                                        + -3. * param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + 18.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(6)
                                                + param.s12.powi(4)
                                                    * (-5. * param.s1.powi(2)
                                                        + 18. * param.s1 * param.s2
                                                        + -5. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + -17. * param.s1.powi(2) * param.s2
                                                        + -17. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                - param.s12.powi(2)
                                                    * (5. * param.s1.powi(4)
                                                        + 17. * param.s1.powi(3) * param.s2
                                                        + -54.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 17. * param.s1 * param.s2.powi(3)
                                                        + 5. * param.s2.powi(4))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + -16. * param.s1.powi(2) * param.s2
                                                        + -16. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12.powi(5) * (param.s1 + param.s2))
                                        + 3. * param.s12
                                            * param.s2
                                            * (param.s12.powi(6)
                                                + param.s12.powi(4)
                                                    * (-5. * param.s1.powi(2)
                                                        + 18. * param.s1 * param.s2
                                                        + -5. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + -17. * param.s1.powi(2) * param.s2
                                                        + -17. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                - param.s12.powi(2)
                                                    * (5. * param.s1.powi(4)
                                                        + 17. * param.s1.powi(3) * param.s2
                                                        + -54.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 17. * param.s1 * param.s2.powi(3)
                                                        + 5. * param.s2.powi(4))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + -16. * param.s1.powi(2) * param.s2
                                                        + -16. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12.powi(5) * (param.s1 + param.s2))
                                        - param.m2_2
                                            * (3. * param.s12.powi(7)
                                                + param.s12.powi(6)
                                                    * (-9. * param.s1 + 15. * param.s2)
                                                + param.s12.powi(5)
                                                    * (param.s1.powi(2)
                                                        + 72. * param.s1 * param.s2
                                                        + -63. * param.s2.powi(2))
                                                + param.s12.powi(4)
                                                    * (25. * param.s1.powi(3)
                                                        + -239. * param.s1.powi(2) * param.s2
                                                        + 153. * param.s1 * param.s2.powi(2)
                                                        + 45. * param.s2.powi(3))
                                                + param.s12.powi(3)
                                                    * (-35. * param.s1.powi(4)
                                                        + 136. * param.s1.powi(3) * param.s2
                                                        + 298.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -432. * param.s1 * param.s2.powi(3)
                                                        + 45. * param.s2.powi(4))
                                                + param.s12.powi(2)
                                                    * (17. * param.s1.powi(5)
                                                        + 81. * param.s1.powi(4) * param.s2
                                                        + -486.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + 298.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + 153. * param.s1 * param.s2.powi(4)
                                                        + -63. * param.s2.powi(5))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(3)
                                                        + 67. * param.s1.powi(2) * param.s2
                                                        + 117. * param.s1 * param.s2.powi(2)
                                                        + 15. * param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(5)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))))
                                + 6. * param.m1_2.powi(2)
                                    * (3.
                                        * param.s2
                                        * (param.s12.powi(6)
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + 18. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12.powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + -17. * param.s1.powi(2) * param.s2
                                                    + -17. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - param.s12.powi(2)
                                                * (5. * param.s1.powi(4)
                                                    + 17. * param.s1.powi(3) * param.s2
                                                    + -54.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 17. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(3)
                                                    + -16. * param.s1.powi(2) * param.s2
                                                    + -16. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - param.s12.powi(5) * (param.s1 + param.s2))
                                        + 5. * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -2.
                                                    * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + -2.
                                                    * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -8. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 2. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 6. * param.s1.powi(2) * param.s2
                                                        + 6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 2. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + -9. * param.s1.powi(2) * param.s2
                                                        + -9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                - param.s12
                                                    * (7. * param.s1.powi(4)
                                                        + 4. * param.s1.powi(3) * param.s2
                                                        + -36.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 4. * param.s1 * param.s2.powi(3)
                                                        + 7. * param.s2.powi(4)))
                                        - param.m2_2
                                            * (2. * param.s12.powi(6)
                                                + param.s12.powi(5)
                                                    * (-7. * param.s1 + 8. * param.s2)
                                                + param.s12.powi(4)
                                                    * (5. * param.s1.powi(2)
                                                        + 41. * param.s1 * param.s2
                                                        + -40. * param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(3)
                                                        + 12. * param.s1.powi(2) * param.s2
                                                        + 18. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + -144. * param.s1.powi(2) * param.s2
                                                        + 81. * param.s1 * param.s2.powi(2)
                                                        + 40. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (-20. * param.s1.powi(4)
                                                        + 106. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -239. * param.s1 * param.s2.powi(3)
                                                        + 10. * param.s2.powi(4))
                                                + param.s12
                                                    * (13. * param.s1.powi(5)
                                                        + 16. * param.s1.powi(4) * param.s2
                                                        + -239.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + 136.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + 106. * param.s1 * param.s2.powi(4)
                                                        + -32. * param.s2.powi(5))))
                                + param.m1_2
                                    * (10.
                                        * param.m2_2.powi(3)
                                        * (param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (7. * param.s1 + -5. * param.s2)
                                            + param.s12.powi(3)
                                                * (-17. * param.s1.powi(2)
                                                    + -8. * param.s1 * param.s2
                                                    + 10. * param.s2.powi(2))
                                            + param.s12
                                                * (20. * param.s1.powi(4)
                                                    + -40. * param.s1.powi(3) * param.s2
                                                    + -45.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 32. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4))
                                            - param.s12.powi(2)
                                                * (param.s1.powi(3)
                                                    + -63. * param.s1.powi(2) * param.s2
                                                    + 18. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(2)
                                                * (10. * param.s1.powi(3)
                                                    + 30. * param.s1.powi(2) * param.s2
                                                    + 15. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3)))
                                        + param.s2
                                            * (-3. * param.s12.powi(7)
                                                + param.s12.powi(6)
                                                    * (-15. * param.s1 + 9. * param.s2)
                                                + param.s12.powi(5)
                                                    * (63. * param.s1.powi(2)
                                                        + -72. * param.s1 * param.s2
                                                        - param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (-45. * param.s1.powi(4)
                                                        + 432. * param.s1.powi(3) * param.s2
                                                        + -298.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -136. * param.s1 * param.s2.powi(3)
                                                        + 35. * param.s2.powi(4))
                                                + param.s12.powi(2)
                                                    * (63. * param.s1.powi(5)
                                                        + -153. * param.s1.powi(4) * param.s2
                                                        + -298.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + 486.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + -81. * param.s1 * param.s2.powi(4)
                                                        + -17. * param.s2.powi(5))
                                                - param.s12.powi(4)
                                                    * (45. * param.s1.powi(3)
                                                        + 153. * param.s1.powi(2) * param.s2
                                                        + -239. * param.s1 * param.s2.powi(2)
                                                        + 25. * param.s2.powi(3))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(3)
                                                    * (15. * param.s1.powi(3)
                                                        + 117. * param.s1.powi(2) * param.s2
                                                        + 67. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(5)
                                                    * (3. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + 3. * param.m2_2
                                            * (param.s12.powi(7)
                                                + param.s12.powi(6) * (param.s1 + param.s2)
                                                + param.s12.powi(5)
                                                    * (-17. * param.s1.powi(2)
                                                        + 64. * param.s1 * param.s2
                                                        + -17. * param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(4)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + param.s12.powi(4)
                                                    * (35. * param.s1.powi(3)
                                                        + -81. * param.s1.powi(2) * param.s2
                                                        + -81. * param.s1 * param.s2.powi(2)
                                                        + 35. * param.s2.powi(3))
                                                + 9. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(4)
                                                        + -6. * param.s1.powi(3) * param.s2
                                                        + -30.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -6. * param.s1 * param.s2.powi(3)
                                                        + param.s2.powi(4))
                                                - param.s12.powi(2)
                                                    * (param.s1.powi(5)
                                                        + -239. * param.s1.powi(4) * param.s2
                                                        + 298.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + 298.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + -239. * param.s1 * param.s2.powi(4)
                                                        + param.s2.powi(5))
                                                - param.s12.powi(3)
                                                    * (25. * param.s1.powi(4)
                                                        + 136. * param.s1.powi(3) * param.s2
                                                        + -486.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 136. * param.s1 * param.s2.powi(3)
                                                        + 25. * param.s2.powi(4)))
                                        + -6.
                                            * param.m2_2.powi(2)
                                            * (2. * param.s12.powi(6)
                                                + param.s12.powi(5)
                                                    * (8. * param.s1 + -7. * param.s2)
                                                + param.s12.powi(4)
                                                    * (-40. * param.s1.powi(2)
                                                        + 41. * param.s1 * param.s2
                                                        + 5. * param.s2.powi(2))
                                                + 3. * (param.s1 - param.s2).powi(3)
                                                    * (4. * param.s1.powi(3)
                                                        + 18. * param.s1.powi(2) * param.s2
                                                        + 12. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + param.s12.powi(3)
                                                    * (40. * param.s1.powi(3)
                                                        + 81. * param.s1.powi(2) * param.s2
                                                        + -144. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (10. * param.s1.powi(4)
                                                        + -239. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 106. * param.s1 * param.s2.powi(3)
                                                        + -20. * param.s2.powi(4))
                                                + param.s12
                                                    * (-32. * param.s1.powi(5)
                                                        + 106. * param.s1.powi(4) * param.s2
                                                        + 136.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + -239.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + 16. * param.s1 * param.s2.powi(4)
                                                        + 13. * param.s2.powi(5)))))
                        + -3.
                            * param.m0_2
                            * (7.
                                * param.m1_2.powi(5)
                                * param.s2.powi(2)
                                * (-3. * param.s1.powi(3)
                                    + 3. * param.s12.powi(3)
                                    + -7. * param.s1.powi(2) * param.s2
                                    + 5. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3)
                                    + param.s12
                                        * (9. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + -7. * param.s2.powi(2))
                                    - param.s12.powi(2) * (9. * param.s1 + param.s2))
                                + 5. * param.m1_2.powi(4)
                                    * param.s2
                                    * (param.s2
                                        * (-9. * param.s12.powi(4)
                                            + 15. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + 4. * (param.s1 - param.s2).powi(2)
                                                * (3. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (9. * param.s1.powi(2)
                                                    + -64. * param.s1 * param.s2
                                                    + 9. * param.s2.powi(2))
                                            + param.s12
                                                * (-27. * param.s1.powi(3)
                                                    + 41. * param.s1.powi(2) * param.s2
                                                    + 41. * param.s1 * param.s2.powi(2)
                                                    + -27. * param.s2.powi(3)))
                                        + 3. * param.m2_2
                                            * (2. * param.s1.powi(4)
                                                + 2. * param.s12.powi(4)
                                                + 13. * param.s1.powi(3) * param.s2
                                                + 5. * param.s1.powi(2) * param.s2.powi(2)
                                                + -15. * param.s1 * param.s2.powi(3)
                                                + -5. * param.s2.powi(4)
                                                + 3. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(2)
                                                        + 5. * param.s1 * param.s2
                                                        + -3. * param.s2.powi(2))
                                                + param.s12
                                                    * (-8. * param.s1.powi(3)
                                                        + -27. * param.s1.powi(2) * param.s2
                                                        + 8. * param.s1 * param.s2.powi(2)
                                                        + 13. * param.s2.powi(3))
                                                - param.s12.powi(3)
                                                    * (8. * param.s1 + param.s2)))
                                + 10.
                                    * param.m1_2.powi(3)
                                    * (param.m2_2.powi(2)
                                        * (param.s12.powi(5)
                                            + -19. * param.s1.powi(4) * param.s2
                                            + -40. * param.s1.powi(3) * param.s2.powi(2)
                                            + 20. * param.s1.powi(2) * param.s2.powi(3)
                                            + 35. * param.s1 * param.s2.powi(4)
                                            + 5. * param.s2.powi(5)
                                            + param.s12.powi(4) * (-5. * param.s1 + param.s2)
                                            + 2. * param.s12.powi(3)
                                                * (5. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + -7. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + 27. * param.s1.powi(2) * param.s2
                                                    + -9. * param.s1 * param.s2.powi(2)
                                                    + -13. * param.s2.powi(3))
                                            + param.s12
                                                * (5. * param.s1.powi(4)
                                                    + 56. * param.s1.powi(3) * param.s2
                                                    + 36.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -64. * param.s1 * param.s2.powi(3)
                                                    + -19. * param.s2.powi(4))
                                            - param.s1.powi(5))
                                        + 3. * param.s2.powi(2)
                                            * (param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (param.s1 + -3. * param.s2)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + 2. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (5. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3))
                                                + param.s12
                                                    * (2. * param.s1.powi(4)
                                                        + -16. * param.s1.powi(3) * param.s2
                                                        + 9. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 8. * param.s1 * param.s2.powi(3)
                                                        + -3. * param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + -4.
                                            * param.m2_2
                                            * param.s2
                                            * (param.s12.powi(5)
                                                + -2.
                                                    * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + -2.
                                                    * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -8. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 2. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 6. * param.s1.powi(2) * param.s2
                                                        + 6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 2. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + -9. * param.s1.powi(2) * param.s2
                                                        + -9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                - param.s12
                                                    * (7. * param.s1.powi(4)
                                                        + 4. * param.s1.powi(3) * param.s2
                                                        + -36.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 4. * param.s1 * param.s2.powi(3)
                                                        + 7. * param.s2.powi(4))))
                                + param.s1.powi(2)
                                    * (7.
                                        * param.m2_2.powi(5)
                                        * (5. * param.s1.powi(3)
                                            + 3. * param.s12.powi(3)
                                            + 5. * param.s1.powi(2) * param.s2
                                            + -7. * param.s1 * param.s2.powi(2)
                                            + -3. * param.s2.powi(3)
                                            + param.s12
                                                * (-7. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 9. * param.s2.powi(2))
                                            - param.s12.powi(2) * (param.s1 + 9. * param.s2))
                                        + 3. * param.s12
                                            * param.s2.powi(2)
                                            * (-2. * param.s12.powi(5)
                                                + 2. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 10. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (5. * param.s1.powi(2)
                                                        + -16. * param.s1 * param.s2
                                                        + 5. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (-7. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + -7. * param.s2.powi(3)))
                                        + -5.
                                            * param.m2_2.powi(4)
                                            * (9. * param.s12.powi(4)
                                                + -15.
                                                    * param.s12.powi(3)
                                                    * (param.s1 + param.s2)
                                                + param.s12.powi(2)
                                                    * (-9. * param.s1.powi(2)
                                                        + 64. * param.s1 * param.s2
                                                        + -9. * param.s2.powi(2))
                                                + -4.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (3. * param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + param.s12
                                                    * (27. * param.s1.powi(3)
                                                        + -41. * param.s1.powi(2) * param.s2
                                                        + -41. * param.s1 * param.s2.powi(2)
                                                        + 27. * param.s2.powi(3)))
                                        + 2. * param.m2_2.powi(2)
                                            * (-3. * param.s12.powi(6)
                                                + param.s12.powi(5)
                                                    * (13. * param.s1 + -27. * param.s2)
                                                + 2. * (param.s1 - param.s2).powi(4)
                                                    * (param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + 6. * param.s2.powi(2))
                                                + param.s12.powi(4)
                                                    * (-20. * param.s1.powi(2)
                                                        + 16. * param.s1 * param.s2
                                                        + 45. * param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + 106. * param.s1.powi(2) * param.s2
                                                        + -239. * param.s1 * param.s2.powi(2)
                                                        + 45. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (5. * param.s1.powi(4)
                                                        + -144. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 136. * param.s1 * param.s2.powi(3)
                                                        + -90. * param.s2.powi(4))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -27. * param.s1.powi(2) * param.s2
                                                        + -142. * param.s1 * param.s2.powi(2)
                                                        + -18. * param.s2.powi(3)))
                                        + 30.
                                            * param.m2_2.powi(3)
                                            * (param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (-3. * param.s1 + param.s2)
                                                + param.s12.powi(3)
                                                    * (2. * param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + -7. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + 2. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 5. * param.s2.powi(3))
                                                + param.s12
                                                    * (-3. * param.s1.powi(4)
                                                        + 8. * param.s1.powi(3) * param.s2
                                                        + 9. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -16. * param.s1 * param.s2.powi(3)
                                                        + 2. * param.s2.powi(4)))
                                        + param.m2_2
                                            * param.s2
                                            * (12. * param.s12.powi(6)
                                                + (param.s1 - param.s2).powi(5)
                                                    * (2. * param.s1 + 3. * param.s2)
                                                + param.s12.powi(5)
                                                    * (-32. * param.s1 + 18. * param.s2)
                                                + 2. * param.s12.powi(4)
                                                    * (5. * param.s1.powi(2)
                                                        + 53. * param.s1 * param.s2
                                                        + -45. * param.s2.powi(2))
                                                + param.s12
                                                    * (param.s1 - param.s2).powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + 65. * param.s1 * param.s2
                                                        + 27. * param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (40. * param.s1.powi(3)
                                                        + -239. * param.s1.powi(2) * param.s2
                                                        + 136. * param.s1 * param.s2.powi(2)
                                                        + 45. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (-40. * param.s1.powi(4)
                                                        + 81. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -239. * param.s1 * param.s2.powi(3)
                                                        + 45. * param.s2.powi(4))))
                                + param.m1_2
                                    * param.s1
                                    * (15.
                                        * param.m2_2.powi(4)
                                        * (-5. * param.s1.powi(4)
                                            + 2. * param.s12.powi(4)
                                            + -15. * param.s1.powi(3) * param.s2
                                            + 5. * param.s1.powi(2) * param.s2.powi(2)
                                            + 13. * param.s1 * param.s2.powi(3)
                                            + 2. * param.s2.powi(4)
                                            + param.s12.powi(2)
                                                * (-9. * param.s1.powi(2)
                                                    + 15. * param.s1 * param.s2
                                                    + 12. * param.s2.powi(2))
                                            + param.s12
                                                * (13. * param.s1.powi(3)
                                                    + 8. * param.s1.powi(2) * param.s2
                                                    + -27. * param.s1 * param.s2.powi(2)
                                                    + -8. * param.s2.powi(3))
                                            - param.s12.powi(3) * (param.s1 + 8. * param.s2))
                                        + param.s2.powi(2)
                                            * (12. * param.s12.powi(6)
                                                + 2. * param.s12.powi(5)
                                                    * (9. * param.s1 + -16. * param.s2)
                                                + param.s12.powi(4)
                                                    * (-90. * param.s1.powi(2)
                                                        + 106. * param.s1 * param.s2
                                                        + 10. * param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (45. * param.s1.powi(3)
                                                        + 136. * param.s1.powi(2) * param.s2
                                                        + -239. * param.s1 * param.s2.powi(2)
                                                        + 40. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (45. * param.s1.powi(4)
                                                        + -239. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 81. * param.s1 * param.s2.powi(3)
                                                        + -40. * param.s2.powi(4))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(3)
                                                    * (27. * param.s1.powi(2)
                                                        + 65. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(5)
                                                    * (3. * param.s1 + 2. * param.s2))
                                        + -24.
                                            * param.m2_2
                                            * param.s2
                                            * (param.s12.powi(6)
                                                + param.s12.powi(4)
                                                    * (-5. * param.s1.powi(2)
                                                        + 18. * param.s1 * param.s2
                                                        + -5. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + -17. * param.s1.powi(2) * param.s2
                                                        + -17. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                - param.s12.powi(2)
                                                    * (5. * param.s1.powi(4)
                                                        + 17. * param.s1.powi(3) * param.s2
                                                        + -54.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 17. * param.s1 * param.s2.powi(3)
                                                        + 5. * param.s2.powi(4))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + -16. * param.s1.powi(2) * param.s2
                                                        + -16. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12.powi(5) * (param.s1 + param.s2))
                                        + -40.
                                            * param.m2_2.powi(3)
                                            * (param.s12.powi(5)
                                                + -2.
                                                    * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + -2.
                                                    * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -8. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 2. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 6. * param.s1.powi(2) * param.s2
                                                        + 6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 2. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + -9. * param.s1.powi(2) * param.s2
                                                        + -9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                - param.s12
                                                    * (7. * param.s1.powi(4)
                                                        + 4. * param.s1.powi(3) * param.s2
                                                        + -36.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 4. * param.s1 * param.s2.powi(3)
                                                        + 7. * param.s2.powi(4)))
                                        + 6. * param.m2_2.powi(2)
                                            * (2. * param.s12.powi(6)
                                                + param.s12.powi(5)
                                                    * (-7. * param.s1 + 8. * param.s2)
                                                + param.s12.powi(4)
                                                    * (5. * param.s1.powi(2)
                                                        + 41. * param.s1 * param.s2
                                                        + -40. * param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(3)
                                                        + 12. * param.s1.powi(2) * param.s2
                                                        + 18. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + -144. * param.s1.powi(2) * param.s2
                                                        + 81. * param.s1 * param.s2.powi(2)
                                                        + 40. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (-20. * param.s1.powi(4)
                                                        + 106. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -239. * param.s1 * param.s2.powi(3)
                                                        + 10. * param.s2.powi(4))
                                                + param.s12
                                                    * (13. * param.s1.powi(5)
                                                        + 16. * param.s1.powi(4) * param.s2
                                                        + -239.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + 136.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + 106. * param.s1 * param.s2.powi(4)
                                                        + -32. * param.s2.powi(5))))
                                + 2. * param.m1_2.powi(2)
                                    * (param.s2.powi(2)
                                        * (-3. * param.s12.powi(6)
                                            + param.s12.powi(5)
                                                * (-27. * param.s1 + 13. * param.s2)
                                            + param.s12.powi(4)
                                                * (45. * param.s1.powi(2)
                                                    + 16. * param.s1 * param.s2
                                                    + -20. * param.s2.powi(2))
                                            + 2. * (param.s1 - param.s2).powi(4)
                                                * (6. * param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (18. * param.s1.powi(3)
                                                    + 142. * param.s1.powi(2) * param.s2
                                                    + 27. * param.s1 * param.s2.powi(2)
                                                    + -7. * param.s2.powi(3))
                                            + param.s12.powi(3)
                                                * (45. * param.s1.powi(3)
                                                    + -239. * param.s1.powi(2) * param.s2
                                                    + 106. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3))
                                            + param.s12.powi(2)
                                                * (-90. * param.s1.powi(4)
                                                    + 136. * param.s1.powi(3) * param.s2
                                                    + 153.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -144. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4)))
                                        + 5. * param.m2_2.powi(3)
                                            * (5. * param.s1.powi(5)
                                                + param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (param.s1 + -5. * param.s2)
                                                + 35. * param.s1.powi(4) * param.s2
                                                + 20. * param.s1.powi(3) * param.s2.powi(2)
                                                + -40. * param.s1.powi(2) * param.s2.powi(3)
                                                + -19. * param.s1 * param.s2.powi(4)
                                                + -2.
                                                    * param.s12.powi(3)
                                                    * (7. * param.s1.powi(2)
                                                        + -8. * param.s1 * param.s2
                                                        + -5. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (13. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -27. * param.s1 * param.s2.powi(2)
                                                        + -5. * param.s2.powi(3))
                                                + param.s12
                                                    * (-19. * param.s1.powi(4)
                                                        + -64. * param.s1.powi(3) * param.s2
                                                        + 36.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 56. * param.s1 * param.s2.powi(3)
                                                        + 5. * param.s2.powi(4))
                                                - param.s2.powi(5))
                                        + 3. * param.m2_2
                                            * param.s2
                                            * (2. * param.s12.powi(6)
                                                + param.s12.powi(5)
                                                    * (8. * param.s1 + -7. * param.s2)
                                                + param.s12.powi(4)
                                                    * (-40. * param.s1.powi(2)
                                                        + 41. * param.s1 * param.s2
                                                        + 5. * param.s2.powi(2))
                                                + 3. * (param.s1 - param.s2).powi(3)
                                                    * (4. * param.s1.powi(3)
                                                        + 18. * param.s1.powi(2) * param.s2
                                                        + 12. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + param.s12.powi(3)
                                                    * (40. * param.s1.powi(3)
                                                        + 81. * param.s1.powi(2) * param.s2
                                                        + -144. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                + param.s12.powi(2)
                                                    * (10. * param.s1.powi(4)
                                                        + -239. * param.s1.powi(3) * param.s2
                                                        + 153.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 106. * param.s1 * param.s2.powi(3)
                                                        + -20. * param.s2.powi(4))
                                                + param.s12
                                                    * (-32. * param.s1.powi(5)
                                                        + 106. * param.s1.powi(4) * param.s2
                                                        + 136.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + -239.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + 16. * param.s1 * param.s2.powi(4)
                                                        + 13. * param.s2.powi(5)))
                                        + -3.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(6)
                                                + -2.
                                                    * param.s12.powi(4)
                                                    * (5. * param.s1.powi(2)
                                                        + -24. * param.s1 * param.s2
                                                        + 5. * param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (30. * param.s1.powi(3)
                                                        + -82. * param.s1.powi(2) * param.s2
                                                        + -82. * param.s1 * param.s2.powi(2)
                                                        + 30. * param.s2.powi(3))
                                                + -4.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(4)
                                                        + 16. * param.s1.powi(3) * param.s2
                                                        + 36.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 16. * param.s1 * param.s2.powi(3)
                                                        + param.s2.powi(4))
                                                + param.s12
                                                    * (19. * param.s1.powi(5)
                                                        + 123. * param.s1.powi(4) * param.s2
                                                        + -212.
                                                            * param.s1.powi(3)
                                                            * param.s2.powi(2)
                                                        + -212.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(3)
                                                        + 123. * param.s1 * param.s2.powi(4)
                                                        + 19. * param.s2.powi(5))
                                                - param.s12.powi(2)
                                                    * (35. * param.s1.powi(4)
                                                        + 32. * param.s1.powi(3) * param.s2
                                                        + -324.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 32. * param.s1 * param.s2.powi(3)
                                                        + 35. * param.s2.powi(4))
                                                - param.s12.powi(5) * (param.s1 + param.s2))))
                        - param.m0_2.powi(3)
                            * (-6.
                                * param.m2_2.powi(2)
                                * (3. * param.s12.powi(6)
                                    + param.s12.powi(5) * (27. * param.s1 + -13. * param.s2)
                                    + -2.
                                        * (param.s1 - param.s2).powi(4)
                                        * (6. * param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-45. * param.s1.powi(2)
                                            + -16. * param.s1 * param.s2
                                            + 20. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (90. * param.s1.powi(4)
                                            + -136. * param.s1.powi(3) * param.s2
                                            + -153. * param.s1.powi(2) * param.s2.powi(2)
                                            + 144. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4))
                                    - param.s12.powi(3)
                                        * (45. * param.s1.powi(3)
                                            + -239. * param.s1.powi(2) * param.s2
                                            + 106. * param.s1 * param.s2.powi(2)
                                            + 10. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (18. * param.s1.powi(3)
                                            + 142. * param.s1.powi(2) * param.s2
                                            + 27. * param.s1 * param.s2.powi(2)
                                            + -7. * param.s2.powi(3)))
                                + 10.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (13. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (10. * param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-35. * param.s1.powi(3)
                                                + 45. * param.s1.powi(2) * param.s2
                                                + 18. * param.s1 * param.s2.powi(2)
                                                + -10. * param.s2.powi(3))
                                        + param.s12
                                            * (10. * param.s1.powi(4)
                                                + 40. * param.s1.powi(3) * param.s2
                                                + -63. * param.s1.powi(2) * param.s2.powi(2)
                                                + 8. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4)))
                                + 10.
                                    * param.m1_2.powi(3)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (-5. * param.s1 + 13. * param.s2)
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-10. * param.s1.powi(3)
                                                + 18. * param.s1.powi(2) * param.s2
                                                + 45. * param.s1 * param.s2.powi(2)
                                                + -35. * param.s2.powi(3))
                                        + param.s12
                                            * (5. * param.s1.powi(4)
                                                + 8. * param.s1.powi(3) * param.s2
                                                + -63. * param.s1.powi(2) * param.s2.powi(2)
                                                + 40. * param.s1 * param.s2.powi(3)
                                                + 10. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2)))
                                + param.s12
                                    * (param.s12.powi(5)
                                        * (17. * param.s1.powi(2)
                                            + -64. * param.s1 * param.s2
                                            + 17. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (-35. * param.s1.powi(3)
                                                + 81. * param.s1.powi(2) * param.s2
                                                + 81. * param.s1 * param.s2.powi(2)
                                                + -35. * param.s2.powi(3))
                                        + 3. * (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + -9.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(4)
                                                + -6. * param.s1.powi(3) * param.s2
                                                + -30. * param.s1.powi(2) * param.s2.powi(2)
                                                + -6. * param.s1 * param.s2.powi(3)
                                                + param.s2.powi(4))
                                        + param.s12.powi(3)
                                            * (25. * param.s1.powi(4)
                                                + 136. * param.s1.powi(3) * param.s2
                                                + -486. * param.s1.powi(2) * param.s2.powi(2)
                                                + 136. * param.s1 * param.s2.powi(3)
                                                + 25. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (param.s1.powi(5)
                                                + -239. * param.s1.powi(4) * param.s2
                                                + 298. * param.s1.powi(3) * param.s2.powi(2)
                                                + 298. * param.s1.powi(2) * param.s2.powi(3)
                                                + -239. * param.s1 * param.s2.powi(4)
                                                + param.s2.powi(5))
                                        - param.s12.powi(6) * (param.s1 + param.s2)
                                        - param.s12.powi(7))
                                + 3. * param.m2_2
                                    * (3. * param.s12.powi(7)
                                        + 3. * param.s12.powi(6)
                                            * (5. * param.s1 + -3. * param.s2)
                                        + (param.s1 - param.s2).powi(5)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(5)
                                            * (-63. * param.s1.powi(2)
                                                + 72. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (15. * param.s1.powi(3)
                                                + 117. * param.s1.powi(2) * param.s2
                                                + 67. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (45. * param.s1.powi(3)
                                                + 153. * param.s1.powi(2) * param.s2
                                                + -239. * param.s1 * param.s2.powi(2)
                                                + 25. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (45. * param.s1.powi(4)
                                                + -432. * param.s1.powi(3) * param.s2
                                                + 298. * param.s1.powi(2) * param.s2.powi(2)
                                                + 136. * param.s1 * param.s2.powi(3)
                                                + -35. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (-63. * param.s1.powi(5)
                                                + 153. * param.s1.powi(4) * param.s2
                                                + 298. * param.s1.powi(3) * param.s2.powi(2)
                                                + -486. * param.s1.powi(2) * param.s2.powi(3)
                                                + 81. * param.s1 * param.s2.powi(4)
                                                + 17. * param.s2.powi(5)))
                                + 6. * param.m1_2.powi(2)
                                    * (-3. * param.s12.powi(6)
                                        + param.s12.powi(5) * (13. * param.s1 + -27. * param.s2)
                                        + 2. * (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (-20. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 45. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (10. * param.s1.powi(3)
                                                + 106. * param.s1.powi(2) * param.s2
                                                + -239. * param.s1 * param.s2.powi(2)
                                                + 45. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (5. * param.s1.powi(4)
                                                + -144. * param.s1.powi(3) * param.s2
                                                + 153. * param.s1.powi(2) * param.s2.powi(2)
                                                + 136. * param.s1 * param.s2.powi(3)
                                                + -90. * param.s2.powi(4))
                                        + 15.
                                            * param.m2_2
                                            * (param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (-3. * param.s1 + param.s2)
                                                + param.s12.powi(3)
                                                    * (2. * param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + -7. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + 2. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 5. * param.s2.powi(3))
                                                + param.s12
                                                    * (-3. * param.s1.powi(4)
                                                        + 8. * param.s1.powi(3) * param.s2
                                                        + 9. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + -16. * param.s1 * param.s2.powi(3)
                                                        + 2. * param.s2.powi(4)))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (7. * param.s1.powi(3)
                                                + -27. * param.s1.powi(2) * param.s2
                                                + -142. * param.s1 * param.s2.powi(2)
                                                + -18. * param.s2.powi(3)))
                                + 3. * param.m1_2
                                    * (3. * param.s12.powi(7)
                                        + param.s12.powi(6) * (-9. * param.s1 + 15. * param.s2)
                                        + param.s12.powi(5)
                                            * (param.s1.powi(2)
                                                + 72. * param.s1 * param.s2
                                                + -63. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (25. * param.s1.powi(3)
                                                + -239. * param.s1.powi(2) * param.s2
                                                + 153. * param.s1 * param.s2.powi(2)
                                                + 45. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-35. * param.s1.powi(4)
                                                + 136. * param.s1.powi(3) * param.s2
                                                + 298. * param.s1.powi(2) * param.s2.powi(2)
                                                + -432. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (17. * param.s1.powi(5)
                                                + 81. * param.s1.powi(4) * param.s2
                                                + -486. * param.s1.powi(3) * param.s2.powi(2)
                                                + 298. * param.s1.powi(2) * param.s2.powi(3)
                                                + 153. * param.s1 * param.s2.powi(4)
                                                + -63. * param.s2.powi(5))
                                        + 30.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (param.s1 + -3. * param.s2)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1.powi(2)
                                                        + 8. * param.s1 * param.s2
                                                        + 2. * param.s2.powi(2))
                                                + param.s12.powi(2)
                                                    * (5. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3))
                                                + param.s12
                                                    * (2. * param.s1.powi(4)
                                                        + -16. * param.s1.powi(3) * param.s2
                                                        + 9. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 8. * param.s1 * param.s2.powi(3)
                                                        + -3. * param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + -24.
                                            * param.m2_2
                                            * (param.s12.powi(6)
                                                + param.s12.powi(4)
                                                    * (-5. * param.s1.powi(2)
                                                        + 18. * param.s1 * param.s2
                                                        + -5. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (10. * param.s1.powi(3)
                                                        + -17. * param.s1.powi(2) * param.s2
                                                        + -17. * param.s1 * param.s2.powi(2)
                                                        + 10. * param.s2.powi(3))
                                                - param.s12.powi(2)
                                                    * (5. * param.s1.powi(4)
                                                        + 17. * param.s1.powi(3) * param.s2
                                                        + -54.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 17. * param.s1 * param.s2.powi(3)
                                                        + 5. * param.s2.powi(4))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + -16. * param.s1.powi(2) * param.s2
                                                        + -16. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12.powi(5) * (param.s1 + param.s2))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(3)
                                                + 67. * param.s1.powi(2) * param.s2
                                                + 117. * param.s1 * param.s2.powi(2)
                                                + 15. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2)))))
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
