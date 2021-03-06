use super::{log_diff, Parameters};

pub(crate) fn c210(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.002083333333333333
            * std::f64::consts::PI
            * param.s1.powi(-3)
            * param.lambda_s12_sqrt.powi(-7)
            * ((param.m0_2.powi(4)
                * (-2. * param.s12.powi(5)
                    + 5. * param.s12.powi(4) * (3. * param.s1 + 2. * param.s2)
                    + 2. * param.s12
                        * (param.s1 - param.s2).powi(2)
                        * (15. * param.s1.powi(2) + -5. * param.s2.powi(2) - param.s1 * param.s2)
                    + -2.
                        * param.s12.powi(3)
                        * (30. * param.s1.powi(2)
                            + 17. * param.s1 * param.s2
                            + 10. * param.s2.powi(2))
                    + 4. * param.s12.powi(2)
                        * (5. * param.s1.powi(3)
                            + 3. * param.s1.powi(2) * param.s2
                            + 3. * param.s1 * param.s2.powi(2)
                            + 5. * param.s2.powi(3))
                    - (3. * param.s1 + -2. * param.s2) * (param.s1 - param.s2).powi(4))
                + -2.
                    * param.m1_2.powi(4)
                    * (param.s12.powi(5)
                        + 8. * param.s1.powi(4) * param.s2
                        + -37. * param.s1.powi(3) * param.s2.powi(2)
                        + -37. * param.s1.powi(2) * param.s2.powi(3)
                        + 8. * param.s1 * param.s2.powi(4)
                        + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                        + param.s12.powi(3)
                            * (10. * param.s1.powi(2)
                                + 7. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        + param.s12.powi(2)
                            * (-10. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + 9. * param.s1 * param.s2.powi(2)
                                + -10. * param.s2.powi(3))
                        + param.s12
                            * (5. * param.s1.powi(4)
                                + -19. * param.s1.powi(3) * param.s2
                                + 18. * param.s1.powi(2) * param.s2.powi(2)
                                + -19. * param.s1 * param.s2.powi(3)
                                + 5. * param.s2.powi(4))
                        - param.s2.powi(5)
                        - param.s1.powi(5))
                + param.m1_2
                    * param.s1.powi(3)
                    * (8. * param.s12.powi(5)
                        + -5. * param.s12.powi(4) * (8. * param.s1 + 11. * param.s2)
                        + 4. * param.s12.powi(3)
                            * (20. * param.s1.powi(2)
                                + 29. * param.s1 * param.s2
                                + 45. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (40. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + -21. * param.s1 * param.s2.powi(2)
                                + 100. * param.s2.powi(3))
                        + 4. * param.s12
                            * (10. * param.s1.powi(4)
                                + -23. * param.s1.powi(3) * param.s2
                                + -24. * param.s1.powi(2) * param.s2.powi(2)
                                + 27. * param.s1 * param.s2.powi(3)
                                + 10. * param.s2.powi(4))
                        + -30.
                            * param.m2_2.powi(3)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 10. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + 10.
                            * param.m2_2.powi(2)
                            * (-4. * param.s1.powi(3)
                                + 4. * param.s12.powi(3)
                                + -3. * param.s12.powi(2) * (4. * param.s1 + -5. * param.s2)
                                + -41. * param.s1.powi(2) * param.s2
                                + 22. * param.s1 * param.s2.powi(2)
                                + 23. * param.s2.powi(3)
                                + 2. * param.s12
                                    * (6. * param.s1.powi(2)
                                        + 13. * param.s1 * param.s2
                                        + -21. * param.s2.powi(2)))
                        + 5. * param.m2_2
                            * (3. * param.s12.powi(4)
                                + -4. * param.s12.powi(3) * (3. * param.s1 + 7. * param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + -20. * param.s1 * param.s2
                                        + -35. * param.s2.powi(2))
                                + 6. * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 5. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (3. * param.s1.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + 35. * param.s1 * param.s2.powi(2)
                                        + -12. * param.s2.powi(3)))
                        - (param.s1 - param.s2).powi(3)
                            * (8. * param.s1.powi(2)
                                + -25. * param.s1 * param.s2
                                + 27. * param.s2.powi(2)))
                + param.s1.powi(4)
                    * (-2. * param.s12.powi(5)
                        + (2. * param.s1 + -3. * param.s2) * (param.s1 - param.s2).powi(4)
                        + -60. * param.m2_2.powi(4) * (param.s12 - param.s2 - param.s1)
                        + 5. * param.s12.powi(4) * (2. * param.s1 + 3. * param.s2)
                        + -2.
                            * param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (5. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + -15. * param.s2.powi(2))
                        + 30.
                            * param.m2_2.powi(3)
                            * (3. * param.s1.powi(2)
                                + -6. * param.s1 * param.s12
                                + 3. * param.s12.powi(2)
                                + 2. * param.s1 * param.s2
                                + 2. * param.s12 * param.s2
                                + -5. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(3)
                            * (10. * param.s1.powi(2)
                                + 17. * param.s1 * param.s2
                                + 30. * param.s2.powi(2))
                        + 4. * param.s12.powi(2)
                            * (5. * param.s1.powi(3)
                                + 3. * param.s1.powi(2) * param.s2
                                + 3. * param.s1 * param.s2.powi(2)
                                + 5. * param.s2.powi(3))
                        + -10.
                            * param.m2_2.powi(2)
                            * (2. * param.s12.powi(3)
                                + param.s12.powi(2) * (-6. * param.s1 + 21. * param.s2)
                                + 2. * param.s12
                                    * (3. * param.s1.powi(2)
                                        + -7. * param.s1 * param.s2
                                        + -6. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(2)
                                    * (2. * param.s1 + 11. * param.s2))
                        + -5.
                            * param.m2_2
                            * (param.s12.powi(4)
                                + (param.s1 + -3. * param.s2) * (param.s1 - param.s2).powi(3)
                                + -4. * param.s12.powi(3) * (param.s1 + 3. * param.s2)
                                + 6. * param.s12.powi(2)
                                    * (param.s1.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + -4. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (param.s1.powi(3)
                                        + 7. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))))
                + param.m1_2.powi(3)
                    * param.s1
                    * (-8. * param.s1.powi(5)
                        + 8. * param.s12.powi(5)
                        + 59. * param.s1.powi(4) * param.s2
                        + -226. * param.s1.powi(3) * param.s2.powi(2)
                        + 54. * param.s1.powi(2) * param.s2.powi(3)
                        + 134. * param.s1 * param.s2.powi(4)
                        + -13. * param.s2.powi(5)
                        + -5. * param.s12.powi(4) * (8. * param.s1 + 9. * param.s2)
                        + 4. * param.s12.powi(3)
                            * (20. * param.s1.powi(2)
                                + 19. * param.s1 * param.s2
                                + 25. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (40. * param.s1.powi(3)
                                + -21. * param.s1.powi(2) * param.s2
                                + -51. * param.s1 * param.s2.powi(2)
                                + 55. * param.s2.powi(3))
                        + 4. * param.s12
                            * (10. * param.s1.powi(4)
                                + -33. * param.s1.powi(3) * param.s2
                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                + -68. * param.s1 * param.s2.powi(3)
                                + 15. * param.s2.powi(4))
                        + 5. * param.m2_2
                            * (param.s1.powi(4)
                                + param.s12.powi(4)
                                + -14. * param.s1.powi(3) * param.s2
                                + -70. * param.s1.powi(2) * param.s2.powi(2)
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
                                        + param.s2.powi(3))))
                + param.m1_2.powi(2)
                    * param.s1.powi(2)
                    * (-12. * param.s12.powi(5)
                        + 15. * param.s12.powi(4) * (4. * param.s1 + 5. * param.s2)
                        + -8.
                            * param.s12.powi(3)
                            * (15. * param.s1.powi(2)
                                + 18. * param.s1 * param.s2
                                + 25. * param.s2.powi(2))
                        + 6. * param.s12.powi(2)
                            * (20. * param.s1.powi(3)
                                + -3. * param.s1.powi(2) * param.s2
                                + -23. * param.s1 * param.s2.powi(2)
                                + 45. * param.s2.powi(3))
                        + (param.s1 - param.s2).powi(2)
                            * (12. * param.s1.powi(3)
                                + -57. * param.s1.powi(2) * param.s2
                                + 128. * param.s1 * param.s2.powi(2)
                                + 47. * param.s2.powi(3))
                        + -4.
                            * param.s12
                            * (15. * param.s1.powi(4)
                                + -42. * param.s1.powi(3) * param.s2
                                + -21. * param.s1.powi(2) * param.s2.powi(2)
                                + -47. * param.s1 * param.s2.powi(3)
                                + 45. * param.s2.powi(4))
                        + -20.
                            * param.m2_2.powi(2)
                            * (param.s12.powi(3)
                                + -17. * param.s1.powi(2) * param.s2
                                + -17. * param.s1 * param.s2.powi(2)
                                + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 20. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                - param.s2.powi(3)
                                - param.s1.powi(3))
                        + -5.
                            * param.m2_2
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + -34. * param.s1.powi(3) * param.s2
                                + -74. * param.s1.powi(2) * param.s2.powi(2)
                                + 94. * param.s1 * param.s2.powi(3)
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
                                        + 22. * param.s1 * param.s2.powi(2)
                                        + 9. * param.s2.powi(3))))
                + param.m0_2.powi(3)
                    * (param.m1_2
                        * (8. * param.s12.powi(5)
                            + -5. * param.s12.powi(4) * (11. * param.s1 + 8. * param.s2)
                            + (param.s1 - param.s2).powi(3)
                                * (27. * param.s1.powi(2)
                                    + -25. * param.s1 * param.s2
                                    + 8. * param.s2.powi(2))
                            + 4. * param.s12.powi(3)
                                * (45. * param.s1.powi(2)
                                    + 29. * param.s1 * param.s2
                                    + 20. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (100. * param.s1.powi(3)
                                    + -21. * param.s1.powi(2) * param.s2
                                    + 9. * param.s1 * param.s2.powi(2)
                                    + 40. * param.s2.powi(3))
                            + 4. * param.s12
                                * (10. * param.s1.powi(4)
                                    + 27. * param.s1.powi(3) * param.s2
                                    + -24. * param.s1.powi(2) * param.s2.powi(2)
                                    + -23. * param.s1 * param.s2.powi(3)
                                    + 10. * param.s2.powi(4)))
                        + param.s1
                            * (8. * param.s12.powi(5)
                                + (7. * param.s1 + -3. * param.s2)
                                    * (param.s1 - param.s2).powi(4)
                                + -5. * param.s12.powi(4) * (15. * param.s1 + 7. * param.s2)
                                + -4.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (15. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (20. * param.s1.powi(2)
                                        + 29. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (20. * param.s1.powi(3)
                                        + -129. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + -25. * param.s2.powi(3))
                                + -5.
                                    * param.m2_2
                                    * (param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 - param.s2)
                                        + -4. * param.s12.powi(3) * (3. * param.s1 + param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 4. * param.s12
                                            * (8. * param.s1.powi(3)
                                                + -7. * param.s1.powi(2) * param.s2
                                                - param.s2.powi(3)))))
                + param.m0_2.powi(2)
                    * (param.m1_2.powi(2)
                        * (-12. * param.s12.powi(5)
                            + 15. * param.s12.powi(4) * (5. * param.s1 + 4. * param.s2)
                            + -8.
                                * param.s12.powi(3)
                                * (25. * param.s1.powi(2)
                                    + 18. * param.s1 * param.s2
                                    + 15. * param.s2.powi(2))
                            + (param.s1 - param.s2).powi(2)
                                * (47. * param.s1.powi(3)
                                    + 128. * param.s1.powi(2) * param.s2
                                    + -57. * param.s1 * param.s2.powi(2)
                                    + 12. * param.s2.powi(3))
                            + 6. * param.s12.powi(2)
                                * (45. * param.s1.powi(3)
                                    + -23. * param.s1.powi(2) * param.s2
                                    + -3. * param.s1 * param.s2.powi(2)
                                    + 20. * param.s2.powi(3))
                            + -4.
                                * param.s12
                                * (45. * param.s1.powi(4)
                                    + -47. * param.s1.powi(3) * param.s2
                                    + -21. * param.s1.powi(2) * param.s2.powi(2)
                                    + -42. * param.s1 * param.s2.powi(3)
                                    + 15. * param.s2.powi(4)))
                        + param.m1_2
                            * param.s1
                            * (-8. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (12. * param.s1 + 5. * param.s2)
                                + -4.
                                    * param.s12.powi(3)
                                    * (30. * param.s1.powi(2)
                                        + -11. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (40. * param.s1.powi(3)
                                        + 204. * param.s1.powi(2) * param.s2
                                        + -111. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3))
                                + 4. * param.s12
                                    * param.s2
                                    * (-122. * param.s1.powi(3)
                                        + 99. * param.s1.powi(2) * param.s2
                                        + 18. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + 5. * param.m2_2
                                    * (3. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (7. * param.s1 + 3. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (2. * param.s1.powi(2)
                                                + 5. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (12. * param.s1.powi(3)
                                                + -35. * param.s1.powi(2) * param.s2
                                                + 6. * param.s1 * param.s2.powi(2)
                                                + -3. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (35. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(3)
                                    * (12. * param.s1.powi(2)
                                        + 25. * param.s1 * param.s2
                                        + -7. * param.s2.powi(2)))
                        + param.s1.powi(2)
                            * (-12. * param.s12.powi(5)
                                + -284. * param.s1 * param.s12.powi(3) * param.s2
                                + 25. * param.s12.powi(4) * (param.s1 + param.s2)
                                + -3. * (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (5. * param.s1.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + -6.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -37. * param.s1.powi(2) * param.s2
                                        + -37. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + -10.
                                    * param.m2_2.powi(2)
                                    * (2. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (7. * param.s1 + -2. * param.s2)
                                        + -2.
                                            * param.s12
                                            * (6. * param.s1.powi(2)
                                                + 7. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (11. * param.s1 + 2. * param.s2))
                                + 5. * param.m2_2
                                    * (7. * param.s12.powi(4)
                                        + 4. * param.s12.powi(3)
                                            * (7. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (5. * param.s1 + param.s2)
                                        + 18.
                                            * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 4. * param.s12
                                            * (8. * param.s1.powi(3)
                                                + 13. * param.s1.powi(2) * param.s2
                                                + -20. * param.s1 * param.s2.powi(2)
                                                - param.s2.powi(3)))))
                + param.m0_2
                    * (param.m1_2.powi(3)
                        * (-13. * param.s1.powi(5)
                            + 8. * param.s12.powi(5)
                            + 134. * param.s1.powi(4) * param.s2
                            + 54. * param.s1.powi(3) * param.s2.powi(2)
                            + -226. * param.s1.powi(2) * param.s2.powi(3)
                            + 59. * param.s1 * param.s2.powi(4)
                            + -8. * param.s2.powi(5)
                            + -5. * param.s12.powi(4) * (9. * param.s1 + 8. * param.s2)
                            + 4. * param.s12.powi(3)
                                * (25. * param.s1.powi(2)
                                    + 19. * param.s1 * param.s2
                                    + 20. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (55. * param.s1.powi(3)
                                    + -51. * param.s1.powi(2) * param.s2
                                    + -21. * param.s1 * param.s2.powi(2)
                                    + 40. * param.s2.powi(3))
                            + 4. * param.s12
                                * (15. * param.s1.powi(4)
                                    + -68. * param.s1.powi(3) * param.s2
                                    + 6. * param.s1.powi(2) * param.s2.powi(2)
                                    + -33. * param.s1 * param.s2.powi(3)
                                    + 10. * param.s2.powi(4)))
                        + param.s1.powi(3)
                            * (8. * param.s12.powi(5)
                                + -5. * param.s12.powi(4) * (7. * param.s1 + 15. * param.s2)
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (5. * param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + -15. * param.s2.powi(2))
                                + 30.
                                    * param.m2_2.powi(3)
                                    * (-5. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 2. * param.s12 * (param.s1 + -3. * param.s2)
                                        + 2. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (15. * param.s1.powi(2)
                                        + 29. * param.s1 * param.s2
                                        + 20. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-50. * param.s1.powi(3)
                                        + 12. * param.s1.powi(2) * param.s2
                                        + -258. * param.s1 * param.s2.powi(2)
                                        + 40. * param.s2.powi(3))
                                + -10.
                                    * param.m2_2.powi(2)
                                    * (14. * param.s12.powi(3)
                                        + -15. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 13.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -4.
                                            * param.s12
                                            * (3. * param.s1.powi(2)
                                                + -16. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2)))
                                + 5. * param.m2_2
                                    * (7. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (5. * param.s1 + -7. * param.s2)
                                        + 18.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1.powi(3)
                                                + 20. * param.s1.powi(2) * param.s2
                                                + -13. * param.s1 * param.s2.powi(2)
                                                + -8. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1 + 5. * param.s2))
                                - (3. * param.s1 + -7. * param.s2)
                                    * (param.s1 - param.s2).powi(4))
                        + param.m1_2
                            * param.s1.powi(2)
                            * (-8. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (5. * param.s1 + 12. * param.s2)
                                + -4.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + -11. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + 4. * param.s1
                                    * param.s12
                                    * (5. * param.s1.powi(3)
                                        + 18. * param.s1.powi(2) * param.s2
                                        + 99. * param.s1 * param.s2.powi(2)
                                        + -122. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-10. * param.s1.powi(3)
                                        + -222. * param.s1.powi(2) * param.s2
                                        + 408. * param.s1 * param.s2.powi(2)
                                        + 80. * param.s2.powi(3))
                                + 10.
                                    * param.m2_2.powi(2)
                                    * (23. * param.s1.powi(3)
                                        + 4. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (5. * param.s1 + -4. * param.s2)
                                        + 22. * param.s1.powi(2) * param.s2
                                        + -41. * param.s1 * param.s2.powi(2)
                                        + -4. * param.s2.powi(3)
                                        + param.s12
                                            * (-42. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2)))
                                + -20.
                                    * param.m2_2
                                    * (param.s12.powi(4)
                                        + -6.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 4. * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -7. * param.s1.powi(2) * param.s2
                                                + -7. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(3)
                                    * (7. * param.s1.powi(2)
                                        + -25. * param.s1 * param.s2
                                        + -12. * param.s2.powi(2)))
                        + param.m1_2.powi(2)
                            * param.s1
                            * (-8. * param.s12.powi(5)
                                + 55. * param.s12.powi(4) * (param.s1 + param.s2)
                                + -4.
                                    * param.s12.powi(3)
                                    * (35. * param.s1.powi(2)
                                        + 59. * param.s1 * param.s2
                                        + 35. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (23. * param.s1.powi(3)
                                        + -153. * param.s1.powi(2) * param.s2
                                        + -153. * param.s1 * param.s2.powi(2)
                                        + 23. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (85. * param.s1.powi(3)
                                        + 54. * param.s1.powi(2) * param.s2
                                        + 54. * param.s1 * param.s2.powi(2)
                                        + 85. * param.s2.powi(3))
                                + -4.
                                    * param.s12
                                    * (25. * param.s1.powi(4)
                                        + -68. * param.s1.powi(3) * param.s2
                                        + 186. * param.s1.powi(2) * param.s2.powi(2)
                                        + -68. * param.s1 * param.s2.powi(3)
                                        + 25. * param.s2.powi(4))
                                + -5.
                                    * param.m2_2
                                    * (11. * param.s1.powi(4)
                                        + 3. * param.s12.powi(4)
                                        + 94. * param.s1.powi(3) * param.s2
                                        + -74. * param.s1.powi(2) * param.s2.powi(2)
                                        + -34. * param.s1 * param.s2.powi(3)
                                        + 3. * param.s2.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (5. * param.s1 + 3. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (7. * param.s1.powi(2)
                                                + param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (9. * param.s1.powi(3)
                                                + 22. * param.s1.powi(2) * param.s2
                                                + -12. * param.s1 * param.s2.powi(2)
                                                + 3. * param.s2.powi(3))))))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s1.powi(3)
                    * ((-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1))
                    * (param.m0_2.powi(2) * param.s12
                        + param.m1_2.powi(2) * param.s2
                        + param.m1_2
                            * (param.m2_2 * (param.s12 - param.s2 - param.s1)
                                + param.s2 * (param.s2 - param.s12 - param.s1))
                        + param.s1
                            * (param.m2_2.powi(2) + param.s12 * param.s2
                                - param.m2_2 * (param.s12 + param.s2 - param.s1))
                        - param.m0_2
                            * (param.m2_2 * (param.s1 + param.s12 - param.s2)
                                + param.s12 * (param.s1 + param.s2 - param.s12)
                                + param.m1_2 * (param.s12 + param.s2 - param.s1)))
                        .powi(2)
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.002083333333333333
            * std::f64::consts::PI
            * param.s2.powi(-2)
            * param.lambda_s12_sqrt.powi(-7)
            * ((param.m0_2.powi(4)
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
                            + -94. * param.s2.powi(2)))
                + param.m2_2.powi(4)
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
                + param.s2.powi(4)
                    * (-120. * param.m1_2.powi(4)
                        + param.s12.powi(4)
                        + -10.
                            * param.m1_2.powi(2)
                            * (13. * param.s12.powi(2)
                                + param.s12 * (46. * param.s1 + -26. * param.s2)
                                + 13. * (param.s1 - param.s2).powi(2))
                        + (param.s1 - param.s2).powi(4)
                        + 240. * param.m1_2.powi(3) * (param.s1 + param.s12 - param.s2)
                        + -2. * param.s12.powi(3) * (7. * param.s1 + 2. * param.s2)
                        + -2.
                            * param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (7. * param.s1 + 2. * param.s2)
                        + param.s12.powi(2)
                            * (-94. * param.s1.powi(2)
                                + 24. * param.s1 * param.s2
                                + 6. * param.s2.powi(2))
                        + 10.
                            * param.m1_2
                            * (param.s12.powi(3)
                                + param.s12.powi(2) * (23. * param.s1 + -3. * param.s2)
                                + (param.s1 - param.s2).powi(3)
                                + param.s12
                                    * (23. * param.s1.powi(2)
                                        + -26. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))))
                + -2.
                    * param.m2_2.powi(3)
                    * param.s2
                    * (7. * param.s1.powi(4)
                        + 2. * param.s12.powi(4)
                        + 87. * param.s1.powi(3) * param.s2
                        + -73. * param.s1.powi(2) * param.s2.powi(2)
                        + -23. * param.s1 * param.s2.powi(3)
                        + 2. * param.s2.powi(4)
                        + 3. * param.s12.powi(2)
                            * (9. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + 4. * param.s2.powi(2))
                        + 5. * param.m1_2
                            * (param.s12.powi(3)
                                + -23. * param.s1.powi(2) * param.s2
                                + -23. * param.s1 * param.s2.powi(2)
                                + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + 26. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                - param.s2.powi(3)
                                - param.s1.powi(3))
                        - param.s12
                            * (23. * param.s1.powi(3)
                                + 82. * param.s1.powi(2) * param.s2
                                + -33. * param.s1 * param.s2.powi(2)
                                + 8. * param.s2.powi(3))
                        - param.s12.powi(3) * (13. * param.s1 + 8. * param.s2))
                + -2.
                    * param.m2_2
                    * param.s2.powi(3)
                    * (2. * param.s12.powi(4)
                        + (7. * param.s1 + -2. * param.s2) * (param.s1 - param.s2).powi(3)
                        + 120. * param.m1_2.powi(3) * (param.s12 - param.s2 - param.s1)
                        + param.s12.powi(2)
                            * (-73. * param.s1.powi(2)
                                + 33. * param.s1 * param.s2
                                + 12. * param.s2.powi(2))
                        + -10.
                            * param.m1_2.powi(2)
                            * (-23. * param.s1.powi(2)
                                + 10. * param.s1 * param.s12
                                + 13. * param.s12.powi(2)
                                + 10. * param.s1 * param.s2
                                + -26. * param.s12 * param.s2
                                + 13. * param.s2.powi(2))
                        + param.s12
                            * (87. * param.s1.powi(3)
                                + -82. * param.s1.powi(2) * param.s2
                                + 3. * param.s1 * param.s2.powi(2)
                                + -8. * param.s2.powi(3))
                        + 5. * param.m1_2
                            * (3. * param.s12.powi(3)
                                + param.s12.powi(2) * (43. * param.s1 + -9. * param.s2)
                                + param.s12
                                    * (-23. * param.s1.powi(2)
                                        + -26. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(2)
                                    * (23. * param.s1 + 3. * param.s2))
                        - param.s12.powi(3) * (23. * param.s1 + 8. * param.s2))
                + 2. * param.m2_2.powi(2)
                    * param.s2.powi(2)
                    * (3. * param.s12.powi(4)
                        + -3. * param.s12.powi(3) * (9. * param.s1 + 4. * param.s2)
                        + param.s12.powi(2)
                            * (-2. * param.s1.powi(2)
                                + 27. * param.s1 * param.s2
                                + 18. * param.s2.powi(2))
                        + param.s12
                            * (73. * param.s1.powi(3)
                                + -188. * param.s1.powi(2) * param.s2
                                + 27. * param.s1 * param.s2.powi(2)
                                + -12. * param.s2.powi(3))
                        + -5.
                            * param.m1_2.powi(2)
                            * (13. * param.s1.powi(2)
                                + 13. * param.s12.powi(2)
                                + 46. * param.s1 * param.s2
                                + 13. * param.s2.powi(2)
                                + -26. * param.s12 * (param.s1 + param.s2))
                        + 5. * param.m1_2
                            * (23. * param.s1.powi(3)
                                + 3. * param.s12.powi(3)
                                + param.s12.powi(2) * (17. * param.s1 + -9. * param.s2)
                                + 23. * param.s1.powi(2) * param.s2
                                + -43. * param.s1 * param.s2.powi(2)
                                + -3. * param.s2.powi(3)
                                + param.s12
                                    * (-43. * param.s1.powi(2)
                                        + 26. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2)))
                        - (param.s1 - param.s2).powi(2)
                            * (47. * param.s1.powi(2)
                                + 21. * param.s1 * param.s2
                                + -3. * param.s2.powi(2)))
                + 2. * param.m0_2.powi(2)
                    * (param.m2_2.powi(2)
                        * (3. * param.s12.powi(4)
                            + -3. * param.s12.powi(3) * (4. * param.s1 + 9. * param.s2)
                            + (param.s1 - param.s2).powi(2)
                                * (3. * param.s1.powi(2)
                                    + -21. * param.s1 * param.s2
                                    + -47. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (18. * param.s1.powi(2)
                                    + 27. * param.s1 * param.s2
                                    + -2. * param.s2.powi(2))
                            + param.s12
                                * (-12. * param.s1.powi(3)
                                    + 27. * param.s1.powi(2) * param.s2
                                    + -188. * param.s1 * param.s2.powi(2)
                                    + 73. * param.s2.powi(3)))
                        + param.s2.powi(2)
                            * (-47. * param.s12.powi(4)
                                + 3. * (param.s1 - param.s2).powi(4)
                                + 73. * param.s12.powi(3) * (param.s1 + param.s2)
                                + -27.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1 + param.s2)
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1.powi(2)
                                        + 94. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -5.
                                    * param.m1_2.powi(2)
                                    * (13. * param.s12.powi(2)
                                        + 13. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (-26. * param.s1 + 46. * param.s2))
                                + 5. * param.m1_2
                                    * (23. * param.s12.powi(3)
                                        + 3. * (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2)
                                            * (-43. * param.s1 + 23. * param.s2)
                                        + param.s12
                                            * (17. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + -43. * param.s2.powi(2))))
                        + param.m2_2
                            * param.s2
                            * (12. * param.s12.powi(4)
                                + -3.
                                    * (param.s1 - param.s2).powi(3)
                                    * (param.s1 + 4. * param.s2)
                                + param.s12.powi(3) * (-33. * param.s1 + 82. * param.s2)
                                + param.s12.powi(2)
                                    * (27. * param.s1.powi(2)
                                        + 133. * param.s1 * param.s2
                                        + -188. * param.s2.powi(2))
                                + param.s12
                                    * (-3. * param.s1.powi(3)
                                        + -212. * param.s1.powi(2) * param.s2
                                        + 133. * param.s1 * param.s2.powi(2)
                                        + 82. * param.s2.powi(3))
                                + -5.
                                    * param.m1_2
                                    * (3. * param.s12.powi(3)
                                        + param.s12.powi(2) * (-9. * param.s1 + 43. * param.s2)
                                        + param.s12
                                            * (9. * param.s1.powi(2)
                                                + -26. * param.s1 * param.s2
                                                + -23. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1 + 23. * param.s2))))
                + -2.
                    * param.m0_2
                    * (param.m2_2.powi(3)
                        * (2. * param.s1.powi(4)
                            + 2. * param.s12.powi(4)
                            + -23. * param.s1.powi(3) * param.s2
                            + -73. * param.s1.powi(2) * param.s2.powi(2)
                            + 87. * param.s1 * param.s2.powi(3)
                            + 7. * param.s2.powi(4)
                            + 3. * param.s12.powi(2)
                                * (4. * param.s1.powi(2)
                                    + param.s1 * param.s2
                                    + 9. * param.s2.powi(2))
                            - param.s12
                                * (8. * param.s1.powi(3)
                                    + -33. * param.s1.powi(2) * param.s2
                                    + 82. * param.s1 * param.s2.powi(2)
                                    + 23. * param.s2.powi(3))
                            - param.s12.powi(3) * (8. * param.s1 + 13. * param.s2))
                        + param.s2.powi(3)
                            * (7. * param.s12.powi(4)
                                + param.s12.powi(3) * (87. * param.s1 + -23. * param.s2)
                                + 2. * (param.s1 - param.s2).powi(4)
                                + -120. * param.m1_2.powi(3) * (param.s12 + param.s2 - param.s1)
                                + param.s12.powi(2)
                                    * (-73. * param.s1.powi(2)
                                        + -82. * param.s1 * param.s2
                                        + 27. * param.s2.powi(2))
                                + 10.
                                    * param.m1_2.powi(2)
                                    * (23. * param.s12.powi(2)
                                        + -13. * (param.s1 - param.s2).powi(2)
                                        + -10. * param.s12 * (param.s1 + param.s2))
                                + -5.
                                    * param.m1_2
                                    * (23. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (23. * param.s1 + -43. * param.s2)
                                        + -3. * (param.s1 - param.s2).powi(3)
                                        + param.s12
                                            * (-43. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + 17. * param.s2.powi(2)))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (23. * param.s1 + 13. * param.s2))
                        - param.m2_2
                            * param.s2.powi(2)
                            * (12. * param.s12.powi(4)
                                + param.s12.powi(3) * (82. * param.s1 + -33. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(3)
                                    * (4. * param.s1 + param.s2)
                                + 10.
                                    * param.m1_2.powi(2)
                                    * (13. * param.s1.powi(2)
                                        + -26. * param.s1 * param.s12
                                        + 13. * param.s12.powi(2)
                                        + 10. * param.s1 * param.s2
                                        + 10. * param.s12 * param.s2
                                        + -23. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-188. * param.s1.powi(2)
                                        + 133. * param.s1 * param.s2
                                        + 27. * param.s2.powi(2))
                                + param.s12
                                    * (82. * param.s1.powi(3)
                                        + 133. * param.s1.powi(2) * param.s2
                                        + -212. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s2.powi(3))
                                + -10.
                                    * param.m1_2
                                    * (13. * param.s12.powi(3)
                                        + -13. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 13.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + param.s12
                                            * (-13. * param.s1.powi(2)
                                                + 66. * param.s1 * param.s2
                                                + -13. * param.s2.powi(2))))
                        - param.m2_2.powi(2)
                            * param.s2
                            * (-3. * param.s12.powi(4)
                                + -3. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 2. * (param.s1 - param.s2).powi(2)
                                    * (6. * param.s1.powi(2)
                                        + 53. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (27. * param.s1.powi(2)
                                        + -212. * param.s1 * param.s2
                                        + 27. * param.s2.powi(2))
                                + param.s12
                                    * (-33. * param.s1.powi(3)
                                        + 133. * param.s1.powi(2) * param.s2
                                        + 133. * param.s1 * param.s2.powi(2)
                                        + -33. * param.s2.powi(3))
                                + 5. * param.m1_2
                                    * (-3. * param.s1.powi(3)
                                        + 3. * param.s12.powi(3)
                                        + -43. * param.s1.powi(2) * param.s2
                                        + 23. * param.s1 * param.s2.powi(2)
                                        + 23. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-9. * param.s1 + 17. * param.s2)
                                        + param.s12
                                            * (9. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + -43. * param.s2.powi(2)))))
                + -2.
                    * param.m0_2.powi(3)
                    * (param.m2_2
                        * (2. * param.s12.powi(4)
                            + (2. * param.s1 + -7. * param.s2) * (param.s1 - param.s2).powi(3)
                            + param.s12.powi(2)
                                * (12. * param.s1.powi(2)
                                    + 33. * param.s1 * param.s2
                                    + -73. * param.s2.powi(2))
                            + param.s12
                                * (-8. * param.s1.powi(3)
                                    + 3. * param.s1.powi(2) * param.s2
                                    + -82. * param.s1 * param.s2.powi(2)
                                    + 87. * param.s2.powi(3))
                            - param.s12.powi(3) * (8. * param.s1 + 23. * param.s2))
                        + param.s2
                            * (7. * param.s12.powi(4)
                                + 2. * (param.s1 - param.s2).powi(4)
                                + param.s12.powi(3) * (-23. * param.s1 + 87. * param.s2)
                                + param.s12.powi(2)
                                    * (27. * param.s1.powi(2)
                                        + -82. * param.s1 * param.s2
                                        + -73. * param.s2.powi(2))
                                + -5.
                                    * param.m1_2
                                    * (param.s12.powi(3)
                                        + param.s12.powi(2) * (-3. * param.s1 + 23. * param.s2)
                                        + param.s12
                                            * (3. * param.s1.powi(2)
                                                + -26. * param.s1 * param.s2
                                                + 23. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (13. * param.s1 + 23. * param.s2))))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s2.powi(2)
                    * ((-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1))
                    * (param.m0_2.powi(2) * param.s12
                        + param.m1_2.powi(2) * param.s2
                        + param.m1_2
                            * (param.m2_2 * (param.s12 - param.s2 - param.s1)
                                + param.s2 * (param.s2 - param.s12 - param.s1))
                        + param.s1
                            * (param.m2_2.powi(2) + param.s12 * param.s2
                                - param.m2_2 * (param.s12 + param.s2 - param.s1))
                        - param.m0_2
                            * (param.m2_2 * (param.s1 + param.s12 - param.s2)
                                + param.s12 * (param.s1 + param.s2 - param.s12)
                                + param.m1_2 * (param.s12 + param.s2 - param.s1)))
                        .powi(2)
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.002083333333333333
            * std::f64::consts::PI
            * param.s12.powi(-3)
            * param.lambda_s12_sqrt.powi(-7)
            * ((-2. * param.m2_2.powi(4) * param.s1.powi(5)
                + 15. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12
                + 8. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12
                + -60. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(2)
                + -75. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(2)
                + -12. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(2)
                + 20. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(3)
                + 80. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(3)
                + 25. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(3)
                + 8. * param.m2_2 * param.s1.powi(5) * param.s12.powi(3)
                + 30. * param.m2_2.powi(4) * param.s1 * param.s12.powi(4)
                + 40. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(4)
                + -35. * param.m2_2 * param.s1.powi(4) * param.s12.powi(4)
                + -2. * param.s1.powi(5) * param.s12.powi(4)
                + -3. * param.m2_2.powi(4) * param.s12.powi(5)
                + -60. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5)
                + -30. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(5)
                + 60. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5)
                + 10. * param.s1.powi(4) * param.s12.powi(5)
                + 7. * param.m2_2.powi(3) * param.s12.powi(6)
                + 20. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6)
                + -50. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6)
                + -20. * param.s1.powi(3) * param.s12.powi(6)
                + -3. * param.m2_2.powi(2) * param.s12.powi(7)
                + 20. * param.m2_2 * param.s1 * param.s12.powi(7)
                + 20. * param.s1.powi(2) * param.s12.powi(7)
                + -3. * param.m2_2 * param.s12.powi(8)
                + -10. * param.s1 * param.s12.powi(8)
                + 2. * param.s12.powi(9)
                + 10. * param.m2_2.powi(4) * param.s1.powi(4) * param.s2
                + -34. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12 * param.s2
                + -35. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12 * param.s2
                + 12. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(2) * param.s2
                + 116. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(2) * param.s2
                + 25. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + -62. * param.m2_2.powi(4) * param.s1 * param.s12.powi(3) * param.s2
                + -258. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(3) * param.s2
                + -284. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + -75. * param.m2_2 * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + 14. * param.m2_2.powi(4) * param.s12.powi(4) * param.s2
                + 88. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2
                + 222. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + 116. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + 15. * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + -31. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2
                + 28. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2
                + 12. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2
                + -34. * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + 9. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2
                + -72. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2
                + 12. * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + 19. * param.m2_2 * param.s12.powi(7) * param.s2
                + 18. * param.s1 * param.s12.powi(7) * param.s2
                + -11. * param.s12.powi(8) * param.s2
                + -20. * param.m2_2.powi(4) * param.s1.powi(3) * param.s2.powi(2)
                + 12. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                + 60. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                + 24. * param.m2_2.powi(4) * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                + 12.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -26. * param.m2_2.powi(4) * param.s12.powi(3) * param.s2.powi(2)
                + 24. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                + 222.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 80. * param.m2_2 * param.s1.powi(3) * param.s12.powi(3) * param.s2.powi(2)
                + 54. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(2)
                + -96. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + -258. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(2)
                + -60. * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(2)
                + -6. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(2)
                + 24. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(2)
                + 12. * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(2)
                + -46. * param.m2_2 * param.s12.powi(6) * param.s2.powi(2)
                + 24. * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + 24. * param.s12.powi(7) * param.s2.powi(2)
                + 20. * param.m2_2.powi(4) * param.s1.powi(2) * param.s2.powi(3)
                + 18. * param.m2_2.powi(4) * param.s1 * param.s12 * param.s2.powi(3)
                + -50. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                + 24. * param.m2_2.powi(4) * param.s12.powi(2) * param.s2.powi(3)
                + -72. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                + -30.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -46. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(3)
                + 28. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + 40. * param.m2_2 * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(3)
                + -6. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(3)
                + 88. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + 20. * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(3)
                + 54. * param.m2_2 * param.s12.powi(5) * param.s2.powi(3)
                + -62. * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + -26. * param.s12.powi(6) * param.s2.powi(3)
                + -10. * param.m2_2.powi(4) * param.s1 * param.s2.powi(4)
                + -11. * param.m2_2.powi(4) * param.s12 * param.s2.powi(4)
                + 20. * param.m2_2.powi(3) * param.s1 * param.s12 * param.s2.powi(4)
                + 19. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(4)
                + 20. * param.m2_2.powi(2) * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + 9. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(4)
                + -60. * param.m2_2 * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + -31. * param.m2_2 * param.s12.powi(4) * param.s2.powi(4)
                + 30. * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + 14. * param.s12.powi(5) * param.s2.powi(4)
                + 2. * param.m2_2.powi(4) * param.s2.powi(5)
                + -3. * param.m2_2.powi(3) * param.s12 * param.s2.powi(5)
                + -3. * param.m2_2.powi(2) * param.s12.powi(2) * param.s2.powi(5)
                + 7. * param.m2_2 * param.s12.powi(3) * param.s2.powi(5)
                + -3. * param.s12.powi(4) * param.s2.powi(5)
                + 60. * param.m0_2.powi(4) * param.s12.powi(4) * (param.s12 + param.s2 - param.s1)
                + 2. * param.m1_2.powi(4)
                    * (param.s12.powi(5)
                        + param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (5. * param.s1 + 8. * param.s2)
                        + param.s12.powi(3)
                            * (10. * param.s1.powi(2)
                                + 19. * param.s1 * param.s2
                                + 37. * param.s2.powi(2))
                        - param.s12.powi(2)
                            * (10. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + 18. * param.s1 * param.s2.powi(2)
                                + -37. * param.s2.powi(3))
                        - param.s12.powi(4) * (5. * param.s1 + 8. * param.s2)
                        - (param.s1 - param.s2).powi(5))
                + 30.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (param.s12
                        * (3. * param.s1.powi(2)
                            + -6. * param.s1 * param.s12
                            + 3. * param.s12.powi(2)
                            + 2. * param.s1 * param.s2
                            + 2. * param.s12 * param.s2
                            + -5. * param.s2.powi(2))
                        + param.m2_2
                            * (-5. * param.s12.powi(2)
                                + 3. * (param.s1 - param.s2).powi(2)
                                + 2. * param.s12 * (param.s1 + param.s2))
                        - param.m1_2
                            * (3. * param.s12.powi(2)
                                + 3. * (param.s1 - param.s2).powi(2)
                                + param.s12 * (-6. * param.s1 + 10. * param.s2)))
                + param.m1_2.powi(3)
                    * (param.m2_2
                        * (-13. * param.s12.powi(5)
                            + 8. * (param.s1 - param.s2).powi(5)
                            + 2. * param.s12.powi(4) * (30. * param.s1 + 67. * param.s2)
                            + -2.
                                * param.s12.powi(3)
                                * (55. * param.s1.powi(2)
                                    + 136. * param.s1 * param.s2
                                    + -27. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (50. * param.s1.powi(3)
                                    + 51. * param.s1.powi(2) * param.s2
                                    + 12. * param.s1 * param.s2.powi(2)
                                    + -113. * param.s2.powi(3))
                            - param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (45. * param.s1 + 59. * param.s2))
                        + param.s12
                            * (-8. * param.s12.powi(5)
                                + (8. * param.s1 + -13. * param.s2)
                                    * (param.s1 - param.s2).powi(4)
                                + param.s12.powi(4) * (40. * param.s1 + 59. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (20. * param.s1.powi(2)
                                        + 2. * param.s1 * param.s2
                                        + -67. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(3)
                                    * (40. * param.s1.powi(2)
                                        + 66. * param.s1 * param.s2
                                        + 113. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (80. * param.s1.powi(3)
                                        + 42. * param.s1.powi(2) * param.s2
                                        + 24. * param.s1 * param.s2.powi(2)
                                        + 54. * param.s2.powi(3))))
                + param.m1_2.powi(2)
                    * (param.m2_2
                        * param.s12
                        * (23. * param.s12.powi(5)
                            + param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (55. * param.s1.powi(2)
                                    + -126. * param.s1 * param.s2
                                    + -199. * param.s2.powi(2))
                            + 2. * param.s12.powi(3)
                                * (85. * param.s1.powi(2)
                                    + 136. * param.s1 * param.s2
                                    + 88. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(2)
                                * (35. * param.s1.powi(3)
                                    + -27. * param.s1.powi(2) * param.s2
                                    + 186. * param.s1 * param.s2.powi(2)
                                    + -44. * param.s2.powi(3))
                            - param.s12.powi(4) * (100. * param.s1 + 199. * param.s2)
                            - (8. * param.s1 + -23. * param.s2) * (param.s1 - param.s2).powi(4))
                        + param.m2_2.powi(2)
                            * (47. * param.s12.powi(5)
                                + -12. * (param.s1 - param.s2).powi(5)
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (25. * param.s1 + 27. * param.s2)
                                + param.s12.powi(4) * (-180. * param.s1 + 34. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (135. * param.s1.powi(2)
                                        + 94. * param.s1 * param.s2
                                        + -133. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-200. * param.s1.powi(3)
                                        + -138. * param.s1.powi(2) * param.s2
                                        + 84. * param.s1 * param.s2.powi(2)
                                        + 254. * param.s2.powi(3)))
                        + param.s12.powi(2)
                            * (12. * param.s12.powi(5)
                                + -3. * param.s12.powi(4) * (20. * param.s1 + 27. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (60. * param.s1.powi(2)
                                        + 84. * param.s1 * param.s2
                                        + 127. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (60. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + -42. * param.s1 * param.s2.powi(2)
                                        + 133. * param.s2.powi(3))
                                + 2. * param.s12
                                    * (30. * param.s1.powi(4)
                                        + -72. * param.s1.powi(3) * param.s2
                                        + -69. * param.s1.powi(2) * param.s2.powi(2)
                                        + 94. * param.s1 * param.s2.powi(3)
                                        + 17. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(3)
                                    * (12. * param.s1.powi(2)
                                        + -39. * param.s1 * param.s2
                                        + 47. * param.s2.powi(2))))
                + param.m1_2
                    * (param.m2_2.powi(3)
                        * (27. * param.s12.powi(5)
                            + 2. * param.s12.powi(4) * (20. * param.s1 + -53. * param.s2)
                            + 8. * (param.s1 - param.s2).powi(5)
                            + -4.
                                * param.s12.powi(3)
                                * (50. * param.s1.powi(2)
                                    + -27. * param.s1 * param.s2
                                    + -41. * param.s2.powi(2))
                            + 6. * param.s12.powi(2)
                                * (30. * param.s1.powi(3)
                                    + 7. * param.s1.powi(2) * param.s2
                                    + -16. * param.s1 * param.s2.powi(2)
                                    + -21. * param.s2.powi(3))
                            - param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (55. * param.s1 + 49. * param.s2))
                        + param.m2_2.powi(2)
                            * param.s12
                            * (-12. * param.s12.powi(5)
                                + 11. * param.s12.powi(4) * param.s2
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (30. * param.s1.powi(2)
                                        + 82. * param.s1 * param.s2
                                        + 23. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (80. * param.s1.powi(2)
                                        + -488. * param.s1 * param.s2
                                        + 46. * param.s2.powi(2))
                                + -12.
                                    * param.s12.powi(2)
                                    * (10. * param.s1.powi(3)
                                        + -34. * param.s1.powi(2) * param.s2
                                        + -33. * param.s1 * param.s2.powi(2)
                                        + 7. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(4)
                                    * (8. * param.s1 + 7. * param.s2))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (-7. * param.s12.powi(5)
                                + param.s12.powi(4) * (20. * param.s1 + 46. * param.s2)
                                + -4.
                                    * (param.s1 - param.s2).powi(3)
                                    * (2. * param.s1.powi(2)
                                        + -9. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + -36. * param.s1 * param.s2
                                        + 42. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-20. * param.s1.powi(3)
                                        + -222. * param.s1.powi(2) * param.s2
                                        + 396. * param.s1 * param.s2.powi(2)
                                        + 46. * param.s2.powi(3))
                                + param.s12
                                    * (25. * param.s1.powi(4)
                                        + 44. * param.s1.powi(3) * param.s2
                                        + 408. * param.s1.powi(2) * param.s2.powi(2)
                                        + -488. * param.s1 * param.s2.powi(3)
                                        + 11. * param.s2.powi(4)))
                        + param.s12.powi(3)
                            * (-8. * param.s12.powi(5)
                                + param.s12.powi(4) * (40. * param.s1 + 49. * param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (40. * param.s1.powi(2)
                                        + 46. * param.s1 * param.s2
                                        + 63. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (8. * param.s1.powi(3)
                                        + -39. * param.s1.powi(2) * param.s2
                                        + 94. * param.s1 * param.s2.powi(2)
                                        + 27. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (40. * param.s1.powi(3)
                                        + -9. * param.s1.powi(2) * param.s2
                                        + -48. * param.s1 * param.s2.powi(2)
                                        + 82. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (20. * param.s1.powi(4)
                                        + -58. * param.s1.powi(3) * param.s2
                                        + -21. * param.s1.powi(2) * param.s2.powi(2)
                                        + -54. * param.s1 * param.s2.powi(3)
                                        + 53. * param.s2.powi(4))))
                + 10.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.s12.powi(2)
                        * (-2. * param.s1.powi(3)
                            + 2. * param.s12.powi(3)
                            + -21. * param.s1.powi(2) * param.s2
                            + 12. * param.s1 * param.s2.powi(2)
                            + 11. * param.s2.powi(3)
                            + param.s12.powi(2) * (-6. * param.s1 + 7. * param.s2)
                            + 2. * param.s12
                                * (3. * param.s1.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + -10. * param.s2.powi(2)))
                        + param.m2_2.powi(2)
                            * (11. * param.s12.powi(3)
                                + 4. * param.s12.powi(2) * (3. * param.s1 + -5. * param.s2)
                                + -2. * (param.s1 - param.s2).powi(3)
                                + 7. * param.s12
                                    * (-3. * param.s1.powi(2)
                                        + 2. * param.s1 * param.s2
                                        + param.s2.powi(2)))
                        + param.m2_2
                            * param.s12
                            * (-13. * param.s12.powi(3)
                                + param.s12.powi(2) * (12. * param.s1 + 13. * param.s2)
                                + param.s12
                                    * (15. * param.s1.powi(2)
                                        + -64. * param.s1 * param.s2
                                        + 13. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(2)
                                    * (14. * param.s1 + 13. * param.s2))
                        + 2. * param.m1_2.powi(2)
                            * (param.s12.powi(3)
                                + param.s12.powi(2) * (-3. * param.s1 + 17. * param.s2)
                                + param.s12
                                    * (3. * param.s1.powi(2)
                                        + -20. * param.s1 * param.s2
                                        + 17. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(3))
                        + param.m1_2
                            * (param.m2_2
                                * (23. * param.s12.powi(3)
                                    + 4. * (param.s1 - param.s2).powi(3)
                                    + param.s12.powi(2) * (-42. * param.s1 + 22. * param.s2)
                                    + param.s12
                                        * (15. * param.s1.powi(2)
                                            + 26. * param.s1 * param.s2
                                            + -41. * param.s2.powi(2)))
                                + param.s12
                                    * (-4. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (12. * param.s1 + -41. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (4. * param.s1 + 23. * param.s2)
                                        + param.s12
                                            * (-12. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + 22. * param.s2.powi(2)))))
                + 5. * param.m0_2
                    * param.s12
                    * (param.m1_2.powi(3)
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
                                    + -70. * param.s2.powi(2)))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (-2. * param.s12.powi(3) * (2. * param.s1 + param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (7. * param.s1.powi(2)
                                        + 42. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + -40. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))
                                + param.s12
                                    * (-20. * param.s1.powi(3)
                                        + 54. * param.s1.powi(2) * param.s2
                                        + 52. * param.s1 * param.s2.powi(2)
                                        + -14. * param.s2.powi(3))
                                - param.s12.powi(4))
                        + param.m2_2.powi(2)
                            * param.s12
                            * (5. * param.s12.powi(4)
                                + 2. * param.s12.powi(3) * (16. * param.s1 + -7. * param.s2)
                                + (param.s1 - param.s2).powi(3) * (7. * param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (-72. * param.s1.powi(2)
                                        + 52. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2))
                                + param.s12
                                    * (28. * param.s1.powi(3)
                                        + 54. * param.s1.powi(2) * param.s2
                                        + -80. * param.s1 * param.s2.powi(2)
                                        + -2. * param.s2.powi(3)))
                        + param.m1_2
                            * (4.
                                * param.m2_2
                                * param.s12
                                * (3. * param.s12.powi(4)
                                    + param.s12.powi(3) * (-8. * param.s1 + 14. * param.s2)
                                    + param.s12.powi(2)
                                        * (6. * param.s1.powi(2)
                                            + 28. * param.s1 * param.s2
                                            + -34. * param.s2.powi(2))
                                    + 14.
                                        * param.s12
                                        * param.s2
                                        * (-3. * param.s1.powi(2)
                                            + 2. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(3)
                                        * (param.s1 + 3. * param.s2))
                                + param.m2_2.powi(2)
                                    * (-35. * param.s12.powi(4)
                                        + 3. * (param.s1 - param.s2).powi(4)
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (14. * param.s1 + 13. * param.s2)
                                        + param.s12.powi(3) * (48. * param.s1 + 50. * param.s2)
                                        + 4. * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -35. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2)))
                                + param.s12.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + 2. * param.s12.powi(2)
                                            * (3. * param.s1 + 2. * param.s2).powi(2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (6. * param.s1 + 13. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -22. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (6. * param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 70. * param.s1 * param.s2.powi(2)
                                                + -25. * param.s2.powi(3))))
                        - param.m1_2.powi(2)
                            * (param.m2_2
                                * (11. * param.s12.powi(4)
                                    + 3. * (param.s1 - param.s2).powi(4)
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (10. * param.s1 + 17. * param.s2)
                                    + param.s12.powi(3) * (-36. * param.s1 + 94. * param.s2)
                                    + param.s12.powi(2)
                                        * (42. * param.s1.powi(2)
                                            + -88. * param.s1 * param.s2
                                            + -74. * param.s2.powi(2)))
                                + param.s12
                                    * (3. * param.s12.powi(4)
                                        + (3. * param.s1 + -11. * param.s2)
                                            * (param.s1 - param.s2).powi(3)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (6. * param.s1 + 17. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 24. * param.s1 * param.s2
                                                + -37. * param.s2.powi(2))
                                        + param.s12
                                            * (-12. * param.s1.powi(3)
                                                + 6. * param.s1.powi(2) * param.s2
                                                + -88. * param.s1 * param.s2.powi(2)
                                                + 94. * param.s2.powi(3))))
                        - param.s12.powi(3)
                            * (param.s1.powi(4)
                                + param.s12.powi(4)
                                + -12. * param.s1.powi(3) * param.s2
                                + -24. * param.s1.powi(2) * param.s2.powi(2)
                                + 32. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -2. * param.s12.powi(3) * (2. * param.s1 + 3. * param.s2)
                                + 6. * param.s12.powi(2)
                                    * (param.s1.powi(2) + 2. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (2. * param.s1.powi(3)
                                        + -9. * param.s1.powi(2) * param.s2
                                        + 14. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)))
                        - param.m2_2.powi(3)
                            * (3. * param.s12.powi(4)
                                + 2. * param.s12.powi(3) * (16. * param.s1 + -5. * param.s2)
                                + (param.s1 - param.s2).powi(4)
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (2. * param.s1 + param.s2)
                                + -4.
                                    * param.s12.powi(2)
                                    * (6. * param.s1.powi(2)
                                        + 7. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2)))))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s12.powi(3)
                    * ((-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1))
                    * (param.m0_2.powi(2) * param.s12
                        + param.m1_2.powi(2) * param.s2
                        + param.m1_2
                            * (param.m2_2 * (param.s12 - param.s2 - param.s1)
                                + param.s2 * (param.s2 - param.s12 - param.s1))
                        + param.s1
                            * (param.m2_2.powi(2) + param.s12 * param.s2
                                - param.m2_2 * (param.s12 + param.s2 - param.s1))
                        - param.m0_2
                            * (param.m2_2 * (param.s1 + param.s12 - param.s2)
                                + param.s12 * (param.s1 + param.s2 - param.s12)
                                + param.m1_2 * (param.s12 + param.s2 - param.s1)))
                        .powi(2)
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
