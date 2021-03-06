use super::{log_diff, Parameters};

pub(crate) fn c022(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.08333333333333333
            * std::f64::consts::PI
            * param.s1.powi(-2)
            * param.lambda_s12_sqrt.powi(-9)
            * ((param.m1_2.powi(3)
                * (param.s1.powi(5)
                    + -23. * param.s1.powi(4) * param.s2
                    + -398. * param.s1.powi(3) * param.s2.powi(2)
                    + -398. * param.s1.powi(2) * param.s2.powi(3)
                    + -23. * param.s1 * param.s2.powi(4)
                    + param.s2.powi(5)
                    + 5. * param.s12.powi(4) * (param.s1 + param.s2)
                    + -2.
                        * param.s12.powi(3)
                        * (5. * param.s1.powi(2)
                            + -4. * param.s1 * param.s2
                            + 5. * param.s2.powi(2))
                    + 2. * param.s12.powi(2)
                        * (5. * param.s1.powi(3)
                            + -27. * param.s1.powi(2) * param.s2
                            + -27. * param.s1 * param.s2.powi(2)
                            + 5. * param.s2.powi(3))
                    + param.s12
                        * (-5. * param.s1.powi(4)
                            + 64. * param.s1.powi(3) * param.s2
                            + 462. * param.s1.powi(2) * param.s2.powi(2)
                            + 64. * param.s1 * param.s2.powi(3)
                            + -5. * param.s2.powi(4))
                    - param.s12.powi(5))
                + param.m0_2.powi(3)
                    * (param.s12.powi(5)
                        + param.s12.powi(3)
                            * (-92. * param.s1.powi(2)
                                + 40. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        + 2. * param.s12.powi(2)
                            * (94. * param.s1.powi(3)
                                + -62. * param.s1.powi(2) * param.s2
                                + -9. * param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3))
                        + param.s12
                            * (-37. * param.s1.powi(4)
                                + -152. * param.s1.powi(3) * param.s2
                                + 200. * param.s1.powi(2) * param.s2.powi(2)
                                + -16. * param.s1 * param.s2.powi(3)
                                + 5. * param.s2.powi(4))
                        - (param.s1 - param.s2).powi(3)
                            * (43. * param.s1.powi(2) + 8. * param.s1 * param.s2
                                - param.s2.powi(2))
                        - param.s12.powi(4) * (17. * param.s1 + 5. * param.s2))
                + param.m1_2.powi(2)
                    * param.s1
                    * (-3. * param.s1.powi(5)
                        + 3. * param.s12.powi(5)
                        + 57. * param.s1.powi(4) * param.s2
                        + 662. * param.s1.powi(3) * param.s2.powi(2)
                        + -238. * param.s1.powi(2) * param.s2.powi(3)
                        + -463. * param.s1 * param.s2.powi(4)
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
                                + 233. * param.s1 * param.s2.powi(2)
                                + 51. * param.s2.powi(3))
                        + param.s12
                            * (15. * param.s1.powi(4)
                                + -144. * param.s1.powi(3) * param.s2
                                + -274. * param.s1.powi(2) * param.s2.powi(2)
                                + 920. * param.s1 * param.s2.powi(3)
                                + 63. * param.s2.powi(4))
                        + 4. * param.m2_2
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 133. * param.s1.powi(3) * param.s2
                                + 358. * param.s1.powi(2) * param.s2.powi(2)
                                + 133. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + 157. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (6. * param.s1.powi(3)
                                        + 139. * param.s1.powi(2) * param.s2
                                        + 139. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3))))
                + param.m1_2
                    * param.s1.powi(2)
                    * (-3. * param.s12.powi(5)
                        + 3. * param.s12.powi(4) * (5. * param.s1 + 13. * param.s2)
                        + -6.
                            * param.s12.powi(3)
                            * (5. * param.s1.powi(2)
                                + 12. * param.s1 * param.s2
                                + -4. * param.s2.powi(2))
                        + 2. * param.s12.powi(2)
                            * (15. * param.s1.powi(3)
                                + -9. * param.s1.powi(2) * param.s2
                                + 322. * param.s1 * param.s2.powi(2)
                                + -138. * param.s2.powi(3))
                        + (param.s1 - param.s2).powi(2)
                            * (3. * param.s1.powi(3)
                                + -39. * param.s1.powi(2) * param.s2
                                + -361. * param.s1 * param.s2.powi(2)
                                + -123. * param.s2.powi(3))
                        + param.s12
                            * (-15. * param.s1.powi(4)
                                + 96. * param.s1.powi(3) * param.s2
                                + -388. * param.s1.powi(2) * param.s2.powi(2)
                                + -472. * param.s1 * param.s2.powi(3)
                                + 339. * param.s2.powi(4))
                        + 30.
                            * param.m2_2.powi(2)
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
                        + -4.
                            * param.m2_2
                            * (6. * param.s1.powi(4)
                                + 6. * param.s12.powi(4)
                                + 191. * param.s1.powi(3) * param.s2
                                + 161. * param.s1.powi(2) * param.s2.powi(2)
                                + -289. * param.s1 * param.s2.powi(3)
                                + -69. * param.s2.powi(4)
                                + param.s12.powi(3) * (-24. * param.s1 + 51. * param.s2)
                                + param.s12.powi(2)
                                    * (36. * param.s1.powi(2)
                                        + 89. * param.s1 * param.s2
                                        + -189. * param.s2.powi(2))
                                + param.s12
                                    * (-24. * param.s1.powi(3)
                                        + -331. * param.s1.powi(2) * param.s2
                                        + 224. * param.s1 * param.s2.powi(2)
                                        + 201. * param.s2.powi(3))))
                + param.m0_2
                    * (param.m1_2.powi(2)
                        * (-15. * param.s1.powi(5)
                            + 3. * param.s12.powi(5)
                            + -463. * param.s1.powi(4) * param.s2
                            + -238. * param.s1.powi(3) * param.s2.powi(2)
                            + 662. * param.s1.powi(2) * param.s2.powi(3)
                            + 57. * param.s1 * param.s2.powi(4)
                            + -3. * param.s2.powi(5)
                            + -3. * param.s12.powi(4) * (9. * param.s1 + 5. * param.s2)
                            + 6. * param.s12.powi(3)
                                * (13. * param.s1.powi(2)
                                    + 4. * param.s1 * param.s2
                                    + 5. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (51. * param.s1.powi(3)
                                    + 233. * param.s1.powi(2) * param.s2
                                    + -45. * param.s1 * param.s2.powi(2)
                                    + 15. * param.s2.powi(3))
                            + param.s12
                                * (63. * param.s1.powi(4)
                                    + 920. * param.s1.powi(3) * param.s2
                                    + -274. * param.s1.powi(2) * param.s2.powi(2)
                                    + -144. * param.s1 * param.s2.powi(3)
                                    + 15. * param.s2.powi(4)))
                        + 4. * param.m1_2
                            * param.s1
                            * (param.s12.powi(5)
                                + param.s12.powi(4) * (param.s1 + param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (7. * param.s1.powi(2)
                                        + -62. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2))
                                + 5. * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + 25. * param.s1.powi(2) * param.s2
                                        + 25. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (13. * param.s1.powi(3)
                                        + -68. * param.s1.powi(2) * param.s2
                                        + -68. * param.s1 * param.s2.powi(2)
                                        + 13. * param.s2.powi(3))
                                - param.m2_2
                                    * (-69. * param.s1.powi(4)
                                        + 6. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3)
                                            * (17. * param.s1 + -8. * param.s2)
                                        + -289. * param.s1.powi(3) * param.s2
                                        + 161. * param.s1.powi(2) * param.s2.powi(2)
                                        + 191. * param.s1 * param.s2.powi(3)
                                        + 6. * param.s2.powi(4)
                                        + param.s12.powi(2)
                                            * (-189. * param.s1.powi(2)
                                                + 89. * param.s1 * param.s2
                                                + 36. * param.s2.powi(2))
                                        + param.s12
                                            * (201. * param.s1.powi(3)
                                                + 224. * param.s1.powi(2) * param.s2
                                                + -331. * param.s1 * param.s2.powi(2)
                                                + -24. * param.s2.powi(3)))
                                - param.s12
                                    * (19. * param.s1.powi(4)
                                        + 104. * param.s1.powi(3) * param.s2
                                        + -466. * param.s1.powi(2) * param.s2.powi(2)
                                        + 104. * param.s1 * param.s2.powi(3)
                                        + 19. * param.s2.powi(4)))
                        - param.s1.powi(2)
                            * (7. * param.s12.powi(5)
                                + param.s12.powi(4) * (-23. * param.s1 + 73. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (11. * param.s1.powi(2)
                                        + 92. * param.s1 * param.s2
                                        + -106. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 96. * param.s1 * param.s2
                                        + 49. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (param.s1.powi(3)
                                        + -253. * param.s1.powi(2) * param.s2
                                        + 206. * param.s1 * param.s2.powi(2)
                                        + 64. * param.s2.powi(3))
                                + param.s12
                                    * (-13. * param.s1.powi(4)
                                        + 168. * param.s1.powi(3) * param.s2
                                        + 416. * param.s1.powi(2) * param.s2.powi(2)
                                        + -624. * param.s1 * param.s2.powi(3)
                                        + 53. * param.s2.powi(4))
                                + 30.
                                    * param.m2_2.powi(2)
                                    * (13. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2) * (param.s1 + -5. * param.s2)
                                        + 11. * param.s1.powi(2) * param.s2
                                        + -19. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-21. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + -4.
                                    * param.m2_2
                                    * (27. * param.s12.powi(4)
                                        + -33. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-63. * param.s1.powi(2)
                                                + 328. * param.s1 * param.s2
                                                + -63. * param.s2.powi(2))
                                        + -4.
                                            * (param.s1 - param.s2).powi(2)
                                            * (12. * param.s1.powi(2)
                                                + 41. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + param.s12
                                            * (117. * param.s1.powi(3)
                                                + -227. * param.s1.powi(2) * param.s2
                                                + -227. * param.s1 * param.s2.powi(2)
                                                + 117. * param.s2.powi(3)))))
                - param.m0_2.powi(2)
                    * (param.m1_2
                        * (3. * param.s12.powi(5)
                            + -3. * param.s12.powi(4) * (13. * param.s1 + 5. * param.s2)
                            + param.s12.powi(3)
                                * (-24. * param.s1.powi(2)
                                    + 72. * param.s1 * param.s2
                                    + 30. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (138. * param.s1.powi(3)
                                    + -322. * param.s1.powi(2) * param.s2
                                    + 9. * param.s1 * param.s2.powi(2)
                                    + -15. * param.s2.powi(3))
                            + (param.s1 - param.s2).powi(2)
                                * (123. * param.s1.powi(3)
                                    + 361. * param.s1.powi(2) * param.s2
                                    + 39. * param.s1 * param.s2.powi(2)
                                    + -3. * param.s2.powi(3))
                            + param.s12
                                * (-339. * param.s1.powi(4)
                                    + 472. * param.s1.powi(3) * param.s2
                                    + 388. * param.s1.powi(2) * param.s2.powi(2)
                                    + -96. * param.s1 * param.s2.powi(3)
                                    + 15. * param.s2.powi(4)))
                        + param.s1
                            * (7. * param.s12.powi(5)
                                + param.s12.powi(4) * (73. * param.s1 + -23. * param.s2)
                                + param.s12.powi(3)
                                    * (-212. * param.s1.powi(2)
                                        + 184. * param.s1 * param.s2
                                        + 22. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (64. * param.s1.powi(3)
                                        + 206. * param.s1.powi(2) * param.s2
                                        + -253. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + param.s12
                                    * (53. * param.s1.powi(4)
                                        + -624. * param.s1.powi(3) * param.s2
                                        + 416. * param.s1.powi(2) * param.s2.powi(2)
                                        + 168. * param.s1 * param.s2.powi(3)
                                        + -13. * param.s2.powi(4))
                                + -4.
                                    * param.m2_2
                                    * (3. * param.s12.powi(4)
                                        + 3. * param.s12.powi(3)
                                            * (21. * param.s1 + -4. * param.s2)
                                        + -2.
                                            * param.s12.powi(2)
                                            * (36. * param.s1.powi(2)
                                                + 34. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (63. * param.s1.powi(2)
                                                + 64. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s12
                                            * (57. * param.s1.powi(3)
                                                + -232. * param.s1.powi(2) * param.s2
                                                + 53. * param.s1 * param.s2.powi(2)
                                                + 12. * param.s2.powi(3)))
                                - (param.s1 - param.s2).powi(3)
                                    * (49. * param.s1.powi(2)
                                        + 96. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))))
                - param.s1.powi(3)
                    * (param.s12.powi(4) * (5. * param.s1 + 17. * param.s2)
                        + -2.
                            * param.s12.powi(3)
                            * (5. * param.s1.powi(2)
                                + 20. * param.s1 * param.s2
                                + -46. * param.s2.powi(2))
                        + (param.s1 - param.s2).powi(3)
                            * (param.s1.powi(2)
                                + -8. * param.s1 * param.s2
                                + -43. * param.s2.powi(2))
                        + 2. * param.s12.powi(2)
                            * (5. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + 62. * param.s1 * param.s2.powi(2)
                                + -94. * param.s2.powi(3))
                        + param.s12
                            * (-5. * param.s1.powi(4)
                                + 16. * param.s1.powi(3) * param.s2
                                + -200. * param.s1.powi(2) * param.s2.powi(2)
                                + 152. * param.s1 * param.s2.powi(3)
                                + 37. * param.s2.powi(4))
                        + -60.
                            * param.m2_2.powi(3)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + 30.
                            * param.m2_2.powi(2)
                            * (-5. * param.s1.powi(3)
                                + 5. * param.s12.powi(3)
                                + -19. * param.s1.powi(2) * param.s2
                                + 11. * param.s1 * param.s2.powi(2)
                                + 13. * param.s2.powi(3)
                                + 3. * param.s12.powi(2) * (-5. * param.s1 + param.s2)
                                + param.s12
                                    * (15. * param.s1.powi(2)
                                        + 16. * param.s1 * param.s2
                                        + -21. * param.s2.powi(2)))
                        + -4.
                            * param.m2_2
                            * (3. * param.s12.powi(4)
                                + param.s12.powi(3) * (-12. * param.s1 + 63. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + -34. * param.s1 * param.s2
                                        + -36. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 64. * param.s1 * param.s2
                                        + 63. * param.s2.powi(2))
                                - param.s12
                                    * (12. * param.s1.powi(3)
                                        + 53. * param.s1.powi(2) * param.s2
                                        + -232. * param.s1 * param.s2.powi(2)
                                        + 57. * param.s2.powi(3)))
                        - param.s12.powi(5)))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s1.powi(2)
                    * (5.
                        * param.m1_2.powi(4)
                        * param.s2.powi(2)
                        * (3. * param.s1.powi(2)
                            + 3. * param.s12.powi(2)
                            + 8. * param.s1 * param.s2
                            + 3. * param.s2.powi(2)
                            + -6. * param.s12 * (param.s1 + param.s2))
                        + param.m0_2.powi(4)
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
                        + 20.
                            * param.m1_2.powi(3)
                            * param.s2
                            * (param.s2
                                * (-2. * param.s1.powi(3)
                                    + -2. * param.s1.powi(2) * param.s2
                                    + 3. * param.s12.powi(2) * param.s2
                                    + 3. * param.s1 * param.s2.powi(2)
                                    + param.s2.powi(3)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            - param.s2.powi(2)
                                            - param.s1 * param.s2)
                                    - param.s12.powi(3))
                                + param.m2_2
                                    * (param.s12.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3)))
                        + -2.
                            * param.m0_2.powi(3)
                            * (param.s1.powi(5)
                                + 3. * param.s1.powi(4) * param.s12
                                + -12. * param.s1.powi(3) * param.s12.powi(2)
                                + 8. * param.s1.powi(2) * param.s12.powi(3)
                                + 3. * param.s1 * param.s12.powi(4)
                                + -3. * param.s12.powi(5)
                                + -3. * param.s1.powi(4) * param.s2
                                + 12. * param.s1.powi(3) * param.s12 * param.s2
                                + 18. * param.s1.powi(2) * param.s12.powi(2) * param.s2
                                + -30. * param.s1 * param.s12.powi(3) * param.s2
                                + 3. * param.s12.powi(4) * param.s2
                                + 2. * param.s1.powi(3) * param.s2.powi(2)
                                + -30. * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                                + 18. * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                                + 8. * param.s12.powi(3) * param.s2.powi(2)
                                + 2. * param.s1.powi(2) * param.s2.powi(3)
                                + 12. * param.s1 * param.s12 * param.s2.powi(3)
                                + -12. * param.s12.powi(2) * param.s2.powi(3)
                                + -3. * param.s1 * param.s2.powi(4)
                                + 3. * param.s12 * param.s2.powi(4)
                                + param.s2.powi(5)
                                + 2. * param.m2_2
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
                                + 2. * param.m1_2
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
                                            * (2. * param.s1 + 3. * param.s2)))
                        + param.s1.powi(2)
                            * (5.
                                * param.m2_2.powi(4)
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + 8. * param.s1 * param.s2
                                    + 3. * param.s2.powi(2)
                                    + -6. * param.s12 * (param.s1 + param.s2))
                                + -20.
                                    * param.m2_2.powi(3)
                                    * (-3. * param.s1 * param.s12.powi(2)
                                        + param.s12.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)
                                        + 3. * param.s12
                                            * (param.s1.powi(2) + param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s1.powi(3))
                                + param.s2.powi(2)
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
                                + -4.
                                    * param.m2_2
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
                                + 6. * param.m2_2.powi(2)
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
                                                + 2. * param.s2.powi(3))))
                        + 6. * param.m1_2.powi(2)
                            * (-2.
                                * param.m2_2
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
                                + param.m2_2.powi(2)
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
                                + param.s2.powi(2)
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
                                                + 2. * param.s2.powi(3))))
                        + -4.
                            * param.m1_2
                            * param.s1
                            * (-5.
                                * param.m2_2.powi(3)
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
                                + 3. * param.m2_2
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
                                + 3. * param.m2_2.powi(2)
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
                        + param.m0_2.powi(2)
                            * (param.s1.powi(6)
                                + -9. * param.s1.powi(4) * param.s12.powi(2)
                                + 16. * param.s1.powi(3) * param.s12.powi(3)
                                + -9. * param.s1.powi(2) * param.s12.powi(4)
                                + param.s12.powi(6)
                                + 36. * param.s1.powi(4) * param.s12 * param.s2
                                + -36. * param.s1.powi(3) * param.s12.powi(2) * param.s2
                                + -36. * param.s1.powi(2) * param.s12.powi(3) * param.s2
                                + 36. * param.s1 * param.s12.powi(4) * param.s2
                                + -9. * param.s1.powi(4) * param.s2.powi(2)
                                + -36. * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                                + 126. * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(2)
                                + -36. * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                                + -9. * param.s12.powi(4) * param.s2.powi(2)
                                + 16. * param.s1.powi(3) * param.s2.powi(3)
                                + -36. * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                                + -36. * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                                + 16. * param.s12.powi(3) * param.s2.powi(3)
                                + -9. * param.s1.powi(2) * param.s2.powi(4)
                                + 36. * param.s1 * param.s12 * param.s2.powi(4)
                                + -9. * param.s12.powi(2) * param.s2.powi(4)
                                + param.s2.powi(6)
                                + 6. * param.m1_2.powi(2)
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
                                + 6. * param.m2_2.powi(2)
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
                                + -6.
                                    * param.m2_2
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
                                + 6. * param.m1_2
                                    * (3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + -6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -4. * param.s1.powi(3) * param.s2
                                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        + 2. * param.m2_2
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
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s12.powi(5)))
                        + -2.
                            * param.m0_2
                            * (10.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (-3. * param.s1 * param.s12.powi(2)
                                    + param.s12.powi(3)
                                    + -3. * param.s1.powi(2) * param.s2
                                    + 2. * param.s1 * param.s2.powi(2)
                                    + 2. * param.s2.powi(3)
                                    + 3. * param.s12
                                        * (param.s1.powi(2) + param.s1 * param.s2
                                            - param.s2.powi(2))
                                    - param.s1.powi(3))
                                + 6. * param.m1_2.powi(2)
                                    * (param.s2
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
                                        + param.m2_2
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
                                    * (10.
                                        * param.m2_2.powi(3)
                                        * (2. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 2. * param.s1.powi(2) * param.s2
                                            + -3. * param.s12.powi(2) * param.s2
                                            + -3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s12
                                                * (param.s1 * param.s2 + param.s2.powi(2)
                                                    - param.s1.powi(2))
                                            - param.s2.powi(3))
                                        + param.s2
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
                                        + -6.
                                            * param.m2_2.powi(2)
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
                                        + 3. * param.m2_2
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
                                + 3. * param.m1_2
                                    * (2.
                                        * param.m2_2.powi(2)
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
                                        + param.s2
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
                                        + param.m2_2
                                            * (param.s12.powi(4) * (param.s1 + param.s2)
                                                + 6. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -5. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + -2.
                                                    * param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                + param.s12
                                                    * (11. * param.s1.powi(4)
                                                        + 14. * param.s1.powi(3) * param.s2
                                                        + -90.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 14. * param.s1 * param.s2.powi(3)
                                                        + 11. * param.s2.powi(4))
                                                - param.s12.powi(5)))))
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.08333333333333333
            * std::f64::consts::PI
            * param.s2.powi(-2)
            * param.lambda_s12_sqrt.powi(-9)
            * ((param.m0_2.powi(3)
                * (param.s12.powi(5)
                    + 2. * param.s12.powi(3)
                        * (5. * param.s1.powi(2)
                            + 20. * param.s1 * param.s2
                            + -46. * param.s2.powi(2))
                    + -2.
                        * param.s12.powi(2)
                        * (5. * param.s1.powi(3)
                            + 9. * param.s1.powi(2) * param.s2
                            + 62. * param.s1 * param.s2.powi(2)
                            + -94. * param.s2.powi(3))
                    + param.s12
                        * (5. * param.s1.powi(4)
                            + -16. * param.s1.powi(3) * param.s2
                            + 200. * param.s1.powi(2) * param.s2.powi(2)
                            + -152. * param.s1 * param.s2.powi(3)
                            + -37. * param.s2.powi(4))
                    - (param.s1 - param.s2).powi(3)
                        * (param.s1.powi(2)
                            + -8. * param.s1 * param.s2
                            + -43. * param.s2.powi(2))
                    - param.s12.powi(4) * (5. * param.s1 + 17. * param.s2))
                + param.m2_2.powi(3)
                    * (param.s1.powi(5)
                        + -23. * param.s1.powi(4) * param.s2
                        + -398. * param.s1.powi(3) * param.s2.powi(2)
                        + -398. * param.s1.powi(2) * param.s2.powi(3)
                        + -23. * param.s1 * param.s2.powi(4)
                        + param.s2.powi(5)
                        + 5. * param.s12.powi(4) * (param.s1 + param.s2)
                        + -2.
                            * param.s12.powi(3)
                            * (5. * param.s1.powi(2)
                                + -4. * param.s1 * param.s2
                                + 5. * param.s2.powi(2))
                        + 2. * param.s12.powi(2)
                            * (5. * param.s1.powi(3)
                                + -27. * param.s1.powi(2) * param.s2
                                + -27. * param.s1 * param.s2.powi(2)
                                + 5. * param.s2.powi(3))
                        + param.s12
                            * (-5. * param.s1.powi(4)
                                + 64. * param.s1.powi(3) * param.s2
                                + 462. * param.s1.powi(2) * param.s2.powi(2)
                                + 64. * param.s1 * param.s2.powi(3)
                                + -5. * param.s2.powi(4))
                        - param.s12.powi(5))
                + param.m2_2
                    * param.s2.powi(2)
                    * (-3. * param.s12.powi(5)
                        + 3. * param.s12.powi(4) * (13. * param.s1 + 5. * param.s2)
                        + 6. * param.s12.powi(3)
                            * (4. * param.s1.powi(2)
                                + -12. * param.s1 * param.s2
                                + -5. * param.s2.powi(2))
                        + param.s12.powi(2)
                            * (-276. * param.s1.powi(3)
                                + 644. * param.s1.powi(2) * param.s2
                                + -18. * param.s1 * param.s2.powi(2)
                                + 30. * param.s2.powi(3))
                        + param.s12
                            * (339. * param.s1.powi(4)
                                + -472. * param.s1.powi(3) * param.s2
                                + -388. * param.s1.powi(2) * param.s2.powi(2)
                                + 96. * param.s1 * param.s2.powi(3)
                                + -15. * param.s2.powi(4))
                        + 30.
                            * param.m1_2.powi(2)
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
                        + -4.
                            * param.m1_2
                            * (-69. * param.s1.powi(4)
                                + 6. * param.s12.powi(4)
                                + 3. * param.s12.powi(3) * (17. * param.s1 + -8. * param.s2)
                                + -289. * param.s1.powi(3) * param.s2
                                + 161. * param.s1.powi(2) * param.s2.powi(2)
                                + 191. * param.s1 * param.s2.powi(3)
                                + 6. * param.s2.powi(4)
                                + param.s12.powi(2)
                                    * (-189. * param.s1.powi(2)
                                        + 89. * param.s1 * param.s2
                                        + 36. * param.s2.powi(2))
                                + param.s12
                                    * (201. * param.s1.powi(3)
                                        + 224. * param.s1.powi(2) * param.s2
                                        + -331. * param.s1 * param.s2.powi(2)
                                        + -24. * param.s2.powi(3)))
                        - (param.s1 - param.s2).powi(2)
                            * (123. * param.s1.powi(3)
                                + 361. * param.s1.powi(2) * param.s2
                                + 39. * param.s1 * param.s2.powi(2)
                                + -3. * param.s2.powi(3)))
                + param.m2_2.powi(2)
                    * param.s2
                    * (-15. * param.s1.powi(5)
                        + 3. * param.s12.powi(5)
                        + -463. * param.s1.powi(4) * param.s2
                        + -238. * param.s1.powi(3) * param.s2.powi(2)
                        + 662. * param.s1.powi(2) * param.s2.powi(3)
                        + 57. * param.s1 * param.s2.powi(4)
                        + -3. * param.s2.powi(5)
                        + -3. * param.s12.powi(4) * (9. * param.s1 + 5. * param.s2)
                        + 6. * param.s12.powi(3)
                            * (13. * param.s1.powi(2)
                                + 4. * param.s1 * param.s2
                                + 5. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (51. * param.s1.powi(3)
                                + 233. * param.s1.powi(2) * param.s2
                                + -45. * param.s1 * param.s2.powi(2)
                                + 15. * param.s2.powi(3))
                        + param.s12
                            * (63. * param.s1.powi(4)
                                + 920. * param.s1.powi(3) * param.s2
                                + -274. * param.s1.powi(2) * param.s2.powi(2)
                                + -144. * param.s1 * param.s2.powi(3)
                                + 15. * param.s2.powi(4))
                        + 4. * param.m1_2
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 133. * param.s1.powi(3) * param.s2
                                + 358. * param.s1.powi(2) * param.s2.powi(2)
                                + 133. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + 157. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (6. * param.s1.powi(3)
                                        + 139. * param.s1.powi(2) * param.s2
                                        + 139. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3))))
                + param.m0_2
                    * (param.m2_2.powi(2)
                        * (-3. * param.s1.powi(5)
                            + 3. * param.s12.powi(5)
                            + 57. * param.s1.powi(4) * param.s2
                            + 662. * param.s1.powi(3) * param.s2.powi(2)
                            + -238. * param.s1.powi(2) * param.s2.powi(3)
                            + -463. * param.s1 * param.s2.powi(4)
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
                                    + 233. * param.s1 * param.s2.powi(2)
                                    + 51. * param.s2.powi(3))
                            + param.s12
                                * (15. * param.s1.powi(4)
                                    + -144. * param.s1.powi(3) * param.s2
                                    + -274. * param.s1.powi(2) * param.s2.powi(2)
                                    + 920. * param.s1 * param.s2.powi(3)
                                    + 63. * param.s2.powi(4)))
                        + 4. * param.m2_2
                            * param.s2
                            * (param.s12.powi(5)
                                + param.s12.powi(4) * (param.s1 + param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (7. * param.s1.powi(2)
                                        + -62. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2))
                                + 5. * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + 25. * param.s1.powi(2) * param.s2
                                        + 25. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (13. * param.s1.powi(3)
                                        + -68. * param.s1.powi(2) * param.s2
                                        + -68. * param.s1 * param.s2.powi(2)
                                        + 13. * param.s2.powi(3))
                                - param.m1_2
                                    * (6. * param.s1.powi(4)
                                        + 6. * param.s12.powi(4)
                                        + 191. * param.s1.powi(3) * param.s2
                                        + 161. * param.s1.powi(2) * param.s2.powi(2)
                                        + -289. * param.s1 * param.s2.powi(3)
                                        + -69. * param.s2.powi(4)
                                        + param.s12.powi(3)
                                            * (-24. * param.s1 + 51. * param.s2)
                                        + param.s12.powi(2)
                                            * (36. * param.s1.powi(2)
                                                + 89. * param.s1 * param.s2
                                                + -189. * param.s2.powi(2))
                                        + param.s12
                                            * (-24. * param.s1.powi(3)
                                                + -331. * param.s1.powi(2) * param.s2
                                                + 224. * param.s1 * param.s2.powi(2)
                                                + 201. * param.s2.powi(3)))
                                - param.s12
                                    * (19. * param.s1.powi(4)
                                        + 104. * param.s1.powi(3) * param.s2
                                        + -466. * param.s1.powi(2) * param.s2.powi(2)
                                        + 104. * param.s1 * param.s2.powi(3)
                                        + 19. * param.s2.powi(4)))
                        - param.s2.powi(2)
                            * (7. * param.s12.powi(5)
                                + param.s12.powi(4) * (73. * param.s1 + -23. * param.s2)
                                + param.s12.powi(3)
                                    * (-212. * param.s1.powi(2)
                                        + 184. * param.s1 * param.s2
                                        + 22. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (64. * param.s1.powi(3)
                                        + 206. * param.s1.powi(2) * param.s2
                                        + -253. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + param.s12
                                    * (53. * param.s1.powi(4)
                                        + -624. * param.s1.powi(3) * param.s2
                                        + 416. * param.s1.powi(2) * param.s2.powi(2)
                                        + 168. * param.s1 * param.s2.powi(3)
                                        + -13. * param.s2.powi(4))
                                + 30.
                                    * param.m1_2.powi(2)
                                    * (-5. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + -19. * param.s1.powi(2) * param.s2
                                        + 11. * param.s1 * param.s2.powi(2)
                                        + 13. * param.s2.powi(3)
                                        + 3. * param.s12.powi(2) * (-5. * param.s1 + param.s2)
                                        + param.s12
                                            * (15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -21. * param.s2.powi(2)))
                                + -4.
                                    * param.m1_2
                                    * (27. * param.s12.powi(4)
                                        + -33. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-63. * param.s1.powi(2)
                                                + 328. * param.s1 * param.s2
                                                + -63. * param.s2.powi(2))
                                        + -4.
                                            * (param.s1 - param.s2).powi(2)
                                            * (12. * param.s1.powi(2)
                                                + 41. * param.s1 * param.s2
                                                + 12. * param.s2.powi(2))
                                        + param.s12
                                            * (117. * param.s1.powi(3)
                                                + -227. * param.s1.powi(2) * param.s2
                                                + -227. * param.s1 * param.s2.powi(2)
                                                + 117. * param.s2.powi(3)))
                                - (param.s1 - param.s2).powi(3)
                                    * (49. * param.s1.powi(2)
                                        + 96. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))))
                - param.m0_2.powi(2)
                    * (param.m2_2
                        * (3. * param.s12.powi(5)
                            + -3. * param.s12.powi(4) * (5. * param.s1 + 13. * param.s2)
                            + 6. * param.s12.powi(3)
                                * (5. * param.s1.powi(2)
                                    + 12. * param.s1 * param.s2
                                    + -4. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-30. * param.s1.powi(3)
                                    + 18. * param.s1.powi(2) * param.s2
                                    + -644. * param.s1 * param.s2.powi(2)
                                    + 276. * param.s2.powi(3))
                            + param.s12
                                * (15. * param.s1.powi(4)
                                    + -96. * param.s1.powi(3) * param.s2
                                    + 388. * param.s1.powi(2) * param.s2.powi(2)
                                    + 472. * param.s1 * param.s2.powi(3)
                                    + -339. * param.s2.powi(4))
                            - (param.s1 - param.s2).powi(2)
                                * (3. * param.s1.powi(3)
                                    + -39. * param.s1.powi(2) * param.s2
                                    + -361. * param.s1 * param.s2.powi(2)
                                    + -123. * param.s2.powi(3)))
                        + param.s2
                            * (7. * param.s12.powi(5)
                                + param.s12.powi(4) * (-23. * param.s1 + 73. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (11. * param.s1.powi(2)
                                        + 92. * param.s1 * param.s2
                                        + -106. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 96. * param.s1 * param.s2
                                        + 49. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (param.s1.powi(3)
                                        + -253. * param.s1.powi(2) * param.s2
                                        + 206. * param.s1 * param.s2.powi(2)
                                        + 64. * param.s2.powi(3))
                                + param.s12
                                    * (-13. * param.s1.powi(4)
                                        + 168. * param.s1.powi(3) * param.s2
                                        + 416. * param.s1.powi(2) * param.s2.powi(2)
                                        + -624. * param.s1 * param.s2.powi(3)
                                        + 53. * param.s2.powi(4))
                                + -4.
                                    * param.m1_2
                                    * (3. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (-12. * param.s1 + 63. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -34. * param.s1 * param.s2
                                                + -36. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(2)
                                                + 64. * param.s1 * param.s2
                                                + 63. * param.s2.powi(2))
                                        - param.s12
                                            * (12. * param.s1.powi(3)
                                                + 53. * param.s1.powi(2) * param.s2
                                                + -232. * param.s1 * param.s2.powi(2)
                                                + 57. * param.s2.powi(3)))))
                - param.s2.powi(3)
                    * (param.s12.powi(4) * (17. * param.s1 + 5. * param.s2)
                        + 2. * param.s12.powi(3)
                            * (46. * param.s1.powi(2)
                                + -20. * param.s1 * param.s2
                                + -5. * param.s2.powi(2))
                        + (param.s1 - param.s2).powi(3)
                            * (43. * param.s1.powi(2) + 8. * param.s1 * param.s2
                                - param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (94. * param.s1.powi(3)
                                + -62. * param.s1.powi(2) * param.s2
                                + -9. * param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3))
                        + param.s12
                            * (37. * param.s1.powi(4)
                                + 152. * param.s1.powi(3) * param.s2
                                + -200. * param.s1.powi(2) * param.s2.powi(2)
                                + 16. * param.s1 * param.s2.powi(3)
                                + -5. * param.s2.powi(4))
                        + -60.
                            * param.m1_2.powi(3)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + 30.
                            * param.m1_2.powi(2)
                            * (13. * param.s1.powi(3)
                                + 5. * param.s12.powi(3)
                                + 3. * param.s12.powi(2) * (param.s1 + -5. * param.s2)
                                + 11. * param.s1.powi(2) * param.s2
                                + -19. * param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3)
                                + param.s12
                                    * (-21. * param.s1.powi(2)
                                        + 16. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2)))
                        + -4.
                            * param.m1_2
                            * (3. * param.s12.powi(4)
                                + 3. * param.s12.powi(3) * (21. * param.s1 + -4. * param.s2)
                                + -2.
                                    * param.s12.powi(2)
                                    * (36. * param.s1.powi(2)
                                        + 34. * param.s1 * param.s2
                                        + -9. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (63. * param.s1.powi(2)
                                        + 64. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                - param.s12
                                    * (57. * param.s1.powi(3)
                                        + -232. * param.s1.powi(2) * param.s2
                                        + 53. * param.s1 * param.s2.powi(2)
                                        + 12. * param.s2.powi(3)))
                        - param.s12.powi(5)))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s2.powi(2)
                    * (5.
                        * param.m1_2.powi(4)
                        * param.s2.powi(2)
                        * (3. * param.s1.powi(2)
                            + 3. * param.s12.powi(2)
                            + 8. * param.s1 * param.s2
                            + 3. * param.s2.powi(2)
                            + -6. * param.s12 * (param.s1 + param.s2))
                        + param.m0_2.powi(4)
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
                        + 20.
                            * param.m1_2.powi(3)
                            * param.s2
                            * (param.s2
                                * (-2. * param.s1.powi(3)
                                    + -2. * param.s1.powi(2) * param.s2
                                    + 3. * param.s12.powi(2) * param.s2
                                    + 3. * param.s1 * param.s2.powi(2)
                                    + param.s2.powi(3)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            - param.s2.powi(2)
                                            - param.s1 * param.s2)
                                    - param.s12.powi(3))
                                + param.m2_2
                                    * (param.s12.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3)))
                        + -2.
                            * param.m0_2.powi(3)
                            * (param.s1.powi(5)
                                + 3. * param.s1.powi(4) * param.s12
                                + -12. * param.s1.powi(3) * param.s12.powi(2)
                                + 8. * param.s1.powi(2) * param.s12.powi(3)
                                + 3. * param.s1 * param.s12.powi(4)
                                + -3. * param.s12.powi(5)
                                + -3. * param.s1.powi(4) * param.s2
                                + 12. * param.s1.powi(3) * param.s12 * param.s2
                                + 18. * param.s1.powi(2) * param.s12.powi(2) * param.s2
                                + -30. * param.s1 * param.s12.powi(3) * param.s2
                                + 3. * param.s12.powi(4) * param.s2
                                + 2. * param.s1.powi(3) * param.s2.powi(2)
                                + -30. * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                                + 18. * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                                + 8. * param.s12.powi(3) * param.s2.powi(2)
                                + 2. * param.s1.powi(2) * param.s2.powi(3)
                                + 12. * param.s1 * param.s12 * param.s2.powi(3)
                                + -12. * param.s12.powi(2) * param.s2.powi(3)
                                + -3. * param.s1 * param.s2.powi(4)
                                + 3. * param.s12 * param.s2.powi(4)
                                + param.s2.powi(5)
                                + 2. * param.m2_2
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
                                + 2. * param.m1_2
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
                                            * (2. * param.s1 + 3. * param.s2)))
                        + param.s1.powi(2)
                            * (5.
                                * param.m2_2.powi(4)
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + 8. * param.s1 * param.s2
                                    + 3. * param.s2.powi(2)
                                    + -6. * param.s12 * (param.s1 + param.s2))
                                + -20.
                                    * param.m2_2.powi(3)
                                    * (-3. * param.s1 * param.s12.powi(2)
                                        + param.s12.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)
                                        + 3. * param.s12
                                            * (param.s1.powi(2) + param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s1.powi(3))
                                + param.s2.powi(2)
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
                                + -4.
                                    * param.m2_2
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
                                + 6. * param.m2_2.powi(2)
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
                                                + 2. * param.s2.powi(3))))
                        + 6. * param.m1_2.powi(2)
                            * (-2.
                                * param.m2_2
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
                                + param.m2_2.powi(2)
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
                                + param.s2.powi(2)
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
                                                + 2. * param.s2.powi(3))))
                        + -4.
                            * param.m1_2
                            * param.s1
                            * (-5.
                                * param.m2_2.powi(3)
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
                                + 3. * param.m2_2
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
                                + 3. * param.m2_2.powi(2)
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
                        + param.m0_2.powi(2)
                            * (param.s1.powi(6)
                                + -9. * param.s1.powi(4) * param.s12.powi(2)
                                + 16. * param.s1.powi(3) * param.s12.powi(3)
                                + -9. * param.s1.powi(2) * param.s12.powi(4)
                                + param.s12.powi(6)
                                + 36. * param.s1.powi(4) * param.s12 * param.s2
                                + -36. * param.s1.powi(3) * param.s12.powi(2) * param.s2
                                + -36. * param.s1.powi(2) * param.s12.powi(3) * param.s2
                                + 36. * param.s1 * param.s12.powi(4) * param.s2
                                + -9. * param.s1.powi(4) * param.s2.powi(2)
                                + -36. * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                                + 126. * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(2)
                                + -36. * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                                + -9. * param.s12.powi(4) * param.s2.powi(2)
                                + 16. * param.s1.powi(3) * param.s2.powi(3)
                                + -36. * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                                + -36. * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                                + 16. * param.s12.powi(3) * param.s2.powi(3)
                                + -9. * param.s1.powi(2) * param.s2.powi(4)
                                + 36. * param.s1 * param.s12 * param.s2.powi(4)
                                + -9. * param.s12.powi(2) * param.s2.powi(4)
                                + param.s2.powi(6)
                                + 6. * param.m1_2.powi(2)
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
                                + 6. * param.m2_2.powi(2)
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
                                + -6.
                                    * param.m2_2
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
                                + 6. * param.m1_2
                                    * (3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + -6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -4. * param.s1.powi(3) * param.s2
                                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        + 2. * param.m2_2
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
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s12.powi(5)))
                        + -2.
                            * param.m0_2
                            * (10.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (-3. * param.s1 * param.s12.powi(2)
                                    + param.s12.powi(3)
                                    + -3. * param.s1.powi(2) * param.s2
                                    + 2. * param.s1 * param.s2.powi(2)
                                    + 2. * param.s2.powi(3)
                                    + 3. * param.s12
                                        * (param.s1.powi(2) + param.s1 * param.s2
                                            - param.s2.powi(2))
                                    - param.s1.powi(3))
                                + 6. * param.m1_2.powi(2)
                                    * (param.s2
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
                                        + param.m2_2
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
                                    * (10.
                                        * param.m2_2.powi(3)
                                        * (2. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 2. * param.s1.powi(2) * param.s2
                                            + -3. * param.s12.powi(2) * param.s2
                                            + -3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s12
                                                * (param.s1 * param.s2 + param.s2.powi(2)
                                                    - param.s1.powi(2))
                                            - param.s2.powi(3))
                                        + param.s2
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
                                        + -6.
                                            * param.m2_2.powi(2)
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
                                        + 3. * param.m2_2
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
                                + 3. * param.m1_2
                                    * (2.
                                        * param.m2_2.powi(2)
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
                                        + param.s2
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
                                        + param.m2_2
                                            * (param.s12.powi(4) * (param.s1 + param.s2)
                                                + 6. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -5. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + -2.
                                                    * param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                + param.s12
                                                    * (11. * param.s1.powi(4)
                                                        + 14. * param.s1.powi(3) * param.s2
                                                        + -90.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 14. * param.s1 * param.s2.powi(3)
                                                        + 11. * param.s2.powi(4))
                                                - param.s12.powi(5)))))
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.08333333333333333
            * std::f64::consts::PI
            * param.s12.powi(-4)
            * param.lambda_s12_sqrt.powi(-9)
            * ((3. * param.m2_2.powi(3) * param.s1.powi(7)
                + -27. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12
                + -5. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12
                + 114. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(2)
                + 49. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(2)
                + param.m2_2 * param.s1.powi(7) * param.s12.powi(2)
                + -330. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(3)
                + -254. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(3)
                + -17. * param.m2_2 * param.s1.powi(6) * param.s12.powi(3)
                + param.s1.powi(7) * param.s12.powi(3)
                + 345. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(4)
                + 470. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(4)
                + 58. * param.m2_2 * param.s1.powi(5) * param.s12.powi(4)
                + -5. * param.s1.powi(6) * param.s12.powi(4)
                + -33. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(5)
                + -295. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(5)
                + -82. * param.m2_2 * param.s1.powi(4) * param.s12.powi(5)
                + 10. * param.s1.powi(5) * param.s12.powi(5)
                + -78. * param.m2_2.powi(3) * param.s1 * param.s12.powi(6)
                + -25. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(6)
                + 53. * param.m2_2 * param.s1.powi(3) * param.s12.powi(6)
                + -10. * param.s1.powi(4) * param.s12.powi(6)
                + 6. * param.m2_2.powi(3) * param.s12.powi(7)
                + 66. * param.m2_2.powi(2) * param.s1 * param.s12.powi(7)
                + -13. * param.m2_2 * param.s1.powi(2) * param.s12.powi(7)
                + 5. * param.s1.powi(3) * param.s12.powi(7)
                + -6. * param.m2_2.powi(2) * param.s12.powi(8)
                + -21. * param.m2_2.powi(3) * param.s1.powi(6) * param.s2
                + 112. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12 * param.s2
                + 31. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12 * param.s2
                + -206. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + -160. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(2) * param.s2
                + param.m2_2 * param.s1.powi(6) * param.s12.powi(2) * param.s2
                + -56. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + 154. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + -64. * param.m2_2 * param.s1.powi(5) * param.s12.powi(3) * param.s2
                + -11. * param.s1.powi(6) * param.s12.powi(3) * param.s2
                + -191. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + -696. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + -254. * param.m2_2 * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + 16. * param.s1.powi(5) * param.s12.powi(4) * param.s2
                + 280. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5) * param.s2
                + 789. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(5) * param.s2
                + 648. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + 18. * param.s1.powi(4) * param.s12.powi(5) * param.s2
                + -38. * param.m2_2.powi(3) * param.s12.powi(6) * param.s2
                + -152. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6) * param.s2
                + -279. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + -40. * param.s1.powi(3) * param.s12.powi(6) * param.s2
                + 34. * param.m2_2.powi(2) * param.s12.powi(7) * param.s2
                + -56. * param.m2_2 * param.s1 * param.s12.powi(7) * param.s2
                + 17. * param.s1.powi(2) * param.s12.powi(7) * param.s2
                + 4. * param.m2_2 * param.s12.powi(8) * param.s2
                + 63. * param.m2_2.powi(3) * param.s1.powi(5) * param.s2.powi(2)
                + -155. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12 * param.s2.powi(2)
                + -81. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12 * param.s2.powi(2)
                + 10.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 141.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -21. * param.m2_2 * param.s1.powi(5) * param.s12.powi(2) * param.s2.powi(2)
                + 126.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 330.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 249. * param.m2_2 * param.s1.powi(4) * param.s12.powi(3) * param.s2.powi(2)
                + 27. * param.s1.powi(5) * param.s12.powi(3) * param.s2.powi(2)
                + -347. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + -590.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -384. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(2)
                + -163. * param.s1.powi(4) * param.s12.powi(4) * param.s2.powi(2)
                + 103. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2.powi(2)
                + 21. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2.powi(2)
                + -344. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(2)
                + -64. * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(2)
                + -81. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2.powi(2)
                + 129. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + 184. * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(2)
                + -21. * param.m2_2 * param.s12.powi(7) * param.s2.powi(2)
                + 17. * param.s1 * param.s12.powi(7) * param.s2.powi(2)
                + -105. * param.m2_2.powi(3) * param.s1.powi(4) * param.s2.powi(3)
                + 40. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12 * param.s2.powi(3)
                + 115. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12 * param.s2.powi(3)
                + 66.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 56.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 55. * param.m2_2 * param.s1.powi(4) * param.s12.powi(2) * param.s2.powi(3)
                + 120. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + -70.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -184. * param.m2_2 * param.s1.powi(3) * param.s12.powi(3) * param.s2.powi(3)
                + -17. * param.s1.powi(4) * param.s12.powi(3) * param.s2.powi(3)
                + -155. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(3)
                + 200. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + 644. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(3)
                + 304. * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(3)
                + 105. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(3)
                + -40. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + -64. * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(3)
                + 45. * param.m2_2 * param.s12.powi(6) * param.s2.powi(3)
                + -40. * param.s1 * param.s12.powi(6) * param.s2.powi(3)
                + 5. * param.s12.powi(7) * param.s2.powi(3)
                + 105. * param.m2_2.powi(3) * param.s1.powi(3) * param.s2.powi(4)
                + 95. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12 * param.s2.powi(4)
                + -95. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12 * param.s2.powi(4)
                + 92. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + -149.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -65. * param.m2_2 * param.s1.powi(3) * param.s12.powi(2) * param.s2.powi(4)
                + 140. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(4)
                + -196. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + -47. * param.m2_2 * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(4)
                + -17. * param.s1.powi(3) * param.s12.powi(3) * param.s2.powi(4)
                + -80. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(4)
                + -94. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + -163. * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(4)
                + -50. * param.m2_2 * param.s12.powi(5) * param.s2.powi(4)
                + 18. * param.s1 * param.s12.powi(5) * param.s2.powi(4)
                + -10. * param.s12.powi(6) * param.s2.powi(4)
                + -63. * param.m2_2.powi(3) * param.s1.powi(2) * param.s2.powi(5)
                + -88. * param.m2_2.powi(3) * param.s1 * param.s12 * param.s2.powi(5)
                + 45. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12 * param.s2.powi(5)
                + -76. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(5)
                + 72. * param.m2_2.powi(2) * param.s1 * param.s12.powi(2) * param.s2.powi(5)
                + 39. * param.m2_2 * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(5)
                + 36. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(5)
                + 72. * param.m2_2 * param.s1 * param.s12.powi(3) * param.s2.powi(5)
                + 27. * param.s1.powi(2) * param.s12.powi(3) * param.s2.powi(5)
                + 30. * param.m2_2 * param.s12.powi(4) * param.s2.powi(5)
                + 16. * param.s1 * param.s12.powi(4) * param.s2.powi(5)
                + 10. * param.s12.powi(5) * param.s2.powi(5)
                + 21. * param.m2_2.powi(3) * param.s1 * param.s2.powi(6)
                + 23. * param.m2_2.powi(3) * param.s12 * param.s2.powi(6)
                + -11. * param.m2_2.powi(2) * param.s1 * param.s12 * param.s2.powi(6)
                + -9. * param.m2_2.powi(2) * param.s12.powi(2) * param.s2.powi(6)
                + -11. * param.m2_2 * param.s1 * param.s12.powi(2) * param.s2.powi(6)
                + -9. * param.m2_2 * param.s12.powi(3) * param.s2.powi(6)
                + -11. * param.s1 * param.s12.powi(3) * param.s2.powi(6)
                + -5. * param.s12.powi(4) * param.s2.powi(6)
                + -3. * param.m2_2.powi(3) * param.s2.powi(7)
                + param.m2_2.powi(2) * param.s12 * param.s2.powi(7)
                + param.m2_2 * param.s12.powi(2) * param.s2.powi(7)
                + param.s12.powi(3) * param.s2.powi(7)
                + 12.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (6. * param.s12.powi(4)
                        + (param.s1 - param.s2).powi(4)
                        + -4. * param.s12.powi(3) * (param.s1 + param.s2)
                        + 6. * param.s12 * (param.s1 - param.s2).powi(2) * (param.s1 + param.s2)
                        + -3.
                            * param.s12.powi(2)
                            * (3. * param.s1.powi(2)
                                + -8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)))
                + param.m1_2.powi(3)
                    * (6. * param.s12.powi(7)
                        + -3. * (param.s1 - param.s2).powi(7)
                        + param.s12
                            * (param.s1 - param.s2).powi(5)
                            * (23. * param.s1 + 27. * param.s2)
                        + -2. * param.s12.powi(6) * (19. * param.s1 + 39. * param.s2)
                        + param.s12.powi(5)
                            * (103. * param.s1.powi(2)
                                + 280. * param.s1 * param.s2
                                + -33. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (param.s1 - param.s2).powi(3)
                            * (38. * param.s1.powi(2)
                                + 68. * param.s1 * param.s2
                                + 57. * param.s2.powi(2))
                        + 2. * param.s12.powi(3)
                            * (70. * param.s1.powi(4)
                                + 60. * param.s1.powi(3) * param.s2
                                + 63. * param.s1.powi(2) * param.s2.powi(2)
                                + -28. * param.s1 * param.s2.powi(3)
                                + -165. * param.s2.powi(4))
                        - param.s12.powi(4)
                            * (155. * param.s1.powi(3)
                                + 347. * param.s1.powi(2) * param.s2
                                + 191. * param.s1 * param.s2.powi(2)
                                + -345. * param.s2.powi(3)))
                + -6.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.s12
                        * (-6. * param.s12.powi(5)
                            + 2. * param.s12.powi(4) * (param.s1 + param.s2)
                            + 3. * (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (7. * param.s1.powi(2)
                                    + 58. * param.s1 * param.s2
                                    + 7. * param.s2.powi(2))
                            + param.s12.powi(3)
                                * (27. * param.s1.powi(2)
                                    + -88. * param.s1 * param.s2
                                    + 27. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-33. * param.s1.powi(3)
                                    + 51. * param.s1.powi(2) * param.s2
                                    + 51. * param.s1 * param.s2.powi(2)
                                    + -33. * param.s2.powi(3)))
                        + param.m2_2
                            * (18. * param.s12.powi(5)
                                + param.s12.powi(4) * (22. * param.s1 + -46. * param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (17. * param.s1 + 11. * param.s2)
                                + param.s12.powi(3)
                                    * (-83. * param.s1.powi(2)
                                        + 72. * param.s1 * param.s2
                                        + 29. * param.s2.powi(2))
                                + 3. * param.s12.powi(2)
                                    * (9. * param.s1.powi(3)
                                        + 23. * param.s1.powi(2) * param.s2
                                        + -35. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(5))
                        + param.m1_2
                            * (18. * param.s12.powi(5)
                                + (param.s1 - param.s2).powi(5)
                                + param.s12.powi(4) * (-46. * param.s1 + 22. * param.s2)
                                + param.s12.powi(3)
                                    * (29. * param.s1.powi(2)
                                        + 72. * param.s1 * param.s2
                                        + -83. * param.s2.powi(2))
                                + 3. * param.s12.powi(2)
                                    * (3. * param.s1.powi(3)
                                        + -35. * param.s1.powi(2) * param.s2
                                        + 23. * param.s1 * param.s2.powi(2)
                                        + 9. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (11. * param.s1 + 17. * param.s2)))
                + -4.
                    * param.m0_2
                    * param.s12
                    * (param.m2_2
                        * param.s12
                        * (3. * param.s12.powi(6)
                            + 8. * param.s12.powi(5) * (5. * param.s1 - param.s2)
                            + (param.s1 - param.s2).powi(5) * (2. * param.s1 + param.s2)
                            + param.s12.powi(4)
                                * (-113. * param.s1.powi(2)
                                    + 92. * param.s1 * param.s2
                                    + param.s2.powi(2))
                            + param.s12.powi(3)
                                * (67. * param.s1.powi(3)
                                    + 200. * param.s1.powi(2) * param.s2
                                    + -277. * param.s1 * param.s2.powi(2)
                                    + 16. * param.s2.powi(3))
                            + param.s12.powi(2)
                                * (32. * param.s1.powi(4)
                                    + -323. * param.s1.powi(3) * param.s2
                                    + 189. * param.s1.powi(2) * param.s2.powi(2)
                                    + 121. * param.s1 * param.s2.powi(3)
                                    + -19. * param.s2.powi(4))
                            - param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (31. * param.s1.powi(2)
                                    + 45. * param.s1 * param.s2
                                    + 8. * param.s2.powi(2)))
                        + param.m1_2
                            * (param.m2_2
                                * (-42. * param.s12.powi(6)
                                    + 2. * (param.s1 - param.s2).powi(6)
                                    + 82. * param.s12.powi(5) * (param.s1 + param.s2)
                                    + -19.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (param.s1 + param.s2)
                                    + param.s12.powi(4)
                                        * (19. * param.s1.powi(2)
                                            + -424. * param.s1 * param.s2
                                            + 19. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (89. * param.s1.powi(2)
                                            + 170. * param.s1 * param.s2
                                            + 89. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (-131. * param.s1.powi(3)
                                            + 305. * param.s1.powi(2) * param.s2
                                            + 305. * param.s1 * param.s2.powi(2)
                                            + -131. * param.s2.powi(3)))
                                + param.s12
                                    * (3. * param.s12.powi(6)
                                        + -8. * param.s12.powi(5) * (param.s1 + -5. * param.s2)
                                        + param.s12.powi(4)
                                            * (param.s1.powi(2)
                                                + 92. * param.s1 * param.s2
                                                + -113. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (8. * param.s1.powi(2)
                                                + 45. * param.s1 * param.s2
                                                + 31. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (16. * param.s1.powi(3)
                                                + -277. * param.s1.powi(2) * param.s2
                                                + 200. * param.s1 * param.s2.powi(2)
                                                + 67. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-19. * param.s1.powi(4)
                                                + 121. * param.s1.powi(3) * param.s2
                                                + 189. * param.s1.powi(2) * param.s2.powi(2)
                                                + -323. * param.s1 * param.s2.powi(3)
                                                + 32. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(5)
                                            * (param.s1 + 2. * param.s2)))
                        - param.s12.powi(2)
                            * (2. * param.s12.powi(5) * (param.s1 + param.s2)
                                + param.s12.powi(4)
                                    * (-7. * param.s1.powi(2)
                                        + 46. * param.s1 * param.s2
                                        + -7. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(4)
                                    * (param.s1.powi(2)
                                        + 7. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + -28. * param.s1.powi(2) * param.s2
                                        + -28. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 8. * param.s12.powi(3)
                                    * (param.s1.powi(3)
                                        + -4. * param.s1.powi(2) * param.s2
                                        + -4. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (2. * param.s1.powi(4)
                                        + 79. * param.s1.powi(3) * param.s2
                                        + -216. * param.s1.powi(2) * param.s2.powi(2)
                                        + 79. * param.s1 * param.s2.powi(3)
                                        + 2. * param.s2.powi(4)))
                        - param.m2_2.powi(2)
                            * (6. * param.s12.powi(6)
                                + param.s12.powi(5) * (74. * param.s1 + -28. * param.s2)
                                + (param.s1 - param.s2).powi(6)
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (35. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 14. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-115. * param.s1.powi(2)
                                        + -104. * param.s1 * param.s2
                                        + 53. * param.s2.powi(2))
                                - param.s12.powi(3)
                                    * (25. * param.s1.powi(3)
                                        + -256. * param.s1.powi(2) * param.s2
                                        + 5. * param.s1 * param.s2.powi(2)
                                        + 52. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (11. * param.s1 + 8. * param.s2))
                        - param.m1_2.powi(2)
                            * (6. * param.s12.powi(6)
                                + (param.s1 - param.s2).powi(6)
                                + param.s12.powi(5) * (-28. * param.s1 + 74. * param.s2)
                                + param.s12.powi(4)
                                    * (53. * param.s1.powi(2)
                                        + -104. * param.s1 * param.s2
                                        + -115. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (14. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 35. * param.s2.powi(2))
                                - param.s12.powi(3)
                                    * (52. * param.s1.powi(3)
                                        + 5. * param.s1.powi(2) * param.s2
                                        + -256. * param.s1 * param.s2.powi(2)
                                        + 25. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (8. * param.s1 + 11. * param.s2)))
                - param.m1_2
                    * (-4.
                        * param.m2_2
                        * param.s12
                        * (10. * param.s12.powi(6) * (param.s1 + param.s2)
                            + (param.s1 - param.s2).powi(6) * (param.s1 + param.s2)
                            + param.s12.powi(5)
                                * (-45. * param.s1.powi(2)
                                    + 128. * param.s1 * param.s2
                                    + -45. * param.s2.powi(2))
                            + 6. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (6. * param.s1.powi(3)
                                    + 23. * param.s1.powi(2) * param.s2
                                    + 23. * param.s1 * param.s2.powi(2)
                                    + 6. * param.s2.powi(3))
                            + param.s12.powi(4)
                                * (81. * param.s1.powi(3)
                                    + -203. * param.s1.powi(2) * param.s2
                                    + -203. * param.s1 * param.s2.powi(2)
                                    + 81. * param.s2.powi(3))
                            + -2.
                                * param.s12.powi(3)
                                * (37. * param.s1.powi(4)
                                    + 6. * param.s1.powi(3) * param.s2
                                    + -260. * param.s1.powi(2) * param.s2.powi(2)
                                    + 6. * param.s1 * param.s2.powi(3)
                                    + 37. * param.s2.powi(4))
                            - param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (9. * param.s1.powi(2)
                                    + 20. * param.s1 * param.s2
                                    + 9. * param.s2.powi(2)))
                        + param.m2_2.powi(2)
                            * (42. * param.s12.powi(7)
                                + param.s12.powi(6) * (62. * param.s1 + -226. * param.s2)
                                + 9. * (param.s1 - param.s2).powi(7)
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (149. * param.s1.powi(2)
                                        + 210. * param.s1 * param.s2
                                        + 130. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-559. * param.s1.powi(2)
                                        + 424. * param.s1 * param.s2
                                        + 521. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (935. * param.s1.powi(3)
                                        + 451. * param.s1.powi(2) * param.s2
                                        + -1061. * param.s1 * param.s2.powi(2)
                                        + -673. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-710. * param.s1.powi(4)
                                        + -424. * param.s1.powi(3) * param.s2
                                        + 162. * param.s1.powi(2) * param.s2.powi(2)
                                        + 440. * param.s1 * param.s2.powi(3)
                                        + 532. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (77. * param.s1 + 73. * param.s2))
                        + param.s12.powi(2)
                            * (-4. * param.s1 * param.s12.powi(6)
                                + param.s12.powi(5)
                                    * (21. * param.s1.powi(2)
                                        + 56. * param.s1 * param.s2
                                        + 13. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (9. * param.s1.powi(3)
                                        + -45. * param.s1.powi(2) * param.s2
                                        + -115. * param.s1 * param.s2.powi(2)
                                        + -17. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (50. * param.s1.powi(4)
                                        + 40. * param.s1.powi(3) * param.s2
                                        + 344. * param.s1.powi(2) * param.s2.powi(2)
                                        + -648. * param.s1 * param.s2.powi(3)
                                        + 82. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-30. * param.s1.powi(5)
                                        + 94. * param.s1.powi(4) * param.s2
                                        + -644. * param.s1.powi(3) * param.s2.powi(2)
                                        + 384. * param.s1.powi(2) * param.s2.powi(3)
                                        + 254. * param.s1 * param.s2.powi(4)
                                        + -58. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (45. * param.s1.powi(3)
                                        + 129. * param.s1.powi(2) * param.s2
                                        + -279. * param.s1 * param.s2.powi(2)
                                        + 53. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(5)
                                    * (param.s1.powi(2) + -6. * param.s1 * param.s2
                                        - param.s2.powi(2))))
                - param.m1_2.powi(2)
                    * (param.s12
                        * (6. * param.s12.powi(7)
                            + -2. * param.s12.powi(6) * (17. * param.s1 + 33. * param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (9. * param.s1.powi(2)
                                    + -36. * param.s1 * param.s2
                                    + -49. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (81. * param.s1.powi(2)
                                    + 152. * param.s1 * param.s2
                                    + 25. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (18. * param.s1.powi(3)
                                    + -62. * param.s1.powi(2) * param.s2
                                    + -177. * param.s1 * param.s2.powi(2)
                                    + -127. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (80. * param.s1.powi(4)
                                    + -200. * param.s1.powi(3) * param.s2
                                    + 590. * param.s1.powi(2) * param.s2.powi(2)
                                    + 696. * param.s1 * param.s2.powi(3)
                                    + -470. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (105. * param.s1.powi(3)
                                    + 21. * param.s1.powi(2) * param.s2
                                    + 789. * param.s1 * param.s2.powi(2)
                                    + -295. * param.s2.powi(3))
                            - (param.s1 + -5. * param.s2) * (param.s1 - param.s2).powi(6))
                        + param.m2_2
                            * (42. * param.s12.powi(7)
                                + -9. * (param.s1 - param.s2).powi(7)
                                + param.s12.powi(6) * (-226. * param.s1 + 62. * param.s2)
                                + param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (73. * param.s1 + 77. * param.s2)
                                + param.s12.powi(5)
                                    * (521. * param.s1.powi(2)
                                        + 424. * param.s1 * param.s2
                                        + -559. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (130. * param.s1.powi(2)
                                        + 210. * param.s1 * param.s2
                                        + 149. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-673. * param.s1.powi(3)
                                        + -1061. * param.s1.powi(2) * param.s2
                                        + 451. * param.s1 * param.s2.powi(2)
                                        + 935. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (266. * param.s1.powi(4)
                                        + 220. * param.s1.powi(3) * param.s2
                                        + 81. * param.s1.powi(2) * param.s2.powi(2)
                                        + -212. * param.s1 * param.s2.powi(3)
                                        + -355. * param.s2.powi(4))))
                - param.s12.powi(8) * param.s2.powi(2)
                - param.s1.powi(2) * param.s12.powi(8))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 12.
                    * param.s12.powi(4)
                    * (5.
                        * param.m1_2.powi(4)
                        * param.s2.powi(2)
                        * (3. * param.s1.powi(2)
                            + 3. * param.s12.powi(2)
                            + 8. * param.s1 * param.s2
                            + 3. * param.s2.powi(2)
                            + -6. * param.s12 * (param.s1 + param.s2))
                        + param.m0_2.powi(4)
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
                        + 20.
                            * param.m1_2.powi(3)
                            * param.s2
                            * (param.s2
                                * (-2. * param.s1.powi(3)
                                    + -2. * param.s1.powi(2) * param.s2
                                    + 3. * param.s12.powi(2) * param.s2
                                    + 3. * param.s1 * param.s2.powi(2)
                                    + param.s2.powi(3)
                                    + 3. * param.s12
                                        * (param.s1.powi(2)
                                            - param.s2.powi(2)
                                            - param.s1 * param.s2)
                                    - param.s12.powi(3))
                                + param.m2_2
                                    * (param.s12.powi(3)
                                        + -6. * param.s1.powi(2) * param.s2
                                        + -6. * param.s1 * param.s2.powi(2)
                                        + -3. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 3. * param.s12
                                            * (param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        - param.s2.powi(3)
                                        - param.s1.powi(3)))
                        + -2.
                            * param.m0_2.powi(3)
                            * (param.s1.powi(5)
                                + 3. * param.s1.powi(4) * param.s12
                                + -12. * param.s1.powi(3) * param.s12.powi(2)
                                + 8. * param.s1.powi(2) * param.s12.powi(3)
                                + 3. * param.s1 * param.s12.powi(4)
                                + -3. * param.s12.powi(5)
                                + -3. * param.s1.powi(4) * param.s2
                                + 12. * param.s1.powi(3) * param.s12 * param.s2
                                + 18. * param.s1.powi(2) * param.s12.powi(2) * param.s2
                                + -30. * param.s1 * param.s12.powi(3) * param.s2
                                + 3. * param.s12.powi(4) * param.s2
                                + 2. * param.s1.powi(3) * param.s2.powi(2)
                                + -30. * param.s1.powi(2) * param.s12 * param.s2.powi(2)
                                + 18. * param.s1 * param.s12.powi(2) * param.s2.powi(2)
                                + 8. * param.s12.powi(3) * param.s2.powi(2)
                                + 2. * param.s1.powi(2) * param.s2.powi(3)
                                + 12. * param.s1 * param.s12 * param.s2.powi(3)
                                + -12. * param.s12.powi(2) * param.s2.powi(3)
                                + -3. * param.s1 * param.s2.powi(4)
                                + 3. * param.s12 * param.s2.powi(4)
                                + param.s2.powi(5)
                                + 2. * param.m2_2
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
                                + 2. * param.m1_2
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
                                            * (2. * param.s1 + 3. * param.s2)))
                        + param.s1.powi(2)
                            * (5.
                                * param.m2_2.powi(4)
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + 8. * param.s1 * param.s2
                                    + 3. * param.s2.powi(2)
                                    + -6. * param.s12 * (param.s1 + param.s2))
                                + -20.
                                    * param.m2_2.powi(3)
                                    * (-3. * param.s1 * param.s12.powi(2)
                                        + param.s12.powi(3)
                                        + -3. * param.s1.powi(2) * param.s2
                                        + 2. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3)
                                        + 3. * param.s12
                                            * (param.s1.powi(2) + param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s1.powi(3))
                                + param.s2.powi(2)
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
                                + -4.
                                    * param.m2_2
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
                                + 6. * param.m2_2.powi(2)
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
                                                + 2. * param.s2.powi(3))))
                        + 6. * param.m1_2.powi(2)
                            * (-2.
                                * param.m2_2
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
                                + param.m2_2.powi(2)
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
                                + param.s2.powi(2)
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
                                                + 2. * param.s2.powi(3))))
                        + -4.
                            * param.m1_2
                            * param.s1
                            * (-5.
                                * param.m2_2.powi(3)
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
                                + 3. * param.m2_2
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
                                + 3. * param.m2_2.powi(2)
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
                        + param.m0_2.powi(2)
                            * (param.s1.powi(6)
                                + -9. * param.s1.powi(4) * param.s12.powi(2)
                                + 16. * param.s1.powi(3) * param.s12.powi(3)
                                + -9. * param.s1.powi(2) * param.s12.powi(4)
                                + param.s12.powi(6)
                                + 36. * param.s1.powi(4) * param.s12 * param.s2
                                + -36. * param.s1.powi(3) * param.s12.powi(2) * param.s2
                                + -36. * param.s1.powi(2) * param.s12.powi(3) * param.s2
                                + 36. * param.s1 * param.s12.powi(4) * param.s2
                                + -9. * param.s1.powi(4) * param.s2.powi(2)
                                + -36. * param.s1.powi(3) * param.s12 * param.s2.powi(2)
                                + 126. * param.s1.powi(2) * param.s12.powi(2) * param.s2.powi(2)
                                + -36. * param.s1 * param.s12.powi(3) * param.s2.powi(2)
                                + -9. * param.s12.powi(4) * param.s2.powi(2)
                                + 16. * param.s1.powi(3) * param.s2.powi(3)
                                + -36. * param.s1.powi(2) * param.s12 * param.s2.powi(3)
                                + -36. * param.s1 * param.s12.powi(2) * param.s2.powi(3)
                                + 16. * param.s12.powi(3) * param.s2.powi(3)
                                + -9. * param.s1.powi(2) * param.s2.powi(4)
                                + 36. * param.s1 * param.s12 * param.s2.powi(4)
                                + -9. * param.s12.powi(2) * param.s2.powi(4)
                                + param.s2.powi(6)
                                + 6. * param.m1_2.powi(2)
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
                                + 6. * param.m2_2.powi(2)
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
                                + -6.
                                    * param.m2_2
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
                                + 6. * param.m1_2
                                    * (3. * param.s12.powi(4) * (param.s1 - param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + -6. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -15. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 3. * param.s12
                                            * (param.s1.powi(4)
                                                + -4. * param.s1.powi(3) * param.s2
                                                + -6. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10. * param.s1 * param.s2.powi(3)
                                                - param.s2.powi(4))
                                        + 2. * param.m2_2
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
                                        - (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        - param.s12.powi(5)))
                        + -2.
                            * param.m0_2
                            * (10.
                                * param.m1_2.powi(3)
                                * param.s2
                                * (-3. * param.s1 * param.s12.powi(2)
                                    + param.s12.powi(3)
                                    + -3. * param.s1.powi(2) * param.s2
                                    + 2. * param.s1 * param.s2.powi(2)
                                    + 2. * param.s2.powi(3)
                                    + 3. * param.s12
                                        * (param.s1.powi(2) + param.s1 * param.s2
                                            - param.s2.powi(2))
                                    - param.s1.powi(3))
                                + 6. * param.m1_2.powi(2)
                                    * (param.s2
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
                                        + param.m2_2
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
                                    * (10.
                                        * param.m2_2.powi(3)
                                        * (2. * param.s1.powi(3)
                                            + param.s12.powi(3)
                                            + 2. * param.s1.powi(2) * param.s2
                                            + -3. * param.s12.powi(2) * param.s2
                                            + -3. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s12
                                                * (param.s1 * param.s2 + param.s2.powi(2)
                                                    - param.s1.powi(2))
                                            - param.s2.powi(3))
                                        + param.s2
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
                                        + -6.
                                            * param.m2_2.powi(2)
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
                                        + 3. * param.m2_2
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
                                + 3. * param.m1_2
                                    * (2.
                                        * param.m2_2.powi(2)
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
                                        + param.s2
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
                                        + param.m2_2
                                            * (param.s12.powi(4) * (param.s1 + param.s2)
                                                + 6. * param.s12.powi(3)
                                                    * (param.s1.powi(2)
                                                        + -5. * param.s1 * param.s2
                                                        + param.s2.powi(2))
                                                + -3.
                                                    * (param.s1 - param.s2).powi(2)
                                                    * (param.s1.powi(3)
                                                        + 9. * param.s1.powi(2) * param.s2
                                                        + 9. * param.s1 * param.s2.powi(2)
                                                        + param.s2.powi(3))
                                                + -2.
                                                    * param.s12.powi(2)
                                                    * (7. * param.s1.powi(3)
                                                        + -18. * param.s1.powi(2) * param.s2
                                                        + -18. * param.s1 * param.s2.powi(2)
                                                        + 7. * param.s2.powi(3))
                                                + param.s12
                                                    * (11. * param.s1.powi(4)
                                                        + 14. * param.s1.powi(3) * param.s2
                                                        + -90.
                                                            * param.s1.powi(2)
                                                            * param.s2.powi(2)
                                                        + 14. * param.s1 * param.s2.powi(3)
                                                        + 11. * param.s2.powi(4))
                                                - param.s12.powi(5)))))
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
