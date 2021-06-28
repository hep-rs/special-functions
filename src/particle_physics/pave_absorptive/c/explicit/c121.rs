use super::{log_diff, Parameters};

pub(crate) fn c121(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.008333333333333333
            * std::f64::consts::PI
            * param.s1.powi(-3)
            * param.lambda_s12_sqrt.powi(-9)
            * ((param.m1_2
                * param.s1.powi(3)
                * (4. * param.s12.powi(6)
                    + -3. * param.s12.powi(5) * (8. * param.s1 + 13. * param.s2)
                    + param.s12.powi(4)
                        * (60. * param.s1.powi(2)
                            + 119. * param.s1 * param.s2
                            + 195. * param.s2.powi(2))
                    + -2.
                        * param.s12.powi(3)
                        * (40. * param.s1.powi(3)
                            + 43. * param.s1.powi(2) * param.s2
                            + 28. * param.s1 * param.s2.powi(2)
                            + 130. * param.s2.powi(3))
                    + (param.s1 - param.s2).powi(3)
                        * (4. * param.s1.powi(3)
                            + -25. * param.s1.powi(2) * param.s2
                            + 80. * param.s1 * param.s2.powi(2)
                            + 131. * param.s2.powi(3))
                    + 2. * param.s12.powi(2)
                        * (30. * param.s1.powi(4)
                            + -33. * param.s1.powi(3) * param.s2
                            + -153. * param.s1.powi(2) * param.s2.powi(2)
                            + 502. * param.s1 * param.s2.powi(3)
                            + -30. * param.s2.powi(4))
                    + param.s12
                        * (-24. * param.s1.powi(5)
                            + 109. * param.s1.powi(4) * param.s2
                            + 980. * param.s1.powi(2) * param.s2.powi(3)
                            + -1356. * param.s1 * param.s2.powi(4)
                            + 291. * param.s2.powi(5))
                    + -210.
                        * param.m2_2.powi(3)
                        * (param.s12.powi(3)
                            + -7. * param.s1.powi(2) * param.s2
                            + -7. * param.s1 * param.s2.powi(2)
                            + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                            + param.s12
                                * (3. * param.s1.powi(2)
                                    + 10. * param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                            - param.s2.powi(3)
                            - param.s1.powi(3))
                    + 10.
                        * param.m2_2.powi(2)
                        * (6. * param.s1.powi(4)
                            + 6. * param.s12.powi(4)
                            + 155. * param.s1.powi(3) * param.s2
                            + 119. * param.s1.powi(2) * param.s2.powi(2)
                            + -223. * param.s1 * param.s2.powi(3)
                            + -57. * param.s2.powi(4)
                            + param.s12.powi(3) * (-24. * param.s1 + 39. * param.s2)
                            + param.s12.powi(2)
                                * (36. * param.s1.powi(2)
                                    + 77. * param.s1 * param.s2
                                    + -153. * param.s2.powi(2))
                            + param.s12
                                * (-24. * param.s1.powi(3)
                                    + -271. * param.s1.powi(2) * param.s2
                                    + 170. * param.s1 * param.s2.powi(2)
                                    + 165. * param.s2.powi(3)))
                    + 5. * param.m2_2
                        * (3. * param.s12.powi(5)
                            + 6. * param.s1
                                * param.s12.powi(3)
                                * (5. * param.s1 + 12. * param.s2)
                            + -3. * param.s12.powi(4) * (5. * param.s1 + 13. * param.s2)
                            + param.s12.powi(2)
                                * (-30. * param.s1.powi(3)
                                    + 18. * param.s1.powi(2) * param.s2
                                    + -524. * param.s1 * param.s2.powi(2)
                                    + 204. * param.s2.powi(3))
                            + param.s12
                                * (15. * param.s1.powi(4)
                                    + -96. * param.s1.powi(3) * param.s2
                                    + 328. * param.s1.powi(2) * param.s2.powi(2)
                                    + 388. * param.s1 * param.s2.powi(3)
                                    + -267. * param.s2.powi(4))
                            - (param.s1 - param.s2).powi(2)
                                * (3. * param.s1.powi(3)
                                    + -39. * param.s1.powi(2) * param.s2
                                    + -277. * param.s1 * param.s2.powi(2)
                                    + -99. * param.s2.powi(3))))
                + param.m1_2.powi(3)
                    * param.s1
                    * (4. * param.s1.powi(6)
                        + 4. * param.s12.powi(6)
                        + -47. * param.s1.powi(5) * param.s2
                        + 337. * param.s1.powi(4) * param.s2.powi(2)
                        + 982. * param.s1.powi(3) * param.s2.powi(3)
                        + -1118. * param.s1.powi(2) * param.s2.powi(4)
                        + -167. * param.s1 * param.s2.powi(5)
                        + 9. * param.s2.powi(6)
                        + param.s12.powi(4)
                            * (60. * param.s1.powi(2)
                                + 69. * param.s1 * param.s2
                                + 85. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(3)
                            * (40. * param.s1.powi(3)
                                + -7. * param.s1.powi(2) * param.s2
                                + -52. * param.s1 * param.s2.powi(2)
                                + 65. * param.s2.powi(3))
                        + 2. * param.s12.powi(2)
                            * (30. * param.s1.powi(4)
                                + -83. * param.s1.powi(3) * param.s2
                                + -63. * param.s1.powi(2) * param.s2.powi(2)
                                + -243. * param.s1 * param.s2.powi(3)
                                + 55. * param.s2.powi(4))
                        + param.s12
                            * (-24. * param.s1.powi(5)
                                + 159. * param.s1.powi(4) * param.s2
                                + -400. * param.s1.powi(3) * param.s2.powi(2)
                                + 1170. * param.s1.powi(2) * param.s2.powi(3)
                                + 504. * param.s1 * param.s2.powi(4)
                                + -49. * param.s2.powi(5))
                        + 5. * param.m2_2
                            * (param.s12.powi(5)
                                + 23. * param.s1.powi(4) * param.s2
                                + 314. * param.s1.powi(3) * param.s2.powi(2)
                                + 314. * param.s1.powi(2) * param.s2.powi(3)
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
                                        + -378. * param.s1.powi(2) * param.s2.powi(2)
                                        + -64. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                - param.s2.powi(5)
                                - param.s1.powi(5))
                        - param.s12.powi(5) * (24. * param.s1 + 29. * param.s2))
                + param.m0_2.powi(3)
                    * (param.m1_2
                        * (4. * param.s12.powi(6)
                            + -3. * param.s12.powi(5) * (13. * param.s1 + 8. * param.s2)
                            + param.s12.powi(4)
                                * (195. * param.s1.powi(2)
                                    + 119. * param.s1 * param.s2
                                    + 60. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(3)
                                * (130. * param.s1.powi(3)
                                    + 28. * param.s1.powi(2) * param.s2
                                    + 43. * param.s1 * param.s2.powi(2)
                                    + 40. * param.s2.powi(3))
                            + -2.
                                * param.s12.powi(2)
                                * (30. * param.s1.powi(4)
                                    + -502. * param.s1.powi(3) * param.s2
                                    + 153. * param.s1.powi(2) * param.s2.powi(2)
                                    + 33. * param.s1 * param.s2.powi(3)
                                    + -30. * param.s2.powi(4))
                            + param.s12
                                * (291. * param.s1.powi(5)
                                    + -1356. * param.s1.powi(4) * param.s2
                                    + 980. * param.s1.powi(3) * param.s2.powi(2)
                                    + 109. * param.s1 * param.s2.powi(4)
                                    + -24. * param.s2.powi(5))
                            - (param.s1 - param.s2).powi(3)
                                * (131. * param.s1.powi(3)
                                    + 80. * param.s1.powi(2) * param.s2
                                    + -25. * param.s1 * param.s2.powi(2)
                                    + 4. * param.s2.powi(3)))
                        + param.s1
                            * (4. * param.s12.powi(6)
                                + (param.s1 - param.s2).powi(4)
                                    * (9. * param.s1.powi(2) + 4. * param.s1 * param.s2
                                        - param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-5. * param.s1.powi(2)
                                        + 104. * param.s1 * param.s2
                                        + 35. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (320. * param.s1.powi(3)
                                        + -836. * param.s1.powi(2) * param.s2
                                        + 34. * param.s1 * param.s2.powi(2)
                                        + -30. * param.s2.powi(3))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (131. * param.s1.powi(3)
                                        + 481. * param.s1.powi(2) * param.s2
                                        + 51. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-400. * param.s1.powi(4)
                                        + 564. * param.s1.powi(3) * param.s2
                                        + 474. * param.s1.powi(2) * param.s2.powi(2)
                                        + -136. * param.s1 * param.s2.powi(3)
                                        + 10. * param.s2.powi(4))
                                + -5.
                                    * param.m2_2
                                    * (param.s12.powi(5)
                                        + param.s12.powi(3)
                                            * (-68. * param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + 10. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (76. * param.s1.powi(3)
                                                + -50. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3))
                                        + param.s12
                                            * (-37. * param.s1.powi(4)
                                                + -116. * param.s1.powi(3) * param.s2
                                                + 164. * param.s1.powi(2) * param.s2.powi(2)
                                                + -16. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (31. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s12.powi(4) * (17. * param.s1 + 5. * param.s2))
                                - param.s12.powi(5) * (59. * param.s1 + 19. * param.s2)))
                + param.m0_2.powi(2)
                    * (param.s1.powi(2)
                        * (-6. * param.s12.powi(6)
                            + param.s12.powi(5) * (param.s1 + param.s2)
                            + param.s12.powi(4)
                                * (55. * param.s1.powi(2)
                                    + -496. * param.s1 * param.s2
                                    + 55. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(3)
                                * (55. * param.s1.powi(3)
                                    + -262. * param.s1.powi(2) * param.s2
                                    + -262. * param.s1 * param.s2.powi(2)
                                    + 55. * param.s2.powi(3))
                            + 8. * param.s12.powi(2)
                                * (10. * param.s1.powi(4)
                                    + 53. * param.s1.powi(3) * param.s2
                                    + -222. * param.s1.powi(2) * param.s2.powi(2)
                                    + 53. * param.s1 * param.s2.powi(3)
                                    + 10. * param.s2.powi(4))
                            + -10.
                                * param.m2_2.powi(2)
                                * (3. * param.s12.powi(4)
                                    + 3. * param.s12.powi(3)
                                        * (17. * param.s1 + -4. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (48. * param.s1.powi(2)
                                            + 52. * param.s1 * param.s2
                                            + 3. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-63. * param.s1.powi(2)
                                            + -56. * param.s1 * param.s2
                                            + 18. * param.s2.powi(2))
                                    - param.s12
                                        * (39. * param.s1.powi(3)
                                            + -184. * param.s1.powi(2) * param.s2
                                            + 41. * param.s1 * param.s2.powi(2)
                                            + 12. * param.s2.powi(3)))
                            + 5. * param.m2_2
                                * (7. * param.s12.powi(5)
                                    + param.s12.powi(4) * (61. * param.s1 + -23. * param.s2)
                                    + param.s12.powi(3)
                                        * (-188. * param.s1.powi(2)
                                            + 148. * param.s1 * param.s2
                                            + 22. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (64. * param.s1.powi(3)
                                            + 158. * param.s1.powi(2) * param.s2
                                            + -211. * param.s1 * param.s2.powi(2)
                                            + param.s2.powi(3))
                                    + param.s12
                                        * (29. * param.s1.powi(4)
                                            + -480. * param.s1.powi(3) * param.s2
                                            + 308. * param.s1.powi(2) * param.s2.powi(2)
                                            + 156. * param.s1 * param.s2.powi(3)
                                            + -13. * param.s2.powi(4))
                                    - (param.s1 - param.s2).powi(3)
                                        * (37. * param.s1.powi(2)
                                            + 72. * param.s1 * param.s2
                                            + 5. * param.s2.powi(2)))
                            - param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (19. * param.s1.powi(3)
                                    + 479. * param.s1.powi(2) * param.s2
                                    + 479. * param.s1 * param.s2.powi(2)
                                    + 19. * param.s2.powi(3))
                            - (param.s1 - param.s2).powi(4)
                                * (param.s1.powi(2)
                                    + 16. * param.s1 * param.s2
                                    + param.s2.powi(2)))
                        + param.m1_2
                            * param.s1
                            * (-4. * param.s12.powi(6)
                                + param.s12.powi(5) * (44. * param.s1 + 9. * param.s2)
                                + param.s12.powi(4)
                                    * (-100. * param.s1.powi(2)
                                        + 111. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (20. * param.s1.powi(3)
                                        + 608. * param.s1.powi(2) * param.s2
                                        + -232. * param.s1 * param.s2.powi(2)
                                        + -35. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(3)
                                    * (36. * param.s1.powi(3)
                                        + 445. * param.s1.powi(2) * param.s2
                                        + 100. * param.s1 * param.s2.powi(2)
                                        + -11. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (50. * param.s1.powi(4)
                                        + -1232. * param.s1.powi(3) * param.s2
                                        + 678. * param.s1.powi(2) * param.s2.powi(2)
                                        + 143. * param.s1 * param.s2.powi(3)
                                        + 45. * param.s2.powi(4))
                                + param.s12
                                    * (-116. * param.s1.powi(5)
                                        + 791. * param.s1.powi(4) * param.s2
                                        + 1580. * param.s1.powi(3) * param.s2.powi(2)
                                        + -2360. * param.s1.powi(2) * param.s2.powi(3)
                                        + 156. * param.s1 * param.s2.powi(4)
                                        + -51. * param.s2.powi(5))
                                + 5. * param.m2_2
                                    * (3. * param.s12.powi(5)
                                        + 6. * param.s12.powi(3)
                                            * param.s2
                                            * (12. * param.s1 + 5. * param.s2)
                                        + -3.
                                            * param.s12.powi(4)
                                            * (13. * param.s1 + 5. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (102. * param.s1.powi(3)
                                                + -262. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -15. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(2)
                                            * (99. * param.s1.powi(3)
                                                + 277. * param.s1.powi(2) * param.s2
                                                + 39. * param.s1 * param.s2.powi(2)
                                                + -3. * param.s2.powi(3))
                                        + param.s12
                                            * (-267. * param.s1.powi(4)
                                                + 388. * param.s1.powi(3) * param.s2
                                                + 328. * param.s1.powi(2) * param.s2.powi(2)
                                                + -96. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4))))
                        - param.m1_2.powi(2)
                            * (6. * param.s12.powi(6)
                                + -3. * param.s12.powi(5) * (17. * param.s1 + 12. * param.s2)
                                + 3. * param.s12.powi(4)
                                    * (65. * param.s1.powi(2)
                                        + 47. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + -6.
                                    * param.s12.powi(3)
                                    * (65. * param.s1.powi(3)
                                        + -16. * param.s1.powi(2) * param.s2
                                        + 9. * param.s1 * param.s2.powi(2)
                                        + 20. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(2)
                                    * (51. * param.s1.powi(4)
                                        + 769. * param.s1.powi(3) * param.s2
                                        + 255. * param.s1.powi(2) * param.s2.powi(2)
                                        + -51. * param.s1 * param.s2.powi(3)
                                        + 6. * param.s2.powi(4))
                                + 2. * param.s12.powi(2)
                                    * (210. * param.s1.powi(4)
                                        + 98. * param.s1.powi(3) * param.s2
                                        + -207. * param.s1.powi(2) * param.s2.powi(2)
                                        + -87. * param.s1 * param.s2.powi(3)
                                        + 45. * param.s2.powi(4))
                                - param.s12
                                    * (231. * param.s1.powi(5)
                                        + 1064. * param.s1.powi(4) * param.s2
                                        + -2290. * param.s1.powi(3) * param.s2.powi(2)
                                        + 240. * param.s1.powi(2) * param.s2.powi(3)
                                        + -201. * param.s1 * param.s2.powi(4)
                                        + 36. * param.s2.powi(5))))
                + param.m0_2
                    * (param.m1_2.powi(3)
                        * (9. * param.s1.powi(6)
                            + 4. * param.s12.powi(6)
                            + -167. * param.s1.powi(5) * param.s2
                            + -1118. * param.s1.powi(4) * param.s2.powi(2)
                            + 982. * param.s1.powi(3) * param.s2.powi(3)
                            + 337. * param.s1.powi(2) * param.s2.powi(4)
                            + -47. * param.s1 * param.s2.powi(5)
                            + 4. * param.s2.powi(6)
                            + param.s12.powi(4)
                                * (85. * param.s1.powi(2)
                                    + 69. * param.s1 * param.s2
                                    + 60. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(3)
                                * (65. * param.s1.powi(3)
                                    + -52. * param.s1.powi(2) * param.s2
                                    + -7. * param.s1 * param.s2.powi(2)
                                    + 40. * param.s2.powi(3))
                            + 2. * param.s12.powi(2)
                                * (55. * param.s1.powi(4)
                                    + -243. * param.s1.powi(3) * param.s2
                                    + -63. * param.s1.powi(2) * param.s2.powi(2)
                                    + -83. * param.s1 * param.s2.powi(3)
                                    + 30. * param.s2.powi(4))
                            + param.s12
                                * (-49. * param.s1.powi(5)
                                    + 504. * param.s1.powi(4) * param.s2
                                    + 1170. * param.s1.powi(3) * param.s2.powi(2)
                                    + -400. * param.s1.powi(2) * param.s2.powi(3)
                                    + 159. * param.s1 * param.s2.powi(4)
                                    + -24. * param.s2.powi(5))
                            - param.s12.powi(5) * (29. * param.s1 + 24. * param.s2))
                        + param.m1_2.powi(2)
                            * param.s1
                            * (-4. * param.s12.powi(6)
                                + 39. * param.s12.powi(5) * (param.s1 + param.s2)
                                + param.s12.powi(3)
                                    * (230. * param.s1.powi(3)
                                        + 326. * param.s1.powi(2) * param.s2
                                        + 326. * param.s1 * param.s2.powi(2)
                                        + 230. * param.s2.powi(3))
                                + -6.
                                    * param.s12.powi(2)
                                    * (35. * param.s1.powi(4)
                                        + -56. * param.s1.powi(3) * param.s2
                                        + 464. * param.s1.powi(2) * param.s2.powi(2)
                                        + -56. * param.s1 * param.s2.powi(3)
                                        + 35. * param.s2.powi(4))
                                + param.s12
                                    * (99. * param.s1.powi(5)
                                        + -709. * param.s1.powi(4) * param.s2
                                        + 1530. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1530. * param.s1.powi(2) * param.s2.powi(3)
                                        + -709. * param.s1 * param.s2.powi(4)
                                        + 99. * param.s2.powi(5))
                                + -5.
                                    * param.m2_2
                                    * (-15. * param.s1.powi(5)
                                        + 3. * param.s12.powi(5)
                                        + -367. * param.s1.powi(4) * param.s2
                                        + -178. * param.s1.powi(3) * param.s2.powi(2)
                                        + 506. * param.s1.powi(2) * param.s2.powi(3)
                                        + 57. * param.s1 * param.s2.powi(4)
                                        + -3. * param.s2.powi(5)
                                        + -3.
                                            * param.s12.powi(4)
                                            * (9. * param.s1 + 5. * param.s2)
                                        + 6. * param.s12.powi(3)
                                            * (13. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (51. * param.s1.powi(3)
                                                + 185. * param.s1.powi(2) * param.s2
                                                + -45. * param.s1 * param.s2.powi(2)
                                                + 15. * param.s2.powi(3))
                                        + param.s12
                                            * (63. * param.s1.powi(4)
                                                + 728. * param.s1.powi(3) * param.s2
                                                + -214. * param.s1.powi(2) * param.s2.powi(2)
                                                + -144. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4)))
                                - (param.s1 - param.s2).powi(2)
                                    * (19. * param.s1.powi(4)
                                        + -254. * param.s1.powi(3) * param.s2
                                        + -1590. * param.s1.powi(2) * param.s2.powi(2)
                                        + -254. * param.s1 * param.s2.powi(3)
                                        + 19. * param.s2.powi(4))
                                - param.s12.powi(4)
                                    * (135. * param.s1.powi(2)
                                        + 284. * param.s1 * param.s2
                                        + 135. * param.s2.powi(2)))
                        + param.m1_2
                            * param.s1.powi(2)
                            * (-4. * param.s12.powi(6)
                                + param.s12.powi(5) * (9. * param.s1 + 44. * param.s2)
                                + param.s12.powi(4)
                                    * (15. * param.s1.powi(2)
                                        + 111. * param.s1 * param.s2
                                        + -100. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (11. * param.s1.powi(3)
                                        + -100. * param.s1.powi(2) * param.s2
                                        + -445. * param.s1 * param.s2.powi(2)
                                        + -36. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-70. * param.s1.powi(3)
                                        + -464. * param.s1.powi(2) * param.s2
                                        + 1216. * param.s1 * param.s2.powi(2)
                                        + 40. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + 143. * param.s1.powi(3) * param.s2
                                        + 678. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1232. * param.s1 * param.s2.powi(3)
                                        + 50. * param.s2.powi(4))
                                + param.s12
                                    * (-51. * param.s1.powi(5)
                                        + 156. * param.s1.powi(4) * param.s2
                                        + -2360. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1580. * param.s1.powi(2) * param.s2.powi(3)
                                        + 791. * param.s1 * param.s2.powi(4)
                                        + -116. * param.s2.powi(5))
                                + 10.
                                    * param.m2_2.powi(2)
                                    * (-57. * param.s1.powi(4)
                                        + 6. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3)
                                            * (13. * param.s1 + -8. * param.s2)
                                        + -223. * param.s1.powi(3) * param.s2
                                        + 119. * param.s1.powi(2) * param.s2.powi(2)
                                        + 155. * param.s1 * param.s2.powi(3)
                                        + 6. * param.s2.powi(4)
                                        + param.s12.powi(2)
                                            * (-153. * param.s1.powi(2)
                                                + 77. * param.s1 * param.s2
                                                + 36. * param.s2.powi(2))
                                        + param.s12
                                            * (165. * param.s1.powi(3)
                                                + 170. * param.s1.powi(2) * param.s2
                                                + -271. * param.s1 * param.s2.powi(2)
                                                + -24. * param.s2.powi(3)))
                                + -20.
                                    * param.m2_2
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (-14. * param.s1.powi(2)
                                                + 103. * param.s1 * param.s2
                                                + -14. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 98. * param.s1.powi(2) * param.s2
                                                + 98. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (26. * param.s1.powi(3)
                                                + -121. * param.s1.powi(2) * param.s2
                                                + -121. * param.s1 * param.s2.powi(2)
                                                + 26. * param.s2.powi(3))
                                        - param.s12
                                            * (19. * param.s1.powi(4)
                                                + 71. * param.s1.powi(3) * param.s2
                                                + -364. * param.s1.powi(2) * param.s2.powi(2)
                                                + 71. * param.s1 * param.s2.powi(3)
                                                + 19. * param.s2.powi(4))))
                        + param.s1.powi(3)
                            * (4. * param.s12.powi(6)
                                + param.s12.powi(4)
                                    * (35. * param.s1.powi(2)
                                        + 104. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + 51. * param.s1.powi(2) * param.s2
                                        + 481. * param.s1 * param.s2.powi(2)
                                        + 131. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-30. * param.s1.powi(3)
                                        + 34. * param.s1.powi(2) * param.s2
                                        + -836. * param.s1 * param.s2.powi(2)
                                        + 320. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (5. * param.s1.powi(4)
                                        + -68. * param.s1.powi(3) * param.s2
                                        + 237. * param.s1.powi(2) * param.s2.powi(2)
                                        + 282. * param.s1 * param.s2.powi(3)
                                        + -200. * param.s2.powi(4))
                                + 30.
                                    * param.m2_2.powi(3)
                                    * (17. * param.s1.powi(3)
                                        + 7. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2) * (param.s1 + -7. * param.s2)
                                        + 15. * param.s1.powi(2) * param.s2
                                        + -25. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3)
                                        + param.s12
                                            * (-27. * param.s1.powi(2)
                                                + 22. * param.s1 * param.s2
                                                + 21. * param.s2.powi(2)))
                                + -10.
                                    * param.m2_2.powi(2)
                                    * (24. * param.s12.powi(4)
                                        + -33. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-45. * param.s1.powi(2)
                                                + 268. * param.s1 * param.s2
                                                + -45. * param.s2.powi(2))
                                        + param.s12
                                            * (93. * param.s1.powi(3)
                                                + -185. * param.s1.powi(2) * param.s2
                                                + -185. * param.s1 * param.s2.powi(2)
                                                + 93. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (39. * param.s1.powi(2)
                                                + 128. * param.s1 * param.s2
                                                + 39. * param.s2.powi(2)))
                                + 5. * param.m2_2
                                    * (7. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-23. * param.s1 + 61. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (11. * param.s1.powi(2)
                                                + 74. * param.s1 * param.s2
                                                + -94. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 72. * param.s1 * param.s2
                                                + 37. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -211. * param.s1.powi(2) * param.s2
                                                + 158. * param.s1 * param.s2.powi(2)
                                                + 64. * param.s2.powi(3))
                                        + param.s12
                                            * (-13. * param.s1.powi(4)
                                                + 156. * param.s1.powi(3) * param.s2
                                                + 308. * param.s1.powi(2) * param.s2.powi(2)
                                                + -480. * param.s1 * param.s2.powi(3)
                                                + 29. * param.s2.powi(4)))
                                - (param.s1 - param.s2).powi(4)
                                    * (param.s1.powi(2)
                                        + -4. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2))
                                - param.s12.powi(5) * (19. * param.s1 + 59. * param.s2)))
                - param.m1_2.powi(2)
                    * param.s1.powi(2)
                    * (6. * param.s12.powi(6)
                        + -3. * param.s12.powi(5) * (12. * param.s1 + 17. * param.s2)
                        + 3. * param.s12.powi(4)
                            * (30. * param.s1.powi(2)
                                + 47. * param.s1 * param.s2
                                + 65. * param.s2.powi(2))
                        + -6.
                            * param.s12.powi(3)
                            * (20. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + -16. * param.s1 * param.s2.powi(2)
                                + 65. * param.s2.powi(3))
                        + (param.s1 - param.s2).powi(2)
                            * (6. * param.s1.powi(4)
                                + -51. * param.s1.powi(3) * param.s2
                                + 255. * param.s1.powi(2) * param.s2.powi(2)
                                + 769. * param.s1 * param.s2.powi(3)
                                + 51. * param.s2.powi(4))
                        + 2. * param.s12.powi(2)
                            * (45. * param.s1.powi(4)
                                + -87. * param.s1.powi(3) * param.s2
                                + -207. * param.s1.powi(2) * param.s2.powi(2)
                                + 98. * param.s1 * param.s2.powi(3)
                                + 210. * param.s2.powi(4))
                        + 10.
                            * param.m2_2.powi(2)
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 109. * param.s1.powi(3) * param.s2
                                + 280. * param.s1.powi(2) * param.s2.powi(2)
                                + 109. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + 133. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (6. * param.s1.powi(3)
                                        + 115. * param.s1.powi(2) * param.s2
                                        + 115. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3)))
                        + 5. * param.m2_2
                            * (-3. * param.s1.powi(5)
                                + 3. * param.s12.powi(5)
                                + 57. * param.s1.powi(4) * param.s2
                                + 506. * param.s1.powi(3) * param.s2.powi(2)
                                + -178. * param.s1.powi(2) * param.s2.powi(3)
                                + -367. * param.s1 * param.s2.powi(4)
                                + -15. * param.s2.powi(5)
                                + -3. * param.s12.powi(4) * (5. * param.s1 + 9. * param.s2)
                                + 6. * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + 13. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (15. * param.s1.powi(3)
                                        + -45. * param.s1.powi(2) * param.s2
                                        + 185. * param.s1 * param.s2.powi(2)
                                        + 51. * param.s2.powi(3))
                                + param.s12
                                    * (15. * param.s1.powi(4)
                                        + -144. * param.s1.powi(3) * param.s2
                                        + -214. * param.s1.powi(2) * param.s2.powi(2)
                                        + 728. * param.s1 * param.s2.powi(3)
                                        + 63. * param.s2.powi(4)))
                        - param.s12
                            * (36. * param.s1.powi(5)
                                + -201. * param.s1.powi(4) * param.s2
                                + 240. * param.s1.powi(3) * param.s2.powi(2)
                                + -2290. * param.s1.powi(2) * param.s2.powi(3)
                                + 1064. * param.s1 * param.s2.powi(4)
                                + 231. * param.s2.powi(5)))
                - param.s1.powi(4)
                    * (param.s12.powi(6)
                        + (param.s1 - param.s2).powi(4)
                            * (param.s1.powi(2)
                                + -4. * param.s1 * param.s2
                                + 6. * param.s2.powi(2))
                        + param.s12.powi(4)
                            * (15. * param.s1.powi(2)
                                + 36. * param.s1 * param.s2
                                + 70. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(3)
                            * (10. * param.s1.powi(3)
                                + 17. * param.s1.powi(2) * param.s2
                                + 32. * param.s1 * param.s2.powi(2)
                                + -10. * param.s2.powi(3))
                        + param.s12.powi(2)
                            * (15. * param.s1.powi(4)
                                + -4. * param.s1.powi(3) * param.s2
                                + -54. * param.s1.powi(2) * param.s2.powi(2)
                                + 376. * param.s1 * param.s2.powi(3)
                                + -205. * param.s2.powi(4))
                        + 60.
                            * param.m2_2.powi(4)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + -30.
                            * param.m2_2.powi(3)
                            * (-7. * param.s1.powi(3)
                                + 7. * param.s12.powi(3)
                                + -25. * param.s1.powi(2) * param.s2
                                + 15. * param.s1 * param.s2.powi(2)
                                + 17. * param.s2.powi(3)
                                + 3. * param.s12.powi(2) * (-7. * param.s1 + param.s2)
                                + param.s12
                                    * (21. * param.s1.powi(2)
                                        + 22. * param.s1 * param.s2
                                        + -27. * param.s2.powi(2)))
                        + 10.
                            * param.m2_2.powi(2)
                            * (3. * param.s12.powi(4)
                                + param.s12.powi(3) * (-12. * param.s1 + 51. * param.s2)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + -56. * param.s1 * param.s2
                                        + -63. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 52. * param.s1 * param.s2
                                        + 48. * param.s2.powi(2))
                                - param.s12
                                    * (12. * param.s1.powi(3)
                                        + 41. * param.s1.powi(2) * param.s2
                                        + -184. * param.s1 * param.s2.powi(2)
                                        + 39. * param.s2.powi(3)))
                        + 5. * param.m2_2
                            * (param.s12.powi(5)
                                + 2. * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 20. * param.s1 * param.s2
                                        + -34. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + 50. * param.s1 * param.s2.powi(2)
                                        + -76. * param.s2.powi(3))
                                + param.s12
                                    * (5. * param.s1.powi(4)
                                        + -16. * param.s1.powi(3) * param.s2
                                        + 164. * param.s1.powi(2) * param.s2.powi(2)
                                        + -116. * param.s1 * param.s2.powi(3)
                                        + -37. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + -31. * param.s2.powi(2))
                                - param.s12.powi(4) * (5. * param.s1 + 17. * param.s2))
                        - param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (6. * param.s1.powi(3)
                                + -9. * param.s1.powi(2) * param.s2
                                + -44. * param.s1 * param.s2.powi(2)
                                + -119. * param.s2.powi(3))
                        - param.s12.powi(5) * (6. * param.s1 + 11. * param.s2))
                - param.m1_2.powi(4)
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
                                + 3. * param.s2.powi(5)))
                - param.m0_2.powi(4)
                    * (param.s12.powi(6)
                        + (param.s1 - param.s2).powi(4)
                            * (6. * param.s1.powi(2)
                                + -4. * param.s1 * param.s2
                                + param.s2.powi(2))
                        + param.s12.powi(4)
                            * (70. * param.s1.powi(2)
                                + 36. * param.s1 * param.s2
                                + 15. * param.s2.powi(2))
                        + param.s12.powi(3)
                            * (20. * param.s1.powi(3)
                                + -64. * param.s1.powi(2) * param.s2
                                + -34. * param.s1 * param.s2.powi(2)
                                + -20. * param.s2.powi(3))
                        + param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (119. * param.s1.powi(3)
                                + 44. * param.s1.powi(2) * param.s2
                                + 9. * param.s1 * param.s2.powi(2)
                                + -6. * param.s2.powi(3))
                        + param.s12.powi(2)
                            * (-205. * param.s1.powi(4)
                                + 376. * param.s1.powi(3) * param.s2
                                + -54. * param.s1.powi(2) * param.s2.powi(2)
                                + -4. * param.s1 * param.s2.powi(3)
                                + 15. * param.s2.powi(4))
                        - param.s12.powi(5) * (11. * param.s1 + 6. * param.s2)))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s1.powi(3)
                    * (7.
                        * param.m1_2.powi(5)
                        * param.s2.powi(3)
                        * (param.s1 + param.s2 - param.s12)
                        + param.m0_2.powi(5)
                            * param.s12
                            * (2. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + 3. * param.s12 * (param.s1 - param.s2) * param.s2
                                + param.s12.powi(2) * (-3. * param.s1 + 2. * param.s2))
                        + 5. * param.m1_2.powi(4)
                            * param.s2.powi(2)
                            * (param.s2
                                * (-4. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -6. * param.s2)
                                    + param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                                - param.m2_2
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)
                                        + -6. * param.s12 * (param.s1 + param.s2)))
                        + 10.
                            * param.m1_2.powi(3)
                            * param.s2
                            * (param.s2.powi(2)
                                * (-3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + (param.s1 - param.s2).powi(2) * (2. * param.s1 + param.s2)
                                    + param.s12
                                        * (2. * param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - param.s12.powi(3))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (2. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + 2. * param.s1.powi(2) * param.s2
                                        + -3. * param.s12.powi(2) * param.s2
                                        + -3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s12
                                            * (param.s1 * param.s2 + param.s2.powi(2)
                                                - param.s1.powi(2))
                                        - param.s2.powi(3))
                                + param.m2_2.powi(2)
                                    * (param.s1.powi(3)
                                        + 6. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)
                                        + 3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(3)))
                        + param.m0_2.powi(4)
                            * (param.m1_2
                                * (-4. * param.s12.powi(4)
                                    + param.s12.powi(3) * (11. * param.s1 + -14. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + 11. * param.s2)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2)))
                                + param.s12
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-7. * param.s1 + 3. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + -6. * param.s1.powi(2) * param.s2
                                                + 4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + 3. * param.s2))
                                - param.m2_2
                                    * (6. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))))
                        + param.s1.powi(2)
                            * (param.s12
                                * (2. * param.s12.powi(3)
                                    + param.s12.powi(2) * (2. * param.s1 + -3. * param.s2)
                                    + -3. * param.s1 * param.s12 * (param.s1 - param.s2)
                                    - (param.s1 - param.s2).powi(3))
                                * param.s2.powi(3)
                                + 5. * param.m2_2.powi(4)
                                    * (-3. * param.s1 * param.s12.powi(2)
                                        + param.s12.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)
                                        + 3. * param.s12
                                            * (param.s1.powi(2) + param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s1.powi(3))
                                + 2. * param.m2_2.powi(2)
                                    * param.s2
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-7. * param.s1 + 3. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + -6. * param.s1.powi(2) * param.s2
                                                + 4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + 3. * param.s2))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (-4. * param.s1 + 6. * param.s2)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + 3. * param.s1.powi(2) * param.s2
                                                + -12. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - param.m2_2
                                    * param.s2.powi(2)
                                    * (6. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2)))
                                - param.m2_2.powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)
                                        + -6. * param.s12 * (param.s1 + param.s2)))
                        + -2.
                            * param.m1_2.powi(2)
                            * (-3.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (-4. * param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                    + -14. * param.s1.powi(3) * param.s2
                                    + 6. * param.s1.powi(2) * param.s2.powi(2)
                                    + 11. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2))
                                    + param.s12
                                        * (11. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -21. * param.s1 * param.s2.powi(2)
                                            + -4. * param.s2.powi(3)))
                                + param.m2_2.powi(3)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 16. * param.s1.powi(3) * param.s2
                                        + 36. * param.s1.powi(2) * param.s2.powi(2)
                                        + 16. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 3. * param.m2_2
                                    * param.s2.powi(2)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (6. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - param.s2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (11. * param.s1 + -4. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (2. * param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12
                                            * (-14. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -4. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (4. * param.s1 + param.s2)))
                        + param.m1_2
                            * param.s1
                            * (param.s2.powi(3)
                                * (-4. * param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (11. * param.s1 + param.s2)
                                    + param.s12.powi(3) * (-14. * param.s1 + 11. * param.s2)
                                    + param.s12.powi(2)
                                        * (6. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2)))
                                + -5.
                                    * param.m2_2.powi(4)
                                    * (param.s12.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3))
                                + 6. * param.m2_2.powi(2)
                                    * param.s2
                                    * (-2. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (-7. * param.s1.powi(3)
                                                + 12. * param.s1.powi(2) * param.s2
                                                + 12. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3)))
                                + 4. * param.m2_2
                                    * param.s2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (3. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 2. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + 4. * param.s1.powi(2) * param.s2
                                                + -6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 4. * param.m2_2.powi(3)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 11. * param.s1.powi(3) * param.s2
                                        + 6. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14. * param.s1 * param.s2.powi(3)
                                        + -4. * param.s2.powi(4)
                                        + param.s12.powi(3) * (-4. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 9. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + param.s12
                                            * (-4. * param.s1.powi(3)
                                                + -21. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3))))
                        + param.m0_2.powi(3)
                            * (2.
                                * param.m1_2.powi(2)
                                * (param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(3) * (param.s1 + 4. * param.s2)
                                    + param.s12.powi(3) * (-4. * param.s1 + 11. * param.s2)
                                    + 3. * param.s12.powi(2)
                                        * (2. * param.s1.powi(2)
                                            + -7. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12
                                        * (-4. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -14. * param.s2.powi(3)))
                                + 2. * param.m2_2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (3. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 2. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + 4. * param.s1.powi(2) * param.s2
                                                + -6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 2. * param.m2_2
                                    * (-3. * param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (8. * param.s1.powi(2)
                                                + -30. * param.s1 * param.s2
                                                + 8. * param.s2.powi(2))
                                        + -6.
                                            * param.s12.powi(2)
                                            * (2. * param.s1.powi(3)
                                                + -3. * param.s1.powi(2) * param.s2
                                                + -3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                + param.s12
                                    * (param.s12.powi(5)
                                        + -3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + -6. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + -3.
                                            * param.s12
                                            * (param.s1.powi(4)
                                                + -4. * param.s1.powi(3) * param.s2
                                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4)))
                                + param.m1_2
                                    * (-3. * param.s12.powi(5)
                                        + param.s12.powi(4) * (11. * param.s1 + -21. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + -28. * param.s1 * param.s2
                                                + -21. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-14. * param.s1.powi(2)
                                                + 14. * param.s1 * param.s2
                                                + 24. * param.s2.powi(2))
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 6. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 4. * param.m2_2
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1 + 3. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + -6. * param.s1.powi(2) * param.s2
                                                        + 4. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1 + 3. * param.s2))
                                        - (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 3. * param.s2)))
                        + param.m0_2.powi(2)
                            * (-10.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (param.s12.powi(3)
                                    + -3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + -3. * param.s1 * param.s2
                                            + -2. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2) * (param.s1 + 2. * param.s2))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (6. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (-6. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -10. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + param.s12
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (-6. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -10. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + -6.
                                    * param.m1_2.powi(2)
                                    * (param.m2_2
                                        * (param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (-4. * param.s1 + 6. * param.s2)
                                            + param.s12.powi(2)
                                                * (6. * param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (2. * param.s1.powi(3)
                                                    + 3. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3)))
                                        - param.s2
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1 + 3. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + -6. * param.s1.powi(2) * param.s2
                                                        + 4. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1 + 3. * param.s2)))
                                + -3.
                                    * param.m1_2
                                    * (2.
                                        * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(4)
                                            + -3. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + -3.
                                                * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (7. * param.s1.powi(3)
                                                    + -12. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3)))
                                        + -2.
                                            * param.m2_2
                                            * (param.s12.powi(5)
                                                + -3.
                                                    * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + -6. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + -3.
                                                    * param.s12
                                                    * (param.s1.powi(4)
                                                        + -4. * param.s1.powi(3) * param.s2
                                                        + -6.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 10. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4)))
                                        - param.s2
                                            * (-3. * param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + 3. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + -30. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                + -6.
                                                    * param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -3. * param.s1.powi(2) * param.s2
                                                        + -3. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3))))
                                - param.m2_2
                                    * (param.s12.powi(6)
                                        + 36.
                                            * param.s1
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * param.s2
                                            * (param.s1 + param.s2)
                                        + -9.
                                            * param.s12.powi(4)
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (4. * param.s1.powi(3)
                                                + -9. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + -9.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(4)
                                                + 4. * param.s1.powi(3) * param.s2
                                                + -14. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                + param.s2.powi(4))))
                        + param.m0_2
                            * (5.
                                * param.m1_2.powi(4)
                                * param.s2.powi(2)
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -4. * param.s2.powi(2)
                                    + param.s12 * (-6. * param.s1 + param.s2))
                                + 10.
                                    * param.m1_2.powi(3)
                                    * param.s2
                                    * (2.
                                        * param.m2_2
                                        * (-3. * param.s1 * param.s12.powi(2)
                                            + param.s12.powi(3)
                                            + -3. * param.s1.powi(2) * param.s2
                                            + 2. * param.s1 * param.s2.powi(2)
                                            + 2. * param.s2.powi(3)
                                            + 3. * param.s12
                                                * (param.s1.powi(2) + param.s1 * param.s2
                                                    - param.s2.powi(2))
                                            - param.s1.powi(3))
                                        + param.s2
                                            * (-3. * param.s12.powi(3)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1 + param.s2)
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (3. * param.s1.powi(2)
                                                        + -10. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))))
                                + 6. * param.m1_2.powi(2)
                                    * (2.
                                        * param.m2_2
                                        * param.s2
                                        * (-2. * param.s12.powi(4)
                                            + 3. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + 3. * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (-7. * param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + -7. * param.s2.powi(3)))
                                        + param.s2.powi(2)
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (3. * param.s1 + -7. * param.s2)
                                                + (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1 + 2. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (-4. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + 4. * param.s1.powi(2) * param.s2
                                                        + -6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3)))
                                        + param.m2_2.powi(2)
                                            * (param.s1.powi(4)
                                                + param.s12.powi(4)
                                                + 11. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + -14. * param.s1 * param.s2.powi(3)
                                                + -4. * param.s2.powi(4)
                                                + param.s12.powi(3)
                                                    * (-4. * param.s1 + param.s2)
                                                + param.s12.powi(2)
                                                    * (6. * param.s1.powi(2)
                                                        + 9. * param.s1 * param.s2
                                                        + -9. * param.s2.powi(2))
                                                + param.s12
                                                    * (-4. * param.s1.powi(3)
                                                        + -21. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 11. * param.s2.powi(3))))
                                + param.s1
                                    * (5.
                                        * param.m2_2.powi(4)
                                        * (2. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 2. * param.s1.powi(2) * param.s2
                                            + -3. * param.s12.powi(2) * param.s2
                                            + -3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s12
                                                * (param.s1 * param.s2 + param.s2.powi(2)
                                                    - param.s1.powi(2))
                                            - param.s2.powi(3))
                                        + param.s12
                                            * param.s2.powi(2)
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (3. * param.s1 + -7. * param.s2)
                                                + (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1 + 2. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (-4. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + 4. * param.s1.powi(2) * param.s2
                                                        + -6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3)))
                                        + 2. * param.m2_2
                                            * param.s2
                                            * (-3. * param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + 3. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + -30. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                + -6.
                                                    * param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -3. * param.s1.powi(2) * param.s2
                                                        + -3. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3)))
                                        + -4.
                                            * param.m2_2.powi(3)
                                            * (2. * param.s12.powi(4)
                                                + -3.
                                                    * param.s12.powi(3)
                                                    * (param.s1 + param.s2)
                                                + -3.
                                                    * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + -6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12
                                                    * (7. * param.s1.powi(3)
                                                        + -12. * param.s1.powi(2) * param.s2
                                                        + -12. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3)))
                                        + 3. * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -3.
                                                    * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + -6. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + -3.
                                                    * param.s12
                                                    * (param.s1.powi(4)
                                                        + -4. * param.s1.powi(3) * param.s2
                                                        + -6.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 10. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4))))
                                + param.m1_2
                                    * (4.
                                        * param.m2_2.powi(3)
                                        * (-4. * param.s1.powi(4)
                                            + param.s12.powi(4)
                                            + param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                            + -14. * param.s1.powi(3) * param.s2
                                            + 6. * param.s1.powi(2) * param.s2.powi(2)
                                            + 11. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4)
                                            + param.s12.powi(2)
                                                * (-9. * param.s1.powi(2)
                                                    + 9. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + param.s12
                                                * (11. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -21. * param.s1 * param.s2.powi(2)
                                                    + -4. * param.s2.powi(3)))
                                        + param.s2.powi(2)
                                            * (-3. * param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (-21. * param.s1 + 11. * param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (12. * param.s1.powi(2)
                                                        + 7. * param.s1 * param.s2
                                                        + -7. * param.s2.powi(2))
                                                + 6. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (21. * param.s1.powi(2)
                                                        + 28. * param.s1 * param.s2
                                                        - param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(4)
                                                    * (3. * param.s1 + param.s2))
                                        + 6. * param.m2_2
                                            * param.s2
                                            * (param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (-6. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -15. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 3. * param.s12
                                                    * (param.s1.powi(4)
                                                        + -10. * param.s1.powi(3) * param.s2
                                                        + 6. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 4. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + -3.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -6.
                                                    * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -5. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 2. * param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                - param.s12
                                                    * (11. * param.s1.powi(4)
                                                        + 14. * param.s1.powi(3) * param.s2
                                                        + -90.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 14. * param.s1 * param.s2.powi(3)
                                                        + 11. * param.s2.powi(4))
                                                - param.s12.powi(4) * (param.s1 + param.s2)))))
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.008333333333333333
            * std::f64::consts::PI
            * param.s2.powi(-2)
            * param.lambda_s12_sqrt.powi(-9)
            * ((param.m0_2.powi(4)
                * (param.s12.powi(5)
                    + -5. * param.s12.powi(4) * (param.s1 + 4. * param.s2)
                    + 2. * param.s12.powi(3)
                        * (5. * param.s1.powi(2)
                            + 26. * param.s1 * param.s2
                            + -110. * param.s2.powi(2))
                    + param.s12
                        * (param.s1 - param.s2).powi(2)
                        * (5. * param.s1.powi(2)
                            + 6. * param.s1 * param.s2
                            + 155. * param.s2.powi(2))
                    + param.s12.powi(2)
                        * (-10. * param.s1.powi(3)
                            + -36. * param.s1.powi(2) * param.s2
                            + 94. * param.s1 * param.s2.powi(2)
                            + 80. * param.s2.powi(3))
                    - (param.s1 + -4. * param.s2) * (param.s1 - param.s2).powi(4))
                + param.m2_2.powi(4)
                    * (param.s12.powi(5)
                        + 23. * param.s1.powi(4) * param.s2
                        + 398. * param.s1.powi(3) * param.s2.powi(2)
                        + 398. * param.s1.powi(2) * param.s2.powi(3)
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
                                + -462. * param.s1.powi(2) * param.s2.powi(2)
                                + -64. * param.s1 * param.s2.powi(3)
                                + 5. * param.s2.powi(4))
                        - param.s2.powi(5)
                        - param.s1.powi(5))
                + param.m2_2.powi(2)
                    * param.s2.powi(2)
                    * (6. * param.s12.powi(5)
                        + -15. * param.s12.powi(4) * (5. * param.s1 + 2. * param.s2)
                        + param.s12.powi(3)
                            * (-50. * param.s1.powi(2)
                                + 132. * param.s1 * param.s2
                                + 60. * param.s2.powi(2))
                        + 6. * param.s12.powi(2)
                            * (90. * param.s1.powi(3)
                                + -216. * param.s1.powi(2) * param.s2
                                + 9. * param.s1 * param.s2.powi(2)
                                + -10. * param.s2.powi(3))
                        + (param.s1 - param.s2).powi(2)
                            * (239. * param.s1.powi(3)
                                + 716. * param.s1.powi(2) * param.s2
                                + 81. * param.s1 * param.s2.powi(2)
                                + -6. * param.s2.powi(3))
                        + param.s12
                            * (-660. * param.s1.powi(4)
                                + 956. * param.s1.powi(3) * param.s2
                                + 798. * param.s1.powi(2) * param.s2.powi(2)
                                + -204. * param.s1 * param.s2.powi(3)
                                + 30. * param.s2.powi(4))
                        + -10.
                            * param.m1_2.powi(2)
                            * (-29. * param.s1.powi(3)
                                + 29. * param.s12.powi(3)
                                + -223. * param.s1.powi(2) * param.s2
                                + -223. * param.s1 * param.s2.powi(2)
                                + -29. * param.s2.powi(3)
                                + -87. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (87. * param.s1.powi(2)
                                        + 310. * param.s1 * param.s2
                                        + 87. * param.s2.powi(2)))
                        + 5. * param.m1_2
                            * (-107. * param.s1.powi(4)
                                + 9. * param.s12.powi(4)
                                + 4. * param.s12.powi(3) * (20. * param.s1 + -9. * param.s2)
                                + -466. * param.s1.powi(3) * param.s2
                                + 254. * param.s1.powi(2) * param.s2.powi(2)
                                + 310. * param.s1 * param.s2.powi(3)
                                + 9. * param.s2.powi(4)
                                + -6.
                                    * param.s12.powi(2)
                                    * (49. * param.s1.powi(2)
                                        + -25. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2))
                                + 4. * param.s12
                                    * (78. * param.s1.powi(3)
                                        + 88. * param.s1.powi(2) * param.s2
                                        + -135. * param.s1 * param.s2.powi(2)
                                        + -9. * param.s2.powi(3))))
                + param.s2.powi(4)
                    * (param.s12.powi(5)
                        + (param.s1 - param.s2).powi(4) * (4. * param.s1 - param.s2)
                        + -420. * param.m1_2.powi(4) * (param.s12 - param.s2 - param.s1)
                        + -5. * param.s12.powi(4) * (4. * param.s1 + param.s2)
                        + param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (155. * param.s1.powi(2)
                                + 6. * param.s1 * param.s2
                                + 5. * param.s2.powi(2))
                        + param.s12.powi(3)
                            * (-220. * param.s1.powi(2)
                                + 52. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        + 30.
                            * param.m1_2.powi(3)
                            * (-33. * param.s1.powi(2)
                                + 10. * param.s1 * param.s12
                                + 23. * param.s12.powi(2)
                                + 10. * param.s1 * param.s2
                                + -46. * param.s12 * param.s2
                                + 23. * param.s2.powi(2))
                        + 2. * param.s12.powi(2)
                            * (40. * param.s1.powi(3)
                                + 47. * param.s1.powi(2) * param.s2
                                + -18. * param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3))
                        + -10.
                            * param.m1_2.powi(2)
                            * (29. * param.s12.powi(3)
                                + 3. * param.s12.powi(2) * (40. * param.s1 + -29. * param.s2)
                                + param.s12
                                    * (-75. * param.s1.powi(2)
                                        + -104. * param.s1 * param.s2
                                        + 87. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(2)
                                    * (74. * param.s1 + 29. * param.s2))
                        + 5. * param.m1_2
                            * (3. * param.s12.powi(4)
                                + 4. * param.s12.powi(3) * (26. * param.s1 + -3. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (42. * param.s1.powi(2)
                                        + -91. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (39. * param.s1.powi(3)
                                        + -29. * param.s1.powi(2) * param.s2
                                        + -13. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3)
                                    * (35. * param.s1 + 3. * param.s2)))
                + param.m2_2.powi(3)
                    * param.s2
                    * (19. * param.s1.powi(5)
                        + -4. * param.s12.powi(5)
                        + 618. * param.s1.powi(4) * param.s2
                        + 318. * param.s1.powi(3) * param.s2.powi(2)
                        + -882. * param.s1.powi(2) * param.s2.powi(3)
                        + -77. * param.s1 * param.s2.powi(4)
                        + 4. * param.s2.powi(5)
                        + 5. * param.s12.powi(4) * (7. * param.s1 + 4. * param.s2)
                        + -4.
                            * param.s12.powi(3)
                            * (25. * param.s1.powi(2)
                                + 7. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        + 2. * param.s12.powi(2)
                            * (65. * param.s1.powi(3)
                                + 307. * param.s1.powi(2) * param.s2
                                + -63. * param.s1 * param.s2.powi(2)
                                + 20. * param.s2.powi(3))
                        + -4.
                            * param.s12
                            * (20. * param.s1.powi(4)
                                + 306. * param.s1.powi(3) * param.s2
                                + -92. * param.s1.powi(2) * param.s2.powi(2)
                                + -49. * param.s1 * param.s2.powi(3)
                                + 5. * param.s2.powi(4))
                        + -5.
                            * param.m1_2
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 142. * param.s1.powi(3) * param.s2
                                + 382. * param.s1.powi(2) * param.s2.powi(2)
                                + 142. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + 83. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (3. * param.s1.powi(3)
                                        + 74. * param.s1.powi(2) * param.s2
                                        + 74. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3))))
                + param.m2_2
                    * param.s2.powi(3)
                    * (-4. * param.s12.powi(5)
                        + 5. * param.s12.powi(4) * (13. * param.s1 + 4. * param.s2)
                        + 4. * param.s12.powi(3)
                            * (90. * param.s1.powi(2)
                                + -37. * param.s1 * param.s2
                                + -10. * param.s2.powi(2))
                        + (param.s1 - param.s2).powi(3)
                            * (159. * param.s1.powi(2)
                                + 35. * param.s1 * param.s2
                                + -4. * param.s2.powi(2))
                        + param.s12.powi(2)
                            * (-740. * param.s1.powi(3)
                                + 534. * param.s1.powi(2) * param.s2
                                + 54. * param.s1 * param.s2.powi(2)
                                + 40. * param.s2.powi(3))
                        + 4. * param.s12
                            * (40. * param.s1.powi(4)
                                + 159. * param.s1.powi(3) * param.s2
                                + -213. * param.s1.powi(2) * param.s2.powi(2)
                                + 19. * param.s1 * param.s2.powi(3)
                                + -5. * param.s2.powi(4))
                        + -30.
                            * param.m1_2.powi(3)
                            * (23. * param.s1.powi(2)
                                + 23. * param.s12.powi(2)
                                + 66. * param.s1 * param.s2
                                + 23. * param.s2.powi(2)
                                + -46. * param.s12 * (param.s1 + param.s2))
                        + 10.
                            * param.m1_2.powi(2)
                            * (149. * param.s1.powi(3)
                                + 58. * param.s12.powi(3)
                                + 3. * param.s12.powi(2) * (11. * param.s1 + -58. * param.s2)
                                + 148. * param.s1.powi(2) * param.s2
                                + -239. * param.s1 * param.s2.powi(2)
                                + -58. * param.s2.powi(3)
                                + param.s12
                                    * (-240. * param.s1.powi(2)
                                        + 206. * param.s1 * param.s2
                                        + 174. * param.s2.powi(2)))
                        + -5.
                            * param.m1_2
                            * (9. * param.s12.powi(4)
                                + 4. * param.s12.powi(3) * (49. * param.s1 + -9. * param.s2)
                                + -6.
                                    * param.s12.powi(2)
                                    * (38. * param.s1.powi(2)
                                        + 33. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (191. * param.s1.powi(2)
                                        + 212. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (42. * param.s1.powi(3)
                                        + -191. * param.s1.powi(2) * param.s2
                                        + 48. * param.s1 * param.s2.powi(2)
                                        + 9. * param.s2.powi(3))))
                + param.m0_2.powi(3)
                    * (param.m2_2
                        * (-4. * param.s12.powi(5)
                            + 5. * param.s12.powi(4) * (4. * param.s1 + 13. * param.s2)
                            + (param.s1 - param.s2).powi(3)
                                * (4. * param.s1.powi(2)
                                    + -35. * param.s1 * param.s2
                                    + -159. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (10. * param.s1.powi(2)
                                    + 37. * param.s1 * param.s2
                                    + -90. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (40. * param.s1.powi(3)
                                    + 54. * param.s1.powi(2) * param.s2
                                    + 534. * param.s1 * param.s2.powi(2)
                                    + -740. * param.s2.powi(3))
                            + param.s12
                                * (-20. * param.s1.powi(4)
                                    + 76. * param.s1.powi(3) * param.s2
                                    + -852. * param.s1.powi(2) * param.s2.powi(2)
                                    + 636. * param.s1 * param.s2.powi(3)
                                    + 160. * param.s2.powi(4)))
                        + param.s2
                            * (-14. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (11. * param.s1 + -61. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (5. * param.s1.powi(2)
                                        + 172. * param.s1 * param.s2
                                        + 155. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (-80. * param.s1.powi(2)
                                        + 272. * param.s1 * param.s2
                                        + 320. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (25. * param.s1.powi(3)
                                        + 182. * param.s1.powi(2) * param.s2
                                        + -623. * param.s1 * param.s2.powi(2)
                                        + 160. * param.s2.powi(3))
                                + 5. * param.m1_2
                                    * (3. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (3. * param.s1 + -26. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 35. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -91. * param.s1 * param.s2
                                                + 42. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (3. * param.s1.powi(3)
                                                + -13. * param.s1.powi(2) * param.s2
                                                + -29. * param.s1 * param.s2.powi(2)
                                                + 39. * param.s2.powi(3)))
                                - (param.s1 - param.s2).powi(4) * (param.s1 + 11. * param.s2)))
                + param.m0_2.powi(2)
                    * (param.m2_2.powi(2)
                        * (6. * param.s12.powi(5)
                            + -15. * param.s12.powi(4) * (2. * param.s1 + 5. * param.s2)
                            + 2. * param.s12.powi(3)
                                * (30. * param.s1.powi(2)
                                    + 66. * param.s1 * param.s2
                                    + -25. * param.s2.powi(2))
                            + -6.
                                * param.s12.powi(2)
                                * (10. * param.s1.powi(3)
                                    + -9. * param.s1.powi(2) * param.s2
                                    + 216. * param.s1 * param.s2.powi(2)
                                    + -90. * param.s2.powi(3))
                            + param.s12
                                * (30. * param.s1.powi(4)
                                    + -204. * param.s1.powi(3) * param.s2
                                    + 798. * param.s1.powi(2) * param.s2.powi(2)
                                    + 956. * param.s1 * param.s2.powi(3)
                                    + -660. * param.s2.powi(4))
                            - (param.s1 - param.s2).powi(2)
                                * (6. * param.s1.powi(3)
                                    + -81. * param.s1.powi(2) * param.s2
                                    + -716. * param.s1 * param.s2.powi(2)
                                    + -239. * param.s2.powi(3)))
                        + param.s2.powi(2)
                            * (-94. * param.s12.powi(5)
                                + 95. * param.s12.powi(4) * (param.s1 + param.s2)
                                + 9. * (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (40. * param.s1.powi(2)
                                        + 169. * param.s1 * param.s2
                                        + 40. * param.s2.powi(2))
                                + 6. * param.s12.powi(3)
                                    * (45. * param.s1.powi(2)
                                        + -208. * param.s1 * param.s2
                                        + 45. * param.s2.powi(2))
                                + -8.
                                    * param.s12.powi(2)
                                    * (55. * param.s1.powi(3)
                                        + -103. * param.s1.powi(2) * param.s2
                                        + -103. * param.s1 * param.s2.powi(2)
                                        + 55. * param.s2.powi(3))
                                + -10.
                                    * param.m1_2.powi(2)
                                    * (29. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (-87. * param.s1 + 120. * param.s2)
                                        + param.s12
                                            * (87. * param.s1.powi(2)
                                                + -104. * param.s1 * param.s2
                                                + -75. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (29. * param.s1 + 74. * param.s2))
                                + 5. * param.m1_2
                                    * (75. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (46. * param.s1 + -29. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (51. * param.s1.powi(2)
                                                + 181. * param.s1 * param.s2
                                                + -192. * param.s2.powi(2))
                                        + 24.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -22. * param.s1.powi(2) * param.s2
                                                + 15. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (41. * param.s1 + 73. * param.s2)))
                        + param.m2_2
                            * param.s2
                            * (24. * param.s12.powi(5)
                                + -75. * param.s12.powi(4) * (param.s1 + -4. * param.s2)
                                + param.s12.powi(3)
                                    * (60. * param.s1.powi(2)
                                        + 748. * param.s1 * param.s2
                                        + -860. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (21. * param.s1.powi(2)
                                        + 365. * param.s1 * param.s2
                                        + 184. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (15. * param.s1.powi(3)
                                        + -1047. * param.s1.powi(2) * param.s2
                                        + 818. * param.s1 * param.s2.powi(2)
                                        + 270. * param.s2.powi(3))
                                + -12.
                                    * param.s12
                                    * (5. * param.s1.powi(4)
                                        + -62. * param.s1.powi(3) * param.s2
                                        + -136. * param.s1.powi(2) * param.s2.powi(2)
                                        + 208. * param.s1 * param.s2.powi(3)
                                        + -15. * param.s2.powi(4))
                                + -5.
                                    * param.m1_2
                                    * (9. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (9. * param.s1 + -49. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -33. * param.s1 * param.s2
                                                + -38. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 212. * param.s1 * param.s2
                                                + 191. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (9. * param.s1.powi(3)
                                                + 48. * param.s1.powi(2) * param.s2
                                                + -191. * param.s1 * param.s2.powi(2)
                                                + 42. * param.s2.powi(3)))))
                + param.m0_2
                    * (param.m2_2.powi(3)
                        * (4. * param.s1.powi(5)
                            + -4. * param.s12.powi(5)
                            + -77. * param.s1.powi(4) * param.s2
                            + -882. * param.s1.powi(3) * param.s2.powi(2)
                            + 318. * param.s1.powi(2) * param.s2.powi(3)
                            + 618. * param.s1 * param.s2.powi(4)
                            + 19. * param.s2.powi(5)
                            + 5. * param.s12.powi(4) * (4. * param.s1 + 7. * param.s2)
                            + -4.
                                * param.s12.powi(3)
                                * (10. * param.s1.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + 25. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (20. * param.s1.powi(3)
                                    + -63. * param.s1.powi(2) * param.s2
                                    + 307. * param.s1 * param.s2.powi(2)
                                    + 65. * param.s2.powi(3))
                            + -4.
                                * param.s12
                                * (5. * param.s1.powi(4)
                                    + -49. * param.s1.powi(3) * param.s2
                                    + -92. * param.s1.powi(2) * param.s2.powi(2)
                                    + 306. * param.s1 * param.s2.powi(3)
                                    + 20. * param.s2.powi(4)))
                        + param.m2_2.powi(2)
                            * param.s2
                            * (-6. * param.s12.powi(5)
                                + -15. * param.s12.powi(4) * (param.s1 + param.s2)
                                + 8. * param.s12.powi(3)
                                    * (15. * param.s1.powi(2)
                                        + -124. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2))
                                + -6.
                                    * param.s12.powi(2)
                                    * (35. * param.s1.powi(3)
                                        + -186. * param.s1.powi(2) * param.s2
                                        + -186. * param.s1 * param.s2.powi(2)
                                        + 35. * param.s2.powi(3))
                                + 2. * param.s12
                                    * (75. * param.s1.powi(4)
                                        + 402. * param.s1.powi(3) * param.s2
                                        + -1874. * param.s1.powi(2) * param.s2.powi(2)
                                        + 402. * param.s1 * param.s2.powi(3)
                                        + 75. * param.s2.powi(4))
                                + 5. * param.m1_2
                                    * (9. * param.s1.powi(4)
                                        + 9. * param.s12.powi(4)
                                        + 310. * param.s1.powi(3) * param.s2
                                        + 254. * param.s1.powi(2) * param.s2.powi(2)
                                        + -466. * param.s1 * param.s2.powi(3)
                                        + -107. * param.s2.powi(4)
                                        + param.s12.powi(3)
                                            * (-36. * param.s1 + 80. * param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 25. * param.s1 * param.s2
                                                + -49. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (9. * param.s1.powi(3)
                                                + 135. * param.s1.powi(2) * param.s2
                                                + -88. * param.s1 * param.s2.powi(2)
                                                + -78. * param.s2.powi(3)))
                                - (param.s1 - param.s2).powi(2)
                                    * (39. * param.s1.powi(3)
                                        + 991. * param.s1.powi(2) * param.s2
                                        + 991. * param.s1 * param.s2.powi(2)
                                        + 39. * param.s2.powi(3)))
                        + param.s2.powi(3)
                            * (-14. * param.s12.powi(5)
                                + param.s12.powi(4) * (-305. * param.s1 + 55. * param.s2)
                                + 30.
                                    * param.m1_2.powi(3)
                                    * (23. * param.s1.powi(2)
                                        + -46. * param.s1 * param.s12
                                        + 23. * param.s12.powi(2)
                                        + 10. * param.s1 * param.s2
                                        + 10. * param.s12 * param.s2
                                        + -33. * param.s2.powi(2))
                                + 16.
                                    * param.s12.powi(3)
                                    * (20. * param.s1.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (155. * param.s1.powi(2)
                                        + 172. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (160. * param.s1.powi(3)
                                        + -623. * param.s1.powi(2) * param.s2
                                        + 182. * param.s1 * param.s2.powi(2)
                                        + 25. * param.s2.powi(3))
                                + -10.
                                    * param.m1_2.powi(2)
                                    * (104. * param.s12.powi(3)
                                        + -105. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 103.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12
                                            * (51. * param.s1.powi(2)
                                                + -194. * param.s1 * param.s2
                                                + 51. * param.s2.powi(2)))
                                + 5. * param.m1_2
                                    * (75. * param.s12.powi(4)
                                        + 4. * param.s12.powi(3)
                                            * (29. * param.s1 + -46. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (73. * param.s1 + 41. * param.s2)
                                        + param.s12.powi(2)
                                            * (-384. * param.s1.powi(2)
                                                + 362. * param.s1 * param.s2
                                                + 102. * param.s2.powi(2))
                                        + 24.
                                            * param.s12
                                            * (5. * param.s1.powi(3)
                                                + 15. * param.s1.powi(2) * param.s2
                                                + -22. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - (param.s1 - param.s2).powi(4) * (11. * param.s1 + param.s2))
                        + param.m2_2
                            * param.s2.powi(2)
                            * (24. * param.s12.powi(5)
                                + 75. * param.s12.powi(4) * (4. * param.s1 - param.s2)
                                + param.s12.powi(3)
                                    * (-860. * param.s1.powi(2)
                                        + 748. * param.s1 * param.s2
                                        + 60. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (270. * param.s1.powi(3)
                                        + 818. * param.s1.powi(2) * param.s2
                                        + -1047. * param.s1 * param.s2.powi(2)
                                        + 15. * param.s2.powi(3))
                                + 12.
                                    * param.s12
                                    * (15. * param.s1.powi(4)
                                        + -208. * param.s1.powi(3) * param.s2
                                        + 136. * param.s1.powi(2) * param.s2.powi(2)
                                        + 62. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                + 10.
                                    * param.m1_2.powi(2)
                                    * (-58. * param.s1.powi(3)
                                        + 58. * param.s12.powi(3)
                                        + -239. * param.s1.powi(2) * param.s2
                                        + 148. * param.s1 * param.s2.powi(2)
                                        + 149. * param.s2.powi(3)
                                        + param.s12.powi(2)
                                            * (-174. * param.s1 + 33. * param.s2)
                                        + 2. * param.s12
                                            * (87. * param.s1.powi(2)
                                                + 103. * param.s1 * param.s2
                                                + -120. * param.s2.powi(2)))
                                + -20.
                                    * param.m1_2
                                    * (21. * param.s12.powi(4)
                                        + -26. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-48. * param.s1.powi(2)
                                                + 266. * param.s1 * param.s2
                                                + -48. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (45. * param.s1.powi(3)
                                                + -91. * param.s1.powi(2) * param.s2
                                                + -91. * param.s1 * param.s2.powi(2)
                                                + 45. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (37. * param.s1.powi(2)
                                                + 132. * param.s1 * param.s2
                                                + 37. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(3)
                                    * (184. * param.s1.powi(2)
                                        + 365. * param.s1 * param.s2
                                        + 21. * param.s2.powi(2)))))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s2.powi(2)
                    * (7.
                        * param.m1_2.powi(5)
                        * param.s2.powi(3)
                        * (param.s1 + param.s2 - param.s12)
                        + param.m0_2.powi(5)
                            * param.s12
                            * (2. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + 3. * param.s12 * (param.s1 - param.s2) * param.s2
                                + param.s12.powi(2) * (-3. * param.s1 + 2. * param.s2))
                        + 5. * param.m1_2.powi(4)
                            * param.s2.powi(2)
                            * (param.s2
                                * (-4. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -6. * param.s2)
                                    + param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                                - param.m2_2
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)
                                        + -6. * param.s12 * (param.s1 + param.s2)))
                        + 10.
                            * param.m1_2.powi(3)
                            * param.s2
                            * (param.s2.powi(2)
                                * (-3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + (param.s1 - param.s2).powi(2) * (2. * param.s1 + param.s2)
                                    + param.s12
                                        * (2. * param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - param.s12.powi(3))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (2. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + 2. * param.s1.powi(2) * param.s2
                                        + -3. * param.s12.powi(2) * param.s2
                                        + -3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s12
                                            * (param.s1 * param.s2 + param.s2.powi(2)
                                                - param.s1.powi(2))
                                        - param.s2.powi(3))
                                + param.m2_2.powi(2)
                                    * (param.s1.powi(3)
                                        + 6. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)
                                        + 3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(3)))
                        + param.m0_2.powi(4)
                            * (param.m1_2
                                * (-4. * param.s12.powi(4)
                                    + param.s12.powi(3) * (11. * param.s1 + -14. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + 11. * param.s2)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2)))
                                + param.s12
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-7. * param.s1 + 3. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + -6. * param.s1.powi(2) * param.s2
                                                + 4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + 3. * param.s2))
                                - param.m2_2
                                    * (6. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))))
                        + param.s1.powi(2)
                            * (param.s12
                                * (2. * param.s12.powi(3)
                                    + param.s12.powi(2) * (2. * param.s1 + -3. * param.s2)
                                    + -3. * param.s1 * param.s12 * (param.s1 - param.s2)
                                    - (param.s1 - param.s2).powi(3))
                                * param.s2.powi(3)
                                + 5. * param.m2_2.powi(4)
                                    * (-3. * param.s1 * param.s12.powi(2)
                                        + param.s12.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)
                                        + 3. * param.s12
                                            * (param.s1.powi(2) + param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s1.powi(3))
                                + 2. * param.m2_2.powi(2)
                                    * param.s2
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-7. * param.s1 + 3. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + -6. * param.s1.powi(2) * param.s2
                                                + 4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + 3. * param.s2))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (-4. * param.s1 + 6. * param.s2)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + 3. * param.s1.powi(2) * param.s2
                                                + -12. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - param.m2_2
                                    * param.s2.powi(2)
                                    * (6. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2)))
                                - param.m2_2.powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)
                                        + -6. * param.s12 * (param.s1 + param.s2)))
                        + -2.
                            * param.m1_2.powi(2)
                            * (-3.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (-4. * param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                    + -14. * param.s1.powi(3) * param.s2
                                    + 6. * param.s1.powi(2) * param.s2.powi(2)
                                    + 11. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2))
                                    + param.s12
                                        * (11. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -21. * param.s1 * param.s2.powi(2)
                                            + -4. * param.s2.powi(3)))
                                + param.m2_2.powi(3)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 16. * param.s1.powi(3) * param.s2
                                        + 36. * param.s1.powi(2) * param.s2.powi(2)
                                        + 16. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 3. * param.m2_2
                                    * param.s2.powi(2)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (6. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - param.s2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (11. * param.s1 + -4. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (2. * param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12
                                            * (-14. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -4. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (4. * param.s1 + param.s2)))
                        + param.m1_2
                            * param.s1
                            * (param.s2.powi(3)
                                * (-4. * param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (11. * param.s1 + param.s2)
                                    + param.s12.powi(3) * (-14. * param.s1 + 11. * param.s2)
                                    + param.s12.powi(2)
                                        * (6. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2)))
                                + -5.
                                    * param.m2_2.powi(4)
                                    * (param.s12.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3))
                                + 6. * param.m2_2.powi(2)
                                    * param.s2
                                    * (-2. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (-7. * param.s1.powi(3)
                                                + 12. * param.s1.powi(2) * param.s2
                                                + 12. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3)))
                                + 4. * param.m2_2
                                    * param.s2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (3. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 2. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + 4. * param.s1.powi(2) * param.s2
                                                + -6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 4. * param.m2_2.powi(3)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 11. * param.s1.powi(3) * param.s2
                                        + 6. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14. * param.s1 * param.s2.powi(3)
                                        + -4. * param.s2.powi(4)
                                        + param.s12.powi(3) * (-4. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 9. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + param.s12
                                            * (-4. * param.s1.powi(3)
                                                + -21. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3))))
                        + param.m0_2.powi(3)
                            * (2.
                                * param.m1_2.powi(2)
                                * (param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(3) * (param.s1 + 4. * param.s2)
                                    + param.s12.powi(3) * (-4. * param.s1 + 11. * param.s2)
                                    + 3. * param.s12.powi(2)
                                        * (2. * param.s1.powi(2)
                                            + -7. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12
                                        * (-4. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -14. * param.s2.powi(3)))
                                + 2. * param.m2_2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (3. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 2. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + 4. * param.s1.powi(2) * param.s2
                                                + -6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 2. * param.m2_2
                                    * (-3. * param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (8. * param.s1.powi(2)
                                                + -30. * param.s1 * param.s2
                                                + 8. * param.s2.powi(2))
                                        + -6.
                                            * param.s12.powi(2)
                                            * (2. * param.s1.powi(3)
                                                + -3. * param.s1.powi(2) * param.s2
                                                + -3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                + param.s12
                                    * (param.s12.powi(5)
                                        + -3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + -6. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + -3.
                                            * param.s12
                                            * (param.s1.powi(4)
                                                + -4. * param.s1.powi(3) * param.s2
                                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4)))
                                + param.m1_2
                                    * (-3. * param.s12.powi(5)
                                        + param.s12.powi(4) * (11. * param.s1 + -21. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + -28. * param.s1 * param.s2
                                                + -21. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-14. * param.s1.powi(2)
                                                + 14. * param.s1 * param.s2
                                                + 24. * param.s2.powi(2))
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 6. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 4. * param.m2_2
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1 + 3. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + -6. * param.s1.powi(2) * param.s2
                                                        + 4. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1 + 3. * param.s2))
                                        - (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 3. * param.s2)))
                        + param.m0_2.powi(2)
                            * (-10.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (param.s12.powi(3)
                                    + -3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + -3. * param.s1 * param.s2
                                            + -2. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2) * (param.s1 + 2. * param.s2))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (6. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (-6. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -10. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + param.s12
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (-6. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -10. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + -6.
                                    * param.m1_2.powi(2)
                                    * (param.m2_2
                                        * (param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (-4. * param.s1 + 6. * param.s2)
                                            + param.s12.powi(2)
                                                * (6. * param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (2. * param.s1.powi(3)
                                                    + 3. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3)))
                                        - param.s2
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1 + 3. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + -6. * param.s1.powi(2) * param.s2
                                                        + 4. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1 + 3. * param.s2)))
                                + -3.
                                    * param.m1_2
                                    * (2.
                                        * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(4)
                                            + -3. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + -3.
                                                * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (7. * param.s1.powi(3)
                                                    + -12. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3)))
                                        + -2.
                                            * param.m2_2
                                            * (param.s12.powi(5)
                                                + -3.
                                                    * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + -6. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + -3.
                                                    * param.s12
                                                    * (param.s1.powi(4)
                                                        + -4. * param.s1.powi(3) * param.s2
                                                        + -6.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 10. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4)))
                                        - param.s2
                                            * (-3. * param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + 3. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + -30. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                + -6.
                                                    * param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -3. * param.s1.powi(2) * param.s2
                                                        + -3. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3))))
                                - param.m2_2
                                    * (param.s12.powi(6)
                                        + 36.
                                            * param.s1
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * param.s2
                                            * (param.s1 + param.s2)
                                        + -9.
                                            * param.s12.powi(4)
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (4. * param.s1.powi(3)
                                                + -9. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + -9.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(4)
                                                + 4. * param.s1.powi(3) * param.s2
                                                + -14. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                + param.s2.powi(4))))
                        + param.m0_2
                            * (5.
                                * param.m1_2.powi(4)
                                * param.s2.powi(2)
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -4. * param.s2.powi(2)
                                    + param.s12 * (-6. * param.s1 + param.s2))
                                + 10.
                                    * param.m1_2.powi(3)
                                    * param.s2
                                    * (2.
                                        * param.m2_2
                                        * (-3. * param.s1 * param.s12.powi(2)
                                            + param.s12.powi(3)
                                            + -3. * param.s1.powi(2) * param.s2
                                            + 2. * param.s1 * param.s2.powi(2)
                                            + 2. * param.s2.powi(3)
                                            + 3. * param.s12
                                                * (param.s1.powi(2) + param.s1 * param.s2
                                                    - param.s2.powi(2))
                                            - param.s1.powi(3))
                                        + param.s2
                                            * (-3. * param.s12.powi(3)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1 + param.s2)
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (3. * param.s1.powi(2)
                                                        + -10. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))))
                                + 6. * param.m1_2.powi(2)
                                    * (2.
                                        * param.m2_2
                                        * param.s2
                                        * (-2. * param.s12.powi(4)
                                            + 3. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + 3. * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (-7. * param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + -7. * param.s2.powi(3)))
                                        + param.s2.powi(2)
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (3. * param.s1 + -7. * param.s2)
                                                + (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1 + 2. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (-4. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + 4. * param.s1.powi(2) * param.s2
                                                        + -6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3)))
                                        + param.m2_2.powi(2)
                                            * (param.s1.powi(4)
                                                + param.s12.powi(4)
                                                + 11. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + -14. * param.s1 * param.s2.powi(3)
                                                + -4. * param.s2.powi(4)
                                                + param.s12.powi(3)
                                                    * (-4. * param.s1 + param.s2)
                                                + param.s12.powi(2)
                                                    * (6. * param.s1.powi(2)
                                                        + 9. * param.s1 * param.s2
                                                        + -9. * param.s2.powi(2))
                                                + param.s12
                                                    * (-4. * param.s1.powi(3)
                                                        + -21. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 11. * param.s2.powi(3))))
                                + param.s1
                                    * (5.
                                        * param.m2_2.powi(4)
                                        * (2. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 2. * param.s1.powi(2) * param.s2
                                            + -3. * param.s12.powi(2) * param.s2
                                            + -3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s12
                                                * (param.s1 * param.s2 + param.s2.powi(2)
                                                    - param.s1.powi(2))
                                            - param.s2.powi(3))
                                        + param.s12
                                            * param.s2.powi(2)
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (3. * param.s1 + -7. * param.s2)
                                                + (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1 + 2. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (-4. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + 4. * param.s1.powi(2) * param.s2
                                                        + -6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3)))
                                        + 2. * param.m2_2
                                            * param.s2
                                            * (-3. * param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + 3. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + -30. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                + -6.
                                                    * param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -3. * param.s1.powi(2) * param.s2
                                                        + -3. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3)))
                                        + -4.
                                            * param.m2_2.powi(3)
                                            * (2. * param.s12.powi(4)
                                                + -3.
                                                    * param.s12.powi(3)
                                                    * (param.s1 + param.s2)
                                                + -3.
                                                    * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + -6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12
                                                    * (7. * param.s1.powi(3)
                                                        + -12. * param.s1.powi(2) * param.s2
                                                        + -12. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3)))
                                        + 3. * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -3.
                                                    * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + -6. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + -3.
                                                    * param.s12
                                                    * (param.s1.powi(4)
                                                        + -4. * param.s1.powi(3) * param.s2
                                                        + -6.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 10. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4))))
                                + param.m1_2
                                    * (4.
                                        * param.m2_2.powi(3)
                                        * (-4. * param.s1.powi(4)
                                            + param.s12.powi(4)
                                            + param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                            + -14. * param.s1.powi(3) * param.s2
                                            + 6. * param.s1.powi(2) * param.s2.powi(2)
                                            + 11. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4)
                                            + param.s12.powi(2)
                                                * (-9. * param.s1.powi(2)
                                                    + 9. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + param.s12
                                                * (11. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -21. * param.s1 * param.s2.powi(2)
                                                    + -4. * param.s2.powi(3)))
                                        + param.s2.powi(2)
                                            * (-3. * param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (-21. * param.s1 + 11. * param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (12. * param.s1.powi(2)
                                                        + 7. * param.s1 * param.s2
                                                        + -7. * param.s2.powi(2))
                                                + 6. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (21. * param.s1.powi(2)
                                                        + 28. * param.s1 * param.s2
                                                        - param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(4)
                                                    * (3. * param.s1 + param.s2))
                                        + 6. * param.m2_2
                                            * param.s2
                                            * (param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (-6. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -15. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 3. * param.s12
                                                    * (param.s1.powi(4)
                                                        + -10. * param.s1.powi(3) * param.s2
                                                        + 6. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 4. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + -3.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -6.
                                                    * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -5. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 2. * param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                - param.s12
                                                    * (11. * param.s1.powi(4)
                                                        + 14. * param.s1.powi(3) * param.s2
                                                        + -90.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 14. * param.s1 * param.s2.powi(3)
                                                        + 11. * param.s2.powi(4))
                                                - param.s12.powi(4) * (param.s1 + param.s2)))))
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.008333333333333333
            * std::f64::consts::PI
            * param.s12.powi(-4)
            * param.lambda_s12_sqrt.powi(-9)
            * ((-3. * param.m2_2.powi(4) * param.s1.powi(7)
                + 27. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12
                + 7. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12
                + -114. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(2)
                + -68. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(2)
                + -3. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(2)
                + 330. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(3)
                + 346. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(3)
                + 42. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(3)
                + -3. * param.m2_2 * param.s1.powi(7) * param.s12.powi(3)
                + -345. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(4)
                + -625. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(4)
                + -134. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(4)
                + 12. * param.m2_2 * param.s1.powi(6) * param.s12.powi(4)
                + 2. * param.s1.powi(7) * param.s12.powi(4)
                + 33. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(5)
                + 365. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(5)
                + 175. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(5)
                + -14. * param.m2_2 * param.s1.powi(5) * param.s12.powi(5)
                + -13. * param.s1.powi(6) * param.s12.powi(5)
                + 78. * param.m2_2.powi(4) * param.s1 * param.s12.powi(6)
                + 68. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(6)
                + -95. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(6)
                + -5. * param.m2_2 * param.s1.powi(4) * param.s12.powi(6)
                + 36. * param.s1.powi(5) * param.s12.powi(6)
                + -6. * param.m2_2.powi(4) * param.s12.powi(7)
                + -102. * param.m2_2.powi(3) * param.s1 * param.s12.powi(7)
                + 8. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(7)
                + 25. * param.m2_2 * param.s1.powi(3) * param.s12.powi(7)
                + -55. * param.s1.powi(4) * param.s12.powi(7)
                + 9. * param.m2_2.powi(3) * param.s12.powi(8)
                + 8. * param.m2_2.powi(2) * param.s1 * param.s12.powi(8)
                + -22. * param.m2_2 * param.s1.powi(2) * param.s12.powi(8)
                + 50. * param.s1.powi(3) * param.s12.powi(8)
                + 8. * param.m2_2 * param.s1 * param.s12.powi(9)
                + -27. * param.s1.powi(2) * param.s12.powi(9)
                + 8. * param.s1 * param.s12.powi(10)
                + 21. * param.m2_2.powi(4) * param.s1.powi(6) * param.s2
                + -112. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12 * param.s2
                + -44. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12 * param.s2
                + 206. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + 228. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(2) * param.s2
                + 6. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(2) * param.s2
                + 56. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + -244. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + 68. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(3) * param.s2
                + 36. * param.m2_2 * param.s1.powi(6) * param.s12.powi(3) * param.s2
                + 191. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + 976. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + 636. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + -12. * param.m2_2 * param.s1.powi(5) * param.s12.powi(4) * param.s2
                + -19. * param.s1.powi(6) * param.s12.powi(4) * param.s2
                + -280. * param.m2_2.powi(4) * param.s1 * param.s12.powi(5) * param.s2
                + -1124. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(5) * param.s2
                + -1404. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + -164. * param.m2_2 * param.s1.powi(4) * param.s12.powi(5) * param.s2
                + 68. * param.s1.powi(5) * param.s12.powi(5) * param.s2
                + 38. * param.m2_2.powi(4) * param.s12.powi(6) * param.s2
                + 260. * param.m2_2.powi(3) * param.s1 * param.s12.powi(6) * param.s2
                + 596. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + 196. * param.m2_2 * param.s1.powi(3) * param.s12.powi(6) * param.s2
                + -74. * param.s1.powi(4) * param.s12.powi(6) * param.s2
                + -52. * param.m2_2.powi(3) * param.s12.powi(7) * param.s2
                + 100. * param.m2_2.powi(2) * param.s1 * param.s12.powi(7) * param.s2
                + -24. * param.m2_2 * param.s1.powi(2) * param.s12.powi(7) * param.s2
                + -4. * param.s1.powi(3) * param.s12.powi(7) * param.s2
                + -2. * param.m2_2.powi(2) * param.s12.powi(8) * param.s2
                + -40. * param.m2_2 * param.s1 * param.s12.powi(8) * param.s2
                + 61. * param.s1.powi(2) * param.s12.powi(8) * param.s2
                + 8. * param.m2_2 * param.s12.powi(9) * param.s2
                + -40. * param.s1 * param.s12.powi(9) * param.s2
                + 8. * param.s12.powi(10) * param.s2
                + -63. * param.m2_2.powi(4) * param.s1.powi(5) * param.s2.powi(2)
                + 155. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12 * param.s2.powi(2)
                + 117. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12 * param.s2.powi(2)
                + -10.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -215.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 17.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -126.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -410.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -395.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -103. * param.m2_2 * param.s1.powi(5) * param.s12.powi(3) * param.s2.powi(2)
                + 347. * param.m2_2.powi(4) * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + 814.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 680.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 545. * param.m2_2 * param.s1.powi(4) * param.s12.powi(4) * param.s2.powi(2)
                + 92. * param.s1.powi(5) * param.s12.powi(4) * param.s2.powi(2)
                + -103. * param.m2_2.powi(4) * param.s12.powi(5) * param.s2.powi(2)
                + -113. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5) * param.s2.powi(2)
                + 644.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 360. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(2)
                + -90. * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(2)
                + 127. * param.m2_2.powi(3) * param.s12.powi(6) * param.s2.powi(2)
                + -293. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + -766. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(2)
                + -80. * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(2)
                + 27. * param.m2_2.powi(2) * param.s12.powi(7) * param.s2.powi(2)
                + -13. * param.m2_2 * param.s1 * param.s12.powi(7) * param.s2.powi(2)
                + 34. * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(2)
                + -23. * param.m2_2 * param.s12.powi(8) * param.s2.powi(2)
                + 72. * param.s1 * param.s12.powi(8) * param.s2.powi(2)
                + -28. * param.s12.powi(9) * param.s2.powi(2)
                + 105. * param.m2_2.powi(4) * param.s1.powi(4) * param.s2.powi(3)
                + -40. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12 * param.s2.powi(3)
                + -170. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12 * param.s2.powi(3)
                + -66.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -60.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -70.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -120. * param.m2_2.powi(4) * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + 104.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 340.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 110. * param.m2_2 * param.s1.powi(4) * param.s12.powi(3) * param.s2.powi(3)
                + 155. * param.m2_2.powi(4) * param.s12.powi(4) * param.s2.powi(3)
                + -220. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + -1226.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + -1060. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(3)
                + -155. * param.s1.powi(4) * param.s12.powi(4) * param.s2.powi(3)
                + -170. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2.powi(3)
                + 180. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + 364. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(3)
                + 280. * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(3)
                + -70. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2.powi(3)
                + 180. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2.powi(3)
                + 224. * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(3)
                + 30. * param.m2_2 * param.s12.powi(7) * param.s2.powi(3)
                + -20. * param.s1 * param.s12.powi(7) * param.s2.powi(3)
                + 55. * param.s12.powi(8) * param.s2.powi(3)
                + -105. * param.m2_2.powi(4) * param.s1.powi(3) * param.s2.powi(4)
                + -95. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12 * param.s2.powi(4)
                + 145. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12 * param.s2.powi(4)
                + -92. * param.m2_2.powi(4) * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + 210.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + 95.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -140. * param.m2_2.powi(4) * param.s12.powi(3) * param.s2.powi(4)
                + 268. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + 40.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -25. * param.m2_2 * param.s1.powi(3) * param.s12.powi(3) * param.s2.powi(4)
                + 135. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(4)
                + 98. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + 480. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(4)
                + 70. * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(4)
                + 85. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(4)
                + -182. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(4)
                + -335. * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(4)
                + -15. * param.m2_2 * param.s12.powi(6) * param.s2.powi(4)
                + -92. * param.s1 * param.s12.powi(6) * param.s2.powi(4)
                + -65. * param.s12.powi(7) * param.s2.powi(4)
                + 63. * param.m2_2.powi(4) * param.s1.powi(2) * param.s2.powi(5)
                + 88. * param.m2_2.powi(4) * param.s1 * param.s12 * param.s2.powi(5)
                + -72. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12 * param.s2.powi(5)
                + 76. * param.m2_2.powi(4) * param.s12.powi(2) * param.s2.powi(5)
                + -112. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(5)
                + -62.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + -64. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(5)
                + -112. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(5)
                + -32. * param.m2_2 * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(5)
                + -54. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(5)
                + 28. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(5)
                + 43. * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(5)
                + -4. * param.m2_2 * param.s12.powi(5) * param.s2.powi(5)
                + 108. * param.s1 * param.s12.powi(5) * param.s2.powi(5)
                + 46. * param.s12.powi(6) * param.s2.powi(5)
                + -21. * param.m2_2.powi(4) * param.s1 * param.s2.powi(6)
                + -23. * param.m2_2.powi(4) * param.s12 * param.s2.powi(6)
                + 19. * param.m2_2.powi(3) * param.s1 * param.s12 * param.s2.powi(6)
                + 17. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(6)
                + 19. * param.m2_2.powi(2) * param.s1 * param.s12.powi(2) * param.s2.powi(6)
                + 17. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(6)
                + 19. * param.m2_2 * param.s1 * param.s12.powi(3) * param.s2.powi(6)
                + 7. * param.m2_2 * param.s12.powi(4) * param.s2.powi(6)
                + -36. * param.s1 * param.s12.powi(4) * param.s2.powi(6)
                + -18. * param.s12.powi(5) * param.s2.powi(6)
                + 3. * param.m2_2.powi(4) * param.s2.powi(7)
                + -2. * param.m2_2.powi(3) * param.s12 * param.s2.powi(7)
                + -2. * param.m2_2.powi(2) * param.s12.powi(2) * param.s2.powi(7)
                + -2. * param.m2_2 * param.s12.powi(3) * param.s2.powi(7)
                + 3. * param.s12.powi(4) * param.s2.powi(7)
                + 60.
                    * param.m0_2.powi(4)
                    * param.s12.powi(4)
                    * (2. * param.s12.powi(3)
                        + (param.s1 - param.s2).powi(3)
                        + 3. * param.s12 * (param.s1 - param.s2) * param.s2
                        + param.s12.powi(2) * (-3. * param.s1 + 2. * param.s2))
                + param.m1_2.powi(4)
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
                                + 126. * param.s2.powi(2)))
                + 30.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (param.m1_2
                        * (-6. * param.s12.powi(4)
                            + param.s12.powi(3) * (17. * param.s1 + -24. * param.s2)
                            + (param.s1 - param.s2).powi(4)
                            + 3. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (param.s1 + 6. * param.s2)
                            + param.s12.powi(2)
                                * (-15. * param.s1.powi(2)
                                    + 16. * param.s1 * param.s2
                                    + 11. * param.s2.powi(2)))
                        + param.s12
                            * (4. * param.s12.powi(4)
                                + param.s12.powi(3) * (-9. * param.s1 + 6. * param.s2)
                                + param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 20. * param.s1 * param.s2
                                        + -19. * param.s2.powi(2))
                                + param.s12
                                    * (5. * param.s1.powi(3)
                                        + -30. * param.s1.powi(2) * param.s2
                                        + 21. * param.s1 * param.s2.powi(2)
                                        + 4. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3)
                                    * (3. * param.s1 + 5. * param.s2))
                        - param.m2_2
                            * (10. * param.s12.powi(4)
                                + (param.s1 - param.s2).powi(4)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (11. * param.s1 + 10. * param.s2)
                                + param.s12.powi(2)
                                    * (-15. * param.s1.powi(2)
                                        + 40. * param.s1 * param.s2
                                        + -13. * param.s2.powi(2))
                                - param.s12.powi(3) * (7. * param.s1 + 8. * param.s2)))
                + param.m1_2
                    * (param.m2_2.powi(3)
                        * (54. * param.s12.powi(7)
                            + param.s12.powi(6) * (103. * param.s1 + -292. * param.s2)
                            + 12. * (param.s1 - param.s2).powi(7)
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(3)
                                * (401. * param.s1.powi(2)
                                    + 559. * param.s1 * param.s2
                                    + 344. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (-797. * param.s1.powi(2)
                                    + 510. * param.s1 * param.s2
                                    + 677. * param.s2.powi(2))
                            + 4. * param.s12.powi(4)
                                * (325. * param.s1.powi(3)
                                    + 159. * param.s1.powi(2) * param.s2
                                    + -342. * param.s1 * param.s2.powi(2)
                                    + -220. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (-970. * param.s1.powi(4)
                                    + -544. * param.s1.powi(3) * param.s2
                                    + 234. * param.s1.powi(2) * param.s2.powi(2)
                                    + 580. * param.s1 * param.s2.powi(3)
                                    + 700. * param.s2.powi(4))
                            - param.s12
                                * (param.s1 - param.s2).powi(5)
                                * (103. * param.s1 + 97. * param.s2))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (-4. * param.s12.powi(7)
                                + param.s12.powi(6) * (7. * param.s1 + 32. * param.s2)
                                + param.s12.powi(5)
                                    * (27. * param.s1.powi(2)
                                        + 170. * param.s1 * param.s2
                                        + -47. * param.s2.powi(2))
                                + -2.
                                    * (param.s1 - param.s2).powi(5)
                                    * (param.s1.powi(2)
                                        + -7. * param.s1 * param.s2
                                        + -4. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (23. * param.s1.powi(3)
                                        + -149. * param.s1.powi(2) * param.s2
                                        + -361. * param.s1 * param.s2.powi(2)
                                        + -73. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(4)
                                    * (50. * param.s1.powi(3)
                                        + 278. * param.s1.powi(2) * param.s2
                                        + -589. * param.s1 * param.s2.powi(2)
                                        + 30. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (65. * param.s1.powi(4)
                                        + 152. * param.s1.powi(3) * param.s2
                                        + 688. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1350. * param.s1 * param.s2.powi(3)
                                        + 105. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-81. * param.s1.powi(5)
                                        + 244. * param.s1.powi(4) * param.s2
                                        + -2580. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1436. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1177. * param.s1 * param.s2.powi(4)
                                        + -196. * param.s2.powi(5)))
                        - param.s12.powi(3)
                            * (param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (-3. * param.s1.powi(2)
                                        + -10. * param.s1 * param.s2
                                        + 28. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (25. * param.s1.powi(3)
                                        + 94. * param.s1.powi(2) * param.s2
                                        + 193. * param.s1 * param.s2.powi(2)
                                        + -40. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(4)
                                    * (3. * param.s1.powi(3)
                                        + -14. * param.s1.powi(2) * param.s2
                                        + 29. * param.s1 * param.s2.powi(2)
                                        + 12. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-45. * param.s1.powi(4)
                                        + -116. * param.s1.powi(3) * param.s2
                                        + -154. * param.s1.powi(2) * param.s2.powi(2)
                                        + 180. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (39. * param.s1.powi(5)
                                        + 4. * param.s1.powi(4) * param.s2
                                        + -280. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1456. * param.s1.powi(2) * param.s2.powi(3)
                                        + -903. * param.s1 * param.s2.powi(4)
                                        + 44. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (17. * param.s1.powi(4)
                                        + -28. * param.s1.powi(3) * param.s2
                                        + -183. * param.s1.powi(2) * param.s2.powi(2)
                                        + -478. * param.s1 * param.s2.powi(3)
                                        + 42. * param.s2.powi(4))
                                - param.s12.powi(6) * (3. * param.s1 + 8. * param.s2))
                        - param.m2_2.powi(2)
                            * param.s12
                            * (9. * param.s12.powi(7)
                                + (param.s1 - param.s2).powi(6)
                                    * (7. * param.s1 + 8. * param.s2)
                                + param.s12.powi(6) * (43. * param.s1 + 28. * param.s2)
                                + param.s12.powi(5)
                                    * (-297. * param.s1.powi(2)
                                        + 1210. * param.s1 * param.s2
                                        + -238. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (261. * param.s1.powi(3)
                                        + 1078. * param.s1.powi(2) * param.s2
                                        + 1015. * param.s1 * param.s2.powi(2)
                                        + 256. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (585. * param.s1.powi(3)
                                        + -1854. * param.s1.powi(2) * param.s2
                                        + -1933. * param.s1 * param.s2.powi(2)
                                        + 500. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-545. * param.s1.powi(4)
                                        + -4. * param.s1.powi(3) * param.s2
                                        + 4184. * param.s1.powi(2) * param.s2.powi(2)
                                        + 100. * param.s1 * param.s2.powi(3)
                                        + -495. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (63. * param.s1.powi(2)
                                        + 154. * param.s1 * param.s2
                                        + 68. * param.s2.powi(2))))
                + param.m1_2.powi(2)
                    * (param.m2_2
                        * param.s12
                        * (31. * param.s12.powi(7)
                            + -2. * param.s12.powi(6) * (89. * param.s1 + 174. * param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (58. * param.s1.powi(2)
                                    + -136. * param.s1 * param.s2
                                    + -207. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (432. * param.s1.powi(2)
                                    + 860. * param.s1 * param.s2
                                    + 383. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (108. * param.s1.powi(3)
                                    + -236. * param.s1.powi(2) * param.s2
                                    + -695. * param.s1 * param.s2.powi(2)
                                    + -482. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (455. * param.s1.powi(4)
                                    + -796. * param.s1.powi(3) * param.s2
                                    + 2416. * param.s1.powi(2) * param.s2.powi(2)
                                    + 2780. * param.s1 * param.s2.powi(3)
                                    + -1615. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (575. * param.s1.powi(3)
                                    + 316. * param.s1.powi(2) * param.s2
                                    + 3477. * param.s1 * param.s2.powi(2)
                                    + -770. * param.s2.powi(3))
                            - (7. * param.s1 + -22. * param.s2) * (param.s1 - param.s2).powi(6))
                        + param.m2_2.powi(2)
                            * (94. * param.s12.powi(7)
                                + -18. * (param.s1 - param.s2).powi(7)
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (49. * param.s1 + 51. * param.s2)
                                + param.s12.powi(6) * (-497. * param.s1 + 68. * param.s2)
                                + param.s12.powi(5)
                                    * (1123. * param.s1.powi(2)
                                        + 1010. * param.s1 * param.s2
                                        + -1003. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(4)
                                    * (355. * param.s1.powi(3)
                                        + 566. * param.s1.powi(2) * param.s2
                                        + -193. * param.s1 * param.s2.powi(2)
                                        + -440. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (550. * param.s1.powi(4)
                                        + 448. * param.s1.powi(3) * param.s2
                                        + 117. * param.s1.powi(2) * param.s2.powi(2)
                                        + -430. * param.s1 * param.s2.powi(3)
                                        + -685. * param.s2.powi(4))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (529. * param.s1.powi(2)
                                        + 841. * param.s1 * param.s2
                                        + 586. * param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (9. * param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (153. * param.s1.powi(2)
                                        + 270. * param.s1 * param.s2
                                        + 362. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (27. * param.s1.powi(3)
                                        + -81. * param.s1.powi(2) * param.s2
                                        + 96. * param.s1 * param.s2.powi(2)
                                        + 238. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (195. * param.s1.powi(4)
                                        + -124. * param.s1.powi(3) * param.s2
                                        + -566. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1500. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-99. * param.s1.powi(5)
                                        + 306. * param.s1.powi(4) * param.s2
                                        + 40. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1394. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2077. * param.s1 * param.s2.powi(4)
                                        + 436. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (225. * param.s1.powi(3)
                                        + 234. * param.s1.powi(2) * param.s2
                                        + 153. * param.s1 * param.s2.powi(2)
                                        + 500. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(5)
                                    * (3. * param.s1.powi(2)
                                        + -11. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                - param.s12.powi(6) * (57. * param.s1 + 82. * param.s2)))
                + 10.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m1_2.powi(2)
                        * (4. * param.s12.powi(5)
                            + 4. * param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (2. * param.s1 + 5. * param.s2)
                            + param.s12.powi(4) * (-17. * param.s1 + 86. * param.s2)
                            + 2. * param.s12.powi(3)
                                * (14. * param.s1.powi(2)
                                    + -83. * param.s1 * param.s2
                                    + 25. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-22. * param.s1.powi(3)
                                    + 79. * param.s1.powi(2) * param.s2
                                    + 64. * param.s1 * param.s2.powi(2)
                                    + -121. * param.s2.powi(3))
                            - (param.s1 - param.s2).powi(5))
                        + param.m2_2
                            * param.s12
                            * (-17. * param.s12.powi(5)
                                + (param.s1 - param.s2).powi(4)
                                    * (5. * param.s1 + 4. * param.s2)
                                + 2. * param.s12.powi(4) * (5. * param.s1 + 7. * param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (26. * param.s1.powi(2)
                                        + 138. * param.s1 * param.s2
                                        + 25. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (67. * param.s1.powi(2)
                                        + -226. * param.s1 * param.s2
                                        + 56. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-91. * param.s1.powi(3)
                                        + 142. * param.s1.powi(2) * param.s2
                                        + 139. * param.s1 * param.s2.powi(2)
                                        + -82. * param.s2.powi(3)))
                        + param.m2_2.powi(2)
                            * (22. * param.s12.powi(5)
                                + param.s12.powi(4) * (28. * param.s1 + -58. * param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (17. * param.s1 + 11. * param.s2)
                                + param.s12.powi(3)
                                    * (-107. * param.s1.powi(2)
                                        + 86. * param.s1 * param.s2
                                        + 41. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (41. * param.s1.powi(3)
                                        + 79. * param.s1.powi(2) * param.s2
                                        + -125. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(5))
                        + param.s12.powi(2)
                            * (param.s12.powi(5)
                                + -2. * param.s12.powi(4) * (param.s1 + -7. * param.s2)
                                + param.s12.powi(3)
                                    * (-2. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + -37. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (2. * param.s1.powi(2)
                                        + 23. * param.s1 * param.s2
                                        + 11. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (8. * param.s1.powi(3)
                                        + -101. * param.s1.powi(2) * param.s2
                                        + 76. * param.s1 * param.s2.powi(2)
                                        + 17. * param.s2.powi(3))
                                + param.s12
                                    * (-7. * param.s1.powi(4)
                                        + 32. * param.s1.powi(3) * param.s2
                                        + 81. * param.s1.powi(2) * param.s2.powi(2)
                                        + -122. * param.s1 * param.s2.powi(3)
                                        + 16. * param.s2.powi(4)))
                        + param.m1_2
                            * (param.m2_2
                                * (46. * param.s12.powi(5)
                                    + 2. * (param.s1 - param.s2).powi(5)
                                    + param.s12.powi(4) * (-119. * param.s1 + 44. * param.s2)
                                    + param.s12.powi(3)
                                        * (79. * param.s1.powi(2)
                                            + 188. * param.s1 * param.s2
                                            + -199. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (17. * param.s1.powi(3)
                                            + -266. * param.s1.powi(2) * param.s2
                                            + 169. * param.s1 * param.s2.powi(2)
                                            + 80. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (25. * param.s1 + 31. * param.s2))
                                - param.s12
                                    * (5. * param.s12.powi(5)
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 8. * param.s2)
                                        + param.s12.powi(4)
                                            * (-19. * param.s1 + 82. * param.s2)
                                        + param.s12.powi(3)
                                            * (26. * param.s1.powi(2)
                                                + -56. * param.s1 * param.s2
                                                + -77. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 102. * param.s1 * param.s2
                                                + 86. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (7. * param.s1.powi(3)
                                                + 65. * param.s1.powi(2) * param.s2
                                                + -178. * param.s1 * param.s2.powi(2)
                                                + 52. * param.s2.powi(3)))))
                + 5. * param.m0_2
                    * param.s12
                    * (param.m1_2.powi(3)
                        * (2. * param.s12.powi(6)
                            + (param.s1 - param.s2).powi(6)
                            + param.s12.powi(4)
                                * (25. * param.s1.powi(2)
                                    + 110. * param.s1 * param.s2
                                    + -201. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (10. * param.s1.powi(2)
                                    + 32. * param.s1 * param.s2
                                    + 45. * param.s2.powi(2))
                            + -6.
                                * param.s12.powi(3)
                                * (5. * param.s1.powi(3)
                                    + 18. * param.s1.powi(2) * param.s2
                                    + -33. * param.s1 * param.s2.powi(2)
                                    + -26. * param.s2.powi(3))
                            - param.s12.powi(5) * (11. * param.s1 + 36. * param.s2)
                            - param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (7. * param.s1 + 12. * param.s2))
                        + param.m2_2.powi(2)
                            * param.s12
                            * (6. * param.s12.powi(6)
                                + param.s12.powi(5) * (69. * param.s1 + -20. * param.s2)
                                + (param.s1 - param.s2).powi(5) * (3. * param.s1 + param.s2)
                                + param.s12.powi(4)
                                    * (-207. * param.s1.powi(2)
                                        + 142. * param.s1 * param.s2
                                        + 19. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (36. * param.s1.powi(3)
                                        + 82. * param.s1.powi(2) * param.s2
                                        + -113. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (15. * param.s1.powi(4)
                                        + -256. * param.s1.powi(3) * param.s2
                                        + 147. * param.s1.powi(2) * param.s2.powi(2)
                                        + 102. * param.s1 * param.s2.powi(3)
                                        + -8. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (45. * param.s1.powi(2)
                                        + 59. * param.s1 * param.s2
                                        + 8. * param.s2.powi(2)))
                        + param.s12.powi(3)
                            * (param.s12.powi(4)
                                * (5. * param.s1.powi(2)
                                    + 14. * param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(3)
                                        + -11. * param.s1.powi(2) * param.s2
                                        + -35. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(3)
                                        + 14. * param.s1.powi(2) * param.s2
                                        + -34. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (5. * param.s1.powi(4)
                                        + 48. * param.s1.powi(2) * param.s2.powi(2)
                                        + -74. * param.s1 * param.s2.powi(3)
                                        + 9. * param.s2.powi(4))
                                + param.s12
                                    * (-5. * param.s1.powi(5)
                                        + 28. * param.s1.powi(4) * param.s2
                                        + -168. * param.s1.powi(3) * param.s2.powi(2)
                                        + 116. * param.s1.powi(2) * param.s2.powi(3)
                                        + 41. * param.s1 * param.s2.powi(4)
                                        + -12. * param.s2.powi(5))
                                - param.s1 * param.s12.powi(5))
                        + param.m1_2.powi(2)
                            * (param.m2_2
                                * (-22. * param.s12.powi(6)
                                    + param.s12.powi(5) * (101. * param.s1 + -236. * param.s2)
                                    + -3. * (param.s1 - param.s2).powi(6)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (25. * param.s1 + 32. * param.s2)
                                    + -2.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (46. * param.s1.powi(2)
                                            + 120. * param.s1 * param.s2
                                            + 95. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-187. * param.s1.powi(2)
                                            + 334. * param.s1 * param.s2
                                            + 403. * param.s2.powi(2))
                                    + 2. * param.s12.powi(3)
                                        * (89. * param.s1.powi(3)
                                            + 4. * param.s1.powi(2) * param.s2
                                            + -425. * param.s1 * param.s2.powi(2)
                                            + 8. * param.s2.powi(3)))
                                + param.s12
                                    * (-4. * param.s12.powi(6)
                                        + 3. * param.s12.powi(5)
                                            * (7. * param.s1 + 20. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (9. * param.s1.powi(2)
                                                + -37. * param.s1 * param.s2
                                                + -84. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (-45. * param.s1.powi(2)
                                                + -146. * param.s1 * param.s2
                                                + 189. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (50. * param.s1.powi(3)
                                                + 68. * param.s1.powi(2) * param.s2
                                                + 326. * param.s1 * param.s2.powi(2)
                                                + -468. * param.s2.powi(3))
                                        + -6.
                                            * param.s12.powi(2)
                                            * (5. * param.s1.powi(4)
                                                + -12. * param.s1.powi(3) * param.s2
                                                + 89. * param.s1.powi(2) * param.s2.powi(2)
                                                + -58. * param.s1 * param.s2.powi(3)
                                                + -24. * param.s2.powi(4))
                                        - (param.s1 + -5. * param.s2)
                                            * (param.s1 - param.s2).powi(5)))
                        + param.m1_2
                            * (param.s12.powi(2)
                                * (2. * param.s12.powi(6)
                                    + -3. * param.s12.powi(5) * (3. * param.s1 + 8. * param.s2)
                                    + param.s12.powi(4)
                                        * (15. * param.s1.powi(2)
                                            + 22. * param.s1 * param.s2
                                            + -27. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(3)
                                        * (5. * param.s1.powi(3)
                                            + -34. * param.s1.powi(2) * param.s2
                                            + 224. * param.s1 * param.s2.powi(2)
                                            + -96. * param.s2.powi(3))
                                    + -12.
                                        * param.s12.powi(2)
                                        * param.s2
                                        * (8. * param.s1.powi(3)
                                            + -20. * param.s1.powi(2) * param.s2
                                            + -23. * param.s1 * param.s2.powi(2)
                                            + 17. * param.s2.powi(3))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1.powi(3)
                                            + 26. * param.s1.powi(2) * param.s2
                                            + 301. * param.s1 * param.s2.powi(2)
                                            + 48. * param.s2.powi(3))
                                    - (param.s1 - param.s2).powi(4)
                                        * (param.s1.powi(2)
                                            + -6. * param.s1 * param.s2
                                            + -13. * param.s2.powi(2)))
                                + 2. * param.m2_2
                                    * param.s12
                                    * (7. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-22. * param.s1 + 62. * param.s2)
                                        + param.s12.powi(4)
                                            * (17. * param.s1.powi(2)
                                                + 162. * param.s1 * param.s2
                                                + -199. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 32. * param.s1 * param.s2
                                                + 19. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (3. * param.s1.powi(3)
                                                + -118. * param.s1.powi(2) * param.s2
                                                + 77. * param.s1 * param.s2.powi(2)
                                                + 38. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-23. * param.s1.powi(4)
                                                + 212. * param.s1.powi(3) * param.s2
                                                + 282. * param.s1.powi(2) * param.s2.powi(2)
                                                + -484. * param.s1 * param.s2.powi(3)
                                                + 13. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(5)
                                            * (param.s1 + 3. * param.s2))
                                - param.m2_2.powi(2)
                                    * (70. * param.s12.powi(6)
                                        + -3. * (param.s1 - param.s2).powi(6)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (29. * param.s1 + 28. * param.s2)
                                        + param.s12.powi(4)
                                            * (-29. * param.s1.powi(2)
                                                + 710. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (71. * param.s1.powi(2)
                                                + 126. * param.s1 * param.s2
                                                + 64. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (53. * param.s1.powi(3)
                                                + -131. * param.s1.powi(2) * param.s2
                                                + -128. * param.s1 * param.s2.powi(2)
                                                + 44. * param.s2.powi(3))
                                        - param.s12.powi(5)
                                            * (137. * param.s1 + 148. * param.s2)))
                        - param.m2_2
                            * param.s12.powi(2)
                            * (param.s12.powi(5) * (9. * param.s1 + 8. * param.s2)
                                + param.s12.powi(4)
                                    * (-33. * param.s1.powi(2)
                                        + 158. * param.s1 * param.s2
                                        + -31. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(4)
                                    * (3. * param.s1.powi(2)
                                        + 14. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (21. * param.s1.powi(3)
                                        + -68. * param.s1.powi(2) * param.s2
                                        + -74. * param.s1 * param.s2.powi(2)
                                        + 22. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(2)
                                    * (9. * param.s1.powi(4)
                                        + 116. * param.s1.powi(3) * param.s2
                                        + -348. * param.s1.powi(2) * param.s2.powi(2)
                                        + 102. * param.s1 * param.s2.powi(3)
                                        + 13. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(3)
                                        + -194. * param.s1.powi(2) * param.s2
                                        + -183. * param.s1 * param.s2.powi(2)
                                        + -4. * param.s2.powi(3)))
                        - param.m2_2.powi(3)
                            * (6. * param.s12.powi(6)
                                + param.s12.powi(5) * (83. * param.s1 + -28. * param.s2)
                                + (param.s1 - param.s2).powi(6)
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (35. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 14. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-133. * param.s1.powi(2)
                                        + -122. * param.s1 * param.s2
                                        + 53. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (4. * param.s1.powi(3)
                                        + -70. * param.s1.powi(2) * param.s2
                                        + 13. * param.s2.powi(3)
                                        - param.s1 * param.s2.powi(2))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (11. * param.s1 + 8. * param.s2)))
                - param.m1_2.powi(3)
                    * (param.s12
                        * (11. * param.s12.powi(7)
                            + param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (53. * param.s1.powi(2)
                                    + -26. * param.s1 * param.s2
                                    + -122. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (207. * param.s1.powi(2)
                                    + 410. * param.s1 * param.s2
                                    + 558. * param.s2.powi(2))
                            + param.s12.powi(3)
                                * (305. * param.s1.powi(4)
                                    + 84. * param.s1.powi(3) * param.s2
                                    + -234. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1600. * param.s1 * param.s2.powi(3)
                                    + -675. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (325. * param.s1.powi(3)
                                    + 506. * param.s1.powi(2) * param.s2
                                    + 747. * param.s1 * param.s2.powi(2)
                                    + 360. * param.s2.powi(3))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (171. * param.s1.powi(3)
                                    + 38. * param.s1.powi(2) * param.s2
                                    + -395. * param.s1 * param.s2.powi(2)
                                    + -684. * param.s2.powi(3))
                            - param.s12.powi(6) * (73. * param.s1 + 108. * param.s2)
                            - (7. * param.s1 + -12. * param.s2) * (param.s1 - param.s2).powi(6))
                        + param.m2_2
                            * (26. * param.s12.powi(7)
                                + -12. * (param.s1 - param.s2).powi(7)
                                + param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (93. * param.s1 + 107. * param.s2)
                                + param.s12.powi(5)
                                    * (437. * param.s1.powi(2)
                                        + 1230. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(4)
                                    * (325. * param.s1.powi(3)
                                        + 748. * param.s1.powi(2) * param.s2
                                        + 451. * param.s1 * param.s2.powi(2)
                                        + -600. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (290. * param.s1.powi(4)
                                        + 252. * param.s1.powi(3) * param.s2
                                        + 243. * param.s1.powi(2) * param.s2.powi(2)
                                        + -170. * param.s1 * param.s2.powi(3)
                                        + -615. * param.s2.powi(4))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (311. * param.s1.powi(2)
                                        + 549. * param.s1 * param.s2
                                        + 444. * param.s2.powi(2))
                                - param.s12.powi(6) * (163. * param.s1 + 348. * param.s2)))
                - param.s12.powi(11)
                - param.m2_2 * param.s12.powi(10)
                - param.m2_2.powi(2) * param.s12.powi(9))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s12.powi(4)
                    * (7.
                        * param.m1_2.powi(5)
                        * param.s2.powi(3)
                        * (param.s1 + param.s2 - param.s12)
                        + param.m0_2.powi(5)
                            * param.s12
                            * (2. * param.s12.powi(3)
                                + (param.s1 - param.s2).powi(3)
                                + 3. * param.s12 * (param.s1 - param.s2) * param.s2
                                + param.s12.powi(2) * (-3. * param.s1 + 2. * param.s2))
                        + 5. * param.m1_2.powi(4)
                            * param.s2.powi(2)
                            * (param.s2
                                * (-4. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s12 * (param.s1 + -6. * param.s2)
                                    + param.s1 * param.s2
                                    + 3. * param.s2.powi(2))
                                - param.m2_2
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)
                                        + -6. * param.s12 * (param.s1 + param.s2)))
                        + 10.
                            * param.m1_2.powi(3)
                            * param.s2
                            * (param.s2.powi(2)
                                * (-3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + (param.s1 - param.s2).powi(2) * (2. * param.s1 + param.s2)
                                    + param.s12
                                        * (2. * param.s1.powi(2)
                                            + 3. * param.s1 * param.s2
                                            + -3. * param.s2.powi(2))
                                    - param.s12.powi(3))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (2. * param.s1.powi(3)
                                        + param.s12.powi(3)
                                        + 2. * param.s1.powi(2) * param.s2
                                        + -3. * param.s12.powi(2) * param.s2
                                        + -3. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s12
                                            * (param.s1 * param.s2 + param.s2.powi(2)
                                                - param.s1.powi(2))
                                        - param.s2.powi(3))
                                + param.m2_2.powi(2)
                                    * (param.s1.powi(3)
                                        + 6. * param.s1.powi(2) * param.s2
                                        + 6. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3)
                                        + 3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s12.powi(3)))
                        + param.m0_2.powi(4)
                            * (param.m1_2
                                * (-4. * param.s12.powi(4)
                                    + param.s12.powi(3) * (11. * param.s1 + -14. * param.s2)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1 + 11. * param.s2)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2)))
                                + param.s12
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-7. * param.s1 + 3. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + -6. * param.s1.powi(2) * param.s2
                                                + 4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + 3. * param.s2))
                                - param.m2_2
                                    * (6. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))))
                        + param.s1.powi(2)
                            * (param.s12
                                * (2. * param.s12.powi(3)
                                    + param.s12.powi(2) * (2. * param.s1 + -3. * param.s2)
                                    + -3. * param.s1 * param.s12 * (param.s1 - param.s2)
                                    - (param.s1 - param.s2).powi(3))
                                * param.s2.powi(3)
                                + 5. * param.m2_2.powi(4)
                                    * (-3. * param.s1 * param.s12.powi(2)
                                        + param.s12.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)
                                        + 3. * param.s12
                                            * (param.s1.powi(2) + param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s1.powi(3))
                                + 2. * param.m2_2.powi(2)
                                    * param.s2
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (-7. * param.s1 + 3. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + -4. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + -6. * param.s1.powi(2) * param.s2
                                                + 4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + 3. * param.s2))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (-4. * param.s1 + 6. * param.s2)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + 3. * param.s1.powi(2) * param.s2
                                                + -12. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - param.m2_2
                                    * param.s2.powi(2)
                                    * (6. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2)))
                                - param.m2_2.powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s12.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2)
                                        + -6. * param.s12 * (param.s1 + param.s2)))
                        + -2.
                            * param.m1_2.powi(2)
                            * (-3.
                                * param.m2_2.powi(2)
                                * param.s2
                                * (-4. * param.s1.powi(4)
                                    + param.s12.powi(4)
                                    + param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                    + -14. * param.s1.powi(3) * param.s2
                                    + 6. * param.s1.powi(2) * param.s2.powi(2)
                                    + 11. * param.s1 * param.s2.powi(3)
                                    + param.s2.powi(4)
                                    + param.s12.powi(2)
                                        * (-9. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + 6. * param.s2.powi(2))
                                    + param.s12
                                        * (11. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + -21. * param.s1 * param.s2.powi(2)
                                            + -4. * param.s2.powi(3)))
                                + param.m2_2.powi(3)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 16. * param.s1.powi(3) * param.s2
                                        + 36. * param.s1.powi(2) * param.s2.powi(2)
                                        + 16. * param.s1 * param.s2.powi(3)
                                        + param.s2.powi(4)
                                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 3. * param.m2_2
                                    * param.s2.powi(2)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (6. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                - param.s2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (11. * param.s1 + -4. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (2. * param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + 2. * param.s2.powi(2))
                                        + param.s12
                                            * (-14. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -4. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (4. * param.s1 + param.s2)))
                        + param.m1_2
                            * param.s1
                            * (param.s2.powi(3)
                                * (-4. * param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(4)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (11. * param.s1 + param.s2)
                                    + param.s12.powi(3) * (-14. * param.s1 + 11. * param.s2)
                                    + param.s12.powi(2)
                                        * (6. * param.s1.powi(2)
                                            + 9. * param.s1 * param.s2
                                            + -9. * param.s2.powi(2)))
                                + -5.
                                    * param.m2_2.powi(4)
                                    * (param.s12.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3))
                                + 6. * param.m2_2.powi(2)
                                    * param.s2
                                    * (-2. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (-7. * param.s1.powi(3)
                                                + 12. * param.s1.powi(2) * param.s2
                                                + 12. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3)))
                                + 4. * param.m2_2
                                    * param.s2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (3. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 2. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + 4. * param.s1.powi(2) * param.s2
                                                + -6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 4. * param.m2_2.powi(3)
                                    * (param.s1.powi(4)
                                        + param.s12.powi(4)
                                        + 11. * param.s1.powi(3) * param.s2
                                        + 6. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14. * param.s1 * param.s2.powi(3)
                                        + -4. * param.s2.powi(4)
                                        + param.s12.powi(3) * (-4. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 9. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + param.s12
                                            * (-4. * param.s1.powi(3)
                                                + -21. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3))))
                        + param.m0_2.powi(3)
                            * (2.
                                * param.m1_2.powi(2)
                                * (param.s12.powi(4)
                                    + (param.s1 - param.s2).powi(3) * (param.s1 + 4. * param.s2)
                                    + param.s12.powi(3) * (-4. * param.s1 + 11. * param.s2)
                                    + 3. * param.s12.powi(2)
                                        * (2. * param.s1.powi(2)
                                            + -7. * param.s1 * param.s2
                                            + 2. * param.s2.powi(2))
                                    + param.s12
                                        * (-4. * param.s1.powi(3)
                                            + 9. * param.s1.powi(2) * param.s2
                                            + 9. * param.s1 * param.s2.powi(2)
                                            + -14. * param.s2.powi(3)))
                                + 2. * param.m2_2.powi(2)
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3) * (3. * param.s1 + -7. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 2. * param.s2)
                                        + 3. * param.s12.powi(2)
                                            * (-4. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1.powi(3)
                                                + 4. * param.s1.powi(2) * param.s2
                                                + -6. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3)))
                                + 2. * param.m2_2
                                    * (-3. * param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (8. * param.s1.powi(2)
                                                + -30. * param.s1 * param.s2
                                                + 8. * param.s2.powi(2))
                                        + -6.
                                            * param.s12.powi(2)
                                            * (2. * param.s1.powi(3)
                                                + -3. * param.s1.powi(2) * param.s2
                                                + -3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                + param.s12
                                    * (param.s12.powi(5)
                                        + -3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + -6. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + -3.
                                            * param.s12
                                            * (param.s1.powi(4)
                                                + -4. * param.s1.powi(3) * param.s2
                                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4)))
                                + param.m1_2
                                    * (-3. * param.s12.powi(5)
                                        + param.s12.powi(4) * (11. * param.s1 + -21. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + -28. * param.s1 * param.s2
                                                + -21. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-14. * param.s1.powi(2)
                                                + 14. * param.s1 * param.s2
                                                + 24. * param.s2.powi(2))
                                        + 6. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + 6. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 4. * param.m2_2
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1 + 3. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + -6. * param.s1.powi(2) * param.s2
                                                        + 4. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1 + 3. * param.s2))
                                        - (param.s1 - param.s2).powi(4)
                                            * (param.s1 + 3. * param.s2)))
                        + param.m0_2.powi(2)
                            * (-10.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (param.s12.powi(3)
                                    + -3. * param.s12.powi(2) * (param.s1 - param.s2)
                                    + param.s12
                                        * (3. * param.s1.powi(2)
                                            + -3. * param.s1 * param.s2
                                            + -2. * param.s2.powi(2))
                                    - (param.s1 - param.s2).powi(2) * (param.s1 + 2. * param.s2))
                                + -2.
                                    * param.m2_2.powi(3)
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (6. * param.s1 + -4. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-9. * param.s1.powi(2)
                                                + -6. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 3. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3)))
                                + 3. * param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (-6. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -10. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + param.s12
                                    * param.s2
                                    * (param.s12.powi(5)
                                        + 3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (-6. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (4. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -10. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + param.s2.powi(2)))
                                + -6.
                                    * param.m1_2.powi(2)
                                    * (param.m2_2
                                        * (param.s12.powi(4)
                                            + param.s12.powi(3)
                                                * (-4. * param.s1 + 6. * param.s2)
                                            + param.s12.powi(2)
                                                * (6. * param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + -9. * param.s2.powi(2))
                                            + (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 8. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + -2.
                                                * param.s12
                                                * (2. * param.s1.powi(3)
                                                    + 3. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 2. * param.s2.powi(3)))
                                        - param.s2
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (-7. * param.s1 + 3. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + -6. * param.s1.powi(2) * param.s2
                                                        + 4. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (2. * param.s1 + 3. * param.s2)))
                                + -3.
                                    * param.m1_2
                                    * (2.
                                        * param.m2_2.powi(2)
                                        * (2. * param.s12.powi(4)
                                            + -3. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + -3.
                                                * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + -3.
                                                * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (7. * param.s1.powi(3)
                                                    + -12. * param.s1.powi(2) * param.s2
                                                    + -12. * param.s1 * param.s2.powi(2)
                                                    + 7. * param.s2.powi(3)))
                                        + -2.
                                            * param.m2_2
                                            * (param.s12.powi(5)
                                                + -3.
                                                    * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + -6. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + -3.
                                                    * param.s12
                                                    * (param.s1.powi(4)
                                                        + -4. * param.s1.powi(3) * param.s2
                                                        + -6.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 10. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4)))
                                        - param.s2
                                            * (-3. * param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + 3. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + -30. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                + -6.
                                                    * param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -3. * param.s1.powi(2) * param.s2
                                                        + -3. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3))))
                                - param.m2_2
                                    * (param.s12.powi(6)
                                        + 36.
                                            * param.s1
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * param.s2
                                            * (param.s1 + param.s2)
                                        + -9.
                                            * param.s12.powi(4)
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (4. * param.s1.powi(3)
                                                + -9. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + -9.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(4)
                                                + 4. * param.s1.powi(3) * param.s2
                                                + -14. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4. * param.s1 * param.s2.powi(3)
                                                + param.s2.powi(4))))
                        + param.m0_2
                            * (5.
                                * param.m1_2.powi(4)
                                * param.s2.powi(2)
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -4. * param.s2.powi(2)
                                    + param.s12 * (-6. * param.s1 + param.s2))
                                + 10.
                                    * param.m1_2.powi(3)
                                    * param.s2
                                    * (2.
                                        * param.m2_2
                                        * (-3. * param.s1 * param.s12.powi(2)
                                            + param.s12.powi(3)
                                            + -3. * param.s1.powi(2) * param.s2
                                            + 2. * param.s1 * param.s2.powi(2)
                                            + 2. * param.s2.powi(3)
                                            + 3. * param.s12
                                                * (param.s1.powi(2) + param.s1 * param.s2
                                                    - param.s2.powi(2))
                                            - param.s1.powi(3))
                                        + param.s2
                                            * (-3. * param.s12.powi(3)
                                                + 3. * param.s12.powi(2)
                                                    * (param.s1 + param.s2)
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (3. * param.s1.powi(2)
                                                        + -10. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))))
                                + 6. * param.m1_2.powi(2)
                                    * (2.
                                        * param.m2_2
                                        * param.s2
                                        * (-2. * param.s12.powi(4)
                                            + 3. * param.s12.powi(3) * (param.s1 + param.s2)
                                            + 3. * param.s12.powi(2)
                                                * (param.s1.powi(2)
                                                    + -6. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + 3. * (param.s1 - param.s2).powi(2)
                                                * (param.s1.powi(2)
                                                    + 3. * param.s1 * param.s2
                                                    + param.s2.powi(2))
                                            + param.s12
                                                * (-7. * param.s1.powi(3)
                                                    + 12. * param.s1.powi(2) * param.s2
                                                    + 12. * param.s1 * param.s2.powi(2)
                                                    + -7. * param.s2.powi(3)))
                                        + param.s2.powi(2)
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (3. * param.s1 + -7. * param.s2)
                                                + (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1 + 2. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (-4. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + 4. * param.s1.powi(2) * param.s2
                                                        + -6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3)))
                                        + param.m2_2.powi(2)
                                            * (param.s1.powi(4)
                                                + param.s12.powi(4)
                                                + 11. * param.s1.powi(3) * param.s2
                                                + 6. * param.s1.powi(2) * param.s2.powi(2)
                                                + -14. * param.s1 * param.s2.powi(3)
                                                + -4. * param.s2.powi(4)
                                                + param.s12.powi(3)
                                                    * (-4. * param.s1 + param.s2)
                                                + param.s12.powi(2)
                                                    * (6. * param.s1.powi(2)
                                                        + 9. * param.s1 * param.s2
                                                        + -9. * param.s2.powi(2))
                                                + param.s12
                                                    * (-4. * param.s1.powi(3)
                                                        + -21. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 11. * param.s2.powi(3))))
                                + param.s1
                                    * (5.
                                        * param.m2_2.powi(4)
                                        * (2. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 2. * param.s1.powi(2) * param.s2
                                            + -3. * param.s12.powi(2) * param.s2
                                            + -3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s12
                                                * (param.s1 * param.s2 + param.s2.powi(2)
                                                    - param.s1.powi(2))
                                            - param.s2.powi(3))
                                        + param.s12
                                            * param.s2.powi(2)
                                            * (3. * param.s12.powi(4)
                                                + param.s12.powi(3)
                                                    * (3. * param.s1 + -7. * param.s2)
                                                + (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1 + 2. * param.s2)
                                                + 3. * param.s12.powi(2)
                                                    * (-4. * param.s1.powi(2)
                                                        + 4. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * param.s12
                                                    * (param.s1.powi(3)
                                                        + 4. * param.s1.powi(2) * param.s2
                                                        + -6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3)))
                                        + 2. * param.m2_2
                                            * param.s2
                                            * (-3. * param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 + param.s2)
                                                + (param.s1 - param.s2).powi(4)
                                                    * (param.s1 + param.s2)
                                                + 3. * param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12.powi(3)
                                                    * (8. * param.s1.powi(2)
                                                        + -30. * param.s1 * param.s2
                                                        + 8. * param.s2.powi(2))
                                                + -6.
                                                    * param.s12.powi(2)
                                                    * (2. * param.s1.powi(3)
                                                        + -3. * param.s1.powi(2) * param.s2
                                                        + -3. * param.s1 * param.s2.powi(2)
                                                        + 2. * param.s2.powi(3)))
                                        + -4.
                                            * param.m2_2.powi(3)
                                            * (2. * param.s12.powi(4)
                                                + -3.
                                                    * param.s12.powi(3)
                                                    * (param.s1 + param.s2)
                                                + -3.
                                                    * param.s12.powi(2)
                                                    * (param.s1.powi(2)
                                                        + -6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(2)
                                                        + 3. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + param.s12
                                                    * (7. * param.s1.powi(3)
                                                        + -12. * param.s1.powi(2) * param.s2
                                                        + -12. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3)))
                                        + 3. * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -3.
                                                    * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + -6. * param.s2.powi(2))
                                                + (param.s1 - param.s2).powi(3)
                                                    * (param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + 3. * param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + 4. * param.s2.powi(3))
                                                + -3.
                                                    * param.s12
                                                    * (param.s1.powi(4)
                                                        + -4. * param.s1.powi(3) * param.s2
                                                        + -6.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 10. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4))))
                                + param.m1_2
                                    * (4.
                                        * param.m2_2.powi(3)
                                        * (-4. * param.s1.powi(4)
                                            + param.s12.powi(4)
                                            + param.s12.powi(3) * (param.s1 + -4. * param.s2)
                                            + -14. * param.s1.powi(3) * param.s2
                                            + 6. * param.s1.powi(2) * param.s2.powi(2)
                                            + 11. * param.s1 * param.s2.powi(3)
                                            + param.s2.powi(4)
                                            + param.s12.powi(2)
                                                * (-9. * param.s1.powi(2)
                                                    + 9. * param.s1 * param.s2
                                                    + 6. * param.s2.powi(2))
                                            + param.s12
                                                * (11. * param.s1.powi(3)
                                                    + 9. * param.s1.powi(2) * param.s2
                                                    + -21. * param.s1 * param.s2.powi(2)
                                                    + -4. * param.s2.powi(3)))
                                        + param.s2.powi(2)
                                            * (-3. * param.s12.powi(5)
                                                + param.s12.powi(4)
                                                    * (-21. * param.s1 + 11. * param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (12. * param.s1.powi(2)
                                                        + 7. * param.s1 * param.s2
                                                        + -7. * param.s2.powi(2))
                                                + 6. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + -15. * param.s1.powi(2) * param.s2
                                                        + 6. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                - param.s12
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (21. * param.s1.powi(2)
                                                        + 28. * param.s1 * param.s2
                                                        - param.s2.powi(2))
                                                - (param.s1 - param.s2).powi(4)
                                                    * (3. * param.s1 + param.s2))
                                        + 6. * param.m2_2
                                            * param.s2
                                            * (param.s12.powi(5)
                                                + 3. * param.s12.powi(4)
                                                    * (param.s1 - param.s2)
                                                + 2. * param.s12.powi(3)
                                                    * (-6. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 2. * param.s12.powi(2)
                                                    * (4. * param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + -15. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 3. * param.s12
                                                    * (param.s1.powi(4)
                                                        + -10. * param.s1.powi(3) * param.s2
                                                        + 6. * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 4. * param.s1 * param.s2.powi(3)
                                                        - param.s2.powi(4))
                                                - (param.s1 - param.s2).powi(3)
                                                    * (3. * param.s1.powi(2)
                                                        + 6. * param.s1 * param.s2
                                                        + param.s2.powi(2)))
                                        + -3.
                                            * param.m2_2.powi(2)
                                            * (param.s12.powi(5)
                                                + -6.
                                                    * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -5. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + 3. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + 2. * param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                - param.s12
                                                    * (11. * param.s1.powi(4)
                                                        + 14. * param.s1.powi(3) * param.s2
                                                        + -90.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 14. * param.s1 * param.s2.powi(3)
                                                        + 11. * param.s2.powi(4))
                                                - param.s12.powi(4) * (param.s1 + param.s2)))))
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
