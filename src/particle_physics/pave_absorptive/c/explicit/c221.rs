use super::{log_diff, Parameters};

pub(crate) fn c221(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.0000992063492063492
            * std::f64::consts::PI
            * param.s1.powi(-4)
            * param.lambda_s12_sqrt.powi(-11)
            * ((param.m0_2.powi(6)
                * (3. * param.s12.powi(8)
                    + -2. * param.s12.powi(7) * (19. * param.s1 + 12. * param.s2)
                    + (param.s1 - param.s2).powi(6)
                        * (10. * param.s1.powi(2)
                            + -10. * param.s1 * param.s2
                            + 3. * param.s2.powi(2))
                    + param.s12.powi(6)
                        * (245. * param.s1.powi(2)
                            + 200. * param.s1 * param.s2
                            + 84. * param.s2.powi(2))
                    + -2.
                        * param.s12
                        * (param.s1 - param.s2).powi(4)
                        * (110. * param.s1.powi(3)
                            + -45. * param.s1.powi(2) * param.s2
                            + -17. * param.s1 * param.s2.powi(2)
                            + 12. * param.s2.powi(3))
                    + -2.
                        * param.s12.powi(5)
                        * (630. * param.s1.powi(3)
                            + 355. * param.s1.powi(2) * param.s2
                            + 201. * param.s1 * param.s2.powi(2)
                            + 84. * param.s2.powi(3))
                    + param.s12.powi(4)
                        * (-525. * param.s1.powi(4)
                            + 970. * param.s1.powi(3) * param.s2
                            + 505. * param.s1.powi(2) * param.s2.powi(2)
                            + 340. * param.s1 * param.s2.powi(3)
                            + 210. * param.s2.powi(4))
                    + 2. * param.s12.powi(3)
                        * (1911. * param.s1.powi(5)
                            + -3190. * param.s1.powi(4) * param.s2
                            + 510. * param.s1.powi(3) * param.s2.powi(2)
                            + 90. * param.s1.powi(2) * param.s2.powi(3)
                            + -5. * param.s1 * param.s2.powi(4)
                            + -84. * param.s2.powi(5))
                    - param.s12.powi(2)
                        * (param.s1 - param.s2).powi(2)
                        * (2037. * param.s1.powi(4)
                            + 710. * param.s1.powi(3) * param.s2
                            + 277. * param.s1.powi(2) * param.s2.powi(2)
                            + 24. * param.s1 * param.s2.powi(3)
                            + -84. * param.s2.powi(4)))
                + 3. * param.m1_2.powi(6)
                    * (param.s1.powi(8)
                        + param.s12.powi(8)
                        + -14. * param.s1.powi(7) * param.s2
                        + 106. * param.s1.powi(6) * param.s2.powi(2)
                        + -734. * param.s1.powi(5) * param.s2.powi(3)
                        + -3758. * param.s1.powi(4) * param.s2.powi(4)
                        + -734. * param.s1.powi(3) * param.s2.powi(5)
                        + 106. * param.s1.powi(2) * param.s2.powi(6)
                        + -14. * param.s1 * param.s2.powi(7)
                        + param.s2.powi(8)
                        + -8. * param.s12.powi(7) * (param.s1 + param.s2)
                        + param.s12.powi(6)
                            * (28. * param.s1.powi(2)
                                + 34. * param.s1 * param.s2
                                + 28. * param.s2.powi(2))
                        + -4.
                            * param.s12.powi(5)
                            * (14. * param.s1.powi(3)
                                + 9. * param.s1.powi(2) * param.s2
                                + 9. * param.s1 * param.s2.powi(2)
                                + 14. * param.s2.powi(3))
                        + 10.
                            * param.s12.powi(4)
                            * (7. * param.s1.powi(4)
                                + -5. * param.s1.powi(3) * param.s2
                                + -3. * param.s1.powi(2) * param.s2.powi(2)
                                + -5. * param.s1 * param.s2.powi(3)
                                + 7. * param.s2.powi(4))
                        + -8.
                            * param.s12.powi(3)
                            * (7. * param.s1.powi(5)
                                + -20. * param.s1.powi(4) * param.s2
                                + 10. * param.s1.powi(3) * param.s2.powi(2)
                                + 10. * param.s1.powi(2) * param.s2.powi(3)
                                + -20. * param.s1 * param.s2.powi(4)
                                + 7. * param.s2.powi(5))
                        + 2. * param.s12.powi(2)
                            * (14. * param.s1.powi(6)
                                + -81. * param.s1.powi(5) * param.s2
                                + 180. * param.s1.powi(4) * param.s2.powi(2)
                                + -100. * param.s1.powi(3) * param.s2.powi(3)
                                + 180. * param.s1.powi(2) * param.s2.powi(4)
                                + -81. * param.s1 * param.s2.powi(5)
                                + 14. * param.s2.powi(6))
                        + -4.
                            * param.s12
                            * (2. * param.s1.powi(7)
                                + -19. * param.s1.powi(6) * param.s2
                                + 87. * param.s1.powi(5) * param.s2.powi(2)
                                + -280. * param.s1.powi(4) * param.s2.powi(3)
                                + -280. * param.s1.powi(3) * param.s2.powi(4)
                                + 87. * param.s1.powi(2) * param.s2.powi(5)
                                + -19. * param.s1 * param.s2.powi(6)
                                + 2. * param.s2.powi(7)))
                + param.s1.powi(6)
                    * (3. * param.s12.powi(8)
                        + -2. * param.s12.powi(7) * (12. * param.s1 + 19. * param.s2)
                        + (param.s1 - param.s2).powi(6)
                            * (3. * param.s1.powi(2)
                                + -10. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        + param.s12.powi(6)
                            * (84. * param.s1.powi(2)
                                + 200. * param.s1 * param.s2
                                + 245. * param.s2.powi(2))
                        + -2.
                            * param.s12
                            * (param.s1 - param.s2).powi(4)
                            * (12. * param.s1.powi(3)
                                + -17. * param.s1.powi(2) * param.s2
                                + -45. * param.s1 * param.s2.powi(2)
                                + 110. * param.s2.powi(3))
                        + -2.
                            * param.s12.powi(5)
                            * (84. * param.s1.powi(3)
                                + 201. * param.s1.powi(2) * param.s2
                                + 355. * param.s1 * param.s2.powi(2)
                                + 630. * param.s2.powi(3))
                        + param.s12.powi(2)
                            * (param.s1 - param.s2).powi(2)
                            * (84. * param.s1.powi(4)
                                + -24. * param.s1.powi(3) * param.s2
                                + -277. * param.s1.powi(2) * param.s2.powi(2)
                                + -710. * param.s1 * param.s2.powi(3)
                                + -2037. * param.s2.powi(4))
                        + 5. * param.s12.powi(4)
                            * (42. * param.s1.powi(4)
                                + 68. * param.s1.powi(3) * param.s2
                                + 101. * param.s1.powi(2) * param.s2.powi(2)
                                + 194. * param.s1 * param.s2.powi(3)
                                + -105. * param.s2.powi(4))
                        + -2.
                            * param.s12.powi(3)
                            * (84. * param.s1.powi(5)
                                + 5. * param.s1.powi(4) * param.s2
                                + -90. * param.s1.powi(3) * param.s2.powi(2)
                                + -510. * param.s1.powi(2) * param.s2.powi(3)
                                + 3190. * param.s1 * param.s2.powi(4)
                                + -1911. * param.s2.powi(5))
                        + -1680.
                            * param.m2_2.powi(6)
                            * (2. * param.s1.powi(2)
                                + 2. * param.s12.powi(2)
                                + 5. * param.s1 * param.s2
                                + 2. * param.s2.powi(2)
                                + -4. * param.s12 * (param.s1 + param.s2))
                        + 420.
                            * param.m2_2.powi(5)
                            * (-17. * param.s1.powi(3)
                                + 17. * param.s12.powi(3)
                                + -43. * param.s1.powi(2) * param.s2
                                + 29. * param.s1 * param.s2.powi(2)
                                + 31. * param.s2.powi(3)
                                + -3. * param.s12.powi(2) * (17. * param.s1 + param.s2)
                                + param.s12
                                    * (51. * param.s1.powi(2)
                                        + 46. * param.s1 * param.s2
                                        + -45. * param.s2.powi(2)))
                        + -70.
                            * param.m2_2.powi(4)
                            * (61. * param.s12.powi(4)
                                + param.s12.powi(3) * (-244. * param.s1 + 266. * param.s2)
                                + param.s12.powi(2)
                                    * (366. * param.s1.powi(2)
                                        + -284. * param.s1 * param.s2
                                        + -444. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (61. * param.s1.powi(2)
                                        + 370. * param.s1 * param.s2
                                        + 271. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (122. * param.s1.powi(3)
                                        + 115. * param.s1.powi(2) * param.s2
                                        + -530. * param.s1 * param.s2.powi(2)
                                        + 77. * param.s2.powi(3)))
                        + 140.
                            * param.m2_2.powi(3)
                            * (3. * param.s12.powi(5)
                                + param.s12.powi(4) * (-15. * param.s1 + 107. * param.s2)
                                + param.s12.powi(3)
                                    * (30. * param.s1.powi(2)
                                        + -260. * param.s1 * param.s2
                                        + 52. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-30. * param.s1.powi(3)
                                        + 138. * param.s1.powi(2) * param.s2
                                        + 368. * param.s1 * param.s2.powi(2)
                                        + -348. * param.s2.powi(3))
                                + param.s12
                                    * (15. * param.s1.powi(4)
                                        + 76. * param.s1.powi(3) * param.s2
                                        + -532. * param.s1.powi(2) * param.s2.powi(2)
                                        + 344. * param.s1 * param.s2.powi(3)
                                        + 97. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(3)
                                    * (3. * param.s1.powi(2)
                                        + 70. * param.s1 * param.s2
                                        + 89. * param.s2.powi(2)))
                        + 21.
                            * param.m2_2.powi(2)
                            * (3. * param.s12.powi(6)
                                + -6. * param.s12.powi(5) * (3. * param.s1 + 13. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (9. * param.s1.powi(2)
                                        + 54. * param.s1 * param.s2
                                        + -175. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(4)
                                    * (3. * param.s1.powi(2)
                                        + -30. * param.s1 * param.s2
                                        + -157. * param.s2.powi(2))
                                + -20.
                                    * param.s12.powi(3)
                                    * (3. * param.s1.powi(3)
                                        + 15. * param.s1.powi(2) * param.s2
                                        + -31. * param.s1 * param.s2.powi(2)
                                        + -41. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (9. * param.s1.powi(3)
                                        + -27. * param.s1.powi(2) * param.s2
                                        + 483. * param.s1 * param.s2.powi(2)
                                        + 419. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + 60. * param.s1.powi(3) * param.s2
                                        + 1366. * param.s1.powi(2) * param.s2.powi(2)
                                        + -3380. * param.s1 * param.s2.powi(3)
                                        + 1125. * param.s2.powi(4)))
                        + 14.
                            * param.m2_2
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
                                        + -401. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(4)
                                    * (7. * param.s1.powi(3)
                                        + 22. * param.s1.powi(2) * param.s2
                                        + 67. * param.s1 * param.s2.powi(2)
                                        + -120. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (7. * param.s1.powi(4)
                                        + 12. * param.s1.powi(3) * param.s2
                                        + 12. * param.s1.powi(2) * param.s2.powi(2)
                                        + 188. * param.s1 * param.s2.powi(3)
                                        + -243. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-21. * param.s1.powi(5)
                                        + 20. * param.s1.powi(4) * param.s2
                                        + 180. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1458. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1225. * param.s1 * param.s2.powi(4)
                                        + 54. * param.s2.powi(5))
                                - (param.s1 - param.s2).powi(5)
                                    * (param.s1.powi(2)
                                        + -5. * param.s1 * param.s2
                                        + 10. * param.s2.powi(2))
                                - param.s12.powi(6) * (7. * param.s1 + 16. * param.s2)))
                + -2.
                    * param.m1_2
                    * param.s1.powi(5)
                    * (9. * param.s12.powi(8)
                        + param.s12.powi(6)
                            * (252. * param.s1.powi(2)
                                + 551. * param.s1 * param.s2
                                + 623. * param.s2.powi(2))
                        + (param.s1 - param.s2).powi(5)
                            * (9. * param.s1.powi(3)
                                + -46. * param.s1.powi(2) * param.s2
                                + 95. * param.s1 * param.s2.powi(2)
                                + -100. * param.s2.powi(3))
                        + 5. * param.s12.powi(4)
                            * (126. * param.s1.powi(4)
                                + 155. * param.s1.powi(3) * param.s2
                                + 149. * param.s1.powi(2) * param.s2.powi(2)
                                + 113. * param.s1 * param.s2.powi(3)
                                + 525. * param.s2.powi(4))
                        + param.s12.powi(3)
                            * (-504. * param.s1.powi(5)
                                + 215. * param.s1.powi(4) * param.s2
                                + 960. * param.s1.powi(3) * param.s2.powi(2)
                                + 3480. * param.s1.powi(2) * param.s2.powi(3)
                                + -12560. * param.s1 * param.s2.powi(4)
                                + 2961. * param.s2.powi(5))
                        + param.s12.powi(2)
                            * (252. * param.s1.powi(6)
                                + -723. * param.s1.powi(5) * param.s2
                                + -295. * param.s1.powi(4) * param.s2.powi(2)
                                + 720. * param.s1.powi(3) * param.s2.powi(3)
                                + -12888. * param.s1.powi(2) * param.s2.powi(4)
                                + 18667. * param.s1 * param.s2.powi(5)
                                + -5733. * param.s2.powi(6))
                        + 210.
                            * param.m2_2.powi(5)
                            * (-17. * param.s1.powi(3)
                                + 17. * param.s12.powi(3)
                                + -91. * param.s1.powi(2) * param.s2
                                + -91. * param.s1 * param.s2.powi(2)
                                + -17. * param.s2.powi(3)
                                + -51. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (51. * param.s1.powi(2)
                                        + 142. * param.s1 * param.s2
                                        + 51. * param.s2.powi(2)))
                        + -70.
                            * param.m2_2.powi(4)
                            * (61. * param.s1.powi(4)
                                + 61. * param.s12.powi(4)
                                + 503. * param.s1.powi(3) * param.s2
                                + 237. * param.s1.powi(2) * param.s2.powi(2)
                                + -607. * param.s1 * param.s2.powi(3)
                                + -194. * param.s2.powi(4)
                                + param.s12.powi(3) * (-244. * param.s1 + 11. * param.s2)
                                + param.s12.powi(2)
                                    * (366. * param.s1.powi(2)
                                        + 481. * param.s1 * param.s2
                                        + -399. * param.s2.powi(2))
                                + param.s12
                                    * (-244. * param.s1.powi(3)
                                        + -995. * param.s1.powi(2) * param.s2
                                        + 370. * param.s1 * param.s2.powi(2)
                                        + 521. * param.s2.powi(3)))
                        + 70.
                            * param.m2_2.powi(3)
                            * (9. * param.s12.powi(5)
                                + param.s12.powi(4) * (-45. * param.s1 + 199. * param.s2)
                                + param.s12.powi(3)
                                    * (90. * param.s1.powi(2)
                                        + -292. * param.s1 * param.s2
                                        + -376. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (45. * param.s1.powi(3)
                                        + 159. * param.s1.powi(2) * param.s2
                                        + -836. * param.s1 * param.s2.powi(2)
                                        + 78. * param.s2.powi(3))
                                + param.s12
                                    * (45. * param.s1.powi(4)
                                        + 716. * param.s1.powi(3) * param.s2
                                        + -1136. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1088. * param.s1 * param.s2.powi(3)
                                        + 599. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(2)
                                    * (9. * param.s1.powi(3)
                                        + 323. * param.s1.powi(2) * param.s2
                                        + 797. * param.s1 * param.s2.powi(2)
                                        + 275. * param.s2.powi(3)))
                        + 42.
                            * param.m2_2.powi(2)
                            * (3. * param.s12.powi(6)
                                + -9. * param.s12.powi(5) * (2. * param.s1 + 7. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (9. * param.s1.powi(2)
                                        + 39. * param.s1 * param.s2
                                        + -68. * param.s2.powi(2))
                                + -10.
                                    * param.s12.powi(3)
                                    * (6. * param.s1.powi(3)
                                        + 15. * param.s1.powi(2) * param.s2
                                        + 68. * param.s1 * param.s2.powi(2)
                                        + -108. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(3)
                                        + -16. * param.s1.powi(2) * param.s2
                                        + -159. * param.s1 * param.s2.powi(2)
                                        + -96. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + -90. * param.s1.powi(3) * param.s2
                                        + 2056. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1540. * param.s1 * param.s2.powi(3)
                                        + -615. * param.s2.powi(4))
                                - param.s12
                                    * (18. * param.s1.powi(5)
                                        + -165. * param.s1.powi(4) * param.s2
                                        + 712. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1512. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2430. * param.s1 * param.s2.powi(4)
                                        + 353. * param.s2.powi(5)))
                        + 7. * param.m2_2
                            * (5. * param.s12.powi(7)
                                + param.s12.powi(5)
                                    * (105. * param.s1.powi(2)
                                        + 296. * param.s1 * param.s2
                                        + 591. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (35. * param.s1.powi(3)
                                        + 83. * param.s1.powi(2) * param.s2
                                        + 173. * param.s1 * param.s2.powi(2)
                                        + -75. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + 24. * param.s1.powi(3) * param.s2
                                        + -120. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1312. * param.s1 * param.s2.powi(3)
                                        + -723. * param.s2.powi(4))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (35. * param.s1.powi(4)
                                        + -154. * param.s1.powi(3) * param.s2
                                        + -198. * param.s1.powi(2) * param.s2.powi(2)
                                        + -4478. * param.s1 * param.s2.powi(3)
                                        + -509. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-105. * param.s1.powi(5)
                                        + 235. * param.s1.powi(4) * param.s2
                                        + 1080. * param.s1.powi(3) * param.s2.powi(2)
                                        + -3192. * param.s1.powi(2) * param.s2.powi(3)
                                        + -4015. * param.s1 * param.s2.powi(4)
                                        + 3645. * param.s2.powi(5))
                                - (param.s1 - param.s2).powi(4)
                                    * (5. * param.s1.powi(3)
                                        + -39. * param.s1.powi(2) * param.s2
                                        + 165. * param.s1 * param.s2.powi(2)
                                        + 421. * param.s2.powi(3))
                                - param.s12.powi(6) * (35. * param.s1 + 71. * param.s2))
                        - param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (72. * param.s1.powi(4)
                                + -223. * param.s1.powi(3) * param.s2
                                + -77. * param.s1.powi(2) * param.s2.powi(2)
                                + 1525. * param.s1 * param.s2.powi(3)
                                + 2147. * param.s2.powi(4))
                        - param.s12.powi(5)
                            * (504. * param.s1.powi(3)
                                + 1059. * param.s1.powi(2) * param.s2
                                + 1640. * param.s1 * param.s2.powi(2)
                                + 2625. * param.s2.powi(3))
                        - param.s12.powi(7) * (72. * param.s1 + 107. * param.s2))
                + param.m1_2.powi(2)
                    * param.s1.powi(4)
                    * (45. * param.s12.powi(8)
                        + -20. * param.s12.powi(7) * (18. * param.s1 + 25. * param.s2)
                        + 2. * param.s12.powi(6)
                            * (630. * param.s1.powi(2)
                                + 1255. * param.s1 * param.s2
                                + 1309. * param.s2.powi(2))
                        + -4.
                            * param.s12.powi(5)
                            * (630. * param.s1.powi(3)
                                + 1140. * param.s1.powi(2) * param.s2
                                + 1532. * param.s1 * param.s2.powi(2)
                                + 2247. * param.s2.powi(3))
                        + (param.s1 - param.s2).powi(4)
                            * (45. * param.s1.powi(4)
                                + -310. * param.s1.powi(3) * param.s2
                                + 978. * param.s1.powi(2) * param.s2.powi(2)
                                + -2130. * param.s1 * param.s2.powi(3)
                                + -2447. * param.s2.powi(4))
                        + 10.
                            * param.s12.powi(4)
                            * (315. * param.s1.powi(4)
                                + 265. * param.s1.powi(3) * param.s2
                                + 82. * param.s1.powi(2) * param.s2.powi(2)
                                + -323. * param.s1 * param.s2.powi(3)
                                + 1575. * param.s2.powi(4))
                        + -4.
                            * param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (90. * param.s1.powi(5)
                                + -430. * param.s1.powi(4) * param.s2
                                + 452. * param.s1.powi(3) * param.s2.powi(2)
                                + 2349. * param.s1.powi(2) * param.s2.powi(3)
                                + 8614. * param.s1 * param.s2.powi(4)
                                + -1793. * param.s2.powi(5))
                        + -20.
                            * param.s12.powi(3)
                            * (126. * param.s1.powi(5)
                                + -115. * param.s1.powi(4) * param.s2
                                + -282. * param.s1.powi(3) * param.s2.powi(2)
                                + -660. * param.s1.powi(2) * param.s2.powi(3)
                                + 844. * param.s1 * param.s2.powi(4)
                                + 525. * param.s2.powi(5))
                        + 2. * param.s12.powi(2)
                            * (630. * param.s1.powi(6)
                                + -2175. * param.s1.powi(5) * param.s2
                                + 85. * param.s1.powi(4) * param.s2.powi(2)
                                + 5580. * param.s1.powi(3) * param.s2.powi(3)
                                + -43392. * param.s1.powi(2) * param.s2.powi(4)
                                + 32615. * param.s1 * param.s2.powi(5)
                                + -1575. * param.s2.powi(6))
                        + -70.
                            * param.m2_2.powi(4)
                            * (61. * param.s1.powi(4)
                                + 61. * param.s12.powi(4)
                                + 758. * param.s1.powi(3) * param.s2
                                + 1602. * param.s1.powi(2) * param.s2.powi(2)
                                + 758. * param.s1 * param.s2.powi(3)
                                + 61. * param.s2.powi(4)
                                + -244. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(2)
                                    * (183. * param.s1.powi(2)
                                        + 623. * param.s1 * param.s2
                                        + 183. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (61. * param.s1.powi(3)
                                        + 440. * param.s1.powi(2) * param.s2
                                        + 440. * param.s1 * param.s2.powi(2)
                                        + 61. * param.s2.powi(3)))
                        + 140.
                            * param.m2_2.powi(3)
                            * (-9. * param.s1.powi(5)
                                + 9. * param.s12.powi(5)
                                + -427. * param.s1.powi(4) * param.s2
                                + -1166. * param.s1.powi(3) * param.s2.powi(2)
                                + 522. * param.s1.powi(2) * param.s2.powi(3)
                                + 967. * param.s1 * param.s2.powi(4)
                                + 113. * param.s2.powi(5)
                                + param.s12.powi(4) * (-45. * param.s1 + 77. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (45. * param.s1.powi(2)
                                        + 98. * param.s1 * param.s2
                                        + -199. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-90. * param.s1.powi(3)
                                        + -1050. * param.s1.powi(2) * param.s2
                                        + 710. * param.s1 * param.s2.powi(2)
                                        + 642. * param.s2.powi(3))
                                + param.s12
                                    * (45. * param.s1.powi(4)
                                        + 1204. * param.s1.powi(3) * param.s2
                                        + 854. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1828. * param.s1 * param.s2.powi(3)
                                        + -443. * param.s2.powi(4)))
                        + 42.
                            * param.m2_2.powi(2)
                            * (9. * param.s12.powi(6)
                                + -18. * param.s12.powi(5) * (3. * param.s1 + 8. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (27. * param.s1.powi(2)
                                        + 72. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + -20.
                                    * param.s12.powi(3)
                                    * (9. * param.s1.powi(3)
                                        + 175. * param.s1 * param.s2.powi(2)
                                        + -68. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (135. * param.s1.powi(4)
                                        + -720. * param.s1.powi(3) * param.s2
                                        + 4578. * param.s1.powi(2) * param.s2.powi(2)
                                        + 3740. * param.s1 * param.s2.powi(3)
                                        + -2625. * param.s2.powi(4))
                                + (param.s1 - param.s2).powi(2)
                                    * (9. * param.s1.powi(4)
                                        + -198. * param.s1.powi(3) * param.s2
                                        + -2902. * param.s1.powi(2) * param.s2.powi(2)
                                        + -3418. * param.s1 * param.s2.powi(3)
                                        + -511. * param.s2.powi(4))
                                + -2.
                                    * param.s12
                                    * (27. * param.s1.powi(5)
                                        + -360. * param.s1.powi(4) * param.s2
                                        + -722. * param.s1.powi(3) * param.s2.powi(2)
                                        + 5108. * param.s1.powi(2) * param.s2.powi(3)
                                        + -925. * param.s1 * param.s2.powi(4)
                                        + -968. * param.s2.powi(5)))
                        + 28.
                            * param.m2_2
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
                                        + 129. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + -12. * param.s1.powi(3) * param.s2
                                        + -210. * param.s1.powi(2) * param.s2.powi(2)
                                        + 904. * param.s1 * param.s2.powi(3)
                                        + -75. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-105. * param.s1.powi(5)
                                        + 370. * param.s1.powi(4) * param.s2
                                        + 810. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2976. * param.s1.powi(2) * param.s2.powi(3)
                                        + -8635. * param.s1 * param.s2.powi(4)
                                        + 1800. * param.s2.powi(5))
                                + param.s12
                                    * (35. * param.s1.powi(6)
                                        + -278. * param.s1.powi(5) * param.s2
                                        + 640. * param.s1.powi(4) * param.s2.powi(2)
                                        + -6372. * param.s1.powi(3) * param.s2.powi(3)
                                        + 3713. * param.s1.powi(2) * param.s2.powi(4)
                                        + 3830. * param.s1 * param.s2.powi(5)
                                        + -1568. * param.s2.powi(6))
                                - (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1.powi(4)
                                        + -53. * param.s1.powi(3) * param.s2
                                        + 348. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1687. * param.s1 * param.s2.powi(3)
                                        + 443. * param.s2.powi(4))
                                - param.s12.powi(6) * (35. * param.s1 + 62. * param.s2)))
                + -2.
                    * param.m1_2.powi(5)
                    * param.s1
                    * (9. * param.s1.powi(8)
                        + 9. * param.s12.powi(8)
                        + -119. * param.s1.powi(7) * param.s2
                        + 821. * param.s1.powi(6) * param.s2.powi(2)
                        + -4779. * param.s1.powi(5) * param.s2.powi(3)
                        + -12843. * param.s1.powi(4) * param.s2.powi(4)
                        + 14373. * param.s1.powi(3) * param.s2.powi(5)
                        + 2781. * param.s1.powi(2) * param.s2.powi(6)
                        + -259. * param.s1 * param.s2.powi(7)
                        + 16. * param.s2.powi(8)
                        + param.s12.powi(6)
                            * (252. * param.s1.powi(2)
                                + 355. * param.s1 * param.s2
                                + 301. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (126. * param.s1.powi(4)
                                + -41. * param.s1.powi(3) * param.s2
                                + -89. * param.s1.powi(2) * param.s2.powi(2)
                                + -125. * param.s1 * param.s2.powi(3)
                                + 175. * param.s2.powi(4))
                        + param.s12.powi(3)
                            * (-504. * param.s1.powi(5)
                                + 1195. * param.s1.powi(4) * param.s2
                                + 120. * param.s1.powi(3) * param.s2.powi(2)
                                + -1140. * param.s1.powi(2) * param.s2.powi(3)
                                + 2280. * param.s1 * param.s2.powi(4)
                                + -749. * param.s2.powi(5))
                        + param.s12.powi(2)
                            * (252. * param.s1.powi(6)
                                + -1311. * param.s1.powi(5) * param.s2
                                + 2155. * param.s1.powi(4) * param.s2.powi(2)
                                + 1980. * param.s1.powi(3) * param.s2.powi(3)
                                + 7020. * param.s1.powi(2) * param.s2.powi(4)
                                + -2543. * param.s1 * param.s2.powi(5)
                                + 399. * param.s2.powi(6))
                        + 7. * param.m2_2
                            * (param.s12.powi(7)
                                + 19. * param.s1.powi(6) * param.s2
                                + -261. * param.s1.powi(5) * param.s2.powi(2)
                                + -2997. * param.s1.powi(4) * param.s2.powi(3)
                                + -2997. * param.s1.powi(3) * param.s2.powi(4)
                                + -261. * param.s1.powi(2) * param.s2.powi(5)
                                + 19. * param.s1 * param.s2.powi(6)
                                + -7. * param.s12.powi(6) * (param.s1 + param.s2)
                                + param.s12.powi(5)
                                    * (21. * param.s1.powi(2)
                                        + 16. * param.s1 * param.s2
                                        + 21. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (7. * param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + 7. * param.s2.powi(3))
                                + 5. * param.s12.powi(3)
                                    * (7. * param.s1.powi(4)
                                        + -24. * param.s1.powi(3) * param.s2
                                        + 12. * param.s1.powi(2) * param.s2.powi(2)
                                        + -24. * param.s1 * param.s2.powi(3)
                                        + 7. * param.s2.powi(4))
                                + param.s12
                                    * (7. * param.s1.powi(6)
                                        + -88. * param.s1.powi(5) * param.s2
                                        + 695. * param.s1.powi(4) * param.s2.powi(2)
                                        + 3692. * param.s1.powi(3) * param.s2.powi(3)
                                        + 695. * param.s1.powi(2) * param.s2.powi(4)
                                        + -88. * param.s1 * param.s2.powi(5)
                                        + 7. * param.s2.powi(6))
                                - param.s12.powi(2)
                                    * (21. * param.s1.powi(5)
                                        + -155. * param.s1.powi(4) * param.s2
                                        + 540. * param.s1.powi(3) * param.s2.powi(2)
                                        + 540. * param.s1.powi(2) * param.s2.powi(3)
                                        + -155. * param.s1 * param.s2.powi(4)
                                        + 21. * param.s2.powi(5))
                                - param.s2.powi(7)
                                - param.s1.powi(7))
                        - param.s12
                            * (72. * param.s1.powi(7)
                                + -635. * param.s1.powi(6) * param.s2
                                + 2516. * param.s1.powi(5) * param.s2.powi(2)
                                + -5215. * param.s1.powi(4) * param.s2.powi(3)
                                + 15764. * param.s1.powi(3) * param.s2.powi(4)
                                + 7997. * param.s1.powi(2) * param.s2.powi(5)
                                + -1300. * param.s1 * param.s2.powi(6)
                                + 121. * param.s2.powi(7))
                        - param.s12.powi(5)
                            * (504. * param.s1.powi(3)
                                + 471. * param.s1.powi(2) * param.s2
                                + 436. * param.s1 * param.s2.powi(2)
                                + 651. * param.s2.powi(3))
                        - param.s12.powi(7) * (72. * param.s1 + 79. * param.s2))
                + param.m1_2.powi(4)
                    * param.s1.powi(2)
                    * (45. * param.s12.powi(8)
                        + -10. * param.s12.powi(7) * (36. * param.s1 + 43. * param.s2)
                        + param.s12.powi(6)
                            * (1260. * param.s1.powi(2)
                                + 2020. * param.s1 * param.s2
                                + 1813. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(5)
                            * (1260. * param.s1.powi(3)
                                + 1545. * param.s1.powi(2) * param.s2
                                + 1559. * param.s1 * param.s2.powi(2)
                                + 2184. * param.s2.powi(3))
                        + 5. * param.s12.powi(4)
                            * (630. * param.s1.powi(4)
                                + 40. * param.s1.powi(3) * param.s2
                                + -431. * param.s1.powi(2) * param.s2.powi(2)
                                + -926. * param.s1 * param.s2.powi(3)
                                + 1309. * param.s2.powi(4))
                        + -10.
                            * param.s12.powi(3)
                            * (252. * param.s1.powi(5)
                                + -475. * param.s1.powi(4) * param.s2
                                + -354. * param.s1.powi(3) * param.s2.powi(2)
                                + 150. * param.s1.powi(2) * param.s2.powi(3)
                                + -2190. * param.s1 * param.s2.powi(4)
                                + 623. * param.s2.powi(5))
                        + (param.s1 - param.s2).powi(2)
                            * (45. * param.s1.powi(6)
                                + -470. * param.s1.powi(5) * param.s2
                                + 2518. * param.s1.powi(4) * param.s2.powi(2)
                                + -11396. * param.s1.powi(3) * param.s2.powi(3)
                                + -36269. * param.s1.powi(2) * param.s2.powi(4)
                                + -3746. * param.s1 * param.s2.powi(5)
                                + 178. * param.s2.powi(6))
                        + param.s12.powi(2)
                            * (1260. * param.s1.powi(6)
                                + -5820. * param.s1.powi(5) * param.s2
                                + 6295. * param.s1.powi(4) * param.s2.powi(2)
                                + 17460. * param.s1.powi(3) * param.s2.powi(3)
                                + -13074. * param.s1.powi(2) * param.s2.powi(4)
                                + -29480. * param.s1 * param.s2.powi(5)
                                + 3675. * param.s2.powi(6))
                        + -2.
                            * param.s12
                            * (180. * param.s1.powi(7)
                                + -1465. * param.s1.powi(6) * param.s2
                                + 4939. * param.s1.powi(5) * param.s2.powi(2)
                                + -4970. * param.s1.powi(4) * param.s2.powi(3)
                                + 48286. * param.s1.powi(3) * param.s2.powi(4)
                                + -23579. * param.s1.powi(2) * param.s2.powi(5)
                                + -8885. * param.s1 * param.s2.powi(6)
                                + 614. * param.s2.powi(7))
                        + 21.
                            * param.m2_2.powi(2)
                            * (3. * param.s1.powi(6)
                                + 3. * param.s12.powi(6)
                                + -102. * param.s1.powi(5) * param.s2
                                + -2459. * param.s1.powi(4) * param.s2.powi(2)
                                + -5684. * param.s1.powi(3) * param.s2.powi(3)
                                + -2459. * param.s1.powi(2) * param.s2.powi(4)
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
                                        + -3194. * param.s1.powi(2) * param.s2.powi(2)
                                        + -540. * param.s1 * param.s2.powi(3)
                                        + 45. * param.s2.powi(4))
                                + param.s12
                                    * (-18. * param.s1.powi(5)
                                        + 390. * param.s1.powi(4) * param.s2
                                        + 5308. * param.s1.powi(3) * param.s2.powi(2)
                                        + 5308. * param.s1.powi(2) * param.s2.powi(3)
                                        + 390. * param.s1 * param.s2.powi(4)
                                        + -18. * param.s2.powi(5)))
                        + 14.
                            * param.m2_2
                            * (-5. * param.s1.powi(7)
                                + 5. * param.s12.powi(7)
                                + 86. * param.s1.powi(6) * param.s2
                                + -999. * param.s1.powi(5) * param.s2.powi(2)
                                + -7608. * param.s1.powi(4) * param.s2.powi(3)
                                + 2067. * param.s1.powi(3) * param.s2.powi(4)
                                + 6072. * param.s1.powi(2) * param.s2.powi(5)
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
                                        + 6882. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2395. * param.s1 * param.s2.powi(4)
                                        + -240. * param.s2.powi(5))
                                + param.s12
                                    * (35. * param.s1.powi(6)
                                        + -386. * param.s1.powi(5) * param.s2
                                        + 2305. * param.s1.powi(4) * param.s2.powi(2)
                                        + 2536. * param.s1.powi(3) * param.s2.powi(3)
                                        + -12449. * param.s1.powi(2) * param.s2.powi(4)
                                        + -1610. * param.s1 * param.s2.powi(5)
                                        + 89. * param.s2.powi(6))
                                - param.s12.powi(6) * (35. * param.s1 + 44. * param.s2)))
                + -4.
                    * param.m1_2.powi(3)
                    * param.s1.powi(3)
                    * (15. * param.s12.powi(8)
                        + -5. * param.s12.powi(7) * (24. * param.s1 + 31. * param.s2)
                        + param.s12.powi(6)
                            * (420. * param.s1.powi(2)
                                + 755. * param.s1 * param.s2
                                + 728. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (210. * param.s1.powi(4)
                                + 95. * param.s1.powi(3) * param.s2
                                + -76. * param.s1.powi(2) * param.s2.powi(2)
                                + -346. * param.s1 * param.s2.powi(3)
                                + 749. * param.s2.powi(4))
                        + (param.s1 - param.s2).powi(3)
                            * (15. * param.s1.powi(5)
                                + -130. * param.s1.powi(4) * param.s2
                                + 553. * param.s1.powi(3) * param.s2.powi(2)
                                + -1848. * param.s1.powi(2) * param.s2.powi(3)
                                + -4042. * param.s1 * param.s2.powi(4)
                                + -218. * param.s2.powi(5))
                        + -5.
                            * param.s12.powi(3)
                            * (168. * param.s1.powi(5)
                                + -235. * param.s1.powi(4) * param.s2
                                + -348. * param.s1.powi(3) * param.s2.powi(2)
                                + -390. * param.s1.powi(2) * param.s2.powi(3)
                                + -984. * param.s1 * param.s2.powi(4)
                                + 875. * param.s2.powi(5))
                        + param.s12.powi(2)
                            * (420. * param.s1.powi(6)
                                + -1695. * param.s1.powi(5) * param.s2
                                + 920. * param.s1.powi(4) * param.s2.powi(2)
                                + 5610. * param.s1.powi(3) * param.s2.powi(3)
                                + -21984. * param.s1.powi(2) * param.s2.powi(4)
                                + 1595. * param.s1 * param.s2.powi(5)
                                + 3150. * param.s2.powi(6))
                        + 105.
                            * param.m2_2.powi(3)
                            * (param.s12.powi(5)
                                + -61. * param.s1.powi(4) * param.s2
                                + -298. * param.s1.powi(3) * param.s2.powi(2)
                                + -298. * param.s1.powi(2) * param.s2.powi(3)
                                + -61. * param.s1 * param.s2.powi(4)
                                + -5. * param.s12.powi(4) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + 99. * param.s1.powi(2) * param.s2
                                        + 99. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + param.s12
                                    * (5. * param.s1.powi(4)
                                        + 188. * param.s1.powi(3) * param.s2
                                        + 486. * param.s1.powi(2) * param.s2.powi(2)
                                        + 188. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                - param.s2.powi(5)
                                - param.s1.powi(5))
                        + 21.
                            * param.m2_2.powi(2)
                            * (3. * param.s1.powi(6)
                                + 3. * param.s12.powi(6)
                                + -87. * param.s1.powi(5) * param.s2
                                + -1544. * param.s1.powi(4) * param.s2.powi(2)
                                + -1214. * param.s1.powi(3) * param.s2.powi(3)
                                + 2011. * param.s1.powi(2) * param.s2.powi(4)
                                + 813. * param.s1 * param.s2.powi(5)
                                + 18. * param.s2.powi(6)
                                + -3. * param.s12.powi(5) * (6. * param.s1 + 11. * param.s2)
                                + 15.
                                    * param.s12.powi(4)
                                    * (3. * param.s1.powi(2)
                                        + 3. * param.s1 * param.s2
                                        + 8. * param.s2.powi(2))
                                + -30.
                                    * param.s12.powi(3)
                                    * (2. * param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 28. * param.s1 * param.s2.powi(2)
                                        + 7. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + -390. * param.s1.powi(3) * param.s2
                                        + -224. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2430. * param.s1 * param.s2.powi(3)
                                        + 195. * param.s2.powi(4))
                                - param.s12
                                    * (18. * param.s1.powi(5)
                                        + -315. * param.s1.powi(4) * param.s2
                                        + -2488. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1982. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2430. * param.s1 * param.s2.powi(4)
                                        + 93. * param.s2.powi(5)))
                        + 7. * param.m2_2
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
                                        + 204. * param.s1 * param.s2.powi(3)
                                        + 197. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-105. * param.s1.powi(5)
                                        + 505. * param.s1.powi(4) * param.s2
                                        + 90. * param.s1.powi(3) * param.s2.powi(2)
                                        + 7554. * param.s1.powi(2) * param.s2.powi(3)
                                        + -4895. * param.s1 * param.s2.powi(4)
                                        + -825. * param.s2.powi(5))
                                + param.s12
                                    * (35. * param.s1.powi(6)
                                        + -332. * param.s1.powi(5) * param.s2
                                        + 1360. * param.s1.powi(4) * param.s2.powi(2)
                                        + -4928. * param.s1.powi(3) * param.s2.powi(3)
                                        + -6503. * param.s1.powi(2) * param.s2.powi(4)
                                        + 5680. * param.s1 * param.s2.powi(5)
                                        + 368. * param.s2.powi(6))
                                - (param.s1 - param.s2).powi(2)
                                    * (5. * param.s1.powi(5)
                                        + -67. * param.s1.powi(4) * param.s2
                                        + 599. * param.s1.powi(3) * param.s2.powi(2)
                                        + 4241. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2174. * param.s1 * param.s2.powi(4)
                                        + 68. * param.s2.powi(5))
                                - param.s12.powi(6) * (35. * param.s1 + 53. * param.s2))
                        - param.s12
                            * (120. * param.s1.powi(7)
                                + -895. * param.s1.powi(6) * param.s2
                                + 2518. * param.s1.powi(5) * param.s2.powi(2)
                                + -140. * param.s1.powi(4) * param.s2.powi(3)
                                + 20692. * param.s1.powi(3) * param.s2.powi(4)
                                + -30893. * param.s1.powi(2) * param.s2.powi(5)
                                + 7330. * param.s1 * param.s2.powi(6)
                                + 1268. * param.s2.powi(7))
                        - param.s12.powi(5)
                            * (840. * param.s1.powi(3)
                                + 1275. * param.s1.powi(2) * param.s2
                                + 1478. * param.s1 * param.s2.powi(2)
                                + 2058. * param.s2.powi(3)))
                + -2.
                    * param.m0_2.powi(5)
                    * (param.m1_2
                        * (9. * param.s12.powi(8)
                            + param.s12.powi(6)
                                * (623. * param.s1.powi(2)
                                    + 551. * param.s1 * param.s2
                                    + 252. * param.s2.powi(2))
                            + (param.s1 - param.s2).powi(5)
                                * (100. * param.s1.powi(3)
                                    + -95. * param.s1.powi(2) * param.s2
                                    + 46. * param.s1 * param.s2.powi(2)
                                    + -9. * param.s2.powi(3))
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (2147. * param.s1.powi(4)
                                    + 1525. * param.s1.powi(3) * param.s2
                                    + -77. * param.s1.powi(2) * param.s2.powi(2)
                                    + -223. * param.s1 * param.s2.powi(3)
                                    + 72. * param.s2.powi(4))
                            + 5. * param.s12.powi(4)
                                * (525. * param.s1.powi(4)
                                    + 113. * param.s1.powi(3) * param.s2
                                    + 149. * param.s1.powi(2) * param.s2.powi(2)
                                    + 155. * param.s1 * param.s2.powi(3)
                                    + 126. * param.s2.powi(4))
                            + param.s12.powi(3)
                                * (2961. * param.s1.powi(5)
                                    + -12560. * param.s1.powi(4) * param.s2
                                    + 3480. * param.s1.powi(3) * param.s2.powi(2)
                                    + 960. * param.s1.powi(2) * param.s2.powi(3)
                                    + 215. * param.s1 * param.s2.powi(4)
                                    + -504. * param.s2.powi(5))
                            + param.s12.powi(2)
                                * (-5733. * param.s1.powi(6)
                                    + 18667. * param.s1.powi(5) * param.s2
                                    + -12888. * param.s1.powi(4) * param.s2.powi(2)
                                    + 720. * param.s1.powi(3) * param.s2.powi(3)
                                    + -295. * param.s1.powi(2) * param.s2.powi(4)
                                    + -723. * param.s1 * param.s2.powi(5)
                                    + 252. * param.s2.powi(6))
                            - param.s12.powi(5)
                                * (2625. * param.s1.powi(3)
                                    + 1640. * param.s1.powi(2) * param.s2
                                    + 1059. * param.s1 * param.s2.powi(2)
                                    + 504. * param.s2.powi(3))
                            - param.s12.powi(7) * (107. * param.s1 + 72. * param.s2))
                        + param.s1
                            * (9. * param.s12.powi(8)
                                + (param.s1 - param.s2).powi(6)
                                    * (16. * param.s1.powi(2)
                                        + -9. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (1085. * param.s1.powi(2)
                                        + 565. * param.s1 * param.s2
                                        + 203. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (84. * param.s1.powi(3)
                                        + -1955. * param.s1.powi(2) * param.s2
                                        + -884. * param.s1 * param.s2.powi(2)
                                        + -357. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(4)
                                    * (1071. * param.s1.powi(4)
                                        + -2045. * param.s1.powi(3) * param.s2
                                        + 68. * param.s1.powi(2) * param.s2.powi(2)
                                        + -85. * param.s1 * param.s2.powi(3)
                                        + -77. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (6426. * param.s1.powi(5)
                                        + -7205. * param.s1.powi(4) * param.s2
                                        + -5690. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1800. * param.s1.powi(2) * param.s2.powi(3)
                                        + 320. * param.s1 * param.s2.powi(4)
                                        + -259. * param.s2.powi(5))
                                + -7.
                                    * param.m2_2
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
                                            * (120. * param.s1.powi(3)
                                                + -67. * param.s1.powi(2) * param.s2
                                                + -22. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (401. * param.s1.powi(3)
                                                + 85. * param.s1.powi(2) * param.s2
                                                + 13. * param.s1 * param.s2.powi(2)
                                                + -7. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-1215. * param.s1.powi(4)
                                                + 940. * param.s1.powi(3) * param.s2
                                                + 60. * param.s1.powi(2) * param.s2.powi(2)
                                                + 60. * param.s1 * param.s2.powi(3)
                                                + 35. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (54. * param.s1.powi(5)
                                                + 1225. * param.s1.powi(4) * param.s2
                                                + -1458. * param.s1.powi(3) * param.s2.powi(2)
                                                + 180. * param.s1.powi(2) * param.s2.powi(3)
                                                + 20. * param.s1 * param.s2.powi(4)
                                                + -21. * param.s2.powi(5))
                                        - param.s12.powi(6) * (16. * param.s1 + 7. * param.s2))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (1827. * param.s1.powi(4)
                                        + 6239. * param.s1.powi(3) * param.s2
                                        + 684. * param.s1.powi(2) * param.s2.powi(2)
                                        + 247. * param.s1 * param.s2.powi(3)
                                        + -105. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (310. * param.s1.powi(3)
                                        + 115. * param.s1.powi(2) * param.s2
                                        + -88. * param.s1 * param.s2.powi(2)
                                        + 23. * param.s2.powi(3))
                                - param.s12.powi(7) * (128. * param.s1 + 65. * param.s2)))
                + param.m0_2.powi(4)
                    * (param.m1_2.powi(2)
                        * (45. * param.s12.powi(8)
                            + -20. * param.s12.powi(7) * (25. * param.s1 + 18. * param.s2)
                            + 2. * param.s12.powi(6)
                                * (1309. * param.s1.powi(2)
                                    + 1255. * param.s1 * param.s2
                                    + 630. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(5)
                                * (2247. * param.s1.powi(3)
                                    + 1532. * param.s1.powi(2) * param.s2
                                    + 1140. * param.s1 * param.s2.powi(2)
                                    + 630. * param.s2.powi(3))
                            + 10.
                                * param.s12.powi(4)
                                * (1575. * param.s1.powi(4)
                                    + -323. * param.s1.powi(3) * param.s2
                                    + 82. * param.s1.powi(2) * param.s2.powi(2)
                                    + 265. * param.s1 * param.s2.powi(3)
                                    + 315. * param.s2.powi(4))
                            + 4. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (1793. * param.s1.powi(5)
                                    + -8614. * param.s1.powi(4) * param.s2
                                    + -2349. * param.s1.powi(3) * param.s2.powi(2)
                                    + -452. * param.s1.powi(2) * param.s2.powi(3)
                                    + 430. * param.s1 * param.s2.powi(4)
                                    + -90. * param.s2.powi(5))
                            + -20.
                                * param.s12.powi(3)
                                * (525. * param.s1.powi(5)
                                    + 844. * param.s1.powi(4) * param.s2
                                    + -660. * param.s1.powi(3) * param.s2.powi(2)
                                    + -282. * param.s1.powi(2) * param.s2.powi(3)
                                    + -115. * param.s1 * param.s2.powi(4)
                                    + 126. * param.s2.powi(5))
                            + -2.
                                * param.s12.powi(2)
                                * (1575. * param.s1.powi(6)
                                    + -32615. * param.s1.powi(5) * param.s2
                                    + 43392. * param.s1.powi(4) * param.s2.powi(2)
                                    + -5580. * param.s1.powi(3) * param.s2.powi(3)
                                    + -85. * param.s1.powi(2) * param.s2.powi(4)
                                    + 2175. * param.s1 * param.s2.powi(5)
                                    + -630. * param.s2.powi(6))
                            - (param.s1 - param.s2).powi(4)
                                * (2447. * param.s1.powi(4)
                                    + 2130. * param.s1.powi(3) * param.s2
                                    + -978. * param.s1.powi(2) * param.s2.powi(2)
                                    + 310. * param.s1 * param.s2.powi(3)
                                    + -45. * param.s2.powi(4)))
                        + 2. * param.m1_2
                            * param.s1
                            * (27. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (2443. * param.s1.powi(2)
                                        + 1268. * param.s1 * param.s2
                                        + 511. * param.s2.powi(2))
                                + 2. * (param.s1 - param.s2).powi(5)
                                    * (73. * param.s1.powi(3)
                                        + 50. * param.s1.powi(2) * param.s2
                                        + -22. * param.s1 * param.s2.powi(2)
                                        + 4. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(4)
                                    * (609. * param.s1.powi(4)
                                        + -6016. * param.s1.powi(3) * param.s2
                                        + 1492. * param.s1.powi(2) * param.s2.powi(2)
                                        + 172. * param.s1 * param.s2.powi(3)
                                        + -133. * param.s2.powi(4))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (2437. * param.s1.powi(4)
                                        + 12618. * param.s1.powi(3) * param.s2
                                        + 2338. * param.s1.powi(2) * param.s2.powi(2)
                                        + -144. * param.s1 * param.s2.powi(3)
                                        + -29. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (12033. * param.s1.powi(5)
                                        + -56965. * param.s1.powi(4) * param.s2
                                        + 21710. * param.s1.powi(3) * param.s2.powi(2)
                                        + 4980. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2185. * param.s1 * param.s2.powi(4)
                                        + -287. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (-9891. * param.s1.powi(6)
                                        + 22226. * param.s1.powi(5) * param.s2
                                        + 28277. * param.s1.powi(4) * param.s2.powi(2)
                                        + -43662. * param.s1.powi(3) * param.s2.powi(3)
                                        + 4225. * param.s1.powi(2) * param.s2.powi(4)
                                        + -1196. * param.s1 * param.s2.powi(5)
                                        + 21. * param.s2.powi(6))
                                + -7.
                                    * param.m2_2
                                    * (5. * param.s12.powi(7)
                                        + param.s12.powi(5)
                                            * (591. * param.s1.powi(2)
                                                + 296. * param.s1 * param.s2
                                                + 105. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (75. * param.s1.powi(3)
                                                + -173. * param.s1.powi(2) * param.s2
                                                + -83. * param.s1 * param.s2.powi(2)
                                                + -35. * param.s2.powi(3))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (723. * param.s1.powi(4)
                                                + -1312. * param.s1.powi(3) * param.s2
                                                + 120. * param.s1.powi(2) * param.s2.powi(2)
                                                + -24. * param.s1 * param.s2.powi(3)
                                                + -35. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (3645. * param.s1.powi(5)
                                                + -4015. * param.s1.powi(4) * param.s2
                                                + -3192. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1080. * param.s1.powi(2) * param.s2.powi(3)
                                                + 235. * param.s1 * param.s2.powi(4)
                                                + -105. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (509. * param.s1.powi(4)
                                                + 4478. * param.s1.powi(3) * param.s2
                                                + 198. * param.s1.powi(2) * param.s2.powi(2)
                                                + 154. * param.s1 * param.s2.powi(3)
                                                + -35. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(4)
                                            * (421. * param.s1.powi(3)
                                                + 165. * param.s1.powi(2) * param.s2
                                                + -39. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        - param.s12.powi(6) * (71. * param.s1 + 35. * param.s2))
                                - param.s12.powi(5)
                                    * (3801. * param.s1.powi(3)
                                        + 1105. * param.s1.powi(2) * param.s2
                                        + 1189. * param.s1 * param.s2.powi(2)
                                        + 777. * param.s2.powi(3))
                                - param.s12.powi(7) * (349. * param.s1 + 181. * param.s2))
                        + param.s1.powi(2)
                            * (45. * param.s12.powi(8)
                                + -2. * param.s12.powi(7) * (439. * param.s1 + 124. * param.s2)
                                + (param.s1 - param.s2).powi(6)
                                    * (31. * param.s1.powi(2)
                                        + 18. * param.s1 * param.s2
                                        + -4. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-196. * param.s1.powi(2)
                                        + 2020. * param.s1 * param.s2
                                        + 539. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (8274. * param.s1.powi(3)
                                        + -22592. * param.s1.powi(2) * param.s2
                                        + 508. * param.s1 * param.s2.powi(2)
                                        + -546. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (257. * param.s1.powi(3)
                                        + 578. * param.s1.powi(2) * param.s2
                                        + 88. * param.s1 * param.s2.powi(2)
                                        + -23. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(4)
                                    * (3024. * param.s1.powi(4)
                                        + -4352. * param.s1.powi(3) * param.s2
                                        + -5407. * param.s1.powi(2) * param.s2.powi(2)
                                        + 926. * param.s1 * param.s2.powi(3)
                                        + -35. * param.s2.powi(4))
                                + 10.
                                    * param.s12.powi(3)
                                    * (987. * param.s1.powi(5)
                                        + 1798. * param.s1.powi(4) * param.s2
                                        + -6884. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1404. * param.s1.powi(2) * param.s2.powi(3)
                                        + 377. * param.s1 * param.s2.powi(4)
                                        + 14. * param.s2.powi(5))
                                + 21.
                                    * param.m2_2.powi(2)
                                    * (3. * param.s12.powi(6)
                                        + -6.
                                            * param.s12.powi(5)
                                            * (13. * param.s1 + 3. * param.s2)
                                        + param.s12.powi(4)
                                            * (-875. * param.s1.powi(2)
                                                + 270. * param.s1 * param.s2
                                                + 45. * param.s2.powi(2))
                                        + 20.
                                            * param.s12.powi(3)
                                            * (41. * param.s1.powi(3)
                                                + 31. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + -3. * param.s2.powi(3))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (419. * param.s1.powi(3)
                                                + 483. * param.s1.powi(2) * param.s2
                                                + -27. * param.s1 * param.s2.powi(2)
                                                + 9. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (1125. * param.s1.powi(4)
                                                + -3380. * param.s1.powi(3) * param.s2
                                                + 1366. * param.s1.powi(2) * param.s2.powi(2)
                                                + 60. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(4)
                                            * (157. * param.s1.powi(2)
                                                + 30. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2)))
                                + -14.
                                    * param.m2_2
                                    * (8. * param.s12.powi(7)
                                        + (param.s1 - param.s2).powi(5)
                                            * (26. * param.s1.powi(2)
                                                + 5. * param.s1 * param.s2
                                                - param.s2.powi(2))
                                        + param.s12.powi(5)
                                            * (-966. * param.s1.powi(2)
                                                + 560. * param.s1 * param.s2
                                                + 114. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (600. * param.s1.powi(3)
                                                + -413. * param.s1.powi(2) * param.s2
                                                + -86. * param.s1 * param.s2.powi(2)
                                                + -29. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (416. * param.s1.powi(3)
                                                + 754. * param.s1.powi(2) * param.s2
                                                + 61. * param.s1 * param.s2.powi(2)
                                                - param.s2.powi(3))
                                        + -10.
                                            * param.s12.powi(3)
                                            * (153. * param.s1.powi(4)
                                                + 460. * param.s1.powi(3) * param.s2
                                                + -591. * param.s1.powi(2) * param.s2.powi(2)
                                                + 24. * param.s1 * param.s2.powi(3)
                                                + -10. * param.s2.powi(4))
                                        - param.s12.powi(2)
                                            * (1188. * param.s1.powi(5)
                                                + -7265. * param.s1.powi(4) * param.s2
                                                + 4722. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1752. * param.s1.powi(2) * param.s2.powi(3)
                                                + -430. * param.s1 * param.s2.powi(4)
                                                + 33. * param.s2.powi(5))
                                        - param.s12.powi(6)
                                            * (182. * param.s1 + 47. * param.s2))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (1512. * param.s1.powi(4)
                                        + 22676. * param.s1.powi(3) * param.s2
                                        + 19359. * param.s1.powi(2) * param.s2.powi(2)
                                        + 766. * param.s1 * param.s2.powi(3)
                                        + 147. * param.s2.powi(4))))
                + -2.
                    * param.m0_2.powi(3)
                    * (2.
                        * param.m1_2.powi(3)
                        * (15. * param.s12.powi(8)
                            + -5. * param.s12.powi(7) * (31. * param.s1 + 24. * param.s2)
                            + param.s12.powi(6)
                                * (728. * param.s1.powi(2)
                                    + 755. * param.s1 * param.s2
                                    + 420. * param.s2.powi(2))
                            + 5. * param.s12.powi(4)
                                * (749. * param.s1.powi(4)
                                    + -346. * param.s1.powi(3) * param.s2
                                    + -76. * param.s1.powi(2) * param.s2.powi(2)
                                    + 95. * param.s1 * param.s2.powi(3)
                                    + 210. * param.s2.powi(4))
                            + (param.s1 - param.s2).powi(3)
                                * (218. * param.s1.powi(5)
                                    + 4042. * param.s1.powi(4) * param.s2
                                    + 1848. * param.s1.powi(3) * param.s2.powi(2)
                                    + -553. * param.s1.powi(2) * param.s2.powi(3)
                                    + 130. * param.s1 * param.s2.powi(4)
                                    + -15. * param.s2.powi(5))
                            + -5.
                                * param.s12.powi(3)
                                * (875. * param.s1.powi(5)
                                    + -984. * param.s1.powi(4) * param.s2
                                    + -390. * param.s1.powi(3) * param.s2.powi(2)
                                    + -348. * param.s1.powi(2) * param.s2.powi(3)
                                    + -235. * param.s1 * param.s2.powi(4)
                                    + 168. * param.s2.powi(5))
                            + param.s12.powi(2)
                                * (3150. * param.s1.powi(6)
                                    + 1595. * param.s1.powi(5) * param.s2
                                    + -21984. * param.s1.powi(4) * param.s2.powi(2)
                                    + 5610. * param.s1.powi(3) * param.s2.powi(3)
                                    + 920. * param.s1.powi(2) * param.s2.powi(4)
                                    + -1695. * param.s1 * param.s2.powi(5)
                                    + 420. * param.s2.powi(6))
                            - param.s12
                                * (1268. * param.s1.powi(7)
                                    + 7330. * param.s1.powi(6) * param.s2
                                    + -30893. * param.s1.powi(5) * param.s2.powi(2)
                                    + 20692. * param.s1.powi(4) * param.s2.powi(3)
                                    + -140. * param.s1.powi(3) * param.s2.powi(4)
                                    + 2518. * param.s1.powi(2) * param.s2.powi(5)
                                    + -895. * param.s1 * param.s2.powi(6)
                                    + 120. * param.s2.powi(7))
                            - param.s12.powi(5)
                                * (2058. * param.s1.powi(3)
                                    + 1478. * param.s1.powi(2) * param.s2
                                    + 1275. * param.s1 * param.s2.powi(2)
                                    + 840. * param.s2.powi(3)))
                        + param.m1_2
                            * param.s1.powi(2)
                            * (18. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (630. * param.s1.powi(2)
                                        + -1453. * param.s1 * param.s2
                                        + -301. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(5)
                                    * (18. * param.s1.powi(3)
                                        + 349. * param.s1.powi(2) * param.s2
                                        + 64. * param.s1 * param.s2.powi(2)
                                        + -11. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (252. * param.s1.powi(3)
                                        + -16839. * param.s1.powi(2) * param.s2
                                        + 6394. * param.s1 * param.s2.powi(2)
                                        + 1029. * param.s2.powi(3))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (360. * param.s1.powi(4)
                                        + 12563. * param.s1.powi(3) * param.s2
                                        + 19999. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1549. * param.s1 * param.s2.powi(3)
                                        + -31. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(4)
                                    * (504. * param.s1.powi(4)
                                        + -9417. * param.s1.powi(3) * param.s2
                                        + 4651. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1153. * param.s1 * param.s2.powi(3)
                                        + 301. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (3402. * param.s1.powi(5)
                                        + -25785. * param.s1.powi(4) * param.s2
                                        + -66680. * param.s1.powi(3) * param.s2.powi(2)
                                        + 79410. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2370. * param.s1 * param.s2.powi(4)
                                        + 1127. * param.s2.powi(5))
                                + 42.
                                    * param.m2_2.powi(2)
                                    * (3. * param.s12.powi(6)
                                        + -9.
                                            * param.s12.powi(5)
                                            * (7. * param.s1 + 2. * param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (68. * param.s1.powi(2)
                                                + -39. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + 10.
                                            * param.s12.powi(3)
                                            * (108. * param.s1.powi(3)
                                                + -68. * param.s1.powi(2) * param.s2
                                                + -15. * param.s1 * param.s2.powi(2)
                                                + -6. * param.s2.powi(3))
                                        + 3. * (param.s1 - param.s2).powi(3)
                                            * (96. * param.s1.powi(3)
                                                + 159. * param.s1.powi(2) * param.s2
                                                + 16. * param.s1 * param.s2.powi(2)
                                                - param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-615. * param.s1.powi(4)
                                                + -1540. * param.s1.powi(3) * param.s2
                                                + 2056. * param.s1.powi(2) * param.s2.powi(2)
                                                + -90. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        - param.s12
                                            * (353. * param.s1.powi(5)
                                                + -2430. * param.s1.powi(4) * param.s2
                                                + 1512. * param.s1.powi(3) * param.s2.powi(2)
                                                + 712. * param.s1.powi(2) * param.s2.powi(3)
                                                + -165. * param.s1 * param.s2.powi(4)
                                                + 18. * param.s2.powi(5)))
                                + -7.
                                    * param.m2_2
                                    * (19. * param.s12.powi(7)
                                        + param.s12.powi(5)
                                            * (-465. * param.s1.powi(2)
                                                + 574. * param.s1 * param.s2
                                                + 183. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (891. * param.s1.powi(3)
                                                + -2179. * param.s1.powi(2) * param.s2
                                                + 185. * param.s1 * param.s2.powi(2)
                                                + -25. * param.s2.powi(3))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (389. * param.s1.powi(4)
                                                + -11680. * param.s1.powi(3) * param.s2
                                                + -9930. * param.s1.powi(2) * param.s2.powi(2)
                                                + 88. * param.s1 * param.s2.powi(3)
                                                + -83. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (1371. * param.s1.powi(4)
                                                + -1820. * param.s1.powi(3) * param.s2
                                                + -2358. * param.s1.powi(2) * param.s2.powi(2)
                                                + 492. * param.s1 * param.s2.powi(3)
                                                + 11. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (3321. * param.s1.powi(5)
                                                + 13225. * param.s1.powi(4) * param.s2
                                                + -37782.
                                                    * param.s1.powi(3)
                                                    * param.s2.powi(2)
                                                + 10362. * param.s1.powi(2) * param.s2.powi(3)
                                                + 1325. * param.s1 * param.s2.powi(4)
                                                + 141. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(4)
                                            * (515. * param.s1.powi(3)
                                                + 1509. * param.s1.powi(2) * param.s2
                                                + 201. * param.s1 * param.s2.powi(2)
                                                + -17. * param.s2.powi(3))
                                        - param.s12.powi(6)
                                            * (349. * param.s1 + 97. * param.s2))
                                - param.s12.powi(2)
                                    * (1890. * param.s1.powi(6)
                                        + 14739. * param.s1.powi(5) * param.s2
                                        + -101953. * param.s1.powi(4) * param.s2.powi(2)
                                        + 65382. * param.s1.powi(3) * param.s2.powi(3)
                                        + 24768. * param.s1.powi(2) * param.s2.powi(4)
                                        + -5225. * param.s1 * param.s2.powi(5)
                                        + 399. * param.s2.powi(6))
                                - param.s12.powi(7) * (270. * param.s1 + 11. * param.s2))
                        + param.s1.powi(3)
                            * (30. * param.s12.powi(8)
                                + 19. * param.s12.powi(7) * (param.s1 + param.s2)
                                + param.s12.powi(6)
                                    * (-532. * param.s1.powi(2)
                                        + 4450. * param.s1 * param.s2
                                        + -532. * param.s2.powi(2))
                                + 2. * (param.s1 - param.s2).powi(6)
                                    * (param.s1.powi(2)
                                        + 13. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (1323. * param.s1.powi(3)
                                        + -6659. * param.s1.powi(2) * param.s2
                                        + -6659. * param.s1 * param.s2.powi(2)
                                        + 1323. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (42. * param.s1.powi(4)
                                        + 2801. * param.s1.powi(3) * param.s2
                                        + 9134. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2801. * param.s1 * param.s2.powi(3)
                                        + 42. * param.s2.powi(4))
                                + -10.
                                    * param.s12.powi(4)
                                    * (140. * param.s1.powi(4)
                                        + 647. * param.s1.powi(3) * param.s2
                                        + -3578. * param.s1.powi(2) * param.s2.powi(2)
                                        + 647. * param.s1 * param.s2.powi(3)
                                        + 140. * param.s2.powi(4))
                                + 5. * param.s12.powi(3)
                                    * (133. * param.s1.powi(5)
                                        + 2913. * param.s1.powi(4) * param.s2
                                        + -4582. * param.s1.powi(3) * param.s2.powi(2)
                                        + -4582. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2913. * param.s1 * param.s2.powi(4)
                                        + 133. * param.s2.powi(5))
                                + -70.
                                    * param.m2_2.powi(3)
                                    * (3. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (107. * param.s1 + -15. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (89. * param.s1.powi(2)
                                                + 70. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (52. * param.s1.powi(2)
                                                + -260. * param.s1 * param.s2
                                                + 30. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-348. * param.s1.powi(3)
                                                + 368. * param.s1.powi(2) * param.s2
                                                + 138. * param.s1 * param.s2.powi(2)
                                                + -30. * param.s2.powi(3))
                                        + param.s12
                                            * (97. * param.s1.powi(4)
                                                + 344. * param.s1.powi(3) * param.s2
                                                + -532. * param.s1.powi(2) * param.s2.powi(2)
                                                + 76. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4)))
                                + 21.
                                    * param.m2_2.powi(2)
                                    * (21. * param.s12.powi(6)
                                        + 24.
                                            * param.s12.powi(5)
                                            * (21. * param.s1 + -4. * param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (139. * param.s1.powi(2)
                                                + 98. * param.s1 * param.s2
                                                + -33. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (109. * param.s1.powi(3)
                                                + 653. * param.s1.powi(2) * param.s2
                                                + 128. * param.s1 * param.s2.powi(2)
                                                + -6. * param.s2.powi(3))
                                        + -20.
                                            * param.s12.powi(3)
                                            * (49. * param.s1.powi(3)
                                                + -225. * param.s1.powi(2) * param.s2
                                                + 70. * param.s1 * param.s2.powi(2)
                                                + 6. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (1755. * param.s1.powi(4)
                                                + -2660. * param.s1.powi(3) * param.s2
                                                + -2778. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2100. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(4)
                                            * (169. * param.s1.powi(2)
                                                + 190. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2)))
                                + -7.
                                    * param.m2_2
                                    * (37. * param.s12.powi(7)
                                        + param.s12.powi(6)
                                            * (467. * param.s1 + -133. * param.s2)
                                        + (param.s1 - param.s2).powi(5)
                                            * (19. * param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(5)
                                            * (-1689. * param.s1.powi(2)
                                                + 1630. * param.s1 * param.s2
                                                + 111. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (273. * param.s1.powi(3)
                                                + 983. * param.s1.powi(2) * param.s2
                                                + -1237. * param.s1 * param.s2.powi(2)
                                                + 29. * param.s2.powi(3))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (473. * param.s1.powi(3)
                                                + 2857. * param.s1.powi(2) * param.s2
                                                + 1543. * param.s1 * param.s2.powi(2)
                                                + 47. * param.s2.powi(3))
                                        + 5. * param.s12.powi(3)
                                            * (159. * param.s1.powi(4)
                                                + -2572. * param.s1.powi(3) * param.s2
                                                + 1602. * param.s1.powi(2) * param.s2.powi(2)
                                                + 828. * param.s1 * param.s2.powi(3)
                                                + -65. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (-1467. * param.s1.powi(5)
                                                + 5065. * param.s1.powi(4) * param.s2
                                                + 7842. * param.s1.powi(3) * param.s2.powi(2)
                                                + -13038.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(3)
                                                + 1385. * param.s1 * param.s2.powi(4)
                                                + 213. * param.s2.powi(5)))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (23. * param.s1.powi(3)
                                        + 577. * param.s1.powi(2) * param.s2
                                        + 577. * param.s1 * param.s2.powi(2)
                                        + 23. * param.s2.powi(3)))
                        + param.m1_2.powi(2)
                            * param.s1
                            * (18. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (1134. * param.s1.powi(2)
                                        + 59. * param.s1 * param.s2
                                        + 14. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-2835. * param.s1.powi(3)
                                        + 5484. * param.s1.powi(2) * param.s2
                                        + 2173. * param.s1 * param.s2.powi(2)
                                        + 462. * param.s2.powi(3))
                                + 5. * param.s12.powi(4)
                                    * (630. * param.s1.powi(4)
                                        + 1983. * param.s1.powi(3) * param.s2
                                        + -2278. * param.s1.powi(2) * param.s2.powi(2)
                                        + -901. * param.s1 * param.s2.powi(3)
                                        + -238. * param.s2.powi(4))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (1431. * param.s1.powi(5)
                                        + -10162. * param.s1.powi(4) * param.s2
                                        + -61150. * param.s1.powi(3) * param.s2.powi(2)
                                        + -3204. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1517. * param.s1 * param.s2.powi(4)
                                        + 346. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (-693. * param.s1.powi(5)
                                        + -57810. * param.s1.powi(4) * param.s2
                                        + 90190. * param.s1.powi(3) * param.s2.powi(2)
                                        + -6060. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2355. * param.s1 * param.s2.powi(4)
                                        + 1442. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (-1638. * param.s1.powi(6)
                                        + 59769. * param.s1.powi(5) * param.s2
                                        + -60482. * param.s1.powi(4) * param.s2.powi(2)
                                        + -45978. * param.s1.powi(3) * param.s2.powi(3)
                                        + 14670. * param.s1.powi(2) * param.s2.powi(4)
                                        + 1697. * param.s1 * param.s2.powi(5)
                                        + -966. * param.s2.powi(6))
                                + -14.
                                    * param.m2_2
                                    * (5. * param.s12.powi(7)
                                        + param.s12.powi(5)
                                            * (402. * param.s1.powi(2)
                                                + 242. * param.s1 * param.s2
                                                + 105. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(4)
                                            * (129. * param.s1.powi(3)
                                                + 56. * param.s1.powi(2) * param.s2
                                                + 56. * param.s1 * param.s2.powi(2)
                                                + 35. * param.s2.powi(3))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (75. * param.s1.powi(4)
                                                + -904. * param.s1.powi(3) * param.s2
                                                + 210. * param.s1.powi(2) * param.s2.powi(2)
                                                + 12. * param.s1 * param.s2.powi(3)
                                                + -35. * param.s2.powi(4))
                                        + (param.s1 - param.s2).powi(3)
                                            * (443. * param.s1.powi(4)
                                                + 1687. * param.s1.powi(3) * param.s2
                                                + 348. * param.s1.powi(2) * param.s2.powi(2)
                                                + -53. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (1800. * param.s1.powi(5)
                                                + -8635. * param.s1.powi(4) * param.s2
                                                + 2976. * param.s1.powi(3) * param.s2.powi(2)
                                                + 810. * param.s1.powi(2) * param.s2.powi(3)
                                                + 370. * param.s1 * param.s2.powi(4)
                                                + -105. * param.s2.powi(5))
                                        + param.s12
                                            * (-1568. * param.s1.powi(6)
                                                + 3830. * param.s1.powi(5) * param.s2
                                                + 3713. * param.s1.powi(4) * param.s2.powi(2)
                                                + -6372. * param.s1.powi(3) * param.s2.powi(3)
                                                + 640. * param.s1.powi(2) * param.s2.powi(4)
                                                + -278. * param.s1 * param.s2.powi(5)
                                                + 35. * param.s2.powi(6))
                                        - param.s12.powi(6) * (62. * param.s1 + 35. * param.s2))
                                - (param.s1 - param.s2).powi(4)
                                    * (360. * param.s1.powi(4)
                                        + 5759. * param.s1.powi(3) * param.s2
                                        + 1986. * param.s1.powi(2) * param.s2.powi(2)
                                        + -429. * param.s1 * param.s2.powi(3)
                                        + 52. * param.s2.powi(4))
                                - param.s12.powi(7) * (207. * param.s1 + 74. * param.s2)))
                + param.m0_2.powi(2)
                    * (param.m1_2.powi(4)
                        * (45. * param.s12.powi(8)
                            + -10. * param.s12.powi(7) * (43. * param.s1 + 36. * param.s2)
                            + param.s12.powi(6)
                                * (1813. * param.s1.powi(2)
                                    + 2020. * param.s1 * param.s2
                                    + 1260. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(5)
                                * (2184. * param.s1.powi(3)
                                    + 1559. * param.s1.powi(2) * param.s2
                                    + 1545. * param.s1 * param.s2.powi(2)
                                    + 1260. * param.s2.powi(3))
                            + 5. * param.s12.powi(4)
                                * (1309. * param.s1.powi(4)
                                    + -926. * param.s1.powi(3) * param.s2
                                    + -431. * param.s1.powi(2) * param.s2.powi(2)
                                    + 40. * param.s1 * param.s2.powi(3)
                                    + 630. * param.s2.powi(4))
                            + -10.
                                * param.s12.powi(3)
                                * (623. * param.s1.powi(5)
                                    + -2190. * param.s1.powi(4) * param.s2
                                    + 150. * param.s1.powi(3) * param.s2.powi(2)
                                    + -354. * param.s1.powi(2) * param.s2.powi(3)
                                    + -475. * param.s1 * param.s2.powi(4)
                                    + 252. * param.s2.powi(5))
                            + (param.s1 - param.s2).powi(2)
                                * (178. * param.s1.powi(6)
                                    + -3746. * param.s1.powi(5) * param.s2
                                    + -36269. * param.s1.powi(4) * param.s2.powi(2)
                                    + -11396. * param.s1.powi(3) * param.s2.powi(3)
                                    + 2518. * param.s1.powi(2) * param.s2.powi(4)
                                    + -470. * param.s1 * param.s2.powi(5)
                                    + 45. * param.s2.powi(6))
                            + param.s12.powi(2)
                                * (3675. * param.s1.powi(6)
                                    + -29480. * param.s1.powi(5) * param.s2
                                    + -13074. * param.s1.powi(4) * param.s2.powi(2)
                                    + 17460. * param.s1.powi(3) * param.s2.powi(3)
                                    + 6295. * param.s1.powi(2) * param.s2.powi(4)
                                    + -5820. * param.s1 * param.s2.powi(5)
                                    + 1260. * param.s2.powi(6))
                            + -2.
                                * param.s12
                                * (614. * param.s1.powi(7)
                                    + -8885. * param.s1.powi(6) * param.s2
                                    + -23579. * param.s1.powi(5) * param.s2.powi(2)
                                    + 48286. * param.s1.powi(4) * param.s2.powi(3)
                                    + -4970. * param.s1.powi(3) * param.s2.powi(4)
                                    + 4939. * param.s1.powi(2) * param.s2.powi(5)
                                    + -1465. * param.s1 * param.s2.powi(6)
                                    + 180. * param.s2.powi(7)))
                        + param.s1.powi(4)
                            * (45. * param.s12.powi(8)
                                + -2. * param.s12.powi(7) * (124. * param.s1 + 439. * param.s2)
                                + param.s12.powi(6)
                                    * (539. * param.s1.powi(2)
                                        + 2020. * param.s1 * param.s2
                                        + -196. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (23. * param.s1.powi(3)
                                        + -88. * param.s1.powi(2) * param.s2
                                        + -578. * param.s1 * param.s2.powi(2)
                                        + -257. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (-546. * param.s1.powi(3)
                                        + 508. * param.s1.powi(2) * param.s2
                                        + -22592. * param.s1 * param.s2.powi(2)
                                        + 8274. * param.s2.powi(3))
                                + 5. * param.s12.powi(4)
                                    * (35. * param.s1.powi(4)
                                        + -926. * param.s1.powi(3) * param.s2
                                        + 5407. * param.s1.powi(2) * param.s2.powi(2)
                                        + 4352. * param.s1 * param.s2.powi(3)
                                        + -3024. * param.s2.powi(4))
                                + 10.
                                    * param.s12.powi(3)
                                    * (14. * param.s1.powi(5)
                                        + 377. * param.s1.powi(4) * param.s2
                                        + 1404. * param.s1.powi(3) * param.s2.powi(2)
                                        + -6884. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1798. * param.s1 * param.s2.powi(4)
                                        + 987. * param.s2.powi(5))
                                + -70.
                                    * param.m2_2.powi(4)
                                    * (61. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (266. * param.s1 + -244. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (271. * param.s1.powi(2)
                                                + 370. * param.s1 * param.s2
                                                + 61. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-444. * param.s1.powi(2)
                                                + -284. * param.s1 * param.s2
                                                + 366. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (77. * param.s1.powi(3)
                                                + -530. * param.s1.powi(2) * param.s2
                                                + 115. * param.s1 * param.s2.powi(2)
                                                + 122. * param.s2.powi(3)))
                                + 140.
                                    * param.m2_2.powi(3)
                                    * (63. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (113. * param.s1 + -193. * param.s2)
                                        + param.s12.powi(3)
                                            * (-572. * param.s1.powi(2)
                                                + 584. * param.s1 * param.s2
                                                + 142. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (204. * param.s1.powi(3)
                                                + 382. * param.s1.powi(2) * param.s2
                                                + -701. * param.s1 * param.s2.powi(2)
                                                + 51. * param.s2.powi(3))
                                        + param.s12
                                            * (133. * param.s1.powi(4)
                                                + -1308. * param.s1.powi(3) * param.s2
                                                + 748. * param.s1.powi(2) * param.s2.powi(2)
                                                + 600. * param.s1 * param.s2.powi(3)
                                                + -173. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (145. * param.s1.powi(2)
                                                + 282. * param.s1 * param.s2
                                                + 59. * param.s2.powi(2)))
                                + -42.
                                    * param.m2_2.powi(2)
                                    * (121. * param.s12.powi(6)
                                        + -96. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + (param.s1 - param.s2).powi(4)
                                            * (101. * param.s1.powi(2)
                                                + 350. * param.s1 * param.s2
                                                + 101. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(4)
                                            * (145. * param.s1.powi(2)
                                                + -578. * param.s1 * param.s2
                                                + 145. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (4. * param.s1.powi(3)
                                                + -667. * param.s1.powi(2) * param.s2
                                                + -667. * param.s1 * param.s2.powi(2)
                                                + 4. * param.s2.powi(3))
                                        + 20.
                                            * param.s12.powi(3)
                                            * (72. * param.s1.powi(3)
                                                + -145. * param.s1.powi(2) * param.s2
                                                + -145. * param.s1 * param.s2.powi(2)
                                                + 72. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (825. * param.s1.powi(4)
                                                + 2540. * param.s1.powi(3) * param.s2
                                                + -9082. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2540. * param.s1 * param.s2.powi(3)
                                                + 825. * param.s2.powi(4)))
                                + 14.
                                    * param.m2_2
                                    * (37. * param.s12.powi(7)
                                        + param.s12.powi(6)
                                            * (-133. * param.s1 + 467. * param.s2)
                                        + param.s12.powi(5)
                                            * (111. * param.s1.powi(2)
                                                + 1630. * param.s1 * param.s2
                                                + -1689. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (29. * param.s1.powi(3)
                                                + -1237. * param.s1.powi(2) * param.s2
                                                + 983. * param.s1 * param.s2.powi(2)
                                                + 273. * param.s2.powi(3))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (65. * param.s1.powi(4)
                                                + -828. * param.s1.powi(3) * param.s2
                                                + -1602. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2572. * param.s1 * param.s2.powi(3)
                                                + -159. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (213. * param.s1.powi(5)
                                                + 1385. * param.s1.powi(4) * param.s2
                                                + -13038.
                                                    * param.s1.powi(3)
                                                    * param.s2.powi(2)
                                                + 7842. * param.s1.powi(2) * param.s2.powi(3)
                                                + 5065. * param.s1 * param.s2.powi(4)
                                                + -1467. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (47. * param.s1.powi(3)
                                                + 1543. * param.s1.powi(2) * param.s2
                                                + 2857. * param.s1 * param.s2.powi(2)
                                                + 473. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(2)
                                                + 40. * param.s1 * param.s2
                                                + 19. * param.s2.powi(2)))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (147. * param.s1.powi(4)
                                        + 766. * param.s1.powi(3) * param.s2
                                        + 19359. * param.s1.powi(2) * param.s2.powi(2)
                                        + 22676. * param.s1 * param.s2.powi(3)
                                        + 1512. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(6)
                                    * (4. * param.s1.powi(2)
                                        + -18. * param.s1 * param.s2
                                        + -31. * param.s2.powi(2)))
                        + -2.
                            * param.m1_2
                            * param.s1.powi(3)
                            * (18. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (-301. * param.s1.powi(2)
                                        + -1453. * param.s1 * param.s2
                                        + 630. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(5)
                                    * (11. * param.s1.powi(3)
                                        + -64. * param.s1.powi(2) * param.s2
                                        + -349. * param.s1 * param.s2.powi(2)
                                        + -18. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (1029. * param.s1.powi(3)
                                        + 6394. * param.s1.powi(2) * param.s2
                                        + -16839. * param.s1 * param.s2.powi(2)
                                        + 252. * param.s2.powi(3))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (31. * param.s1.powi(4)
                                        + -1549. * param.s1.powi(3) * param.s2
                                        + -19999. * param.s1.powi(2) * param.s2.powi(2)
                                        + -12563. * param.s1 * param.s2.powi(3)
                                        + -360. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(4)
                                    * (301. * param.s1.powi(4)
                                        + 1153. * param.s1.powi(3) * param.s2
                                        + 4651. * param.s1.powi(2) * param.s2.powi(2)
                                        + -9417. * param.s1 * param.s2.powi(3)
                                        + 504. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (1127. * param.s1.powi(5)
                                        + -2370. * param.s1.powi(4) * param.s2
                                        + 79410. * param.s1.powi(3) * param.s2.powi(2)
                                        + -66680. * param.s1.powi(2) * param.s2.powi(3)
                                        + -25785. * param.s1 * param.s2.powi(4)
                                        + 3402. * param.s2.powi(5))
                                + 70.
                                    * param.m2_2.powi(3)
                                    * (9. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (199. * param.s1 + -45. * param.s2)
                                        + param.s12.powi(3)
                                            * (-376. * param.s1.powi(2)
                                                + -292. * param.s1 * param.s2
                                                + 90. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (78. * param.s1.powi(3)
                                                + -836. * param.s1.powi(2) * param.s2
                                                + 159. * param.s1 * param.s2.powi(2)
                                                + 45. * param.s2.powi(3))
                                        + param.s12
                                            * (599. * param.s1.powi(4)
                                                + -1088. * param.s1.powi(3) * param.s2
                                                + -1136. * param.s1.powi(2) * param.s2.powi(2)
                                                + 716. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(2)
                                            * (275. * param.s1.powi(3)
                                                + 797. * param.s1.powi(2) * param.s2
                                                + 323. * param.s1 * param.s2.powi(2)
                                                + 9. * param.s2.powi(3)))
                                + 42.
                                    * param.m2_2
                                    * (3. * param.s12.powi(7)
                                        + 15. * param.s12.powi(6) * (param.s1 + param.s2)
                                        + -4.
                                            * param.s12.powi(5)
                                            * (27. * param.s1.powi(2)
                                                + -194. * param.s1 * param.s2
                                                + 27. * param.s2.powi(2))
                                        + -12.
                                            * (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(3)
                                                + 22. * param.s1.powi(2) * param.s2
                                                + 22. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + 10.
                                            * param.s12.powi(4)
                                            * (21. * param.s1.powi(3)
                                                + -95. * param.s1.powi(2) * param.s2
                                                + -95. * param.s1 * param.s2.powi(2)
                                                + 21. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (15. * param.s1.powi(4)
                                                + -380. * param.s1.powi(3) * param.s2
                                                + -1922. * param.s1.powi(2) * param.s2.powi(2)
                                                + -380. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (33. * param.s1.powi(4)
                                                + 348. * param.s1.powi(3) * param.s2
                                                + -1354. * param.s1.powi(2) * param.s2.powi(2)
                                                + 348. * param.s1 * param.s2.powi(3)
                                                + 33. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (27. * param.s1.powi(5)
                                                + 2935. * param.s1.powi(4) * param.s2
                                                + -4138. * param.s1.powi(3) * param.s2.powi(2)
                                                + -4138. * param.s1.powi(2) * param.s2.powi(3)
                                                + 2935. * param.s1 * param.s2.powi(4)
                                                + 27. * param.s2.powi(5)))
                                + -42.
                                    * param.m2_2.powi(2)
                                    * (18. * param.s12.powi(6)
                                        + 63. * param.s12.powi(5) * (4. * param.s1 - param.s2)
                                        + param.s12.powi(4)
                                            * (-920. * param.s1.powi(2)
                                                + 815. * param.s1 * param.s2
                                                + 45. * param.s2.powi(2))
                                        + 10.
                                            * param.s12.powi(3)
                                            * (80. * param.s1.powi(3)
                                                + 252. * param.s1.powi(2) * param.s2
                                                + -326. * param.s1 * param.s2.powi(2)
                                                + 9. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(3)
                                            * (268. * param.s1.powi(3)
                                                + 1357. * param.s1.powi(2) * param.s2
                                                + 778. * param.s1 * param.s2.powi(2)
                                                + 27. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (165. * param.s1.powi(4)
                                                + -3340. * param.s1.powi(3) * param.s2
                                                + 2008. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1185. * param.s1 * param.s2.powi(3)
                                                + -90. * param.s2.powi(4))
                                        + param.s12
                                            * (-748. * param.s1.powi(5)
                                                + 2855. * param.s1.powi(4) * param.s2
                                                + 3768. * param.s1.powi(3) * param.s2.powi(2)
                                                + -6512. * param.s1.powi(2) * param.s2.powi(3)
                                                + 520. * param.s1 * param.s2.powi(4)
                                                + 117. * param.s2.powi(5)))
                                - param.s12.powi(2)
                                    * (399. * param.s1.powi(6)
                                        + -5225. * param.s1.powi(5) * param.s2
                                        + 24768. * param.s1.powi(4) * param.s2.powi(2)
                                        + 65382. * param.s1.powi(3) * param.s2.powi(3)
                                        + -101953. * param.s1.powi(2) * param.s2.powi(4)
                                        + 14739. * param.s1 * param.s2.powi(5)
                                        + 1890. * param.s2.powi(6))
                                - param.s12.powi(7) * (11. * param.s1 + 270. * param.s2))
                        + -2.
                            * param.m1_2.powi(3)
                            * param.s1
                            * (18. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (973. * param.s1.powi(2)
                                        + 1781. * param.s1 * param.s2
                                        + 994. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (973. * param.s1.powi(4)
                                        + 2081. * param.s1.powi(3) * param.s2
                                        + 263. * param.s1.powi(2) * param.s2.powi(2)
                                        + 821. * param.s1 * param.s2.powi(3)
                                        + 742. * param.s2.powi(4))
                                + (param.s1 - param.s2).powi(3)
                                    * (193. * param.s1.powi(5)
                                        + -3222. * param.s1.powi(4) * param.s2
                                        + -25690. * param.s1.powi(3) * param.s2.powi(2)
                                        + -6146. * param.s1.powi(2) * param.s2.powi(3)
                                        + 933. * param.s1 * param.s2.powi(4)
                                        + -88. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (3423. * param.s1.powi(6)
                                        + -14081. * param.s1.powi(5) * param.s2
                                        + 104032. * param.s1.powi(4) * param.s2.powi(2)
                                        + -54378. * param.s1.powi(3) * param.s2.powi(3)
                                        + -9305. * param.s1.powi(2) * param.s2.powi(4)
                                        + -7697. * param.s1 * param.s2.powi(5)
                                        + 1974. * param.s2.powi(6))
                                + 14.
                                    * param.m2_2
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
                                                + 204. * param.s1.powi(3) * param.s2
                                                + -210. * param.s1.powi(2) * param.s2.powi(2)
                                                + -48. * param.s1 * param.s2.powi(3)
                                                + 35. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (-825. * param.s1.powi(5)
                                                + -4895. * param.s1.powi(4) * param.s2
                                                + 7554. * param.s1.powi(3) * param.s2.powi(2)
                                                + 90. * param.s1.powi(2) * param.s2.powi(3)
                                                + 505. * param.s1 * param.s2.powi(4)
                                                + -105. * param.s2.powi(5))
                                        + param.s12
                                            * (368. * param.s1.powi(6)
                                                + 5680. * param.s1.powi(5) * param.s2
                                                + -6503. * param.s1.powi(4) * param.s2.powi(2)
                                                + -4928. * param.s1.powi(3) * param.s2.powi(3)
                                                + 1360. * param.s1.powi(2) * param.s2.powi(4)
                                                + -332. * param.s1 * param.s2.powi(5)
                                                + 35. * param.s2.powi(6))
                                        - (param.s1 - param.s2).powi(2)
                                            * (68. * param.s1.powi(5)
                                                + 2174. * param.s1.powi(4) * param.s2
                                                + 4241. * param.s1.powi(3) * param.s2.powi(2)
                                                + 599. * param.s1.powi(2) * param.s2.powi(3)
                                                + -67. * param.s1 * param.s2.powi(4)
                                                + 5. * param.s2.powi(5))
                                        - param.s12.powi(6) * (53. * param.s1 + 35. * param.s2))
                                - param.s12
                                    * (1243. * param.s1.powi(7)
                                        + -13254. * param.s1.powi(6) * param.s2
                                        + 31513. * param.s1.powi(5) * param.s2.powi(2)
                                        + 68026. * param.s1.powi(4) * param.s2.powi(3)
                                        + -96971. * param.s1.powi(3) * param.s2.powi(4)
                                        + 13978. * param.s1.powi(2) * param.s2.powi(5)
                                        + -5169. * param.s1 * param.s2.powi(6)
                                        + 634. * param.s2.powi(7))
                                - param.s12.powi(3)
                                    * (5243. * param.s1.powi(5)
                                        + 410. * param.s1.powi(4) * param.s2
                                        + 54570. * param.s1.powi(3) * param.s2.powi(2)
                                        + -18720. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2845. * param.s1 * param.s2.powi(4)
                                        + 3458. * param.s2.powi(5))
                                - param.s12.powi(5)
                                    * (2793. * param.s1.powi(3)
                                        + 6934. * param.s1.powi(2) * param.s2
                                        + 4813. * param.s1 * param.s2.powi(2)
                                        + 2478. * param.s2.powi(3))
                                - param.s12.powi(7) * (193. * param.s1 + 214. * param.s2))
                        + 6. * param.m1_2.powi(2)
                            * param.s1.powi(2)
                            * (7.
                                * param.m2_2.powi(2)
                                * (9. * param.s12.powi(6)
                                    + -18.
                                        * param.s12.powi(5)
                                        * (8. * param.s1 + 3. * param.s2)
                                    + -5.
                                        * param.s12.powi(4)
                                        * (5. * param.s1.powi(2)
                                            + -72. * param.s1 * param.s2
                                            + -27. * param.s2.powi(2))
                                    + 20.
                                        * param.s12.powi(3)
                                        * (68. * param.s1.powi(3)
                                            + -175. * param.s1.powi(2) * param.s2
                                            + -9. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (-2625. * param.s1.powi(4)
                                            + 3740. * param.s1.powi(3) * param.s2
                                            + 4578. * param.s1.powi(2) * param.s2.powi(2)
                                            + -720. * param.s1 * param.s2.powi(3)
                                            + 135. * param.s2.powi(4))
                                    + 2. * param.s12
                                        * (968. * param.s1.powi(5)
                                            + 925. * param.s1.powi(4) * param.s2
                                            + -5108. * param.s1.powi(3) * param.s2.powi(2)
                                            + 722. * param.s1.powi(2) * param.s2.powi(3)
                                            + 360. * param.s1 * param.s2.powi(4)
                                            + -27. * param.s2.powi(5))
                                    - (param.s1 - param.s2).powi(2)
                                        * (511. * param.s1.powi(4)
                                            + 3418. * param.s1.powi(3) * param.s2
                                            + 2902. * param.s1.powi(2) * param.s2.powi(2)
                                            + 198. * param.s1 * param.s2.powi(3)
                                            + -9. * param.s2.powi(4)))
                                + -3.
                                    * (param.s12.powi(8)
                                        + -15. * param.s12.powi(7) * (param.s1 + param.s2)
                                        + 4. * param.s12.powi(6)
                                            * (14. * param.s1.powi(2)
                                                + 47. * param.s1 * param.s2
                                                + 14. * param.s2.powi(2))
                                        + -10.
                                            * param.s1
                                            * param.s12.powi(4)
                                            * param.s2
                                            * (103. * param.s1.powi(2)
                                                + -648. * param.s1 * param.s2
                                                + 103. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (69. * param.s1.powi(5)
                                                + -185. * param.s1.powi(4) * param.s2
                                                + 6304. * param.s1.powi(3) * param.s2.powi(2)
                                                + 6304. * param.s1.powi(2) * param.s2.powi(3)
                                                + -185. * param.s1 * param.s2.powi(4)
                                                + 69. * param.s2.powi(5))
                                        + param.s12.powi(3)
                                            * (119. * param.s1.powi(5)
                                                + 1735. * param.s1.powi(4) * param.s2
                                                + -5610. * param.s1.powi(3) * param.s2.powi(2)
                                                + -5610. * param.s1.powi(2) * param.s2.powi(3)
                                                + 1735. * param.s1 * param.s2.powi(4)
                                                + 119. * param.s2.powi(5))
                                        + -4.
                                            * param.s12.powi(2)
                                            * (35. * param.s1.powi(6)
                                                + 170. * param.s1.powi(5) * param.s2
                                                + 1968. * param.s1.powi(4) * param.s2.powi(2)
                                                + -5718. * param.s1.powi(3) * param.s2.powi(3)
                                                + 1968. * param.s1.powi(2) * param.s2.powi(4)
                                                + 170. * param.s1 * param.s2.powi(5)
                                                + 35. * param.s2.powi(6))
                                        - (param.s1 - param.s2).powi(4)
                                            * (13. * param.s1.powi(4)
                                                + -158. * param.s1.powi(3) * param.s2
                                                + -998. * param.s1.powi(2) * param.s2.powi(2)
                                                + -158. * param.s1 * param.s2.powi(3)
                                                + 13. * param.s2.powi(4))
                                        - param.s12.powi(5)
                                            * (77. * param.s1.powi(3)
                                                + 85. * param.s1.powi(2) * param.s2
                                                + 85. * param.s1 * param.s2.powi(2)
                                                + 77. * param.s2.powi(3)))
                                + -7.
                                    * param.m2_2
                                    * (3. * param.s12.powi(7)
                                        + -3. * param.s12.powi(6) * (13. * param.s1 + param.s2)
                                        + 3. * param.s12.powi(5)
                                            * (27. * param.s1.powi(2)
                                                + -74. * param.s1 * param.s2
                                                + -15. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (15. * param.s1.powi(3)
                                                + -499. * param.s1.powi(2) * param.s2
                                                + 195. * param.s1 * param.s2.powi(2)
                                                + 33. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(3)
                                            * (69. * param.s1.powi(4)
                                                + 1834. * param.s1.powi(3) * param.s2
                                                + 2714. * param.s1.powi(2) * param.s2.powi(2)
                                                + 258. * param.s1 * param.s2.powi(3)
                                                + -15. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (87. * param.s1.powi(4)
                                                + -1444. * param.s1.powi(3) * param.s2
                                                + 650. * param.s1.powi(2) * param.s2.powi(2)
                                                + 180. * param.s1 * param.s2.powi(3)
                                                + 51. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (567. * param.s1.powi(5)
                                                + -4425. * param.s1.powi(4) * param.s2
                                                + -9626. * param.s1.powi(3) * param.s2.powi(2)
                                                + 11766. * param.s1.powi(2) * param.s2.powi(3)
                                                + -345. * param.s1 * param.s2.powi(4)
                                                + 207. * param.s2.powi(5))
                                        - param.s12
                                            * (321. * param.s1.powi(6)
                                                + 1702. * param.s1.powi(5) * param.s2
                                                + -14527.
                                                    * param.s1.powi(4)
                                                    * param.s2.powi(2)
                                                + 9044. * param.s1.powi(3) * param.s2.powi(3)
                                                + 4207. * param.s1.powi(2) * param.s2.powi(4)
                                                + -834. * param.s1 * param.s2.powi(5)
                                                + 87. * param.s2.powi(6)))))
                + -2.
                    * param.m0_2
                    * (param.m1_2.powi(5)
                        * (16. * param.s1.powi(8)
                            + 9. * param.s12.powi(8)
                            + -259. * param.s1.powi(7) * param.s2
                            + 2781. * param.s1.powi(6) * param.s2.powi(2)
                            + 14373. * param.s1.powi(5) * param.s2.powi(3)
                            + -12843. * param.s1.powi(4) * param.s2.powi(4)
                            + -4779. * param.s1.powi(3) * param.s2.powi(5)
                            + 821. * param.s1.powi(2) * param.s2.powi(6)
                            + -119. * param.s1 * param.s2.powi(7)
                            + 9. * param.s2.powi(8)
                            + param.s12.powi(6)
                                * (301. * param.s1.powi(2)
                                    + 355. * param.s1 * param.s2
                                    + 252. * param.s2.powi(2))
                            + 5. * param.s12.powi(4)
                                * (175. * param.s1.powi(4)
                                    + -125. * param.s1.powi(3) * param.s2
                                    + -89. * param.s1.powi(2) * param.s2.powi(2)
                                    + -41. * param.s1 * param.s2.powi(3)
                                    + 126. * param.s2.powi(4))
                            + param.s12.powi(3)
                                * (-749. * param.s1.powi(5)
                                    + 2280. * param.s1.powi(4) * param.s2
                                    + -1140. * param.s1.powi(3) * param.s2.powi(2)
                                    + 120. * param.s1.powi(2) * param.s2.powi(3)
                                    + 1195. * param.s1 * param.s2.powi(4)
                                    + -504. * param.s2.powi(5))
                            + param.s12.powi(2)
                                * (399. * param.s1.powi(6)
                                    + -2543. * param.s1.powi(5) * param.s2
                                    + 7020. * param.s1.powi(4) * param.s2.powi(2)
                                    + 1980. * param.s1.powi(3) * param.s2.powi(3)
                                    + 2155. * param.s1.powi(2) * param.s2.powi(4)
                                    + -1311. * param.s1 * param.s2.powi(5)
                                    + 252. * param.s2.powi(6))
                            - param.s12
                                * (121. * param.s1.powi(7)
                                    + -1300. * param.s1.powi(6) * param.s2
                                    + 7997. * param.s1.powi(5) * param.s2.powi(2)
                                    + 15764. * param.s1.powi(4) * param.s2.powi(3)
                                    + -5215. * param.s1.powi(3) * param.s2.powi(4)
                                    + 2516. * param.s1.powi(2) * param.s2.powi(5)
                                    + -635. * param.s1 * param.s2.powi(6)
                                    + 72. * param.s2.powi(7))
                            - param.s12.powi(5)
                                * (651. * param.s1.powi(3)
                                    + 436. * param.s1.powi(2) * param.s2
                                    + 471. * param.s1 * param.s2.powi(2)
                                    + 504. * param.s2.powi(3))
                            - param.s12.powi(7) * (79. * param.s1 + 72. * param.s2))
                        + param.s1.powi(5)
                            * (9. * param.s12.powi(8)
                                + (param.s1 - param.s2).powi(6)
                                    * (2. * param.s1.powi(2)
                                        + -9. * param.s1 * param.s2
                                        + 16. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (203. * param.s1.powi(2)
                                        + 565. * param.s1 * param.s2
                                        + 1085. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (105. * param.s1.powi(4)
                                        + -247. * param.s1.powi(3) * param.s2
                                        + -684. * param.s1.powi(2) * param.s2.powi(2)
                                        + -6239. * param.s1 * param.s2.powi(3)
                                        + -1827. * param.s2.powi(4))
                                + 5. * param.s12.powi(4)
                                    * (77. * param.s1.powi(4)
                                        + 85. * param.s1.powi(3) * param.s2
                                        + -68. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2045. * param.s1 * param.s2.powi(3)
                                        + -1071. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (-259. * param.s1.powi(5)
                                        + 320. * param.s1.powi(4) * param.s2
                                        + 1800. * param.s1.powi(3) * param.s2.powi(2)
                                        + -5690. * param.s1.powi(2) * param.s2.powi(3)
                                        + -7205. * param.s1 * param.s2.powi(4)
                                        + 6426. * param.s2.powi(5))
                                + -210.
                                    * param.m2_2.powi(5)
                                    * (31. * param.s1.powi(3)
                                        + 17. * param.s12.powi(3)
                                        + 29. * param.s1.powi(2) * param.s2
                                        + -43. * param.s1 * param.s2.powi(2)
                                        + -17. * param.s2.powi(3)
                                        + -3. * param.s12.powi(2) * (param.s1 + 17. * param.s2)
                                        + param.s12
                                            * (-45. * param.s1.powi(2)
                                                + 46. * param.s1 * param.s2
                                                + 51. * param.s2.powi(2)))
                                + 70.
                                    * param.m2_2.powi(4)
                                    * (107. * param.s12.powi(4)
                                        + -173. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-123. * param.s1.powi(2)
                                                + 836. * param.s1 * param.s2
                                                + -123. * param.s2.powi(2))
                                        + -2.
                                            * (param.s1 - param.s2).powi(2)
                                            * (74. * param.s1.powi(2)
                                                + 203. * param.s1 * param.s2
                                                + 74. * param.s2.powi(2))
                                        + param.s12
                                            * (337. * param.s1.powi(3)
                                                + -553. * param.s1.powi(2) * param.s2
                                                + -553. * param.s1 * param.s2.powi(2)
                                                + 337. * param.s2.powi(3)))
                                + -70.
                                    * param.m2_2.powi(3)
                                    * (63. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-193. * param.s1 + 113. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (71. * param.s1.powi(2)
                                                + 292. * param.s1 * param.s2
                                                + -286. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (59. * param.s1.powi(2)
                                                + 282. * param.s1 * param.s2
                                                + 145. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (51. * param.s1.powi(3)
                                                + -701. * param.s1.powi(2) * param.s2
                                                + 382. * param.s1 * param.s2.powi(2)
                                                + 204. * param.s2.powi(3))
                                        + param.s12
                                            * (-173. * param.s1.powi(4)
                                                + 600. * param.s1.powi(3) * param.s2
                                                + 748. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1308. * param.s1 * param.s2.powi(3)
                                                + 133. * param.s2.powi(4)))
                                + 21.
                                    * param.m2_2.powi(2)
                                    * (21. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-96. * param.s1 + 504. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (33. * param.s1.powi(2)
                                                + -98. * param.s1 * param.s2
                                                + -139. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (6. * param.s1.powi(3)
                                                + -128. * param.s1.powi(2) * param.s2
                                                + -653. * param.s1 * param.s2.powi(2)
                                                + -109. * param.s2.powi(3))
                                        + -20.
                                            * param.s12.powi(3)
                                            * (6. * param.s1.powi(3)
                                                + 70. * param.s1.powi(2) * param.s2
                                                + -225. * param.s1 * param.s2.powi(2)
                                                + 49. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (15. * param.s1.powi(4)
                                                + 2100. * param.s1.powi(3) * param.s2
                                                + -2778. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2660. * param.s1 * param.s2.powi(3)
                                                + 1755. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(4)
                                            * (9. * param.s1.powi(2)
                                                + 190. * param.s1 * param.s2
                                                + 169. * param.s2.powi(2)))
                                + 7. * param.m2_2
                                    * (8. * param.s12.powi(7)
                                        + 2. * param.s12.powi(5)
                                            * (57. * param.s1.powi(2)
                                                + 280. * param.s1 * param.s2
                                                + -483. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(2)
                                                + -5. * param.s1 * param.s2
                                                + -26. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(4)
                                            * (29. * param.s1.powi(3)
                                                + 86. * param.s1.powi(2) * param.s2
                                                + 413. * param.s1 * param.s2.powi(2)
                                                + -600. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(3)
                                                + -61. * param.s1.powi(2) * param.s2
                                                + -754. * param.s1 * param.s2.powi(2)
                                                + -416. * param.s2.powi(3))
                                        + 10.
                                            * param.s12.powi(3)
                                            * (10. * param.s1.powi(4)
                                                + -24. * param.s1.powi(3) * param.s2
                                                + 591. * param.s1.powi(2) * param.s2.powi(2)
                                                + -460. * param.s1 * param.s2.powi(3)
                                                + -153. * param.s2.powi(4))
                                        - param.s12.powi(2)
                                            * (33. * param.s1.powi(5)
                                                + -430. * param.s1.powi(4) * param.s2
                                                + 1752. * param.s1.powi(3) * param.s2.powi(2)
                                                + 4722. * param.s1.powi(2) * param.s2.powi(3)
                                                + -7265. * param.s1 * param.s2.powi(4)
                                                + 1188. * param.s2.powi(5))
                                        - param.s12.powi(6)
                                            * (47. * param.s1 + 182. * param.s2))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (23. * param.s1.powi(3)
                                        + -88. * param.s1.powi(2) * param.s2
                                        + 115. * param.s1 * param.s2.powi(2)
                                        + 310. * param.s2.powi(3))
                                - param.s12.powi(5)
                                    * (357. * param.s1.powi(3)
                                        + 884. * param.s1.powi(2) * param.s2
                                        + 1955. * param.s1 * param.s2.powi(2)
                                        + -84. * param.s2.powi(3))
                                - param.s12.powi(7) * (65. * param.s1 + 128. * param.s2))
                        + param.m1_2.powi(3)
                            * param.s1.powi(2)
                            * (18. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (994. * param.s1.powi(2)
                                        + 1781. * param.s1 * param.s2
                                        + 973. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (742. * param.s1.powi(4)
                                        + 821. * param.s1.powi(3) * param.s2
                                        + 263. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2081. * param.s1 * param.s2.powi(3)
                                        + 973. * param.s2.powi(4))
                                + (param.s1 - param.s2).powi(3)
                                    * (88. * param.s1.powi(5)
                                        + -933. * param.s1.powi(4) * param.s2
                                        + 6146. * param.s1.powi(3) * param.s2.powi(2)
                                        + 25690. * param.s1.powi(2) * param.s2.powi(3)
                                        + 3222. * param.s1 * param.s2.powi(4)
                                        + -193. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (1974. * param.s1.powi(6)
                                        + -7697. * param.s1.powi(5) * param.s2
                                        + -9305. * param.s1.powi(4) * param.s2.powi(2)
                                        + -54378. * param.s1.powi(3) * param.s2.powi(3)
                                        + 104032. * param.s1.powi(2) * param.s2.powi(4)
                                        + -14081. * param.s1 * param.s2.powi(5)
                                        + 3423. * param.s2.powi(6))
                                + 42.
                                    * param.m2_2.powi(2)
                                    * (18. * param.s1.powi(6)
                                        + 3. * param.s12.powi(6)
                                        + 813. * param.s1.powi(5) * param.s2
                                        + 2011. * param.s1.powi(4) * param.s2.powi(2)
                                        + -1214. * param.s1.powi(3) * param.s2.powi(3)
                                        + -1544. * param.s1.powi(2) * param.s2.powi(4)
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
                                        + -30.
                                            * param.s12.powi(3)
                                            * (7. * param.s1.powi(3)
                                                + 28. * param.s1.powi(2) * param.s2
                                                + -5. * param.s1 * param.s2.powi(2)
                                                + 2. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (195. * param.s1.powi(4)
                                                + 2430. * param.s1.powi(3) * param.s2
                                                + -224. * param.s1.powi(2) * param.s2.powi(2)
                                                + -390. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        - param.s12
                                            * (93. * param.s1.powi(5)
                                                + 2430. * param.s1.powi(4) * param.s2
                                                + 1982. * param.s1.powi(3) * param.s2.powi(2)
                                                + -2488. * param.s1.powi(2) * param.s2.powi(3)
                                                + -315. * param.s1 * param.s2.powi(4)
                                                + 18. * param.s2.powi(5)))
                                + 7. * param.m2_2
                                    * (7. * param.s12.powi(7)
                                        + -85. * param.s12.powi(6) * (param.s1 + param.s2)
                                        + 11.
                                            * param.s12.powi(5)
                                            * (33. * param.s1.powi(2)
                                                + 74. * param.s1 * param.s2
                                                + 33. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(4)
                                            * (157. * param.s1.powi(3)
                                                + 271. * param.s1.powi(2) * param.s2
                                                + 271. * param.s1 * param.s2.powi(2)
                                                + 157. * param.s2.powi(3))
                                        + 5. * param.s12.powi(3)
                                            * (193. * param.s1.powi(4)
                                                + -204. * param.s1.powi(3) * param.s2
                                                + 3006. * param.s1.powi(2) * param.s2.powi(2)
                                                + -204. * param.s1 * param.s2.powi(3)
                                                + 193. * param.s2.powi(4))
                                        + param.s12
                                            * (265. * param.s1.powi(6)
                                                + -3730. * param.s1.powi(5) * param.s2
                                                + -8881. * param.s1.powi(4) * param.s2.powi(2)
                                                + 41972. * param.s1.powi(3) * param.s2.powi(3)
                                                + -8881. * param.s1.powi(2) * param.s2.powi(4)
                                                + -3730. * param.s1 * param.s2.powi(5)
                                                + 265. * param.s2.powi(6))
                                        - param.s12.powi(2)
                                            * (687. * param.s1.powi(5)
                                                + -4325. * param.s1.powi(4) * param.s2
                                                + 16302. * param.s1.powi(3) * param.s2.powi(2)
                                                + 16302. * param.s1.powi(2) * param.s2.powi(3)
                                                + -4325. * param.s1 * param.s2.powi(4)
                                                + 687. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(2)
                                            * (43. * param.s1.powi(5)
                                                + -965. * param.s1.powi(4) * param.s2
                                                + -13118.
                                                    * param.s1.powi(3)
                                                    * param.s2.powi(2)
                                                + -13118.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(3)
                                                + -965. * param.s1 * param.s2.powi(4)
                                                + 43. * param.s2.powi(5)))
                                - param.s12
                                    * (634. * param.s1.powi(7)
                                        + -5169. * param.s1.powi(6) * param.s2
                                        + 13978. * param.s1.powi(5) * param.s2.powi(2)
                                        + -96971. * param.s1.powi(4) * param.s2.powi(3)
                                        + 68026. * param.s1.powi(3) * param.s2.powi(4)
                                        + 31513. * param.s1.powi(2) * param.s2.powi(5)
                                        + -13254. * param.s1 * param.s2.powi(6)
                                        + 1243. * param.s2.powi(7))
                                - param.s12.powi(3)
                                    * (3458. * param.s1.powi(5)
                                        + -2845. * param.s1.powi(4) * param.s2
                                        + -18720. * param.s1.powi(3) * param.s2.powi(2)
                                        + 54570. * param.s1.powi(2) * param.s2.powi(3)
                                        + 410. * param.s1 * param.s2.powi(4)
                                        + 5243. * param.s2.powi(5))
                                - param.s12.powi(5)
                                    * (2478. * param.s1.powi(3)
                                        + 4813. * param.s1.powi(2) * param.s2
                                        + 6934. * param.s1 * param.s2.powi(2)
                                        + 2793. * param.s2.powi(3))
                                - param.s12.powi(7) * (214. * param.s1 + 193. * param.s2))
                        - param.m1_2.powi(2)
                            * param.s1.powi(3)
                            * (-18. * param.s12.powi(8)
                                + param.s12.powi(7) * (74. * param.s1 + 207. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (238. * param.s1.powi(4)
                                        + 901. * param.s1.powi(3) * param.s2
                                        + 2278. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1983. * param.s1 * param.s2.powi(3)
                                        + -630. * param.s2.powi(4))
                                + (param.s1 - param.s2).powi(4)
                                    * (52. * param.s1.powi(4)
                                        + -429. * param.s1.powi(3) * param.s2
                                        + 1986. * param.s1.powi(2) * param.s2.powi(2)
                                        + 5759. * param.s1 * param.s2.powi(3)
                                        + 360. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (-1442. * param.s1.powi(5)
                                        + -2355. * param.s1.powi(4) * param.s2
                                        + 6060. * param.s1.powi(3) * param.s2.powi(2)
                                        + -90190. * param.s1.powi(2) * param.s2.powi(3)
                                        + 57810. * param.s1 * param.s2.powi(4)
                                        + 693. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (966. * param.s1.powi(6)
                                        + -1697. * param.s1.powi(5) * param.s2
                                        + -14670. * param.s1.powi(4) * param.s2.powi(2)
                                        + 45978. * param.s1.powi(3) * param.s2.powi(3)
                                        + 60482. * param.s1.powi(2) * param.s2.powi(4)
                                        + -59769. * param.s1 * param.s2.powi(5)
                                        + 1638. * param.s2.powi(6))
                                + 70.
                                    * param.m2_2.powi(3)
                                    * (113. * param.s1.powi(5)
                                        + 9. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (77. * param.s1 + -45. * param.s2)
                                        + 967. * param.s1.powi(4) * param.s2
                                        + 522. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1166. * param.s1.powi(2) * param.s2.powi(3)
                                        + -427. * param.s1 * param.s2.powi(4)
                                        + -9. * param.s2.powi(5)
                                        + param.s12.powi(3)
                                            * (-398. * param.s1.powi(2)
                                                + 196. * param.s1 * param.s2
                                                + 90. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (321. * param.s1.powi(3)
                                                + 355. * param.s1.powi(2) * param.s2
                                                + -525. * param.s1 * param.s2.powi(2)
                                                + -45. * param.s2.powi(3))
                                        + param.s12
                                            * (-443. * param.s1.powi(4)
                                                + -1828. * param.s1.powi(3) * param.s2
                                                + 854. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1204. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4)))
                                + -21.
                                    * param.m2_2.powi(2)
                                    * (9. * param.s12.powi(6)
                                        + 36. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + -35.
                                            * param.s12.powi(4)
                                            * (9. * param.s1.powi(2)
                                                + -58. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + 80.
                                            * param.s12.powi(3)
                                            * (9. * param.s1.powi(3)
                                                + -43. * param.s1.powi(2) * param.s2
                                                + -43. * param.s1 * param.s2.powi(2)
                                                + 9. * param.s2.powi(3))
                                        + 4. * param.s12
                                            * (99. * param.s1.powi(5)
                                                + 1615. * param.s1.powi(4) * param.s2
                                                + -2794. * param.s1.powi(3) * param.s2.powi(2)
                                                + -2794. * param.s1.powi(2) * param.s2.powi(3)
                                                + 1615. * param.s1 * param.s2.powi(4)
                                                + 99. * param.s2.powi(5))
                                        - param.s12.powi(2)
                                            * (765. * param.s1.powi(4)
                                                + 2220. * param.s1.powi(3) * param.s2
                                                + -16778.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(2)
                                                + 2220. * param.s1 * param.s2.powi(3)
                                                + 765. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(2)
                                            * (81. * param.s1.powi(4)
                                                + 3028. * param.s1.powi(3) * param.s2
                                                + 7822. * param.s1.powi(2) * param.s2.powi(2)
                                                + 3028. * param.s1 * param.s2.powi(3)
                                                + 81. * param.s2.powi(4)))
                                + -21.
                                    * param.m2_2
                                    * (3. * param.s12.powi(7)
                                        + -3. * param.s12.powi(6) * (param.s1 + 13. * param.s2)
                                        + -3.
                                            * param.s12.powi(5)
                                            * (15. * param.s1.powi(2)
                                                + 74. * param.s1 * param.s2
                                                + -27. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (33. * param.s1.powi(3)
                                                + 195. * param.s1.powi(2) * param.s2
                                                + -499. * param.s1 * param.s2.powi(2)
                                                + 15. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(3)
                                            * (15. * param.s1.powi(4)
                                                + -258. * param.s1.powi(3) * param.s2
                                                + -2714. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1834. * param.s1 * param.s2.powi(3)
                                                + -69. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (51. * param.s1.powi(4)
                                                + 180. * param.s1.powi(3) * param.s2
                                                + 650. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1444. * param.s1 * param.s2.powi(3)
                                                + 87. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (207. * param.s1.powi(5)
                                                + -345. * param.s1.powi(4) * param.s2
                                                + 11766. * param.s1.powi(3) * param.s2.powi(2)
                                                + -9626. * param.s1.powi(2) * param.s2.powi(3)
                                                + -4425. * param.s1 * param.s2.powi(4)
                                                + 567. * param.s2.powi(5))
                                        - param.s12
                                            * (87. * param.s1.powi(6)
                                                + -834. * param.s1.powi(5) * param.s2
                                                + 4207. * param.s1.powi(4) * param.s2.powi(2)
                                                + 9044. * param.s1.powi(3) * param.s2.powi(3)
                                                + -14527.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(4)
                                                + 1702. * param.s1 * param.s2.powi(5)
                                                + 321. * param.s2.powi(6)))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (346. * param.s1.powi(5)
                                        + -1517. * param.s1.powi(4) * param.s2
                                        + -3204. * param.s1.powi(3) * param.s2.powi(2)
                                        + -61150. * param.s1.powi(2) * param.s2.powi(3)
                                        + -10162. * param.s1 * param.s2.powi(4)
                                        + 1431. * param.s2.powi(5))
                                - param.s12.powi(5)
                                    * (462. * param.s1.powi(3)
                                        + 2173. * param.s1.powi(2) * param.s2
                                        + 5484. * param.s1 * param.s2.powi(2)
                                        + -2835. * param.s2.powi(3))
                                - param.s12.powi(6)
                                    * (14. * param.s1.powi(2)
                                        + 59. * param.s1 * param.s2
                                        + 1134. * param.s2.powi(2)))
                        - param.m1_2.powi(4)
                            * param.s1
                            * (27. * param.s12.powi(8)
                                + -251. * param.s12.powi(7) * (param.s1 + param.s2)
                                + param.s12.powi(6)
                                    * (1001. * param.s1.powi(2)
                                        + 1492. * param.s1 * param.s2
                                        + 1001. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (623. * param.s1.powi(4)
                                        + 52. * param.s1.powi(3) * param.s2
                                        + -680. * param.s1.powi(2) * param.s2.powi(2)
                                        + 52. * param.s1 * param.s2.powi(3)
                                        + 623. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (-2737. * param.s1.powi(5)
                                        + 5615. * param.s1.powi(4) * param.s2
                                        + 8340. * param.s1.powi(3) * param.s2.powi(2)
                                        + 8340. * param.s1.powi(2) * param.s2.powi(3)
                                        + 5615. * param.s1 * param.s2.powi(4)
                                        + -2737. * param.s2.powi(5))
                                + 2. * (param.s1 - param.s2).powi(2)
                                    * (31. * param.s1.powi(6)
                                        + -400. * param.s1.powi(5) * param.s2
                                        + 3407. * param.s1.powi(4) * param.s2.powi(2)
                                        + 18494. * param.s1.powi(3) * param.s2.powi(3)
                                        + 3407. * param.s1.powi(2) * param.s2.powi(4)
                                        + -400. * param.s1 * param.s2.powi(5)
                                        + 31. * param.s2.powi(6))
                                + param.s12.powi(2)
                                    * (1491. * param.s1.powi(6)
                                        + -7804. * param.s1.powi(5) * param.s2
                                        + 8075. * param.s1.powi(4) * param.s2.powi(2)
                                        + -59832. * param.s1.powi(3) * param.s2.powi(3)
                                        + 8075. * param.s1.powi(2) * param.s2.powi(4)
                                        + -7804. * param.s1 * param.s2.powi(5)
                                        + 1491. * param.s2.powi(6))
                                + param.s12
                                    * (-461. * param.s1.powi(7)
                                        + 4383. * param.s1.powi(6) * param.s2
                                        + -19721. * param.s1.powi(5) * param.s2.powi(2)
                                        + 30919. * param.s1.powi(4) * param.s2.powi(3)
                                        + 30919. * param.s1.powi(3) * param.s2.powi(4)
                                        + -19721. * param.s1.powi(2) * param.s2.powi(5)
                                        + 4383. * param.s1 * param.s2.powi(6)
                                        + -461. * param.s2.powi(7))
                                + 7. * param.m2_2
                                    * (-14. * param.s1.powi(7)
                                        + 5. * param.s12.powi(7)
                                        + 401. * param.s1.powi(6) * param.s2
                                        + 6072. * param.s1.powi(5) * param.s2.powi(2)
                                        + 2067. * param.s1.powi(4) * param.s2.powi(3)
                                        + -7608. * param.s1.powi(3) * param.s2.powi(4)
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
                                                + 6882. * param.s1.powi(3) * param.s2.powi(2)
                                                + -1080. * param.s1.powi(2) * param.s2.powi(3)
                                                + 640. * param.s1 * param.s2.powi(4)
                                                + -105. * param.s2.powi(5))
                                        + param.s12
                                            * (89. * param.s1.powi(6)
                                                + -1610. * param.s1.powi(5) * param.s2
                                                + -12449.
                                                    * param.s1.powi(4)
                                                    * param.s2.powi(2)
                                                + 2536. * param.s1.powi(3) * param.s2.powi(3)
                                                + 2305. * param.s1.powi(2) * param.s2.powi(4)
                                                + -386. * param.s1 * param.s2.powi(5)
                                                + 35. * param.s2.powi(6))
                                        - param.s12.powi(6) * (44. * param.s1 + 35. * param.s2))
                                - param.s12.powi(5)
                                    * (2247. * param.s1.powi(3)
                                        + 2771. * param.s1.powi(2) * param.s2
                                        + 2771. * param.s1 * param.s2.powi(2)
                                        + 2247. * param.s2.powi(3)))
                        - param.m1_2
                            * param.s1.powi(4)
                            * (27. * param.s12.powi(8)
                                + param.s12.powi(6)
                                    * (511. * param.s1.powi(2)
                                        + 1268. * param.s1 * param.s2
                                        + 2443. * param.s2.powi(2))
                                + -2.
                                    * (param.s1 - param.s2).powi(5)
                                    * (4. * param.s1.powi(3)
                                        + -22. * param.s1.powi(2) * param.s2
                                        + 50. * param.s1 * param.s2.powi(2)
                                        + 73. * param.s2.powi(3))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (29. * param.s1.powi(4)
                                        + 144. * param.s1.powi(3) * param.s2
                                        + -2338. * param.s1.powi(2) * param.s2.powi(2)
                                        + -12618. * param.s1 * param.s2.powi(3)
                                        + -2437. * param.s2.powi(4))
                                + 5. * param.s12.powi(4)
                                    * (133. * param.s1.powi(4)
                                        + -172. * param.s1.powi(3) * param.s2
                                        + -1492. * param.s1.powi(2) * param.s2.powi(2)
                                        + 6016. * param.s1 * param.s2.powi(3)
                                        + -609. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (-287. * param.s1.powi(5)
                                        + 2185. * param.s1.powi(4) * param.s2
                                        + 4980. * param.s1.powi(3) * param.s2.powi(2)
                                        + 21710. * param.s1.powi(2) * param.s2.powi(3)
                                        + -56965. * param.s1 * param.s2.powi(4)
                                        + 12033. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (21. * param.s1.powi(6)
                                        + -1196. * param.s1.powi(5) * param.s2
                                        + 4225. * param.s1.powi(4) * param.s2.powi(2)
                                        + -43662. * param.s1.powi(3) * param.s2.powi(3)
                                        + 28277. * param.s1.powi(2) * param.s2.powi(4)
                                        + 22226. * param.s1 * param.s2.powi(5)
                                        + -9891. * param.s2.powi(6))
                                + 70.
                                    * param.m2_2.powi(4)
                                    * (-194. * param.s1.powi(4)
                                        + 61. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (11. * param.s1 + -244. * param.s2)
                                        + -607. * param.s1.powi(3) * param.s2
                                        + 237. * param.s1.powi(2) * param.s2.powi(2)
                                        + 503. * param.s1 * param.s2.powi(3)
                                        + 61. * param.s2.powi(4)
                                        + param.s12.powi(2)
                                            * (-399. * param.s1.powi(2)
                                                + 481. * param.s1 * param.s2
                                                + 366. * param.s2.powi(2))
                                        + param.s12
                                            * (521. * param.s1.powi(3)
                                                + 370. * param.s1.powi(2) * param.s2
                                                + -995. * param.s1 * param.s2.powi(2)
                                                + -244. * param.s2.powi(3)))
                                + -280.
                                    * param.m2_2.powi(3)
                                    * (18. * param.s12.powi(5)
                                        + -29. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (-64. * param.s1.powi(2)
                                                + 379. * param.s1 * param.s2
                                                + -64. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (43. * param.s1.powi(3)
                                                + 308. * param.s1.powi(2) * param.s2
                                                + 308. * param.s1 * param.s2.powi(2)
                                                + 43. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (186. * param.s1.powi(3)
                                                + -449. * param.s1.powi(2) * param.s2
                                                + -449. * param.s1 * param.s2.powi(2)
                                                + 186. * param.s2.powi(3))
                                        - param.s12
                                            * (154. * param.s1.powi(4)
                                                + 123. * param.s1.powi(3) * param.s2
                                                + -986. * param.s1.powi(2) * param.s2.powi(2)
                                                + 123. * param.s1 * param.s2.powi(3)
                                                + 154. * param.s2.powi(4)))
                                + 42.
                                    * param.m2_2.powi(2)
                                    * (18. * param.s12.powi(6)
                                        + -63.
                                            * param.s12.powi(5)
                                            * (param.s1 + -4. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (9. * param.s1.powi(2)
                                                + 163. * param.s1 * param.s2
                                                + -184. * param.s2.powi(2))
                                        + 10.
                                            * param.s12.powi(3)
                                            * (9. * param.s1.powi(3)
                                                + -326. * param.s1.powi(2) * param.s2
                                                + 252. * param.s1 * param.s2.powi(2)
                                                + 80. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-180. * param.s1.powi(4)
                                                + 2370. * param.s1.powi(3) * param.s2
                                                + 4016. * param.s1.powi(2) * param.s2.powi(2)
                                                + -6680. * param.s1 * param.s2.powi(3)
                                                + 330. * param.s2.powi(4))
                                        + param.s12
                                            * (117. * param.s1.powi(5)
                                                + 520. * param.s1.powi(4) * param.s2
                                                + -6512. * param.s1.powi(3) * param.s2.powi(2)
                                                + 3768. * param.s1.powi(2) * param.s2.powi(3)
                                                + 2855. * param.s1 * param.s2.powi(4)
                                                + -748. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(3)
                                            * (27. * param.s1.powi(3)
                                                + 778. * param.s1.powi(2) * param.s2
                                                + 1357. * param.s1 * param.s2.powi(2)
                                                + 268. * param.s2.powi(3)))
                                + 7. * param.m2_2
                                    * (19. * param.s12.powi(7)
                                        + param.s12.powi(5)
                                            * (183. * param.s1.powi(2)
                                                + 574. * param.s1 * param.s2
                                                + -465. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(4)
                                            * (25. * param.s1.powi(3)
                                                + -185. * param.s1.powi(2) * param.s2
                                                + 2179. * param.s1 * param.s2.powi(2)
                                                + -891. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(4)
                                            * (17. * param.s1.powi(3)
                                                + -201. * param.s1.powi(2) * param.s2
                                                + -1509. * param.s1 * param.s2.powi(2)
                                                + -515. * param.s2.powi(3))
                                        + -5.
                                            * param.s12.powi(3)
                                            * (11. * param.s1.powi(4)
                                                + 492. * param.s1.powi(3) * param.s2
                                                + -2358. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1820. * param.s1 * param.s2.powi(3)
                                                + 1371. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (141. * param.s1.powi(5)
                                                + 1325. * param.s1.powi(4) * param.s2
                                                + 10362. * param.s1.powi(3) * param.s2.powi(2)
                                                + -37782.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(3)
                                                + 13225. * param.s1 * param.s2.powi(4)
                                                + 3321. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (83. * param.s1.powi(4)
                                                + -88. * param.s1.powi(3) * param.s2
                                                + 9930. * param.s1.powi(2) * param.s2.powi(2)
                                                + 11680. * param.s1 * param.s2.powi(3)
                                                + -389. * param.s2.powi(4))
                                        - param.s12.powi(6)
                                            * (97. * param.s1 + 349. * param.s2))
                                - param.s12.powi(5)
                                    * (777. * param.s1.powi(3)
                                        + 1189. * param.s1.powi(2) * param.s2
                                        + 1105. * param.s1 * param.s2.powi(2)
                                        + 3801. * param.s2.powi(3))
                                - param.s12.powi(7) * (181. * param.s1 + 349. * param.s2))))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 420.
                    * param.s1.powi(4)
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
                    * (-18.
                        * param.m1_2.powi(3)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(3)
                            * (5. * param.s12.powi(3)
                                + 3. * (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-7. * param.s1 + 5. * param.s2)
                                - param.s12
                                    * (param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2)))
                        + 2. * param.m1_2.powi(2)
                            * param.s2
                            * (param.s2
                                * (-17. * param.s1.powi(2)
                                    + 7. * param.s1 * param.s12
                                    + 10. * param.s12.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + -20. * param.s12 * param.s2
                                    + 10. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (5. * param.s1.powi(2)
                                        + 5. * param.s12.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2)
                                        + -10. * param.s12 * (param.s1 + param.s2)))
                        + param.s1
                            * (-4.
                                * param.m2_2.powi(3)
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + 2. * param.s2.powi(2)
                                    + -4. * param.s12 * (param.s1 + param.s2))
                                + param.m2_2.powi(2)
                                    * (-5. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + -25. * param.s1.powi(2) * param.s2
                                        + 11. * param.s1 * param.s2.powi(2)
                                        + 19. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-15. * param.s1 + 9. * param.s2)
                                        + param.s12
                                            * (15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -33. * param.s2.powi(2)))
                                + -2.
                                    * param.m2_2
                                    * param.s2
                                    * (5. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1 + 7. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        - param.s12.powi(2) * (5. * param.s1 + 3. * param.s2))
                                + param.s2.powi(2)
                                    * (5. * param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -7. * param.s2)
                                        + -3. * (param.s1 - param.s2).powi(3)
                                        - param.s12
                                            * (7. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + param.s2.powi(2))))
                        + param.m0_2.powi(2)
                            * (-3. * param.s1.powi(4)
                                + 7. * param.s1.powi(3) * param.s12
                                + -3. * param.s1.powi(2) * param.s12.powi(2)
                                + -3. * param.s1 * param.s12.powi(3)
                                + 2. * param.s12.powi(4)
                                + 3. * param.s1.powi(3) * param.s2
                                + -32. * param.s1.powi(2) * param.s12 * param.s2
                                + 21. * param.s1 * param.s12.powi(2) * param.s2
                                + 8. * param.s12.powi(3) * param.s2
                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                + 25. * param.s1 * param.s12 * param.s2.powi(2)
                                + -16. * param.s12.powi(2) * param.s2.powi(2)
                                + -15. * param.s1 * param.s2.powi(3)
                                + 6. * param.s2.powi(4)
                                + -2.
                                    * param.m2_2
                                    * (5. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2)
                                            * (7. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (-9. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s12.powi(2) * (3. * param.s1 + 5. * param.s2))
                                + param.m1_2
                                    * (-5. * param.s12.powi(3)
                                        + 5. * param.s12.powi(2)
                                            * (3. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1 + 19. * param.s2)
                                        + param.s12
                                            * (-15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 11. * param.s2.powi(2))))
                        + param.m1_2
                            * (param.s2.powi(2)
                                * (-5. * param.s12.powi(3)
                                    + -5. * param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (19. * param.s1 + 5. * param.s2)
                                    + param.s12
                                        * (11. * param.s1.powi(2)
                                            + 16. * param.s1 * param.s2
                                            + -15. * param.s2.powi(2)))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (15. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 5. * param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 19. * param.s1.powi(2) * param.s2
                                        + -29. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-25. * param.s1.powi(2)
                                                + 24. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + param.m2_2.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -5. * param.s12.powi(3)
                                        + 49. * param.s1.powi(2) * param.s2
                                        + 49. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)
                                        + 15. * param.s12.powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (15. * param.s1.powi(2)
                                                + 64. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2))))
                        + param.m0_2
                            * (2.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (10. * param.s1.powi(2)
                                    + -20. * param.s1 * param.s12
                                    + 10. * param.s12.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + 7. * param.s12 * param.s2
                                    + -17. * param.s2.powi(2))
                                + param.s2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (8. * param.s1 + -3. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-16. * param.s1.powi(2)
                                                + 21. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + param.s12
                                            * param.s2
                                            * (25. * param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + 7. * param.s2.powi(2)))
                                + param.m2_2.powi(2)
                                    * (19. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (3. * param.s1 + -5. * param.s2)
                                        + 11. * param.s1.powi(2) * param.s2
                                        + -25. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-33. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + -2.
                                    * param.m2_2
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (param.s1 + param.s2)
                                        + -4.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (11. * param.s1.powi(3)
                                                + -17. * param.s1.powi(2) * param.s2
                                                + -17. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3)))
                                + 2. * param.m1_2
                                    * (param.m2_2
                                        * (-5. * param.s1.powi(3)
                                            + 5. * param.s12.powi(3)
                                            + -29. * param.s1.powi(2) * param.s2
                                            + 19. * param.s1 * param.s2.powi(2)
                                            + 15. * param.s2.powi(3)
                                            + 5. * param.s12.powi(2)
                                                * (-3. * param.s1 + param.s2)
                                            + param.s12
                                                * (15. * param.s1.powi(2)
                                                    + 24. * param.s1 * param.s2
                                                    + -25. * param.s2.powi(2)))
                                        + -4.
                                            * param.s2
                                            * (2. * param.s12.powi(3)
                                                + 3. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (-4. * param.s1.powi(2)
                                                        + 11. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2)))))
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.0000992063492063492
            * std::f64::consts::PI
            * param.s2.powi(-3)
            * param.lambda_s12_sqrt.powi(-11)
            * ((param.m0_2.powi(6)
                * (-2. * param.s12.powi(7)
                    + (2. * param.s1 + -5. * param.s2) * (param.s1 - param.s2).powi(6)
                    + 7. * param.s12.powi(6) * (2. * param.s1 + 5. * param.s2)
                    + -2.
                        * param.s12
                        * (param.s1 - param.s2).powi(4)
                        * (7. * param.s1.powi(2)
                            + 3. * param.s1 * param.s2
                            + -70. * param.s2.powi(2))
                    + -2.
                        * param.s12.powi(5)
                        * (21. * param.s1.powi(2)
                            + 79. * param.s1 * param.s2
                            + 210. * param.s2.powi(2))
                    + 5. * param.s12.powi(4)
                        * (14. * param.s1.powi(3)
                            + 53. * param.s1.powi(2) * param.s2
                            + 200. * param.s1 * param.s2.powi(2)
                            + -735. * param.s2.powi(3))
                    + param.s12.powi(2)
                        * (param.s1 - param.s2).powi(2)
                        * (42. * param.s1.powi(3)
                            + 89. * param.s1.powi(2) * param.s2
                            + -44. * param.s1 * param.s2.powi(2)
                            + 2877. * param.s2.powi(3))
                    + -2.
                        * param.s12.powi(3)
                        * (35. * param.s1.powi(4)
                            + 90. * param.s1.powi(3) * param.s2
                            + 270. * param.s1.powi(2) * param.s2.powi(2)
                            + -638. * param.s1 * param.s2.powi(3)
                            + -525. * param.s2.powi(4)))
                + -2.
                    * param.m2_2.powi(6)
                    * (param.s12.powi(7)
                        + 19. * param.s1.powi(6) * param.s2
                        + -261. * param.s1.powi(5) * param.s2.powi(2)
                        + -3537. * param.s1.powi(4) * param.s2.powi(3)
                        + -3537. * param.s1.powi(3) * param.s2.powi(4)
                        + -261. * param.s1.powi(2) * param.s2.powi(5)
                        + 19. * param.s1 * param.s2.powi(6)
                        + -7. * param.s12.powi(6) * (param.s1 + param.s2)
                        + param.s12.powi(5)
                            * (21. * param.s1.powi(2)
                                + 16. * param.s1 * param.s2
                                + 21. * param.s2.powi(2))
                        + -5.
                            * param.s12.powi(4)
                            * (7. * param.s1.powi(3)
                                + -5. * param.s1.powi(2) * param.s2
                                + -5. * param.s1 * param.s2.powi(2)
                                + 7. * param.s2.powi(3))
                        + 5. * param.s12.powi(3)
                            * (7. * param.s1.powi(4)
                                + -24. * param.s1.powi(3) * param.s2
                                + 12. * param.s1.powi(2) * param.s2.powi(2)
                                + -24. * param.s1 * param.s2.powi(3)
                                + 7. * param.s2.powi(4))
                        + param.s12
                            * (7. * param.s1.powi(6)
                                + -88. * param.s1.powi(5) * param.s2
                                + 695. * param.s1.powi(4) * param.s2.powi(2)
                                + 4232. * param.s1.powi(3) * param.s2.powi(3)
                                + 695. * param.s1.powi(2) * param.s2.powi(4)
                                + -88. * param.s1 * param.s2.powi(5)
                                + 7. * param.s2.powi(6))
                        - param.s12.powi(2)
                            * (21. * param.s1.powi(5)
                                + -155. * param.s1.powi(4) * param.s2
                                + 540. * param.s1.powi(3) * param.s2.powi(2)
                                + 540. * param.s1.powi(2) * param.s2.powi(3)
                                + -155. * param.s1 * param.s2.powi(4)
                                + 21. * param.s2.powi(5))
                        - param.s2.powi(7)
                        - param.s1.powi(7))
                + param.s2.powi(6)
                    * (-2. * param.s12.powi(7)
                        + -7560. * param.m1_2.powi(6) * (param.s12 - param.s2 - param.s1)
                        + 7. * param.s12.powi(6) * (5. * param.s1 + 2. * param.s2)
                        + 2. * param.s12
                            * (param.s1 - param.s2).powi(4)
                            * (70. * param.s1.powi(2)
                                + -3. * param.s1 * param.s2
                                + -7. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(5)
                            * (210. * param.s1.powi(2)
                                + 79. * param.s1 * param.s2
                                + 21. * param.s2.powi(2))
                        + 420.
                            * param.m1_2.powi(5)
                            * (-61. * param.s1.powi(2)
                                + 14. * param.s1 * param.s12
                                + 47. * param.s12.powi(2)
                                + 14. * param.s1 * param.s2
                                + -94. * param.s12 * param.s2
                                + 47. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (-735. * param.s1.powi(3)
                                + 200. * param.s1.powi(2) * param.s2
                                + 53. * param.s1 * param.s2.powi(2)
                                + 14. * param.s2.powi(3))
                        + param.s12.powi(2)
                            * (param.s1 - param.s2).powi(2)
                            * (2877. * param.s1.powi(3)
                                + -44. * param.s1.powi(2) * param.s2
                                + 89. * param.s1 * param.s2.powi(2)
                                + 42. * param.s2.powi(3))
                        + 2. * param.s12.powi(3)
                            * (525. * param.s1.powi(4)
                                + 638. * param.s1.powi(3) * param.s2
                                + -270. * param.s1.powi(2) * param.s2.powi(2)
                                + -90. * param.s1 * param.s2.powi(3)
                                + -35. * param.s2.powi(4))
                        + -420.
                            * param.m1_2.powi(4)
                            * (41. * param.s12.powi(3)
                                + param.s12.powi(2) * (112. * param.s1 + -123. * param.s2)
                                + param.s12
                                    * (-77. * param.s1.powi(2)
                                        + -118. * param.s1 * param.s2
                                        + 123. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(2)
                                    * (76. * param.s1 + 41. * param.s2))
                        + -84.
                            * param.m1_2.powi(2)
                            * (3. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (35. * param.s1 + -3. * param.s2)
                                + param.s12.powi(3)
                                    * (500. * param.s1.powi(2)
                                        + -494. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (115. * param.s1.powi(3)
                                        + 184. * param.s1.powi(2) * param.s2
                                        + -216. * param.s1 * param.s2.powi(2)
                                        + 15. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (405. * param.s1.powi(2)
                                        + 52. * param.s1 * param.s2
                                        + -15. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(4)
                                    * (43. * param.s1 + 3. * param.s2))
                        + 280.
                            * param.m1_2.powi(3)
                            * (19. * param.s12.powi(4)
                                + 2. * param.s12.powi(3) * (85. * param.s1 + -38. * param.s2)
                                + param.s12.powi(2)
                                    * (81. * param.s1.powi(2)
                                        + -335. * param.s1 * param.s2
                                        + 114. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (52. * param.s1.powi(3)
                                        + -31. * param.s1.powi(2) * param.s2
                                        + -40. * param.s1 * param.s2.powi(2)
                                        + 19. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3)
                                    * (62. * param.s1 + 19. * param.s2))
                        + -21.
                            * param.m1_2
                            * (param.s12.powi(6)
                                + (param.s1 - param.s2).powi(5) * (5. * param.s1 - param.s2)
                                + -6. * param.s12.powi(5) * (5. * param.s1 + param.s2)
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (157. * param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-625. * param.s1.powi(2)
                                        + 110. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (125. * param.s1.powi(3)
                                        + -253. * param.s1.powi(2) * param.s2
                                        + 35. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (835. * param.s1.powi(4)
                                        + -796. * param.s1.powi(3) * param.s2
                                        + -114. * param.s1.powi(2) * param.s2.powi(2)
                                        + 60. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4)))
                        - (5. * param.s1 + -2. * param.s2) * (param.s1 - param.s2).powi(6))
                + -3.
                    * param.m2_2
                    * param.s2.powi(5)
                    * (-4. * param.s12.powi(7)
                        + 7. * param.s12.powi(6) * (9. * param.s1 + 4. * param.s2)
                        + -2.
                            * param.s12.powi(5)
                            * (315. * param.s1.powi(2)
                                + 137. * param.s1 * param.s2
                                + 42. * param.s2.powi(2))
                        + -5.
                            * param.s12.powi(4)
                            * (595. * param.s1.powi(3)
                                + -246. * param.s1.powi(2) * param.s2
                                + -85. * param.s1 * param.s2.powi(2)
                                + -28. * param.s2.powi(3))
                        + -2.
                            * param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (959. * param.s1.powi(3)
                                + 174. * param.s1.powi(2) * param.s2
                                + 29. * param.s1 * param.s2.powi(2)
                                + -14. * param.s2.powi(3))
                        + 4. * param.s12.powi(3)
                            * (1400. * param.s1.powi(4)
                                + -1133. * param.s1.powi(3) * param.s2
                                + -25. * param.s1.powi(2) * param.s2.powi(2)
                                + -55. * param.s1 * param.s2.powi(3)
                                + -35. * param.s2.powi(4))
                        + 140.
                            * param.m1_2.powi(5)
                            * (47. * param.s1.powi(2)
                                + 47. * param.s12.powi(2)
                                + 122. * param.s1 * param.s2
                                + 47. * param.s2.powi(2)
                                + -94. * param.s12 * (param.s1 + param.s2))
                        + -140.
                            * param.m1_2.powi(4)
                            * (153. * param.s1.powi(3)
                                + 82. * param.s12.powi(3)
                                + 152. * param.s1.powi(2) * param.s2
                                + -223. * param.s1 * param.s2.powi(2)
                                + -82. * param.s2.powi(3)
                                + param.s12
                                    * (-224. * param.s1.powi(2)
                                        + 234. * param.s1 * param.s2
                                        + 246. * param.s2.powi(2))
                                - param.s12.powi(2) * (11. * param.s1 + 246. * param.s2))
                        + 280.
                            * param.m1_2.powi(3)
                            * (19. * param.s12.powi(4)
                                + param.s12.powi(3) * (88. * param.s1 + -76. * param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (90. * param.s1.powi(2)
                                        + 125. * param.s1 * param.s2
                                        + 19. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-143. * param.s1.powi(2)
                                        + -89. * param.s1 * param.s2
                                        + 114. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (27. * param.s1.powi(3)
                                        + -180. * param.s1.powi(2) * param.s2
                                        + 43. * param.s1 * param.s2.powi(2)
                                        + 38. * param.s2.powi(3)))
                        + -56.
                            * param.m1_2.powi(2)
                            * (6. * param.s12.powi(5)
                                + 15. * param.s12.powi(4) * (17. * param.s1 + -2. * param.s2)
                                + (param.s1 - param.s2).powi(3)
                                    * (224. * param.s1.powi(2)
                                        + 175. * param.s1 * param.s2
                                        + 6. * param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (75. * param.s1.powi(2)
                                        + -304. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-865. * param.s1.powi(3)
                                        + 939. * param.s1.powi(2) * param.s2
                                        + 294. * param.s1 * param.s2.powi(2)
                                        + -60. * param.s2.powi(3))
                                + 2. * param.s12
                                    * (115. * param.s1.powi(4)
                                        + 448. * param.s1.powi(3) * param.s2
                                        + -686. * param.s1.powi(2) * param.s2.powi(2)
                                        + 108. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4)))
                        + -7.
                            * param.m1_2
                            * (5. * param.s12.powi(6)
                                + -6. * param.s12.powi(5) * (21. * param.s1 + 5. * param.s2)
                                + param.s12.powi(4)
                                    * (-1725. * param.s1.powi(2)
                                        + 430. * param.s1 * param.s2
                                        + 75. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (375. * param.s1.powi(3)
                                        + 277. * param.s1.powi(2) * param.s2
                                        + -115. * param.s1 * param.s2.powi(2)
                                        + -25. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (835. * param.s1.powi(3)
                                        + 973. * param.s1.powi(2) * param.s2
                                        + -55. * param.s1 * param.s2.powi(2)
                                        + 15. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (2335. * param.s1.powi(4)
                                        + -6924. * param.s1.powi(3) * param.s2
                                        + 2886. * param.s1.powi(2) * param.s2.powi(2)
                                        + 60. * param.s1 * param.s2.powi(3)
                                        + 75. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(4)
                                    * (319. * param.s1.powi(2)
                                        + 54. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2)))
                        - param.s12.powi(2)
                            * (91. * param.s1.powi(5)
                                + 6024. * param.s1.powi(4) * param.s2
                                + -6906. * param.s1.powi(3) * param.s2.powi(2)
                                + 780. * param.s1.powi(2) * param.s2.powi(3)
                                + 95. * param.s1 * param.s2.powi(4)
                                + -84. * param.s2.powi(5))
                        - (param.s1 - param.s2).powi(5)
                            * (45. * param.s1.powi(2)
                                + -21. * param.s1 * param.s2
                                + 4. * param.s2.powi(2)))
                + -2.
                    * param.m2_2.powi(3)
                    * param.s2.powi(3)
                    * (-20. * param.s12.powi(7)
                        + 35. * param.s12.powi(6) * (7. * param.s1 + 4. * param.s2)
                        + -2.
                            * param.s12.powi(5)
                            * (777. * param.s1.powi(2)
                                + 475. * param.s1 * param.s2
                                + 210. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (427. * param.s1.powi(3)
                                + 194. * param.s1.powi(2) * param.s2
                                + 215. * param.s1 * param.s2.powi(2)
                                + 140. * param.s2.powi(3))
                        + 4. * param.s12.powi(3)
                            * (700. * param.s1.powi(4)
                                + -5287. * param.s1.powi(3) * param.s2
                                + 1065. * param.s1.powi(2) * param.s2.powi(2)
                                + 75. * param.s1 * param.s2.powi(3)
                                + -175. * param.s2.powi(4))
                        + 2. * param.s12
                            * (3675. * param.s1.powi(6)
                                + -8787. * param.s1.powi(5) * param.s2
                                + -9036. * param.s1.powi(4) * param.s2.powi(2)
                                + 15038. * param.s1.powi(3) * param.s2.powi(3)
                                + -1385. * param.s1.powi(2) * param.s2.powi(4)
                                + 565. * param.s1 * param.s2.powi(5)
                                + -70. * param.s2.powi(6))
                        + 140.
                            * param.m1_2.powi(3)
                            * (19. * param.s1.powi(4)
                                + 19. * param.s12.powi(4)
                                + 251. * param.s1.powi(3) * param.s2
                                + 540. * param.s1.powi(2) * param.s2.powi(2)
                                + 251. * param.s1 * param.s2.powi(3)
                                + 19. * param.s2.powi(4)
                                + -76. * param.s12.powi(3) * (param.s1 + param.s2)
                                + param.s12.powi(2)
                                    * (114. * param.s1.powi(2)
                                        + 403. * param.s1 * param.s2
                                        + 114. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (38. * param.s1.powi(3)
                                        + 289. * param.s1.powi(2) * param.s2
                                        + 289. * param.s1 * param.s2.powi(2)
                                        + 38. * param.s2.powi(3)))
                        + -84.
                            * param.m1_2.powi(2)
                            * (89. * param.s1.powi(5)
                                + 6. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (13. * param.s1 + -6. * param.s2)
                                + 813. * param.s1.powi(4) * param.s2
                                + 448. * param.s1.powi(3) * param.s2.powi(2)
                                + -997. * param.s1.powi(2) * param.s2.powi(3)
                                + -347. * param.s1 * param.s2.powi(4)
                                + -6. * param.s2.powi(5)
                                + param.s12.powi(3)
                                    * (-320. * param.s1.powi(2)
                                        + 152. * param.s1 * param.s2
                                        + 60. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (510. * param.s1.powi(3)
                                        + 599. * param.s1.powi(2) * param.s2
                                        + -846. * param.s1 * param.s2.powi(2)
                                        + -60. * param.s2.powi(3))
                                + param.s12
                                    * (-350. * param.s1.powi(4)
                                        + -1534. * param.s1.powi(3) * param.s2
                                        + 718. * param.s1.powi(2) * param.s2.powi(2)
                                        + 976. * param.s1 * param.s2.powi(3)
                                        + 30. * param.s2.powi(4)))
                        + -21.
                            * param.m1_2
                            * (5. * param.s12.powi(6)
                                + -6. * param.s12.powi(5) * (13. * param.s1 + 5. * param.s2)
                                + param.s12.powi(4)
                                    * (-65. * param.s1.powi(2)
                                        + 190. * param.s1 * param.s2
                                        + 75. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (235. * param.s1.powi(3)
                                        + -559. * param.s1.powi(2) * param.s2
                                        + 5. * param.s1 * param.s2.powi(2)
                                        + -25. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-1725. * param.s1.powi(4)
                                        + 2368. * param.s1.powi(3) * param.s2
                                        + 2958. * param.s1.powi(2) * param.s2.powi(2)
                                        + -420. * param.s1 * param.s2.powi(3)
                                        + 75. * param.s2.powi(4))
                                + 2. * param.s12
                                    * (625. * param.s1.powi(5)
                                        + 681. * param.s1.powi(4) * param.s2
                                        + -3462. * param.s1.powi(3) * param.s2.powi(2)
                                        + 526. * param.s1.powi(2) * param.s2.powi(3)
                                        + 205. * param.s1 * param.s2.powi(4)
                                        + -15. * param.s2.powi(5))
                                - (param.s1 - param.s2).powi(2)
                                    * (327. * param.s1.powi(4)
                                        + 2308. * param.s1.powi(3) * param.s2
                                        + 1938. * param.s1.powi(2) * param.s2.powi(2)
                                        + 112. * param.s1 * param.s2.powi(3)
                                        + -5. * param.s2.powi(4)))
                        - param.s12.powi(2)
                            * (8925. * param.s1.powi(5)
                                + -40524. * param.s1.powi(4) * param.s2
                                + 14106. * param.s1.powi(3) * param.s2.powi(2)
                                + 3060. * param.s1.powi(2) * param.s2.powi(3)
                                + 1525. * param.s1 * param.s2.powi(4)
                                + -420. * param.s2.powi(5))
                        - (param.s1 - param.s2).powi(3)
                            * (2031. * param.s1.powi(4)
                                + 8055. * param.s1.powi(3) * param.s2
                                + 1449. * param.s1.powi(2) * param.s2.powi(2)
                                + -215. * param.s1 * param.s2.powi(3)
                                + 20. * param.s2.powi(4)))
                + -3.
                    * param.m2_2.powi(5)
                    * param.s2
                    * (11. * param.s1.powi(7)
                        + -4. * param.s12.powi(7)
                        + -314. * param.s1.powi(6) * param.s2
                        + -5767. * param.s1.powi(5) * param.s2.powi(2)
                        + -2008. * param.s1.powi(4) * param.s2.powi(3)
                        + 7337. * param.s1.powi(3) * param.s2.powi(4)
                        + 806. * param.s1.powi(2) * param.s2.powi(5)
                        + -69. * param.s1 * param.s2.powi(6)
                        + 4. * param.s2.powi(7)
                        + 7. * param.s12.powi(6) * (5. * param.s1 + 4. * param.s2)
                        + -2.
                            * param.s12.powi(5)
                            * (63. * param.s1.powi(2)
                                + 53. * param.s1 * param.s2
                                + 42. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (49. * param.s1.powi(3)
                                + -34. * param.s1.powi(2) * param.s2
                                + param.s1 * param.s2.powi(2)
                                + 28. * param.s2.powi(3))
                        + -20.
                            * param.s12.powi(3)
                            * (14. * param.s1.powi(4)
                                + -59. * param.s1.powi(3) * param.s2
                                + -23. * param.s1.powi(2) * param.s2.powi(2)
                                + -17. * param.s1 * param.s2.powi(3)
                                + 7. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (189. * param.s1.powi(5)
                                + -1880. * param.s1.powi(4) * param.s2
                                + -6366. * param.s1.powi(3) * param.s2.powi(2)
                                + 900. * param.s1.powi(2) * param.s2.powi(3)
                                + -515. * param.s1 * param.s2.powi(4)
                                + 84. * param.s2.powi(5))
                        + -2.
                            * param.s12
                            * (35. * param.s1.powi(6)
                                + -631. * param.s1.powi(5) * param.s2
                                + -5876. * param.s1.powi(4) * param.s2.powi(2)
                                + 1198. * param.s1.powi(3) * param.s2.powi(3)
                                + 935. * param.s1.powi(2) * param.s2.powi(4)
                                + -155. * param.s1 * param.s2.powi(5)
                                + 14. * param.s2.powi(6))
                        + -7.
                            * param.m1_2
                            * (param.s1.powi(6)
                                + param.s12.powi(6)
                                + -34. * param.s1.powi(5) * param.s2
                                + -973. * param.s1.powi(4) * param.s2.powi(2)
                                + -2308. * param.s1.powi(3) * param.s2.powi(3)
                                + -973. * param.s1.powi(2) * param.s2.powi(4)
                                + -34. * param.s1 * param.s2.powi(5)
                                + param.s2.powi(6)
                                + -6. * param.s12.powi(5) * (param.s1 + param.s2)
                                + 5. * param.s12.powi(4)
                                    * (3. * param.s1.powi(2)
                                        + -2. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + -20.
                                    * param.s12.powi(3)
                                    * (param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + param.s2.powi(3))
                                + 3. * param.s12.powi(2)
                                    * (5. * param.s1.powi(4)
                                        + -60. * param.s1.powi(3) * param.s2
                                        + -406. * param.s1.powi(2) * param.s2.powi(2)
                                        + -60. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                + param.s12
                                    * (-6. * param.s1.powi(5)
                                        + 130. * param.s1.powi(4) * param.s2
                                        + 2076. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2076. * param.s1.powi(2) * param.s2.powi(3)
                                        + 130. * param.s1 * param.s2.powi(4)
                                        + -6. * param.s2.powi(5))))
                + 3. * param.m2_2.powi(4)
                    * param.s2.powi(2)
                    * (-10. * param.s12.powi(7)
                        + 35. * param.s12.powi(6) * (3. * param.s1 + 2. * param.s2)
                        + -2.
                            * param.s12.powi(5)
                            * (252. * param.s1.powi(2)
                                + 185. * param.s1 * param.s2
                                + 105. * param.s2.powi(2))
                        + 5. * param.s12.powi(4)
                            * (259. * param.s1.powi(3)
                                + -36. * param.s1.powi(2) * param.s2
                                + 55. * param.s1 * param.s2.powi(2)
                                + 70. * param.s2.powi(3))
                        + -2.
                            * param.s12.powi(3)
                            * (945. * param.s1.powi(4)
                                + 1374. * param.s1.powi(3) * param.s2
                                + -1030. * param.s1.powi(2) * param.s2.powi(2)
                                + -250. * param.s1 * param.s2.powi(3)
                                + 175. * param.s2.powi(4))
                        + (param.s1 - param.s2).powi(2)
                            * (129. * param.s1.powi(5)
                                + 5066. * param.s1.powi(4) * param.s2
                                + 10086. * param.s1.powi(3) * param.s2.powi(2)
                                + 1224. * param.s1.powi(2) * param.s2.powi(3)
                                + -135. * param.s1 * param.s2.powi(4)
                                + 10. * param.s2.powi(5))
                        + param.s12.powi(2)
                            * (1575. * param.s1.powi(5)
                                + 11974. * param.s1.powi(4) * param.s2
                                + -17406. * param.s1.powi(3) * param.s2.powi(2)
                                + -60. * param.s1.powi(2) * param.s2.powi(3)
                                + -1025. * param.s1 * param.s2.powi(4)
                                + 210. * param.s2.powi(5))
                        + -2.
                            * param.s12
                            * (350. * param.s1.powi(6)
                                + 6777. * param.s1.powi(5) * param.s2
                                + -7599. * param.s1.powi(4) * param.s2.powi(2)
                                + -5678. * param.s1.powi(3) * param.s2.powi(3)
                                + 1410. * param.s1.powi(2) * param.s2.powi(4)
                                + -335. * param.s1 * param.s2.powi(5)
                                + 35. * param.s2.powi(6))
                        + -28.
                            * param.m1_2.powi(2)
                            * (-3. * param.s1.powi(5)
                                + 3. * param.s12.powi(5)
                                + -221. * param.s1.powi(4) * param.s2
                                + -1126. * param.s1.powi(3) * param.s2.powi(2)
                                + -1126. * param.s1.powi(2) * param.s2.powi(3)
                                + -221. * param.s1 * param.s2.powi(4)
                                + -3. * param.s2.powi(5)
                                + -15. * param.s12.powi(4) * (param.s1 + param.s2)
                                + param.s12.powi(3)
                                    * (30. * param.s1.powi(2)
                                        + 266. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + -6.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + 118. * param.s1.powi(2) * param.s2
                                        + 118. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))
                                + param.s12
                                    * (15. * param.s1.powi(4)
                                        + 678. * param.s1.powi(3) * param.s2
                                        + 1804. * param.s1.powi(2) * param.s2.powi(2)
                                        + 678. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4)))
                        + -7.
                            * param.m1_2
                            * (29. * param.s1.powi(6)
                                + 5. * param.s12.powi(6)
                                + 1598. * param.s1.powi(5) * param.s2
                                + 4143. * param.s1.powi(4) * param.s2.powi(2)
                                + -2532. * param.s1.powi(3) * param.s2.powi(3)
                                + -3097. * param.s1.powi(2) * param.s2.powi(4)
                                + -146. * param.s1 * param.s2.powi(5)
                                + 5. * param.s2.powi(6)
                                + -6. * param.s12.powi(5) * (9. * param.s1 + 5. * param.s2)
                                + 5. * param.s12.powi(4)
                                    * (39. * param.s1.powi(2)
                                        + 14. * param.s1 * param.s2
                                        + 15. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (85. * param.s1.powi(3)
                                        + 407. * param.s1.powi(2) * param.s2
                                        + -65. * param.s1 * param.s2.powi(2)
                                        + 25. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (315. * param.s1.powi(4)
                                        + 4764. * param.s1.powi(3) * param.s2
                                        + -426. * param.s1.powi(2) * param.s2.powi(2)
                                        + -660. * param.s1 * param.s2.powi(3)
                                        + 75. * param.s2.powi(4))
                                + -2.
                                    * param.s12
                                    * (75. * param.s1.powi(5)
                                        + 2387. * param.s1.powi(4) * param.s2
                                        + 2026. * param.s1.powi(3) * param.s2.powi(2)
                                        + -2478. * param.s1.powi(2) * param.s2.powi(3)
                                        + -265. * param.s1 * param.s2.powi(4)
                                        + 15. * param.s2.powi(5))))
                + 6. * param.m2_2.powi(2)
                    * param.s2.powi(4)
                    * (-5. * param.s12.powi(7)
                        + 35. * param.s12.powi(6) * (2. * param.s1 + param.s2)
                        + (param.s1 - param.s2).powi(4)
                            * (502. * param.s1.powi(3)
                                + 177. * param.s1.powi(2) * param.s2
                                + -40. * param.s1 * param.s2.powi(2)
                                + 5. * param.s2.powi(3))
                        + param.s12.powi(4)
                            * (-700. * param.s1.powi(3)
                                + 785. * param.s1.powi(2) * param.s2
                                + 400. * param.s1 * param.s2.powi(2)
                                + 175. * param.s2.powi(3))
                        + param.s12.powi(3)
                            * (4375. * param.s1.powi(4)
                                + -7604. * param.s1.powi(3) * param.s2
                                + 680. * param.s1.powi(2) * param.s2.powi(2)
                                + -100. * param.s1 * param.s2.powi(3)
                                + -175. * param.s2.powi(4))
                        + param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (525. * param.s1.powi(4)
                                + 5368. * param.s1.powi(3) * param.s2
                                + 170. * param.s1.powi(2) * param.s2.powi(2)
                                + 160. * param.s1 * param.s2.powi(3)
                                + -35. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (-4200. * param.s1.powi(5)
                                + 4587. * param.s1.powi(4) * param.s2
                                + 3582. * param.s1.powi(3) * param.s2.powi(2)
                                + -1080. * param.s1.powi(2) * param.s2.powi(3)
                                + -250. * param.s1 * param.s2.powi(4)
                                + 105. * param.s2.powi(5))
                        + -70.
                            * param.m1_2.powi(4)
                            * (-41. * param.s1.powi(3)
                                + 41. * param.s12.powi(3)
                                + -229. * param.s1.powi(2) * param.s2
                                + -229. * param.s1 * param.s2.powi(2)
                                + -41. * param.s2.powi(3)
                                + -123. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (123. * param.s1.powi(2)
                                        + 352. * param.s1 * param.s2
                                        + 123. * param.s2.powi(2)))
                        + 140.
                            * param.m1_2.powi(3)
                            * (-63. * param.s1.powi(4)
                                + 19. * param.s12.powi(4)
                                + param.s12.powi(3) * (6. * param.s1 + -76. * param.s2)
                                + -207. * param.s1.powi(3) * param.s2
                                + 82. * param.s1.powi(2) * param.s2.powi(2)
                                + 169. * param.s1 * param.s2.powi(3)
                                + 19. * param.s2.powi(4)
                                + param.s12.powi(2)
                                    * (-132. * param.s1.powi(2)
                                        + 157. * param.s1 * param.s2
                                        + 114. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (85. * param.s1.powi(3)
                                        + 63. * param.s1.powi(2) * param.s2
                                        + -166. * param.s1 * param.s2.powi(2)
                                        + -38. * param.s2.powi(3)))
                        + -84.
                            * param.m1_2.powi(2)
                            * (3. * param.s12.powi(5)
                                + 5. * param.s12.powi(4) * (16. * param.s1 + -3. * param.s2)
                                + param.s12.powi(3)
                                    * (-145. * param.s1.powi(2)
                                        + -114. * param.s1 * param.s2
                                        + 30. * param.s2.powi(2))
                                + param.s12
                                    * (250. * param.s1.powi(4)
                                        + -452. * param.s1.powi(3) * param.s2
                                        + -471. * param.s1.powi(2) * param.s2.powi(2)
                                        + 298. * param.s1 * param.s2.powi(3)
                                        + 15. * param.s2.powi(4))
                                - param.s12.powi(2)
                                    * (75. * param.s1.powi(3)
                                        + -692. * param.s1.powi(2) * param.s2
                                        + 138. * param.s1 * param.s2.powi(2)
                                        + 30. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(2)
                                    * (113. * param.s1.powi(3)
                                        + 337. * param.s1.powi(2) * param.s2
                                        + 132. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)))
                        + -7.
                            * param.m1_2
                            * (5. * param.s12.powi(6)
                                + -6. * param.s12.powi(5) * (17. * param.s1 + 5. * param.s2)
                                + param.s12.powi(4)
                                    * (-705. * param.s1.powi(2)
                                        + 310. * param.s1 * param.s2
                                        + 75. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (525. * param.s1.powi(3)
                                        + -331. * param.s1.powi(2) * param.s2
                                        + -55. * param.s1 * param.s2.powi(2)
                                        + -25. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(3)
                                    * (577. * param.s1.powi(3)
                                        + 965. * param.s1.powi(2) * param.s2
                                        + 83. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3))
                                + -3.
                                    * param.s12.powi(2)
                                    * (375. * param.s1.powi(4)
                                        + 1056. * param.s1.powi(3) * param.s2
                                        + -1354. * param.s1.powi(2) * param.s2.powi(2)
                                        + 60. * param.s1 * param.s2.powi(3)
                                        + -25. * param.s2.powi(4))
                                + -2.
                                    * param.s12
                                    * (375. * param.s1.powi(5)
                                        + -2489. * param.s1.powi(4) * param.s2
                                        + 1578. * param.s1.powi(3) * param.s2.powi(2)
                                        + 666. * param.s1.powi(2) * param.s2.powi(3)
                                        + -145. * param.s1 * param.s2.powi(4)
                                        + 15. * param.s2.powi(5)))
                        - param.s12.powi(5)
                            * (567. * param.s1.powi(2)
                                + 290. * param.s1 * param.s2
                                + 105. * param.s2.powi(2)))
                + param.m0_2.powi(5)
                    * (3.
                        * param.m2_2
                        * (4. * param.s12.powi(7)
                            + -7. * param.s12.powi(6) * (4. * param.s1 + 9. * param.s2)
                            + param.s12.powi(5)
                                * (84. * param.s1.powi(2)
                                    + 274. * param.s1 * param.s2
                                    + 630. * param.s2.powi(2))
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (14. * param.s1.powi(3)
                                    + -29. * param.s1.powi(2) * param.s2
                                    + -174. * param.s1 * param.s2.powi(2)
                                    + -959. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(4)
                                * (28. * param.s1.powi(3)
                                    + 85. * param.s1.powi(2) * param.s2
                                    + 246. * param.s1 * param.s2.powi(2)
                                    + -595. * param.s2.powi(3))
                            + 4. * param.s12.powi(3)
                                * (35. * param.s1.powi(4)
                                    + 55. * param.s1.powi(3) * param.s2
                                    + 25. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1133. * param.s1 * param.s2.powi(3)
                                    + -1400. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-84. * param.s1.powi(5)
                                    + 95. * param.s1.powi(4) * param.s2
                                    + 780. * param.s1.powi(3) * param.s2.powi(2)
                                    + -6906. * param.s1.powi(2) * param.s2.powi(3)
                                    + 6024. * param.s1 * param.s2.powi(4)
                                    + 91. * param.s2.powi(5))
                            - (param.s1 - param.s2).powi(5)
                                * (4. * param.s1.powi(2)
                                    + -21. * param.s1 * param.s2
                                    + 45. * param.s2.powi(2)))
                        + param.s2
                            * (26. * param.s12.powi(7)
                                + -7. * param.s12.powi(6) * (23. * param.s1 + 95. * param.s2)
                                + 4. * param.s12.powi(5)
                                    * (105. * param.s1.powi(2)
                                        + 608. * param.s1 * param.s2
                                        + -2037. * param.s2.powi(2))
                                + 8. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (7. * param.s1.powi(2)
                                        + -27. * param.s1 * param.s2
                                        + -70. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-595. * param.s1.powi(3)
                                        + -3025. * param.s1.powi(2) * param.s2
                                        + 7055. * param.s1 * param.s2.powi(2)
                                        + 8925. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (245. * param.s1.powi(4)
                                        + 540. * param.s1.powi(3) * param.s2
                                        + 5274. * param.s1.powi(2) * param.s2.powi(2)
                                        + -14972. * param.s1 * param.s2.powi(3)
                                        + 4305. * param.s2.powi(4))
                                + -21.
                                    * param.m1_2
                                    * (param.s12.powi(6)
                                        + (param.s1 + -5. * param.s2)
                                            * (param.s1 - param.s2).powi(5)
                                        + -6. * param.s12.powi(5) * (param.s1 + 5. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (3. * param.s1.powi(2)
                                                + 22. * param.s1 * param.s2
                                                + -125. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + 157. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (5. * param.s1.powi(3)
                                                + 35. * param.s1.powi(2) * param.s2
                                                + -253. * param.s1 * param.s2.powi(2)
                                                + 125. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (15. * param.s1.powi(4)
                                                + 60. * param.s1.powi(3) * param.s2
                                                + -114. * param.s1.powi(2) * param.s2.powi(2)
                                                + -796. * param.s1 * param.s2.powi(3)
                                                + 835. * param.s2.powi(4)))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (231. * param.s1.powi(3)
                                        + -103. * param.s1.powi(2) * param.s2
                                        + 9445. * param.s1 * param.s2.powi(2)
                                        + 8211. * param.s2.powi(3))
                                - (5. * param.s1 + -23. * param.s2)
                                    * (param.s1 - param.s2).powi(6)))
                + param.m0_2.powi(4)
                    * (-6.
                        * param.m2_2.powi(2)
                        * (5. * param.s12.powi(7)
                            + -35. * param.s12.powi(6) * (param.s1 + 2. * param.s2)
                            + param.s12.powi(5)
                                * (105. * param.s1.powi(2)
                                    + 290. * param.s1 * param.s2
                                    + 567. * param.s2.powi(2))
                            + -5.
                                * param.s12.powi(4)
                                * (35. * param.s1.powi(3)
                                    + 80. * param.s1.powi(2) * param.s2
                                    + 157. * param.s1 * param.s2.powi(2)
                                    + -140. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (175. * param.s1.powi(4)
                                    + 100. * param.s1.powi(3) * param.s2
                                    + -680. * param.s1.powi(2) * param.s2.powi(2)
                                    + 7604. * param.s1 * param.s2.powi(3)
                                    + -4375. * param.s2.powi(4))
                            + param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (35. * param.s1.powi(4)
                                    + -160. * param.s1.powi(3) * param.s2
                                    + -170. * param.s1.powi(2) * param.s2.powi(2)
                                    + -5368. * param.s1 * param.s2.powi(3)
                                    + -525. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-105. * param.s1.powi(5)
                                    + 250. * param.s1.powi(4) * param.s2
                                    + 1080. * param.s1.powi(3) * param.s2.powi(2)
                                    + -3582. * param.s1.powi(2) * param.s2.powi(3)
                                    + -4587. * param.s1 * param.s2.powi(4)
                                    + 4200. * param.s2.powi(5))
                            - (param.s1 - param.s2).powi(4)
                                * (5. * param.s1.powi(3)
                                    + -40. * param.s1.powi(2) * param.s2
                                    + 177. * param.s1 * param.s2.powi(2)
                                    + 502. * param.s2.powi(3)))
                        + param.s2.powi(2)
                            * (-226. * param.s12.powi(7)
                                + 7. * param.s12.powi(6) * (157. * param.s1 + -848. * param.s2)
                                + -5.
                                    * (param.s1 - param.s2).powi(6)
                                    * (param.s1 + 8. * param.s2)
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (28. * param.s1.powi(2)
                                        + 459. * param.s1 * param.s2
                                        + 413. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-2100. * param.s1.powi(2)
                                        + 7094. * param.s1 * param.s2
                                        + 9450. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (385. * param.s1.powi(3)
                                        + 2734. * param.s1.powi(2) * param.s2
                                        + -10559. * param.s1 * param.s2.powi(2)
                                        + 1764. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (21. * param.s1.powi(3)
                                        + 8734. * param.s1.powi(2) * param.s2
                                        + 28229. * param.s1 * param.s2.powi(2)
                                        + 7476. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-770. * param.s1.powi(4)
                                        + -24204. * param.s1.powi(3) * param.s2
                                        + 34908. * param.s1.powi(2) * param.s2.powi(2)
                                        + 33476. * param.s1 * param.s2.powi(3)
                                        + -20370. * param.s2.powi(4))
                                + -84.
                                    * param.m1_2.powi(2)
                                    * (3. * param.s12.powi(5)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (3. * param.s1 + -35. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (15. * param.s1.powi(2)
                                                + -52. * param.s1 * param.s2
                                                + -405. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (30. * param.s1.powi(2)
                                                + -494. * param.s1 * param.s2
                                                + 500. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (15. * param.s1.powi(3)
                                                + -216. * param.s1.powi(2) * param.s2
                                                + 184. * param.s1 * param.s2.powi(2)
                                                + 115. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(4)
                                            * (3. * param.s1 + 43. * param.s2))
                                + 21.
                                    * param.m1_2
                                    * (23. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-114. * param.s1 + 966. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (45. * param.s1.powi(2)
                                                + -446. * param.s1 * param.s2
                                                + 133. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (9. * param.s1.powi(2)
                                                + 340. * param.s1 * param.s2
                                                + 471. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (55. * param.s1.powi(3)
                                                + -227. * param.s1.powi(2) * param.s2
                                                + -927. * param.s1 * param.s2.powi(2)
                                                + 835. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (105. * param.s1.powi(4)
                                                + 996. * param.s1.powi(3) * param.s2
                                                + -5502. * param.s1.powi(2) * param.s2.powi(2)
                                                + 3676. * param.s1 * param.s2.powi(3)
                                                + 725. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(5)
                                            * (param.s1 + 19. * param.s2)))
                        + param.m2_2
                            * param.s2
                            * (-92. * param.s12.powi(7)
                                + 7. * param.s12.powi(6) * (77. * param.s1 + 284. * param.s2)
                                + -2.
                                    * param.s12.powi(5)
                                    * (651. * param.s1.powi(2)
                                        + 2941. * param.s1 * param.s2
                                        + -7098. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (329. * param.s1.powi(3)
                                        + 758. * param.s1.powi(2) * param.s2
                                        + 5609. * param.s1 * param.s2.powi(2)
                                        + -8148. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (7. * param.s1.powi(3)
                                        + -730. * param.s1.powi(2) * param.s2
                                        + -10603. * param.s1 * param.s2.powi(2)
                                        + -5894. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (280. * param.s1.powi(4)
                                        + -975. * param.s1.powi(3) * param.s2
                                        + 20553. * param.s1.powi(2) * param.s2.powi(2)
                                        + -16501. * param.s1 * param.s2.powi(3)
                                        + -4725. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (357. * param.s1.powi(5)
                                        + -5440. * param.s1.powi(4) * param.s2
                                        + 23346. * param.s1.powi(3) * param.s2.powi(2)
                                        + 68412. * param.s1.powi(2) * param.s2.powi(3)
                                        + -104567. * param.s1 * param.s2.powi(4)
                                        + 17892. * param.s2.powi(5))
                                + 21.
                                    * param.m1_2
                                    * (5. * param.s12.powi(6)
                                        + -6.
                                            * param.s12.powi(5)
                                            * (5. * param.s1 + 21. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (15. * param.s1.powi(2)
                                                + 86. * param.s1 * param.s2
                                                + -345. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (5. * param.s1.powi(2)
                                                + -54. * param.s1 * param.s2
                                                + -319. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (25. * param.s1.powi(3)
                                                + 115. * param.s1.powi(2) * param.s2
                                                + -277. * param.s1 * param.s2.powi(2)
                                                + -375. * param.s2.powi(3))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (15. * param.s1.powi(3)
                                                + -55. * param.s1.powi(2) * param.s2
                                                + 973. * param.s1 * param.s2.powi(2)
                                                + 835. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (75. * param.s1.powi(4)
                                                + 60. * param.s1.powi(3) * param.s2
                                                + 2886. * param.s1.powi(2) * param.s2.powi(2)
                                                + -6924. * param.s1 * param.s2.powi(3)
                                                + 2335. * param.s2.powi(4)))
                                - (param.s1 - param.s2).powi(5)
                                    * (13. * param.s1.powi(2)
                                        + -77. * param.s1 * param.s2
                                        + -356. * param.s2.powi(2))))
                + param.m0_2
                    * (3.
                        * param.m2_2.powi(5)
                        * (-4. * param.s1.powi(7)
                            + 4. * param.s12.powi(7)
                            + 69. * param.s1.powi(6) * param.s2
                            + -806. * param.s1.powi(5) * param.s2.powi(2)
                            + -7337. * param.s1.powi(4) * param.s2.powi(3)
                            + 2008. * param.s1.powi(3) * param.s2.powi(4)
                            + 5767. * param.s1.powi(2) * param.s2.powi(5)
                            + 314. * param.s1 * param.s2.powi(6)
                            + -11. * param.s2.powi(7)
                            + -7. * param.s12.powi(6) * (4. * param.s1 + 5. * param.s2)
                            + 2. * param.s12.powi(5)
                                * (42. * param.s1.powi(2)
                                    + 53. * param.s1 * param.s2
                                    + 63. * param.s2.powi(2))
                            + -5.
                                * param.s12.powi(4)
                                * (28. * param.s1.powi(3)
                                    + param.s1.powi(2) * param.s2
                                    + -34. * param.s1 * param.s2.powi(2)
                                    + 49. * param.s2.powi(3))
                            + 20.
                                * param.s12.powi(3)
                                * (7. * param.s1.powi(4)
                                    + -17. * param.s1.powi(3) * param.s2
                                    + -23. * param.s1.powi(2) * param.s2.powi(2)
                                    + -59. * param.s1 * param.s2.powi(3)
                                    + 14. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-84. * param.s1.powi(5)
                                    + 515. * param.s1.powi(4) * param.s2
                                    + -900. * param.s1.powi(3) * param.s2.powi(2)
                                    + 6366. * param.s1.powi(2) * param.s2.powi(3)
                                    + 1880. * param.s1 * param.s2.powi(4)
                                    + -189. * param.s2.powi(5))
                            + 2. * param.s12
                                * (14. * param.s1.powi(6)
                                    + -155. * param.s1.powi(5) * param.s2
                                    + 935. * param.s1.powi(4) * param.s2.powi(2)
                                    + 1198. * param.s1.powi(3) * param.s2.powi(3)
                                    + -5876. * param.s1.powi(2) * param.s2.powi(4)
                                    + -631. * param.s1 * param.s2.powi(5)
                                    + 35. * param.s2.powi(6)))
                        + param.m2_2
                            * param.s2.powi(4)
                            * (-92. * param.s12.powi(7)
                                + 7. * param.s12.powi(6) * (284. * param.s1 + 77. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (7098. * param.s1.powi(2)
                                        + -2941. * param.s1 * param.s2
                                        + -651. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (8148. * param.s1.powi(3)
                                        + -5609. * param.s1.powi(2) * param.s2
                                        + -758. * param.s1 * param.s2.powi(2)
                                        + -329. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (5894. * param.s1.powi(3)
                                        + 10603. * param.s1.powi(2) * param.s2
                                        + 730. * param.s1 * param.s2.powi(2)
                                        + -7. * param.s2.powi(3))
                                + 4. * param.s12.powi(3)
                                    * (4725. * param.s1.powi(4)
                                        + 16501. * param.s1.powi(3) * param.s2
                                        + -20553. * param.s1.powi(2) * param.s2.powi(2)
                                        + 975. * param.s1 * param.s2.powi(3)
                                        + -280. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (17892. * param.s1.powi(5)
                                        + -104567. * param.s1.powi(4) * param.s2
                                        + 68412. * param.s1.powi(3) * param.s2.powi(2)
                                        + 23346. * param.s1.powi(2) * param.s2.powi(3)
                                        + -5440. * param.s1 * param.s2.powi(4)
                                        + 357. * param.s2.powi(5))
                                + 420.
                                    * param.m1_2.powi(4)
                                    * (-82. * param.s1.powi(3)
                                        + 82. * param.s12.powi(3)
                                        + -223. * param.s1.powi(2) * param.s2
                                        + 152. * param.s1 * param.s2.powi(2)
                                        + 153. * param.s2.powi(3)
                                        + param.s12
                                            * (246. * param.s1.powi(2)
                                                + 234. * param.s1 * param.s2
                                                + -224. * param.s2.powi(2))
                                        - param.s12.powi(2)
                                            * (246. * param.s1 + 11. * param.s2))
                                + -280.
                                    * param.m1_2.powi(3)
                                    * (203. * param.s12.powi(4)
                                        + -320. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12.powi(2)
                                            * (129. * param.s1.powi(2)
                                                + -832. * param.s1 * param.s2
                                                + 129. * param.s2.powi(2))
                                        + 8. * param.s12
                                            * (83. * param.s1.powi(3)
                                                + -137. * param.s1.powi(2) * param.s2
                                                + -137. * param.s1 * param.s2.powi(2)
                                                + 83. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (289. * param.s1.powi(2)
                                                + 826. * param.s1 * param.s2
                                                + 289. * param.s2.powi(2)))
                                + -168.
                                    * param.m1_2
                                    * (8. * param.s12.powi(6)
                                        + 12.
                                            * param.s12.powi(5)
                                            * (20. * param.s1 + -3. * param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (61. * param.s1.powi(2)
                                                + 43. * param.s1 * param.s2
                                                + -12. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (52. * param.s1.powi(3)
                                                + 332. * param.s1.powi(2) * param.s2
                                                + 61. * param.s1 * param.s2.powi(2)
                                                + -3. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (130. * param.s1.powi(3)
                                                + -559. * param.s1.powi(2) * param.s2
                                                + 179. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        + 2. * param.s1
                                            * param.s12.powi(2)
                                            * (435. * param.s1.powi(3)
                                                + -659. * param.s1.powi(2) * param.s2
                                                + -687. * param.s1 * param.s2.powi(2)
                                                + 519. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(4)
                                            * (85. * param.s1.powi(2)
                                                + 95. * param.s1 * param.s2
                                                + 4. * param.s2.powi(2)))
                                + 168.
                                    * param.m1_2.powi(2)
                                    * (144. * param.s12.powi(5)
                                        + 5. * param.s12.powi(4)
                                            * (59. * param.s1 + -87. * param.s2)
                                        + param.s12.powi(3)
                                            * (-1390. * param.s1.powi(2)
                                                + 1448. * param.s1 * param.s2
                                                + 300. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (960. * param.s1.powi(3)
                                                + 1951. * param.s1.powi(2) * param.s2
                                                + -3489. * param.s1 * param.s2.powi(2)
                                                + 270. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (175. * param.s1.powi(4)
                                                + -1663. * param.s1.powi(3) * param.s2
                                                + 971. * param.s1.powi(2) * param.s2.powi(2)
                                                + 727. * param.s1 * param.s2.powi(3)
                                                + -210. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (359. * param.s1.powi(2)
                                                + 715. * param.s1 * param.s2
                                                + 141. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(5)
                                    * (356. * param.s1.powi(2)
                                        + 77. * param.s1 * param.s2
                                        + -13. * param.s2.powi(2)))
                        + param.s2.powi(5)
                            * (26. * param.s12.powi(7)
                                + (23. * param.s1 + -5. * param.s2)
                                    * (param.s1 - param.s2).powi(6)
                                + -7. * param.s12.powi(6) * (95. * param.s1 + 23. * param.s2)
                                + 420.
                                    * param.m1_2.powi(5)
                                    * (47. * param.s1.powi(2)
                                        + -94. * param.s1 * param.s12
                                        + 47. * param.s12.powi(2)
                                        + 14. * param.s1 * param.s2
                                        + 14. * param.s12 * param.s2
                                        + -61. * param.s2.powi(2))
                                + -8.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (70. * param.s1.powi(2)
                                        + 27. * param.s1 * param.s2
                                        + -7. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-8148. * param.s1.powi(2)
                                        + 2432. * param.s1 * param.s2
                                        + 420. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (1785. * param.s1.powi(3)
                                        + 1411. * param.s1.powi(2) * param.s2
                                        + -605. * param.s1 * param.s2.powi(2)
                                        + -119. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (4305. * param.s1.powi(4)
                                        + -14972. * param.s1.powi(3) * param.s2
                                        + 5274. * param.s1.powi(2) * param.s2.powi(2)
                                        + 540. * param.s1 * param.s2.powi(3)
                                        + 245. * param.s2.powi(4))
                                + -420.
                                    * param.m1_2.powi(4)
                                    * (118. * param.s12.powi(3)
                                        + -119. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 117.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -4.
                                            * param.s12
                                            * (29. * param.s1.powi(2)
                                                + -94. * param.s1 * param.s2
                                                + 29. * param.s2.powi(2)))
                                + 280.
                                    * param.m1_2.powi(3)
                                    * (146. * param.s12.powi(4)
                                        + 2. * param.s12.powi(3)
                                            * (62. * param.s1 + -169. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (143. * param.s1 + 100. * param.s2)
                                        + param.s12.powi(2)
                                            * (-543. * param.s1.powi(2)
                                                + 545. * param.s1 * param.s2
                                                + 138. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (65. * param.s1.powi(3)
                                                + 271. * param.s1.powi(2) * param.s2
                                                + -413. * param.s1 * param.s2.powi(2)
                                                + 77. * param.s2.powi(3)))
                                + -168.
                                    * param.m1_2.powi(2)
                                    * (69. * param.s12.powi(5)
                                        + 5. * param.s12.powi(4)
                                            * (77. * param.s1 + -50. * param.s2)
                                        + 2. * (param.s1 - param.s2).powi(4)
                                            * (33. * param.s1 + 13. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (385. * param.s1.powi(2)
                                                + 534. * param.s1 * param.s2
                                                + -35. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-460. * param.s1.powi(2)
                                                + -242. * param.s1 * param.s2
                                                + 310. * param.s2.powi(2))
                                        - param.s12.powi(2)
                                            * (445. * param.s1.powi(3)
                                                + -1666. * param.s1.powi(2) * param.s2
                                                + 709. * param.s1 * param.s2.powi(2)
                                                + 120. * param.s2.powi(3)))
                                + 21.
                                    * param.m1_2
                                    * (23. * param.s12.powi(6)
                                        + 6. * param.s12.powi(5)
                                            * (161. * param.s1 + -19. * param.s2)
                                        + (param.s1 - param.s2).powi(5)
                                            * (19. * param.s1 + param.s2)
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (471. * param.s1.powi(2)
                                                + 340. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + 5. * param.s12.powi(4)
                                            * (133. * param.s1.powi(2)
                                                + -446. * param.s1 * param.s2
                                                + 45. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (835. * param.s1.powi(3)
                                                + -927. * param.s1.powi(2) * param.s2
                                                + -227. * param.s1 * param.s2.powi(2)
                                                + 55. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (725. * param.s1.powi(4)
                                                + 3676. * param.s1.powi(3) * param.s2
                                                + -5502. * param.s1.powi(2) * param.s2.powi(2)
                                                + 996. * param.s1 * param.s2.powi(3)
                                                + 105. * param.s2.powi(4)))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (8211. * param.s1.powi(3)
                                        + 9445. * param.s1.powi(2) * param.s2
                                        + -103. * param.s1 * param.s2.powi(2)
                                        + 231. * param.s2.powi(3)))
                        + 2. * param.m2_2.powi(3)
                            * param.s2.powi(2)
                            * (-16. * param.s12.powi(7)
                                + 7. * param.s12.powi(6) * (28. * param.s1 + param.s2)
                                + param.s12.powi(5)
                                    * (-336. * param.s1.powi(2)
                                        + 1382. * param.s1 * param.s2
                                        + 294. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (140. * param.s1.powi(3)
                                        + -3469. * param.s1.powi(2) * param.s2
                                        + 1130. * param.s1 * param.s2.powi(2)
                                        + 203. * param.s2.powi(3))
                                + 4. * param.s12.powi(3)
                                    * (700. * param.s1.powi(4)
                                        + -12183. * param.s1.powi(3) * param.s2
                                        + 5661. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1215. * param.s1 * param.s2.powi(3)
                                        + 385. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-3444. * param.s1.powi(5)
                                        + 28201. * param.s1.powi(4) * param.s2
                                        + 68952. * param.s1.powi(3) * param.s2.powi(2)
                                        + -81702. * param.s1.powi(2) * param.s2.powi(3)
                                        + 2560. * param.s1 * param.s2.powi(4)
                                        + -1239. * param.s2.powi(5))
                                + 2. * param.s12
                                    * (952. * param.s1.powi(6)
                                        + 6731. * param.s1.powi(5) * param.s2
                                        + -52201. * param.s1.powi(4) * param.s2.powi(2)
                                        + 32966. * param.s1.powi(3) * param.s2.powi(3)
                                        + 13886. * param.s1.powi(2) * param.s2.powi(4)
                                        + -2593. * param.s1 * param.s2.powi(5)
                                        + 259. * param.s2.powi(6))
                                + 84.
                                    * param.m1_2.powi(2)
                                    * (-6. * param.s1.powi(5)
                                        + 6. * param.s12.powi(5)
                                        + -347. * param.s1.powi(4) * param.s2
                                        + -997. * param.s1.powi(3) * param.s2.powi(2)
                                        + 448. * param.s1.powi(2) * param.s2.powi(3)
                                        + 813. * param.s1 * param.s2.powi(4)
                                        + 89. * param.s2.powi(5)
                                        + param.s12.powi(4)
                                            * (-30. * param.s1 + 65. * param.s2)
                                        + 4. * param.s12.powi(3)
                                            * (15. * param.s1.powi(2)
                                                + 38. * param.s1 * param.s2
                                                + -80. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-60. * param.s1.powi(3)
                                                + -846. * param.s1.powi(2) * param.s2
                                                + 599. * param.s1 * param.s2.powi(2)
                                                + 510. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (15. * param.s1.powi(4)
                                                + 488. * param.s1.powi(3) * param.s2
                                                + 359. * param.s1.powi(2) * param.s2.powi(2)
                                                + -767. * param.s1 * param.s2.powi(3)
                                                + -175. * param.s2.powi(4)))
                                + -84.
                                    * param.m1_2
                                    * (param.s12.powi(6)
                                        + 6. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (9. * param.s1.powi(2)
                                                + -65. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (25. * param.s1.powi(3)
                                                + -134. * param.s1.powi(2) * param.s2
                                                + -134. * param.s1 * param.s2.powi(2)
                                                + 25. * param.s2.powi(3))
                                        + -3.
                                            * param.s12.powi(2)
                                            * (35. * param.s1.powi(4)
                                                + 134. * param.s1.powi(3) * param.s2
                                                + -926. * param.s1.powi(2) * param.s2.powi(2)
                                                + 134. * param.s1 * param.s2.powi(3)
                                                + 35. * param.s2.powi(4))
                                        + 2. * param.s12
                                            * (27. * param.s1.powi(5)
                                                + 541. * param.s1.powi(4) * param.s2
                                                + -928. * param.s1.powi(3) * param.s2.powi(2)
                                                + -928. * param.s1.powi(2) * param.s2.powi(3)
                                                + 541. * param.s1 * param.s2.powi(4)
                                                + 27. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(2)
                                            * (11. * param.s1.powi(4)
                                                + 497. * param.s1.powi(3) * param.s2
                                                + 1324. * param.s1.powi(2) * param.s2.powi(2)
                                                + 497. * param.s1 * param.s2.powi(3)
                                                + 11. * param.s2.powi(4)))
                                - (param.s1 - param.s2).powi(3)
                                    * (404. * param.s1.powi(4)
                                        + 12877. * param.s1.powi(3) * param.s2
                                        + 19257. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1571. * param.s1 * param.s2.powi(3)
                                        + -89. * param.s2.powi(4)))
                        + 6. * param.m2_2.powi(2)
                            * param.s2.powi(3)
                            * (18. * param.s12.powi(7)
                                + -7. * param.s12.powi(6) * (45. * param.s1 + 13. * param.s2)
                                + param.s12.powi(5)
                                    * (-756. * param.s1.powi(2)
                                        + 456. * param.s1 * param.s2
                                        + 168. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (1043. * param.s1.powi(3)
                                        + -2451. * param.s1.powi(2) * param.s2
                                        + 209. * param.s1 * param.s2.powi(2)
                                        + -21. * param.s2.powi(3))
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (140. * param.s1.powi(4)
                                        + -3448. * param.s1.powi(3) * param.s2
                                        + -2897. * param.s1.powi(2) * param.s2.powi(2)
                                        + 38. * param.s1 * param.s2.powi(3)
                                        + -21. * param.s2.powi(4))
                                + -2.
                                    * param.s12.powi(3)
                                    * (3815. * param.s1.powi(4)
                                        + -4912. * param.s1.powi(3) * param.s2
                                        + -6574. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1220. * param.s1 * param.s2.powi(3)
                                        + 35. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (3507. * param.s1.powi(5)
                                        + 16363. * param.s1.powi(4) * param.s2
                                        + -44874. * param.s1.powi(3) * param.s2.powi(2)
                                        + 12666. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1215. * param.s1 * param.s2.powi(4)
                                        + 147. * param.s2.powi(5))
                                + 140.
                                    * param.m1_2.powi(3)
                                    * (19. * param.s1.powi(4)
                                        + 19. * param.s12.powi(4)
                                        + 169. * param.s1.powi(3) * param.s2
                                        + 82. * param.s1.powi(2) * param.s2.powi(2)
                                        + -207. * param.s1 * param.s2.powi(3)
                                        + -63. * param.s2.powi(4)
                                        + param.s12.powi(3) * (-76. * param.s1 + 6. * param.s2)
                                        + param.s12.powi(2)
                                            * (114. * param.s1.powi(2)
                                                + 157. * param.s1 * param.s2
                                                + -132. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (38. * param.s1.powi(3)
                                                + 166. * param.s1.powi(2) * param.s2
                                                + -63. * param.s1 * param.s2.powi(2)
                                                + -85. * param.s2.powi(3)))
                                + -84.
                                    * param.m1_2.powi(2)
                                    * (27. * param.s12.powi(5)
                                        + -40. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (55. * param.s1.powi(2)
                                                + -307. * param.s1 * param.s2
                                                + 55. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (68. * param.s1.powi(3)
                                                + 517. * param.s1.powi(2) * param.s2
                                                + 517. * param.s1 * param.s2.powi(2)
                                                + 68. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (300. * param.s1.powi(3)
                                                + -727. * param.s1.powi(2) * param.s2
                                                + -727. * param.s1 * param.s2.powi(2)
                                                + 300. * param.s2.powi(3))
                                        - param.s12
                                            * (245. * param.s1.powi(4)
                                                + 228. * param.s1.powi(3) * param.s2
                                                + -1666. * param.s1.powi(2) * param.s2.powi(2)
                                                + 228. * param.s1 * param.s2.powi(3)
                                                + 245. * param.s2.powi(4)))
                                + 21.
                                    * param.m1_2
                                    * (9. * param.s12.powi(6)
                                        + 6. * param.s12.powi(5)
                                            * (27. * param.s1 + -5. * param.s2)
                                        + param.s12.powi(4)
                                            * (-565. * param.s1.powi(2)
                                                + 530. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (115. * param.s1.powi(3)
                                                + 423. * param.s1.powi(2) * param.s2
                                                + -523. * param.s1 * param.s2.powi(2)
                                                + 15. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(3)
                                            * (173. * param.s1.powi(3)
                                                + 917. * param.s1.powi(2) * param.s2
                                                + 515. * param.s1 * param.s2.powi(2)
                                                + 15. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (255. * param.s1.powi(4)
                                                + -4456. * param.s1.powi(3) * param.s2
                                                + 2718. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1476. * param.s1 * param.s2.powi(3)
                                                + -105. * param.s2.powi(4))
                                        + param.s12
                                            * (-494. * param.s1.powi(5)
                                                + 1866. * param.s1.powi(4) * param.s2
                                                + 2596. * param.s1.powi(3) * param.s2.powi(2)
                                                + -4428. * param.s1.powi(2) * param.s2.powi(3)
                                                + 394. * param.s1 * param.s2.powi(4)
                                                + 66. * param.s2.powi(5)))
                                - (param.s1 - param.s2).powi(4)
                                    * (599. * param.s1.powi(3)
                                        + 1781. * param.s1.powi(2) * param.s2
                                        + 213. * param.s1 * param.s2.powi(2)
                                        + -17. * param.s2.powi(3)))
                        + param.m2_2.powi(4)
                            * param.s2
                            * (-22. * param.s12.powi(7)
                                + 259. * param.s12.powi(6) * (param.s1 + param.s2)
                                + -4.
                                    * param.s12.powi(5)
                                    * (273. * param.s1.powi(2)
                                        + 592. * param.s1 * param.s2
                                        + 273. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (469. * param.s1.powi(3)
                                        + 751. * param.s1.powi(2) * param.s2
                                        + 751. * param.s1 * param.s2.powi(2)
                                        + 469. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-2870. * param.s1.powi(4)
                                        + 3480. * param.s1.powi(3) * param.s2
                                        + -51132. * param.s1.powi(2) * param.s2.powi(2)
                                        + 3480. * param.s1 * param.s2.powi(3)
                                        + -2870. * param.s2.powi(4))
                                + (param.s1 - param.s2).powi(2)
                                    * (127. * param.s1.powi(5)
                                        + -2873. * param.s1.powi(4) * param.s2
                                        + -46394. * param.s1.powi(3) * param.s2.powi(2)
                                        + -46394. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2873. * param.s1 * param.s2.powi(4)
                                        + 127. * param.s2.powi(5))
                                + param.s12.powi(2)
                                    * (2037. * param.s1.powi(5)
                                        + -13175. * param.s1.powi(4) * param.s2
                                        + 54846. * param.s1.powi(3) * param.s2.powi(2)
                                        + 54846. * param.s1.powi(2) * param.s2.powi(3)
                                        + -13175. * param.s1 * param.s2.powi(4)
                                        + 2037. * param.s2.powi(5))
                                + -8.
                                    * param.s12
                                    * (98. * param.s1.powi(6)
                                        + -1397. * param.s1.powi(5) * param.s2
                                        + -4268. * param.s1.powi(4) * param.s2.powi(2)
                                        + 18694. * param.s1.powi(3) * param.s2.powi(3)
                                        + -4268. * param.s1.powi(2) * param.s2.powi(4)
                                        + -1397. * param.s1 * param.s2.powi(5)
                                        + 98. * param.s2.powi(6))
                                + -21.
                                    * param.m1_2
                                    * (5. * param.s1.powi(6)
                                        + 5. * param.s12.powi(6)
                                        + -146. * param.s1.powi(5) * param.s2
                                        + -3097. * param.s1.powi(4) * param.s2.powi(2)
                                        + -2532. * param.s1.powi(3) * param.s2.powi(3)
                                        + 4143. * param.s1.powi(2) * param.s2.powi(4)
                                        + 1598. * param.s1 * param.s2.powi(5)
                                        + 29. * param.s2.powi(6)
                                        + -6.
                                            * param.s12.powi(5)
                                            * (5. * param.s1 + 9. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (15. * param.s1.powi(2)
                                                + 14. * param.s1 * param.s2
                                                + 39. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (25. * param.s1.powi(3)
                                                + -65. * param.s1.powi(2) * param.s2
                                                + 407. * param.s1 * param.s2.powi(2)
                                                + 85. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (75. * param.s1.powi(4)
                                                + -660. * param.s1.powi(3) * param.s2
                                                + -426. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4764. * param.s1 * param.s2.powi(3)
                                                + 315. * param.s2.powi(4))
                                        + -2.
                                            * param.s12
                                            * (15. * param.s1.powi(5)
                                                + -265. * param.s1.powi(4) * param.s2
                                                + -2478. * param.s1.powi(3) * param.s2.powi(2)
                                                + 2026. * param.s1.powi(2) * param.s2.powi(3)
                                                + 2387. * param.s1 * param.s2.powi(4)
                                                + 75. * param.s2.powi(5)))))
                + 2. * param.m0_2.powi(3)
                    * (param.m2_2.powi(3)
                        * (20. * param.s12.powi(7)
                            + -35. * param.s12.powi(6) * (4. * param.s1 + 7. * param.s2)
                            + 2. * param.s12.powi(5)
                                * (210. * param.s1.powi(2)
                                    + 475. * param.s1 * param.s2
                                    + 777. * param.s2.powi(2))
                            + -5.
                                * param.s12.powi(4)
                                * (140. * param.s1.powi(3)
                                    + 215. * param.s1.powi(2) * param.s2
                                    + 194. * param.s1 * param.s2.powi(2)
                                    + 427. * param.s2.powi(3))
                            + 4. * param.s12.powi(3)
                                * (175. * param.s1.powi(4)
                                    + -75. * param.s1.powi(3) * param.s2
                                    + -1065. * param.s1.powi(2) * param.s2.powi(2)
                                    + 5287. * param.s1 * param.s2.powi(3)
                                    + -700. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-420. * param.s1.powi(5)
                                    + 1525. * param.s1.powi(4) * param.s2
                                    + 3060. * param.s1.powi(3) * param.s2.powi(2)
                                    + 14106. * param.s1.powi(2) * param.s2.powi(3)
                                    + -40524. * param.s1 * param.s2.powi(4)
                                    + 8925. * param.s2.powi(5))
                            + 2. * param.s12
                                * (70. * param.s1.powi(6)
                                    + -565. * param.s1.powi(5) * param.s2
                                    + 1385. * param.s1.powi(4) * param.s2.powi(2)
                                    + -15038. * param.s1.powi(3) * param.s2.powi(3)
                                    + 9036. * param.s1.powi(2) * param.s2.powi(4)
                                    + 8787. * param.s1 * param.s2.powi(5)
                                    + -3675. * param.s2.powi(6))
                            - (param.s1 - param.s2).powi(3)
                                * (20. * param.s1.powi(4)
                                    + -215. * param.s1.powi(3) * param.s2
                                    + 1449. * param.s1.powi(2) * param.s2.powi(2)
                                    + 8055. * param.s1 * param.s2.powi(3)
                                    + 2031. * param.s2.powi(4)))
                        + 3. * param.m2_2
                            * param.s2.powi(2)
                            * (60. * param.s12.powi(7)
                                + -7. * param.s12.powi(6) * (28. * param.s1 + -151. * param.s2)
                                + 6. * param.s12.powi(5)
                                    * (14. * param.s1.powi(2)
                                        + 615. * param.s1 * param.s2
                                        + -595. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (84. * param.s1.powi(3)
                                        + -2773. * param.s1.powi(2) * param.s2
                                        + 2302. * param.s1 * param.s2.powi(2)
                                        + 511. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (42. * param.s1.powi(3)
                                        + 1793. * param.s1.powi(2) * param.s2
                                        + 3366. * param.s1 * param.s2.powi(2)
                                        + 539. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (175. * param.s1.powi(4)
                                        + -2239. * param.s1.powi(3) * param.s2
                                        + -4743. * param.s1.powi(2) * param.s2.powi(2)
                                        + 7519. * param.s1 * param.s2.powi(3)
                                        + -560. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (420. * param.s1.powi(5)
                                        + 3567. * param.s1.powi(4) * param.s2
                                        + -31056. * param.s1.powi(3) * param.s2.powi(2)
                                        + 18826. * param.s1.powi(2) * param.s2.powi(3)
                                        + 11708. * param.s1 * param.s2.powi(4)
                                        + -3465. * param.s2.powi(5))
                                + -28.
                                    * param.m1_2
                                    * (8. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-36. * param.s1 + 240. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (12. * param.s1.powi(2)
                                                + -43. * param.s1 * param.s2
                                                + -61. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(3)
                                                + -61. * param.s1.powi(2) * param.s2
                                                + -332. * param.s1 * param.s2.powi(2)
                                                + -52. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (10. * param.s1.powi(3)
                                                + 179. * param.s1.powi(2) * param.s2
                                                + -559. * param.s1 * param.s2.powi(2)
                                                + 130. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * param.s2
                                            * (519. * param.s1.powi(3)
                                                + -687. * param.s1.powi(2) * param.s2
                                                + -659. * param.s1 * param.s2.powi(2)
                                                + 435. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(4)
                                            * (4. * param.s1.powi(2)
                                                + 95. * param.s1 * param.s2
                                                + 85. * param.s2.powi(2)))
                                + 28.
                                    * param.m1_2.powi(2)
                                    * (6. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-30. * param.s1 + 255. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (30. * param.s1.powi(2)
                                                + -304. * param.s1 * param.s2
                                                + 75. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (-60. * param.s1.powi(3)
                                                + 294. * param.s1.powi(2) * param.s2
                                                + 939. * param.s1 * param.s2.powi(2)
                                                + -865. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (15. * param.s1.powi(4)
                                                + 108. * param.s1.powi(3) * param.s2
                                                + -686. * param.s1.powi(2) * param.s2.powi(2)
                                                + 448. * param.s1 * param.s2.powi(3)
                                                + 115. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (6. * param.s1.powi(2)
                                                + 175. * param.s1 * param.s2
                                                + 224. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(5)
                                    * (4. * param.s1.powi(2)
                                        + 91. * param.s1 * param.s2
                                        + 45. * param.s2.powi(2)))
                        + param.s2.powi(3)
                            * (-638. * param.s12.powi(7)
                                + 707. * param.s12.powi(6) * (param.s1 + param.s2)
                                + 15. * (param.s1 - param.s2).powi(6) * (param.s1 + param.s2)
                                + -12.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (21. * param.s1.powi(2)
                                        + 58. * param.s1 * param.s2
                                        + 21. * param.s2.powi(2))
                                + 16.
                                    * param.s12.powi(5)
                                    * (210. * param.s1.powi(2)
                                        + -953. * param.s1 * param.s2
                                        + 210. * param.s2.powi(2))
                                + -85.
                                    * param.s12.powi(4)
                                    * (91. * param.s1.powi(3)
                                        + -199. * param.s1.powi(2) * param.s2
                                        + -199. * param.s1 * param.s2.powi(2)
                                        + 91. * param.s2.powi(3))
                                + -3.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (329. * param.s1.powi(3)
                                        + 4611. * param.s1.powi(2) * param.s2
                                        + 4611. * param.s1 * param.s2.powi(2)
                                        + 329. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (5530. * param.s1.powi(4)
                                        + 9248. * param.s1.powi(3) * param.s2
                                        + -44916. * param.s1.powi(2) * param.s2.powi(2)
                                        + 9248. * param.s1 * param.s2.powi(3)
                                        + 5530. * param.s2.powi(4))
                                + 140.
                                    * param.m1_2.powi(3)
                                    * (19. * param.s12.powi(4)
                                        + (param.s1 - param.s2).powi(3)
                                            * (19. * param.s1 + 62. * param.s2)
                                        + param.s12.powi(3)
                                            * (-76. * param.s1 + 170. * param.s2)
                                        + param.s12.powi(2)
                                            * (114. * param.s1.powi(2)
                                                + -335. * param.s1 * param.s2
                                                + 81. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (19. * param.s1.powi(3)
                                                + -40. * param.s1.powi(2) * param.s2
                                                + -31. * param.s1 * param.s2.powi(2)
                                                + 52. * param.s2.powi(3)))
                                + -84.
                                    * param.m1_2.powi(2)
                                    * (69. * param.s12.powi(5)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (50. * param.s1 + -77. * param.s2)
                                        + 2. * (param.s1 - param.s2).powi(4)
                                            * (13. * param.s1 + 33. * param.s2)
                                        + param.s12.powi(3)
                                            * (310. * param.s1.powi(2)
                                                + -242. * param.s1 * param.s2
                                                + -460. * param.s2.powi(2))
                                        - param.s12.powi(2)
                                            * (120. * param.s1.powi(3)
                                                + 709. * param.s1.powi(2) * param.s2
                                                + -1666. * param.s1 * param.s2.powi(2)
                                                + 445. * param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (35. * param.s1.powi(2)
                                                + -534. * param.s1 * param.s2
                                                + -385. * param.s2.powi(2)))
                                + 21.
                                    * param.m1_2
                                    * (179. * param.s12.powi(6)
                                        + (param.s1 - param.s2).powi(5)
                                            * (7. * param.s1 + 13. * param.s2)
                                        + param.s12.powi(5)
                                            * (-522. * param.s1 + 398. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (61. * param.s1.powi(2)
                                                + 374. * param.s1 * param.s2
                                                + -351. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (83. * param.s1.powi(2)
                                                + 500. * param.s1 * param.s2
                                                + 237. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (105. * param.s1.powi(3)
                                                + -1119. * param.s1.powi(2) * param.s2
                                                + 651. * param.s1 * param.s2.powi(2)
                                                + 275. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-555. * param.s1.powi(4)
                                                + 1728. * param.s1.powi(3) * param.s2
                                                + 2694. * param.s1.powi(2) * param.s2.powi(2)
                                                + -4432. * param.s1 * param.s2.powi(3)
                                                + 565. * param.s2.powi(4))))
                        + -3.
                            * param.m2_2.powi(2)
                            * param.s2
                            * (-18. * param.s12.powi(7)
                                + 7. * param.s12.powi(6) * (13. * param.s1 + 45. * param.s2)
                                + -12.
                                    * param.s12.powi(5)
                                    * (14. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + -63. * param.s2.powi(2))
                                + 5. * param.s12.powi(4)
                                    * (21. * param.s1.powi(3)
                                        + -209. * param.s1.powi(2) * param.s2
                                        + 2451. * param.s1 * param.s2.powi(2)
                                        + -1043. * param.s2.powi(3))
                                + 4. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (21. * param.s1.powi(4)
                                        + -38. * param.s1.powi(3) * param.s2
                                        + 2897. * param.s1.powi(2) * param.s2.powi(2)
                                        + 3448. * param.s1 * param.s2.powi(3)
                                        + -140. * param.s2.powi(4))
                                + 2. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + 1220. * param.s1.powi(3) * param.s2
                                        + -6574. * param.s1.powi(2) * param.s2.powi(2)
                                        + -4912. * param.s1 * param.s2.powi(3)
                                        + 3815. * param.s2.powi(4))
                                + 7. * param.m1_2
                                    * (5. * param.s12.powi(6)
                                        + -6.
                                            * param.s12.powi(5)
                                            * (5. * param.s1 + 17. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (15. * param.s1.powi(2)
                                                + 62. * param.s1 * param.s2
                                                + -141. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (5. * param.s1.powi(3)
                                                + -83. * param.s1.powi(2) * param.s2
                                                + -965. * param.s1 * param.s2.powi(2)
                                                + -577. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (25. * param.s1.powi(3)
                                                + 55. * param.s1.powi(2) * param.s2
                                                + 331. * param.s1 * param.s2.powi(2)
                                                + -525. * param.s2.powi(3))
                                        + 3. * param.s12.powi(2)
                                            * (25. * param.s1.powi(4)
                                                + -60. * param.s1.powi(3) * param.s2
                                                + 1354. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1056. * param.s1 * param.s2.powi(3)
                                                + -375. * param.s2.powi(4))
                                        + -2.
                                            * param.s12
                                            * (15. * param.s1.powi(5)
                                                + -145. * param.s1.powi(4) * param.s2
                                                + 666. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1578. * param.s1.powi(2) * param.s2.powi(3)
                                                + -2489. * param.s1 * param.s2.powi(4)
                                                + 375. * param.s2.powi(5)))
                                - param.s12.powi(2)
                                    * (147. * param.s1.powi(5)
                                        + 1215. * param.s1.powi(4) * param.s2
                                        + 12666. * param.s1.powi(3) * param.s2.powi(2)
                                        + -44874. * param.s1.powi(2) * param.s2.powi(3)
                                        + 16363. * param.s1 * param.s2.powi(4)
                                        + 3507. * param.s2.powi(5))
                                - (param.s1 - param.s2).powi(4)
                                    * (17. * param.s1.powi(3)
                                        + -213. * param.s1.powi(2) * param.s2
                                        + -1781. * param.s1 * param.s2.powi(2)
                                        + -599. * param.s2.powi(3))))
                + param.m0_2.powi(2)
                    * (-3.
                        * param.m2_2.powi(4)
                        * (10. * param.s12.powi(7)
                            + -35. * param.s12.powi(6) * (2. * param.s1 + 3. * param.s2)
                            + param.s12.powi(5)
                                * (210. * param.s1.powi(2)
                                    + 370. * param.s1 * param.s2
                                    + 504. * param.s2.powi(2))
                            + -5.
                                * param.s12.powi(4)
                                * (70. * param.s1.powi(3)
                                    + 55. * param.s1.powi(2) * param.s2
                                    + -36. * param.s1 * param.s2.powi(2)
                                    + 259. * param.s2.powi(3))
                            + 2. * param.s12.powi(3)
                                * (175. * param.s1.powi(4)
                                    + -250. * param.s1.powi(3) * param.s2
                                    + -1030. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1374. * param.s1 * param.s2.powi(3)
                                    + 945. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-210. * param.s1.powi(5)
                                    + 1025. * param.s1.powi(4) * param.s2
                                    + 60. * param.s1.powi(3) * param.s2.powi(2)
                                    + 17406. * param.s1.powi(2) * param.s2.powi(3)
                                    + -11974. * param.s1 * param.s2.powi(4)
                                    + -1575. * param.s2.powi(5))
                            + 2. * param.s12
                                * (35. * param.s1.powi(6)
                                    + -335. * param.s1.powi(5) * param.s2
                                    + 1410. * param.s1.powi(4) * param.s2.powi(2)
                                    + -5678. * param.s1.powi(3) * param.s2.powi(3)
                                    + -7599. * param.s1.powi(2) * param.s2.powi(4)
                                    + 6777. * param.s1 * param.s2.powi(5)
                                    + 350. * param.s2.powi(6))
                            - (param.s1 - param.s2).powi(2)
                                * (10. * param.s1.powi(5)
                                    + -135. * param.s1.powi(4) * param.s2
                                    + 1224. * param.s1.powi(3) * param.s2.powi(2)
                                    + 10086. * param.s1.powi(2) * param.s2.powi(3)
                                    + 5066. * param.s1 * param.s2.powi(4)
                                    + 129. * param.s2.powi(5)))
                        + param.s2.powi(4)
                            * (-226. * param.s12.powi(7)
                                + -7. * param.s12.powi(6) * (848. * param.s1 + -157. * param.s2)
                                + -5.
                                    * (param.s1 - param.s2).powi(6)
                                    * (8. * param.s1 + param.s2)
                                + param.s12.powi(5)
                                    * (9450. * param.s1.powi(2)
                                        + 7094. * param.s1 * param.s2
                                        + -2100. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (413. * param.s1.powi(2)
                                        + 459. * param.s1 * param.s2
                                        + 28. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (7476. * param.s1.powi(3)
                                        + 28229. * param.s1.powi(2) * param.s2
                                        + 8734. * param.s1 * param.s2.powi(2)
                                        + 21. * param.s2.powi(3))
                                + 5. * param.s12.powi(4)
                                    * (1764. * param.s1.powi(3)
                                        + -10559. * param.s1.powi(2) * param.s2
                                        + 2734. * param.s1 * param.s2.powi(2)
                                        + 385. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-20370. * param.s1.powi(4)
                                        + 33476. * param.s1.powi(3) * param.s2
                                        + 34908. * param.s1.powi(2) * param.s2.powi(2)
                                        + -24204. * param.s1 * param.s2.powi(3)
                                        + -770. * param.s2.powi(4))
                                + -420.
                                    * param.m1_2.powi(4)
                                    * (41. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (-123. * param.s1 + 112. * param.s2)
                                        + param.s12
                                            * (123. * param.s1.powi(2)
                                                + -118. * param.s1 * param.s2
                                                + -77. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (41. * param.s1 + 76. * param.s2))
                                + 280.
                                    * param.m1_2.powi(3)
                                    * (146. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (-338. * param.s1 + 124. * param.s2)
                                        + param.s12.powi(2)
                                            * (138. * param.s1.powi(2)
                                                + 545. * param.s1 * param.s2
                                                + -543. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (77. * param.s1.powi(3)
                                                + -413. * param.s1.powi(2) * param.s2
                                                + 271. * param.s1 * param.s2.powi(2)
                                                + 65. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (100. * param.s1 + 143. * param.s2))
                                + -168.
                                    * param.m1_2.powi(2)
                                    * (184. * param.s12.powi(5)
                                        + -190. * param.s12.powi(4) * (param.s1 + param.s2)
                                        + -69.
                                            * (param.s1 - param.s2).powi(4)
                                            * (param.s1 + param.s2)
                                        + param.s12.powi(3)
                                            * (-465. * param.s1.powi(2)
                                                + 1678. * param.s1 * param.s2
                                                + -465. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (695. * param.s1.powi(3)
                                                + -989. * param.s1.powi(2) * param.s2
                                                + -989. * param.s1 * param.s2.powi(2)
                                                + 695. * param.s2.powi(3))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (155. * param.s1.powi(2)
                                                + 1016. * param.s1 * param.s2
                                                + 155. * param.s2.powi(2)))
                                + 42.
                                    * param.m1_2
                                    * (179. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (398. * param.s1 + -522. * param.s2)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (351. * param.s1.powi(2)
                                                + -374. * param.s1 * param.s2
                                                + -61. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (237. * param.s1.powi(2)
                                                + 500. * param.s1 * param.s2
                                                + 83. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (275. * param.s1.powi(3)
                                                + 651. * param.s1.powi(2) * param.s2
                                                + -1119. * param.s1 * param.s2.powi(2)
                                                + 105. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (565. * param.s1.powi(4)
                                                + -4432. * param.s1.powi(3) * param.s2
                                                + 2694. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1728. * param.s1 * param.s2.powi(3)
                                                + -555. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(5)
                                            * (13. * param.s1 + 7. * param.s2)))
                        + -6.
                            * param.m2_2
                            * param.s2.powi(3)
                            * (-60. * param.s12.powi(7)
                                + -7. * param.s12.powi(6) * (151. * param.s1 + -28. * param.s2)
                                + 6. * param.s12.powi(5)
                                    * (595. * param.s1.powi(2)
                                        + -615. * param.s1 * param.s2
                                        + -14. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (539. * param.s1.powi(3)
                                        + 3366. * param.s1.powi(2) * param.s2
                                        + 1793. * param.s1 * param.s2.powi(2)
                                        + 42. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(4)
                                    * (511. * param.s1.powi(3)
                                        + 2302. * param.s1.powi(2) * param.s2
                                        + -2773. * param.s1 * param.s2.powi(2)
                                        + 84. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (560. * param.s1.powi(4)
                                        + -7519. * param.s1.powi(3) * param.s2
                                        + 4743. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2239. * param.s1 * param.s2.powi(3)
                                        + -175. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (3465. * param.s1.powi(5)
                                        + -11708. * param.s1.powi(4) * param.s2
                                        + -18826. * param.s1.powi(3) * param.s2.powi(2)
                                        + 31056. * param.s1.powi(2) * param.s2.powi(3)
                                        + -3567. * param.s1 * param.s2.powi(4)
                                        + -420. * param.s2.powi(5))
                                + 140.
                                    * param.m1_2.powi(3)
                                    * (19. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (-76. * param.s1 + 88. * param.s2)
                                        + param.s12.powi(2)
                                            * (114. * param.s1.powi(2)
                                                + -89. * param.s1 * param.s2
                                                + -143. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (19. * param.s1.powi(2)
                                                + 125. * param.s1 * param.s2
                                                + 90. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (38. * param.s1.powi(3)
                                                + 43. * param.s1.powi(2) * param.s2
                                                + -180. * param.s1 * param.s2.powi(2)
                                                + 27. * param.s2.powi(3)))
                                + -28.
                                    * param.m1_2.powi(2)
                                    * (144. * param.s12.powi(5)
                                        + -5.
                                            * param.s12.powi(4)
                                            * (87. * param.s1 + -59. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (150. * param.s1.powi(2)
                                                + 724. * param.s1 * param.s2
                                                + -695. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (141. * param.s1.powi(2)
                                                + 715. * param.s1 * param.s2
                                                + 359. * param.s2.powi(2))
                                        + param.s12.powi(2)
                                            * (270. * param.s1.powi(3)
                                                + -3489. * param.s1.powi(2) * param.s2
                                                + 1951. * param.s1 * param.s2.powi(2)
                                                + 960. * param.s2.powi(3))
                                        + param.s12
                                            * (-420. * param.s1.powi(4)
                                                + 1454. * param.s1.powi(3) * param.s2
                                                + 1942. * param.s1.powi(2) * param.s2.powi(2)
                                                + -3326. * param.s1 * param.s2.powi(3)
                                                + 350. * param.s2.powi(4)))
                                + 7. * param.m1_2
                                    * (211. * param.s12.powi(6)
                                        + -114. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + -15.
                                            * param.s12.powi(4)
                                            * (97. * param.s1.powi(2)
                                                + -374. * param.s1 * param.s2
                                                + 97. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (199. * param.s1.powi(2)
                                                + 706. * param.s1 * param.s2
                                                + 199. * param.s2.powi(2))
                                        + -6.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (11. * param.s1.powi(3)
                                                + -895. * param.s1.powi(2) * param.s2
                                                + -895. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3))
                                        + 4. * param.s12.powi(3)
                                            * (685. * param.s1.powi(3)
                                                + -1389. * param.s1.powi(2) * param.s2
                                                + -1389. * param.s1 * param.s2.powi(2)
                                                + 685. * param.s2.powi(3))
                                        + -3.
                                            * param.s12.powi(2)
                                            * (505. * param.s1.powi(4)
                                                + 1784. * param.s1.powi(3) * param.s2
                                                + -6146. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1784. * param.s1 * param.s2.powi(3)
                                                + 505. * param.s2.powi(4)))
                                - (param.s1 - param.s2).powi(5)
                                    * (45. * param.s1.powi(2)
                                        + 91. * param.s1 * param.s2
                                        + 4. * param.s2.powi(2)))
                        + 18.
                            * param.m2_2.powi(2)
                            * param.s2.powi(2)
                            * (-4. * param.s12.powi(7)
                                + -35. * param.s12.powi(6) * (param.s1 + param.s2)
                                + 6. * param.s12.powi(5)
                                    * (35. * param.s1.powi(2)
                                        + -286. * param.s1 * param.s2
                                        + 35. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(4)
                                    * (25. * param.s1.powi(3)
                                        + 619. * param.s1.powi(2) * param.s2
                                        + 619. * param.s1 * param.s2.powi(2)
                                        + 25. * param.s2.powi(3))
                                + -5.
                                    * param.s12.powi(4)
                                    * (77. * param.s1.powi(3)
                                        + -393. * param.s1.powi(2) * param.s2
                                        + -393. * param.s1 * param.s2.powi(2)
                                        + 77. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (35. * param.s1.powi(4)
                                        + -848. * param.s1.powi(3) * param.s2
                                        + -4562. * param.s1.powi(2) * param.s2.powi(2)
                                        + -848. * param.s1 * param.s2.powi(3)
                                        + 35. * param.s2.powi(4))
                                + 8. * param.s12.powi(3)
                                    * (35. * param.s1.powi(4)
                                        + 529. * param.s1.powi(3) * param.s2
                                        + -1962. * param.s1.powi(2) * param.s2.powi(2)
                                        + 529. * param.s1 * param.s2.powi(3)
                                        + 35. * param.s2.powi(4))
                                + -28.
                                    * param.m1_2.powi(2)
                                    * (3. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-15. * param.s1 + 80. * param.s2)
                                        + param.s12.powi(3)
                                            * (30. * param.s1.powi(2)
                                                + -114. * param.s1 * param.s2
                                                + -145. * param.s2.powi(2))
                                        + param.s12
                                            * (15. * param.s1.powi(4)
                                                + 298. * param.s1.powi(3) * param.s2
                                                + -471. * param.s1.powi(2) * param.s2.powi(2)
                                                + -452. * param.s1 * param.s2.powi(3)
                                                + 250. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(2)
                                            * (3. * param.s1.powi(3)
                                                + 132. * param.s1.powi(2) * param.s2
                                                + 337. * param.s1 * param.s2.powi(2)
                                                + 113. * param.s2.powi(3))
                                        - param.s12.powi(2)
                                            * (30. * param.s1.powi(3)
                                                + 138. * param.s1.powi(2) * param.s2
                                                + -692. * param.s1 * param.s2.powi(2)
                                                + 75. * param.s2.powi(3)))
                                + 7. * param.m1_2
                                    * (9. * param.s12.powi(6)
                                        + -6.
                                            * param.s12.powi(5)
                                            * (5. * param.s1 + -27. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (3. * param.s1.powi(2)
                                                + 106. * param.s1 * param.s2
                                                + -113. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (15. * param.s1.powi(3)
                                                + -523. * param.s1.powi(2) * param.s2
                                                + 423. * param.s1 * param.s2.powi(2)
                                                + 115. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-105. * param.s1.powi(4)
                                                + 1476. * param.s1.powi(3) * param.s2
                                                + 2718. * param.s1.powi(2) * param.s2.powi(2)
                                                + -4456. * param.s1 * param.s2.powi(3)
                                                + 255. * param.s2.powi(4))
                                        + 2. * param.s12
                                            * (33. * param.s1.powi(5)
                                                + 197. * param.s1.powi(4) * param.s2
                                                + -2214. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1298. * param.s1.powi(2) * param.s2.powi(3)
                                                + 933. * param.s1 * param.s2.powi(4)
                                                + -247. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(3)
                                            * (15. * param.s1.powi(3)
                                                + 515. * param.s1.powi(2) * param.s2
                                                + 917. * param.s1 * param.s2.powi(2)
                                                + 173. * param.s2.powi(3)))
                                - param.s12.powi(2)
                                    * (21. * param.s1.powi(5)
                                        + 6801. * param.s1.powi(4) * param.s2
                                        + -9566. * param.s1.powi(3) * param.s2.powi(2)
                                        + -9566. * param.s1.powi(2) * param.s2.powi(3)
                                        + 6801. * param.s1 * param.s2.powi(4)
                                        + 21. * param.s2.powi(5)))
                        + 2. * param.m2_2.powi(3)
                            * param.s2
                            * (-16. * param.s12.powi(7)
                                + 7. * param.s12.powi(6) * (param.s1 + 28. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (147. * param.s1.powi(2)
                                        + 691. * param.s1 * param.s2
                                        + -168. * param.s2.powi(2))
                                + -5.
                                    * param.s12.powi(4)
                                    * (203. * param.s1.powi(3)
                                        + 1130. * param.s1.powi(2) * param.s2
                                        + -3469. * param.s1 * param.s2.powi(2)
                                        + 140. * param.s2.powi(3))
                                + 4. * param.s12.powi(3)
                                    * (385. * param.s1.powi(4)
                                        + 1215. * param.s1.powi(3) * param.s2
                                        + 5661. * param.s1.powi(2) * param.s2.powi(2)
                                        + -12183. * param.s1 * param.s2.powi(3)
                                        + 700. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-1239. * param.s1.powi(5)
                                        + 2560. * param.s1.powi(4) * param.s2
                                        + -81702. * param.s1.powi(3) * param.s2.powi(2)
                                        + 68952. * param.s1.powi(2) * param.s2.powi(3)
                                        + 28201. * param.s1 * param.s2.powi(4)
                                        + -3444. * param.s2.powi(5))
                                + 2. * param.s12
                                    * (259. * param.s1.powi(6)
                                        + -2593. * param.s1.powi(5) * param.s2
                                        + 13886. * param.s1.powi(4) * param.s2.powi(2)
                                        + 32966. * param.s1.powi(3) * param.s2.powi(3)
                                        + -52201. * param.s1.powi(2) * param.s2.powi(4)
                                        + 6731. * param.s1 * param.s2.powi(5)
                                        + 952. * param.s2.powi(6))
                                + 21.
                                    * param.m1_2
                                    * (5. * param.s12.powi(6)
                                        + -6.
                                            * param.s12.powi(5)
                                            * (5. * param.s1 + 13. * param.s2)
                                        + 5. * param.s12.powi(4)
                                            * (15. * param.s1.powi(2)
                                                + 38. * param.s1 * param.s2
                                                + -13. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (25. * param.s1.powi(3)
                                                + -5. * param.s1.powi(2) * param.s2
                                                + 559. * param.s1 * param.s2.powi(2)
                                                + -235. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (75. * param.s1.powi(4)
                                                + -420. * param.s1.powi(3) * param.s2
                                                + 2958. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2368. * param.s1 * param.s2.powi(3)
                                                + -1725. * param.s2.powi(4))
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1.powi(4)
                                                + -112. * param.s1.powi(3) * param.s2
                                                + -1938. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2308. * param.s1 * param.s2.powi(3)
                                                + -327. * param.s2.powi(4))
                                        + 2. * param.s12
                                            * (-15. * param.s1.powi(5)
                                                + 205. * param.s1.powi(4) * param.s2
                                                + 526. * param.s1.powi(3) * param.s2.powi(2)
                                                + -3462. * param.s1.powi(2) * param.s2.powi(3)
                                                + 681. * param.s1 * param.s2.powi(4)
                                                + 625. * param.s2.powi(5)))
                                - (param.s1 - param.s2).powi(3)
                                    * (89. * param.s1.powi(4)
                                        + -1571. * param.s1.powi(3) * param.s2
                                        + -19257. * param.s1.powi(2) * param.s2.powi(2)
                                        + -12877. * param.s1 * param.s2.powi(3)
                                        + -404. * param.s2.powi(4)))))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 420.
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
                    * (-18.
                        * param.m1_2.powi(3)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(3)
                            * (5. * param.s12.powi(3)
                                + 3. * (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-7. * param.s1 + 5. * param.s2)
                                - param.s12
                                    * (param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2)))
                        + 2. * param.m1_2.powi(2)
                            * param.s2
                            * (param.s2
                                * (-17. * param.s1.powi(2)
                                    + 7. * param.s1 * param.s12
                                    + 10. * param.s12.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + -20. * param.s12 * param.s2
                                    + 10. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (5. * param.s1.powi(2)
                                        + 5. * param.s12.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2)
                                        + -10. * param.s12 * (param.s1 + param.s2)))
                        + param.s1
                            * (-4.
                                * param.m2_2.powi(3)
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + 2. * param.s2.powi(2)
                                    + -4. * param.s12 * (param.s1 + param.s2))
                                + param.m2_2.powi(2)
                                    * (-5. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + -25. * param.s1.powi(2) * param.s2
                                        + 11. * param.s1 * param.s2.powi(2)
                                        + 19. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-15. * param.s1 + 9. * param.s2)
                                        + param.s12
                                            * (15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -33. * param.s2.powi(2)))
                                + -2.
                                    * param.m2_2
                                    * param.s2
                                    * (5. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1 + 7. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        - param.s12.powi(2) * (5. * param.s1 + 3. * param.s2))
                                + param.s2.powi(2)
                                    * (5. * param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -7. * param.s2)
                                        + -3. * (param.s1 - param.s2).powi(3)
                                        - param.s12
                                            * (7. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + param.s2.powi(2))))
                        + param.m0_2.powi(2)
                            * (-3. * param.s1.powi(4)
                                + 7. * param.s1.powi(3) * param.s12
                                + -3. * param.s1.powi(2) * param.s12.powi(2)
                                + -3. * param.s1 * param.s12.powi(3)
                                + 2. * param.s12.powi(4)
                                + 3. * param.s1.powi(3) * param.s2
                                + -32. * param.s1.powi(2) * param.s12 * param.s2
                                + 21. * param.s1 * param.s12.powi(2) * param.s2
                                + 8. * param.s12.powi(3) * param.s2
                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                + 25. * param.s1 * param.s12 * param.s2.powi(2)
                                + -16. * param.s12.powi(2) * param.s2.powi(2)
                                + -15. * param.s1 * param.s2.powi(3)
                                + 6. * param.s2.powi(4)
                                + -2.
                                    * param.m2_2
                                    * (5. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2)
                                            * (7. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (-9. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s12.powi(2) * (3. * param.s1 + 5. * param.s2))
                                + param.m1_2
                                    * (-5. * param.s12.powi(3)
                                        + 5. * param.s12.powi(2)
                                            * (3. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1 + 19. * param.s2)
                                        + param.s12
                                            * (-15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 11. * param.s2.powi(2))))
                        + param.m1_2
                            * (param.s2.powi(2)
                                * (-5. * param.s12.powi(3)
                                    + -5. * param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (19. * param.s1 + 5. * param.s2)
                                    + param.s12
                                        * (11. * param.s1.powi(2)
                                            + 16. * param.s1 * param.s2
                                            + -15. * param.s2.powi(2)))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (15. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 5. * param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 19. * param.s1.powi(2) * param.s2
                                        + -29. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-25. * param.s1.powi(2)
                                                + 24. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + param.m2_2.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -5. * param.s12.powi(3)
                                        + 49. * param.s1.powi(2) * param.s2
                                        + 49. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)
                                        + 15. * param.s12.powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (15. * param.s1.powi(2)
                                                + 64. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2))))
                        + param.m0_2
                            * (2.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (10. * param.s1.powi(2)
                                    + -20. * param.s1 * param.s12
                                    + 10. * param.s12.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + 7. * param.s12 * param.s2
                                    + -17. * param.s2.powi(2))
                                + param.s2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (8. * param.s1 + -3. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-16. * param.s1.powi(2)
                                                + 21. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + param.s12
                                            * param.s2
                                            * (25. * param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + 7. * param.s2.powi(2)))
                                + param.m2_2.powi(2)
                                    * (19. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (3. * param.s1 + -5. * param.s2)
                                        + 11. * param.s1.powi(2) * param.s2
                                        + -25. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-33. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + -2.
                                    * param.m2_2
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (param.s1 + param.s2)
                                        + -4.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (11. * param.s1.powi(3)
                                                + -17. * param.s1.powi(2) * param.s2
                                                + -17. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3)))
                                + 2. * param.m1_2
                                    * (param.m2_2
                                        * (-5. * param.s1.powi(3)
                                            + 5. * param.s12.powi(3)
                                            + -29. * param.s1.powi(2) * param.s2
                                            + 19. * param.s1 * param.s2.powi(2)
                                            + 15. * param.s2.powi(3)
                                            + 5. * param.s12.powi(2)
                                                * (-3. * param.s1 + param.s2)
                                            + param.s12
                                                * (15. * param.s1.powi(2)
                                                    + 24. * param.s1 * param.s2
                                                    + -25. * param.s2.powi(2)))
                                        + -4.
                                            * param.s2
                                            * (2. * param.s12.powi(3)
                                                + 3. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (-4. * param.s1.powi(2)
                                                        + 11. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2)))))
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.0000992063492063492
            * std::f64::consts::PI
            * param.s12.powi(-5)
            * param.lambda_s12_sqrt.powi(-11)
            * ((12. * param.m2_2.powi(6) * param.s1.powi(9)
                + -132. * param.m2_2.powi(6) * param.s1.powi(8) * param.s12
                + -51. * param.m2_2.powi(5) * param.s1.powi(9) * param.s12
                + 680. * param.m2_2.powi(6) * param.s1.powi(7) * param.s12.powi(2)
                + 582. * param.m2_2.powi(5) * param.s1.powi(8) * param.s12.powi(2)
                + 75. * param.m2_2.powi(4) * param.s1.powi(9) * param.s12.powi(2)
                + -2240. * param.m2_2.powi(6) * param.s1.powi(6) * param.s12.powi(3)
                + -3205. * param.m2_2.powi(5) * param.s1.powi(7) * param.s12.powi(3)
                + -930. * param.m2_2.powi(4) * param.s1.powi(8) * param.s12.powi(3)
                + -30. * param.m2_2.powi(3) * param.s1.powi(9) * param.s12.powi(3)
                + 5880. * param.m2_2.powi(6) * param.s1.powi(5) * param.s12.powi(4)
                + 12460. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12.powi(4)
                + 6245. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12.powi(4)
                + 540. * param.m2_2.powi(3) * param.s1.powi(8) * param.s12.powi(4)
                + -30. * param.m2_2.powi(2) * param.s1.powi(9) * param.s12.powi(4)
                + -5712. * param.m2_2.powi(6) * param.s1.powi(4) * param.s12.powi(5)
                + -19971. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12.powi(5)
                + -15218. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12.powi(5)
                + -2155. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12.powi(5)
                + 120. * param.m2_2.powi(2) * param.s1.powi(8) * param.s12.powi(5)
                + 33. * param.m2_2 * param.s1.powi(9) * param.s12.powi(5)
                + -168. * param.m2_2.powi(6) * param.s1.powi(3) * param.s12.powi(6)
                + 8946. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(6)
                + 13923. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(6)
                + 3682. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(6)
                + -55. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(6)
                + -258. * param.m2_2 * param.s1.powi(8) * param.s12.powi(6)
                + -9. * param.s1.powi(9) * param.s12.powi(6)
                + 1920. * param.m2_2.powi(6) * param.s1.powi(2) * param.s12.powi(7)
                + 5733. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(7)
                + -2877. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(7)
                + -518. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(7)
                + 890. * param.m2_2 * param.s1.powi(7) * param.s12.powi(7)
                + 78. * param.s1.powi(8) * param.s12.powi(7)
                + -260. * param.m2_2.powi(6) * param.s1 * param.s12.powi(8)
                + -5220. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(8)
                + -7665. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(8)
                + 560. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(8)
                + 1323. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(8)
                + -1778. * param.m2_2 * param.s1.powi(6) * param.s12.powi(8)
                + -300. * param.s1.powi(7) * param.s12.powi(8)
                + 20. * param.m2_2.powi(6) * param.s12.powi(9)
                + 790. * param.m2_2.powi(5) * param.s1 * param.s12.powi(9)
                + 4230. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(9)
                + 595. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(9)
                + -1540. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(9)
                + 2268. * param.m2_2 * param.s1.powi(5) * param.s12.powi(9)
                + 672. * param.s1.powi(6) * param.s12.powi(9)
                + -64. * param.m2_2.powi(5) * param.s12.powi(10)
                + -722. * param.m2_2.powi(4) * param.s1 * param.s12.powi(10)
                + -390. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(10)
                + 1015. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(10)
                + -1918. * param.m2_2 * param.s1.powi(4) * param.s12.powi(10)
                + -966. * param.s1.powi(5) * param.s12.powi(10)
                + 62. * param.m2_2.powi(4) * param.s12.powi(11)
                + 83. * param.m2_2.powi(3) * param.s1 * param.s12.powi(11)
                + -390. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(11)
                + 1078. * param.m2_2 * param.s1.powi(3) * param.s12.powi(11)
                + 924. * param.s1.powi(4) * param.s12.powi(11)
                + -8. * param.m2_2.powi(3) * param.s12.powi(12)
                + 83. * param.m2_2.powi(2) * param.s1 * param.s12.powi(12)
                + -390. * param.m2_2 * param.s1.powi(2) * param.s12.powi(12)
                + -588. * param.s1.powi(3) * param.s12.powi(12)
                + -8. * param.m2_2.powi(2) * param.s12.powi(13)
                + 83. * param.m2_2 * param.s1 * param.s12.powi(13)
                + 240. * param.s1.powi(2) * param.s12.powi(13)
                + -8. * param.m2_2 * param.s12.powi(14)
                + -57. * param.s1 * param.s12.powi(14)
                + 6. * param.s12.powi(15)
                + -108. * param.m2_2.powi(6) * param.s1.powi(8) * param.s2
                + 810. * param.m2_2.powi(6) * param.s1.powi(7) * param.s12 * param.s2
                + 438. * param.m2_2.powi(5) * param.s1.powi(8) * param.s12 * param.s2
                + -2510. * param.m2_2.powi(6) * param.s1.powi(6) * param.s12.powi(2) * param.s2
                + -3320. * param.m2_2.powi(5) * param.s1.powi(7) * param.s12.powi(2) * param.s2
                + -570. * param.m2_2.powi(4) * param.s1.powi(8) * param.s12.powi(2) * param.s2
                + 3650. * param.m2_2.powi(6) * param.s1.powi(5) * param.s12.powi(3) * param.s2
                + 10230. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12.powi(3) * param.s2
                + 4240. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12.powi(3) * param.s2
                + 60. * param.m2_2.powi(3) * param.s1.powi(8) * param.s12.powi(3) * param.s2
                + 1130. * param.m2_2.powi(6) * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + -11050. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12.powi(4) * param.s2
                + -9930. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12.powi(4) * param.s2
                + 1090. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12.powi(4) * param.s2
                + 480. * param.m2_2.powi(2) * param.s1.powi(8) * param.s12.powi(4) * param.s2
                + 3818. * param.m2_2.powi(6) * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + 26540. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(5) * param.s2
                + 44978. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(5) * param.s2
                + 15270. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(5) * param.s2
                + 40. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(5) * param.s2
                + -402. * param.m2_2 * param.s1.powi(8) * param.s12.powi(5) * param.s2
                + -6430. * param.m2_2.powi(6) * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + -33268. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(6) * param.s2
                + -75100. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(6) * param.s2
                + -49312. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(6) * param.s2
                + -4680. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(6) * param.s2
                + 1930. * param.m2_2 * param.s1.powi(7) * param.s12.powi(6) * param.s2
                + 102. * param.s1.powi(8) * param.s12.powi(6) * param.s2
                + 1490. * param.m2_2.powi(6) * param.s1 * param.s12.powi(7) * param.s2
                + 14150. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(7) * param.s2
                + 39560. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(7) * param.s2
                + 44180. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(7) * param.s2
                + 8438. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(7) * param.s2
                + -3420. * param.m2_2 * param.s1.powi(6) * param.s12.powi(7) * param.s2
                + -590. * param.s1.powi(7) * param.s12.powi(7) * param.s2
                + -170. * param.m2_2.powi(6) * param.s12.powi(8) * param.s2
                + -4250. * param.m2_2.powi(5) * param.s1 * param.s12.powi(8) * param.s2
                + -6010. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(8) * param.s2
                + -6990. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(8) * param.s2
                + -4470. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(8) * param.s2
                + 2138. * param.m2_2 * param.s1.powi(5) * param.s12.powi(8) * param.s2
                + 1340. * param.s1.powi(6) * param.s12.powi(8) * param.s2
                + 530. * param.m2_2.powi(5) * param.s12.powi(9) * param.s2
                + 3310. * param.m2_2.powi(4) * param.s1 * param.s12.powi(9) * param.s2
                + -4610. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(9) * param.s2
                + -1180. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(9) * param.s2
                + 1200. * param.m2_2 * param.s1.powi(4) * param.s12.powi(9) * param.s2
                + -1362. * param.s1.powi(5) * param.s12.powi(9) * param.s2
                + -478. * param.m2_2.powi(4) * param.s12.powi(10) * param.s2
                + 300. * param.m2_2.powi(3) * param.s1 * param.s12.powi(10) * param.s2
                + 1900. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(10) * param.s2
                + -2818. * param.m2_2 * param.s1.powi(3) * param.s12.powi(10) * param.s2
                + 220. * param.s1.powi(4) * param.s12.powi(10) * param.s2
                + 12. * param.m2_2.powi(3) * param.s12.powi(11) * param.s2
                + -610. * param.m2_2.powi(2) * param.s1 * param.s12.powi(11) * param.s2
                + 1900. * param.m2_2 * param.s1.powi(2) * param.s12.powi(11) * param.s2
                + 878. * param.s1.powi(3) * param.s12.powi(11) * param.s2
                + 82. * param.m2_2.powi(2) * param.s12.powi(12) * param.s2
                + -610. * param.m2_2 * param.s1 * param.s12.powi(12) * param.s2
                + -900. * param.s1.powi(2) * param.s12.powi(12) * param.s2
                + 82. * param.m2_2 * param.s12.powi(13) * param.s2
                + 370. * param.s1 * param.s12.powi(13) * param.s2
                + -58. * param.s12.powi(14) * param.s2
                + 432. * param.m2_2.powi(6) * param.s1.powi(7) * param.s2.powi(2)
                + -1974. * param.m2_2.powi(6) * param.s1.powi(6) * param.s12 * param.s2.powi(2)
                + -1668. * param.m2_2.powi(5) * param.s1.powi(7) * param.s12 * param.s2.powi(2)
                + 2832.
                    * param.m2_2.powi(6)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 7406.
                    * param.m2_2.powi(5)
                    * param.s1.powi(6)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 1902.
                    * param.m2_2.powi(4)
                    * param.s1.powi(7)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 50.
                    * param.m2_2.powi(6)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -8753.
                    * param.m2_2.powi(5)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -6874.
                    * param.m2_2.powi(4)
                    * param.s1.powi(6)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 327.
                    * param.m2_2.powi(3)
                    * param.s1.powi(7)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -1920.
                    * param.m2_2.powi(6)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -9960.
                    * param.m2_2.powi(5)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -6233.
                    * param.m2_2.powi(4)
                    * param.s1.powi(5)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -8974.
                    * param.m2_2.powi(3)
                    * param.s1.powi(6)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -1773.
                    * param.m2_2.powi(2)
                    * param.s1.powi(7)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + 7002.
                    * param.m2_2.powi(6)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 20830.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 17970.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 16972.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 13916.
                    * param.m2_2.powi(2)
                    * param.s1.powi(6)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 2595. * param.m2_2 * param.s1.powi(7) * param.s12.powi(5) * param.s2.powi(2)
                + -3424. * param.m2_2.powi(6) * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + -8678.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + 17680.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + 46320.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + 7732.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + -4900. * param.m2_2 * param.s1.powi(6) * param.s12.powi(6) * param.s2.powi(2)
                + -555. * param.s1.powi(7) * param.s12.powi(6) * param.s2.powi(2)
                + 642. * param.m2_2.powi(6) * param.s12.powi(7) * param.s2.powi(2)
                + 8931. * param.m2_2.powi(5) * param.s1 * param.s12.powi(7) * param.s2.powi(2)
                + -6158.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(7)
                    * param.s2.powi(2)
                + -60965.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(7)
                    * param.s2.powi(2)
                + -46710.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(7)
                    * param.s2.powi(2)
                + -815. * param.m2_2 * param.s1.powi(5) * param.s12.powi(7) * param.s2.powi(2)
                + 1820. * param.s1.powi(6) * param.s12.powi(7) * param.s2.powi(2)
                + -1948. * param.m2_2.powi(5) * param.s12.powi(8) * param.s2.powi(2)
                + -5349. * param.m2_2.powi(4) * param.s1 * param.s12.powi(8) * param.s2.powi(2)
                + 8542.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(8)
                    * param.s2.powi(2)
                + 24015.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(8)
                    * param.s2.powi(2)
                + 5160. * param.m2_2 * param.s1.powi(4) * param.s12.powi(8) * param.s2.powi(2)
                + -1655. * param.s1.powi(5) * param.s12.powi(8) * param.s2.powi(2)
                + 1622. * param.m2_2.powi(4) * param.s12.powi(9) * param.s2.powi(2)
                + -2374. * param.m2_2.powi(3) * param.s1 * param.s12.powi(9) * param.s2.powi(2)
                + 1892.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(9)
                    * param.s2.powi(2)
                + -135. * param.m2_2 * param.s1.powi(3) * param.s12.powi(9) * param.s2.powi(2)
                + -230. * param.s1.powi(4) * param.s12.powi(9) * param.s2.powi(2)
                + 152. * param.m2_2.powi(3) * param.s12.powi(10) * param.s2.powi(2)
                + 1266. * param.m2_2.powi(2) * param.s1 * param.s12.powi(10) * param.s2.powi(2)
                + -3400. * param.m2_2 * param.s1.powi(2) * param.s12.powi(10) * param.s2.powi(2)
                + 495. * param.s1.powi(3) * param.s12.powi(10) * param.s2.powi(2)
                + -338. * param.m2_2.powi(2) * param.s12.powi(11) * param.s2.powi(2)
                + 1875. * param.m2_2 * param.s1 * param.s12.powi(11) * param.s2.powi(2)
                + 800. * param.s1.powi(2) * param.s12.powi(11) * param.s2.powi(2)
                + -380. * param.m2_2 * param.s12.powi(12) * param.s2.powi(2)
                + -925. * param.s1 * param.s12.powi(12) * param.s2.powi(2)
                + 250. * param.s12.powi(13) * param.s2.powi(2)
                + -1008. * param.m2_2.powi(6) * param.s1.powi(6) * param.s2.powi(3)
                + 2226. * param.m2_2.powi(6) * param.s1.powi(5) * param.s12 * param.s2.powi(3)
                + 3696. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12 * param.s2.powi(3)
                + -310.
                    * param.m2_2.powi(6)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -7308.
                    * param.m2_2.powi(5)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -3654.
                    * param.m2_2.powi(4)
                    * param.s1.powi(6)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + -840.
                    * param.m2_2.powi(6)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -1920.
                    * param.m2_2.powi(5)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + 3192.
                    * param.m2_2.powi(4)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -1554.
                    * param.m2_2.powi(3)
                    * param.s1.powi(6)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -1800.
                    * param.m2_2.powi(6)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 2100.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 14250.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 13692.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 2646.
                    * param.m2_2.powi(2)
                    * param.s1.powi(6)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 3762. * param.m2_2.powi(6) * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + -5580.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + -46200.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + -67160.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + -40278.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + -6006. * param.m2_2 * param.s1.powi(6) * param.s12.powi(5) * param.s2.powi(3)
                + -1414. * param.m2_2.powi(6) * param.s12.powi(6) * param.s2.powi(3)
                + -8264. * param.m2_2.powi(5) * param.s1 * param.s12.powi(6) * param.s2.powi(3)
                + 10590.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(3)
                + 34580.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(6)
                    * param.s2.powi(3)
                + 38050.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(6)
                    * param.s2.powi(3)
                + 19656. * param.m2_2 * param.s1.powi(5) * param.s12.powi(6) * param.s2.powi(3)
                + 2100. * param.s1.powi(6) * param.s12.powi(6) * param.s2.powi(3)
                + 4172. * param.m2_2.powi(5) * param.s12.powi(7) * param.s2.powi(3)
                + 2236. * param.m2_2.powi(4) * param.s1 * param.s12.powi(7) * param.s2.powi(3)
                + 4290.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(7)
                    * param.s2.powi(3)
                + 27860.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(7)
                    * param.s2.powi(3)
                + 5920. * param.m2_2 * param.s1.powi(4) * param.s12.powi(7) * param.s2.powi(3)
                + -2100. * param.s1.powi(5) * param.s12.powi(7) * param.s2.powi(3)
                + -3178. * param.m2_2.powi(4) * param.s12.powi(8) * param.s2.powi(3)
                + 4336. * param.m2_2.powi(3) * param.s1 * param.s12.powi(8) * param.s2.powi(3)
                + -8310.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(8)
                    * param.s2.powi(3)
                + -17920. * param.m2_2 * param.s1.powi(3) * param.s12.powi(8) * param.s2.powi(3)
                + -1430. * param.s1.powi(4) * param.s12.powi(8) * param.s2.powi(3)
                + -728. * param.m2_2.powi(3) * param.s12.powi(9) * param.s2.powi(3)
                + -214. * param.m2_2.powi(2) * param.s1 * param.s12.powi(9) * param.s2.powi(3)
                + 90. * param.m2_2 * param.s1.powi(2) * param.s12.powi(9) * param.s2.powi(3)
                + 420. * param.s1.powi(3) * param.s12.powi(9) * param.s2.powi(3)
                + 742. * param.m2_2.powi(2) * param.s12.powi(10) * param.s2.powi(3)
                + -2776. * param.m2_2 * param.s1 * param.s12.powi(10) * param.s2.powi(3)
                + 720. * param.s1.powi(2) * param.s12.powi(10) * param.s2.powi(3)
                + 1036. * param.m2_2 * param.s12.powi(11) * param.s2.powi(3)
                + 920. * param.s1 * param.s12.powi(11) * param.s2.powi(3)
                + -630. * param.s12.powi(12) * param.s2.powi(3)
                + 1512. * param.m2_2.powi(6) * param.s1.powi(5) * param.s2.powi(4)
                + -630. * param.m2_2.powi(6) * param.s1.powi(4) * param.s12 * param.s2.powi(4)
                + -5250. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12 * param.s2.powi(4)
                + -680.
                    * param.m2_2.powi(6)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + 910.
                    * param.m2_2.powi(5)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + 4452.
                    * param.m2_2.powi(4)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -680.
                    * param.m2_2.powi(6)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + 2785.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + 3850.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + 2877.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -1400. * param.m2_2.powi(6) * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + 4360.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + 895.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + -5600.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + -1323.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + 2002. * param.m2_2.powi(6) * param.s12.powi(5) * param.s2.powi(4)
                + 805. * param.m2_2.powi(5) * param.s1 * param.s12.powi(5) * param.s2.powi(4)
                + 2470.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(4)
                + 34425.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(4)
                + 35070.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(5)
                    * param.s2.powi(4)
                + 4725. * param.m2_2 * param.s1.powi(5) * param.s12.powi(5) * param.s2.powi(4)
                + -5740. * param.m2_2.powi(5) * param.s12.powi(6) * param.s2.powi(4)
                + 3745. * param.m2_2.powi(4) * param.s1 * param.s12.powi(6) * param.s2.powi(4)
                + -11530.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(4)
                + -47195.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(6)
                    * param.s2.powi(4)
                + -32130. * param.m2_2 * param.s1.powi(4) * param.s12.powi(6) * param.s2.powi(4)
                + -3213. * param.s1.powi(5) * param.s12.powi(6) * param.s2.powi(4)
                + 3962. * param.m2_2.powi(4) * param.s12.powi(7) * param.s2.powi(4)
                + -2380. * param.m2_2.powi(3) * param.s1 * param.s12.powi(7) * param.s2.powi(4)
                + 1070.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(7)
                    * param.s2.powi(4)
                + 6040. * param.m2_2 * param.s1.powi(3) * param.s12.powi(7) * param.s2.powi(4)
                + 4830. * param.s1.powi(4) * param.s12.powi(7) * param.s2.powi(4)
                + 1512. * param.m2_2.powi(3) * param.s12.powi(8) * param.s2.powi(4)
                + -2380. * param.m2_2.powi(2) * param.s1 * param.s12.powi(8) * param.s2.powi(4)
                + 4850. * param.m2_2 * param.s1.powi(2) * param.s12.powi(8) * param.s2.powi(4)
                + 3730. * param.s1.powi(3) * param.s12.powi(8) * param.s2.powi(4)
                + -938. * param.m2_2.powi(2) * param.s12.powi(9) * param.s2.powi(4)
                + 1295. * param.m2_2 * param.s1 * param.s12.powi(9) * param.s2.powi(4)
                + -540. * param.s1.powi(2) * param.s12.powi(9) * param.s2.powi(4)
                + -1820. * param.m2_2 * param.s12.powi(10) * param.s2.powi(4)
                + 315. * param.s1 * param.s12.powi(10) * param.s2.powi(4)
                + 1022. * param.s12.powi(11) * param.s2.powi(4)
                + -1512. * param.m2_2.powi(6) * param.s1.powi(4) * param.s2.powi(5)
                + -1218. * param.m2_2.powi(6) * param.s1.powi(3) * param.s12 * param.s2.powi(5)
                + 4956. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12 * param.s2.powi(5)
                + -1050.
                    * param.m2_2.powi(6)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + 4928.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + -3570.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + -1130. * param.m2_2.powi(6) * param.s1 * param.s12.powi(3) * param.s2.powi(5)
                + 4550.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + -6160.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + -2940.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + -1890. * param.m2_2.powi(6) * param.s12.powi(4) * param.s2.powi(5)
                + 5310. * param.m2_2.powi(5) * param.s1 * param.s12.powi(4) * param.s2.powi(5)
                + -8050.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(5)
                + -3430.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(5)
                + -840.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(5)
                + 5264. * param.m2_2.powi(5) * param.s12.powi(5) * param.s2.powi(5)
                + -5778. * param.m2_2.powi(4) * param.s1 * param.s12.powi(5) * param.s2.powi(5)
                + 770.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(5)
                + -5880.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(5)
                + 1512. * param.m2_2 * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(5)
                + -3262. * param.m2_2.powi(4) * param.s12.powi(6) * param.s2.powi(5)
                + -1508. * param.m2_2.powi(3) * param.s1 * param.s12.powi(6) * param.s2.powi(5)
                + 7420.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(5)
                + 18018. * param.m2_2 * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(5)
                + 1134. * param.s1.powi(4) * param.s12.powi(6) * param.s2.powi(5)
                + -1792. * param.m2_2.powi(3) * param.s12.powi(7) * param.s2.powi(5)
                + 3042. * param.m2_2.powi(2) * param.s1 * param.s12.powi(7) * param.s2.powi(5)
                + -1400. * param.m2_2 * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(5)
                + -6258. * param.s1.powi(3) * param.s12.powi(7) * param.s2.powi(5)
                + 658. * param.m2_2.powi(2) * param.s12.powi(8) * param.s2.powi(5)
                + 1782. * param.m2_2 * param.s1 * param.s12.powi(8) * param.s2.powi(5)
                + -2240. * param.s1.powi(2) * param.s12.powi(8) * param.s2.powi(5)
                + 2128. * param.m2_2 * param.s12.powi(9) * param.s2.powi(5)
                + -1718. * param.s1 * param.s12.powi(9) * param.s2.powi(5)
                + -1106. * param.s12.powi(10) * param.s2.powi(5)
                + 1008. * param.m2_2.powi(6) * param.s1.powi(3) * param.s2.powi(6)
                + 1470. * param.m2_2.powi(6) * param.s1.powi(2) * param.s12 * param.s2.powi(6)
                + -3108. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12 * param.s2.powi(6)
                + 1520. * param.m2_2.powi(6) * param.s1 * param.s12.powi(2) * param.s2.powi(6)
                + -4830.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(6)
                + 1890.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(6)
                + 1190. * param.m2_2.powi(6) * param.s12.powi(3) * param.s2.powi(6)
                + -4955. * param.m2_2.powi(5) * param.s1 * param.s12.powi(3) * param.s2.powi(6)
                + 3570.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(6)
                + 1785.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(6)
                + -3220. * param.m2_2.powi(5) * param.s12.powi(4) * param.s2.powi(6)
                + 3445. * param.m2_2.powi(4) * param.s1 * param.s12.powi(4) * param.s2.powi(6)
                + 3570.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(6)
                + 1365.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(6)
                + 1778. * param.m2_2.powi(4) * param.s12.powi(5) * param.s2.powi(6)
                + 2430. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5) * param.s2.powi(6)
                + -2940.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(6)
                + -4263. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(6)
                + 1288. * param.m2_2.powi(3) * param.s12.powi(6) * param.s2.powi(6)
                + -1210. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6) * param.s2.powi(6)
                + -3780. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(6)
                + 1323. * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(6)
                + -182. * param.m2_2.powi(2) * param.s12.powi(7) * param.s2.powi(6)
                + -2995. * param.m2_2 * param.s1 * param.s12.powi(7) * param.s2.powi(6)
                + 2940. * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(6)
                + -1652. * param.m2_2 * param.s12.powi(8) * param.s2.powi(6)
                + 1765. * param.s1 * param.s12.powi(8) * param.s2.powi(6)
                + 798. * param.s12.powi(9) * param.s2.powi(6)
                + -432. * param.m2_2.powi(6) * param.s1.powi(2) * param.s2.powi(7)
                + -666. * param.m2_2.powi(6) * param.s1 * param.s12 * param.s2.powi(7)
                + 1248. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12 * param.s2.powi(7)
                + -482. * param.m2_2.powi(6) * param.s12.powi(2) * param.s2.powi(7)
                + 1924. * param.m2_2.powi(5) * param.s1 * param.s12.powi(2) * param.s2.powi(7)
                + -642.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(7)
                + 1268. * param.m2_2.powi(5) * param.s12.powi(3) * param.s2.powi(7)
                + -1016. * param.m2_2.powi(4) * param.s1 * param.s12.powi(3) * param.s2.powi(7)
                + -642.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(7)
                + -622. * param.m2_2.powi(4) * param.s12.powi(4) * param.s2.powi(7)
                + -1016. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2.powi(7)
                + -642.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(7)
                + -552. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2.powi(7)
                + -106. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2.powi(7)
                + 2130. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(7)
                + -62. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2.powi(7)
                + 1700. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2.powi(7)
                + -1020. * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(7)
                + 820. * param.m2_2 * param.s12.powi(7) * param.s2.powi(7)
                + -820. * param.s1 * param.s12.powi(7) * param.s2.powi(7)
                + -370. * param.s12.powi(8) * param.s2.powi(7)
                + 108. * param.m2_2.powi(6) * param.s1 * param.s2.powi(8)
                + 114. * param.m2_2.powi(6) * param.s12 * param.s2.powi(8)
                + -291. * param.m2_2.powi(5) * param.s1 * param.s12 * param.s2.powi(8)
                + -292. * param.m2_2.powi(5) * param.s12.powi(2) * param.s2.powi(8)
                + 129. * param.m2_2.powi(4) * param.s1 * param.s12.powi(2) * param.s2.powi(8)
                + 128. * param.m2_2.powi(4) * param.s12.powi(3) * param.s2.powi(8)
                + 129. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(8)
                + 128. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(8)
                + 129. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(8)
                + 58. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(8)
                + -354. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(8)
                + -236. * param.m2_2 * param.s12.powi(6) * param.s2.powi(8)
                + 150. * param.s1 * param.s12.powi(6) * param.s2.powi(8)
                + 100. * param.s12.powi(7) * param.s2.powi(8)
                + -12. * param.m2_2.powi(6) * param.s2.powi(9)
                + 30. * param.m2_2.powi(5) * param.s12 * param.s2.powi(9)
                + -12. * param.m2_2.powi(4) * param.s12.powi(2) * param.s2.powi(9)
                + -12. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(9)
                + -12. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(9)
                + 30. * param.m2_2 * param.s12.powi(5) * param.s2.powi(9)
                + -12. * param.s12.powi(6) * param.s2.powi(9)
                + 420.
                    * param.m0_2.powi(6)
                    * param.s12.powi(6)
                    * (5. * param.s12.powi(3)
                        + 3. * (param.s1 - param.s2).powi(3)
                        + param.s12.powi(2) * (-7. * param.s1 + 5. * param.s2)
                        - param.s12
                            * (param.s1.powi(2)
                                + -8. * param.s1 * param.s2
                                + 7. * param.s2.powi(2)))
                + -3.
                    * param.m1_2.powi(6)
                    * (5. * param.s12.powi(9)
                        + -4. * (param.s1 - param.s2).powi(9)
                        + -4. * param.s12.powi(8) * (11. * param.s1 + 15. * param.s2)
                        + param.s12
                            * (param.s1 - param.s2).powi(7)
                            * (37. * param.s1 + 45. * param.s2)
                        + -4.
                            * param.s12.powi(2)
                            * (param.s1 - param.s2).powi(5)
                            * (38. * param.s1.powi(2)
                                + 73. * param.s1 * param.s2
                                + 60. * param.s2.powi(2))
                        + 2. * param.s12.powi(7)
                            * (86. * param.s1.powi(2)
                                + 181. * param.s1 * param.s2
                                + 180. * param.s2.powi(2))
                        + -4.
                            * param.s12.powi(6)
                            * (98. * param.s1.powi(3)
                                + 219. * param.s1.powi(2) * param.s2
                                + 333. * param.s1 * param.s2.powi(2)
                                + 420. * param.s2.powi(3))
                        + 2. * param.s12.powi(3)
                            * (param.s1 - param.s2).powi(3)
                            * (182. * param.s1.powi(3)
                                + 381. * param.s1.powi(2) * param.s2
                                + 507. * param.s1 * param.s2.powi(2)
                                + 420. * param.s2.powi(3))
                        + 2. * param.s12.powi(5)
                            * (287. * param.s1.powi(4)
                                + 507. * param.s1.powi(3) * param.s2
                                + 807. * param.s1.powi(2) * param.s2.powi(2)
                                + 1487. * param.s1 * param.s2.powi(3)
                                + -252. * param.s2.powi(4))
                        + -40.
                            * param.s12.powi(4)
                            * (14. * param.s1.powi(5)
                                + 10. * param.s1.powi(4) * param.s2
                                + 12. * param.s1.powi(3) * param.s2.powi(2)
                                + 20. * param.s1.powi(2) * param.s2.powi(3)
                                + 7. * param.s1 * param.s2.powi(4)
                                + -63. * param.s2.powi(5)))
                + 210.
                    * param.m0_2.powi(5)
                    * param.s12.powi(5)
                    * (param.m2_2
                        * (-35. * param.s12.powi(4)
                            + -9. * (param.s1 - param.s2).powi(4)
                            + 2. * param.s12.powi(3) * (9. * param.s1 + 10. * param.s2)
                            + -2.
                                * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (17. * param.s1 + 16. * param.s2)
                            + 4. * param.s12.powi(2)
                                * (15. * param.s1.powi(2)
                                    + -35. * param.s1 * param.s2
                                    + 14. * param.s2.powi(2)))
                        + param.m1_2
                            * (-25. * param.s12.powi(4)
                                + param.s12.powi(3) * (66. * param.s1 + -80. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 + -34. * param.s2)
                                    * (param.s1 - param.s2).powi(2)
                                + 9. * (param.s1 - param.s2).powi(4)
                                + param.s12.powi(2)
                                    * (-48. * param.s1.powi(2)
                                        + 44. * param.s1 * param.s2
                                        + 28. * param.s2.powi(2)))
                        + param.s12
                            * (19. * param.s12.powi(4)
                                + -3.
                                    * (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1 + 7. * param.s2)
                                + param.s12.powi(3) * (-42. * param.s1 + 16. * param.s2)
                                + 4. * param.s12.powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 18. * param.s1 * param.s2
                                        + -17. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (13. * param.s1.powi(3)
                                        + -56. * param.s1.powi(2) * param.s2
                                        + 37. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3))))
                + param.m1_2.powi(5)
                    * (param.s12
                        * (69. * param.s12.powi(9)
                            + -3.
                                * (17. * param.s1 + -24. * param.s2)
                                * (param.s1 - param.s2).powi(8)
                            + param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (477. * param.s1.powi(2)
                                    + -94. * param.s1 * param.s2
                                    + -845. * param.s2.powi(2))
                            + 4. * param.s12.powi(7)
                                * (585. * param.s1.powi(2)
                                    + 1181. * param.s1 * param.s2
                                    + 1130. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (990. * param.s1.powi(3)
                                    + 532. * param.s1.powi(2) * param.s2
                                    + -1287. * param.s1 * param.s2.powi(2)
                                    + -2440. * param.s2.powi(3))
                            + -2.
                                * param.s12.powi(6)
                                * (2646. * param.s1.powi(3)
                                    + 5532. * param.s1.powi(2) * param.s2
                                    + 7837. * param.s1 * param.s2.powi(2)
                                    + 9240. * param.s2.powi(3))
                            + 4. * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (1197. * param.s1.powi(4)
                                    + 909. * param.s1.powi(3) * param.s2
                                    + -364. * param.s1.powi(2) * param.s2.powi(2)
                                    + -3317. * param.s1 * param.s2.powi(3)
                                    + -5250. * param.s2.powi(4))
                            + 2. * param.s12.powi(5)
                                * (3843. * param.s1.powi(4)
                                    + 5990. * param.s1.powi(3) * param.s2
                                    + 8177. * param.s1.powi(2) * param.s2.powi(2)
                                    + 12164. * param.s1 * param.s2.powi(3)
                                    + 6384. * param.s2.powi(4))
                            + -2.
                                * param.s12.powi(4)
                                * (3717. * param.s1.powi(5)
                                    + 1640. * param.s1.powi(4) * param.s2
                                    + 540. * param.s1.powi(3) * param.s2.powi(2)
                                    + -2880. * param.s1.powi(2) * param.s2.powi(3)
                                    + 20335. * param.s1 * param.s2.powi(4)
                                    + -9408. * param.s2.powi(5))
                            - param.s12.powi(8) * (603. * param.s1 + 800. * param.s2))
                        + param.m2_2
                            * (125. * param.s12.powi(9)
                                + -72. * (param.s1 - param.s2).powi(9)
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(7)
                                    * (229. * param.s1 + 263. * param.s2)
                                + -2. * param.s12.powi(8) * (529. * param.s1 + 820. * param.s2)
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(5)
                                    * (1459. * param.s1.powi(2)
                                        + 2677. * param.s1 * param.s2
                                        + 2020. * param.s2.powi(2))
                                + 2. * param.s12.powi(7)
                                    * (1989. * param.s1.powi(2)
                                        + 4672. * param.s1 * param.s2
                                        + 6180. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(6)
                                    * (4361. * param.s1.powi(3)
                                        + 10670. * param.s1.powi(2) * param.s2
                                        + 20507. * param.s1 * param.s2.powi(2)
                                        + 1848. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(3)
                                    * (3619. * param.s1.powi(3)
                                        + 7397. * param.s1.powi(2) * param.s2
                                        + 9224. * param.s1 * param.s2.powi(2)
                                        + 6580. * param.s2.powi(3))
                                + 4. * param.s12.powi(5)
                                    * (3073. * param.s1.powi(4)
                                        + 5823. * param.s1.powi(3) * param.s2
                                        + 11043. * param.s1.powi(2) * param.s2.powi(2)
                                        + 6341. * param.s1 * param.s2.powi(3)
                                        + -7728. * param.s2.powi(4))
                                + -10.
                                    * param.s12.powi(4)
                                    * (1155. * param.s1.powi(5)
                                        + 860. * param.s1.powi(4) * param.s2
                                        + 1116. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1104. * param.s1.powi(2) * param.s2.powi(3)
                                        + -875. * param.s1 * param.s2.powi(4)
                                        + -3360. * param.s2.powi(5))))
                + param.m1_2.powi(3)
                    * (param.m2_2.powi(2)
                        * param.s12
                        * (635. * param.s12.powi(9)
                            + -6.
                                * (17. * param.s1 + -52. * param.s2)
                                * (param.s1 - param.s2).powi(8)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (519. * param.s1.powi(2)
                                    + -1130. * param.s1 * param.s2
                                    + -1699. * param.s2.powi(2))
                            + param.s12.powi(7)
                                * (14193. * param.s1.powi(2)
                                    + 33556. * param.s1 * param.s2
                                    + 15347. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (30667. * param.s1.powi(4)
                                    + -5888. * param.s1.powi(3) * param.s2
                                    + 233146. * param.s1.powi(2) * param.s2.powi(2)
                                    + 204672. * param.s1 * param.s2.powi(3)
                                    + -91917. * param.s2.powi(4))
                            + param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (13307. * param.s1.powi(4)
                                    + -24886. * param.s1.powi(3) * param.s2
                                    + -94024. * param.s1.powi(2) * param.s2.powi(2)
                                    + -111082. * param.s1 * param.s2.powi(3)
                                    + -56315. * param.s2.powi(4))
                            + -15.
                                * param.s12.powi(4)
                                * (1631. * param.s1.powi(5)
                                    + -3352. * param.s1.powi(4) * param.s2
                                    + 2090. * param.s1.powi(3) * param.s2.powi(2)
                                    + 22556. * param.s1.powi(2) * param.s2.powi(3)
                                    + 2527. * param.s1 * param.s2.powi(4)
                                    + -6860. * param.s2.powi(5))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (4793. * param.s1.powi(3)
                                    + -8512. * param.s1.powi(2) * param.s2
                                    + -23103. * param.s1 * param.s2.powi(2)
                                    + -17278. * param.s2.powi(3))
                            - param.s12.powi(6)
                                * (25963. * param.s1.powi(3)
                                    + 37612. * param.s1.powi(2) * param.s2
                                    + 166875. * param.s1 * param.s2.powi(2)
                                    + -24318. * param.s2.powi(3))
                            - param.s12.powi(8) * (4517. * param.s1 + 9160. * param.s2))
                        + param.m2_2.powi(3)
                            * (1595. * param.s12.powi(9)
                                + -2.
                                    * param.s12.powi(8)
                                    * (5359. * param.s1 + -545. * param.s2)
                                + -240. * (param.s1 - param.s2).powi(9)
                                + 30.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(7)
                                    * (81. * param.s1 + 83. * param.s2)
                                + param.s12.powi(7)
                                    * (32223. * param.s1.powi(2)
                                        + 32234. * param.s1 * param.s2
                                        + -26973. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(5)
                                    * (2777. * param.s1.powi(2)
                                        + 4541. * param.s1 * param.s2
                                        + 2942. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(6)
                                    * (14350. * param.s1.powi(3)
                                        + 28855. * param.s1.powi(2) * param.s2
                                        + -6998. * param.s1 * param.s2.powi(2)
                                        + -17325. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(3)
                                    * (30233. * param.s1.powi(3)
                                        + 56269. * param.s1.powi(2) * param.s2
                                        + 58663. * param.s1 * param.s2.powi(2)
                                        + 33635. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (67375. * param.s1.powi(4)
                                        + 135180. * param.s1.powi(3) * param.s2
                                        + 58074. * param.s1.powi(2) * param.s2.powi(2)
                                        + -112564. * param.s1 * param.s2.powi(3)
                                        + -86625. * param.s2.powi(4))
                                + -10.
                                    * param.s12.powi(4)
                                    * (5439. * param.s1.powi(5)
                                        + 4367. * param.s1.powi(4) * param.s2
                                        + 3258. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1626. * param.s1.powi(2) * param.s2.powi(3)
                                        + -4865. * param.s1 * param.s2.powi(4)
                                        + -6573. * param.s2.powi(5)))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (278. * param.s12.powi(9)
                                + -2.
                                    * param.s12.powi(8)
                                    * (1072. * param.s1 + 1619. * param.s2)
                                + -12.
                                    * (param.s1 - param.s2).powi(7)
                                    * (5. * param.s1.powi(2)
                                        + -17. * param.s1 * param.s2
                                        + 26. * param.s2.powi(2))
                                + param.s12.powi(7)
                                    * (7284. * param.s1.powi(2)
                                        + 14866. * param.s1 * param.s2
                                        + 18875. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (618. * param.s1.powi(3)
                                        + -1576. * param.s1.powi(2) * param.s2
                                        + 1571. * param.s1 * param.s2.powi(2)
                                        + 3755. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(6)
                                    * (7154. * param.s1.powi(3)
                                        + 12107. * param.s1.powi(2) * param.s2
                                        + 13413. * param.s1 * param.s2.powi(2)
                                        + 18690. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (1462. * param.s1.powi(4)
                                        + -3261. * param.s1.powi(3) * param.s2
                                        + 3656. * param.s1.powi(2) * param.s2.powi(2)
                                        + 17623. * param.s1 * param.s2.powi(3)
                                        + 11600. * param.s2.powi(4))
                                + param.s12.powi(5)
                                    * (17920. * param.s1.powi(4)
                                        + 10786. * param.s1.powi(3) * param.s2
                                        + -26015. * param.s1.powi(2) * param.s2.powi(2)
                                        + 156792. * param.s1 * param.s2.powi(3)
                                        + 13293. * param.s2.powi(4))
                                + -6.
                                    * param.s12.powi(4)
                                    * (2478. * param.s1.powi(5)
                                        + -2605. * param.s1.powi(4) * param.s2
                                        + -7270. * param.s1.powi(3) * param.s2.powi(2)
                                        + -12840. * param.s1.powi(2) * param.s2.powi(3)
                                        + 48160. * param.s1 * param.s2.powi(4)
                                        + -6867. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (8204. * param.s1.powi(6)
                                        + -25082. * param.s1.powi(5) * param.s2
                                        + 13365. * param.s1.powi(4) * param.s2.powi(2)
                                        + -188160. * param.s1.powi(3) * param.s2.powi(3)
                                        + 78650. * param.s1.powi(2) * param.s2.powi(4)
                                        + 165810. * param.s1 * param.s2.powi(5)
                                        + -52787. * param.s2.powi(6)))
                        + param.s12.powi(3)
                            * (90. * param.s12.powi(9)
                                + -2. * param.s12.powi(8) * (375. * param.s1 + 484. * param.s2)
                                + param.s12.powi(7)
                                    * (2760. * param.s1.powi(2)
                                        + 5144. * param.s1 * param.s2
                                        + 4835. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(6)
                                    * (10. * param.s1.powi(3)
                                        + -44. * param.s1.powi(2) * param.s2
                                        + 79. * param.s1 * param.s2.powi(2)
                                        + -80. * param.s2.powi(3))
                                + param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (330. * param.s1.powi(4)
                                        + -1216. * param.s1.powi(3) * param.s2
                                        + 1605. * param.s1.powi(2) * param.s2.powi(2)
                                        + -924. * param.s1 * param.s2.powi(3)
                                        + -3995. * param.s2.powi(4))
                                + param.s12.powi(5)
                                    * (7980. * param.s1.powi(4)
                                        + 7528. * param.s1.powi(3) * param.s2
                                        + 4699. * param.s1.powi(2) * param.s2.powi(2)
                                        + -872. * param.s1 * param.s2.powi(3)
                                        + 26145. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(4)
                                    * (1428. * param.s1.powi(5)
                                        + -688. * param.s1.powi(4) * param.s2
                                        + -1863. * param.s1.powi(3) * param.s2.powi(2)
                                        + -3966. * param.s1.powi(2) * param.s2.powi(3)
                                        + 4165. * param.s1 * param.s2.powi(4)
                                        + 3780. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (4200. * param.s1.powi(6)
                                        + -10392. * param.s1.powi(5) * param.s2
                                        + -895. * param.s1.powi(4) * param.s2.powi(2)
                                        + 16800. * param.s1.powi(3) * param.s2.powi(3)
                                        + -116670. * param.s1.powi(2) * param.s2.powi(4)
                                        + 85264. * param.s1 * param.s2.powi(5)
                                        + -1827. * param.s2.powi(6))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (1560. * param.s1.powi(5)
                                        + -4576. * param.s1.powi(4) * param.s2
                                        + 1583. * param.s1.powi(3) * param.s2.powi(2)
                                        + 16452. * param.s1.powi(2) * param.s2.powi(3)
                                        + 49291. * param.s1 * param.s2.powi(4)
                                        + -9710. * param.s2.powi(5))
                                - param.s12.powi(6)
                                    * (5880. * param.s1.powi(3)
                                        + 10224. * param.s1.powi(2) * param.s2
                                        + 12629. * param.s1 * param.s2.powi(2)
                                        + 15330. * param.s2.powi(3))))
                + param.m1_2.powi(4)
                    * (param.m2_2.powi(2)
                        * (-505. * param.s12.powi(9)
                            + 180. * (param.s1 - param.s2).powi(9)
                            + -30.
                                * param.s12
                                * (param.s1 - param.s2).powi(7)
                                * (59. * param.s1 + 64. * param.s2)
                            + param.s12.powi(8) * (3982. * param.s1 + 8440. * param.s2)
                            + 2. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(5)
                                * (3896. * param.s1.powi(2)
                                    + 6773. * param.s1 * param.s2
                                    + 4721. * param.s2.powi(2))
                            + param.s12.powi(6)
                                * (28700. * param.s1.powi(3)
                                    + 85130. * param.s1.powi(2) * param.s2
                                    + 62852. * param.s1 * param.s2.powi(2)
                                    + -40950. * param.s2.powi(3))
                            + param.s12.powi(5)
                                * (-38045. * param.s1.powi(4)
                                    + -82170. * param.s1.powi(3) * param.s2
                                    + -103836. * param.s1.powi(2) * param.s2.powi(2)
                                    + 27926. * param.s1 * param.s2.powi(3)
                                    + 80325. * param.s2.powi(4))
                            + 10.
                                * param.s12.powi(4)
                                * (3381. * param.s1.powi(5)
                                    + 2668. * param.s1.powi(4) * param.s2
                                    + 3042. * param.s1.powi(3) * param.s2.powi(2)
                                    + 786. * param.s1.powi(2) * param.s2.powi(3)
                                    + -3535. * param.s1 * param.s2.powi(4)
                                    + -6342. * param.s2.powi(5))
                            - param.s12.powi(3)
                                * (param.s1 - param.s2).powi(3)
                                * (20167. * param.s1.powi(3)
                                    + 39701. * param.s1.powi(2) * param.s2
                                    + 45497. * param.s1 * param.s2.powi(2)
                                    + 28735. * param.s2.powi(3))
                            - param.s12.powi(7)
                                * (13977. * param.s1.powi(2)
                                    + 42316. * param.s1 * param.s2
                                    + 4923. * param.s2.powi(2)))
                        + param.m2_2
                            * param.s12
                            * (-340. * param.s12.powi(9)
                                + 3. * (51. * param.s1 + -86. * param.s2)
                                    * (param.s1 - param.s2).powi(8)
                                + param.s12.powi(8) * (2803. * param.s1 + 4220. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(6)
                                    * (747. * param.s1.powi(2)
                                        + -442. * param.s1 * param.s2
                                        + -1460. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(7)
                                    * (5127. * param.s1.powi(2)
                                        + 11272. * param.s1 * param.s2
                                        + 14060. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(4)
                                    * (3257. * param.s1.powi(3)
                                        + 245. * param.s1.powi(2) * param.s2
                                        + -6612. * param.s1 * param.s2.powi(2)
                                        + -7915. * param.s2.powi(3))
                                + param.s12.powi(6)
                                    * (21854. * param.s1.powi(3)
                                        + 46982. * param.s1.powi(2) * param.s2
                                        + 74784. * param.s1 * param.s2.powi(2)
                                        + 35910. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(2)
                                    * (8309. * param.s1.powi(4)
                                        + 2738. * param.s1.powi(3) * param.s2
                                        + -14413. * param.s1.powi(2) * param.s2.powi(2)
                                        + -35344. * param.s1 * param.s2.powi(3)
                                        + -29540. * param.s2.powi(4))
                                + -2.
                                    * param.s12.powi(5)
                                    * (14959. * param.s1.powi(4)
                                        + 21484. * param.s1.powi(3) * param.s2
                                        + 21619. * param.s1.powi(2) * param.s2.powi(2)
                                        + 85254. * param.s1 * param.s2.powi(3)
                                        + -16254. * param.s2.powi(4))
                                + 30.
                                    * param.s12.powi(4)
                                    * (910. * param.s1.powi(5)
                                        + 125. * param.s1.powi(4) * param.s2
                                        + -1068. * param.s1.powi(3) * param.s2.powi(2)
                                        + 3442. * param.s1.powi(2) * param.s2.powi(3)
                                        + 4242. * param.s1 * param.s2.powi(4)
                                        + -3003. * param.s2.powi(5)))
                        + param.s12.powi(2)
                            * (-120. * param.s12.powi(9)
                                + param.s12.powi(8) * (1035. * param.s1 + 1342. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(7)
                                    * (25. * param.s1.powi(2)
                                        + -71. * param.s1 * param.s2
                                        + 60. * param.s2.powi(2))
                                + 2. * param.s12.powi(6)
                                    * (4410. * param.s1.powi(3)
                                        + 8538. * param.s1.powi(2) * param.s2
                                        + 11273. * param.s1 * param.s2.powi(2)
                                        + 12810. * param.s2.powi(3))
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (1530. * param.s1.powi(4)
                                        + -1432. * param.s1.powi(3) * param.s2
                                        + -3011. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2413. * param.s1 * param.s2.powi(3)
                                        + 8270. * param.s2.powi(4))
                                + 10.
                                    * param.s12.powi(4)
                                    * (1197. * param.s1.powi(5)
                                        + 134. * param.s1.powi(4) * param.s2
                                        + -591. * param.s1.powi(3) * param.s2.powi(2)
                                        + -2427. * param.s1.powi(2) * param.s2.powi(3)
                                        + 6664. * param.s1 * param.s2.powi(4)
                                        + 315. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (-7560. * param.s1.powi(6)
                                        + 12498. * param.s1.powi(5) * param.s2
                                        + 6455. * param.s1.powi(4) * param.s2.powi(2)
                                        + 2100. * param.s1.powi(3) * param.s2.powi(3)
                                        + 56790. * param.s1.powi(2) * param.s2.powi(4)
                                        + -96806. * param.s1 * param.s2.powi(5)
                                        + 26523. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (12600. * param.s1.powi(4)
                                        + 16622. * param.s1.powi(3) * param.s2
                                        + 18401. * param.s1.powi(2) * param.s2.powi(2)
                                        + 19772. * param.s1 * param.s2.powi(3)
                                        + 34965. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (720. * param.s1.powi(3)
                                        + -1214. * param.s1.powi(2) * param.s2
                                        + -719. * param.s1 * param.s2.powi(2)
                                        + 2305. * param.s2.powi(3))
                                - param.s12.powi(7)
                                    * (3960. * param.s1.powi(2)
                                        + 7666. * param.s1 * param.s2
                                        + 7135. * param.s2.powi(2))))
                + param.m1_2.powi(2)
                    * (param.m2_2.powi(4)
                        * (1070. * param.s12.powi(9)
                            + 2. * param.s12.powi(8) * (941. * param.s1 + -3655. * param.s2)
                            + 180. * (param.s1 - param.s2).powi(9)
                            + -15.
                                * param.s12
                                * (param.s1 - param.s2).powi(7)
                                * (125. * param.s1 + 121. * param.s2)
                            + 4. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(5)
                                * (2228. * param.s1.powi(2)
                                    + 3404. * param.s1 * param.s2
                                    + 2063. * param.s2.powi(2))
                            + param.s12.powi(7)
                                * (-23427. * param.s1.powi(2)
                                    + 19424. * param.s1 * param.s2
                                    + 22377. * param.s2.powi(2))
                            + 4. * param.s12.powi(6)
                                * (14175. * param.s1.powi(3)
                                    + 7475. * param.s1.powi(2) * param.s2
                                    + -19672. * param.s1 * param.s2.powi(2)
                                    + -10150. * param.s2.powi(3))
                            + param.s12.powi(5)
                                * (-68775. * param.s1.powi(4)
                                    + -90430. * param.s1.powi(3) * param.s2
                                    + 38544. * param.s1.powi(2) * param.s2.powi(2)
                                    + 95826. * param.s1 * param.s2.powi(3)
                                    + 48475. * param.s2.powi(4))
                            + 10.
                                * param.s12.powi(4)
                                * (5103. * param.s1.powi(5)
                                    + 3683. * param.s1.powi(4) * param.s2
                                    + 942. * param.s1.powi(3) * param.s2.powi(2)
                                    + -2574. * param.s1.powi(2) * param.s2.powi(3)
                                    + -3185. * param.s1 * param.s2.powi(4)
                                    + -3969. * param.s2.powi(5))
                            - param.s12.powi(3)
                                * (param.s1 - param.s2).powi(3)
                                * (25697. * param.s1.powi(3)
                                    + 44251. * param.s1.powi(2) * param.s2
                                    + 41857. * param.s1 * param.s2.powi(2)
                                    + 22295. * param.s2.powi(3)))
                        + -3.
                            * param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (33. * param.s12.powi(9)
                                + 2. * (param.s1 - param.s2).powi(7)
                                    * (5. * param.s1.powi(2)
                                        + -31. * param.s1 * param.s2
                                        + -16. * param.s2.powi(2))
                                + param.s12.powi(7)
                                    * (-353. * param.s1.powi(2)
                                        + -2018. * param.s1 * param.s2
                                        + 766. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (1645. * param.s1.powi(3)
                                        + 9834. * param.s1.powi(2) * param.s2
                                        + -19329. * param.s1 * param.s2.powi(2)
                                        + 518. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (583. * param.s1.powi(4)
                                        + -3453. * param.s1.powi(3) * param.s2
                                        + -14697. * param.s1.powi(2) * param.s2.powi(2)
                                        + -11719. * param.s1 * param.s2.powi(3)
                                        + -1794. * param.s2.powi(4))
                                + param.s12.powi(4)
                                    * (2947. * param.s1.powi(5)
                                        + -50. * param.s1.powi(4) * param.s2
                                        + 95770. * param.s1.powi(3) * param.s2.powi(2)
                                        + -70520. * param.s1.powi(2) * param.s2.powi(3)
                                        + -54845. * param.s1 * param.s2.powi(4)
                                        + 6202. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (-1715. * param.s1.powi(6)
                                        + 8154. * param.s1.powi(5) * param.s2
                                        + -48480. * param.s1.powi(4) * param.s2.powi(2)
                                        + -56280. * param.s1.powi(3) * param.s2.powi(3)
                                        + 97305. * param.s1.powi(2) * param.s2.powi(4)
                                        + 5622. * param.s1 * param.s2.powi(5)
                                        + -4606. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (2975. * param.s1.powi(4)
                                        + 11418. * param.s1.powi(3) * param.s2
                                        + 23775. * param.s1.powi(2) * param.s2.powi(2)
                                        + -64000. * param.s1 * param.s2.powi(3)
                                        + 4032. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (110. * param.s1.powi(3)
                                        + -636. * param.s1.powi(2) * param.s2
                                        + -1305. * param.s1 * param.s2.powi(2)
                                        + -353. * param.s2.powi(3))
                                - param.s12.powi(8) * (65. * param.s1 + 354. * param.s2))
                        + param.m2_2
                            * param.s12.powi(3)
                            * (-44. * param.s12.powi(9)
                                + 2. * param.s12.powi(8) * (97. * param.s1 + 229. * param.s2)
                                + -2.
                                    * param.s12.powi(7)
                                    * (54. * param.s1.powi(2)
                                        + 43. * param.s1 * param.s2
                                        + 1080. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(6)
                                    * (6. * param.s1.powi(3)
                                        + -32. * param.s1.powi(2) * param.s2
                                        + 95. * param.s1 * param.s2.powi(2)
                                        + 36. * param.s2.powi(3))
                                + 2. * param.s12.powi(5)
                                    * (1414. * param.s1.powi(4)
                                        + 5673. * param.s1.powi(3) * param.s2
                                        + 11853. * param.s1.powi(2) * param.s2.powi(2)
                                        + -9426. * param.s1 * param.s2.powi(3)
                                        + -1946. * param.s2.powi(4))
                                + 6. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (40. * param.s1.powi(4)
                                        + -207. * param.s1.powi(3) * param.s2
                                        + 591. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1456. * param.s1 * param.s2.powi(3)
                                        + 220. * param.s2.powi(4))
                                + 2. * param.s12.powi(3)
                                    * (1358. * param.s1.powi(6)
                                        + -1649. * param.s1.powi(5) * param.s2
                                        + -18710. * param.s1.powi(4) * param.s2.powi(2)
                                        + 51940. * param.s1.powi(3) * param.s2.powi(3)
                                        + 67780. * param.s1.powi(2) * param.s2.powi(4)
                                        + -68435. * param.s1 * param.s2.powi(5)
                                        + 2996. * param.s2.powi(6))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (1132. * param.s1.powi(5)
                                        + -3482. * param.s1.powi(4) * param.s2
                                        + -10631. * param.s1.powi(3) * param.s2.powi(2)
                                        + -122930. * param.s1.powi(2) * param.s2.powi(3)
                                        + -32339. * param.s1 * param.s2.powi(4)
                                        + 4450. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (3696. * param.s1.powi(5)
                                        + 6910. * param.s1.powi(4) * param.s2
                                        + -16635. * param.s1.powi(3) * param.s2.powi(2)
                                        + 184410. * param.s1.powi(2) * param.s2.powi(3)
                                        + -122675. * param.s1 * param.s2.powi(4)
                                        + 1806. * param.s2.powi(5))
                                - param.s12.powi(6)
                                    * (980. * param.s1.powi(3)
                                        + 5258. * param.s1.powi(2) * param.s2
                                        + 12119. * param.s1 * param.s2.powi(2)
                                        + -4690. * param.s2.powi(3)))
                        - param.s12.powi(4)
                            * (15. * param.s12.powi(9)
                                + -2. * param.s12.powi(8) * (45. * param.s1 + 76. * param.s2)
                                + param.s12.powi(6)
                                    * param.s2
                                    * (564. * param.s1.powi(2)
                                        + 589. * param.s1 * param.s2
                                        + -1890. * param.s2.powi(2))
                                + param.s12.powi(7)
                                    * (180. * param.s1.powi(2)
                                        + 386. * param.s1 * param.s2
                                        + 695. * param.s2.powi(2))
                                + 3. * (param.s1 - param.s2).powi(5)
                                    * (10. * param.s1.powi(4)
                                        + -54. * param.s1.powi(3) * param.s2
                                        + 123. * param.s1.powi(2) * param.s2.powi(2)
                                        + -159. * param.s1 * param.s2.powi(3)
                                        + -60. * param.s2.powi(4))
                                + 5. * param.s12.powi(4)
                                    * (252. * param.s1.powi(5)
                                        + 712. * param.s1.powi(4) * param.s2
                                        + 657. * param.s1.powi(3) * param.s2.powi(2)
                                        + 234. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1785. * param.s1 * param.s2.powi(4)
                                        + -560. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (-1260. * param.s1.powi(6)
                                        + -738. * param.s1.powi(5) * param.s2
                                        + 4745. * param.s1.powi(4) * param.s2.powi(2)
                                        + 16800. * param.s1.powi(3) * param.s2.powi(3)
                                        + -61410. * param.s1.powi(2) * param.s2.powi(4)
                                        + 19526. * param.s1 * param.s2.powi(5)
                                        + 777. * param.s2.powi(6))
                                + param.s12.powi(2)
                                    * (720. * param.s1.powi(7)
                                        + -1676. * param.s1.powi(6) * param.s2
                                        + -3385. * param.s1.powi(5) * param.s2.powi(2)
                                        + 5210. * param.s1.powi(4) * param.s2.powi(3)
                                        + -63790. * param.s1.powi(3) * param.s2.powi(4)
                                        + 95900. * param.s1.powi(2) * param.s2.powi(5)
                                        + -33769. * param.s1 * param.s2.powi(6)
                                        + 790. * param.s2.powi(7))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (225. * param.s1.powi(5)
                                        + -671. * param.s1.powi(4) * param.s2
                                        + -679. * param.s1.powi(3) * param.s2.powi(2)
                                        + 7971. * param.s1.powi(2) * param.s2.powi(3)
                                        + 11944. * param.s1 * param.s2.powi(4)
                                        + -730. * param.s2.powi(5))
                                - param.s12.powi(5)
                                    * (630. * param.s1.powi(4)
                                        + 2978. * param.s1.powi(3) * param.s2
                                        + 5399. * param.s1.powi(2) * param.s2.powi(2)
                                        + 9278. * param.s1 * param.s2.powi(3)
                                        + -3115. * param.s2.powi(4)))
                        - param.m2_2.powi(3)
                            * param.s12
                            * (240. * param.s12.powi(9)
                                + 6. * (param.s1 - param.s2).powi(8)
                                    * (17. * param.s1 + 18. * param.s2)
                                + param.s12.powi(8) * (723. * param.s1 + 410. * param.s2)
                                + param.s12.powi(7)
                                    * (-8586. * param.s1.powi(2)
                                        + 40126. * param.s1 * param.s2
                                        + -6968. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(6)
                                    * (270. * param.s1.powi(2)
                                        + 604. * param.s1 * param.s2
                                        + 281. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(4)
                                    * (5241. * param.s1.powi(3)
                                        + 16828. * param.s1.powi(2) * param.s2
                                        + 16713. * param.s1 * param.s2.powi(2)
                                        + 5318. * param.s2.powi(3))
                                + param.s12.powi(6)
                                    * (24255. * param.s1.powi(3)
                                        + -89004. * param.s1.powi(2) * param.s2
                                        + -93385. * param.s1 * param.s2.powi(2)
                                        + 20874. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(2)
                                    * (7749. * param.s1.powi(4)
                                        + 34863. * param.s1.powi(3) * param.s2
                                        + 53207. * param.s1.powi(2) * param.s2.powi(2)
                                        + 33121. * param.s1 * param.s2.powi(3)
                                        + 7560. * param.s2.powi(4))
                                + -4.
                                    * param.s12.powi(5)
                                    * (8673. * param.s1.powi(4)
                                        + -4587. * param.s1.powi(3) * param.s2
                                        + -84398. * param.s1.powi(2) * param.s2.powi(2)
                                        + -7079. * param.s1 * param.s2.powi(3)
                                        + 7777. * param.s2.powi(4))
                                + 5. * param.s12.powi(4)
                                    * (5859. * param.s1.powi(5)
                                        + 13926. * param.s1.powi(4) * param.s2
                                        + -46958. * param.s1.powi(3) * param.s2.powi(2)
                                        + -46320. * param.s1.powi(2) * param.s2.powi(3)
                                        + 12243. * param.s1 * param.s2.powi(4)
                                        + 5474. * param.s2.powi(5))))
                + param.m1_2
                    * (param.m2_2.powi(4)
                        * param.s12
                        * (290. * param.s12.powi(9)
                            + 3. * (51. * param.s1 + -16. * param.s2)
                                * (param.s1 - param.s2).powi(8)
                            + -10. * param.s12.powi(8) * (419. * param.s1 + 208. * param.s2)
                            + param.s12.powi(7)
                                * (5475. * param.s1.powi(2)
                                    + 15760. * param.s1 * param.s2
                                    + 6593. * param.s2.powi(2))
                            + param.s12.powi(6)
                                * (17703. * param.s1.powi(3)
                                    + -79900. * param.s1.powi(2) * param.s2
                                    + -18771. * param.s1 * param.s2.powi(2)
                                    + -12166. * param.s2.powi(3))
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (8705. * param.s1.powi(3)
                                    + 11480. * param.s1.powi(2) * param.s2
                                    + 4143. * param.s1 * param.s2.powi(2)
                                    + -2278. * param.s2.powi(3))
                            + -5.
                                * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (5887. * param.s1.powi(4)
                                    + 11374. * param.s1.powi(3) * param.s2
                                    + 9016. * param.s1.powi(2) * param.s2.powi(2)
                                    + 2290. * param.s1 * param.s2.powi(3)
                                    + -1267. * param.s2.powi(4))
                            + param.s12.powi(5)
                                * (-52983. * param.s1.powi(4)
                                    + 98012. * param.s1.powi(3) * param.s2
                                    + 116908. * param.s1.powi(2) * param.s2.powi(2)
                                    + -632. * param.s1 * param.s2.powi(3)
                                    + 14483. * param.s2.powi(4))
                            + 5. * param.s12.powi(4)
                                * (11193. * param.s1.powi(5)
                                    + -3632. * param.s1.powi(4) * param.s2
                                    + -34030. * param.s1.powi(3) * param.s2.powi(2)
                                    + -3540. * param.s1.powi(2) * param.s2.powi(3)
                                    + 4445. * param.s1 * param.s2.powi(4)
                                    + -2324. * param.s2.powi(5))
                            - param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (1683. * param.s1.powi(2)
                                    + 1118. * param.s1 * param.s2
                                    + -491. * param.s2.powi(2)))
                        + param.m2_2.powi(5)
                            * (-190. * param.s12.powi(9)
                                + -72. * (param.s1 - param.s2).powi(9)
                                + 10. * param.s12.powi(8) * (310. * param.s1 + 151. * param.s2)
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(7)
                                    * (257. * param.s1 + 235. * param.s2)
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(5)
                                    * (1907. * param.s1.powi(2)
                                        + 2705. * param.s1 * param.s2
                                        + 1544. * param.s2.powi(2))
                                + 5. * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(3)
                                    * (2345. * param.s1.powi(3)
                                        + 3649. * param.s1.powi(2) * param.s2
                                        + 3145. * param.s1 * param.s2.powi(2)
                                        + 1589. * param.s2.powi(3))
                                + param.s12.powi(6)
                                    * (-19026. * param.s1.powi(3)
                                        + 21752. * param.s1.powi(2) * param.s2
                                        + 32066. * param.s1 * param.s2.powi(2)
                                        + 11060. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (34587. * param.s1.powi(4)
                                        + 13352. * param.s1.powi(3) * param.s2
                                        + -39114. * param.s1.powi(2) * param.s2.powi(2)
                                        + -31392. * param.s1 * param.s2.powi(3)
                                        + -14777. * param.s2.powi(4))
                                + -10.
                                    * param.s12.powi(4)
                                    * (2646. * param.s1.powi(5)
                                        + 1357. * param.s1.powi(4) * param.s2
                                        + -438. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1206. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1036. * param.s1 * param.s2.powi(4)
                                        + -1323. * param.s2.powi(5))
                                - param.s12.powi(7)
                                    * (621. * param.s1.powi(2)
                                        + 15730. * param.s1 * param.s2
                                        + 5343. * param.s2.powi(2)))
                        + param.m2_2.powi(2)
                            * param.s12.powi(3)
                            * (-23. * param.s12.powi(9)
                                + param.s12.powi(8) * (257. * param.s1 + 248. * param.s2)
                                + -3.
                                    * (param.s1 - param.s2).powi(6)
                                    * (6. * param.s1.powi(3)
                                        + -60. * param.s1.powi(2) * param.s2
                                        + -59. * param.s1 * param.s2.powi(2)
                                        + 8. * param.s2.powi(3))
                                + param.s12.powi(6)
                                    * (1183. * param.s1.powi(3)
                                        + -1100. * param.s1.powi(2) * param.s2
                                        + -464. * param.s1 * param.s2.powi(2)
                                        + 1204. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (-7. * param.s1.powi(4)
                                        + 17352. * param.s1.powi(3) * param.s2
                                        + -76653. * param.s1.powi(2) * param.s2.powi(2)
                                        + 15504. * param.s1 * param.s2.powi(3)
                                        + -196. * param.s2.powi(4))
                                + 3. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (94. * param.s1.powi(4)
                                        + -904. * param.s1.powi(3) * param.s2
                                        + -2843. * param.s1.powi(2) * param.s2.powi(2)
                                        + -644. * param.s1 * param.s2.powi(3)
                                        + 97. * param.s2.powi(4))
                                + param.s12.powi(3)
                                    * (2233. * param.s1.powi(6)
                                        + 9092. * param.s1.powi(5) * param.s2
                                        + 96140. * param.s1.powi(4) * param.s2.powi(2)
                                        + -280000. * param.s1.powi(3) * param.s2.powi(3)
                                        + 88625. * param.s1.powi(2) * param.s2.powi(4)
                                        + 11348. * param.s1 * param.s2.powi(5)
                                        + 2002. * param.s2.powi(6))
                                - param.s12.powi(4)
                                    * (1827. * param.s1.powi(5)
                                        + 25600. * param.s1.powi(4) * param.s2
                                        + -70815. * param.s1.powi(3) * param.s2.powi(2)
                                        + -74310. * param.s1.powi(2) * param.s2.powi(3)
                                        + 24850. * param.s1 * param.s2.powi(4)
                                        + 1512. * param.s2.powi(5))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (1195. * param.s1.powi(5)
                                        + -3230. * param.s1.powi(4) * param.s2
                                        + 84163. * param.s1.powi(3) * param.s2.powi(2)
                                        + 81316. * param.s1.powi(2) * param.s2.powi(3)
                                        + -776. * param.s1 * param.s2.powi(4)
                                        + 1132. * param.s2.powi(5))
                                - param.s12.powi(7)
                                    * (885. * param.s1.powi(2)
                                        + 2060. * param.s1 * param.s2
                                        + 858. * param.s2.powi(2)))
                        + param.m2_2.powi(3)
                            * param.s12.powi(2)
                            * (-51. * param.s12.powi(9)
                                + param.s12.powi(8) * (558. * param.s1 + 52. * param.s2)
                                + -12.
                                    * (param.s1 - param.s2).powi(7)
                                    * (5. * param.s1.powi(2)
                                        + 11. * param.s1 * param.s2
                                        + -2. * param.s2.powi(2))
                                + param.s12.powi(7)
                                    * (-675. * param.s1.powi(2)
                                        + 4100. * param.s1 * param.s2
                                        + 878. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (702. * param.s1.powi(3)
                                        + 2680. * param.s1.powi(2) * param.s2
                                        + 1249. * param.s1 * param.s2.powi(2)
                                        + -263. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(6)
                                    * (1806. * param.s1.powi(3)
                                        + -19785. * param.s1.powi(2) * param.s2
                                        + 9703. * param.s1 * param.s2.powi(2)
                                        + 1764. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (1980. * param.s1.powi(4)
                                        + 12755. * param.s1.powi(3) * param.s2
                                        + 14086. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2923. * param.s1 * param.s2.powi(3)
                                        + -664. * param.s2.powi(4))
                                + param.s12.powi(5)
                                    * (12285. * param.s1.powi(4)
                                        + -129438. * param.s1.powi(3) * param.s2
                                        + 42193. * param.s1.powi(2) * param.s2.powi(2)
                                        + 24100. * param.s1 * param.s2.powi(3)
                                        + 6244. * param.s2.powi(4))
                                + -2.
                                    * param.s12.powi(4)
                                    * (8127. * param.s1.powi(5)
                                        + -54540. * param.s1.powi(4) * param.s2
                                        + -73400. * param.s1.powi(3) * param.s2.powi(2)
                                        + 92730. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1785. * param.s1 * param.s2.powi(4)
                                        + 3122. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (11067. * param.s1.powi(6)
                                        + -9192. * param.s1.powi(5) * param.s2
                                        + -201500. * param.s1.powi(4) * param.s2.powi(2)
                                        + 116340. * param.s1.powi(3) * param.s2.powi(3)
                                        + 92895. * param.s1.powi(2) * param.s2.powi(4)
                                        + -13348. * param.s1 * param.s2.powi(5)
                                        + 3738. * param.s2.powi(6)))
                        + param.m2_2
                            * param.s12.powi(4)
                            * (-11. * param.s12.powi(9)
                                + 2. * param.s12.powi(8) * (61. * param.s1 + 59. * param.s2)
                                + -5.
                                    * param.s12.powi(7)
                                    * (102. * param.s1.powi(2)
                                        + 194. * param.s1 * param.s2
                                        + 115. * param.s2.powi(2))
                                + 2. * param.s12.powi(6)
                                    * (539. * param.s1.powi(3)
                                        + 965. * param.s1.powi(2) * param.s2
                                        + 1650. * param.s1 * param.s2.powi(2)
                                        + 770. * param.s2.powi(3))
                                + param.s12.powi(5)
                                    * (-1204. * param.s1.powi(4)
                                        + 362. * param.s1.powi(3) * param.s2
                                        + 4775. * param.s1.powi(2) * param.s2.powi(2)
                                        + -2620. * param.s1 * param.s2.powi(3)
                                        + -2387. * param.s2.powi(4))
                                + -12.
                                    * (param.s1 - param.s2).powi(5)
                                    * (3. * param.s1.powi(4)
                                        + -19. * param.s1.powi(3) * param.s2
                                        + 60. * param.s1.powi(2) * param.s2.powi(2)
                                        + 30. * param.s1 * param.s2.powi(3)
                                        + -4. * param.s2.powi(4))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (207. * param.s1.powi(5)
                                        + -269. * param.s1.powi(4) * param.s2
                                        + -6223. * param.s1.powi(3) * param.s2.powi(2)
                                        + -28155. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1490. * param.s1 * param.s2.powi(4)
                                        + -190. * param.s2.powi(5))
                                + 2. * param.s12.powi(4)
                                    * (273. * param.s1.powi(5)
                                        + -2535. * param.s1.powi(4) * param.s2
                                        + -8460. * param.s1.powi(3) * param.s2.powi(2)
                                        + 23280. * param.s1.powi(2) * param.s2.powi(3)
                                        + -3395. * param.s1 * param.s2.powi(4)
                                        + 1043. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (238. * param.s1.powi(6)
                                        + 5402. * param.s1.powi(5) * param.s2
                                        + -45. * param.s1.powi(4) * param.s2.powi(2)
                                        + 57680. * param.s1.powi(3) * param.s2.powi(3)
                                        + -91480. * param.s1.powi(2) * param.s2.powi(4)
                                        + 14478. * param.s1 * param.s2.powi(5)
                                        + -833. * param.s2.powi(6))
                                + -10.
                                    * param.s12.powi(2)
                                    * (43. * param.s1.powi(7)
                                        + 129. * param.s1.powi(6) * param.s2
                                        + -1648. * param.s1.powi(5) * param.s2.powi(2)
                                        + 9854. * param.s1.powi(4) * param.s2.powi(3)
                                        + -7687. * param.s1.powi(3) * param.s2.powi(4)
                                        + -1603. * param.s1.powi(2) * param.s2.powi(5)
                                        + 904. * param.s1 * param.s2.powi(6)
                                        + 8. * param.s2.powi(7)))
                        + param.s12.powi(5)
                            * (-15. * param.s12.powi(9)
                                + param.s12.powi(8) * (153. * param.s1 + 152. * param.s2)
                                + param.s12.powi(6)
                                    * (1764. * param.s1.powi(3)
                                        + 3048. * param.s1.powi(2) * param.s2
                                        + 3275. * param.s1 * param.s2.powi(2)
                                        + 1890. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (11. * param.s1.powi(5)
                                        + -76. * param.s1.powi(4) * param.s2
                                        + 243. * param.s1.powi(3) * param.s2.powi(2)
                                        + -558. * param.s1.powi(2) * param.s2.powi(3)
                                        + -274. * param.s1 * param.s2.powi(4)
                                        + 24. * param.s2.powi(5))
                                + 5. * param.s12.powi(4)
                                    * (630. * param.s1.powi(5)
                                        + 128. * param.s1.powi(4) * param.s2
                                        + -195. * param.s1.powi(3) * param.s2.powi(2)
                                        + -654. * param.s1.powi(2) * param.s2.powi(3)
                                        + 525. * param.s1 * param.s2.powi(4)
                                        + 812. * param.s2.powi(5))
                                + param.s12.powi(3)
                                    * (-2268. * param.s1.powi(6)
                                        + 3468. * param.s1.powi(5) * param.s2
                                        + 2815. * param.s1.powi(4) * param.s2.powi(2)
                                        + 3360. * param.s1.powi(3) * param.s2.powi(3)
                                        + -5790. * param.s1.powi(2) * param.s2.powi(4)
                                        + 3532. * param.s1 * param.s2.powi(5)
                                        + -3297. * param.s2.powi(6))
                                + param.s12.powi(2)
                                    * (1044. * param.s1.powi(7)
                                        + -4120. * param.s1.powi(6) * param.s2
                                        + 2965. * param.s1.powi(5) * param.s2.powi(2)
                                        + 7390. * param.s1.powi(4) * param.s2.powi(3)
                                        + -35330. * param.s1.powi(3) * param.s2.powi(4)
                                        + 28336. * param.s1.powi(2) * param.s2.powi(5)
                                        + -7055. * param.s1 * param.s2.powi(6)
                                        + 1730. * param.s2.powi(7))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (279. * param.s1.powi(6)
                                        + -1414. * param.s1.powi(5) * param.s2
                                        + 2108. * param.s1.powi(4) * param.s2.powi(2)
                                        + 3950. * param.s1.powi(3) * param.s2.powi(3)
                                        + 11987. * param.s1.powi(2) * param.s2.powi(4)
                                        + -3580. * param.s1 * param.s2.powi(5)
                                        + 530. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (2898. * param.s1.powi(4)
                                        + 3700. * param.s1.powi(3) * param.s2
                                        + 4009. * param.s1.powi(2) * param.s2.powi(2)
                                        + 4960. * param.s1 * param.s2.powi(3)
                                        + 3367. * param.s2.powi(4))
                                - param.s12.powi(7)
                                    * (684. * param.s1.powi(2)
                                        + 1100. * param.s1 * param.s2
                                        + 695. * param.s2.powi(2))))
                + 70.
                    * param.m0_2.powi(4)
                    * param.s12.powi(4)
                    * (param.m2_2
                        * param.s12
                        * (-140. * param.s12.powi(5)
                            + 3. * (param.s1 - param.s2).powi(4)
                                * (23. * param.s1 + 22. * param.s2)
                            + param.s12.powi(4) * (119. * param.s1 + 134. * param.s2)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (47. * param.s1.powi(2)
                                    + 404. * param.s1 * param.s2
                                    + 44. * param.s2.powi(2))
                            + 2. * param.s12.powi(3)
                                * (207. * param.s1.powi(2)
                                    + -634. * param.s1 * param.s2
                                    + 186. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-556. * param.s1.powi(3)
                                    + 724. * param.s1.powi(2) * param.s2
                                    + 712. * param.s1 * param.s2.powi(2)
                                    + -520. * param.s2.powi(3)))
                        + 2. * param.m2_2.powi(2)
                            * (65. * param.s12.powi(5)
                                + param.s12.powi(4) * (73. * param.s1 + -140. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(5)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (83. * param.s1 + 46. * param.s2)
                                + param.s12.powi(3)
                                    * (-252. * param.s1.powi(2)
                                        + 257. * param.s1 * param.s2
                                        + 33. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (28. * param.s1.powi(3)
                                        + 281. * param.s1.powi(2) * param.s2
                                        + -400. * param.s1 * param.s2.powi(2)
                                        + 91. * param.s2.powi(3)))
                        + param.m1_2.powi(2)
                            * (55. * param.s12.powi(5)
                                + 6. * (param.s1 - param.s2).powi(5)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (31. * param.s1 + 227. * param.s2)
                                + param.s12.powi(4) * (-214. * param.s1 + 470. * param.s2)
                                + param.s12.powi(3)
                                    * (306. * param.s1.powi(2)
                                        + -866. * param.s1 * param.s2
                                        + 276. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(2)
                                    * (46. * param.s1.powi(3)
                                        + -73. * param.s1.powi(2) * param.s2
                                        + -115. * param.s1 * param.s2.powi(2)
                                        + 142. * param.s2.powi(3)))
                        + param.s12.powi(2)
                            * (28. * param.s12.powi(5)
                                + param.s12.powi(4) * (-79. * param.s1 + 56. * param.s2)
                                + param.s12.powi(3)
                                    * (36. * param.s1.powi(2)
                                        + 286. * param.s1 * param.s2
                                        + -258. * param.s2.powi(2))
                                + 3. * (param.s1 - param.s2).powi(3)
                                    * (11. * param.s1.powi(2)
                                        + 53. * param.s1 * param.s2
                                        + 26. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (43. * param.s1.powi(3)
                                        + -340. * param.s1.powi(2) * param.s2
                                        + 188. * param.s1 * param.s2.powi(2)
                                        + 79. * param.s2.powi(3))
                                + param.s12
                                    * (-104. * param.s1.powi(4)
                                        + 278. * param.s1.powi(3) * param.s2
                                        + 390. * param.s1.powi(2) * param.s2.powi(2)
                                        + -658. * param.s1 * param.s2.powi(3)
                                        + 94. * param.s2.powi(4)))
                        + param.m1_2
                            * (param.m2_2
                                * (265. * param.s12.powi(5)
                                    + -12. * (param.s1 - param.s2).powi(5)
                                    + param.s12.powi(4) * (-562. * param.s1 + 260. * param.s2)
                                    + 4. * param.s12.powi(3)
                                        * (27. * param.s1.powi(2)
                                            + 268. * param.s1 * param.s2
                                            + -243. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (199. * param.s1.powi(3)
                                            + -832. * param.s1.powi(2) * param.s2
                                            + 575. * param.s1 * param.s2.powi(2)
                                            + 58. * param.s2.powi(3))
                                    - param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (197. * param.s1 + 319. * param.s2))
                                + param.s12
                                    * (-83. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (293. * param.s1 + -436. * param.s2)
                                        + -3.
                                            * (param.s1 - param.s2).powi(4)
                                            * (13. * param.s1 + 32. * param.s2)
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (73. * param.s1.powi(2)
                                                + -626. * param.s1 * param.s2
                                                + -437. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-342. * param.s1.powi(2)
                                                + 220. * param.s1 * param.s2
                                                + 492. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (49. * param.s1.powi(3)
                                                + 464. * param.s1.powi(2) * param.s2
                                                + -973. * param.s1 * param.s2.powi(2)
                                                + 280. * param.s2.powi(3)))))
                + 35.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (-3.
                        * param.m1_2.powi(3)
                        * (5. * param.s12.powi(6)
                            + (param.s1 - param.s2).powi(6)
                            + -10.
                                * param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (param.s1 + 3. * param.s2)
                            + param.s12.powi(5) * (-26. * param.s1 + 210. * param.s2)
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (35. * param.s1.powi(2)
                                    + -134. * param.s1 * param.s2
                                    + -421. * param.s2.powi(2))
                            + param.s12.powi(4)
                                * (55. * param.s1.powi(2)
                                    + -622. * param.s1 * param.s2
                                    + 527. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(3)
                                * (15. * param.s1.powi(3)
                                    + -153. * param.s1.powi(2) * param.s2
                                    + 121. * param.s1 * param.s2.powi(2)
                                    + 73. * param.s2.powi(3)))
                        + param.m2_2.powi(3)
                            * (-125. * param.s12.powi(6)
                                + 3. * (param.s1 - param.s2).powi(6)
                                + -6.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (13. * param.s1 + 7. * param.s2)
                                + param.s12.powi(5) * (-646. * param.s1 + 470. * param.s2)
                                + param.s12.powi(4)
                                    * (837. * param.s1.powi(2)
                                        + 502. * param.s1 * param.s2
                                        + -627. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (173. * param.s1.powi(3)
                                        + -665. * param.s1.powi(2) * param.s2
                                        + 247. * param.s1 * param.s2.powi(2)
                                        + 77. * param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (683. * param.s1.powi(2)
                                        + 890. * param.s1 * param.s2
                                        + -13. * param.s2.powi(2)))
                        + param.m2_2.powi(2)
                            * param.s12
                            * (199. * param.s12.powi(6)
                                + param.s12.powi(5) * (506. * param.s1 + -554. * param.s2)
                                + -3.
                                    * (param.s1 - param.s2).powi(5)
                                    * (11. * param.s1 + 5. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (303. * param.s1.powi(2)
                                        + 638. * param.s1 * param.s2
                                        + 91. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-2007. * param.s1.powi(2)
                                        + 2078. * param.s1 * param.s2
                                        + 241. * param.s2.powi(2))
                                + 4. * param.s12.powi(3)
                                    * (257. * param.s1.powi(3)
                                        + 787. * param.s1.powi(2) * param.s2
                                        + -1257. * param.s1 * param.s2.powi(2)
                                        + 149. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (913. * param.s1.powi(4)
                                        + -5364. * param.s1.powi(3) * param.s2
                                        + 3374. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1756. * param.s1 * param.s2.powi(3)
                                        + -679. * param.s2.powi(4)))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (-77. * param.s12.powi(6)
                                + 2. * param.s12.powi(5) * (param.s1 + 11. * param.s2)
                                + -3.
                                    * (param.s1 - param.s2).powi(4)
                                    * (35. * param.s1.powi(2)
                                        + 114. * param.s1 * param.s2
                                        + 31. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (657. * param.s1.powi(2)
                                        + -2074. * param.s1 * param.s2
                                        + 589. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (57. * param.s1.powi(3)
                                        + -1055. * param.s1.powi(2) * param.s2
                                        + -1025. * param.s1 * param.s2.powi(2)
                                        + 43. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (277. * param.s1.powi(3)
                                        + -475. * param.s1.powi(2) * param.s2
                                        + -495. * param.s1 * param.s2.powi(2)
                                        + 259. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (517. * param.s1.powi(4)
                                        + 2412. * param.s1.powi(3) * param.s2
                                        + -7162. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2284. * param.s1 * param.s2.powi(3)
                                        + 509. * param.s2.powi(4)))
                        + param.s12.powi(3)
                            * (3. * param.s12.powi(6)
                                + param.s12.powi(5) * (-6. * param.s1 + 62. * param.s2)
                                + param.s12.powi(4)
                                    * (-15. * param.s1.powi(2)
                                        + 214. * param.s1 * param.s2
                                        + -203. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(3)
                                    * (3. * param.s1.powi(3)
                                        + 79. * param.s1.powi(2) * param.s2
                                        + 133. * param.s1 * param.s2.powi(2)
                                        + 25. * param.s2.powi(3))
                                + 4. * param.s12.powi(3)
                                    * (15. * param.s1.powi(3)
                                        + -201. * param.s1.powi(2) * param.s2
                                        + 155. * param.s1 * param.s2.powi(2)
                                        + 33. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (-75. * param.s1.powi(4)
                                        + 508. * param.s1.powi(3) * param.s2
                                        + 1094. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1684. * param.s1 * param.s2.powi(3)
                                        + 157. * param.s2.powi(4))
                                + 2. * param.s12
                                    * (21. * param.s1.powi(5)
                                        + 115. * param.s1.powi(4) * param.s2
                                        + -898. * param.s1.powi(3) * param.s2.powi(2)
                                        + 534. * param.s1.powi(2) * param.s2.powi(3)
                                        + 341. * param.s1 * param.s2.powi(4)
                                        + -113. * param.s2.powi(5)))
                        + param.m1_2
                            * (param.m2_2.powi(2)
                                * (-665. * param.s12.powi(6)
                                    + -9. * (param.s1 - param.s2).powi(6)
                                    + 6. * param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (31. * param.s1 + 29. * param.s2)
                                    + 10.
                                        * param.s12.powi(5)
                                        * (77. * param.s1 + 83. * param.s2)
                                    + param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (721. * param.s1.powi(2)
                                            + 3262. * param.s1 * param.s2
                                            + 697. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (1521. * param.s1.powi(2)
                                            + -5618. * param.s1 * param.s2
                                            + 1353. * param.s2.powi(2))
                                    + -4.
                                        * param.s12.powi(3)
                                        * (631. * param.s1.powi(3)
                                            + -871. * param.s1.powi(2) * param.s2
                                            + -859. * param.s1 * param.s2.powi(2)
                                            + 595. * param.s2.powi(3)))
                                + param.s12.powi(2)
                                    * (-21. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (90. * param.s1 + -538. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(4)
                                            * (5. * param.s1.powi(2)
                                                + 94. * param.s1 * param.s2
                                                + 81. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (-135. * param.s1.powi(2)
                                                + 358. * param.s1 * param.s2
                                                + 637. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (27. * param.s1.powi(3)
                                                + -241. * param.s1.powi(2) * param.s2
                                                + -1579. * param.s1 * param.s2.powi(2)
                                                + -187. * param.s2.powi(3))
                                        + 4. * param.s12.powi(3)
                                            * (15. * param.s1.powi(3)
                                                + 483. * param.s1.powi(2) * param.s2
                                                + -1261. * param.s1 * param.s2.powi(2)
                                                + 321. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (45. * param.s1.powi(4)
                                                + -2564. * param.s1.powi(3) * param.s2
                                                + 3062. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2876. * param.s1 * param.s2.powi(3)
                                                + -1979. * param.s2.powi(4)))
                                + 4. * param.m2_2
                                    * param.s12
                                    * (103. * param.s12.powi(6)
                                        + 3. * (param.s1 - param.s2).powi(5)
                                            * (3. * param.s1 + 5. * param.s2)
                                        + param.s12.powi(5)
                                            * (-280. * param.s1 + 226. * param.s2)
                                        + param.s12.powi(4)
                                            * (99. * param.s1.powi(2)
                                                + 1106. * param.s1 * param.s2
                                                + -965. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (48. * param.s1.powi(2)
                                                + 325. * param.s1 * param.s2
                                                + 143. * param.s2.powi(2))
                                        + 4. * param.s12.powi(3)
                                            * (86. * param.s1.powi(3)
                                                + -647. * param.s1.powi(2) * param.s2
                                                + 378. * param.s1 * param.s2.powi(2)
                                                + 131. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-371. * param.s1.powi(4)
                                                + 924. * param.s1.powi(3) * param.s2
                                                + 1646. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2612. * param.s1 * param.s2.powi(3)
                                                + 413. * param.s2.powi(4))))
                        + param.m1_2.powi(2)
                            * (param.m2_2
                                * (-395. * param.s12.powi(6)
                                    + 2. * param.s12.powi(5)
                                        * (739. * param.s1 + -935. * param.s2)
                                    + 9. * (param.s1 - param.s2).powi(6)
                                    + -6.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (23. * param.s1 + 37. * param.s2)
                                    + param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (67. * param.s1.powi(2)
                                            + -2774. * param.s1 * param.s2
                                            + -1973. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-1953. * param.s1.powi(2)
                                            + 1330. * param.s1 * param.s2
                                            + 2535. * param.s2.powi(2))
                                    + 4. * param.s12.powi(3)
                                        * (233. * param.s1.powi(3)
                                            + 793. * param.s1.powi(2) * param.s2
                                            + -2009. * param.s1 * param.s2.powi(2)
                                            + 479. * param.s2.powi(3)))
                                + param.s12
                                    * (33. * param.s12.powi(6)
                                        + -2.
                                            * param.s12.powi(5)
                                            * (81. * param.s1 + -553. * param.s2)
                                        + -3.
                                            * (param.s1 - param.s2).powi(5)
                                            * (param.s1 + 15. * param.s2)
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (9. * param.s1.powi(2)
                                                + 422. * param.s1 * param.s2
                                                + 601. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (315. * param.s1.powi(2)
                                                + -2438. * param.s1 * param.s2
                                                + 787. * param.s2.powi(2))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (75. * param.s1.powi(3)
                                                + -177. * param.s1.powi(2) * param.s2
                                                + -1103. * param.s1 * param.s2.powi(2)
                                                + 933. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (135. * param.s1.powi(4)
                                                + 1444. * param.s1.powi(3) * param.s2
                                                + -6670. * param.s1.powi(2) * param.s2.powi(2)
                                                + 4532. * param.s1 * param.s2.powi(3)
                                                + 559. * param.s2.powi(4)))))
                + 21.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m1_2.powi(4)
                        * (-5. * param.s12.powi(7)
                            + 2. * (param.s1 - param.s2).powi(7)
                            + 4. * param.s12.powi(6) * (8. * param.s1 + 35. * param.s2)
                            + 20.
                                * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(3)
                                * (3. * param.s1.powi(2)
                                    + 13. * param.s1 * param.s2
                                    + 21. * param.s2.powi(2))
                            + param.s12.powi(5)
                                * (-87. * param.s1.powi(2)
                                    + -566. * param.s1 * param.s2
                                    + 1937. * param.s2.powi(2))
                            + 2. * param.s12.powi(4)
                                * (65. * param.s1.powi(3)
                                    + 425. * param.s1.powi(2) * param.s2
                                    + -1959. * param.s1 * param.s2.powi(2)
                                    + 385. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (-115. * param.s1.powi(4)
                                    + -540. * param.s1.powi(3) * param.s2
                                    + 2114. * param.s1.powi(2) * param.s2.powi(2)
                                    + 996. * param.s1 * param.s2.powi(3)
                                    + -2455. * param.s2.powi(4))
                            - param.s12
                                * (param.s1 - param.s2).powi(5)
                                * (17. * param.s1 + 35. * param.s2))
                        + param.m2_2.powi(3)
                            * param.s12
                            * (-40. * param.s12.powi(7)
                                + param.s12.powi(6) * (-1053. * param.s1 + 190. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (729. * param.s1.powi(2)
                                        + 557. * param.s1 * param.s2
                                        + -174. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (149. * param.s1.powi(2)
                                        + 141. * param.s1 * param.s2
                                        + 10. * param.s2.powi(2))
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (225. * param.s1.powi(3)
                                        + 1064. * param.s1.powi(2) * param.s2
                                        + 277. * param.s1 * param.s2.powi(2)
                                        + -6. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (2085. * param.s1.powi(3)
                                        + -9320. * param.s1.powi(2) * param.s2
                                        + 2777. * param.s1 * param.s2.powi(2)
                                        + 290. * param.s2.powi(3))
                                + -4.
                                    * param.s12.powi(3)
                                    * (965. * param.s1.powi(4)
                                        + -1445. * param.s1.powi(3) * param.s2
                                        + -1501. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1121. * param.s1 * param.s2.powi(3)
                                        + 20. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(6)
                                    * (13. * param.s1 + 2. * param.s2))
                        + 2. * param.m2_2.powi(4)
                            * (10. * param.s12.powi(7)
                                + param.s12.powi(6) * (321. * param.s1 + -55. * param.s2)
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
                                    * (27. * param.s1.powi(2)
                                        + -439. * param.s1 * param.s2
                                        + 63. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-975. * param.s1.powi(3)
                                        + 1005. * param.s1.powi(2) * param.s2
                                        + 721. * param.s1 * param.s2.powi(2)
                                        + -155. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (220. * param.s1.powi(4)
                                        + 400. * param.s1.powi(3) * param.s2
                                        + -619. * param.s1.powi(2) * param.s2.powi(2)
                                        + -56. * param.s1 * param.s2.powi(3)
                                        + 55. * param.s2.powi(4)))
                        + param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (20. * param.s12.powi(7)
                                + param.s12.powi(6) * (447. * param.s1 + -50. * param.s2)
                                + (param.s1 - param.s2).powi(5)
                                    * (47. * param.s1.powi(2)
                                        + 71. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(5)
                                    * (681. * param.s1.powi(2)
                                        + -737. * param.s1 * param.s2
                                        + 24. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (219. * param.s1.powi(3)
                                        + 1562. * param.s1.powi(2) * param.s2
                                        + 779. * param.s1 * param.s2.powi(2)
                                        + 20. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (625. * param.s1.powi(3)
                                        + 4840. * param.s1.powi(2) * param.s2
                                        + -5483. * param.s1 * param.s2.powi(2)
                                        + 290. * param.s2.powi(3))
                                + 4. * param.s12.powi(3)
                                    * (370. * param.s1.powi(4)
                                        + -3105. * param.s1.powi(3) * param.s2
                                        + 1971. * param.s1.powi(2) * param.s2.powi(2)
                                        + 799. * param.s1 * param.s2.powi(3)
                                        + -95. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(2)
                                    * (339. * param.s1.powi(5)
                                        + -902. * param.s1.powi(4) * param.s2
                                        + -1638. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2616. * param.s1.powi(2) * param.s2.powi(3)
                                        + -373. * param.s1 * param.s2.powi(4)
                                        + -42. * param.s2.powi(5)))
                        + param.m2_2
                            * param.s12.powi(3)
                            * (-3. * param.s12.powi(6) * (11. * param.s1 + 10. * param.s2)
                                + 2. * param.s12.powi(5)
                                    * (69. * param.s1.powi(2)
                                        + -443. * param.s1 * param.s2
                                        + 66. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-195. * param.s1.powi(3)
                                        + 820. * param.s1.powi(2) * param.s2
                                        + 917. * param.s1 * param.s2.powi(2)
                                        + -210. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (9. * param.s1.powi(3)
                                        + 148. * param.s1.powi(2) * param.s2
                                        + 137. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (51. * param.s1.powi(4)
                                        + -403. * param.s1.powi(3) * param.s2
                                        + -2577. * param.s1.powi(2) * param.s2.powi(2)
                                        + -401. * param.s1 * param.s2.powi(3)
                                        + 30. * param.s2.powi(4))
                                + 4. * param.s12.powi(3)
                                    * (15. * param.s1.powi(4)
                                        + 705. * param.s1.powi(3) * param.s2
                                        + -2099. * param.s1.powi(2) * param.s2.powi(2)
                                        + 639. * param.s1 * param.s2.powi(3)
                                        + 30. * param.s2.powi(4))
                                + 5. * param.s12.powi(2)
                                    * (21. * param.s1.powi(5)
                                        + -814. * param.s1.powi(4) * param.s2
                                        + 1022. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1008. * param.s1.powi(2) * param.s2.powi(3)
                                        + -763. * param.s1 * param.s2.powi(4)
                                        + 6. * param.s2.powi(5)))
                        + param.s12.powi(4)
                            * (-3. * param.s1 * param.s12.powi(6)
                                + 6. * param.s12.powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 9. * param.s1 * param.s2
                                        + 2. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(4)
                                        + -19. * param.s1.powi(3) * param.s2
                                        + -169. * param.s1.powi(2) * param.s2.powi(2)
                                        + -109. * param.s1 * param.s2.powi(3)
                                        + -4. * param.s2.powi(4))
                                + 4. * param.s12.powi(3)
                                    * (15. * param.s1.powi(4)
                                        + 15. * param.s1.powi(3) * param.s2
                                        + 146. * param.s1.powi(2) * param.s2.powi(2)
                                        + -261. * param.s1 * param.s2.powi(3)
                                        + 30. * param.s2.powi(4))
                                + -5.
                                    * param.s12.powi(2)
                                    * (9. * param.s1.powi(5)
                                        + -36. * param.s1.powi(4) * param.s2
                                        + 386. * param.s1.powi(3) * param.s2.powi(2)
                                        + -312. * param.s1.powi(2) * param.s2.powi(3)
                                        + -111. * param.s1 * param.s2.powi(4)
                                        + 24. * param.s2.powi(5))
                                + 2. * param.s12
                                    * (9. * param.s1.powi(6)
                                        + -105. * param.s1.powi(5) * param.s2
                                        + 330. * param.s1.powi(4) * param.s2.powi(2)
                                        + 770. * param.s1.powi(3) * param.s2.powi(3)
                                        + -1225. * param.s1.powi(2) * param.s2.powi(4)
                                        + 191. * param.s1 * param.s2.powi(5)
                                        + 30. * param.s2.powi(6))
                                - param.s12.powi(4)
                                    * (45. * param.s1.powi(3)
                                        + 150. * param.s1.powi(2) * param.s2
                                        + -347. * param.s1 * param.s2.powi(2)
                                        + 60. * param.s2.powi(3)))
                        + param.m1_2.powi(3)
                            * (param.m2_2
                                * (95. * param.s12.powi(7)
                                    + -518. * param.s12.powi(6) * (param.s1 + -5. * param.s2)
                                    + -8. * (param.s1 - param.s2).powi(7)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(5)
                                        * (83. * param.s1 + 125. * param.s2)
                                    + -10.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(3)
                                        * (39. * param.s1.powi(2)
                                            + 134. * param.s1 * param.s2
                                            + 123. * param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (1173. * param.s1.powi(2)
                                            + -7066. * param.s1 * param.s2
                                            + 157. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-1420. * param.s1.powi(3)
                                            + 5780. * param.s1.powi(2) * param.s2
                                            + 8412. * param.s1 * param.s2.powi(2)
                                            + -7460. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (985. * param.s1.powi(4)
                                            + -900. * param.s1.powi(3) * param.s2
                                            + -10226. * param.s1.powi(2) * param.s2.powi(2)
                                            + 6636. * param.s1 * param.s2.powi(3)
                                            + 3505. * param.s2.powi(4)))
                                + param.s12
                                    * (15. * param.s12.powi(7)
                                        + -3.
                                            * (param.s1 + -6. * param.s2)
                                            * (param.s1 - param.s2).powi(6)
                                        + -3.
                                            * param.s12.powi(6)
                                            * (31. * param.s1 + 120. * param.s2)
                                        + 3. * param.s12.powi(5)
                                            * (81. * param.s1.powi(2)
                                                + 428. * param.s1 * param.s2
                                                + -1181. * param.s2.powi(2))
                                        + 3. * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (11. * param.s1.powi(2)
                                                + -56. * param.s1 * param.s2
                                                + -155. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (27. * param.s1.powi(3)
                                                + -42. * param.s1.powi(2) * param.s2
                                                + 859. * param.s1 * param.s2.powi(2)
                                                + 716. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (-345. * param.s1.powi(3)
                                                + -1500. * param.s1.powi(2) * param.s2
                                                + 2587. * param.s1 * param.s2.powi(2)
                                                + 3650. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (285. * param.s1.powi(4)
                                                + 360. * param.s1.powi(3) * param.s2
                                                + 5554. * param.s1.powi(2) * param.s2.powi(2)
                                                + -13824. * param.s1 * param.s2.powi(3)
                                                + 4265. * param.s2.powi(4))))
                        + param.m1_2
                            * (param.m2_2.powi(3)
                                * (545. * param.s12.powi(7)
                                    + 2. * param.s12.powi(6)
                                        * (331. * param.s1 + -955. * param.s2)
                                    + -8. * (param.s1 - param.s2).powi(7)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(5)
                                        * (113. * param.s1 + 95. * param.s2)
                                    + -10.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(3)
                                        * (93. * param.s1.powi(2)
                                            + 146. * param.s1 * param.s2
                                            + 57. * param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (-4617. * param.s1.powi(2)
                                            + 4514. * param.s1 * param.s2
                                            + 2127. * param.s2.powi(2))
                                    + 4. * param.s12.powi(4)
                                        * (1085. * param.s1.powi(3)
                                            + 1315. * param.s1.powi(2) * param.s2
                                            + -2677. * param.s1 * param.s2.powi(2)
                                            + -75. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (-105. * param.s1.powi(4)
                                            + -8780. * param.s1.powi(3) * param.s2
                                            + 4354. * param.s1.powi(2) * param.s2.powi(2)
                                            + 5476. * param.s1 * param.s2.powi(3)
                                            + -945. * param.s2.powi(4)))
                                + param.m2_2.powi(2)
                                    * param.s12
                                    * (-495. * param.s12.powi(7)
                                        + (param.s1 - param.s2).powi(6)
                                            * (23. * param.s1 + 22. * param.s2)
                                        + param.s12.powi(6)
                                            * (333. * param.s1 + 440. * param.s2)
                                        + param.s12.powi(5)
                                            * (3297. * param.s1.powi(2)
                                                + -11924. * param.s1 * param.s2
                                                + 2923. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (-6595. * param.s1.powi(3)
                                                + 12300. * param.s1.powi(2) * param.s2
                                                + 12713. * param.s1 * param.s2.powi(2)
                                                + -6170. * param.s2.powi(3))
                                        + -5.
                                            * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (45. * param.s1.powi(3)
                                                + 2314. * param.s1.powi(2) * param.s2
                                                + 2253. * param.s1 * param.s2.powi(2)
                                                + 68. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (4075. * param.s1.powi(4)
                                                + 9800. * param.s1.powi(3) * param.s2
                                                + -37114.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(2)
                                                + 9184. * param.s1 * param.s2.powi(3)
                                                + 3975. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (413. * param.s1.powi(2)
                                                + 1032. * param.s1 * param.s2
                                                + 355. * param.s2.powi(2)))
                                + param.m2_2
                                    * param.s12.powi(2)
                                    * (45. * param.s12.powi(7)
                                        + -34.
                                            * param.s12.powi(6)
                                            * (3. * param.s1 + -25. * param.s2)
                                        + param.s12.powi(5)
                                            * (-153. * param.s1.powi(2)
                                                + 3186. * param.s1 * param.s2
                                                + -2717. * param.s2.powi(2))
                                        + -12.
                                            * (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(2)
                                                + 13. * param.s1 * param.s2
                                                + 6. * param.s2.powi(2))
                                        + 4. * param.s12.powi(4)
                                            * (180. * param.s1.powi(3)
                                                + -2815. * param.s1.powi(2) * param.s2
                                                + 2272. * param.s1 * param.s2.powi(2)
                                                + 385. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-885. * param.s1.powi(4)
                                                + 6260. * param.s1.powi(3) * param.s2
                                                + 16346. * param.s1.powi(2) * param.s2.powi(2)
                                                + -24156. * param.s1 * param.s2.powi(3)
                                                + 2435. * param.s2.powi(4))
                                        + 10.
                                            * param.s12.powi(2)
                                            * (45. * param.s1.powi(5)
                                                + 417. * param.s1.powi(4) * param.s2
                                                + -2690. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1674. * param.s1.powi(2) * param.s2.powi(3)
                                                + 861. * param.s1 * param.s2.powi(4)
                                                + -307. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (63. * param.s1.powi(3)
                                                + 3299. * param.s1.powi(2) * param.s2
                                                + 6113. * param.s1 * param.s2.powi(2)
                                                + 845. * param.s2.powi(3)))
                                + param.s12.powi(3)
                                    * (5. * param.s12.powi(7)
                                        + param.s12.powi(5)
                                            * (33. * param.s1.powi(2)
                                                + 44. * param.s1 * param.s2
                                                + -233. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (7. * param.s1.powi(3)
                                                + -96. * param.s1.powi(2) * param.s2
                                                + -609. * param.s1 * param.s2.powi(2)
                                                + -202. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (5. * param.s1.powi(3)
                                                + 500. * param.s1.powi(2) * param.s2
                                                + -3243. * param.s1 * param.s2.powi(2)
                                                + 1430. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-65. * param.s1.powi(4)
                                                + -840. * param.s1.powi(3) * param.s2
                                                + 3214. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2496. * param.s1 * param.s2.powi(3)
                                                + -1965. * param.s2.powi(4))
                                        + 5. * param.s12.powi(2)
                                            * (15. * param.s1.powi(5)
                                                + 56. * param.s1.powi(4) * param.s2
                                                + 810. * param.s1.powi(3) * param.s2.powi(2)
                                                + -2484. * param.s1.powi(2) * param.s2.powi(3)
                                                + 975. * param.s1 * param.s2.powi(4)
                                                + 148. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (37. * param.s1.powi(4)
                                                + -146. * param.s1.powi(3) * param.s2
                                                + 3276. * param.s1.powi(2) * param.s2.powi(2)
                                                + 3738. * param.s1 * param.s2.powi(3)
                                                + -305. * param.s2.powi(4))
                                        - param.s12.powi(6) * (23. * param.s1 + 80. * param.s2)))
                        + param.m1_2.powi(2)
                            * (param.m2_2.powi(2)
                                * (845. * param.s12.powi(7)
                                    + 12. * (param.s1 - param.s2).powi(7)
                                    + -3.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(5)
                                        * (49. * param.s1 + 55. * param.s2)
                                    + param.s12.powi(6) * (-2918. * param.s1 + 790. * param.s2)
                                    + param.s12.powi(5)
                                        * (3123. * param.s1.powi(2)
                                            + 7274. * param.s1 * param.s2
                                            + -6573. * param.s2.powi(2))
                                    + 30.
                                        * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(3)
                                        * (31. * param.s1.powi(2)
                                            + 74. * param.s1 * param.s2
                                            + 43. * param.s2.powi(2))
                                    + -8.
                                        * param.s12.powi(4)
                                        * (25. * param.s1.powi(3)
                                            + 2075. * param.s1.powi(2) * param.s2
                                            + -934. * param.s1 * param.s2.powi(2)
                                            + -800. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (-1645. * param.s1.powi(4)
                                            + 8620. * param.s1.powi(3) * param.s2
                                            + 6234. * param.s1.powi(2) * param.s2.powi(2)
                                            + -12884. * param.s1 * param.s2.powi(3)
                                            + -325. * param.s2.powi(4)))
                                + param.m2_2
                                    * param.s12
                                    * (-140. * param.s12.powi(7)
                                        + param.s12.powi(6)
                                            * (653. * param.s1 + -3110. * param.s2)
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (41. * param.s1.powi(2)
                                                + 459. * param.s1 * param.s2
                                                + 400. * param.s2.powi(2))
                                        + param.s12.powi(5)
                                            * (-1158. * param.s1.powi(2)
                                                + 2966. * param.s1 * param.s2
                                                + 4648. * param.s2.powi(2))
                                        + -5.
                                            * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (33. * param.s1.powi(3)
                                                + -848. * param.s1.powi(2) * param.s2
                                                + -3195. * param.s1 * param.s2.powi(2)
                                                + -670. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (895. * param.s1.powi(3)
                                                + 9160. * param.s1.powi(2) * param.s2
                                                + -27957. * param.s1 * param.s2.powi(2)
                                                + 5430. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (40. * param.s1.powi(4)
                                                + 3545. * param.s1.powi(3) * param.s2
                                                + -4559. * param.s1.powi(2) * param.s2.powi(2)
                                                + -4281. * param.s1 * param.s2.powi(3)
                                                + 2735. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(6)
                                            * (7. * param.s1 + 38. * param.s2))
                                + -3.
                                    * param.s12.powi(2)
                                    * (5. * param.s12.powi(7)
                                        + param.s12.powi(5)
                                            * (69. * param.s1.powi(2)
                                                + 272. * param.s1 * param.s2
                                                + -609. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + -34. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (55. * param.s1.powi(4)
                                                + -320. * param.s1.powi(3) * param.s2
                                                + 3822. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2992. * param.s1 * param.s2.powi(3)
                                                + -645. * param.s2.powi(4))
                                        + -5.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(5)
                                                + -68. * param.s1.powi(4) * param.s2
                                                + 194. * param.s1.powi(3) * param.s2.powi(2)
                                                + 652. * param.s1.powi(2) * param.s2.powi(3)
                                                + -973. * param.s1 * param.s2.powi(4)
                                                + 192. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(3)
                                                + 83. * param.s1.powi(2) * param.s2
                                                + 1091. * param.s1 * param.s2.powi(2)
                                                + 545. * param.s2.powi(3))
                                        - param.s12.powi(4)
                                            * (85. * param.s1.powi(3)
                                                + 100. * param.s1.powi(2) * param.s2
                                                + 1409. * param.s1 * param.s2.powi(2)
                                                + -1730. * param.s2.powi(3))
                                        - param.s12.powi(6)
                                            * (29. * param.s1 + 100. * param.s2))))
                + -7.
                    * param.m0_2
                    * param.s12
                    * (param.m2_2.powi(5)
                        * (-10. * param.s12.powi(8)
                            + -3. * (param.s1 - param.s2).powi(8)
                            + 10. * param.s12.powi(7) * (22. * param.s1 + 7. * param.s2)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(6)
                                * (19. * param.s1 + 14. * param.s2)
                            + param.s12.powi(6)
                                * (1557. * param.s1.powi(2)
                                    + -970. * param.s1 * param.s2
                                    + -213. * param.s2.powi(2))
                            + -5.
                                * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(4)
                                * (49. * param.s1.powi(2)
                                    + 54. * param.s1 * param.s2
                                    + 23. * param.s2.powi(2))
                            + 10.
                                * param.s12.powi(3)
                                * (param.s1 - param.s2).powi(2)
                                * (126. * param.s1.powi(3)
                                    + 155. * param.s1.powi(2) * param.s2
                                    + 82. * param.s1 * param.s2.powi(2)
                                    + 27. * param.s2.powi(3))
                            + param.s12.powi(5)
                                * (-2862. * param.s1.powi(3)
                                    + -2404. * param.s1.powi(2) * param.s2
                                    + 1646. * param.s1 * param.s2.powi(2)
                                    + 368. * param.s2.powi(3))
                            + param.s12.powi(4)
                                * (45. * param.s1.powi(4)
                                    + 5180. * param.s1.powi(3) * param.s2
                                    + 414. * param.s1.powi(2) * param.s2.powi(2)
                                    + -1260. * param.s1 * param.s2.powi(3)
                                    + -395. * param.s2.powi(4)))
                        + param.m1_2.powi(5)
                            * (5. * param.s12.powi(8)
                                + 3. * (param.s1 - param.s2).powi(8)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(6)
                                    * (13. * param.s1 + 20. * param.s2)
                                + -2. * param.s12.powi(7) * (19. * param.s1 + 40. * param.s2)
                                + 14.
                                    * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(4)
                                    * (7. * param.s1.powi(2)
                                        + 18. * param.s1 * param.s2
                                        + 20. * param.s2.powi(2))
                                + 2. * param.s12.powi(6)
                                    * (63. * param.s1.powi(2)
                                        + 202. * param.s1 * param.s2
                                        + 420. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(5)
                                    * (119. * param.s1.powi(3)
                                        + 398. * param.s1.powi(2) * param.s2
                                        + 1217. * param.s1 * param.s2.powi(2)
                                        + -1896. * param.s2.powi(3))
                                + -10.
                                    * param.s12.powi(3)
                                    * (param.s1 - param.s2).powi(2)
                                    * (21. * param.s1.powi(3)
                                        + 62. * param.s1.powi(2) * param.s2
                                        + 139. * param.s1 * param.s2.powi(2)
                                        + 168. * param.s2.powi(3))
                                + 8. * param.s12.powi(4)
                                    * (35. * param.s1.powi(4)
                                        + 90. * param.s1.powi(3) * param.s2
                                        + 270. * param.s1.powi(2) * param.s2.powi(2)
                                        + -503. * param.s1 * param.s2.powi(3)
                                        + -390. * param.s2.powi(4)))
                        + param.m2_2.powi(4)
                            * param.s12
                            * (26. * param.s12.powi(8)
                                + 3. * (param.s1 - param.s2).powi(7)
                                    * (5. * param.s1 - param.s2)
                                + -10. * param.s12.powi(7) * (50. * param.s1 + 17. * param.s2)
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (107. * param.s1.powi(2)
                                        + 65. * param.s1 * param.s2
                                        + -16. * param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-2241. * param.s1.powi(2)
                                        + 1790. * param.s1 * param.s2
                                        + 477. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (8190. * param.s1.powi(3)
                                        + -4312. * param.s1.powi(2) * param.s2
                                        + -2014. * param.s1 * param.s2.powi(2)
                                        + -748. * param.s2.powi(3))
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (365. * param.s1.powi(3)
                                        + 445. * param.s1.powi(2) * param.s2
                                        + 109. * param.s1 * param.s2.powi(2)
                                        + -31. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (-6345. * param.s1.powi(4)
                                        + -9760. * param.s1.powi(3) * param.s2
                                        + 14214. * param.s1.powi(2) * param.s2.powi(2)
                                        + 120. * param.s1 * param.s2.powi(3)
                                        + 715. * param.s2.powi(4))
                                + -2.
                                    * param.s12.powi(3)
                                    * (378. * param.s1.powi(5)
                                        + -7435. * param.s1.powi(4) * param.s2
                                        + 4050. * param.s1.powi(3) * param.s2.powi(2)
                                        + 3474. * param.s1.powi(2) * param.s2.powi(3)
                                        + -680. * param.s1 * param.s2.powi(4)
                                        + 213. * param.s2.powi(5)))
                        + param.m2_2.powi(2)
                            * param.s12.powi(3)
                            * (param.s12.powi(8)
                                + -10. * param.s12.powi(7) * (param.s1 + -2. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(5)
                                    * (10. * param.s1.powi(3)
                                        + 64. * param.s1.powi(2) * param.s2
                                        + 7. * param.s1 * param.s2.powi(2)
                                        - param.s2.powi(3))
                                + 2. * param.s12.powi(5)
                                    * (320. * param.s1.powi(3)
                                        + -2131. * param.s1.powi(2) * param.s2
                                        + 743. * param.s1 * param.s2.powi(2)
                                        + 176. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (2. * param.s1.powi(4)
                                        + -1919. * param.s1.powi(3) * param.s2
                                        + -3024. * param.s1.powi(2) * param.s2.powi(2)
                                        + -223. * param.s1 * param.s2.powi(3)
                                        + 4. * param.s2.powi(4))
                                + 2. * param.s12.powi(3)
                                    * (567. * param.s1.powi(5)
                                        + -2300. * param.s1.powi(4) * param.s2
                                        + -9600. * param.s1.powi(3) * param.s2.powi(2)
                                        + 10746. * param.s1.powi(2) * param.s2.powi(3)
                                        + -695. * param.s1 * param.s2.powi(4)
                                        + 162. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(2)
                                    * (85. * param.s1.powi(6)
                                        + 1232. * param.s1.powi(5) * param.s2
                                        + -6100. * param.s1.powi(4) * param.s2.powi(2)
                                        + 3932. * param.s1.powi(3) * param.s2.powi(3)
                                        + 1169. * param.s1.powi(2) * param.s2.powi(4)
                                        + -340. * param.s1 * param.s2.powi(5)
                                        + 22. * param.s2.powi(6))
                                - param.s12.powi(4)
                                    * (1255. * param.s1.powi(4)
                                        + -11610. * param.s1.powi(3) * param.s2
                                        + 6531. * param.s1.powi(2) * param.s2.powi(2)
                                        + 780. * param.s1 * param.s2.powi(3)
                                        + 460. * param.s2.powi(4))
                                - param.s12.powi(6)
                                    * (111. * param.s1.powi(2)
                                        + 500. * param.s1 * param.s2
                                        + 138. * param.s2.powi(2)))
                        + param.m2_2
                            * param.s12.powi(4)
                            * (param.s12.powi(8)
                                + -10. * param.s12.powi(7) * (param.s1 + param.s2)
                                + param.s12.powi(6)
                                    * (24. * param.s1.powi(2)
                                        + 70. * param.s1 * param.s2
                                        + 27. * param.s2.powi(2))
                                + 2. * param.s12.powi(5)
                                    * (5. * param.s1.powi(3)
                                        + 119. * param.s1.powi(2) * param.s2
                                        + 98. * param.s1 * param.s2.powi(2)
                                        + -4. * param.s2.powi(3))
                                + -3.
                                    * (param.s1 - param.s2).powi(4)
                                    * (5. * param.s1.powi(4)
                                        + -74. * param.s1.powi(3) * param.s2
                                        + -333. * param.s1.powi(2) * param.s2.powi(2)
                                        + -52. * param.s1 * param.s2.powi(3)
                                        + 4. * param.s2.powi(4))
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (43. * param.s1.powi(5)
                                        + -269. * param.s1.powi(4) * param.s2
                                        + 2695. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2599. * param.s1.powi(2) * param.s2.powi(3)
                                        + -152. * param.s1 * param.s2.powi(4)
                                        + 34. * param.s2.powi(5))
                                + 2. * param.s12.powi(3)
                                    * (117. * param.s1.powi(5)
                                        + 625. * param.s1.powi(4) * param.s2
                                        + -1830. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1944. * param.s1.powi(2) * param.s2.powi(3)
                                        + 655. * param.s1 * param.s2.powi(4)
                                        + 87. * param.s2.powi(5))
                                + -5.
                                    * param.s12.powi(2)
                                    * (40. * param.s1.powi(6)
                                        + -10. * param.s1.powi(5) * param.s2
                                        + 1583. * param.s1.powi(4) * param.s2.powi(2)
                                        + -3880. * param.s1.powi(3) * param.s2.powi(3)
                                        + 1466. * param.s1.powi(2) * param.s2.powi(4)
                                        + 50. * param.s1 * param.s2.powi(5)
                                        + 31. * param.s2.powi(6))
                                - param.s12.powi(4)
                                    * (130. * param.s1.powi(4)
                                        + 1170. * param.s1.powi(3) * param.s2
                                        + -4779. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1080. * param.s1 * param.s2.powi(3)
                                        + 85. * param.s2.powi(4)))
                        + param.s12.powi(5)
                            * (param.s12.powi(8)
                                + -10. * param.s12.powi(7) * (param.s1 + param.s2)
                                + param.s12.powi(6)
                                    * (42. * param.s1.powi(2)
                                        + 70. * param.s1 * param.s2
                                        + 45. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(5)
                                    * (49. * param.s1.powi(3)
                                        + 79. * param.s1.powi(2) * param.s2
                                        + 100. * param.s1 * param.s2.powi(2)
                                        + 58. * param.s2.powi(3))
                                + 5. * param.s12.powi(4)
                                    * (28. * param.s1.powi(4)
                                        + 18. * param.s1.powi(3) * param.s2
                                        + -9. * param.s1.powi(2) * param.s2.powi(2)
                                        + 36. * param.s1 * param.s2.powi(3)
                                        + 37. * param.s2.powi(4))
                                + 3. * (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(5)
                                        + -11. * param.s1.powi(4) * param.s2
                                        + 79. * param.s1.powi(3) * param.s2.powi(2)
                                        + 259. * param.s1.powi(2) * param.s2.powi(3)
                                        + 34. * param.s1 * param.s2.powi(4)
                                        + -2. * param.s2.powi(5))
                                + -2.
                                    * param.s12.powi(3)
                                    * (63. * param.s1.powi(5)
                                        + -85. * param.s1.powi(4) * param.s2
                                        + -270. * param.s1.powi(3) * param.s2.powi(2)
                                        + 684. * param.s1.powi(2) * param.s2.powi(3)
                                        + -115. * param.s1 * param.s2.powi(4)
                                        + 93. * param.s2.powi(5))
                                + 5. * param.s12.powi(2)
                                    * (14. * param.s1.powi(6)
                                        + -62. * param.s1.powi(5) * param.s2
                                        + -5. * param.s1.powi(4) * param.s2.powi(2)
                                        + -320. * param.s1.powi(3) * param.s2.powi(3)
                                        + 568. * param.s1.powi(2) * param.s2.powi(4)
                                        + -122. * param.s1 * param.s2.powi(5)
                                        + 23. * param.s2.powi(6))
                                + -2.
                                    * param.s12
                                    * (11. * param.s1.powi(7)
                                        + -95. * param.s1.powi(6) * param.s2
                                        + 330. * param.s1.powi(5) * param.s2.powi(2)
                                        + -1470. * param.s1.powi(4) * param.s2.powi(3)
                                        + 1005. * param.s1.powi(3) * param.s2.powi(4)
                                        + 429. * param.s1.powi(2) * param.s2.powi(5)
                                        + -230. * param.s1 * param.s2.powi(6)
                                        + 20. * param.s2.powi(7)))
                        + param.m1_2.powi(3)
                            * (param.m2_2.powi(2)
                                * (395. * param.s12.powi(8)
                                    + 30. * (param.s1 - param.s2).powi(8)
                                    + -44.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (7. * param.s1 + 8. * param.s2)
                                    + param.s12.powi(7)
                                        * (-2318. * param.s1 + 5290. * param.s2)
                                    + param.s12.powi(6)
                                        * (5823. * param.s1.powi(2)
                                            + -10366. * param.s1 * param.s2
                                            + -14373. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (1433. * param.s1.powi(2)
                                            + 2862. * param.s1 * param.s2
                                            + 2005. * param.s2.powi(2))
                                    + -10.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (399. * param.s1.powi(3)
                                            + 1145. * param.s1.powi(2) * param.s2
                                            + 1549. * param.s1 * param.s2.powi(2)
                                            + 807. * param.s2.powi(3))
                                    + param.s12.powi(5)
                                        * (-8200. * param.s1.powi(3)
                                            + -820. * param.s1.powi(2) * param.s2
                                            + 47912. * param.s1 * param.s2.powi(2)
                                            + 6300. * param.s2.powi(3))
                                    + param.s12.powi(4)
                                        * (7135. * param.s1.powi(4)
                                            + 10980. * param.s1.powi(3) * param.s2
                                            + -34446. * param.s1.powi(2) * param.s2.powi(2)
                                            + -32284. * param.s1 * param.s2.powi(3)
                                            + 8775. * param.s2.powi(4)))
                                + param.s12.powi(2)
                                    * (26. * param.s12.powi(8)
                                        + -2.
                                            * param.s12.powi(7)
                                            * (94. * param.s1 + 175. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(6)
                                            * (2. * param.s1.powi(2)
                                                + -10. * param.s1 * param.s2
                                                + 23. * param.s2.powi(2))
                                        + param.s12.powi(6)
                                            * (588. * param.s1.powi(2)
                                                + 1514. * param.s1 * param.s2
                                                + 2655. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(5)
                                            * (518. * param.s1.powi(3)
                                                + 1127. * param.s1.powi(2) * param.s2
                                                + 1997. * param.s1 * param.s2.powi(2)
                                                + -498. * param.s2.powi(3))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (34. * param.s1.powi(3)
                                                + -147. * param.s1.powi(2) * param.s2
                                                + 273. * param.s1 * param.s2.powi(2)
                                                + 740. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (1120. * param.s1.powi(4)
                                                + 810. * param.s1.powi(3) * param.s2
                                                + -2835. * param.s1.powi(2) * param.s2.powi(2)
                                                + 27176. * param.s1 * param.s2.powi(3)
                                                + -14535. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (308. * param.s1.powi(4)
                                                + -874. * param.s1.powi(3) * param.s2
                                                + -1671. * param.s1.powi(2) * param.s2.powi(2)
                                                + -18068. * param.s1 * param.s2.powi(3)
                                                + -3095. * param.s2.powi(4))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (378. * param.s1.powi(5)
                                                + -635. * param.s1.powi(4) * param.s2
                                                + -2790. * param.s1.powi(3) * param.s2.powi(2)
                                                + 7164. * param.s1.powi(2) * param.s2.powi(3)
                                                + 8780. * param.s1 * param.s2.powi(4)
                                                + -7857. * param.s2.powi(5)))
                                + 4. * param.m2_2
                                    * param.s12
                                    * (29. * param.s12.powi(8)
                                        + 3. * (param.s1 + -5. * param.s2)
                                            * (param.s1 - param.s2).powi(7)
                                        + -4.
                                            * param.s12.powi(7)
                                            * (47. * param.s1 + 140. * param.s2)
                                        + 2. * param.s12.powi(6)
                                            * (261. * param.s1.powi(2)
                                                + 982. * param.s1 * param.s2
                                                + -996. * param.s2.powi(2))
                                        + -8.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (4. * param.s1.powi(2)
                                                + -17. * param.s1 * param.s2
                                                + -26. * param.s2.powi(2))
                                        + -8.
                                            * param.s12.powi(5)
                                            * (101. * param.s1.powi(3)
                                                + 263. * param.s1.powi(2) * param.s2
                                                + 605. * param.s1 * param.s2.powi(2)
                                                + -990. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (79. * param.s1.powi(3)
                                                + -313. * param.s1.powi(2) * param.s2
                                                + -1156. * param.s1 * param.s2.powi(2)
                                                + -830. * param.s2.powi(3))
                                        + 4. * param.s12.powi(4)
                                            * (190. * param.s1.powi(4)
                                                + -15. * param.s1.powi(3) * param.s2
                                                + 3654. * param.s1.powi(2) * param.s2.powi(2)
                                                + -2341. * param.s1 * param.s2.powi(3)
                                                + -1620. * param.s2.powi(4))
                                        + -4.
                                            * param.s12.powi(3)
                                            * (111. * param.s1.powi(5)
                                                + -400. * param.s1.powi(4) * param.s2
                                                + 1800. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1968. * param.s1.powi(2) * param.s2.powi(3)
                                                + -3575. * param.s1 * param.s2.powi(4)
                                                + 96. * param.s2.powi(5))))
                        + param.m1_2
                            * (param.m2_2.powi(4)
                                * (170. * param.s12.powi(8)
                                    + 2. * param.s12.powi(7)
                                        * (1376. * param.s1 + -505. * param.s2)
                                    + 15. * (param.s1 - param.s2).powi(8)
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (89. * param.s1 + 76. * param.s2)
                                    + param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (1033. * param.s1.powi(2)
                                            + 1422. * param.s1 * param.s2
                                            + 695. * param.s2.powi(2))
                                    + param.s12.powi(6)
                                        * (-7137. * param.s1.powi(2)
                                            + -5686. * param.s1 * param.s2
                                            + 2577. * param.s2.powi(2))
                                    + param.s12.powi(5)
                                        * (2610. * param.s1.powi(3)
                                            + 24080. * param.s1.powi(2) * param.s2
                                            + 422. * param.s1 * param.s2.powi(2)
                                            + -3700. * param.s2.powi(3))
                                    + -10.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (432. * param.s1.powi(3)
                                            + 781. * param.s1.powi(2) * param.s2
                                            + 548. * param.s1 * param.s2.powi(2)
                                            + 189. * param.s2.powi(3))
                                    + param.s12.powi(4)
                                        * (5055. * param.s1.powi(4)
                                            + -16300. * param.s1.powi(3) * param.s2
                                            + -16926. * param.s1.powi(2) * param.s2.powi(2)
                                            + 4956. * param.s1 * param.s2.powi(3)
                                            + 3295. * param.s2.powi(4)))
                                + -4.
                                    * param.m2_2.powi(3)
                                    * param.s12
                                    * (46. * param.s12.powi(8)
                                        + 2. * param.s12.powi(7)
                                            * (316. * param.s1 + -95. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(7)
                                            * (3. * param.s1 + param.s2)
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (29. * param.s1.powi(2)
                                                + 41. * param.s1 * param.s2
                                                + 8. * param.s2.powi(2))
                                        + param.s12.powi(6)
                                            * (-2559. * param.s1.powi(2)
                                                + 1654. * param.s1 * param.s2
                                                + 243. * param.s2.powi(2))
                                        + 4. * param.s12.powi(5)
                                            * (729. * param.s1.powi(3)
                                                + 1465. * param.s1.powi(2) * param.s2
                                                + -1985. * param.s1 * param.s2.powi(2)
                                                + 7. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (803. * param.s1.powi(3)
                                                + 2119. * param.s1.powi(2) * param.s2
                                                + 1363. * param.s1 * param.s2.powi(2)
                                                + 155. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (-315. * param.s1.powi(4)
                                                + -15560. * param.s1.powi(3) * param.s2
                                                + 8166. * param.s1.powi(2) * param.s2.powi(2)
                                                + 7536. * param.s1 * param.s2.powi(3)
                                                + -355. * param.s2.powi(4))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (708. * param.s1.powi(5)
                                                + -4085. * param.s1.powi(4) * param.s2
                                                + -3240. * param.s1.powi(3) * param.s2.powi(2)
                                                + 6294. * param.s1.powi(2) * param.s2.powi(3)
                                                + 500. * param.s1 * param.s2.powi(4)
                                                + -177. * param.s2.powi(5)))
                                + 3. * param.m2_2.powi(2)
                                    * param.s12.powi(2)
                                    * (3. * param.s12.powi(8)
                                        + 2. * param.s12.powi(7)
                                            * (57. * param.s1 + 50. * param.s2)
                                        + param.s12.powi(6)
                                            * (-561. * param.s1.powi(2)
                                                + 3068. * param.s1 * param.s2
                                                + -514. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(6)
                                            * (6. * param.s1.powi(2)
                                                + 34. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (42. * param.s1.powi(3)
                                                + 437. * param.s1.powi(2) * param.s2
                                                + 393. * param.s1 * param.s2.powi(2)
                                                + 28. * param.s2.powi(3))
                                        + param.s12.powi(5)
                                            * (984. * param.s1.powi(3)
                                                + -3738. * param.s1.powi(2) * param.s2
                                                + -4070. * param.s1 * param.s2.powi(2)
                                                + 960. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (201. * param.s1.powi(4)
                                                + -4038. * param.s1.powi(3) * param.s2
                                                + -15677.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(2)
                                                + -3976. * param.s1 * param.s2.powi(3)
                                                + 90. * param.s2.powi(4))
                                        + 2. * param.s12.powi(3)
                                            * (21. * param.s1.powi(5)
                                                + 6600. * param.s1.powi(4) * param.s2
                                                + -9040. * param.s1.powi(3) * param.s2.powi(2)
                                                + -8922. * param.s1.powi(2) * param.s2.powi(3)
                                                + 6195. * param.s1 * param.s2.powi(4)
                                                + 106. * param.s2.powi(5))
                                        - param.s12.powi(4)
                                            * (705. * param.s1.powi(4)
                                                + 7650. * param.s1.powi(3) * param.s2
                                                + -27967.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(2)
                                                + 6788. * param.s1 * param.s2.powi(3)
                                                + 800. * param.s2.powi(4)))
                                + param.s12.powi(4)
                                    * (param.s12.powi(8)
                                        + 2. * param.s12.powi(7) * (param.s1 + -5. * param.s2)
                                        + param.s12.powi(6)
                                            * (-42. * param.s1.powi(2)
                                                + -86. * param.s1 * param.s2
                                                + 45. * param.s2.powi(2))
                                        + 2. * param.s12.powi(5)
                                            * (77. * param.s1.powi(3)
                                                + 233. * param.s1.powi(2) * param.s2
                                                + 458. * param.s1 * param.s2.powi(2)
                                                + -22. * param.s2.powi(3))
                                        + -3.
                                            * (param.s1 - param.s2).powi(4)
                                            * (3. * param.s1.powi(4)
                                                + -26. * param.s1.powi(3) * param.s2
                                                + 135. * param.s1.powi(2) * param.s2.powi(2)
                                                + 316. * param.s1 * param.s2.powi(3)
                                                + 22. * param.s2.powi(4))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (31. * param.s1.powi(5)
                                                + -155. * param.s1.powi(4) * param.s2
                                                + -101. * param.s1.powi(3) * param.s2.powi(2)
                                                + -4397. * param.s1.powi(2) * param.s2.powi(3)
                                                + -488. * param.s1 * param.s2.powi(4)
                                                + 160. * param.s2.powi(5))
                                        + 2. * param.s12.powi(3)
                                            * (147. * param.s1.powi(5)
                                                + 85. * param.s1.powi(4) * param.s2
                                                + -810. * param.s1.powi(3) * param.s2.powi(2)
                                                + 6036. * param.s1.powi(2) * param.s2.powi(3)
                                                + -4115. * param.s1 * param.s2.powi(4)
                                                + 267. * param.s2.powi(5))
                                        + param.s12.powi(2)
                                            * (-182. * param.s1.powi(6)
                                                + 470. * param.s1.powi(5) * param.s2
                                                + 2135. * param.s1.powi(4) * param.s2.powi(2)
                                                + -5920. * param.s1.powi(3) * param.s2.powi(3)
                                                + -7780. * param.s1.powi(2) * param.s2.powi(4)
                                                + 8282. * param.s1 * param.s2.powi(5)
                                                + -605. * param.s2.powi(6))
                                        - param.s12.powi(4)
                                            * (280. * param.s1.powi(4)
                                                + 690. * param.s1.powi(3) * param.s2
                                                + 1185. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1416. * param.s1 * param.s2.powi(3)
                                                + 175. * param.s2.powi(4)))
                                + 4. * param.m2_2
                                    * param.s12.powi(3)
                                    * (param.s12.powi(8)
                                        + 2. * param.s12.powi(7)
                                            * (4. * param.s1 + -5. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(5)
                                            * (param.s1.powi(3)
                                                + -9. * param.s1.powi(2) * param.s2
                                                + -30. * param.s1 * param.s2.powi(2)
                                                + -2. * param.s2.powi(3))
                                        + 2. * param.s12.powi(5)
                                            * (86. * param.s1.powi(3)
                                                + 371. * param.s1.powi(2) * param.s2
                                                + -1036. * param.s1 * param.s2.powi(2)
                                                + 122. * param.s2.powi(3))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1.powi(4)
                                                + 109. * param.s1.powi(3) * param.s2
                                                + 1557. * param.s1.powi(2) * param.s2.powi(2)
                                                + 896. * param.s1 * param.s2.powi(3)
                                                + 16. * param.s2.powi(4))
                                        + 2. * param.s12.powi(3)
                                            * (72. * param.s1.powi(5)
                                                + -455. * param.s1.powi(4) * param.s2
                                                + 5520. * param.s1.powi(3) * param.s2.powi(2)
                                                + -4644. * param.s1.powi(2) * param.s2.powi(3)
                                                + -1280. * param.s1 * param.s2.powi(4)
                                                + 267. * param.s2.powi(5))
                                        - param.s12.powi(2)
                                            * (38. * param.s1.powi(6)
                                                + -950. * param.s1.powi(5) * param.s2
                                                + 3025. * param.s1.powi(4) * param.s2.powi(2)
                                                + 9640. * param.s1.powi(3) * param.s2.powi(3)
                                                + -14660.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(4)
                                                + 2662. * param.s1 * param.s2.powi(5)
                                                + 245. * param.s2.powi(6))
                                        - param.s12.powi(4)
                                            * (220. * param.s1.powi(4)
                                                + 270. * param.s1.powi(3) * param.s2
                                                + 3519. * param.s1.powi(2) * param.s2.powi(2)
                                                + -5784. * param.s1 * param.s2.powi(3)
                                                + 535. * param.s2.powi(4))
                                        - param.s12.powi(6)
                                            * (66. * param.s1.powi(2)
                                                + 254. * param.s1 * param.s2
                                                + 27. * param.s2.powi(2))))
                        + param.m1_2.powi(2)
                            * (param.m2_2.powi(3)
                                * (1295. * param.s12.powi(8)
                                    + -30. * (param.s1 - param.s2).powi(8)
                                    + 4. * param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (83. * param.s1 + 82. * param.s2)
                                    + -2.
                                        * param.s12.powi(7)
                                        * (1759. * param.s1 + 1855. * param.s2)
                                    + param.s12.powi(6)
                                        * (423. * param.s1.powi(2)
                                            + 24914. * param.s1 * param.s2
                                            + 1227. * param.s2.powi(2))
                                    + 10.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (585. * param.s1.powi(3)
                                            + 1403. * param.s1.powi(2) * param.s2
                                            + 1363. * param.s1 * param.s2.powi(2)
                                            + 549. * param.s2.powi(3))
                                    + 4. * param.s12.powi(5)
                                        * (1950. * param.s1.powi(3)
                                            + -8095. * param.s1.powi(2) * param.s2
                                            + -8242. * param.s1 * param.s2.powi(2)
                                            + 1625. * param.s2.powi(3))
                                    + param.s12.powi(4)
                                        * (-10425. * param.s1.powi(4)
                                            + 6260. * param.s1.powi(3) * param.s2
                                            + 46914. * param.s1.powi(2) * param.s2.powi(2)
                                            + 6516. * param.s1 * param.s2.powi(3)
                                            + -9425. * param.s2.powi(4))
                                    - param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (1727. * param.s1.powi(2)
                                            + 2898. * param.s1 * param.s2
                                            + 1675. * param.s2.powi(2)))
                                + -3.
                                    * param.m2_2.powi(2)
                                    * param.s12
                                    * (103. * param.s12.powi(8)
                                        + -6.
                                            * (param.s1 - param.s2).powi(7)
                                            * (param.s1 + 3. * param.s2)
                                        + param.s12.powi(7)
                                            * (-406. * param.s1 + 1170. * param.s2)
                                        + param.s12.powi(6)
                                            * (447. * param.s1.powi(2)
                                                + 3738. * param.s1 * param.s2
                                                + -4981. * param.s2.powi(2))
                                        + 4. * param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (17. * param.s1.powi(2)
                                                + 83. * param.s1 * param.s2
                                                + 56. * param.s2.powi(2))
                                        + 4. * param.s12.powi(5)
                                            * (64. * param.s1.powi(3)
                                                + -4121. * param.s1.powi(2) * param.s2
                                                + 2696. * param.s1 * param.s2.powi(2)
                                                + 1503. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (-985. * param.s1.powi(4)
                                                + 15260. * param.s1.powi(3) * param.s2
                                                + 17098. * param.s1.powi(2) * param.s2.powi(2)
                                                + -30148. * param.s1 * param.s2.powi(3)
                                                + -1225. * param.s2.powi(4))
                                        + 2. * param.s12.powi(3)
                                            * (441. * param.s1.powi(5)
                                                + -915. * param.s1.powi(4) * param.s2
                                                + -12890.
                                                    * param.s1.powi(3)
                                                    * param.s2.powi(2)
                                                + 6738. * param.s1.powi(2) * param.s2.powi(3)
                                                + 7785. * param.s1 * param.s2.powi(4)
                                                + -1159. * param.s2.powi(5))
                                        - param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (359. * param.s1.powi(3)
                                                + 2947. * param.s1.powi(2) * param.s2
                                                + 4129. * param.s1 * param.s2.powi(2)
                                                + 1445. * param.s2.powi(3)))
                                + -3.
                                    * param.m2_2
                                    * param.s12.powi(2)
                                    * (22. * param.s12.powi(8)
                                        + -2.
                                            * param.s12.powi(7)
                                            * (58. * param.s1 + 175. * param.s2)
                                        + param.s12.powi(6)
                                            * (232. * param.s1.powi(2)
                                                + 498. * param.s1 * param.s2
                                                + -509. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(5)
                                            * (94. * param.s1.powi(3)
                                                + -645. * param.s1.powi(2) * param.s2
                                                + 5565. * param.s1 * param.s2.powi(2)
                                                + -2358. * param.s2.powi(3))
                                        + 2. * param.s12
                                            * (param.s1 - param.s2).powi(4)
                                            * (14. * param.s1.powi(3)
                                                + -113. * param.s1.powi(2) * param.s2
                                                + -553. * param.s1 * param.s2.powi(2)
                                                + -248. * param.s2.powi(3))
                                        + param.s12.powi(4)
                                            * (-20. * param.s1.powi(4)
                                                + -3110. * param.s1.powi(3) * param.s2
                                                + 12517. * param.s1.powi(2) * param.s2.powi(2)
                                                + 10152. * param.s1 * param.s2.powi(3)
                                                + -7515. * param.s2.powi(4))
                                        + 2. * param.s12.powi(3)
                                            * (74. * param.s1.powi(5)
                                                + 915. * param.s1.powi(4) * param.s2
                                                + 5070. * param.s1.powi(3) * param.s2.powi(2)
                                                + -19268.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(3)
                                                + 6120. * param.s1 * param.s2.powi(4)
                                                + 2049. * param.s2.powi(5))
                                        - param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(2)
                                            * (104. * param.s1.powi(4)
                                                + 58. * param.s1.powi(3) * param.s2
                                                + 10887. * param.s1.powi(2) * param.s2.powi(2)
                                                + 12356. * param.s1 * param.s2.powi(3)
                                                + -5. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(6)
                                            * (2. * param.s1.powi(2)
                                                + -18. * param.s1 * param.s2
                                                + -29. * param.s2.powi(2)))
                                + param.s12.powi(3)
                                    * (-14. * param.s12.powi(8)
                                        + 2. * param.s12.powi(7)
                                            * (46. * param.s1 + 85. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(5)
                                            * (2. * param.s1.powi(3)
                                                + -12. * param.s1.powi(2) * param.s2
                                                + 33. * param.s1 * param.s2.powi(2)
                                                + 57. * param.s2.powi(3))
                                        + 2. * param.s12.powi(5)
                                            * (182. * param.s1.powi(3)
                                                + 173. * param.s1.powi(2) * param.s2
                                                + -97. * param.s1 * param.s2.powi(2)
                                                + 578. * param.s2.powi(3))
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (14. * param.s1.powi(4)
                                                + 19. * param.s1.powi(3) * param.s2
                                                + -780. * param.s1.powi(2) * param.s2.powi(2)
                                                + -3913. * param.s1 * param.s2.powi(3)
                                                + -500. * param.s2.powi(4))
                                        + param.s12.powi(4)
                                            * (-280. * param.s1.powi(4)
                                                + 810. * param.s1.powi(3) * param.s2
                                                + 4365. * param.s1.powi(2) * param.s2.powi(2)
                                                + -15384. * param.s1 * param.s2.powi(3)
                                                + 2945. * param.s2.powi(4))
                                        + 2. * param.s12.powi(3)
                                            * (42. * param.s1.powi(5)
                                                + -665. * param.s1.powi(4) * param.s2
                                                + -810. * param.s1.powi(3) * param.s2.powi(2)
                                                + -6684. * param.s1.powi(2) * param.s2.powi(3)
                                                + 14920. * param.s1 * param.s2.powi(4)
                                                + -3603. * param.s2.powi(5))
                                        + param.s12.powi(2)
                                            * (28. * param.s1.powi(6)
                                                + 590. * param.s1.powi(5) * param.s2
                                                + -3415. * param.s1.powi(4) * param.s2.powi(2)
                                                + 25280. * param.s1.powi(3) * param.s2.powi(3)
                                                + -17950.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(4)
                                                + -9718. * param.s1 * param.s2.powi(5)
                                                + 5185. * param.s2.powi(6))
                                        - param.s12.powi(6)
                                            * (252. * param.s1.powi(2)
                                                + 566. * param.s1 * param.s2
                                                + 1065. * param.s2.powi(2))))
                        - param.m1_2.powi(4)
                            * (param.m2_2
                                * (55. * param.s12.powi(8)
                                    + 15. * (param.s1 - param.s2).powi(8)
                                    + -2.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(6)
                                        * (71. * param.s1 + 94. * param.s2)
                                    + -2.
                                        * param.s12.powi(7)
                                        * (191. * param.s1 + 620. * param.s2)
                                    + 2. * param.s12.powi(6)
                                        * (576. * param.s1.powi(2)
                                            + 2708. * param.s1 * param.s2
                                            + -3711. * param.s2.powi(2))
                                    + 2. * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(4)
                                        * (296. * param.s1.powi(2)
                                            + 684. * param.s1 * param.s2
                                            + 595. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(5)
                                        * (985. * param.s1.powi(3)
                                            + 4540. * param.s1.powi(2) * param.s2
                                            + -5669. * param.s1 * param.s2.powi(2)
                                            + -7170. * param.s2.powi(3))
                                    + -10.
                                        * param.s12.powi(3)
                                        * (param.s1 - param.s2).powi(2)
                                        * (141. * param.s1.powi(3)
                                            + 430. * param.s1.powi(2) * param.s2
                                            + 791. * param.s1 * param.s2.powi(2)
                                            + 588. * param.s2.powi(3))
                                    + 2. * param.s12.powi(4)
                                        * (1045. * param.s1.powi(4)
                                            + 3420. * param.s1.powi(3) * param.s2
                                            + -942. * param.s1.powi(2) * param.s2.powi(2)
                                            + -13048. * param.s1 * param.s2.powi(3)
                                            + -435. * param.s2.powi(4)))
                                + param.s12
                                    * (19. * param.s12.powi(8)
                                        + 3. * (3. * param.s1 + -7. * param.s2)
                                            * (param.s1 - param.s2).powi(7)
                                        + -2.
                                            * param.s12.powi(7)
                                            * (71. * param.s1 + 140. * param.s2)
                                        + -2.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(5)
                                            * (41. * param.s1.powi(2)
                                                + -37. * param.s1 * param.s2
                                                + -160. * param.s2.powi(2))
                                        + 2. * param.s12.powi(6)
                                            * (231. * param.s1.powi(2)
                                                + 668. * param.s1 * param.s2
                                                + 1260. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(5)
                                            * (427. * param.s1.powi(3)
                                                + 1198. * param.s1.powi(2) * param.s2
                                                + 2953. * param.s1 * param.s2.powi(2)
                                                + -2892. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1 - param.s2).powi(3)
                                            * (161. * param.s1.powi(3)
                                                + 43. * param.s1.powi(2) * param.s2
                                                + -884. * param.s1 * param.s2.powi(2)
                                                + -1540. * param.s2.powi(3))
                                        + 4. * param.s12.powi(4)
                                            * (245. * param.s1.powi(4)
                                                + 435. * param.s1.powi(3) * param.s2
                                                + 615. * param.s1.powi(2) * param.s2.powi(2)
                                                + 2341. * param.s1 * param.s2.powi(3)
                                                + -3900. * param.s2.powi(4))
                                        + param.s12.powi(3)
                                            * (-714. * param.s1.powi(5)
                                                + 80. * param.s1.powi(4) * param.s2
                                                + 2520. * param.s1.powi(3) * param.s2.powi(2)
                                                + -16512.
                                                    * param.s1.powi(2)
                                                    * param.s2.powi(3)
                                                + 9850. * param.s1 * param.s2.powi(4)
                                                + 4776. * param.s2.powi(5))))
                        - param.m2_2.powi(3)
                            * param.s12.powi(2)
                            * (19. * param.s12.powi(8)
                                + -10. * param.s12.powi(7) * (31. * param.s1 + 10. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(6)
                                    * (10. * param.s1.powi(2) + 6. * param.s1 * param.s2
                                        - param.s2.powi(2))
                                + param.s12.powi(6)
                                    * (-729. * param.s1.powi(2)
                                        + 460. * param.s1 * param.s2
                                        + 198. * param.s2.powi(2))
                                + 2. * param.s12.powi(5)
                                    * (2580. * param.s1.powi(3)
                                        + -5449. * param.s1.powi(2) * param.s2
                                        + 557. * param.s1 * param.s2.powi(2)
                                        + -76. * param.s2.powi(3))
                                + -2.
                                    * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (298. * param.s1.powi(3)
                                        + 537. * param.s1.powi(2) * param.s2
                                        + 81. * param.s1 * param.s2.powi(2)
                                        + -16. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (-7785. * param.s1.powi(4)
                                        + 9550. * param.s1.powi(3) * param.s2
                                        + 12831. * param.s1.powi(2) * param.s2.powi(2)
                                        + -2820. * param.s1 * param.s2.powi(3)
                                        + -40. * param.s2.powi(4))
                                + 5. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (61. * param.s1.powi(4)
                                        + -2558. * param.s1.powi(3) * param.s2
                                        + -2121. * param.s1.powi(2) * param.s2.powi(2)
                                        + -40. * param.s1 * param.s2.powi(3)
                                        + -22. * param.s2.powi(4))
                                + 2. * param.s12.powi(3)
                                    * (1953. * param.s1.powi(5)
                                        + 6620. * param.s1.powi(4) * param.s2
                                        + -19320. * param.s1.powi(3) * param.s2.powi(2)
                                        + 4734. * param.s1.powi(2) * param.s2.powi(3)
                                        + 895. * param.s1 * param.s2.powi(4)
                                        + 78. * param.s2.powi(5)))))
                * param.lambda_m12_sqrt
                * param.lambda_s12_sqrt
                + 420.
                    * param.s12.powi(5)
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
                    * (-18.
                        * param.m1_2.powi(3)
                        * (param.s12 - param.s2 - param.s1)
                        * param.s2.powi(2)
                        + param.m0_2.powi(3)
                            * (5. * param.s12.powi(3)
                                + 3. * (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-7. * param.s1 + 5. * param.s2)
                                - param.s12
                                    * (param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + 7. * param.s2.powi(2)))
                        + 2. * param.m1_2.powi(2)
                            * param.s2
                            * (param.s2
                                * (-17. * param.s1.powi(2)
                                    + 7. * param.s1 * param.s12
                                    + 10. * param.s12.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + -20. * param.s12 * param.s2
                                    + 10. * param.s2.powi(2))
                                + -2.
                                    * param.m2_2
                                    * (5. * param.s1.powi(2)
                                        + 5. * param.s12.powi(2)
                                        + 17. * param.s1 * param.s2
                                        + 5. * param.s2.powi(2)
                                        + -10. * param.s12 * (param.s1 + param.s2)))
                        + param.s1
                            * (-4.
                                * param.m2_2.powi(3)
                                * (2. * param.s1.powi(2)
                                    + 2. * param.s12.powi(2)
                                    + 5. * param.s1 * param.s2
                                    + 2. * param.s2.powi(2)
                                    + -4. * param.s12 * (param.s1 + param.s2))
                                + param.m2_2.powi(2)
                                    * (-5. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + -25. * param.s1.powi(2) * param.s2
                                        + 11. * param.s1 * param.s2.powi(2)
                                        + 19. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-15. * param.s1 + 9. * param.s2)
                                        + param.s12
                                            * (15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + -33. * param.s2.powi(2)))
                                + -2.
                                    * param.m2_2
                                    * param.s2
                                    * (5. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1 + 7. * param.s2)
                                        + param.s12
                                            * (-5. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        - param.s12.powi(2) * (5. * param.s1 + 3. * param.s2))
                                + param.s2.powi(2)
                                    * (5. * param.s12.powi(3)
                                        + param.s12.powi(2) * (5. * param.s1 + -7. * param.s2)
                                        + -3. * (param.s1 - param.s2).powi(3)
                                        - param.s12
                                            * (7. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + param.s2.powi(2))))
                        + param.m0_2.powi(2)
                            * (-3. * param.s1.powi(4)
                                + 7. * param.s1.powi(3) * param.s12
                                + -3. * param.s1.powi(2) * param.s12.powi(2)
                                + -3. * param.s1 * param.s12.powi(3)
                                + 2. * param.s12.powi(4)
                                + 3. * param.s1.powi(3) * param.s2
                                + -32. * param.s1.powi(2) * param.s12 * param.s2
                                + 21. * param.s1 * param.s12.powi(2) * param.s2
                                + 8. * param.s12.powi(3) * param.s2
                                + 9. * param.s1.powi(2) * param.s2.powi(2)
                                + 25. * param.s1 * param.s12 * param.s2.powi(2)
                                + -16. * param.s12.powi(2) * param.s2.powi(2)
                                + -15. * param.s1 * param.s2.powi(3)
                                + 6. * param.s2.powi(4)
                                + -2.
                                    * param.m2_2
                                    * (5. * param.s12.powi(3)
                                        + (param.s1 - param.s2).powi(2)
                                            * (7. * param.s1 + 5. * param.s2)
                                        + param.s12
                                            * (-9. * param.s1.powi(2)
                                                + 20. * param.s1 * param.s2
                                                + -5. * param.s2.powi(2))
                                        - param.s12.powi(2) * (3. * param.s1 + 5. * param.s2))
                                + param.m1_2
                                    * (-5. * param.s12.powi(3)
                                        + 5. * param.s12.powi(2)
                                            * (3. * param.s1 + -5. * param.s2)
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1 + 19. * param.s2)
                                        + param.s12
                                            * (-15. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 11. * param.s2.powi(2))))
                        + param.m1_2
                            * (param.s2.powi(2)
                                * (-5. * param.s12.powi(3)
                                    + -5. * param.s12.powi(2) * (5. * param.s1 + -3. * param.s2)
                                    + (param.s1 - param.s2).powi(2)
                                        * (19. * param.s1 + 5. * param.s2)
                                    + param.s12
                                        * (11. * param.s1.powi(2)
                                            + 16. * param.s1 * param.s2
                                            + -15. * param.s2.powi(2)))
                                + 2. * param.m2_2
                                    * param.s2
                                    * (15. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 5. * param.s12.powi(2) * (param.s1 + -3. * param.s2)
                                        + 19. * param.s1.powi(2) * param.s2
                                        + -29. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-25. * param.s1.powi(2)
                                                + 24. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + param.m2_2.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -5. * param.s12.powi(3)
                                        + 49. * param.s1.powi(2) * param.s2
                                        + 49. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3)
                                        + 15. * param.s12.powi(2) * (param.s1 + param.s2)
                                        - param.s12
                                            * (15. * param.s1.powi(2)
                                                + 64. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2))))
                        + param.m0_2
                            * (2.
                                * param.m1_2.powi(2)
                                * param.s2
                                * (10. * param.s1.powi(2)
                                    + -20. * param.s1 * param.s12
                                    + 10. * param.s12.powi(2)
                                    + 7. * param.s1 * param.s2
                                    + 7. * param.s12 * param.s2
                                    + -17. * param.s2.powi(2))
                                + param.s2
                                    * (2. * param.s12.powi(4)
                                        + param.s12.powi(3) * (8. * param.s1 + -3. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(3)
                                            * (2. * param.s1 + param.s2)
                                        + param.s12.powi(2)
                                            * (-16. * param.s1.powi(2)
                                                + 21. * param.s1 * param.s2
                                                + -3. * param.s2.powi(2))
                                        + param.s12
                                            * param.s2
                                            * (25. * param.s1.powi(2)
                                                + -32. * param.s1 * param.s2
                                                + 7. * param.s2.powi(2)))
                                + param.m2_2.powi(2)
                                    * (19. * param.s1.powi(3)
                                        + 5. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2)
                                            * (3. * param.s1 + -5. * param.s2)
                                        + 11. * param.s1.powi(2) * param.s2
                                        + -25. * param.s1 * param.s2.powi(2)
                                        + -5. * param.s2.powi(3)
                                        + param.s12
                                            * (-33. * param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2)))
                                + -2.
                                    * param.m2_2
                                    * (param.s12.powi(4)
                                        + param.s12.powi(3) * (param.s1 + param.s2)
                                        + -4.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + -3.
                                            * param.s12.powi(2)
                                            * (3. * param.s1.powi(2)
                                                + -8. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + param.s12
                                            * (11. * param.s1.powi(3)
                                                + -17. * param.s1.powi(2) * param.s2
                                                + -17. * param.s1 * param.s2.powi(2)
                                                + 11. * param.s2.powi(3)))
                                + 2. * param.m1_2
                                    * (param.m2_2
                                        * (-5. * param.s1.powi(3)
                                            + 5. * param.s12.powi(3)
                                            + -29. * param.s1.powi(2) * param.s2
                                            + 19. * param.s1 * param.s2.powi(2)
                                            + 15. * param.s2.powi(3)
                                            + 5. * param.s12.powi(2)
                                                * (-3. * param.s1 + param.s2)
                                            + param.s12
                                                * (15. * param.s1.powi(2)
                                                    + 24. * param.s1 * param.s2
                                                    + -25. * param.s2.powi(2)))
                                        + -4.
                                            * param.s2
                                            * (2. * param.s12.powi(3)
                                                + 3. * (param.s1 - param.s2).powi(2)
                                                    * (param.s1 + param.s2)
                                                + param.s12
                                                    * (-4. * param.s1.powi(2)
                                                        + 11. * param.s1 * param.s2
                                                        + -4. * param.s2.powi(2))
                                                - param.s12.powi(2) * (param.s1 + param.s2)))))
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
