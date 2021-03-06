use super::{log_diff, Parameters};

pub(crate) fn c211(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.0006944444444444444
            * std::f64::consts::PI
            * param.s1.powi(-3)
            * param.lambda_s12_sqrt.powi(-9)
            * (60.
                * param.s1.powi(3)
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
                * (7. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1) * param.s2
                    + param.m0_2.powi(2)
                        * (4. * param.s12.powi(2) + -3. * (param.s1 - param.s2).powi(2)
                            - param.s12 * (param.s1 + param.s2))
                    + param.m0_2
                        * (3. * param.s1.powi(3)
                            + -5. * param.s1.powi(2) * param.s12
                            + param.s1 * param.s12.powi(2)
                            + param.s12.powi(3)
                            + -3. * param.s1.powi(2) * param.s2
                            + 14. * param.s1 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s2
                            + -3. * param.s1 * param.s2.powi(2)
                            + -5. * param.s12 * param.s2.powi(2)
                            + 3. * param.s2.powi(3)
                            + -2.
                                * param.m1_2
                                * (2. * param.s1.powi(2)
                                    + -4. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + 3. * param.s12 * param.s2
                                    + -5. * param.s2.powi(2))
                            + -2.
                                * param.m2_2
                                * (-5. * param.s1.powi(2)
                                    + 3. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + -4. * param.s12 * param.s2
                                    + 2. * param.s2.powi(2)))
                    + 2. * param.m1_2
                        * (2.
                            * param.m2_2
                            * (param.s1.powi(2)
                                + param.s12.powi(2)
                                + 5. * param.s1 * param.s2
                                + param.s2.powi(2)
                                + -2. * param.s12 * (param.s1 + param.s2))
                            - param.s2
                                * (-5. * param.s1.powi(2)
                                    + 3. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + -4. * param.s12 * param.s2
                                    + 2. * param.s2.powi(2)))
                    - param.s1
                        * (7. * param.m2_2.powi(2) * (param.s1 + param.s2 - param.s12)
                            + 2. * param.m2_2
                                * (2. * param.s1.powi(2)
                                    + -4. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + 3. * param.s12 * param.s2
                                    + -5. * param.s2.powi(2))
                            + param.s2
                                * (-4. * param.s12.powi(2)
                                    + 3. * (param.s1 - param.s2).powi(2)
                                    + param.s12 * (param.s1 + param.s2))))
                * log_diff(
                    param.m0_2 * (param.s1 + param.s12 - param.s2)
                        + param.m1_2 * (param.s1 + param.s2 - param.s12)
                        + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                    param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m0_2.powi(5)
                    * (param.s12.powi(6)
                        + (param.s1 - param.s2).powi(5) * (2. * param.s1 - param.s2)
                        + -6. * param.s12.powi(5) * (2. * param.s1 + param.s2)
                        + -2.
                            * param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (18. * param.s1.powi(2) + -3. * param.s2.powi(2)
                                - param.s1 * param.s2)
                        + param.s12.powi(4)
                            * (90. * param.s1.powi(2)
                                + 41. * param.s1 * param.s2
                                + 15. * param.s2.powi(2))
                        + 4. * param.s12.powi(3)
                            * (60. * param.s1.powi(3)
                                + -29. * param.s1.powi(2) * param.s2
                                + -11. * param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3))
                        + -3.
                            * param.s12.powi(2)
                            * (95. * param.s1.powi(4)
                                + -94. * param.s1.powi(3) * param.s2
                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                + -2. * param.s1 * param.s2.powi(3)
                                + -5. * param.s2.powi(4)))
                    + param.s1.powi(5)
                        * (param.s12.powi(6)
                            + (param.s1 + -2. * param.s2) * (param.s1 - param.s2).powi(5)
                            + -420. * param.m2_2.powi(5) * (param.s12 - param.s2 - param.s1)
                            + -6. * param.s12.powi(5) * (param.s1 + 2. * param.s2)
                            + 30.
                                * param.m2_2.powi(4)
                                * (29. * param.s1.powi(2)
                                    + -58. * param.s1 * param.s12
                                    + 29. * param.s12.powi(2)
                                    + 12. * param.s1 * param.s2
                                    + 12. * param.s12 * param.s2
                                    + -41. * param.s2.powi(2))
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (3. * param.s1.powi(2)
                                    + param.s1 * param.s2
                                    + -18. * param.s2.powi(2))
                            + param.s12.powi(4)
                                * (15. * param.s1.powi(2)
                                    + 41. * param.s1 * param.s2
                                    + 90. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (5. * param.s1.powi(3)
                                    + 11. * param.s1.powi(2) * param.s2
                                    + 29. * param.s1 * param.s2.powi(2)
                                    + -60. * param.s2.powi(3))
                            + 3. * param.s12.powi(2)
                                * (5. * param.s1.powi(4)
                                    + 2. * param.s1.powi(3) * param.s2
                                    + -6. * param.s1.powi(2) * param.s2.powi(2)
                                    + 94. * param.s1 * param.s2.powi(3)
                                    + -95. * param.s2.powi(4))
                            + -20.
                                * param.m2_2.powi(3)
                                * (25. * param.s12.powi(3)
                                    + param.s12.powi(2) * (-75. * param.s1 + 99. * param.s2)
                                    + param.s12
                                        * (75. * param.s1.powi(2)
                                            + -88. * param.s1 * param.s2
                                            + -63. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2)
                                        * (25. * param.s1 + 61. * param.s2))
                            + 15.
                                * param.m2_2.powi(2)
                                * (3. * param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(3)
                                        * (3. * param.s1 + 29. * param.s2)
                                    + param.s12.powi(3) * (-12. * param.s1 + 88. * param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (3. * param.s1.powi(2)
                                            + -26. * param.s1 * param.s2
                                            + 11. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (3. * param.s1.powi(3)
                                            + -12. * param.s1.powi(2) * param.s2
                                            + -23. * param.s1 * param.s2.powi(2)
                                            + 32. * param.s2.powi(3)))
                            + 6. * param.m2_2
                                * (param.s12.powi(5)
                                    + -5. * param.s12.powi(4) * (param.s1 + 4. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + 26. * param.s1 * param.s2
                                            + -90. * param.s2.powi(2))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (5. * param.s1.powi(2)
                                            + 6. * param.s1 * param.s2
                                            + 125. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-10. * param.s1.powi(3)
                                            + -36. * param.s1.powi(2) * param.s2
                                            + 84. * param.s1 * param.s2.powi(2)
                                            + 70. * param.s2.powi(3))
                                    - (param.s1 + -4. * param.s2)
                                        * (param.s1 - param.s2).powi(4)))
                    + param.m1_2.powi(2)
                        * param.s1.powi(3)
                        * (10. * param.s12.powi(6)
                            + -12. * param.s12.powi(5) * (5. * param.s1 + 8. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (30. * param.s1.powi(2)
                                    + 58. * param.s1 * param.s2
                                    + 93. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (50. * param.s1.powi(3)
                                    + 50. * param.s1.powi(2) * param.s2
                                    + 23. * param.s1 * param.s2.powi(2)
                                    + 150. * param.s2.powi(3))
                            + (param.s1 - param.s2).powi(3)
                                * (10. * param.s1.powi(3)
                                    + -64. * param.s1.powi(2) * param.s2
                                    + 215. * param.s1 * param.s2.powi(2)
                                    + 319. * param.s2.powi(3))
                            + 6. * param.s12.powi(2)
                                * (25. * param.s1.powi(4)
                                    + -30. * param.s1.powi(3) * param.s2
                                    + -129. * param.s1.powi(2) * param.s2.powi(2)
                                    + 416. * param.s1 * param.s2.powi(3)
                                    + -30. * param.s2.powi(4))
                            + -4.
                                * param.s12
                                * (15. * param.s1.powi(5)
                                    + -70. * param.s1.powi(4) * param.s2
                                    + 9. * param.s1.powi(3) * param.s2.powi(2)
                                    + -618. * param.s1.powi(2) * param.s2.powi(3)
                                    + 844. * param.s1 * param.s2.powi(4)
                                    + -180. * param.s2.powi(5))
                            + -100.
                                * param.m2_2.powi(3)
                                * (-5. * param.s1.powi(3)
                                    + 5. * param.s12.powi(3)
                                    + -37. * param.s1.powi(2) * param.s2
                                    + -37. * param.s1 * param.s2.powi(2)
                                    + -5. * param.s2.powi(3)
                                    + -15. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (15. * param.s1.powi(2)
                                            + 52. * param.s1 * param.s2
                                            + 15. * param.s2.powi(2)))
                            + 15.
                                * param.m2_2.powi(2)
                                * (9. * param.s1.powi(4)
                                    + 9. * param.s12.powi(4)
                                    + 260. * param.s1.powi(3) * param.s2
                                    + 202. * param.s1.powi(2) * param.s2.powi(2)
                                    + -380. * param.s1 * param.s2.powi(3)
                                    + -91. * param.s2.powi(4)
                                    + param.s12.powi(3) * (-36. * param.s1 + 64. * param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (9. * param.s1.powi(2)
                                            + 22. * param.s1 * param.s2
                                            + -41. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (9. * param.s1.powi(3)
                                            + 114. * param.s1.powi(2) * param.s2
                                            + -71. * param.s1 * param.s2.powi(2)
                                            + -66. * param.s2.powi(3)))
                            + 6. * param.m2_2
                                * (6. * param.s12.powi(5)
                                    + -15. * param.s12.powi(4) * (2. * param.s1 + 5. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (30. * param.s1.powi(2)
                                            + 66. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + -6.
                                        * param.s12.powi(2)
                                        * (10. * param.s1.powi(3)
                                            + -9. * param.s1.powi(2) * param.s2
                                            + 181. * param.s1 * param.s2.powi(2)
                                            + -70. * param.s2.powi(3))
                                    + param.s12
                                        * (30. * param.s1.powi(4)
                                            + -204. * param.s1.powi(3) * param.s2
                                            + 678. * param.s1.powi(2) * param.s2.powi(2)
                                            + 796. * param.s1 * param.s2.powi(3)
                                            + -540. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(2)
                                        * (6. * param.s1.powi(3)
                                            + -81. * param.s1.powi(2) * param.s2
                                            + -586. * param.s1 * param.s2.powi(2)
                                            + -199. * param.s2.powi(3))))
                    + param.m1_2
                        * param.s1.powi(4)
                        * (-5. * param.s12.powi(6)
                            + 6. * param.s12.powi(5) * (5. * param.s1 + 9. * param.s2)
                            + -5.
                                * param.s12.powi(4)
                                * (15. * param.s1.powi(2)
                                    + 35. * param.s1 * param.s2
                                    + 66. * param.s2.powi(2))
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (15. * param.s1.powi(3)
                                    + -25. * param.s1.powi(2) * param.s2
                                    + -113. * param.s1 * param.s2.powi(2)
                                    + -285. * param.s2.powi(3))
                            + 4. * param.s12.powi(3)
                                * (25. * param.s1.powi(3)
                                    + 40. * param.s1.powi(2) * param.s2
                                    + 67. * param.s1 * param.s2.powi(2)
                                    + -30. * param.s2.powi(3))
                            + param.s12.powi(2)
                                * (-75. * param.s1.powi(4)
                                    + 30. * param.s1.powi(3) * param.s2
                                    + 306. * param.s1.powi(2) * param.s2.powi(2)
                                    + -1914. * param.s1 * param.s2.powi(3)
                                    + 1005. * param.s2.powi(4))
                            + -30.
                                * param.m2_2.powi(4)
                                * (29. * param.s1.powi(2)
                                    + 29. * param.s12.powi(2)
                                    + 82. * param.s1 * param.s2
                                    + 29. * param.s2.powi(2)
                                    + -58. * param.s12 * (param.s1 + param.s2))
                            + 40.
                                * param.m2_2.powi(3)
                                * (-25. * param.s1.powi(3)
                                    + 25. * param.s12.powi(3)
                                    + -98. * param.s1.powi(2) * param.s2
                                    + 61. * param.s1 * param.s2.powi(2)
                                    + 62. * param.s2.powi(3)
                                    + param.s12.powi(2) * (-75. * param.s1 + 12. * param.s2)
                                    + param.s12
                                        * (75. * param.s1.powi(2)
                                            + 86. * param.s1 * param.s2
                                            + -99. * param.s2.powi(2)))
                            + -15.
                                * param.m2_2.powi(2)
                                * (9. * param.s12.powi(4)
                                    + -4.
                                        * param.s12.powi(3)
                                        * (9. * param.s1 + -41. * param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (9. * param.s1.powi(2)
                                            + -28. * param.s1 * param.s2
                                            + -33. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (9. * param.s1.powi(2)
                                            + 178. * param.s1 * param.s2
                                            + 157. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (9. * param.s1.powi(3)
                                            + 39. * param.s1.powi(2) * param.s2
                                            + -157. * param.s1 * param.s2.powi(2)
                                            + 33. * param.s2.powi(3)))
                            + -6.
                                * param.m2_2
                                * (4. * param.s12.powi(5)
                                    + -5. * param.s12.powi(4) * (4. * param.s1 + 13. * param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + 37. * param.s1 * param.s2
                                            + -70. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (20. * param.s1.powi(3)
                                            + 27. * param.s1.powi(2) * param.s2
                                            + 222. * param.s1 * param.s2.powi(2)
                                            + -305. * param.s2.powi(3))
                                    + 4. * param.s12
                                        * (5. * param.s1.powi(4)
                                            + -19. * param.s1.powi(3) * param.s2
                                            + 178. * param.s1.powi(2) * param.s2.powi(2)
                                            + -129. * param.s1 * param.s2.powi(3)
                                            + -35. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (4. * param.s1.powi(2)
                                            + -35. * param.s1 * param.s2
                                            + -129. * param.s2.powi(2)))
                            - (param.s1 - param.s2).powi(4)
                                * (5. * param.s1.powi(2)
                                    + -21. * param.s1 * param.s2
                                    + 34. * param.s2.powi(2)))
                    + param.m1_2.powi(4)
                        * param.s1
                        * (5. * param.s1.powi(6)
                            + 5. * param.s12.powi(6)
                            + -59. * param.s1.powi(5) * param.s2
                            + 427. * param.s1.powi(4) * param.s2.powi(2)
                            + 1222. * param.s1.powi(3) * param.s2.powi(3)
                            + -1403. * param.s1.powi(2) * param.s2.powi(4)
                            + -203. * param.s1 * param.s2.powi(5)
                            + 11. * param.s2.powi(6)
                            + -6. * param.s12.powi(5) * (5. * param.s1 + 6. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (15. * param.s1.powi(2)
                                    + 17. * param.s1 * param.s2
                                    + 21. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (25. * param.s1.powi(3)
                                    + -5. * param.s1.powi(2) * param.s2
                                    + -32. * param.s1 * param.s2.powi(2)
                                    + 40. * param.s2.powi(3))
                            + 3. * param.s12.powi(2)
                                * (25. * param.s1.powi(4)
                                    + -70. * param.s1.powi(3) * param.s2
                                    + -48. * param.s1.powi(2) * param.s2.powi(2)
                                    + -198. * param.s1 * param.s2.powi(3)
                                    + 45. * param.s2.powi(4))
                            + param.s12
                                * (-30. * param.s1.powi(5)
                                    + 200. * param.s1.powi(4) * param.s2
                                    + -516. * param.s1.powi(3) * param.s2.powi(2)
                                    + 1452. * param.s1.powi(2) * param.s2.powi(3)
                                    + 614. * param.s1 * param.s2.powi(4)
                                    + -60. * param.s2.powi(5))
                            + 6. * param.m2_2
                                * (param.s12.powi(5)
                                    + 23. * param.s1.powi(4) * param.s2
                                    + 328. * param.s1.powi(3) * param.s2.powi(2)
                                    + 328. * param.s1.powi(2) * param.s2.powi(3)
                                    + 23. * param.s1 * param.s2.powi(4)
                                    + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + -27. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + -64. * param.s1.powi(3) * param.s2
                                            + -392. * param.s1.powi(2) * param.s2.powi(2)
                                            + -64. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s2.powi(5)
                                    - param.s1.powi(5)))
                    + param.m0_2.powi(4)
                        * (param.s1
                            * (-8. * param.s12.powi(6)
                                + 6. * param.s12.powi(5) * (22. * param.s1 + 7. * param.s2)
                                + param.s12.powi(4)
                                    * (435. * param.s1.powi(2)
                                        + -343. * param.s1 * param.s2
                                        + -90. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (54. * param.s1.powi(2)
                                        + 25. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2))
                                + -20.
                                    * param.s12.powi(3)
                                    * (57. * param.s1.powi(3)
                                        + -32. * param.s1.powi(2) * param.s2
                                        + -11. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3))
                                + 6. * param.s12.powi(2)
                                    * (80. * param.s1.powi(4)
                                        + 103. * param.s1.powi(3) * param.s2
                                        + -186. * param.s1.powi(2) * param.s2.powi(2)
                                        + 13. * param.s1 * param.s2.powi(3)
                                        + -10. * param.s2.powi(4))
                                + 6. * param.m2_2
                                    * (param.s12.powi(5)
                                        + (param.s1 - param.s2).powi(4)
                                            * (4. * param.s1 - param.s2)
                                        + -5. * param.s12.powi(4) * (4. * param.s1 + param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (125. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-180. * param.s1.powi(2)
                                                + 52. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (35. * param.s1.powi(3)
                                                + 42. * param.s1.powi(2) * param.s2
                                                + -18. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3)))
                                - (7. * param.s1 + -2. * param.s2)
                                    * (param.s1 - param.s2).powi(5))
                            - param.m1_2
                                * (5. * param.s12.powi(6)
                                    + -6. * param.s12.powi(5) * (9. * param.s1 + 5. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                        * (34. * param.s1.powi(2)
                                            + -21. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (66. * param.s1.powi(2)
                                            + 35. * param.s1 * param.s2
                                            + 15. * param.s2.powi(2))
                                    + 4. * param.s12.powi(3)
                                        * (30. * param.s1.powi(3)
                                            + -67. * param.s1.powi(2) * param.s2
                                            + -40. * param.s1 * param.s2.powi(2)
                                            + -25. * param.s2.powi(3))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (285. * param.s1.powi(3)
                                            + 113. * param.s1.powi(2) * param.s2
                                            + 25. * param.s1 * param.s2.powi(2)
                                            + -15. * param.s2.powi(3))
                                    + -3.
                                        * param.s12.powi(2)
                                        * (335. * param.s1.powi(4)
                                            + -638. * param.s1.powi(3) * param.s2
                                            + 102. * param.s1.powi(2) * param.s2.powi(2)
                                            + 10. * param.s1 * param.s2.powi(3)
                                            + -25. * param.s2.powi(4))))
                    + param.m0_2.powi(3)
                        * (param.m1_2.powi(2)
                            * (10. * param.s12.powi(6)
                                + -12. * param.s12.powi(5) * (8. * param.s1 + 5. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (93. * param.s1.powi(2)
                                        + 58. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (150. * param.s1.powi(3)
                                        + 23. * param.s1.powi(2) * param.s2
                                        + 50. * param.s1 * param.s2.powi(2)
                                        + 50. * param.s2.powi(3))
                                + -6.
                                    * param.s12.powi(2)
                                    * (30. * param.s1.powi(4)
                                        + -416. * param.s1.powi(3) * param.s2
                                        + 129. * param.s1.powi(2) * param.s2.powi(2)
                                        + 30. * param.s1 * param.s2.powi(3)
                                        + -25. * param.s2.powi(4))
                                + 4. * param.s12
                                    * (180. * param.s1.powi(5)
                                        + -844. * param.s1.powi(4) * param.s2
                                        + 618. * param.s1.powi(3) * param.s2.powi(2)
                                        + -9. * param.s1.powi(2) * param.s2.powi(3)
                                        + 70. * param.s1 * param.s2.powi(4)
                                        + -15. * param.s2.powi(5))
                                - (param.s1 - param.s2).powi(3)
                                    * (319. * param.s1.powi(3)
                                        + 215. * param.s1.powi(2) * param.s2
                                        + -64. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3)))
                            + param.s1.powi(2)
                                * (37. * param.s12.powi(6)
                                    + 6. * param.s12.powi(5)
                                        * (47. * param.s1 + -23. * param.s2)
                                    + 2. * (param.s1 - param.s2).powi(5)
                                        * (4. * param.s1 + param.s2)
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (51. * param.s1.powi(2)
                                            + 80. * param.s1 * param.s2
                                            + 9. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-990. * param.s1.powi(2)
                                            + 662. * param.s1 * param.s2
                                            + 180. * param.s2.powi(2))
                                    + 20.
                                        * param.s12.powi(3)
                                        * (45. * param.s1.powi(3)
                                            + 68. * param.s1.powi(2) * param.s2
                                            + -103. * param.s1 * param.s2.powi(2)
                                            + -4. * param.s2.powi(3))
                                    + -3.
                                        * param.s12.powi(2)
                                        * (45. * param.s1.powi(4)
                                            + 664. * param.s1.powi(3) * param.s2
                                            + -378. * param.s1.powi(2) * param.s2.powi(2)
                                            + -336. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    + 15.
                                        * param.m2_2.powi(2)
                                        * (3. * param.s12.powi(4)
                                            + 4. * param.s12.powi(3)
                                                * (22. * param.s1 + -3. * param.s2)
                                            + 6. * param.s12.powi(2)
                                                * (11. * param.s1.powi(2)
                                                    + -26. * param.s1 * param.s2
                                                    + 3. * param.s2.powi(2))
                                            + -4.
                                                * param.s12
                                                * (32. * param.s1.powi(3)
                                                    + -23. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 3. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(3)
                                                * (29. * param.s1 + 3. * param.s2))
                                    + -6.
                                        * param.m2_2
                                        * (14. * param.s12.powi(5)
                                            + 5. * param.s12.powi(4)
                                                * (51. * param.s1 + -11. * param.s2)
                                            + (param.s1 - param.s2).powi(4)
                                                * (11. * param.s1 + param.s2)
                                            + -8.
                                                * param.s12.powi(3)
                                                * (35. * param.s1.powi(2)
                                                    + 29. * param.s1 * param.s2
                                                    + -10. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (125. * param.s1.powi(2)
                                                    + 142. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (125. * param.s1.powi(3)
                                                    + -513. * param.s1.powi(2) * param.s2
                                                    + 147. * param.s1 * param.s2.powi(2)
                                                    + 25. * param.s2.powi(3))))
                            + param.m1_2
                                * param.s1
                                * (19. * param.s12.powi(6)
                                    + -6.
                                        * param.s12.powi(5)
                                        * (43. * param.s1 + 15. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                        * (53. * param.s1.powi(2)
                                            + 24. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-105. * param.s1.powi(2)
                                            + 404. * param.s1 * param.s2
                                            + 165. * param.s2.powi(2))
                                    + 4. * param.s12.powi(3)
                                        * (405. * param.s1.powi(3)
                                            + -1031. * param.s1.powi(2) * param.s2
                                            + 73. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (303. * param.s1.powi(3)
                                            + 1201. * param.s1.powi(2) * param.s2
                                            + 125. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3))
                                    + -9.
                                        * param.s12.powi(2)
                                        * (215. * param.s1.powi(4)
                                            + -312. * param.s1.powi(3) * param.s2
                                            + -266. * param.s1.powi(2) * param.s2.powi(2)
                                            + 80. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4))
                                    + -6.
                                        * param.m2_2
                                        * (4. * param.s12.powi(5)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (13. * param.s1 + 4. * param.s2)
                                            + param.s12.powi(3)
                                                * (-280. * param.s1.powi(2)
                                                    + 148. * param.s1 * param.s2
                                                    + 40. * param.s2.powi(2))
                                            + param.s12.powi(2)
                                                * (610. * param.s1.powi(3)
                                                    + -444. * param.s1.powi(2) * param.s2
                                                    + -54. * param.s1 * param.s2.powi(2)
                                                    + -40. * param.s2.powi(3))
                                            + -4.
                                                * param.s12
                                                * (35. * param.s1.powi(4)
                                                    + 129. * param.s1.powi(3) * param.s2
                                                    + -178.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 19. * param.s1 * param.s2.powi(3)
                                                    + -5. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (129. * param.s1.powi(2)
                                                    + 35. * param.s1 * param.s2
                                                    + -4. * param.s2.powi(2)))))
                    + param.m0_2.powi(2)
                        * (param.s1.powi(3)
                            * (37. * param.s12.powi(6)
                                + -6. * param.s12.powi(5) * (23. * param.s1 + -47. * param.s2)
                                + -2.
                                    * (param.s1 - param.s2).powi(5)
                                    * (param.s1 + 4. * param.s2)
                                + 2. * param.s12.powi(4)
                                    * (90. * param.s1.powi(2)
                                        + 331. * param.s1 * param.s2
                                        + -495. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (9. * param.s1.powi(2)
                                        + 80. * param.s1 * param.s2
                                        + 51. * param.s2.powi(2))
                                + -20.
                                    * param.s12.powi(3)
                                    * (4. * param.s1.powi(3)
                                        + 103. * param.s1.powi(2) * param.s2
                                        + -68. * param.s1 * param.s2.powi(2)
                                        + -45. * param.s2.powi(3))
                                + -3.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(4)
                                        + -336. * param.s1.powi(3) * param.s2
                                        + -378. * param.s1.powi(2) * param.s2.powi(2)
                                        + 664. * param.s1 * param.s2.powi(3)
                                        + 45. * param.s2.powi(4))
                                + -20.
                                    * param.m2_2.powi(3)
                                    * (25. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (99. * param.s1 + -75. * param.s2)
                                        + param.s12
                                            * (-63. * param.s1.powi(2)
                                                + -88. * param.s1 * param.s2
                                                + 75. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (61. * param.s1 + 25. * param.s2))
                                + 15.
                                    * param.m2_2.powi(2)
                                    * (65. * param.s12.powi(4)
                                        + 4. * param.s12.powi(3)
                                            * (23. * param.s1 + -40. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (61. * param.s1 + 35. * param.s2)
                                        + param.s12.powi(2)
                                            * (-318. * param.s1.powi(2)
                                                + 300. * param.s1 * param.s2
                                                + 90. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (25. * param.s1.powi(3)
                                                + 74. * param.s1.powi(2) * param.s2
                                                + -109. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3)))
                                + -6.
                                    * param.m2_2
                                    * (84. * param.s12.powi(5)
                                        + -95. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + -9.
                                            * (param.s1 - param.s2).powi(4)
                                            * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (65. * param.s1.powi(2)
                                                + 278. * param.s1 * param.s2
                                                + 65. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (105. * param.s1.powi(2)
                                                + -524. * param.s1 * param.s2
                                                + 105. * param.s2.powi(2))
                                        + 36.
                                            * param.s12.powi(2)
                                            * (10. * param.s1.powi(3)
                                                + -19. * param.s1.powi(2) * param.s2
                                                + -19. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))))
                            + -3.
                                * param.m1_2
                                * param.s1.powi(2)
                                * (5.
                                    * param.m2_2.powi(2)
                                    * (9. * param.s12.powi(4)
                                        + 4. * param.s12.powi(3)
                                            * (41. * param.s1 + -9. * param.s2)
                                        + -6.
                                            * param.s12.powi(2)
                                            * (33. * param.s1.powi(2)
                                                + 28. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (157. * param.s1.powi(2)
                                                + 178. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (33. * param.s1.powi(3)
                                                + -157. * param.s1.powi(2) * param.s2
                                                + 39. * param.s1 * param.s2.powi(2)
                                                + 9. * param.s2.powi(3)))
                                    + -2.
                                        * param.m2_2
                                        * (24. * param.s12.powi(5)
                                            + 25.
                                                * param.s12.powi(4)
                                                * (10. * param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-740. * param.s1.powi(2)
                                                    + 628. * param.s1 * param.s2
                                                    + 60. * param.s2.powi(2))
                                            + 6. * param.s12.powi(2)
                                                * (80. * param.s1.powi(3)
                                                    + 221. * param.s1.powi(2) * param.s2
                                                    + -294. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + 4. * param.s12
                                                * (35. * param.s1.powi(4)
                                                    + -509. * param.s1.powi(3) * param.s2
                                                    + 328.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 161. * param.s1 * param.s2.powi(3)
                                                    + -15. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (154. * param.s1.powi(2)
                                                    + 305. * param.s1 * param.s2
                                                    + 21. * param.s2.powi(2)))
                                    + 3. * (2. * param.s12.powi(6)
                                        + 4. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + param.s12.powi(4)
                                            * (-35. * param.s1.powi(2)
                                                + 262. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 8. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + 33. * param.s1.powi(2) * param.s2
                                                + 33. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 12.
                                            * param.s12.powi(3)
                                            * (5. * param.s1.powi(3)
                                                + -23. * param.s1.powi(2) * param.s2
                                                + -23. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(2)
                                            * (10. * param.s1.powi(4)
                                                + 61. * param.s1.powi(3) * param.s2
                                                + -250. * param.s1.powi(2) * param.s2.powi(2)
                                                + 61. * param.s1 * param.s2.powi(3)
                                                + 10. * param.s2.powi(4))))
                            + 3. * param.m1_2.powi(2)
                                * param.s1
                                * (2.
                                    * param.m2_2
                                    * (6. * param.s12.powi(5)
                                        + -15.
                                            * param.s12.powi(4)
                                            * (5. * param.s1 + 2. * param.s2)
                                        + param.s12.powi(3)
                                            * (-10. * param.s1.powi(2)
                                                + 132. * param.s1 * param.s2
                                                + 60. * param.s2.powi(2))
                                        + 6. * param.s12.powi(2)
                                            * (70. * param.s1.powi(3)
                                                + -181. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -10. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(2)
                                            * (199. * param.s1.powi(3)
                                                + 586. * param.s1.powi(2) * param.s2
                                                + 81. * param.s1 * param.s2.powi(2)
                                                + -6. * param.s2.powi(3))
                                        + param.s12
                                            * (-540. * param.s1.powi(4)
                                                + 796. * param.s1.powi(3) * param.s2
                                                + 678. * param.s1.powi(2) * param.s2.powi(2)
                                                + -204. * param.s1 * param.s2.powi(3)
                                                + 30. * param.s2.powi(4)))
                                    + -3.
                                        * (param.s12.powi(6)
                                            + -2.
                                                * param.s12.powi(5)
                                                * (5. * param.s1 + param.s2)
                                            + param.s12.powi(4)
                                                * (20. * param.s1.powi(2)
                                                    + -34. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + 4. * param.s12.powi(3)
                                                * param.s2
                                                * (-83. * param.s1.powi(2)
                                                    + 31. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (17. * param.s1.powi(5)
                                                    + -109. * param.s1.powi(4) * param.s2
                                                    + -224.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 334.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -25. * param.s1 * param.s2.powi(4)
                                                    + 7. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (35. * param.s1.powi(4)
                                                    + -680. * param.s1.powi(3) * param.s2
                                                    + 384.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 68. * param.s1 * param.s2.powi(3)
                                                    + 25. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + 124. * param.s1.powi(2) * param.s2
                                                    + 29. * param.s1 * param.s2.powi(2)
                                                    + -3. * param.s2.powi(3))))
                            - param.m1_2.powi(3)
                                * (10. * param.s12.powi(6)
                                    + -12. * param.s12.powi(5) * (7. * param.s1 + 5. * param.s2)
                                    + 5. * param.s12.powi(4)
                                        * (63. * param.s1.powi(2)
                                            + 46. * param.s1 * param.s2
                                            + 30. * param.s2.powi(2))
                                    + -4.
                                        * param.s12.powi(3)
                                        * (155. * param.s1.powi(3)
                                            + -43. * param.s1.powi(2) * param.s2
                                            + 20. * param.s1 * param.s2.powi(2)
                                            + 50. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(2)
                                        * (79. * param.s1.powi(4)
                                            + 1276. * param.s1.powi(3) * param.s2
                                            + 441. * param.s1.powi(2) * param.s2.powi(2)
                                            + -86. * param.s1 * param.s2.powi(3)
                                            + 10. * param.s2.powi(4))
                                    + 6. * param.s12.powi(2)
                                        * (110. * param.s1.powi(4)
                                            + 54. * param.s1.powi(3) * param.s2
                                            + -111. * param.s1.powi(2) * param.s2.powi(2)
                                            + -50. * param.s1 * param.s2.powi(3)
                                            + 25. * param.s2.powi(4))
                                    + -4.
                                        * param.s12
                                        * (90. * param.s1.powi(5)
                                            + 446. * param.s1.powi(4) * param.s2
                                            + -957. * param.s1.powi(3) * param.s2.powi(2)
                                            + 111. * param.s1.powi(2) * param.s2.powi(3)
                                            + -85. * param.s1 * param.s2.powi(4)
                                            + 15. * param.s2.powi(5))))
                    + param.m0_2
                        * (param.m1_2.powi(4)
                            * (11. * param.s1.powi(6)
                                + 5. * param.s12.powi(6)
                                + -203. * param.s1.powi(5) * param.s2
                                + -1403. * param.s1.powi(4) * param.s2.powi(2)
                                + 1222. * param.s1.powi(3) * param.s2.powi(3)
                                + 427. * param.s1.powi(2) * param.s2.powi(4)
                                + -59. * param.s1 * param.s2.powi(5)
                                + 5. * param.s2.powi(6)
                                + -6. * param.s12.powi(5) * (6. * param.s1 + 5. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (21. * param.s1.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (40. * param.s1.powi(3)
                                        + -32. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + 25. * param.s2.powi(3))
                                + 3. * param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + -198. * param.s1.powi(3) * param.s2
                                        + -48. * param.s1.powi(2) * param.s2.powi(2)
                                        + -70. * param.s1 * param.s2.powi(3)
                                        + 25. * param.s2.powi(4))
                                + param.s12
                                    * (-60. * param.s1.powi(5)
                                        + 614. * param.s1.powi(4) * param.s2
                                        + 1452. * param.s1.powi(3) * param.s2.powi(2)
                                        + -516. * param.s1.powi(2) * param.s2.powi(3)
                                        + 200. * param.s1 * param.s2.powi(4)
                                        + -30. * param.s2.powi(5)))
                            + param.s1.powi(4)
                                * (-8. * param.s12.powi(6)
                                    + 6. * param.s12.powi(5) * (7. * param.s1 + 22. * param.s2)
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (9. * param.s1.powi(2)
                                            + -25. * param.s1 * param.s2
                                            + -54. * param.s2.powi(2))
                                    + 30.
                                        * param.m2_2.powi(4)
                                        * (-41. * param.s1.powi(2)
                                            + 12. * param.s1 * param.s12
                                            + 29. * param.s12.powi(2)
                                            + 12. * param.s1 * param.s2
                                            + -58. * param.s12 * param.s2
                                            + 29. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-90. * param.s1.powi(2)
                                            + -343. * param.s1 * param.s2
                                            + 435. * param.s2.powi(2))
                                    + 20.
                                        * param.s12.powi(3)
                                        * (5. * param.s1.powi(3)
                                            + 11. * param.s1.powi(2) * param.s2
                                            + 32. * param.s1 * param.s2.powi(2)
                                            + -57. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (-60. * param.s1.powi(4)
                                            + 78. * param.s1.powi(3) * param.s2
                                            + -1116. * param.s1.powi(2) * param.s2.powi(2)
                                            + 618. * param.s1 * param.s2.powi(3)
                                            + 480. * param.s2.powi(4))
                                    + -40.
                                        * param.m2_2.powi(3)
                                        * (44. * param.s12.powi(3)
                                            + -45. * param.s12.powi(2) * (param.s1 + param.s2)
                                            + 43.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12
                                                * (21. * param.s1.powi(2)
                                                    + -80. * param.s1 * param.s2
                                                    + 21. * param.s2.powi(2)))
                                    + 15.
                                        * param.m2_2.powi(2)
                                        * (65. * param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (-160. * param.s1 + 92. * param.s2)
                                            + 6. * param.s12.powi(2)
                                                * (15. * param.s1.powi(2)
                                                    + 50. * param.s1 * param.s2
                                                    + -53. * param.s2.powi(2))
                                            + 4. * param.s12
                                                * (10. * param.s1.powi(3)
                                                    + -109. * param.s1.powi(2) * param.s2
                                                    + 74. * param.s1 * param.s2.powi(2)
                                                    + 25. * param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(3)
                                                * (35. * param.s1 + 61. * param.s2))
                                    + -6.
                                        * param.m2_2
                                        * (14. * param.s12.powi(5)
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1 + 11. * param.s2)
                                            + param.s12.powi(4)
                                                * (-55. * param.s1 + 255. * param.s2)
                                            + 8. * param.s12.powi(3)
                                                * (10. * param.s1.powi(2)
                                                    + -29. * param.s1 * param.s2
                                                    + -35. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (5. * param.s1.powi(2)
                                                    + 142. * param.s1 * param.s2
                                                    + 125. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (25. * param.s1.powi(3)
                                                    + 147. * param.s1.powi(2) * param.s2
                                                    + -513. * param.s1 * param.s2.powi(2)
                                                    + 125. * param.s2.powi(3)))
                                    - (2. * param.s1 + -7. * param.s2)
                                        * (param.s1 - param.s2).powi(5))
                            + param.m1_2
                                * param.s1.powi(3)
                                * (19. * param.s12.powi(6)
                                    + -6.
                                        * param.s12.powi(5)
                                        * (15. * param.s1 + 43. * param.s2)
                                    + param.s12.powi(4)
                                        * (165. * param.s1.powi(2)
                                            + 404. * param.s1 * param.s2
                                            + -105. * param.s2.powi(2))
                                    + -4.
                                        * param.s12.powi(3)
                                        * (35. * param.s1.powi(3)
                                            + -73. * param.s1.powi(2) * param.s2
                                            + 1031. * param.s1 * param.s2.powi(2)
                                            + -405. * param.s2.powi(3))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(3)
                                            + 125. * param.s1.powi(2) * param.s2
                                            + 1201. * param.s1 * param.s2.powi(2)
                                            + 303. * param.s2.powi(3))
                                    + 9. * param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + -80. * param.s1.powi(3) * param.s2
                                            + 266. * param.s1.powi(2) * param.s2.powi(2)
                                            + 312. * param.s1 * param.s2.powi(3)
                                            + -215. * param.s2.powi(4))
                                    + 40.
                                        * param.m2_2.powi(3)
                                        * (62. * param.s1.powi(3)
                                            + 25. * param.s12.powi(3)
                                            + 3. * param.s12.powi(2)
                                                * (4. * param.s1 + -25. * param.s2)
                                            + 61. * param.s1.powi(2) * param.s2
                                            + -98. * param.s1 * param.s2.powi(2)
                                            + -25. * param.s2.powi(3)
                                            + param.s12
                                                * (-99. * param.s1.powi(2)
                                                    + 86. * param.s1 * param.s2
                                                    + 75. * param.s2.powi(2)))
                                    + -30.
                                        * param.m2_2.powi(2)
                                        * (37. * param.s12.powi(4)
                                            + -48. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + -6.
                                                * param.s12.powi(2)
                                                * (13. * param.s1.powi(2)
                                                    + -74. * param.s1 * param.s2
                                                    + 13. * param.s2.powi(2))
                                            + 152.
                                                * param.s12
                                                * (param.s1.powi(3)
                                                    + -2. * param.s1.powi(2) * param.s2
                                                    + -2. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(2)
                                                * (63. * param.s1.powi(2)
                                                    + 218. * param.s1 * param.s2
                                                    + 63. * param.s2.powi(2)))
                                    + 6. * param.m2_2
                                        * (24. * param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (-75. * param.s1 + 250. * param.s2)
                                            + param.s12.powi(3)
                                                * (60. * param.s1.powi(2)
                                                    + 628. * param.s1 * param.s2
                                                    + -740. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (21. * param.s1.powi(2)
                                                    + 305. * param.s1 * param.s2
                                                    + 154. * param.s2.powi(2))
                                            + 6. * param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + -294. * param.s1.powi(2) * param.s2
                                                    + 221. * param.s1 * param.s2.powi(2)
                                                    + 80. * param.s2.powi(3))
                                            + -4.
                                                * param.s12
                                                * (15. * param.s1.powi(4)
                                                    + -161. * param.s1.powi(3) * param.s2
                                                    + -328.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 509. * param.s1 * param.s2.powi(3)
                                                    + -35. * param.s2.powi(4)))
                                    - (param.s1 - param.s2).powi(4)
                                        * (5. * param.s1.powi(2)
                                            + -24. * param.s1 * param.s2
                                            + -53. * param.s2.powi(2)))
                            + param.m1_2.powi(3)
                                * param.s1
                                * (-7. * param.s12.powi(6)
                                    + 66. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (95. * param.s1.powi(3)
                                            + 119. * param.s1.powi(2) * param.s2
                                            + 119. * param.s1 * param.s2.powi(2)
                                            + 95. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (-345. * param.s1.powi(4)
                                            + 624. * param.s1.powi(3) * param.s2
                                            + -4590. * param.s1.powi(2) * param.s2.powi(2)
                                            + 624. * param.s1 * param.s2.powi(3)
                                            + -345. * param.s2.powi(4))
                                    + 2. * param.s12
                                        * (81. * param.s1.powi(5)
                                            + -599. * param.s1.powi(4) * param.s2
                                            + 1278. * param.s1.powi(3) * param.s2.powi(2)
                                            + 1278. * param.s1.powi(2) * param.s2.powi(3)
                                            + -599. * param.s1 * param.s2.powi(4)
                                            + 81. * param.s2.powi(5))
                                    + -6.
                                        * param.m2_2
                                        * (-19. * param.s1.powi(5)
                                            + 4. * param.s12.powi(5)
                                            + -508. * param.s1.powi(4) * param.s2
                                            + -258. * param.s1.powi(3) * param.s2.powi(2)
                                            + 712. * param.s1.powi(2) * param.s2.powi(3)
                                            + 77. * param.s1 * param.s2.powi(4)
                                            + -4. * param.s2.powi(5)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (7. * param.s1 + 4. * param.s2)
                                            + 4. * param.s12.powi(3)
                                                * (25. * param.s1.powi(2)
                                                    + 7. * param.s1 * param.s2
                                                    + 10. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (65. * param.s1.powi(3)
                                                    + 252. * param.s1.powi(2) * param.s2
                                                    + -63. * param.s1 * param.s2.powi(2)
                                                    + 20. * param.s2.powi(3))
                                            + 4. * param.s12
                                                * (20. * param.s1.powi(4)
                                                    + 251. * param.s1.powi(3) * param.s2
                                                    + -77.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -49. * param.s1 * param.s2.powi(3)
                                                    + 5. * param.s2.powi(4)))
                                    - (param.s1 - param.s2).powi(2)
                                        * (31. * param.s1.powi(4)
                                            + -422. * param.s1.powi(3) * param.s2
                                            + -2658. * param.s1.powi(2) * param.s2.powi(2)
                                            + -422. * param.s1 * param.s2.powi(3)
                                            + 31. * param.s2.powi(4))
                                    - param.s12.powi(4)
                                        * (225. * param.s1.powi(2)
                                            + 452. * param.s1 * param.s2
                                            + 225. * param.s2.powi(2)))
                            + 3. * param.m1_2.powi(2)
                                * param.s1.powi(2)
                                * (5.
                                    * param.m2_2.powi(2)
                                    * (-91. * param.s1.powi(4)
                                        + 9. * param.s12.powi(4)
                                        + 4. * param.s12.powi(3)
                                            * (16. * param.s1 + -9. * param.s2)
                                        + -380. * param.s1.powi(3) * param.s2
                                        + 202. * param.s1.powi(2) * param.s2.powi(2)
                                        + 260. * param.s1 * param.s2.powi(3)
                                        + 9. * param.s2.powi(4)
                                        + -6.
                                            * param.s12.powi(2)
                                            * (41. * param.s1.powi(2)
                                                + -22. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (66. * param.s1.powi(3)
                                                + 71. * param.s1.powi(2) * param.s2
                                                + -114. * param.s1 * param.s2.powi(2)
                                                + -9. * param.s2.powi(3)))
                                    + -2.
                                        * param.m2_2
                                        * (6. * param.s12.powi(5)
                                            + 15. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -8.
                                                * param.s12.powi(3)
                                                * (15. * param.s1.powi(2)
                                                    + -104. * param.s1 * param.s2
                                                    + 15. * param.s2.powi(2))
                                            + 42.
                                                * param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + -23. * param.s1.powi(2) * param.s2
                                                    + -23. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(2)
                                                * (39. * param.s1.powi(3)
                                                    + 821. * param.s1.powi(2) * param.s2
                                                    + 821. * param.s1 * param.s2.powi(2)
                                                    + 39. * param.s2.powi(3))
                                            + -2.
                                                * param.s12
                                                * (75. * param.s1.powi(4)
                                                    + 312. * param.s1.powi(3) * param.s2
                                                    + -1534.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 312. * param.s1 * param.s2.powi(3)
                                                    + 75. * param.s2.powi(4)))
                                    + -3.
                                        * (param.s12.powi(6)
                                            + -2.
                                                * param.s12.powi(5)
                                                * (param.s1 + 5. * param.s2)
                                            + 4. * param.s1
                                                * param.s12.powi(3)
                                                * (5. * param.s1.powi(2)
                                                    + 31. * param.s1 * param.s2
                                                    + -83. * param.s2.powi(2))
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + -34. * param.s1 * param.s2
                                                    + 20. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (7. * param.s1.powi(5)
                                                    + -25. * param.s1.powi(4) * param.s2
                                                    + 334.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -224.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -109. * param.s1 * param.s2.powi(4)
                                                    + 17. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (25. * param.s1.powi(4)
                                                    + 68. * param.s1.powi(3) * param.s2
                                                    + 384.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -680. * param.s1 * param.s2.powi(3)
                                                    + 35. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (3. * param.s1.powi(3)
                                                    + -29. * param.s1.powi(2) * param.s2
                                                    + -124. * param.s1 * param.s2.powi(2)
                                                    + -10. * param.s2.powi(3)))))
                    - param.m1_2.powi(3)
                        * param.s1.powi(2)
                        * (10. * param.s12.powi(6)
                            + -12. * param.s12.powi(5) * (5. * param.s1 + 7. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (30. * param.s1.powi(2)
                                    + 46. * param.s1 * param.s2
                                    + 63. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (50. * param.s1.powi(3)
                                    + 20. * param.s1.powi(2) * param.s2
                                    + -43. * param.s1 * param.s2.powi(2)
                                    + 155. * param.s2.powi(3))
                            + (param.s1 - param.s2).powi(2)
                                * (10. * param.s1.powi(4)
                                    + -86. * param.s1.powi(3) * param.s2
                                    + 441. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1276. * param.s1 * param.s2.powi(3)
                                    + 79. * param.s2.powi(4))
                            + 6. * param.s12.powi(2)
                                * (25. * param.s1.powi(4)
                                    + -50. * param.s1.powi(3) * param.s2
                                    + -111. * param.s1.powi(2) * param.s2.powi(2)
                                    + 54. * param.s1 * param.s2.powi(3)
                                    + 110. * param.s2.powi(4))
                            + -4.
                                * param.s12
                                * (15. * param.s1.powi(5)
                                    + -85. * param.s1.powi(4) * param.s2
                                    + 111. * param.s1.powi(3) * param.s2.powi(2)
                                    + -957. * param.s1.powi(2) * param.s2.powi(3)
                                    + 446. * param.s1 * param.s2.powi(4)
                                    + 90. * param.s2.powi(5))
                            + 15.
                                * param.m2_2.powi(2)
                                * (3. * param.s1.powi(4)
                                    + 3. * param.s12.powi(4)
                                    + 120. * param.s1.powi(3) * param.s2
                                    + 314. * param.s1.powi(2) * param.s2.powi(2)
                                    + 120. * param.s1 * param.s2.powi(3)
                                    + 3. * param.s2.powi(4)
                                    + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 18.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -12.
                                        * param.s12
                                        * (param.s1.powi(3)
                                            + 21. * param.s1.powi(2) * param.s2
                                            + 21. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + 6. * param.m2_2
                                * (-4. * param.s1.powi(5)
                                    + 4. * param.s12.powi(5)
                                    + 77. * param.s1.powi(4) * param.s2
                                    + 712. * param.s1.powi(3) * param.s2.powi(2)
                                    + -258. * param.s1.powi(2) * param.s2.powi(3)
                                    + -508. * param.s1 * param.s2.powi(4)
                                    + -19. * param.s2.powi(5)
                                    + -5. * param.s12.powi(4) * (4. * param.s1 + 7. * param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (10. * param.s1.powi(2)
                                            + 7. * param.s1 * param.s2
                                            + 25. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (20. * param.s1.powi(3)
                                            + -63. * param.s1.powi(2) * param.s2
                                            + 252. * param.s1 * param.s2.powi(2)
                                            + 65. * param.s2.powi(3))
                                    + 4. * param.s12
                                        * (5. * param.s1.powi(4)
                                            + -49. * param.s1.powi(3) * param.s2
                                            + -77. * param.s1.powi(2) * param.s2.powi(2)
                                            + 251. * param.s1 * param.s2.powi(3)
                                            + 20. * param.s2.powi(4))))
                    - param.m1_2.powi(5)
                        * (param.s1.powi(6)
                            + param.s12.powi(6)
                            + -13. * param.s1.powi(5) * param.s2
                            + 113. * param.s1.powi(4) * param.s2.powi(2)
                            + 638. * param.s1.powi(3) * param.s2.powi(3)
                            + 113. * param.s1.powi(2) * param.s2.powi(4)
                            + -13. * param.s1 * param.s2.powi(5)
                            + param.s2.powi(6)
                            + -6. * param.s12.powi(5) * (param.s1 + param.s2)
                            + param.s12.powi(4)
                                * (15. * param.s1.powi(2)
                                    + 11. * param.s1 * param.s2
                                    + 15. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (5. * param.s1.powi(3)
                                    + -4. * param.s1.powi(2) * param.s2
                                    + -4. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3))
                            + 3. * param.s12.powi(2)
                                * (5. * param.s1.powi(4)
                                    + -18. * param.s1.powi(3) * param.s2
                                    + 12. * param.s1.powi(2) * param.s2.powi(2)
                                    + -18. * param.s1 * param.s2.powi(3)
                                    + 5. * param.s2.powi(4))
                            + -2.
                                * param.s12
                                * (3. * param.s1.powi(5)
                                    + -23. * param.s1.powi(4) * param.s2
                                    + 90. * param.s1.powi(3) * param.s2.powi(2)
                                    + 90. * param.s1.powi(2) * param.s2.powi(3)
                                    + -23. * param.s1 * param.s2.powi(4)
                                    + 3. * param.s2.powi(5))))
                    * param.lambda_m01_sqrt
                    * param.lambda_s12_sqrt)
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.0006944444444444444
            * std::f64::consts::PI
            * param.s2.powi(-3)
            * param.lambda_s12_sqrt.powi(-9)
            * (60.
                * param.s2.powi(3)
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
                * (7. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1) * param.s2
                    + param.m0_2.powi(2)
                        * (4. * param.s12.powi(2) + -3. * (param.s1 - param.s2).powi(2)
                            - param.s12 * (param.s1 + param.s2))
                    + param.m0_2
                        * (3. * param.s1.powi(3)
                            + -5. * param.s1.powi(2) * param.s12
                            + param.s1 * param.s12.powi(2)
                            + param.s12.powi(3)
                            + -3. * param.s1.powi(2) * param.s2
                            + 14. * param.s1 * param.s12 * param.s2
                            + param.s12.powi(2) * param.s2
                            + -3. * param.s1 * param.s2.powi(2)
                            + -5. * param.s12 * param.s2.powi(2)
                            + 3. * param.s2.powi(3)
                            + -2.
                                * param.m1_2
                                * (2. * param.s1.powi(2)
                                    + -4. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + 3. * param.s12 * param.s2
                                    + -5. * param.s2.powi(2))
                            + -2.
                                * param.m2_2
                                * (-5. * param.s1.powi(2)
                                    + 3. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + -4. * param.s12 * param.s2
                                    + 2. * param.s2.powi(2)))
                    + 2. * param.m1_2
                        * (2.
                            * param.m2_2
                            * (param.s1.powi(2)
                                + param.s12.powi(2)
                                + 5. * param.s1 * param.s2
                                + param.s2.powi(2)
                                + -2. * param.s12 * (param.s1 + param.s2))
                            - param.s2
                                * (-5. * param.s1.powi(2)
                                    + 3. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + -4. * param.s12 * param.s2
                                    + 2. * param.s2.powi(2)))
                    - param.s1
                        * (7. * param.m2_2.powi(2) * (param.s1 + param.s2 - param.s12)
                            + 2. * param.m2_2
                                * (2. * param.s1.powi(2)
                                    + -4. * param.s1 * param.s12
                                    + 2. * param.s12.powi(2)
                                    + 3. * param.s1 * param.s2
                                    + 3. * param.s12 * param.s2
                                    + -5. * param.s2.powi(2))
                            + param.s2
                                * (-4. * param.s12.powi(2)
                                    + 3. * (param.s1 - param.s2).powi(2)
                                    + param.s12 * (param.s1 + param.s2))))
                * log_diff(
                    (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                        + param.m2_2 * (param.s1 + param.s2 - param.s12)
                        + param.m0_2 * (param.s12 + param.s2 - param.s1),
                    param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                )
                - (param.m0_2.powi(5)
                    * (param.s12.powi(6)
                        + (param.s1 + -2. * param.s2) * (param.s1 - param.s2).powi(5)
                        + -6. * param.s12.powi(5) * (param.s1 + 2. * param.s2)
                        + -2.
                            * param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (3. * param.s1.powi(2)
                                + param.s1 * param.s2
                                + -18. * param.s2.powi(2))
                        + param.s12.powi(4)
                            * (15. * param.s1.powi(2)
                                + 41. * param.s1 * param.s2
                                + 90. * param.s2.powi(2))
                        + -4.
                            * param.s12.powi(3)
                            * (5. * param.s1.powi(3)
                                + 11. * param.s1.powi(2) * param.s2
                                + 29. * param.s1 * param.s2.powi(2)
                                + -60. * param.s2.powi(3))
                        + 3. * param.s12.powi(2)
                            * (5. * param.s1.powi(4)
                                + 2. * param.s1.powi(3) * param.s2
                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                + 94. * param.s1 * param.s2.powi(3)
                                + -95. * param.s2.powi(4)))
                    + param.s2.powi(5)
                        * (param.s12.powi(6)
                            + (param.s1 - param.s2).powi(5) * (2. * param.s1 - param.s2)
                            + -420. * param.m1_2.powi(5) * (param.s12 - param.s2 - param.s1)
                            + -6. * param.s12.powi(5) * (2. * param.s1 + param.s2)
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (18. * param.s1.powi(2) + -3. * param.s2.powi(2)
                                    - param.s1 * param.s2)
                            + param.s12.powi(4)
                                * (90. * param.s1.powi(2)
                                    + 41. * param.s1 * param.s2
                                    + 15. * param.s2.powi(2))
                            + 30.
                                * param.m1_2.powi(4)
                                * (-41. * param.s1.powi(2)
                                    + 12. * param.s1 * param.s12
                                    + 29. * param.s12.powi(2)
                                    + 12. * param.s1 * param.s2
                                    + -58. * param.s12 * param.s2
                                    + 29. * param.s2.powi(2))
                            + 4. * param.s12.powi(3)
                                * (60. * param.s1.powi(3)
                                    + -29. * param.s1.powi(2) * param.s2
                                    + -11. * param.s1 * param.s2.powi(2)
                                    + -5. * param.s2.powi(3))
                            + -3.
                                * param.s12.powi(2)
                                * (95. * param.s1.powi(4)
                                    + -94. * param.s1.powi(3) * param.s2
                                    + 6. * param.s1.powi(2) * param.s2.powi(2)
                                    + -2. * param.s1 * param.s2.powi(3)
                                    + -5. * param.s2.powi(4))
                            + -20.
                                * param.m1_2.powi(3)
                                * (25. * param.s12.powi(3)
                                    + param.s12.powi(2) * (99. * param.s1 + -75. * param.s2)
                                    + param.s12
                                        * (-63. * param.s1.powi(2)
                                            + -88. * param.s1 * param.s2
                                            + 75. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2)
                                        * (61. * param.s1 + 25. * param.s2))
                            + 6. * param.m1_2
                                * (param.s12.powi(5)
                                    + (param.s1 - param.s2).powi(4) * (4. * param.s1 - param.s2)
                                    + -5. * param.s12.powi(4) * (4. * param.s1 + param.s2)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (125. * param.s1.powi(2)
                                            + 6. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (-180. * param.s1.powi(2)
                                            + 52. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (35. * param.s1.powi(3)
                                            + 42. * param.s1.powi(2) * param.s2
                                            + -18. * param.s1 * param.s2.powi(2)
                                            + -5. * param.s2.powi(3)))
                            + 15.
                                * param.m1_2.powi(2)
                                * (3. * param.s12.powi(4)
                                    + 4. * param.s12.powi(3) * (22. * param.s1 + -3. * param.s2)
                                    + 6. * param.s12.powi(2)
                                        * (11. * param.s1.powi(2)
                                            + -26. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (32. * param.s1.powi(3)
                                            + -23. * param.s1.powi(2) * param.s2
                                            + -12. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(3)
                                        * (29. * param.s1 + 3. * param.s2)))
                    + param.m2_2
                        * param.s2.powi(4)
                        * (-5. * param.s12.powi(6)
                            + 6. * param.s12.powi(5) * (9. * param.s1 + 5. * param.s2)
                            + -5.
                                * param.s12.powi(4)
                                * (66. * param.s1.powi(2)
                                    + 35. * param.s1 * param.s2
                                    + 15. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (30. * param.s1.powi(3)
                                    + -67. * param.s1.powi(2) * param.s2
                                    + -40. * param.s1 * param.s2.powi(2)
                                    + -25. * param.s2.powi(3))
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (285. * param.s1.powi(3)
                                    + 113. * param.s1.powi(2) * param.s2
                                    + 25. * param.s1 * param.s2.powi(2)
                                    + -15. * param.s2.powi(3))
                            + 3. * param.s12.powi(2)
                                * (335. * param.s1.powi(4)
                                    + -638. * param.s1.powi(3) * param.s2
                                    + 102. * param.s1.powi(2) * param.s2.powi(2)
                                    + 10. * param.s1 * param.s2.powi(3)
                                    + -25. * param.s2.powi(4))
                            + -30.
                                * param.m1_2.powi(4)
                                * (29. * param.s1.powi(2)
                                    + 29. * param.s12.powi(2)
                                    + 82. * param.s1 * param.s2
                                    + 29. * param.s2.powi(2)
                                    + -58. * param.s12 * (param.s1 + param.s2))
                            + 40.
                                * param.m1_2.powi(3)
                                * (62. * param.s1.powi(3)
                                    + 25. * param.s12.powi(3)
                                    + 3. * param.s12.powi(2) * (4. * param.s1 + -25. * param.s2)
                                    + 61. * param.s1.powi(2) * param.s2
                                    + -98. * param.s1 * param.s2.powi(2)
                                    + -25. * param.s2.powi(3)
                                    + param.s12
                                        * (-99. * param.s1.powi(2)
                                            + 86. * param.s1 * param.s2
                                            + 75. * param.s2.powi(2)))
                            + -15.
                                * param.m1_2.powi(2)
                                * (9. * param.s12.powi(4)
                                    + 4. * param.s12.powi(3) * (41. * param.s1 + -9. * param.s2)
                                    + -6.
                                        * param.s12.powi(2)
                                        * (33. * param.s1.powi(2)
                                            + 28. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2))
                                    + (param.s1 - param.s2).powi(2)
                                        * (157. * param.s1.powi(2)
                                            + 178. * param.s1 * param.s2
                                            + 9. * param.s2.powi(2))
                                    + -4.
                                        * param.s12
                                        * (33. * param.s1.powi(3)
                                            + -157. * param.s1.powi(2) * param.s2
                                            + 39. * param.s1 * param.s2.powi(2)
                                            + 9. * param.s2.powi(3)))
                            + -6.
                                * param.m1_2
                                * (4. * param.s12.powi(5)
                                    + -5. * param.s12.powi(4) * (13. * param.s1 + 4. * param.s2)
                                    + param.s12.powi(3)
                                        * (-280. * param.s1.powi(2)
                                            + 148. * param.s1 * param.s2
                                            + 40. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (610. * param.s1.powi(3)
                                            + -444. * param.s1.powi(2) * param.s2
                                            + -54. * param.s1 * param.s2.powi(2)
                                            + -40. * param.s2.powi(3))
                                    + -4.
                                        * param.s12
                                        * (35. * param.s1.powi(4)
                                            + 129. * param.s1.powi(3) * param.s2
                                            + -178. * param.s1.powi(2) * param.s2.powi(2)
                                            + 19. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (129. * param.s1.powi(2)
                                            + 35. * param.s1 * param.s2
                                            + -4. * param.s2.powi(2)))
                            - (param.s1 - param.s2).powi(4)
                                * (34. * param.s1.powi(2)
                                    + -21. * param.s1 * param.s2
                                    + 5. * param.s2.powi(2)))
                    + param.m2_2.powi(4)
                        * param.s2
                        * (11. * param.s1.powi(6)
                            + 5. * param.s12.powi(6)
                            + -203. * param.s1.powi(5) * param.s2
                            + -1403. * param.s1.powi(4) * param.s2.powi(2)
                            + 1222. * param.s1.powi(3) * param.s2.powi(3)
                            + 427. * param.s1.powi(2) * param.s2.powi(4)
                            + -59. * param.s1 * param.s2.powi(5)
                            + 5. * param.s2.powi(6)
                            + -6. * param.s12.powi(5) * (6. * param.s1 + 5. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (21. * param.s1.powi(2)
                                    + 17. * param.s1 * param.s2
                                    + 15. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (40. * param.s1.powi(3)
                                    + -32. * param.s1.powi(2) * param.s2
                                    + -5. * param.s1 * param.s2.powi(2)
                                    + 25. * param.s2.powi(3))
                            + 3. * param.s12.powi(2)
                                * (45. * param.s1.powi(4)
                                    + -198. * param.s1.powi(3) * param.s2
                                    + -48. * param.s1.powi(2) * param.s2.powi(2)
                                    + -70. * param.s1 * param.s2.powi(3)
                                    + 25. * param.s2.powi(4))
                            + param.s12
                                * (-60. * param.s1.powi(5)
                                    + 614. * param.s1.powi(4) * param.s2
                                    + 1452. * param.s1.powi(3) * param.s2.powi(2)
                                    + -516. * param.s1.powi(2) * param.s2.powi(3)
                                    + 200. * param.s1 * param.s2.powi(4)
                                    + -30. * param.s2.powi(5))
                            + 6. * param.m1_2
                                * (param.s12.powi(5)
                                    + 23. * param.s1.powi(4) * param.s2
                                    + 328. * param.s1.powi(3) * param.s2.powi(2)
                                    + 328. * param.s1.powi(2) * param.s2.powi(3)
                                    + 23. * param.s1 * param.s2.powi(4)
                                    + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (5. * param.s1.powi(2)
                                            + -4. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(3)
                                            + -27. * param.s1.powi(2) * param.s2
                                            + -27. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3))
                                    + param.s12
                                        * (5. * param.s1.powi(4)
                                            + -64. * param.s1.powi(3) * param.s2
                                            + -392. * param.s1.powi(2) * param.s2.powi(2)
                                            + -64. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))
                                    - param.s2.powi(5)
                                    - param.s1.powi(5)))
                    + param.m2_2.powi(2)
                        * param.s2.powi(3)
                        * (10. * param.s12.powi(6)
                            + -12. * param.s12.powi(5) * (8. * param.s1 + 5. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (93. * param.s1.powi(2)
                                    + 58. * param.s1 * param.s2
                                    + 30. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (150. * param.s1.powi(3)
                                    + 23. * param.s1.powi(2) * param.s2
                                    + 50. * param.s1 * param.s2.powi(2)
                                    + 50. * param.s2.powi(3))
                            + -6.
                                * param.s12.powi(2)
                                * (30. * param.s1.powi(4)
                                    + -416. * param.s1.powi(3) * param.s2
                                    + 129. * param.s1.powi(2) * param.s2.powi(2)
                                    + 30. * param.s1 * param.s2.powi(3)
                                    + -25. * param.s2.powi(4))
                            + 4. * param.s12
                                * (180. * param.s1.powi(5)
                                    + -844. * param.s1.powi(4) * param.s2
                                    + 618. * param.s1.powi(3) * param.s2.powi(2)
                                    + -9. * param.s1.powi(2) * param.s2.powi(3)
                                    + 70. * param.s1 * param.s2.powi(4)
                                    + -15. * param.s2.powi(5))
                            + -100.
                                * param.m1_2.powi(3)
                                * (-5. * param.s1.powi(3)
                                    + 5. * param.s12.powi(3)
                                    + -37. * param.s1.powi(2) * param.s2
                                    + -37. * param.s1 * param.s2.powi(2)
                                    + -5. * param.s2.powi(3)
                                    + -15. * param.s12.powi(2) * (param.s1 + param.s2)
                                    + param.s12
                                        * (15. * param.s1.powi(2)
                                            + 52. * param.s1 * param.s2
                                            + 15. * param.s2.powi(2)))
                            + 15.
                                * param.m1_2.powi(2)
                                * (-91. * param.s1.powi(4)
                                    + 9. * param.s12.powi(4)
                                    + 4. * param.s12.powi(3) * (16. * param.s1 + -9. * param.s2)
                                    + -380. * param.s1.powi(3) * param.s2
                                    + 202. * param.s1.powi(2) * param.s2.powi(2)
                                    + 260. * param.s1 * param.s2.powi(3)
                                    + 9. * param.s2.powi(4)
                                    + -6.
                                        * param.s12.powi(2)
                                        * (41. * param.s1.powi(2)
                                            + -22. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2))
                                    + 4. * param.s12
                                        * (66. * param.s1.powi(3)
                                            + 71. * param.s1.powi(2) * param.s2
                                            + -114. * param.s1 * param.s2.powi(2)
                                            + -9. * param.s2.powi(3)))
                            + 6. * param.m1_2
                                * (6. * param.s12.powi(5)
                                    + -15. * param.s12.powi(4) * (5. * param.s1 + 2. * param.s2)
                                    + param.s12.powi(3)
                                        * (-10. * param.s1.powi(2)
                                            + 132. * param.s1 * param.s2
                                            + 60. * param.s2.powi(2))
                                    + 6. * param.s12.powi(2)
                                        * (70. * param.s1.powi(3)
                                            + -181. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -10. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(2)
                                        * (199. * param.s1.powi(3)
                                            + 586. * param.s1.powi(2) * param.s2
                                            + 81. * param.s1 * param.s2.powi(2)
                                            + -6. * param.s2.powi(3))
                                    + param.s12
                                        * (-540. * param.s1.powi(4)
                                            + 796. * param.s1.powi(3) * param.s2
                                            + 678. * param.s1.powi(2) * param.s2.powi(2)
                                            + -204. * param.s1 * param.s2.powi(3)
                                            + 30. * param.s2.powi(4)))
                            - (param.s1 - param.s2).powi(3)
                                * (319. * param.s1.powi(3)
                                    + 215. * param.s1.powi(2) * param.s2
                                    + -64. * param.s1 * param.s2.powi(2)
                                    + 10. * param.s2.powi(3)))
                    + param.m0_2.powi(4)
                        * (param.s2
                            * (-8. * param.s12.powi(6)
                                + 6. * param.s12.powi(5) * (7. * param.s1 + 22. * param.s2)
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (9. * param.s1.powi(2)
                                        + -25. * param.s1 * param.s2
                                        + -54. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-90. * param.s1.powi(2)
                                        + -343. * param.s1 * param.s2
                                        + 435. * param.s2.powi(2))
                                + 20.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(3)
                                        + 11. * param.s1.powi(2) * param.s2
                                        + 32. * param.s1 * param.s2.powi(2)
                                        + -57. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-60. * param.s1.powi(4)
                                        + 78. * param.s1.powi(3) * param.s2
                                        + -1116. * param.s1.powi(2) * param.s2.powi(2)
                                        + 618. * param.s1 * param.s2.powi(3)
                                        + 480. * param.s2.powi(4))
                                + 6. * param.m1_2
                                    * (param.s12.powi(5)
                                        + -5. * param.s12.powi(4) * (param.s1 + 4. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + -90. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 125. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-10. * param.s1.powi(3)
                                                + -36. * param.s1.powi(2) * param.s2
                                                + 84. * param.s1 * param.s2.powi(2)
                                                + 70. * param.s2.powi(3))
                                        - (param.s1 + -4. * param.s2)
                                            * (param.s1 - param.s2).powi(4))
                                - (2. * param.s1 + -7. * param.s2)
                                    * (param.s1 - param.s2).powi(5))
                            - param.m2_2
                                * (5. * param.s12.powi(6)
                                    + -6. * param.s12.powi(5) * (5. * param.s1 + 9. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                        * (5. * param.s1.powi(2)
                                            + -21. * param.s1 * param.s2
                                            + 34. * param.s2.powi(2))
                                    + 5. * param.s12.powi(4)
                                        * (15. * param.s1.powi(2)
                                            + 35. * param.s1 * param.s2
                                            + 66. * param.s2.powi(2))
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (15. * param.s1.powi(3)
                                            + -25. * param.s1.powi(2) * param.s2
                                            + -113. * param.s1 * param.s2.powi(2)
                                            + -285. * param.s2.powi(3))
                                    + -4.
                                        * param.s12.powi(3)
                                        * (25. * param.s1.powi(3)
                                            + 40. * param.s1.powi(2) * param.s2
                                            + 67. * param.s1 * param.s2.powi(2)
                                            + -30. * param.s2.powi(3))
                                    + 3. * param.s12.powi(2)
                                        * (25. * param.s1.powi(4)
                                            + -10. * param.s1.powi(3) * param.s2
                                            + -102. * param.s1.powi(2) * param.s2.powi(2)
                                            + 638. * param.s1 * param.s2.powi(3)
                                            + -335. * param.s2.powi(4))))
                    + param.m0_2.powi(3)
                        * (param.m2_2.powi(2)
                            * (10. * param.s12.powi(6)
                                + -12. * param.s12.powi(5) * (5. * param.s1 + 8. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (30. * param.s1.powi(2)
                                        + 58. * param.s1 * param.s2
                                        + 93. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (50. * param.s1.powi(3)
                                        + 50. * param.s1.powi(2) * param.s2
                                        + 23. * param.s1 * param.s2.powi(2)
                                        + 150. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(3)
                                    * (10. * param.s1.powi(3)
                                        + -64. * param.s1.powi(2) * param.s2
                                        + 215. * param.s1 * param.s2.powi(2)
                                        + 319. * param.s2.powi(3))
                                + 6. * param.s12.powi(2)
                                    * (25. * param.s1.powi(4)
                                        + -30. * param.s1.powi(3) * param.s2
                                        + -129. * param.s1.powi(2) * param.s2.powi(2)
                                        + 416. * param.s1 * param.s2.powi(3)
                                        + -30. * param.s2.powi(4))
                                + -4.
                                    * param.s12
                                    * (15. * param.s1.powi(5)
                                        + -70. * param.s1.powi(4) * param.s2
                                        + 9. * param.s1.powi(3) * param.s2.powi(2)
                                        + -618. * param.s1.powi(2) * param.s2.powi(3)
                                        + 844. * param.s1 * param.s2.powi(4)
                                        + -180. * param.s2.powi(5)))
                            + param.s2.powi(2)
                                * (37. * param.s12.powi(6)
                                    + -6.
                                        * param.s12.powi(5)
                                        * (23. * param.s1 + -47. * param.s2)
                                    + -2.
                                        * (param.s1 - param.s2).powi(5)
                                        * (param.s1 + 4. * param.s2)
                                    + 2. * param.s12.powi(4)
                                        * (90. * param.s1.powi(2)
                                            + 331. * param.s1 * param.s2
                                            + -495. * param.s2.powi(2))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (9. * param.s1.powi(2)
                                            + 80. * param.s1 * param.s2
                                            + 51. * param.s2.powi(2))
                                    + -20.
                                        * param.s12.powi(3)
                                        * (4. * param.s1.powi(3)
                                            + 103. * param.s1.powi(2) * param.s2
                                            + -68. * param.s1 * param.s2.powi(2)
                                            + -45. * param.s2.powi(3))
                                    + -3.
                                        * param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + -336. * param.s1.powi(3) * param.s2
                                            + -378. * param.s1.powi(2) * param.s2.powi(2)
                                            + 664. * param.s1 * param.s2.powi(3)
                                            + 45. * param.s2.powi(4))
                                    + 15.
                                        * param.m1_2.powi(2)
                                        * (3. * param.s12.powi(4)
                                            + (param.s1 - param.s2).powi(3)
                                                * (3. * param.s1 + 29. * param.s2)
                                            + param.s12.powi(3)
                                                * (-12. * param.s1 + 88. * param.s2)
                                            + 6. * param.s12.powi(2)
                                                * (3. * param.s1.powi(2)
                                                    + -26. * param.s1 * param.s2
                                                    + 11. * param.s2.powi(2))
                                            + -4.
                                                * param.s12
                                                * (3. * param.s1.powi(3)
                                                    + -12. * param.s1.powi(2) * param.s2
                                                    + -23. * param.s1 * param.s2.powi(2)
                                                    + 32. * param.s2.powi(3)))
                                    + -6.
                                        * param.m1_2
                                        * (14. * param.s12.powi(5)
                                            + (param.s1 - param.s2).powi(4)
                                                * (param.s1 + 11. * param.s2)
                                            + param.s12.powi(4)
                                                * (-55. * param.s1 + 255. * param.s2)
                                            + 8. * param.s12.powi(3)
                                                * (10. * param.s1.powi(2)
                                                    + -29. * param.s1 * param.s2
                                                    + -35. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (5. * param.s1.powi(2)
                                                    + 142. * param.s1 * param.s2
                                                    + 125. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (25. * param.s1.powi(3)
                                                    + 147. * param.s1.powi(2) * param.s2
                                                    + -513. * param.s1 * param.s2.powi(2)
                                                    + 125. * param.s2.powi(3))))
                            + param.m2_2
                                * param.s2
                                * (19. * param.s12.powi(6)
                                    + -6.
                                        * param.s12.powi(5)
                                        * (15. * param.s1 + 43. * param.s2)
                                    + param.s12.powi(4)
                                        * (165. * param.s1.powi(2)
                                            + 404. * param.s1 * param.s2
                                            + -105. * param.s2.powi(2))
                                    + -4.
                                        * param.s12.powi(3)
                                        * (35. * param.s1.powi(3)
                                            + -73. * param.s1.powi(2) * param.s2
                                            + 1031. * param.s1 * param.s2.powi(2)
                                            + -405. * param.s2.powi(3))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(3)
                                            + 125. * param.s1.powi(2) * param.s2
                                            + 1201. * param.s1 * param.s2.powi(2)
                                            + 303. * param.s2.powi(3))
                                    + 9. * param.s12.powi(2)
                                        * (5. * param.s1.powi(4)
                                            + -80. * param.s1.powi(3) * param.s2
                                            + 266. * param.s1.powi(2) * param.s2.powi(2)
                                            + 312. * param.s1 * param.s2.powi(3)
                                            + -215. * param.s2.powi(4))
                                    + -6.
                                        * param.m1_2
                                        * (4. * param.s12.powi(5)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (4. * param.s1 + 13. * param.s2)
                                            + 4. * param.s12.powi(3)
                                                * (10. * param.s1.powi(2)
                                                    + 37. * param.s1 * param.s2
                                                    + -70. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (20. * param.s1.powi(3)
                                                    + 27. * param.s1.powi(2) * param.s2
                                                    + 222. * param.s1 * param.s2.powi(2)
                                                    + -305. * param.s2.powi(3))
                                            + 4. * param.s12
                                                * (5. * param.s1.powi(4)
                                                    + -19. * param.s1.powi(3) * param.s2
                                                    + 178.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -129. * param.s1 * param.s2.powi(3)
                                                    + -35. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (4. * param.s1.powi(2)
                                                    + -35. * param.s1 * param.s2
                                                    + -129. * param.s2.powi(2)))
                                    - (param.s1 - param.s2).powi(4)
                                        * (5. * param.s1.powi(2)
                                            + -24. * param.s1 * param.s2
                                            + -53. * param.s2.powi(2))))
                    + param.m0_2
                        * (param.m2_2.powi(4)
                            * (5. * param.s1.powi(6)
                                + 5. * param.s12.powi(6)
                                + -59. * param.s1.powi(5) * param.s2
                                + 427. * param.s1.powi(4) * param.s2.powi(2)
                                + 1222. * param.s1.powi(3) * param.s2.powi(3)
                                + -1403. * param.s1.powi(2) * param.s2.powi(4)
                                + -203. * param.s1 * param.s2.powi(5)
                                + 11. * param.s2.powi(6)
                                + -6. * param.s12.powi(5) * (5. * param.s1 + 6. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (15. * param.s1.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + 21. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (25. * param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + -32. * param.s1 * param.s2.powi(2)
                                        + 40. * param.s2.powi(3))
                                + 3. * param.s12.powi(2)
                                    * (25. * param.s1.powi(4)
                                        + -70. * param.s1.powi(3) * param.s2
                                        + -48. * param.s1.powi(2) * param.s2.powi(2)
                                        + -198. * param.s1 * param.s2.powi(3)
                                        + 45. * param.s2.powi(4))
                                + param.s12
                                    * (-30. * param.s1.powi(5)
                                        + 200. * param.s1.powi(4) * param.s2
                                        + -516. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1452. * param.s1.powi(2) * param.s2.powi(3)
                                        + 614. * param.s1 * param.s2.powi(4)
                                        + -60. * param.s2.powi(5)))
                            + param.s2.powi(4)
                                * (-8. * param.s12.powi(6)
                                    + 6. * param.s12.powi(5) * (22. * param.s1 + 7. * param.s2)
                                    + param.s12.powi(4)
                                        * (435. * param.s1.powi(2)
                                            + -343. * param.s1 * param.s2
                                            + -90. * param.s2.powi(2))
                                    + 30.
                                        * param.m1_2.powi(4)
                                        * (29. * param.s1.powi(2)
                                            + -58. * param.s1 * param.s12
                                            + 29. * param.s12.powi(2)
                                            + 12. * param.s1 * param.s2
                                            + 12. * param.s12 * param.s2
                                            + -41. * param.s2.powi(2))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (54. * param.s1.powi(2)
                                            + 25. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2))
                                    + -20.
                                        * param.s12.powi(3)
                                        * (57. * param.s1.powi(3)
                                            + -32. * param.s1.powi(2) * param.s2
                                            + -11. * param.s1 * param.s2.powi(2)
                                            + -5. * param.s2.powi(3))
                                    + 6. * param.s12.powi(2)
                                        * (80. * param.s1.powi(4)
                                            + 103. * param.s1.powi(3) * param.s2
                                            + -186. * param.s1.powi(2) * param.s2.powi(2)
                                            + 13. * param.s1 * param.s2.powi(3)
                                            + -10. * param.s2.powi(4))
                                    + -40.
                                        * param.m1_2.powi(3)
                                        * (44. * param.s12.powi(3)
                                            + -45. * param.s12.powi(2) * (param.s1 + param.s2)
                                            + 43.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1 + param.s2)
                                            + -2.
                                                * param.s12
                                                * (21. * param.s1.powi(2)
                                                    + -80. * param.s1 * param.s2
                                                    + 21. * param.s2.powi(2)))
                                    + 15.
                                        * param.m1_2.powi(2)
                                        * (65. * param.s12.powi(4)
                                            + 4. * param.s12.powi(3)
                                                * (23. * param.s1 + -40. * param.s2)
                                            + (param.s1 - param.s2).powi(3)
                                                * (61. * param.s1 + 35. * param.s2)
                                            + param.s12.powi(2)
                                                * (-318. * param.s1.powi(2)
                                                    + 300. * param.s1 * param.s2
                                                    + 90. * param.s2.powi(2))
                                            + 4. * param.s12
                                                * (25. * param.s1.powi(3)
                                                    + 74. * param.s1.powi(2) * param.s2
                                                    + -109. * param.s1 * param.s2.powi(2)
                                                    + 10. * param.s2.powi(3)))
                                    + -6.
                                        * param.m1_2
                                        * (14. * param.s12.powi(5)
                                            + 5. * param.s12.powi(4)
                                                * (51. * param.s1 + -11. * param.s2)
                                            + (param.s1 - param.s2).powi(4)
                                                * (11. * param.s1 + param.s2)
                                            + -8.
                                                * param.s12.powi(3)
                                                * (35. * param.s1.powi(2)
                                                    + 29. * param.s1 * param.s2
                                                    + -10. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (param.s1 - param.s2).powi(2)
                                                * (125. * param.s1.powi(2)
                                                    + 142. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (125. * param.s1.powi(3)
                                                    + -513. * param.s1.powi(2) * param.s2
                                                    + 147. * param.s1 * param.s2.powi(2)
                                                    + 25. * param.s2.powi(3)))
                                    - (7. * param.s1 + -2. * param.s2)
                                        * (param.s1 - param.s2).powi(5))
                            + param.m2_2
                                * param.s2.powi(3)
                                * (19. * param.s12.powi(6)
                                    + -6.
                                        * param.s12.powi(5)
                                        * (43. * param.s1 + 15. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                        * (53. * param.s1.powi(2)
                                            + 24. * param.s1 * param.s2
                                            + -5. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-105. * param.s1.powi(2)
                                            + 404. * param.s1 * param.s2
                                            + 165. * param.s2.powi(2))
                                    + 4. * param.s12.powi(3)
                                        * (405. * param.s1.powi(3)
                                            + -1031. * param.s1.powi(2) * param.s2
                                            + 73. * param.s1 * param.s2.powi(2)
                                            + -35. * param.s2.powi(3))
                                    + 2. * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (303. * param.s1.powi(3)
                                            + 1201. * param.s1.powi(2) * param.s2
                                            + 125. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3))
                                    + -9.
                                        * param.s12.powi(2)
                                        * (215. * param.s1.powi(4)
                                            + -312. * param.s1.powi(3) * param.s2
                                            + -266. * param.s1.powi(2) * param.s2.powi(2)
                                            + 80. * param.s1 * param.s2.powi(3)
                                            + -5. * param.s2.powi(4))
                                    + 40.
                                        * param.m1_2.powi(3)
                                        * (-25. * param.s1.powi(3)
                                            + 25. * param.s12.powi(3)
                                            + -98. * param.s1.powi(2) * param.s2
                                            + 61. * param.s1 * param.s2.powi(2)
                                            + 62. * param.s2.powi(3)
                                            + param.s12.powi(2)
                                                * (-75. * param.s1 + 12. * param.s2)
                                            + param.s12
                                                * (75. * param.s1.powi(2)
                                                    + 86. * param.s1 * param.s2
                                                    + -99. * param.s2.powi(2)))
                                    + -30.
                                        * param.m1_2.powi(2)
                                        * (37. * param.s12.powi(4)
                                            + -48. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + -6.
                                                * param.s12.powi(2)
                                                * (13. * param.s1.powi(2)
                                                    + -74. * param.s1 * param.s2
                                                    + 13. * param.s2.powi(2))
                                            + 152.
                                                * param.s12
                                                * (param.s1.powi(3)
                                                    + -2. * param.s1.powi(2) * param.s2
                                                    + -2. * param.s1 * param.s2.powi(2)
                                                    + param.s2.powi(3))
                                            - (param.s1 - param.s2).powi(2)
                                                * (63. * param.s1.powi(2)
                                                    + 218. * param.s1 * param.s2
                                                    + 63. * param.s2.powi(2)))
                                    + 6. * param.m1_2
                                        * (24. * param.s12.powi(5)
                                            + 25.
                                                * param.s12.powi(4)
                                                * (10. * param.s1 + -3. * param.s2)
                                            + param.s12.powi(3)
                                                * (-740. * param.s1.powi(2)
                                                    + 628. * param.s1 * param.s2
                                                    + 60. * param.s2.powi(2))
                                            + 6. * param.s12.powi(2)
                                                * (80. * param.s1.powi(3)
                                                    + 221. * param.s1.powi(2) * param.s2
                                                    + -294. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + 4. * param.s12
                                                * (35. * param.s1.powi(4)
                                                    + -509. * param.s1.powi(3) * param.s2
                                                    + 328.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 161. * param.s1 * param.s2.powi(3)
                                                    + -15. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (154. * param.s1.powi(2)
                                                    + 305. * param.s1 * param.s2
                                                    + 21. * param.s2.powi(2))))
                            + param.m2_2.powi(3)
                                * param.s2
                                * (-7. * param.s12.powi(6)
                                    + 66. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (95. * param.s1.powi(3)
                                            + 119. * param.s1.powi(2) * param.s2
                                            + 119. * param.s1 * param.s2.powi(2)
                                            + 95. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (-345. * param.s1.powi(4)
                                            + 624. * param.s1.powi(3) * param.s2
                                            + -4590. * param.s1.powi(2) * param.s2.powi(2)
                                            + 624. * param.s1 * param.s2.powi(3)
                                            + -345. * param.s2.powi(4))
                                    + 2. * param.s12
                                        * (81. * param.s1.powi(5)
                                            + -599. * param.s1.powi(4) * param.s2
                                            + 1278. * param.s1.powi(3) * param.s2.powi(2)
                                            + 1278. * param.s1.powi(2) * param.s2.powi(3)
                                            + -599. * param.s1 * param.s2.powi(4)
                                            + 81. * param.s2.powi(5))
                                    + -6.
                                        * param.m1_2
                                        * (-4. * param.s1.powi(5)
                                            + 4. * param.s12.powi(5)
                                            + 77. * param.s1.powi(4) * param.s2
                                            + 712. * param.s1.powi(3) * param.s2.powi(2)
                                            + -258. * param.s1.powi(2) * param.s2.powi(3)
                                            + -508. * param.s1 * param.s2.powi(4)
                                            + -19. * param.s2.powi(5)
                                            + -5.
                                                * param.s12.powi(4)
                                                * (4. * param.s1 + 7. * param.s2)
                                            + 4. * param.s12.powi(3)
                                                * (10. * param.s1.powi(2)
                                                    + 7. * param.s1 * param.s2
                                                    + 25. * param.s2.powi(2))
                                            + -2.
                                                * param.s12.powi(2)
                                                * (20. * param.s1.powi(3)
                                                    + -63. * param.s1.powi(2) * param.s2
                                                    + 252. * param.s1 * param.s2.powi(2)
                                                    + 65. * param.s2.powi(3))
                                            + 4. * param.s12
                                                * (5. * param.s1.powi(4)
                                                    + -49. * param.s1.powi(3) * param.s2
                                                    + -77.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 251. * param.s1 * param.s2.powi(3)
                                                    + 20. * param.s2.powi(4)))
                                    - (param.s1 - param.s2).powi(2)
                                        * (31. * param.s1.powi(4)
                                            + -422. * param.s1.powi(3) * param.s2
                                            + -2658. * param.s1.powi(2) * param.s2.powi(2)
                                            + -422. * param.s1 * param.s2.powi(3)
                                            + 31. * param.s2.powi(4))
                                    - param.s12.powi(4)
                                        * (225. * param.s1.powi(2)
                                            + 452. * param.s1 * param.s2
                                            + 225. * param.s2.powi(2)))
                            + 3. * param.m2_2.powi(2)
                                * param.s2.powi(2)
                                * (5.
                                    * param.m1_2.powi(2)
                                    * (9. * param.s1.powi(4)
                                        + 9. * param.s12.powi(4)
                                        + 260. * param.s1.powi(3) * param.s2
                                        + 202. * param.s1.powi(2) * param.s2.powi(2)
                                        + -380. * param.s1 * param.s2.powi(3)
                                        + -91. * param.s2.powi(4)
                                        + param.s12.powi(3)
                                            * (-36. * param.s1 + 64. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 22. * param.s1 * param.s2
                                                + -41. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (9. * param.s1.powi(3)
                                                + 114. * param.s1.powi(2) * param.s2
                                                + -71. * param.s1 * param.s2.powi(2)
                                                + -66. * param.s2.powi(3)))
                                    + -2.
                                        * param.m1_2
                                        * (6. * param.s12.powi(5)
                                            + 15. * param.s12.powi(4) * (param.s1 + param.s2)
                                            + -8.
                                                * param.s12.powi(3)
                                                * (15. * param.s1.powi(2)
                                                    + -104. * param.s1 * param.s2
                                                    + 15. * param.s2.powi(2))
                                            + 42.
                                                * param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + -23. * param.s1.powi(2) * param.s2
                                                    + -23. * param.s1 * param.s2.powi(2)
                                                    + 5. * param.s2.powi(3))
                                            + (param.s1 - param.s2).powi(2)
                                                * (39. * param.s1.powi(3)
                                                    + 821. * param.s1.powi(2) * param.s2
                                                    + 821. * param.s1 * param.s2.powi(2)
                                                    + 39. * param.s2.powi(3))
                                            + -2.
                                                * param.s12
                                                * (75. * param.s1.powi(4)
                                                    + 312. * param.s1.powi(3) * param.s2
                                                    + -1534.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 312. * param.s1 * param.s2.powi(3)
                                                    + 75. * param.s2.powi(4)))
                                    + -3.
                                        * (param.s12.powi(6)
                                            + -2.
                                                * param.s12.powi(5)
                                                * (5. * param.s1 + param.s2)
                                            + param.s12.powi(4)
                                                * (20. * param.s1.powi(2)
                                                    + -34. * param.s1 * param.s2
                                                    + -5. * param.s2.powi(2))
                                            + 4. * param.s12.powi(3)
                                                * param.s2
                                                * (-83. * param.s1.powi(2)
                                                    + 31. * param.s1 * param.s2
                                                    + 5. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (17. * param.s1.powi(5)
                                                    + -109. * param.s1.powi(4) * param.s2
                                                    + -224.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + 334.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -25. * param.s1 * param.s2.powi(4)
                                                    + 7. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (35. * param.s1.powi(4)
                                                    + -680. * param.s1.powi(3) * param.s2
                                                    + 384.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 68. * param.s1 * param.s2.powi(3)
                                                    + 25. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (10. * param.s1.powi(3)
                                                    + 124. * param.s1.powi(2) * param.s2
                                                    + 29. * param.s1 * param.s2.powi(2)
                                                    + -3. * param.s2.powi(3)))))
                    + param.m0_2.powi(2)
                        * (param.s2.powi(3)
                            * (37. * param.s12.powi(6)
                                + 6. * param.s12.powi(5) * (47. * param.s1 + -23. * param.s2)
                                + 2. * (param.s1 - param.s2).powi(5)
                                    * (4. * param.s1 + param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (51. * param.s1.powi(2)
                                        + 80. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-990. * param.s1.powi(2)
                                        + 662. * param.s1 * param.s2
                                        + 180. * param.s2.powi(2))
                                + 20.
                                    * param.s12.powi(3)
                                    * (45. * param.s1.powi(3)
                                        + 68. * param.s1.powi(2) * param.s2
                                        + -103. * param.s1 * param.s2.powi(2)
                                        + -4. * param.s2.powi(3))
                                + -3.
                                    * param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + 664. * param.s1.powi(3) * param.s2
                                        + -378. * param.s1.powi(2) * param.s2.powi(2)
                                        + -336. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                + -20.
                                    * param.m1_2.powi(3)
                                    * (25. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (-75. * param.s1 + 99. * param.s2)
                                        + param.s12
                                            * (75. * param.s1.powi(2)
                                                + -88. * param.s1 * param.s2
                                                + -63. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (25. * param.s1 + 61. * param.s2))
                                + -6.
                                    * param.m1_2
                                    * (84. * param.s12.powi(5)
                                        + -95. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + -9.
                                            * (param.s1 - param.s2).powi(4)
                                            * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (65. * param.s1.powi(2)
                                                + 278. * param.s1 * param.s2
                                                + 65. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (105. * param.s1.powi(2)
                                                + -524. * param.s1 * param.s2
                                                + 105. * param.s2.powi(2))
                                        + 36.
                                            * param.s12.powi(2)
                                            * (10. * param.s1.powi(3)
                                                + -19. * param.s1.powi(2) * param.s2
                                                + -19. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3)))
                                + 15.
                                    * param.m1_2.powi(2)
                                    * (65. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (-160. * param.s1 + 92. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (15. * param.s1.powi(2)
                                                + 50. * param.s1 * param.s2
                                                + -53. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (10. * param.s1.powi(3)
                                                + -109. * param.s1.powi(2) * param.s2
                                                + 74. * param.s1 * param.s2.powi(2)
                                                + 25. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (35. * param.s1 + 61. * param.s2)))
                            + -3.
                                * param.m2_2
                                * param.s2.powi(2)
                                * (5.
                                    * param.m1_2.powi(2)
                                    * (9. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (9. * param.s1 + -41. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -28. * param.s1 * param.s2
                                                + -33. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 178. * param.s1 * param.s2
                                                + 157. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (9. * param.s1.powi(3)
                                                + 39. * param.s1.powi(2) * param.s2
                                                + -157. * param.s1 * param.s2.powi(2)
                                                + 33. * param.s2.powi(3)))
                                    + -2.
                                        * param.m1_2
                                        * (24. * param.s12.powi(5)
                                            + param.s12.powi(4)
                                                * (-75. * param.s1 + 250. * param.s2)
                                            + param.s12.powi(3)
                                                * (60. * param.s1.powi(2)
                                                    + 628. * param.s1 * param.s2
                                                    + -740. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(3)
                                                * (21. * param.s1.powi(2)
                                                    + 305. * param.s1 * param.s2
                                                    + 154. * param.s2.powi(2))
                                            + 6. * param.s12.powi(2)
                                                * (5. * param.s1.powi(3)
                                                    + -294. * param.s1.powi(2) * param.s2
                                                    + 221. * param.s1 * param.s2.powi(2)
                                                    + 80. * param.s2.powi(3))
                                            + -4.
                                                * param.s12
                                                * (15. * param.s1.powi(4)
                                                    + -161. * param.s1.powi(3) * param.s2
                                                    + -328.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + 509. * param.s1 * param.s2.powi(3)
                                                    + -35. * param.s2.powi(4)))
                                    + 3. * (2. * param.s12.powi(6)
                                        + 4. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + param.s12.powi(4)
                                            * (-35. * param.s1.powi(2)
                                                + 262. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 10. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 8. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + 33. * param.s1.powi(2) * param.s2
                                                + 33. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 12.
                                            * param.s12.powi(3)
                                            * (5. * param.s1.powi(3)
                                                + -23. * param.s1.powi(2) * param.s2
                                                + -23. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(2)
                                            * (10. * param.s1.powi(4)
                                                + 61. * param.s1.powi(3) * param.s2
                                                + -250. * param.s1.powi(2) * param.s2.powi(2)
                                                + 61. * param.s1 * param.s2.powi(3)
                                                + 10. * param.s2.powi(4))))
                            + 3. * param.m2_2.powi(2)
                                * param.s2
                                * (2.
                                    * param.m1_2
                                    * (6. * param.s12.powi(5)
                                        + -15.
                                            * param.s12.powi(4)
                                            * (2. * param.s1 + 5. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (30. * param.s1.powi(2)
                                                + 66. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        + -6.
                                            * param.s12.powi(2)
                                            * (10. * param.s1.powi(3)
                                                + -9. * param.s1.powi(2) * param.s2
                                                + 181. * param.s1 * param.s2.powi(2)
                                                + -70. * param.s2.powi(3))
                                        + param.s12
                                            * (30. * param.s1.powi(4)
                                                + -204. * param.s1.powi(3) * param.s2
                                                + 678. * param.s1.powi(2) * param.s2.powi(2)
                                                + 796. * param.s1 * param.s2.powi(3)
                                                + -540. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(3)
                                                + -81. * param.s1.powi(2) * param.s2
                                                + -586. * param.s1 * param.s2.powi(2)
                                                + -199. * param.s2.powi(3)))
                                    + -3.
                                        * (param.s12.powi(6)
                                            + -2.
                                                * param.s12.powi(5)
                                                * (param.s1 + 5. * param.s2)
                                            + 4. * param.s1
                                                * param.s12.powi(3)
                                                * (5. * param.s1.powi(2)
                                                    + 31. * param.s1 * param.s2
                                                    + -83. * param.s2.powi(2))
                                            + param.s12.powi(4)
                                                * (-5. * param.s1.powi(2)
                                                    + -34. * param.s1 * param.s2
                                                    + 20. * param.s2.powi(2))
                                            + 2. * param.s12
                                                * (7. * param.s1.powi(5)
                                                    + -25. * param.s1.powi(4) * param.s2
                                                    + 334.
                                                        * param.s1.powi(3)
                                                        * param.s2.powi(2)
                                                    + -224.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(3)
                                                    + -109. * param.s1 * param.s2.powi(4)
                                                    + 17. * param.s2.powi(5))
                                            - param.s12.powi(2)
                                                * (25. * param.s1.powi(4)
                                                    + 68. * param.s1.powi(3) * param.s2
                                                    + 384.
                                                        * param.s1.powi(2)
                                                        * param.s2.powi(2)
                                                    + -680. * param.s1 * param.s2.powi(3)
                                                    + 35. * param.s2.powi(4))
                                            - (param.s1 - param.s2).powi(3)
                                                * (3. * param.s1.powi(3)
                                                    + -29. * param.s1.powi(2) * param.s2
                                                    + -124. * param.s1 * param.s2.powi(2)
                                                    + -10. * param.s2.powi(3))))
                            - param.m2_2.powi(3)
                                * (10. * param.s12.powi(6)
                                    + -12. * param.s12.powi(5) * (5. * param.s1 + 7. * param.s2)
                                    + 5. * param.s12.powi(4)
                                        * (30. * param.s1.powi(2)
                                            + 46. * param.s1 * param.s2
                                            + 63. * param.s2.powi(2))
                                    + -4.
                                        * param.s12.powi(3)
                                        * (50. * param.s1.powi(3)
                                            + 20. * param.s1.powi(2) * param.s2
                                            + -43. * param.s1 * param.s2.powi(2)
                                            + 155. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(2)
                                        * (10. * param.s1.powi(4)
                                            + -86. * param.s1.powi(3) * param.s2
                                            + 441. * param.s1.powi(2) * param.s2.powi(2)
                                            + 1276. * param.s1 * param.s2.powi(3)
                                            + 79. * param.s2.powi(4))
                                    + 6. * param.s12.powi(2)
                                        * (25. * param.s1.powi(4)
                                            + -50. * param.s1.powi(3) * param.s2
                                            + -111. * param.s1.powi(2) * param.s2.powi(2)
                                            + 54. * param.s1 * param.s2.powi(3)
                                            + 110. * param.s2.powi(4))
                                    + -4.
                                        * param.s12
                                        * (15. * param.s1.powi(5)
                                            + -85. * param.s1.powi(4) * param.s2
                                            + 111. * param.s1.powi(3) * param.s2.powi(2)
                                            + -957. * param.s1.powi(2) * param.s2.powi(3)
                                            + 446. * param.s1 * param.s2.powi(4)
                                            + 90. * param.s2.powi(5))))
                    - param.m2_2.powi(3)
                        * param.s2.powi(2)
                        * (10. * param.s12.powi(6)
                            + -12. * param.s12.powi(5) * (7. * param.s1 + 5. * param.s2)
                            + 5. * param.s12.powi(4)
                                * (63. * param.s1.powi(2)
                                    + 46. * param.s1 * param.s2
                                    + 30. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (155. * param.s1.powi(3)
                                    + -43. * param.s1.powi(2) * param.s2
                                    + 20. * param.s1 * param.s2.powi(2)
                                    + 50. * param.s2.powi(3))
                            + (param.s1 - param.s2).powi(2)
                                * (79. * param.s1.powi(4)
                                    + 1276. * param.s1.powi(3) * param.s2
                                    + 441. * param.s1.powi(2) * param.s2.powi(2)
                                    + -86. * param.s1 * param.s2.powi(3)
                                    + 10. * param.s2.powi(4))
                            + 6. * param.s12.powi(2)
                                * (110. * param.s1.powi(4)
                                    + 54. * param.s1.powi(3) * param.s2
                                    + -111. * param.s1.powi(2) * param.s2.powi(2)
                                    + -50. * param.s1 * param.s2.powi(3)
                                    + 25. * param.s2.powi(4))
                            + -4.
                                * param.s12
                                * (90. * param.s1.powi(5)
                                    + 446. * param.s1.powi(4) * param.s2
                                    + -957. * param.s1.powi(3) * param.s2.powi(2)
                                    + 111. * param.s1.powi(2) * param.s2.powi(3)
                                    + -85. * param.s1 * param.s2.powi(4)
                                    + 15. * param.s2.powi(5))
                            + 15.
                                * param.m1_2.powi(2)
                                * (3. * param.s1.powi(4)
                                    + 3. * param.s12.powi(4)
                                    + 120. * param.s1.powi(3) * param.s2
                                    + 314. * param.s1.powi(2) * param.s2.powi(2)
                                    + 120. * param.s1 * param.s2.powi(3)
                                    + 3. * param.s2.powi(4)
                                    + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 18.
                                        * param.s12.powi(2)
                                        * (param.s1.powi(2)
                                            + 8. * param.s1 * param.s2
                                            + param.s2.powi(2))
                                    + -12.
                                        * param.s12
                                        * (param.s1.powi(3)
                                            + 21. * param.s1.powi(2) * param.s2
                                            + 21. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3)))
                            + 6. * param.m1_2
                                * (-19. * param.s1.powi(5)
                                    + 4. * param.s12.powi(5)
                                    + -508. * param.s1.powi(4) * param.s2
                                    + -258. * param.s1.powi(3) * param.s2.powi(2)
                                    + 712. * param.s1.powi(2) * param.s2.powi(3)
                                    + 77. * param.s1 * param.s2.powi(4)
                                    + -4. * param.s2.powi(5)
                                    + -5. * param.s12.powi(4) * (7. * param.s1 + 4. * param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (25. * param.s1.powi(2)
                                            + 7. * param.s1 * param.s2
                                            + 10. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (65. * param.s1.powi(3)
                                            + 252. * param.s1.powi(2) * param.s2
                                            + -63. * param.s1 * param.s2.powi(2)
                                            + 20. * param.s2.powi(3))
                                    + 4. * param.s12
                                        * (20. * param.s1.powi(4)
                                            + 251. * param.s1.powi(3) * param.s2
                                            + -77. * param.s1.powi(2) * param.s2.powi(2)
                                            + -49. * param.s1 * param.s2.powi(3)
                                            + 5. * param.s2.powi(4))))
                    - param.m2_2.powi(5)
                        * (param.s1.powi(6)
                            + param.s12.powi(6)
                            + -13. * param.s1.powi(5) * param.s2
                            + 113. * param.s1.powi(4) * param.s2.powi(2)
                            + 638. * param.s1.powi(3) * param.s2.powi(3)
                            + 113. * param.s1.powi(2) * param.s2.powi(4)
                            + -13. * param.s1 * param.s2.powi(5)
                            + param.s2.powi(6)
                            + -6. * param.s12.powi(5) * (param.s1 + param.s2)
                            + param.s12.powi(4)
                                * (15. * param.s1.powi(2)
                                    + 11. * param.s1 * param.s2
                                    + 15. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (5. * param.s1.powi(3)
                                    + -4. * param.s1.powi(2) * param.s2
                                    + -4. * param.s1 * param.s2.powi(2)
                                    + 5. * param.s2.powi(3))
                            + 3. * param.s12.powi(2)
                                * (5. * param.s1.powi(4)
                                    + -18. * param.s1.powi(3) * param.s2
                                    + 12. * param.s1.powi(2) * param.s2.powi(2)
                                    + -18. * param.s1 * param.s2.powi(3)
                                    + 5. * param.s2.powi(4))
                            + -2.
                                * param.s12
                                * (3. * param.s1.powi(5)
                                    + -23. * param.s1.powi(4) * param.s2
                                    + 90. * param.s1.powi(3) * param.s2.powi(2)
                                    + 90. * param.s1.powi(2) * param.s2.powi(3)
                                    + -23. * param.s1 * param.s2.powi(4)
                                    + 3. * param.s2.powi(5))))
                    * param.lambda_m02_sqrt
                    * param.lambda_s12_sqrt)
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.0006944444444444444
            * std::f64::consts::PI
            * param.s12.powi(-4)
            * param.lambda_s12_sqrt.powi(-9)
            * ((-3. * param.m2_2.powi(5) * param.s1.powi(7)
                + 28. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12
                + 15. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12
                + -126. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12.powi(2)
                + -152. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12.powi(2)
                + -30. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12.powi(2)
                + 420. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(3)
                + 846. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(3)
                + 388. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(3)
                + 30. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(3)
                + -105. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(4)
                + -810. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(4)
                + -669. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(4)
                + -92. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(4)
                + -15. * param.m2_2 * param.s1.powi(7) * param.s12.powi(4)
                + -252. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(5)
                + -495. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(5)
                + -120. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(5)
                + 51. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(5)
                + 88. * param.m2_2 * param.s1.powi(6) * param.s12.powi(5)
                + 3. * param.s1.powi(7) * param.s12.powi(5)
                + 42. * param.m2_2.powi(5) * param.s1 * param.s12.powi(6)
                + 720. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(6)
                + 960. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(6)
                + 120. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(6)
                + -219. * param.m2_2 * param.s1.powi(5) * param.s12.powi(6)
                + -20. * param.s1.powi(6) * param.s12.powi(6)
                + -4. * param.m2_2.powi(5) * param.s12.powi(7)
                + -138. * param.m2_2.powi(4) * param.s1 * param.s12.powi(7)
                + -660. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(7)
                + -200. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(7)
                + 300. * param.m2_2 * param.s1.powi(4) * param.s12.powi(7)
                + 57. * param.s1.powi(5) * param.s12.powi(7)
                + 14. * param.m2_2.powi(4) * param.s12.powi(8)
                + 147. * param.m2_2.powi(3) * param.s1 * param.s12.powi(8)
                + 120. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(8)
                + -245. * param.m2_2 * param.s1.powi(3) * param.s12.powi(8)
                + -90. * param.s1.powi(4) * param.s12.powi(8)
                + -16. * param.m2_2.powi(3) * param.s12.powi(9)
                + -33. * param.m2_2.powi(2) * param.s1 * param.s12.powi(9)
                + 120. * param.m2_2 * param.s1.powi(2) * param.s12.powi(9)
                + 85. * param.s1.powi(3) * param.s12.powi(9)
                + 4. * param.m2_2.powi(2) * param.s12.powi(10)
                + -33. * param.m2_2 * param.s1 * param.s12.powi(10)
                + -48. * param.s1.powi(2) * param.s12.powi(10)
                + 4. * param.m2_2 * param.s12.powi(11)
                + 15. * param.s1 * param.s12.powi(11)
                + -2. * param.s12.powi(12)
                + 21. * param.m2_2.powi(5) * param.s1.powi(6) * param.s2
                + -118. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12 * param.s2
                + -99. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12 * param.s2
                + 247. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + 578. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(2) * param.s2
                + 171. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(2) * param.s2
                + -60. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + -1217. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + -1042. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(3) * param.s2
                + -69. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(3) * param.s2
                + 473. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + 2016. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + 3448. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + 1418. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(4) * param.s2
                + 186. * param.m2_2 * param.s1.powi(6) * param.s12.powi(4) * param.s2
                + -170. * param.m2_2.powi(5) * param.s1 * param.s12.powi(5) * param.s2
                + -991. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(5) * param.s2
                + -2544. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + -2192. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(5) * param.s2
                + -532. * param.m2_2 * param.s1.powi(5) * param.s12.powi(5) * param.s2
                + -30. * param.s1.powi(6) * param.s12.powi(5) * param.s2
                + 27. * param.m2_2.powi(5) * param.s12.powi(6) * param.s2
                + 526. * param.m2_2.powi(4) * param.s1 * param.s12.powi(6) * param.s2
                + 359. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + 396. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(6) * param.s2
                + 403. * param.m2_2 * param.s1.powi(4) * param.s12.powi(6) * param.s2
                + 116. * param.s1.powi(5) * param.s12.powi(6) * param.s2
                + -93. * param.m2_2.powi(4) * param.s12.powi(7) * param.s2
                + -494. * param.m2_2.powi(3) * param.s1 * param.s12.powi(7) * param.s2
                + 439. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(7) * param.s2
                + 156. * param.m2_2 * param.s1.powi(3) * param.s12.powi(7) * param.s2
                + -149. * param.s1.powi(4) * param.s12.powi(7) * param.s2
                + 102. * param.m2_2.powi(3) * param.s12.powi(8) * param.s2
                + 26. * param.m2_2.powi(2) * param.s1 * param.s12.powi(8) * param.s2
                + -356. * param.m2_2 * param.s1.powi(2) * param.s12.powi(8) * param.s2
                + 36. * param.s1.powi(3) * param.s12.powi(8) * param.s2
                + -18. * param.m2_2.powi(2) * param.s12.powi(9) * param.s2
                + 176. * param.m2_2 * param.s1 * param.s12.powi(9) * param.s2
                + 76. * param.s1.powi(2) * param.s12.powi(9) * param.s2
                + -33. * param.m2_2 * param.s12.powi(10) * param.s2
                + -64. * param.s1 * param.s12.powi(10) * param.s2
                + 15. * param.s12.powi(11) * param.s2
                + -63. * param.m2_2.powi(5) * param.s1.powi(5) * param.s2.powi(2)
                + 170. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12 * param.s2.powi(2)
                + 279. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12 * param.s2.powi(2)
                + -54.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -724.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -411.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -144.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -162.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 656.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -51.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 239. * param.m2_2.powi(5) * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + -252.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -2292.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -2104.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -276. * param.m2_2 * param.s1.powi(5) * param.s12.powi(4) * param.s2.powi(2)
                + -78. * param.m2_2.powi(5) * param.s12.powi(5) * param.s2.powi(2)
                + -655. * param.m2_2.powi(4) * param.s1 * param.s12.powi(5) * param.s2.powi(2)
                + 948.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 3648.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 1706. * param.m2_2 * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(2)
                + 162. * param.s1.powi(5) * param.s12.powi(5) * param.s2.powi(2)
                + 264. * param.m2_2.powi(4) * param.s12.powi(6) * param.s2.powi(2)
                + 455. * param.m2_2.powi(3) * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + -492.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + -1122. * param.m2_2 * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(2)
                + -244. * param.s1.powi(4) * param.s12.powi(6) * param.s2.powi(2)
                + -276. * param.m2_2.powi(3) * param.s12.powi(7) * param.s2.powi(2)
                + 175. * param.m2_2.powi(2) * param.s1 * param.s12.powi(7) * param.s2.powi(2)
                + -132. * param.m2_2 * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(2)
                + -18. * param.s1.powi(3) * param.s12.powi(7) * param.s2.powi(2)
                + 24. * param.m2_2.powi(2) * param.s12.powi(8) * param.s2.powi(2)
                + -290. * param.m2_2 * param.s1 * param.s12.powi(8) * param.s2.powi(2)
                + 72. * param.s1.powi(2) * param.s12.powi(8) * param.s2.powi(2)
                + 114. * param.m2_2 * param.s12.powi(9) * param.s2.powi(2)
                + 76. * param.s1 * param.s12.powi(9) * param.s2.powi(2)
                + -48. * param.s12.powi(10) * param.s2.powi(2)
                + 105. * param.m2_2.powi(5) * param.s1.powi(4) * param.s2.powi(3)
                + -60. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12 * param.s2.powi(3)
                + -435. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12 * param.s2.powi(3)
                + -60.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 156.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 540.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -96. * param.m2_2.powi(5) * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + 372.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 456.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 300.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 125. * param.m2_2.powi(5) * param.s12.powi(4) * param.s2.powi(3)
                + 120. * param.m2_2.powi(4) * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + -138.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 356.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + -195. * param.m2_2 * param.s1.powi(4) * param.s12.powi(4) * param.s2.powi(3)
                + -415. * param.m2_2.powi(4) * param.s12.powi(5) * param.s2.powi(3)
                + 180. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + -618.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + -1324. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(3)
                + -135. * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(3)
                + 410. * param.m2_2.powi(3) * param.s12.powi(6) * param.s2.powi(3)
                + -300. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6) * param.s2.powi(3)
                + 462. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(3)
                + 416. * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(3)
                + 10. * param.m2_2.powi(2) * param.s12.powi(7) * param.s2.powi(3)
                + 60. * param.m2_2 * param.s1 * param.s12.powi(7) * param.s2.powi(3)
                + -18. * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(3)
                + -215. * param.m2_2 * param.s12.powi(8) * param.s2.powi(3)
                + 36. * param.s1 * param.s12.powi(8) * param.s2.powi(3)
                + 85. * param.s12.powi(9) * param.s2.powi(3)
                + -105. * param.m2_2.powi(5) * param.s1.powi(3) * param.s2.powi(4)
                + -80. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12 * param.s2.powi(4)
                + 405. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12 * param.s2.powi(4)
                + -76. * param.m2_2.powi(5) * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + 376.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -420.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -120. * param.m2_2.powi(5) * param.s12.powi(3) * param.s2.powi(4)
                + 380. * param.m2_2.powi(4) * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + -704.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -360.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + 390. * param.m2_2.powi(4) * param.s12.powi(4) * param.s2.powi(4)
                + -535. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + 356.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + 615. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(4)
                + -360. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2.powi(4)
                + 85. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2.powi(4)
                + 296. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(4)
                + -135. * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(4)
                + -60. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2.powi(4)
                + 295. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2.powi(4)
                + -244. * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(4)
                + 240. * param.m2_2 * param.s12.powi(7) * param.s2.powi(4)
                + -149. * param.s1 * param.s12.powi(7) * param.s2.powi(4)
                + -90. * param.s12.powi(8) * param.s2.powi(4)
                + 63. * param.m2_2.powi(5) * param.s1.powi(2) * param.s2.powi(5)
                + 82. * param.m2_2.powi(5) * param.s1 * param.s12 * param.s2.powi(5)
                + -225. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12 * param.s2.powi(5)
                + 69. * param.m2_2.powi(5) * param.s12.powi(2) * param.s2.powi(5)
                + -302. * param.m2_2.powi(4) * param.s1 * param.s12.powi(2) * param.s2.powi(5)
                + 195.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + -219. * param.m2_2.powi(4) * param.s12.powi(3) * param.s2.powi(5)
                + 298. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(5)
                + 195.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + 186. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(5)
                + 98. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(5)
                + -390. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(5)
                + 66. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(5)
                + -292. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(5)
                + 162. * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(5)
                + -159. * param.m2_2 * param.s12.powi(6) * param.s2.powi(5)
                + 116. * param.s1 * param.s12.powi(6) * param.s2.powi(5)
                + 57. * param.s12.powi(7) * param.s2.powi(5)
                + -21. * param.m2_2.powi(5) * param.s1 * param.s2.powi(6)
                + -22. * param.m2_2.powi(5) * param.s12 * param.s2.powi(6)
                + 69. * param.m2_2.powi(4) * param.s1 * param.s12 * param.s2.powi(6)
                + 68. * param.m2_2.powi(4) * param.s12.powi(2) * param.s2.powi(6)
                + -51. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(6)
                + -52. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(6)
                + -51. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(6)
                + -32. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(6)
                + 84. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(6)
                + 58. * param.m2_2 * param.s12.powi(5) * param.s2.powi(6)
                + -30. * param.s1 * param.s12.powi(5) * param.s2.powi(6)
                + -20. * param.s12.powi(6) * param.s2.powi(6)
                + 3. * param.m2_2.powi(5) * param.s2.powi(7)
                + -9. * param.m2_2.powi(4) * param.s12 * param.s2.powi(7)
                + 6. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(7)
                + 6. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(7)
                + -9. * param.m2_2 * param.s12.powi(4) * param.s2.powi(7)
                + 3. * param.s12.powi(5) * param.s2.powi(7)
                + 60.
                    * param.m0_2.powi(5)
                    * param.s12.powi(5)
                    * (4. * param.s12.powi(2) + -3. * (param.s1 - param.s2).powi(2)
                        - param.s12 * (param.s1 + param.s2))
                + -30.
                    * param.m0_2.powi(4)
                    * param.s12.powi(4)
                    * (param.m1_2
                        * (20. * param.s12.powi(3)
                            + 9. * (param.s1 - param.s2).powi(3)
                            + param.s12.powi(2) * (-31. * param.s1 + 21. * param.s2)
                            + 2. * param.s12
                                * (param.s1.powi(2)
                                    + 15. * param.s1 * param.s2
                                    + -16. * param.s2.powi(2)))
                        + param.m2_2
                            * (20. * param.s12.powi(3)
                                + param.s12.powi(2) * (21. * param.s1 + -31. * param.s2)
                                + -9. * (param.s1 - param.s2).powi(3)
                                + param.s12
                                    * (-32. * param.s1.powi(2)
                                        + 30. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2)))
                        + param.s12
                            * (-14. * param.s12.powi(3)
                                + 13. * param.s12.powi(2) * (param.s1 + param.s2)
                                + -15. * (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                                + 4. * param.s12
                                    * (4. * param.s1.powi(2)
                                        + -13. * param.s1 * param.s2
                                        + 4. * param.s2.powi(2))))
                + param.m1_2.powi(4)
                    * (param.s12
                        * (14. * param.s12.powi(7)
                            + -3.
                                * (3. * param.s1 + -5. * param.s2)
                                * (param.s1 - param.s2).powi(6)
                            + -3. * param.s12.powi(6) * (31. * param.s1 + 46. * param.s2)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (34. * param.s1.powi(2)
                                    + -15. * param.s1 * param.s2
                                    + -76. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (264. * param.s1.powi(2)
                                    + 526. * param.s1 * param.s2
                                    + 720. * param.s2.powi(2))
                            + 6. * param.s12.powi(3)
                                * (65. * param.s1.powi(4)
                                    + 20. * param.s1.powi(3) * param.s2
                                    + -42. * param.s1.powi(2) * param.s2.powi(2)
                                    + 336. * param.s1 * param.s2.powi(3)
                                    + -135. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (415. * param.s1.powi(3)
                                    + 655. * param.s1.powi(2) * param.s2
                                    + 991. * param.s1 * param.s2.powi(2)
                                    + 495. * param.s2.powi(3))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (219. * param.s1.powi(3)
                                    + 58. * param.s1.powi(2) * param.s2
                                    + -475. * param.s1 * param.s2.powi(2)
                                    + -846. * param.s2.powi(3)))
                        + param.m2_2
                            * (32. * param.s12.powi(7)
                                + -15. * (param.s1 - param.s2).powi(7)
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (58. * param.s1 + 67. * param.s2)
                                + -3. * param.s12.powi(6) * (67. * param.s1 + 142. * param.s2)
                                + 10.
                                    * param.s12.powi(5)
                                    * (54. * param.s1.powi(2)
                                        + 151. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + 12.
                                    * param.s12.powi(3)
                                    * (60. * param.s1.powi(4)
                                        + 52. * param.s1.powi(3) * param.s2
                                        + 51. * param.s1.powi(2) * param.s2.powi(2)
                                        + -33. * param.s1 * param.s2.powi(3)
                                        + -130. * param.s2.powi(4))
                                - param.s12.powi(4)
                                    * (805. * param.s1.powi(3)
                                        + 1843. * param.s1.powi(2) * param.s2
                                        + 1093. * param.s1 * param.s2.powi(2)
                                        + -1545. * param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (387. * param.s1.powi(2)
                                        + 685. * param.s1 * param.s2
                                        + 558. * param.s2.powi(2))))
                + param.m1_2.powi(2)
                    * (3.
                        * param.m2_2.powi(2)
                        * param.s12
                        * (12. * param.s12.powi(7)
                            + 7. * param.s12.powi(6) * (param.s1 + param.s2)
                            + 6. * (param.s1 - param.s2).powi(6) * (param.s1 + param.s2)
                            + -4.
                                * param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (13. * param.s1.powi(2)
                                    + 31. * param.s1 * param.s2
                                    + 13. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(5)
                                * (44. * param.s1.powi(2)
                                    + -273. * param.s1 * param.s2
                                    + 44. * param.s2.powi(2))
                            + 29.
                                * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (7. * param.s1.powi(3)
                                    + 29. * param.s1.powi(2) * param.s2
                                    + 29. * param.s1 * param.s2.powi(2)
                                    + 7. * param.s2.powi(3))
                            + 4. * param.s12.powi(4)
                                * (100. * param.s1.powi(3)
                                    + -423. * param.s1.powi(2) * param.s2
                                    + -423. * param.s1 * param.s2.powi(2)
                                    + 100. * param.s2.powi(3))
                            + -8.
                                * param.s12.powi(3)
                                * (50. * param.s1.powi(4)
                                    + -13. * param.s1.powi(3) * param.s2
                                    + -440. * param.s1.powi(2) * param.s2.powi(2)
                                    + -13. * param.s1 * param.s2.powi(3)
                                    + 50. * param.s2.powi(4)))
                        + param.s12.powi(3)
                            * (4. * param.s12.powi(7)
                                + -3. * param.s12.powi(6) * (6. * param.s1 + 11. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (12. * param.s1.powi(2)
                                        + 13. * param.s1 * param.s2
                                        + 60. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (10. * param.s1.powi(3)
                                        + 175. * param.s1.powi(2) * param.s2
                                        + 439. * param.s1 * param.s2.powi(2)
                                        + -200. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (2. * param.s1.powi(3)
                                        + -9. * param.s1.powi(2) * param.s2
                                        + 17. * param.s1 * param.s2.powi(2)
                                        + 10. * param.s2.powi(3))
                                + -12.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(4)
                                        + 25. * param.s1.powi(3) * param.s2
                                        + 41. * param.s1.powi(2) * param.s2.powi(2)
                                        + -33. * param.s1 * param.s2.powi(3)
                                        + -10. * param.s2.powi(4))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (16. * param.s1.powi(4)
                                        + -17. * param.s1.powi(3) * param.s2
                                        + -228. * param.s1.powi(2) * param.s2.powi(2)
                                        + -617. * param.s1 * param.s2.powi(3)
                                        + 46. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (66. * param.s1.powi(5)
                                        + 85. * param.s1.powi(4) * param.s2
                                        + -618. * param.s1.powi(3) * param.s2.powi(2)
                                        + 3648. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2192. * param.s1 * param.s2.powi(4)
                                        + 51. * param.s2.powi(5)))
                        + 3. * param.m2_2
                            * param.s12.powi(2)
                            * (4. * param.s12.powi(7)
                                + -5. * param.s12.powi(6) * (2. * param.s1 + 7. * param.s2)
                                + -2.
                                    * param.s12.powi(5)
                                    * (6. * param.s1.powi(2)
                                        + 61. * param.s1 * param.s2
                                        + -32. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(5)
                                    * (2. * param.s1.powi(2)
                                        + -11. * param.s1 * param.s2
                                        + -6. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (10. * param.s1.powi(3)
                                        + -49. * param.s1.powi(2) * param.s2
                                        + -141. * param.s1 * param.s2.powi(2)
                                        + -30. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (70. * param.s1.powi(3)
                                        + 481. * param.s1.powi(2) * param.s2
                                        + -1035. * param.s1 * param.s2.powi(2)
                                        + 20. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (25. * param.s1.powi(4)
                                        + 85. * param.s1.powi(3) * param.s2
                                        + 276. * param.s1.powi(2) * param.s2.powi(2)
                                        + -584. * param.s1 * param.s2.powi(3)
                                        + 40. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (66. * param.s1.powi(5)
                                        + -121. * param.s1.powi(4) * param.s2
                                        + 2078. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1136. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1048. * param.s1 * param.s2.powi(4)
                                        + 161. * param.s2.powi(5)))
                        - param.m2_2.powi(3)
                            * (148. * param.s12.powi(7)
                                + 3. * param.s12.powi(6) * (57. * param.s1 + -263. * param.s2)
                                + 30. * (param.s1 - param.s2).powi(7)
                                + -4.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (64. * param.s1 + 61. * param.s2)
                                + -40.
                                    * param.s12.powi(5)
                                    * (45. * param.s1.powi(2)
                                        + -38. * param.s1 * param.s2
                                        + -45. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (987. * param.s1.powi(2)
                                        + 1400. * param.s1 * param.s2
                                        + 873. * param.s2.powi(2))
                                + 4. * param.s12.powi(4)
                                    * (765. * param.s1.powi(3)
                                        + 352. * param.s1.powi(2) * param.s2
                                        + -908. * param.s1 * param.s2.powi(2)
                                        + -575. * param.s2.powi(3))
                                + -36.
                                    * param.s12.powi(3)
                                    * (65. * param.s1.powi(4)
                                        + 39. * param.s1.powi(3) * param.s2
                                        + -13. * param.s1.powi(2) * param.s2.powi(2)
                                        + -41. * param.s1 * param.s2.powi(3)
                                        + -50. * param.s2.powi(4))))
                + param.m1_2
                    * (-4.
                        * param.m2_2.powi(3)
                        * param.s12
                        * (13. * param.s12.powi(7)
                            + 3. * (param.s1 - param.s2).powi(6) * (3. * param.s1 - param.s2)
                            + -75. * param.s12.powi(6) * (2. * param.s1 + param.s2)
                            + param.s12.powi(5)
                                * (171. * param.s1.powi(2)
                                    + 386. * param.s1 * param.s2
                                    + 183. * param.s2.powi(2))
                            + param.s12.powi(4)
                                * (315. * param.s1.powi(3)
                                    + -1493. * param.s1.powi(2) * param.s2
                                    + -185. * param.s1 * param.s2.powi(2)
                                    + -245. * param.s2.powi(3))
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (402. * param.s1.powi(3)
                                    + 557. * param.s1.powi(2) * param.s2
                                    + 178. * param.s1 * param.s2.powi(2)
                                    + -93. * param.s2.powi(3))
                            + -3.
                                * param.s12.powi(3)
                                * (225. * param.s1.powi(4)
                                    + -400. * param.s1.powi(3) * param.s2
                                    + -344. * param.s1.powi(2) * param.s2.powi(2)
                                    + 96. * param.s1 * param.s2.powi(3)
                                    + -65. * param.s2.powi(4))
                            - param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (85. * param.s1.powi(2)
                                    + 54. * param.s1 * param.s2
                                    + -25. * param.s2.powi(2)))
                        + param.m2_2.powi(4)
                            * (32. * param.s12.powi(7)
                                + 15. * (param.s1 - param.s2).powi(7)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (67. * param.s1 + 58. * param.s2)
                                + -3. * param.s12.powi(6) * (142. * param.s1 + 67. * param.s2)
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (558. * param.s1.powi(2)
                                        + 685. * param.s1 * param.s2
                                        + 387. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-30. * param.s1.powi(2)
                                        + 1510. * param.s1 * param.s2
                                        + 540. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (1545. * param.s1.powi(3)
                                        + -1093. * param.s1.powi(2) * param.s2
                                        + -1843. * param.s1 * param.s2.powi(2)
                                        + -805. * param.s2.powi(3))
                                + -12.
                                    * param.s12.powi(3)
                                    * (130. * param.s1.powi(4)
                                        + 33. * param.s1.powi(3) * param.s2
                                        + -51. * param.s1.powi(2) * param.s2.powi(2)
                                        + -52. * param.s1 * param.s2.powi(3)
                                        + -60. * param.s2.powi(4)))
                        + 4. * param.m2_2
                            * param.s12.powi(3)
                            * (param.s12.powi(7)
                                + -9. * param.s12.powi(6) * (param.s1 + param.s2)
                                + param.s12.powi(5)
                                    * (27. * param.s1.powi(2)
                                        + 56. * param.s1 * param.s2
                                        + 27. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-35. * param.s1.powi(3)
                                        + 4. * param.s1.powi(2) * param.s2
                                        + 4. * param.s1 * param.s2.powi(2)
                                        + -35. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (param.s1.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 3. * param.s12.powi(3)
                                    * (5. * param.s1.powi(4)
                                        + -76. * param.s1.powi(3) * param.s2
                                        + 330. * param.s1.powi(2) * param.s2.powi(2)
                                        + -76. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (9. * param.s1.powi(5)
                                        + 235. * param.s1.powi(4) * param.s2
                                        + -504. * param.s1.powi(3) * param.s2.powi(2)
                                        + -504. * param.s1.powi(2) * param.s2.powi(3)
                                        + 235. * param.s1 * param.s2.powi(4)
                                        + 9. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (11. * param.s1.powi(4)
                                        + 50. * param.s1.powi(3) * param.s2
                                        + 678. * param.s1.powi(2) * param.s2.powi(2)
                                        + 50. * param.s1 * param.s2.powi(3)
                                        + 11. * param.s2.powi(4)))
                        + 3. * param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (4. * param.s12.powi(7)
                                + -5. * param.s12.powi(6) * (7. * param.s1 + 2. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (32. * param.s1.powi(2)
                                        + -61. * param.s1 * param.s2
                                        + -6. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(5)
                                    * (6. * param.s1.powi(2)
                                        + 11. * param.s1 * param.s2
                                        + -2. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (30. * param.s1.powi(3)
                                        + 141. * param.s1.powi(2) * param.s2
                                        + 49. * param.s1 * param.s2.powi(2)
                                        + -10. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (20. * param.s1.powi(3)
                                        + -1035. * param.s1.powi(2) * param.s2
                                        + 481. * param.s1 * param.s2.powi(2)
                                        + 70. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (40. * param.s1.powi(4)
                                        + -584. * param.s1.powi(3) * param.s2
                                        + 276. * param.s1.powi(2) * param.s2.powi(2)
                                        + 85. * param.s1 * param.s2.powi(3)
                                        + 25. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (161. * param.s1.powi(5)
                                        + -1048. * param.s1.powi(4) * param.s2
                                        + -1136. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2078. * param.s1.powi(2) * param.s2.powi(3)
                                        + -121. * param.s1 * param.s2.powi(4)
                                        + 66. * param.s2.powi(5)))
                        + param.s12.powi(4)
                            * (4. * param.s12.powi(7)
                                + -33. * param.s12.powi(6) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(5)
                                    * (57. * param.s1.powi(2)
                                        + 88. * param.s1 * param.s2
                                        + 60. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(3)
                                    * (3. * param.s1.powi(4)
                                        + -19. * param.s1.powi(3) * param.s2
                                        + 64. * param.s1.powi(2) * param.s2.powi(2)
                                        + 47. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                + 12.
                                    * param.s12.powi(3)
                                    * (20. * param.s1.powi(4)
                                        + 5. * param.s1.powi(3) * param.s2
                                        + -11. * param.s1.powi(2) * param.s2.powi(2)
                                        + 13. * param.s1 * param.s2.powi(3)
                                        + 25. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-159. * param.s1.powi(5)
                                        + 295. * param.s1.powi(4) * param.s2
                                        + 462. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1122. * param.s1.powi(2) * param.s2.powi(3)
                                        + 403. * param.s1 * param.s2.powi(4)
                                        + -219. * param.s2.powi(5))
                                + 2. * param.s12
                                    * (29. * param.s1.powi(6)
                                        + -146. * param.s1.powi(5) * param.s2
                                        + 148. * param.s1.powi(4) * param.s2.powi(2)
                                        + -662. * param.s1.powi(3) * param.s2.powi(3)
                                        + 853. * param.s1.powi(2) * param.s2.powi(4)
                                        + -266. * param.s1 * param.s2.powi(5)
                                        + 44. * param.s2.powi(6))
                                - param.s12.powi(4)
                                    * (215. * param.s1.powi(3)
                                        + 290. * param.s1.powi(2) * param.s2
                                        + 356. * param.s1 * param.s2.powi(2)
                                        + 245. * param.s2.powi(3))))
                + 10.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (2.
                        * param.m1_2.powi(2)
                        * (22. * param.s12.powi(4)
                            + -3. * (param.s1 - param.s2).powi(4)
                            + param.s12.powi(3) * (-63. * param.s1 + 93. * param.s2)
                            + param.s12.powi(2)
                                * (57. * param.s1.powi(2)
                                    + -64. * param.s1 * param.s2
                                    + -45. * param.s2.powi(2))
                            - param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (13. * param.s1 + 67. * param.s2))
                        + 2. * param.m2_2.powi(2)
                            * (22. * param.s12.powi(4)
                                + param.s12.powi(3) * (93. * param.s1 + -63. * param.s2)
                                + -3. * (param.s1 - param.s2).powi(4)
                                + param.s12.powi(2)
                                    * (-45. * param.s1.powi(2)
                                        + -64. * param.s1 * param.s2
                                        + 57. * param.s2.powi(2))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (67. * param.s1 + 13. * param.s2))
                        + param.s12.powi(2)
                            * (17. * param.s12.powi(4)
                                + -18. * param.s12.powi(3) * (param.s1 + param.s2)
                                + -16.
                                    * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + -13. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(2)
                                    * (11. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 11. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (41. * param.s1.powi(3)
                                        + -71. * param.s1.powi(2) * param.s2
                                        + -71. * param.s1 * param.s2.powi(2)
                                        + 41. * param.s2.powi(3)))
                        + param.m1_2
                            * (4.
                                * param.m2_2
                                * (38. * param.s12.powi(4)
                                    + 3. * (param.s1 - param.s2).powi(4)
                                    + -30. * param.s12.powi(3) * (param.s1 + param.s2)
                                    + 40.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + param.s2)
                                    + param.s12.powi(2)
                                        * (-51. * param.s1.powi(2)
                                            + 154. * param.s1 * param.s2
                                            + -51. * param.s2.powi(2)))
                                - param.s12
                                    * (61. * param.s12.powi(4)
                                        + -48.
                                            * param.s12.powi(3)
                                            * (3. * param.s1 + -2. * param.s2)
                                        + -3.
                                            * (param.s1 - param.s2).powi(3)
                                            * (13. * param.s1 + 23. * param.s2)
                                        + param.s12.powi(2)
                                            * (66. * param.s1.powi(2)
                                                + 296. * param.s1 * param.s2
                                                + -306. * param.s2.powi(2))
                                        + 8. * param.s12
                                            * (7. * param.s1.powi(3)
                                                + -55. * param.s1.powi(2) * param.s2
                                                + 38. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))))
                        - param.m2_2
                            * param.s12
                            * (61. * param.s12.powi(4)
                                + 48. * param.s12.powi(3) * (2. * param.s1 + -3. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(3)
                                    * (23. * param.s1 + 13. * param.s2)
                                + param.s12.powi(2)
                                    * (-306. * param.s1.powi(2)
                                        + 296. * param.s1 * param.s2
                                        + 66. * param.s2.powi(2))
                                + 8. * param.s12
                                    * (10. * param.s1.powi(3)
                                        + 38. * param.s1.powi(2) * param.s2
                                        + -55. * param.s1 * param.s2.powi(2)
                                        + 7. * param.s2.powi(3))))
                + -15.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m1_2.powi(3)
                        * (4. * param.s12.powi(5)
                            + 4. * param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (2. * param.s1 + 5. * param.s2)
                            + param.s12.powi(4) * (-17. * param.s1 + 97. * param.s2)
                            + 4. * param.s12.powi(3)
                                * (7. * param.s1.powi(2)
                                    + -47. * param.s1 * param.s2
                                    + 14. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-22. * param.s1.powi(3)
                                    + 90. * param.s1.powi(2) * param.s2
                                    + 70. * param.s1 * param.s2.powi(2)
                                    + -138. * param.s2.powi(3))
                            - (param.s1 - param.s2).powi(5))
                        + param.m2_2.powi(3)
                            * (4. * param.s12.powi(5)
                                + param.s12.powi(4) * (97. * param.s1 + -17. * param.s2)
                                + (param.s1 - param.s2).powi(5)
                                + -4.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1 + 2. * param.s2)
                                + 4. * param.s12.powi(3)
                                    * (14. * param.s1.powi(2)
                                        + -47. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-138. * param.s1.powi(3)
                                        + 70. * param.s1.powi(2) * param.s2
                                        + 90. * param.s1 * param.s2.powi(2)
                                        + -22. * param.s2.powi(3)))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (4. * param.s12.powi(5)
                                + param.s12.powi(4) * (49. * param.s1 + -11. * param.s2)
                                + 4. * param.s12.powi(3)
                                    * (-34. * param.s1.powi(2)
                                        + 31. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (37. * param.s1.powi(3)
                                        + 131. * param.s1.powi(2) * param.s2
                                        + -171. * param.s1 * param.s2.powi(2)
                                        + 7. * param.s2.powi(3))
                                + 4. * param.s12
                                    * (11. * param.s1.powi(4)
                                        + -103. * param.s1.powi(3) * param.s2
                                        + 67. * param.s1.powi(2) * param.s2.powi(2)
                                        + 29. * param.s1 * param.s2.powi(3)
                                        + -4. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(3)
                                    * (35. * param.s1.powi(2)
                                        + 68. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2)))
                        + param.s12.powi(3)
                            * (-3. * param.s12.powi(4) * (param.s1 + param.s2)
                                + 12.
                                    * param.s12.powi(3)
                                    * (param.s1.powi(2)
                                        + -4. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -18.
                                    * param.s12.powi(2)
                                    * (param.s1.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + -3. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + -3.
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + 19. * param.s1.powi(2) * param.s2
                                        + 19. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 4. * param.s12
                                    * (3. * param.s1.powi(4)
                                        + 12. * param.s1.powi(3) * param.s2
                                        + -50. * param.s1.powi(2) * param.s2.powi(2)
                                        + 12. * param.s1 * param.s2.powi(3)
                                        + 3. * param.s2.powi(4)))
                        + param.m1_2.powi(2)
                            * (param.m2_2
                                * (76. * param.s12.powi(5)
                                    + 3. * (param.s1 - param.s2).powi(5)
                                    + -12.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (3. * param.s1 + 4. * param.s2)
                                    + param.s12.powi(4) * (-201. * param.s1 + 81. * param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (36. * param.s1.powi(2)
                                            + 77. * param.s1 * param.s2
                                            + -87. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (7. * param.s1.powi(3)
                                            + -217. * param.s1.powi(2) * param.s2
                                            + 137. * param.s1 * param.s2.powi(2)
                                            + 73. * param.s2.powi(3)))
                                - param.s12
                                    * (8. * param.s12.powi(5)
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 11. * param.s2)
                                        + param.s12.powi(4)
                                            * (-31. * param.s1 + 143. * param.s2)
                                        + 4. * param.s12.powi(3)
                                            * (11. * param.s1.powi(2)
                                                + -28. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 42. * param.s1 * param.s2
                                                + 37. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (13. * param.s1.powi(3)
                                                + 99. * param.s1.powi(2) * param.s2
                                                + -301. * param.s1 * param.s2.powi(2)
                                                + 85. * param.s2.powi(3))))
                        + param.m1_2
                            * (param.m2_2.powi(2)
                                * (76. * param.s12.powi(5)
                                    + 3. * param.s12.powi(4)
                                        * (27. * param.s1 + -67. * param.s2)
                                    + -3. * (param.s1 - param.s2).powi(5)
                                    + 12.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (4. * param.s1 + 3. * param.s2)
                                    + -4.
                                        * param.s12.powi(3)
                                        * (87. * param.s1.powi(2)
                                            + -77. * param.s1 * param.s2
                                            + -36. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (73. * param.s1.powi(3)
                                            + 137. * param.s1.powi(2) * param.s2
                                            + -217. * param.s1 * param.s2.powi(2)
                                            + 7. * param.s2.powi(3)))
                                + -4.
                                    * param.m2_2
                                    * param.s12
                                    * (15. * param.s12.powi(5)
                                        + -13. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + -3.
                                            * (param.s1 - param.s2).powi(4)
                                            * (param.s1 + param.s2)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (12. * param.s1.powi(2)
                                                + -49. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + 4. * param.s12.powi(2)
                                            * (18. * param.s1.powi(3)
                                                + -31. * param.s1.powi(2) * param.s2
                                                + -31. * param.s1 * param.s2.powi(2)
                                                + 18. * param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (23. * param.s1.powi(2)
                                                + 114. * param.s1 * param.s2
                                                + 23. * param.s2.powi(2)))
                                + param.s12.powi(2)
                                    * (4. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-11. * param.s1 + 49. * param.s2)
                                        + 4. * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 31. * param.s1 * param.s2
                                                + -34. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 68. * param.s1 * param.s2
                                                + 35. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (7. * param.s1.powi(3)
                                                + -171. * param.s1.powi(2) * param.s2
                                                + 131. * param.s1 * param.s2.powi(2)
                                                + 37. * param.s2.powi(3))
                                        + -4.
                                            * param.s12
                                            * (4. * param.s1.powi(4)
                                                + -29. * param.s1.powi(3) * param.s2
                                                + -67. * param.s1.powi(2) * param.s2.powi(2)
                                                + 103. * param.s1 * param.s2.powi(3)
                                                + -11. * param.s2.powi(4))))
                        - param.m2_2.powi(2)
                            * param.s12
                            * (8. * param.s12.powi(5)
                                + param.s12.powi(4) * (143. * param.s1 + -31. * param.s2)
                                + (param.s1 - param.s2).powi(4) * (11. * param.s1 + param.s2)
                                + -4.
                                    * param.s12.powi(3)
                                    * (35. * param.s1.powi(2)
                                        + 28. * param.s1 * param.s2
                                        + -11. * param.s2.powi(2))
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (37. * param.s1.powi(2)
                                        + 42. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (85. * param.s1.powi(3)
                                        + -301. * param.s1.powi(2) * param.s2
                                        + 99. * param.s1 * param.s2.powi(2)
                                        + 13. * param.s2.powi(3))))
                + -3.
                    * param.m0_2
                    * param.s12
                    * (2.
                        * param.m1_2.powi(4)
                        * (2. * param.s12.powi(6)
                            + (param.s1 - param.s2).powi(6)
                            + 5. * param.s12.powi(4)
                                * (5. * param.s1.powi(2)
                                    + 22. * param.s1 * param.s2
                                    + -43. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (10. * param.s1.powi(2)
                                    + 32. * param.s1 * param.s2
                                    + 45. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(3)
                                * (15. * param.s1.powi(3)
                                    + 54. * param.s1.powi(2) * param.s2
                                    + -106. * param.s1 * param.s2.powi(2)
                                    + -85. * param.s2.powi(3))
                            - param.s12.powi(5) * (11. * param.s1 + 36. * param.s2)
                            - param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (7. * param.s1 + 12. * param.s2))
                        + 2. * param.m2_2.powi(4)
                            * (2. * param.s12.powi(6)
                                + (param.s1 - param.s2).powi(6)
                                + -5.
                                    * param.s12.powi(4)
                                    * (43. * param.s1.powi(2)
                                        + -22. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (45. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + 10. * param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (85. * param.s1.powi(3)
                                        + 106. * param.s1.powi(2) * param.s2
                                        + -54. * param.s1 * param.s2.powi(2)
                                        + -15. * param.s2.powi(3))
                                - param.s12.powi(5) * (36. * param.s1 + 11. * param.s2)
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (12. * param.s1 + 7. * param.s2))
                        + param.m2_2.powi(3)
                            * param.s12
                            * (-11. * param.s12.powi(6)
                                + 2. * param.s12.powi(5) * (84. * param.s1 + 29. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (109. * param.s1.powi(2)
                                        + -84. * param.s1 * param.s2
                                        + -25. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (108. * param.s1.powi(2)
                                        + 45. * param.s1 * param.s2
                                        + -13. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (-340. * param.s1.powi(3)
                                        + 221. * param.s1.powi(2) * param.s2
                                        + 56. * param.s1 * param.s2.powi(2)
                                        + 35. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (455. * param.s1.powi(4)
                                        + 928. * param.s1.powi(3) * param.s2
                                        + -1466. * param.s1.powi(2) * param.s2.powi(2)
                                        + 168. * param.s1 * param.s2.powi(3)
                                        + -85. * param.s2.powi(4))
                                - (13. * param.s1 + -3. * param.s2)
                                    * (param.s1 - param.s2).powi(5))
                        + param.s12.powi(4)
                            * (8. * param.s12.powi(5) * (param.s1 + param.s2)
                                + 4. * param.s12
                                    * (param.s1 + param.s2)
                                    * (2. * param.s1.powi(2)
                                        + -9. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2))
                                    .powi(2)
                                + -5.
                                    * param.s12.powi(4)
                                    * (5. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + 8. * param.s12.powi(3)
                                    * (5. * param.s1.powi(3)
                                        + 3. * param.s1.powi(2) * param.s2
                                        + 3. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-35. * param.s1.powi(4)
                                        + 88. * param.s1.powi(3) * param.s2
                                        + -346. * param.s1.powi(2) * param.s2.powi(2)
                                        + 88. * param.s1 * param.s2.powi(3)
                                        + -35. * param.s2.powi(4))
                                + -3.
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(4)
                                        + -14. * param.s1.powi(3) * param.s2
                                        + -74. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4))
                                - param.s12.powi(6))
                        + param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (9. * param.s12.powi(6)
                                + -14. * param.s12.powi(5) * (8. * param.s1 + 3. * param.s2)
                                + (param.s1 - param.s2).powi(4)
                                    * (47. * param.s1.powi(2)
                                        + 16. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-95. * param.s1.powi(2)
                                        + 140. * param.s1 * param.s2
                                        + 75. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (205. * param.s1.powi(3)
                                        + -484. * param.s1.powi(2) * param.s2
                                        + 56. * param.s1 * param.s2.powi(2)
                                        + -15. * param.s2.powi(3))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (118. * param.s1.powi(3)
                                        + 617. * param.s1.powi(2) * param.s2
                                        + 62. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-905. * param.s1.powi(4)
                                        + 1248. * param.s1.powi(3) * param.s2
                                        + 1074. * param.s1.powi(2) * param.s2.powi(2)
                                        + -392. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4)))
                        + param.m2_2
                            * param.s12.powi(3)
                            * (param.s12.powi(5) * (8. * param.s1 + -2. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (param.s1.powi(2)
                                        + 20. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (27. * param.s1.powi(3)
                                        + 269. * param.s1.powi(2) * param.s2
                                        + 71. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (20. * param.s1.powi(3)
                                        + -151. * param.s1.powi(2) * param.s2
                                        + 64. * param.s1 * param.s2.powi(2)
                                        + 15. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (145. * param.s1.powi(4)
                                        + -1312. * param.s1.powi(3) * param.s2
                                        + 774. * param.s1.powi(2) * param.s2.powi(2)
                                        + 88. * param.s1 * param.s2.powi(3)
                                        + 65. * param.s2.powi(4))
                                + -2.
                                    * param.s12
                                    * (52. * param.s1.powi(5)
                                        + -211. * param.s1.powi(4) * param.s2
                                        + -456. * param.s1.powi(3) * param.s2.powi(2)
                                        + 674. * param.s1.powi(2) * param.s2.powi(3)
                                        + -76. * param.s1 * param.s2.powi(4)
                                        + 17. * param.s2.powi(5))
                                - param.s12.powi(6))
                        + param.m1_2.powi(2)
                            * (-2.
                                * param.m2_2.powi(2)
                                * (148. * param.s12.powi(6)
                                    + -6. * (param.s1 - param.s2).powi(6)
                                    + 1520. * param.s1 * param.s12.powi(4) * param.s2
                                    + -309. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + 57.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (param.s1 + param.s2)
                                    + -18.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (15. * param.s1.powi(2)
                                            + 28. * param.s1 * param.s2
                                            + 15. * param.s2.powi(2))
                                    + 4. * param.s12.powi(3)
                                        * (95. * param.s1.powi(3)
                                            + -278. * param.s1.powi(2) * param.s2
                                            + -278. * param.s1 * param.s2.powi(2)
                                            + 95. * param.s2.powi(3)))
                                + param.s12.powi(2)
                                    * (9. * param.s12.powi(6)
                                        + -14.
                                            * param.s12.powi(5)
                                            * (3. * param.s1 + 8. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (15. * param.s1.powi(2)
                                                + 28. * param.s1 * param.s2
                                                + -19. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (15. * param.s1.powi(3)
                                                + -56. * param.s1.powi(2) * param.s2
                                                + 484. * param.s1 * param.s2.powi(2)
                                                + -205. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(3)
                                                + 62. * param.s1.powi(2) * param.s2
                                                + 617. * param.s1 * param.s2.powi(2)
                                                + 118. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (15. * param.s1.powi(4)
                                                + -392. * param.s1.powi(3) * param.s2
                                                + 1074. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1248. * param.s1 * param.s2.powi(3)
                                                + -905. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(4)
                                            * (3. * param.s1.powi(2)
                                                + -16. * param.s1 * param.s2
                                                + -47. * param.s2.powi(2)))
                                + param.m2_2
                                    * param.s12
                                    * (61. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-208. * param.s1 + 542. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (43. * param.s1.powi(2)
                                                + 260. * param.s1 * param.s2
                                                + -351. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (32. * param.s1.powi(2)
                                                + 235. * param.s1 * param.s2
                                                + 153. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * param.s2
                                            * (-981. * param.s1.powi(2)
                                                + 664. * param.s1 * param.s2
                                                + 345. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-125. * param.s1.powi(4)
                                                + 1792. * param.s1.powi(3) * param.s2
                                                + 2366. * param.s1.powi(2) * param.s2.powi(2)
                                                + -4088. * param.s1 * param.s2.powi(3)
                                                + 55. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(5)
                                            * (7. * param.s1 + 23. * param.s2)))
                        + param.m1_2.powi(3)
                            * (-2.
                                * param.m2_2
                                * (28. * param.s12.powi(6)
                                    + 4. * (param.s1 - param.s2).powi(6)
                                    + param.s12.powi(5) * (-129. * param.s1 + 341. * param.s2)
                                    + 20.
                                        * param.s12.powi(4)
                                        * (12. * param.s1.powi(2)
                                            + -25. * param.s1 * param.s2
                                            + -29. * param.s2.powi(2))
                                    + 4. * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (30. * param.s1.powi(2)
                                            + 79. * param.s1 * param.s2
                                            + 65. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(3)
                                        * (115. * param.s1.powi(3)
                                            + -9. * param.s1.powi(2) * param.s2
                                            + -599. * param.s1 * param.s2.powi(2)
                                            + 5. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (33. * param.s1 + 43. * param.s2))
                                + param.s12
                                    * (-11. * param.s12.powi(6)
                                        + 2. * param.s12.powi(5)
                                            * (29. * param.s1 + 84. * param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (25. * param.s1.powi(2)
                                                + 84. * param.s1 * param.s2
                                                + -109. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (13. * param.s1.powi(2)
                                                + -45. * param.s1 * param.s2
                                                + -108. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (35. * param.s1.powi(3)
                                                + 56. * param.s1.powi(2) * param.s2
                                                + 221. * param.s1 * param.s2.powi(2)
                                                + -340. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-85. * param.s1.powi(4)
                                                + 168. * param.s1.powi(3) * param.s2
                                                + -1466. * param.s1.powi(2) * param.s2.powi(2)
                                                + 928. * param.s1 * param.s2.powi(3)
                                                + 455. * param.s2.powi(4))
                                        - (3. * param.s1 + -13. * param.s2)
                                            * (param.s1 - param.s2).powi(5)))
                        + param.m1_2
                            * (-2.
                                * param.m2_2.powi(3)
                                * (28. * param.s12.powi(6)
                                    + param.s12.powi(5) * (341. * param.s1 + -129. * param.s2)
                                    + 4. * (param.s1 - param.s2).powi(6)
                                    + -20.
                                        * param.s12.powi(4)
                                        * (29. * param.s1.powi(2)
                                            + 25. * param.s1 * param.s2
                                            + -12. * param.s2.powi(2))
                                    + 4. * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (65. * param.s1.powi(2)
                                            + 79. * param.s1 * param.s2
                                            + 30. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(3)
                                        * (5. * param.s1.powi(3)
                                            + -599. * param.s1.powi(2) * param.s2
                                            + -9. * param.s1 * param.s2.powi(2)
                                            + 115. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (43. * param.s1 + 33. * param.s2))
                                + param.m2_2.powi(2)
                                    * param.s12
                                    * (61. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (542. * param.s1 + -208. * param.s2)
                                        + (param.s1 - param.s2).powi(5)
                                            * (23. * param.s1 + 7. * param.s2)
                                        + 4. * param.s1
                                            * param.s12.powi(3)
                                            * (345. * param.s1.powi(2)
                                                + 664. * param.s1 * param.s2
                                                + -981. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(4)
                                            * (351. * param.s1.powi(2)
                                                + -260. * param.s1 * param.s2
                                                + -43. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (153. * param.s1.powi(2)
                                                + 235. * param.s1 * param.s2
                                                + 32. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (55. * param.s1.powi(4)
                                                + -4088. * param.s1.powi(3) * param.s2
                                                + 2366. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1792. * param.s1 * param.s2.powi(3)
                                                + -125. * param.s2.powi(4)))
                                + -2.
                                    * param.m2_2
                                    * param.s12.powi(2)
                                    * (2. * param.s12.powi(6)
                                        + 29. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + 6. * (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -10.
                                            * param.s12.powi(4)
                                            * (13. * param.s1.powi(2)
                                                + -70. * param.s1 * param.s2
                                                + 13. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (13. * param.s1.powi(3)
                                                + 787. * param.s1.powi(2) * param.s2
                                                + 787. * param.s1 * param.s2.powi(2)
                                                + 13. * param.s2.powi(3))
                                        + 2. * param.s12.powi(3)
                                            * (95. * param.s1.powi(3)
                                                + -339. * param.s1.powi(2) * param.s2
                                                + -339. * param.s1 * param.s2.powi(2)
                                                + 95. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (55. * param.s1.powi(4)
                                                + 418. * param.s1.powi(3) * param.s2
                                                + -1466. * param.s1.powi(2) * param.s2.powi(2)
                                                + 418. * param.s1 * param.s2.powi(3)
                                                + 55. * param.s2.powi(4)))
                                + param.s12.powi(3)
                                    * (-2. * param.s12.powi(5) * (param.s1 + -4. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (5. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (7. * param.s1.powi(3)
                                                + -71. * param.s1.powi(2) * param.s2
                                                + -269. * param.s1 * param.s2.powi(2)
                                                + -27. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (15. * param.s1.powi(3)
                                                + 64. * param.s1.powi(2) * param.s2
                                                + -151. * param.s1 * param.s2.powi(2)
                                                + 20. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (65. * param.s1.powi(4)
                                                + 88. * param.s1.powi(3) * param.s2
                                                + 774. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1312. * param.s1 * param.s2.powi(3)
                                                + 145. * param.s2.powi(4))
                                        + -2.
                                            * param.s12
                                            * (17. * param.s1.powi(5)
                                                + -76. * param.s1.powi(4) * param.s2
                                                + 674. * param.s1.powi(3) * param.s2.powi(2)
                                                + -456. * param.s1.powi(2) * param.s2.powi(3)
                                                + -211. * param.s1 * param.s2.powi(4)
                                                + 52. * param.s2.powi(5))
                                        - param.s12.powi(6))))
                - param.m1_2.powi(3)
                    * (4.
                        * param.m2_2
                        * param.s12
                        * (13. * param.s12.powi(7)
                            + -3. * (param.s1 + -3. * param.s2) * (param.s1 - param.s2).powi(6)
                            + -75. * param.s12.powi(6) * (param.s1 + 2. * param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (25. * param.s1.powi(2)
                                    + -54. * param.s1 * param.s2
                                    + -85. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (183. * param.s1.powi(2)
                                    + 386. * param.s1 * param.s2
                                    + 171. * param.s2.powi(2))
                            + 3. * param.s12.powi(3)
                                * (65. * param.s1.powi(4)
                                    + -96. * param.s1.powi(3) * param.s2
                                    + 344. * param.s1.powi(2) * param.s2.powi(2)
                                    + 400. * param.s1 * param.s2.powi(3)
                                    + -225. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (245. * param.s1.powi(3)
                                    + 185. * param.s1.powi(2) * param.s2
                                    + 1493. * param.s1 * param.s2.powi(2)
                                    + -315. * param.s2.powi(3))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (93. * param.s1.powi(3)
                                    + -178. * param.s1.powi(2) * param.s2
                                    + -557. * param.s1 * param.s2.powi(2)
                                    + -402. * param.s2.powi(3)))
                        + param.m2_2.powi(2)
                            * (148. * param.s12.powi(7)
                                + -30. * (param.s1 - param.s2).powi(7)
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (61. * param.s1 + 64. * param.s2)
                                + param.s12.powi(6) * (-789. * param.s1 + 171. * param.s2)
                                + 40.
                                    * param.s12.powi(5)
                                    * (45. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + -45. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(4)
                                    * (575. * param.s1.powi(3)
                                        + 908. * param.s1.powi(2) * param.s2
                                        + -352. * param.s1 * param.s2.powi(2)
                                        + -765. * param.s2.powi(3))
                                + 36.
                                    * param.s12.powi(3)
                                    * (50. * param.s1.powi(4)
                                        + 41. * param.s1.powi(3) * param.s2
                                        + 13. * param.s1.powi(2) * param.s2.powi(2)
                                        + -39. * param.s1 * param.s2.powi(3)
                                        + -65. * param.s2.powi(4))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (873. * param.s1.powi(2)
                                        + 1400. * param.s1 * param.s2
                                        + 987. * param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (16. * param.s12.powi(7)
                                + -3. * param.s12.powi(6) * (34. * param.s1 + 49. * param.s2)
                                + -3.
                                    * (param.s1 - param.s2).powi(5)
                                    * (2. * param.s1.powi(2)
                                        + -7. * param.s1 * param.s2
                                        + 10. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (276. * param.s1.powi(2)
                                        + 494. * param.s1 * param.s2
                                        + 660. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (26. * param.s1.powi(3)
                                        + -71. * param.s1.powi(2) * param.s2
                                        + 61. * param.s1 * param.s2.powi(2)
                                        + 194. * param.s2.powi(3))
                                + 12.
                                    * param.s12.powi(3)
                                    * (30. * param.s1.powi(4)
                                        + -15. * param.s1.powi(3) * param.s2
                                        + -79. * param.s1.powi(2) * param.s2.powi(2)
                                        + 212. * param.s1 * param.s2.powi(3)
                                        + 10. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-186. * param.s1.powi(5)
                                        + 535. * param.s1.powi(4) * param.s2
                                        + 138. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2292. * param.s1.powi(2) * param.s2.powi(3)
                                        + -3448. * param.s1 * param.s2.powi(4)
                                        + 669. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (410. * param.s1.powi(3)
                                        + 455. * param.s1.powi(2) * param.s2
                                        + 359. * param.s1 * param.s2.powi(2)
                                        + 960. * param.s2.powi(3))))
                - param.m1_2.powi(5)
                    * (4. * param.s12.powi(7)
                        + -3. * (param.s1 - param.s2).powi(7)
                        + -3. * param.s12.powi(6) * (9. * param.s1 + 14. * param.s2)
                        + 2. * param.s12
                            * (param.s1 - param.s2).powi(5)
                            * (11. * param.s1 + 14. * param.s2)
                        + 2. * param.s12.powi(5)
                            * (39. * param.s1.powi(2)
                                + 85. * param.s1 * param.s2
                                + 126. * param.s2.powi(2))
                        + 12.
                            * param.s12.powi(3)
                            * (10. * param.s1.powi(4)
                                + 8. * param.s1.powi(3) * param.s2
                                + 12. * param.s1.powi(2) * param.s2.powi(2)
                                + 5. * param.s1 * param.s2.powi(3)
                                + -35. * param.s2.powi(4))
                        - param.s12.powi(4)
                            * (125. * param.s1.powi(3)
                                + 239. * param.s1.powi(2) * param.s2
                                + 473. * param.s1 * param.s2.powi(2)
                                + -105. * param.s2.powi(3))
                        - param.s12.powi(2)
                            * (param.s1 - param.s2).powi(3)
                            * (69. * param.s1.powi(2)
                                + 131. * param.s1 * param.s2
                                + 126. * param.s2.powi(2))))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s12.powi(4)
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
                    * (7. * param.m1_2.powi(2) * (param.s12 - param.s2 - param.s1) * param.s2
                        + param.m0_2.powi(2)
                            * (4. * param.s12.powi(2) + -3. * (param.s1 - param.s2).powi(2)
                                - param.s12 * (param.s1 + param.s2))
                        + param.m0_2
                            * (3. * param.s1.powi(3)
                                + -5. * param.s1.powi(2) * param.s12
                                + param.s1 * param.s12.powi(2)
                                + param.s12.powi(3)
                                + -3. * param.s1.powi(2) * param.s2
                                + 14. * param.s1 * param.s12 * param.s2
                                + param.s12.powi(2) * param.s2
                                + -3. * param.s1 * param.s2.powi(2)
                                + -5. * param.s12 * param.s2.powi(2)
                                + 3. * param.s2.powi(3)
                                + -2.
                                    * param.m1_2
                                    * (2. * param.s1.powi(2)
                                        + -4. * param.s1 * param.s12
                                        + 2. * param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + 3. * param.s12 * param.s2
                                        + -5. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (-5. * param.s1.powi(2)
                                        + 3. * param.s1 * param.s12
                                        + 2. * param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + -4. * param.s12 * param.s2
                                        + 2. * param.s2.powi(2)))
                        + 2. * param.m1_2
                            * (2.
                                * param.m2_2
                                * (param.s1.powi(2)
                                    + param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + param.s2.powi(2)
                                    + -2. * param.s12 * (param.s1 + param.s2))
                                - param.s2
                                    * (-5. * param.s1.powi(2)
                                        + 3. * param.s1 * param.s12
                                        + 2. * param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + -4. * param.s12 * param.s2
                                        + 2. * param.s2.powi(2)))
                        - param.s1
                            * (7. * param.m2_2.powi(2) * (param.s1 + param.s2 - param.s12)
                                + 2. * param.m2_2
                                    * (2. * param.s1.powi(2)
                                        + -4. * param.s1 * param.s12
                                        + 2. * param.s12.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + 3. * param.s12 * param.s2
                                        + -5. * param.s2.powi(2))
                                + param.s2
                                    * (-4. * param.s12.powi(2)
                                        + 3. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2))))
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
