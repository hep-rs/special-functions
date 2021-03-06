use super::{log_diff, Parameters};

pub(crate) fn c220(param: &Parameters) -> f64 {
    (if param.s1 > (param.m0 + param.m1).powi(2) {
        0.0006944444444444444
            * std::f64::consts::PI
            * param.s1.powi(-4)
            * param.lambda_s12_sqrt.powi(-9)
            * ((param.m0_2.powi(5)
                * (3. * param.s12.powi(7)
                    + -3. * param.s12.powi(6) * (9. * param.s1 + 7. * param.s2)
                    + (param.s1 - param.s2).powi(5)
                        * (6. * param.s1.powi(2)
                            + -8. * param.s1 * param.s2
                            + 3. * param.s2.powi(2))
                    + param.s12.powi(5)
                        * (114. * param.s1.powi(2)
                            + 112. * param.s1 * param.s2
                            + 63. * param.s2.powi(2))
                    + param.s12.powi(3)
                        * (345. * param.s1.powi(4)
                            + -56. * param.s1.powi(3) * param.s2
                            + 10. * param.s1.powi(2) * param.s2.powi(2)
                            + 40. * param.s1 * param.s2.powi(3)
                            + 105. * param.s2.powi(4))
                    + param.s12.powi(2)
                        * (-33. * param.s1.powi(5)
                            + -191. * param.s1.powi(4) * param.s2
                            + 126. * param.s1.powi(3) * param.s2.powi(2)
                            + 66. * param.s1.powi(2) * param.s2.powi(3)
                            + 95. * param.s1 * param.s2.powi(4)
                            + -63. * param.s2.powi(5))
                    - param.s12.powi(4)
                        * (330. * param.s1.powi(3)
                            + 206. * param.s1.powi(2) * param.s2
                            + 155. * param.s1 * param.s2.powi(2)
                            + 105. * param.s2.powi(3))
                    - param.s12
                        * (param.s1 - param.s2).powi(3)
                        * (78. * param.s1.powi(3)
                            + -46. * param.s1.powi(2) * param.s2
                            + -25. * param.s1 * param.s2.powi(2)
                            + 21. * param.s2.powi(3)))
                + param.m1_2.powi(5)
                    * (3. * param.s1.powi(7)
                        + -3. * param.s12.powi(7)
                        + -29. * param.s1.powi(6) * param.s2
                        + 139. * param.s1.powi(5) * param.s2.powi(2)
                        + -533. * param.s1.powi(4) * param.s2.powi(3)
                        + -533. * param.s1.powi(3) * param.s2.powi(4)
                        + 139. * param.s1.powi(2) * param.s2.powi(5)
                        + -29. * param.s1 * param.s2.powi(6)
                        + 3. * param.s2.powi(7)
                        + 21. * param.s12.powi(6) * (param.s1 + param.s2)
                        + 5. * param.s12.powi(4)
                            * (21. * param.s1.powi(3)
                                + 13. * param.s1.powi(2) * param.s2
                                + 13. * param.s1 * param.s2.powi(2)
                                + 21. * param.s2.powi(3))
                        + param.s12.powi(3)
                            * (-105. * param.s1.powi(4)
                                + 80. * param.s1.powi(3) * param.s2
                                + 44. * param.s1.powi(2) * param.s2.powi(2)
                                + 80. * param.s1 * param.s2.powi(3)
                                + -105. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (63. * param.s1.powi(5)
                                + -185. * param.s1.powi(4) * param.s2
                                + 108. * param.s1.powi(3) * param.s2.powi(2)
                                + 108. * param.s1.powi(2) * param.s2.powi(3)
                                + -185. * param.s1 * param.s2.powi(4)
                                + 63. * param.s2.powi(5))
                        + param.s12
                            * (-21. * param.s1.powi(6)
                                + 124. * param.s1.powi(5) * param.s2
                                + -293. * param.s1.powi(4) * param.s2.powi(2)
                                + 240. * param.s1.powi(3) * param.s2.powi(3)
                                + -293. * param.s1.powi(2) * param.s2.powi(4)
                                + 124. * param.s1 * param.s2.powi(5)
                                + -21. * param.s2.powi(6))
                        - param.s12.powi(5)
                            * (63. * param.s1.powi(2)
                                + 76. * param.s1 * param.s2
                                + 63. * param.s2.powi(2)))
                + param.m1_2.powi(3)
                    * param.s1.powi(2)
                    * (-30. * param.s12.powi(7)
                        + 6. * param.s12.powi(6) * (35. * param.s1 + 39. * param.s2)
                        + param.s12.powi(4)
                            * (1050. * param.s1.powi(3)
                                + 1010. * param.s1.powi(2) * param.s2
                                + 989. * param.s1 * param.s2.powi(2)
                                + 1485. * param.s2.powi(3))
                        + -2.
                            * param.s12.powi(3)
                            * (525. * param.s1.powi(4)
                                + -160. * param.s1.powi(3) * param.s2
                                + -337. * param.s1.powi(2) * param.s2.powi(2)
                                + -652. * param.s1 * param.s2.powi(3)
                                + 840. * param.s2.powi(4))
                        + (param.s1 - param.s2).powi(2)
                            * (30. * param.s1.powi(5)
                                + -206. * param.s1.powi(4) * param.s2
                                + 651. * param.s1.powi(3) * param.s2.powi(2)
                                + -1455. * param.s1.powi(2) * param.s2.powi(3)
                                + -809. * param.s1 * param.s2.powi(4)
                                + 69. * param.s2.powi(5))
                        + 2. * param.s12.powi(2)
                            * (315. * param.s1.powi(5)
                                + -745. * param.s1.powi(4) * param.s2
                                + -33. * param.s1.powi(3) * param.s2.powi(2)
                                + 567. * param.s1.powi(2) * param.s2.powi(3)
                                + -1978. * param.s1 * param.s2.powi(4)
                                + 570. * param.s2.powi(5))
                        + -15.
                            * param.m2_2.powi(2)
                            * (param.s12.powi(5)
                                + 23. * param.s1.powi(4) * param.s2
                                + 258. * param.s1.powi(3) * param.s2.powi(2)
                                + 258. * param.s1.powi(2) * param.s2.powi(3)
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
                                        + -322. * param.s1.powi(2) * param.s2.powi(2)
                                        + -64. * param.s1 * param.s2.powi(3)
                                        + 5. * param.s2.powi(4))
                                - param.s2.powi(5)
                                - param.s1.powi(5))
                        + -6.
                            * param.m2_2
                            * (4. * param.s1.powi(6)
                                + 4. * param.s12.powi(6)
                                + -47. * param.s1.powi(5) * param.s2
                                + 337. * param.s1.powi(4) * param.s2.powi(2)
                                + 702. * param.s1.powi(3) * param.s2.powi(3)
                                + -838. * param.s1.powi(2) * param.s2.powi(4)
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
                                        + 890. * param.s1.powi(2) * param.s2.powi(3)
                                        + 504. * param.s1 * param.s2.powi(4)
                                        + -49. * param.s2.powi(5))
                                - param.s12.powi(5) * (24. * param.s1 + 29. * param.s2))
                        - param.s12
                            * (210. * param.s1.powi(6)
                                + -1096. * param.s1.powi(5) * param.s2
                                + 1901. * param.s1.powi(4) * param.s2.powi(2)
                                + 960. * param.s1.powi(3) * param.s2.powi(3)
                                + 2420. * param.s1.powi(2) * param.s2.powi(4)
                                + -3304. * param.s1 * param.s2.powi(5)
                                + 429. * param.s2.powi(6))
                        - param.s12.powi(5)
                            * (630. * param.s1.powi(2)
                                + 904. * param.s1 * param.s2
                                + 789. * param.s2.powi(2)))
                + param.m1_2
                    * param.s1.powi(4)
                    * (-15. * param.s12.powi(7)
                        + 3. * param.s12.powi(6) * (35. * param.s1 + 43. * param.s2)
                        + (param.s1 - param.s2).powi(4)
                            * (15. * param.s1.powi(3)
                                + -61. * param.s1.powi(2) * param.s2
                                + 94. * param.s1 * param.s2.powi(2)
                                + -66. * param.s2.powi(3))
                        + param.s12.powi(4)
                            * (525. * param.s1.powi(3)
                                + 685. * param.s1.powi(2) * param.s2
                                + 814. * param.s1 * param.s2.powi(2)
                                + 1230. * param.s2.powi(3))
                        + param.s12.powi(3)
                            * (-525. * param.s1.powi(4)
                                + -80. * param.s1.powi(3) * param.s2
                                + 154. * param.s1.powi(2) * param.s2.powi(2)
                                + 664. * param.s1 * param.s2.powi(3)
                                + -1665. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (315. * param.s1.powi(5)
                                + -565. * param.s1.powi(4) * param.s2
                                + -306. * param.s1.powi(3) * param.s2.powi(2)
                                + -306. * param.s1.powi(2) * param.s2.powi(3)
                                + -821. * param.s1 * param.s2.powi(4)
                                + 1035. * param.s2.powi(5))
                        + 30.
                            * param.m2_2.powi(4)
                            * (-9. * param.s1.powi(3)
                                + 9. * param.s12.powi(3)
                                + -61. * param.s1.powi(2) * param.s2
                                + -61. * param.s1 * param.s2.powi(2)
                                + -9. * param.s2.powi(3)
                                + -27. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (27. * param.s1.powi(2)
                                        + 88. * param.s1 * param.s2
                                        + 27. * param.s2.powi(2)))
                        + -40.
                            * param.m2_2.powi(3)
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + -3. * param.s12.powi(3) * (4. * param.s1 + -5. * param.s2)
                                + 65. * param.s1.powi(3) * param.s2
                                + 47. * param.s1.powi(2) * param.s2.powi(2)
                                + -91. * param.s1 * param.s2.powi(3)
                                + -24. * param.s2.powi(4)
                                + param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + 35. * param.s1 * param.s2
                                        + -63. * param.s2.powi(2))
                                + param.s12
                                    * (-12. * param.s1.powi(3)
                                        + -115. * param.s1.powi(2) * param.s2
                                        + 68. * param.s1 * param.s2.powi(2)
                                        + 69. * param.s2.powi(3)))
                        + -15.
                            * param.m2_2.powi(2)
                            * (3. * param.s12.powi(5)
                                + -3. * param.s12.powi(4) * (5. * param.s1 + 13. * param.s2)
                                + 6. * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 12. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (15. * param.s1.powi(3)
                                        + -9. * param.s1.powi(2) * param.s2
                                        + 221. * param.s1 * param.s2.powi(2)
                                        + -75. * param.s2.powi(3))
                                + param.s12
                                    * (15. * param.s1.powi(4)
                                        + -96. * param.s1.powi(3) * param.s2
                                        + 278. * param.s1.powi(2) * param.s2.powi(2)
                                        + 320. * param.s1 * param.s2.powi(3)
                                        + -213. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(3)
                                        + -39. * param.s1.powi(2) * param.s2
                                        + -227. * param.s1 * param.s2.powi(2)
                                        + -81. * param.s2.powi(3)))
                        + -6.
                            * param.m2_2
                            * (4. * param.s12.powi(6)
                                + -3. * param.s12.powi(5) * (8. * param.s1 + 13. * param.s2)
                                + param.s12.powi(4)
                                    * (60. * param.s1.powi(2)
                                        + 119. * param.s1 * param.s2
                                        + 195. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (4. * param.s1.powi(3)
                                        + -25. * param.s1.powi(2) * param.s2
                                        + 80. * param.s1 * param.s2.powi(2)
                                        + 101. * param.s2.powi(3))
                                + -2.
                                    * param.s12.powi(3)
                                    * (40. * param.s1.powi(3)
                                        + 43. * param.s1.powi(2) * param.s2
                                        + 28. * param.s1 * param.s2.powi(2)
                                        + 145. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (60. * param.s1.powi(4)
                                        + -66. * param.s1.powi(3) * param.s2
                                        + -306. * param.s1.powi(2) * param.s2.powi(2)
                                        + 754. * param.s1 * param.s2.powi(3)
                                        + 30. * param.s2.powi(4))
                                + param.s12
                                    * (-24. * param.s1.powi(5)
                                        + 109. * param.s1.powi(4) * param.s2
                                        + 730. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1016. * param.s1 * param.s2.powi(4)
                                        + 201. * param.s2.powi(5)))
                        - param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (105. * param.s1.powi(4)
                                + -266. * param.s1.powi(3) * param.s2
                                + -51. * param.s1.powi(2) * param.s2.powi(2)
                                + 884. * param.s1 * param.s2.powi(3)
                                + 144. * param.s2.powi(4))
                        - param.s12.powi(5)
                            * (315. * param.s1.powi(2)
                                + 524. * param.s1 * param.s2
                                + 504. * param.s2.powi(2)))
                + param.m1_2.powi(2)
                    * param.s1.powi(3)
                    * (30. * param.s12.powi(7)
                        + -6. * param.s12.powi(6) * (35. * param.s1 + 41. * param.s2)
                        + param.s12.powi(5)
                            * (630. * param.s1.powi(2)
                                + 976. * param.s1 * param.s2
                                + 891. * param.s2.powi(2))
                        + 2. * param.s12.powi(3)
                            * (525. * param.s1.powi(4)
                                + -40. * param.s1.powi(3) * param.s2
                                + -283. * param.s1.powi(2) * param.s2.powi(2)
                                + -748. * param.s1 * param.s2.powi(3)
                                + 1230. * param.s2.powi(4))
                        + -2.
                            * param.s12.powi(2)
                            * (315. * param.s1.powi(5)
                                + -655. * param.s1.powi(4) * param.s2
                                + -207. * param.s1.powi(3) * param.s2.powi(2)
                                + 153. * param.s1.powi(2) * param.s2.powi(3)
                                + -1952. * param.s1 * param.s2.powi(4)
                                + 990. * param.s2.powi(5))
                        + param.s12
                            * (210. * param.s1.powi(6)
                                + -1024. * param.s1.powi(5) * param.s2
                                + 1499. * param.s1.powi(4) * param.s2.powi(2)
                                + 1440. * param.s1.powi(3) * param.s2.powi(3)
                                + -1160. * param.s1.powi(2) * param.s2.powi(4)
                                + -1856. * param.s1 * param.s2.powi(5)
                                + 891. * param.s2.powi(6))
                        + 20.
                            * param.m2_2.powi(3)
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 92. * param.s1.powi(3) * param.s2
                                + 230. * param.s1.powi(2) * param.s2.powi(2)
                                + 92. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + 58. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (3. * param.s1.powi(3)
                                        + 49. * param.s1.powi(2) * param.s2
                                        + 49. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)))
                        + 15.
                            * param.m2_2.powi(2)
                            * (-3. * param.s1.powi(5)
                                + 3. * param.s12.powi(5)
                                + 57. * param.s1.powi(4) * param.s2
                                + 406. * param.s1.powi(3) * param.s2.powi(2)
                                + -146. * param.s1.powi(2) * param.s2.powi(3)
                                + -299. * param.s1 * param.s2.powi(4)
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
                                        + 151. * param.s1 * param.s2.powi(2)
                                        + 51. * param.s2.powi(3))
                                + param.s12
                                    * (15. * param.s1.powi(4)
                                        + -144. * param.s1.powi(3) * param.s2
                                        + -182. * param.s1.powi(2) * param.s2.powi(2)
                                        + 592. * param.s1 * param.s2.powi(3)
                                        + 63. * param.s2.powi(4)))
                        + 6. * param.m2_2
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
                                        + 599. * param.s1 * param.s2.powi(3)
                                        + 51. * param.s2.powi(4))
                                + 2. * param.s12.powi(2)
                                    * (45. * param.s1.powi(4)
                                        + -87. * param.s1.powi(3) * param.s2
                                        + -207. * param.s1.powi(2) * param.s2.powi(2)
                                        + 13. * param.s1 * param.s2.powi(3)
                                        + 210. * param.s2.powi(4))
                                - param.s12
                                    * (36. * param.s1.powi(5)
                                        + -201. * param.s1.powi(4) * param.s2
                                        + 240. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1790. * param.s1.powi(2) * param.s2.powi(3)
                                        + 724. * param.s1 * param.s2.powi(4)
                                        + 231. * param.s2.powi(5)))
                        - (param.s1 - param.s2).powi(3)
                            * (30. * param.s1.powi(4)
                                + -164. * param.s1.powi(3) * param.s2
                                + 385. * param.s1.powi(2) * param.s2.powi(2)
                                + -560. * param.s1 * param.s2.powi(3)
                                + -171. * param.s2.powi(4))
                        - param.s12.powi(4)
                            * (1050. * param.s1.powi(3)
                                + 1190. * param.s1.powi(2) * param.s2
                                + 1271. * param.s1 * param.s2.powi(2)
                                + 1875. * param.s2.powi(3)))
                + param.m0_2.powi(3)
                    * (param.m1_2.powi(2)
                        * (30. * param.s12.powi(7)
                            + -6. * param.s12.powi(6) * (41. * param.s1 + 35. * param.s2)
                            + param.s12.powi(5)
                                * (891. * param.s1.powi(2)
                                    + 976. * param.s1 * param.s2
                                    + 630. * param.s2.powi(2))
                            + 2. * param.s12.powi(3)
                                * (1230. * param.s1.powi(4)
                                    + -748. * param.s1.powi(3) * param.s2
                                    + -283. * param.s1.powi(2) * param.s2.powi(2)
                                    + -40. * param.s1 * param.s2.powi(3)
                                    + 525. * param.s2.powi(4))
                            + -2.
                                * param.s12.powi(2)
                                * (990. * param.s1.powi(5)
                                    + -1952. * param.s1.powi(4) * param.s2
                                    + 153. * param.s1.powi(3) * param.s2.powi(2)
                                    + -207. * param.s1.powi(2) * param.s2.powi(3)
                                    + -655. * param.s1 * param.s2.powi(4)
                                    + 315. * param.s2.powi(5))
                            + param.s12
                                * (891. * param.s1.powi(6)
                                    + -1856. * param.s1.powi(5) * param.s2
                                    + -1160. * param.s1.powi(4) * param.s2.powi(2)
                                    + 1440. * param.s1.powi(3) * param.s2.powi(3)
                                    + 1499. * param.s1.powi(2) * param.s2.powi(4)
                                    + -1024. * param.s1 * param.s2.powi(5)
                                    + 210. * param.s2.powi(6))
                            - (param.s1 - param.s2).powi(3)
                                * (171. * param.s1.powi(4)
                                    + 560. * param.s1.powi(3) * param.s2
                                    + -385. * param.s1.powi(2) * param.s2.powi(2)
                                    + 164. * param.s1 * param.s2.powi(3)
                                    + -30. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (1875. * param.s1.powi(3)
                                    + 1271. * param.s1.powi(2) * param.s2
                                    + 1190. * param.s1 * param.s2.powi(2)
                                    + 1050. * param.s2.powi(3)))
                        + param.s1.powi(2)
                            * (6. * param.s12.powi(7)
                                + -6. * param.s12.powi(6) * (13. * param.s1 + 3. * param.s2)
                                + param.s12.powi(5)
                                    * (237. * param.s1.powi(2)
                                        + -56. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 10. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-285. * param.s1.powi(3)
                                        + -1181. * param.s1.powi(2) * param.s2
                                        + 559. * param.s1 * param.s2.powi(2)
                                        + 75. * param.s2.powi(3))
                                + 8. * param.s12.powi(3)
                                    * (15. * param.s1.powi(4)
                                        + 308. * param.s1.powi(3) * param.s2
                                        + -130. * param.s1.powi(2) * param.s2.powi(2)
                                        + -67. * param.s1 * param.s2.powi(3)
                                        + -15. * param.s2.powi(4))
                                + 4. * param.s12.powi(2)
                                    * (6. * param.s1.powi(5)
                                        + -269. * param.s1.powi(4) * param.s2
                                        + -252. * param.s1.powi(3) * param.s2.powi(2)
                                        + 498. * param.s1.powi(2) * param.s2.powi(3)
                                        + -4. * param.s1 * param.s2.powi(4)
                                        + 21. * param.s2.powi(5))
                                + 15.
                                    * param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + 10.
                                            * param.s12.powi(3)
                                            * (-5. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (61. * param.s1.powi(3)
                                                + -43. * param.s1.powi(2) * param.s2
                                                + -9. * param.s1 * param.s2.powi(2)
                                                + -5. * param.s2.powi(3))
                                        + param.s12
                                            * (-31. * param.s1.powi(4)
                                                + -96. * param.s1.powi(3) * param.s2
                                                + 138. * param.s1.powi(2) * param.s2.powi(2)
                                                + -16. * param.s1 * param.s2.powi(3)
                                                + 5. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (25. * param.s1.powi(2)
                                                + 8. * param.s1 * param.s2
                                                - param.s2.powi(2))
                                        - param.s12.powi(4) * (17. * param.s1 + 5. * param.s2))
                                + -6.
                                    * param.m2_2
                                    * (4. * param.s12.powi(6)
                                        + (param.s1 - param.s2).powi(4)
                                            * (9. * param.s1.powi(2)
                                                + 4. * param.s1 * param.s2
                                                - param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (25. * param.s1.powi(2)
                                                + 104. * param.s1 * param.s2
                                                + 35. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (230. * param.s1.powi(3)
                                                + -706. * param.s1.powi(2) * param.s2
                                                + 34. * param.s1 * param.s2.powi(2)
                                                + -30. * param.s2.powi(3))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (101. * param.s1.powi(3)
                                                + 391. * param.s1.powi(2) * param.s2
                                                + 51. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-310. * param.s1.powi(4)
                                                + 464. * param.s1.powi(3) * param.s2
                                                + 404. * param.s1.powi(2) * param.s2.powi(2)
                                                + -136. * param.s1 * param.s2.powi(3)
                                                + 10. * param.s2.powi(4))
                                        - param.s12.powi(5) * (59. * param.s1 + 19. * param.s2))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (27. * param.s1.powi(3)
                                        + 209. * param.s1.powi(2) * param.s2
                                        + 71. * param.s1 * param.s2.powi(2)
                                        + -27. * param.s2.powi(3)))
                        + 2. * param.m1_2
                            * param.s1
                            * (-3.
                                * param.m2_2
                                * (4. * param.s12.powi(6)
                                    + -3.
                                        * param.s12.powi(5)
                                        * (13. * param.s1 + 8. * param.s2)
                                    + param.s12.powi(4)
                                        * (195. * param.s1.powi(2)
                                            + 119. * param.s1 * param.s2
                                            + 60. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(3)
                                        * (145. * param.s1.powi(3)
                                            + 28. * param.s1.powi(2) * param.s2
                                            + 43. * param.s1 * param.s2.powi(2)
                                            + 40. * param.s2.powi(3))
                                    + param.s12.powi(2)
                                        * (30. * param.s1.powi(4)
                                            + 754. * param.s1.powi(3) * param.s2
                                            + -306. * param.s1.powi(2) * param.s2.powi(2)
                                            + -66. * param.s1 * param.s2.powi(3)
                                            + 60. * param.s2.powi(4))
                                    + param.s12
                                        * (201. * param.s1.powi(5)
                                            + -1016. * param.s1.powi(4) * param.s2
                                            + 730. * param.s1.powi(3) * param.s2.powi(2)
                                            + 109. * param.s1 * param.s2.powi(4)
                                            + -24. * param.s2.powi(5))
                                    - (param.s1 - param.s2).powi(3)
                                        * (101. * param.s1.powi(3)
                                            + 80. * param.s1.powi(2) * param.s2
                                            + -25. * param.s1 * param.s2.powi(2)
                                            + 4. * param.s2.powi(3)))
                                + 2. * (3. * param.s12.powi(7)
                                    + -3. * param.s12.powi(6) * (9. * param.s1 + 5. * param.s2)
                                    + param.s12.powi(5)
                                        * (114. * param.s1.powi(2)
                                            + 46. * param.s1 * param.s2
                                            + 27. * param.s2.powi(2))
                                    + param.s12.powi(4)
                                        * (-240. * param.s1.powi(3)
                                            + 214. * param.s1.powi(2) * param.s2
                                            + 61. * param.s1 * param.s2.powi(2)
                                            + -15. * param.s2.powi(3))
                                    + (param.s1 - param.s2).powi(4)
                                        * (6. * param.s1.powi(3)
                                            + 22. * param.s1.powi(2) * param.s2
                                            + -13. * param.s1 * param.s2.powi(2)
                                            + 3. * param.s2.powi(3))
                                    + param.s12.powi(3)
                                        * (255. * param.s1.powi(4)
                                            + 34. * param.s1.powi(3) * param.s2
                                            + -374. * param.s1.powi(2) * param.s2.powi(2)
                                            + -164. * param.s1 * param.s2.powi(3)
                                            + -15. * param.s2.powi(4))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (12. * param.s1.powi(4)
                                            + 568. * param.s1.powi(3) * param.s2
                                            + 243. * param.s1.powi(2) * param.s2.powi(2)
                                            + 8. * param.s1 * param.s2.powi(3)
                                            + -15. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (-123. * param.s1.powi(5)
                                            + -821. * param.s1.powi(4) * param.s2
                                            + 1752. * param.s1.powi(3) * param.s2.powi(2)
                                            + -258. * param.s1.powi(2) * param.s2.powi(3)
                                            + 71. * param.s1 * param.s2.powi(4)
                                            + 27. * param.s2.powi(5)))))
                + param.m0_2
                    * (param.m1_2.powi(4)
                        * (-21. * param.s1.powi(7)
                            + 15. * param.s12.powi(7)
                            + 223. * param.s1.powi(6) * param.s2
                            + -1373. * param.s1.powi(5) * param.s2.powi(2)
                            + -323. * param.s1.powi(4) * param.s2.powi(3)
                            + 1987. * param.s1.powi(3) * param.s2.powi(4)
                            + -617. * param.s1.powi(2) * param.s2.powi(5)
                            + 139. * param.s1 * param.s2.powi(6)
                            + -15. * param.s2.powi(7)
                            + -3. * param.s12.powi(6) * (37. * param.s1 + 35. * param.s2)
                            + param.s12.powi(5)
                                * (351. * param.s1.powi(2)
                                    + 416. * param.s1 * param.s2
                                    + 315. * param.s2.powi(2))
                            + param.s12.powi(3)
                                * (645. * param.s1.powi(4)
                                    + -496. * param.s1.powi(3) * param.s2
                                    + -316. * param.s1.powi(2) * param.s2.powi(2)
                                    + -280. * param.s1 * param.s2.powi(3)
                                    + 525. * param.s2.powi(4))
                            + param.s12
                                * (141. * param.s1.powi(6)
                                    + -896. * param.s1.powi(5) * param.s2
                                    + 2545. * param.s1.powi(4) * param.s2.powi(2)
                                    + -120. * param.s1.powi(3) * param.s2.powi(3)
                                    + 1189. * param.s1.powi(2) * param.s2.powi(4)
                                    + -584. * param.s1 * param.s2.powi(5)
                                    + 105. * param.s2.powi(6))
                            - param.s12.powi(2)
                                * (405. * param.s1.powi(5)
                                    + -1249. * param.s1.powi(4) * param.s2
                                    + 756. * param.s1.powi(3) * param.s2.powi(2)
                                    + 216. * param.s1.powi(2) * param.s2.powi(3)
                                    + -835. * param.s1 * param.s2.powi(4)
                                    + 315. * param.s2.powi(5))
                            - param.s12.powi(4)
                                * (615. * param.s1.powi(3)
                                    + 391. * param.s1.powi(2) * param.s2
                                    + 415. * param.s1 * param.s2.powi(2)
                                    + 525. * param.s2.powi(3)))
                        + param.s1.powi(4)
                            * (-9. * param.s12.powi(7)
                                + param.s12.powi(6) * (57. * param.s1 + 87. * param.s2)
                                + (param.s1 - param.s2).powi(5)
                                    * (3. * param.s1.powi(2)
                                        + -10. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (225. * param.s1.powi(3)
                                        + 289. * param.s1.powi(2) * param.s2
                                        + 334. * param.s1 * param.s2.powi(2)
                                        + 780. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-195. * param.s1.powi(4)
                                        + 64. * param.s1.powi(3) * param.s2
                                        + 490. * param.s1.powi(2) * param.s2.powi(2)
                                        + -1256. * param.s1 * param.s2.powi(3)
                                        + -435. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (99. * param.s1.powi(5)
                                        + -271. * param.s1.powi(4) * param.s2
                                        + -138. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1038. * param.s1.powi(2) * param.s2.powi(3)
                                        + 1459. * param.s1 * param.s2.powi(4)
                                        + -111. * param.s2.powi(5))
                                + -30.
                                    * param.m2_2.powi(4)
                                    * (21. * param.s1.powi(3)
                                        + 9. * param.s12.powi(3)
                                        + 3. * param.s12.powi(2) * (param.s1 + -9. * param.s2)
                                        + 19. * param.s1.powi(2) * param.s2
                                        + -31. * param.s1 * param.s2.powi(2)
                                        + -9. * param.s2.powi(3)
                                        + param.s12
                                            * (-33. * param.s1.powi(2)
                                                + 28. * param.s1 * param.s2
                                                + 27. * param.s2.powi(2)))
                                + 20.
                                    * param.m2_2.powi(3)
                                    * (21. * param.s12.powi(4)
                                        + -30. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + -4.
                                            * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -56. * param.s1 * param.s2
                                                + 9. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (39. * param.s1.powi(3)
                                                + -77. * param.s1.powi(2) * param.s2
                                                + -77. * param.s1 * param.s2.powi(2)
                                                + 39. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (33. * param.s1.powi(2)
                                                + 106. * param.s1 * param.s2
                                                + 33. * param.s2.powi(2)))
                                + -6.
                                    * param.m2_2
                                    * (4. * param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (35. * param.s1.powi(2)
                                                + 104. * param.s1 * param.s2
                                                + 25. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1.powi(3)
                                                + 51. * param.s1.powi(2) * param.s2
                                                + 391. * param.s1 * param.s2.powi(2)
                                                + 101. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-30. * param.s1.powi(3)
                                                + 34. * param.s1.powi(2) * param.s2
                                                + -706. * param.s1 * param.s2.powi(2)
                                                + 230. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (5. * param.s1.powi(4)
                                                + -68. * param.s1.powi(3) * param.s2
                                                + 202. * param.s1.powi(2) * param.s2.powi(2)
                                                + 232. * param.s1 * param.s2.powi(3)
                                                + -155. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        - param.s12.powi(5) * (19. * param.s1 + 59. * param.s2))
                                + -15.
                                    * param.m2_2.powi(2)
                                    * (7. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (-23. * param.s1 + 49. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (11. * param.s1.powi(2)
                                                + 64. * param.s1 * param.s2
                                                + -79. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(3)
                                            * (5. * param.s1.powi(2)
                                                + 60. * param.s1 * param.s2
                                                + 31. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -179. * param.s1.powi(2) * param.s2
                                                + 127. * param.s1 * param.s2.powi(2)
                                                + 55. * param.s2.powi(3))
                                        + param.s12
                                            * (-13. * param.s1.powi(4)
                                                + 136. * param.s1.powi(3) * param.s2
                                                + 246. * param.s1.powi(2) * param.s2.powi(2)
                                                + -392. * param.s1 * param.s2.powi(3)
                                                + 23. * param.s2.powi(4)))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (27. * param.s1.powi(3)
                                        + -71. * param.s1.powi(2) * param.s2
                                        + 46. * param.s1 * param.s2.powi(2)
                                        + 138. * param.s2.powi(3))
                                - param.s12.powi(5)
                                    * (153. * param.s1.powi(2)
                                        + 296. * param.s1 * param.s2
                                        + 438. * param.s2.powi(2)))
                        + -3.
                            * param.m1_2.powi(2)
                            * param.s1.powi(2)
                            * (-6. * param.s12.powi(7)
                                + param.s12.powi(6) * (54. * param.s1 + 50. * param.s2)
                                + -3.
                                    * param.s12.powi(5)
                                    * (66. * param.s1.powi(2)
                                        + 104. * param.s1 * param.s2
                                        + 63. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (390. * param.s1.powi(3)
                                        + 582. * param.s1.powi(2) * param.s2
                                        + 793. * param.s1 * param.s2.powi(2)
                                        + 405. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(3)
                                    * (18. * param.s1.powi(4)
                                        + -112. * param.s1.powi(3) * param.s2
                                        + 355. * param.s1.powi(2) * param.s2.powi(2)
                                        + 248. * param.s1 * param.s2.powi(3)
                                        + -29. * param.s2.powi(4))
                                + -2.
                                    * param.s12.powi(3)
                                    * (225. * param.s1.powi(4)
                                        + 104. * param.s1.powi(3) * param.s2
                                        + -49. * param.s1.powi(2) * param.s2.powi(2)
                                        + 356. * param.s1 * param.s2.powi(3)
                                        + 260. * param.s2.powi(4))
                                + 2. * param.s12.powi(2)
                                    * (153. * param.s1.powi(5)
                                        + -249. * param.s1.powi(4) * param.s2
                                        + -537. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1043. * param.s1.powi(2) * param.s2.powi(3)
                                        + -156. * param.s1 * param.s2.powi(4)
                                        + 198. * param.s2.powi(5))
                                + param.s12
                                    * (-114. * param.s1.powi(6)
                                        + 552. * param.s1.powi(5) * param.s2
                                        + -373. * param.s1.powi(4) * param.s2.powi(2)
                                        + 2320. * param.s1.powi(3) * param.s2.powi(3)
                                        + -3044. * param.s1.powi(2) * param.s2.powi(4)
                                        + 824. * param.s1 * param.s2.powi(5)
                                        + -165. * param.s2.powi(6))
                                + -5.
                                    * param.m2_2.powi(2)
                                    * (-15. * param.s1.powi(5)
                                        + 3. * param.s12.powi(5)
                                        + -299. * param.s1.powi(4) * param.s2
                                        + -146. * param.s1.powi(3) * param.s2.powi(2)
                                        + 406. * param.s1.powi(2) * param.s2.powi(3)
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
                                                + 151. * param.s1.powi(2) * param.s2
                                                + -45. * param.s1 * param.s2.powi(2)
                                                + 15. * param.s2.powi(3))
                                        + param.s12
                                            * (63. * param.s1.powi(4)
                                                + 592. * param.s1.powi(3) * param.s2
                                                + -182. * param.s1.powi(2) * param.s2.powi(2)
                                                + -144. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4)))
                                + -2.
                                    * param.m2_2
                                    * (4. * param.s12.powi(6)
                                        + -39. * param.s12.powi(5) * (param.s1 + param.s2)
                                        + param.s12.powi(4)
                                            * (135. * param.s1.powi(2)
                                                + 284. * param.s1 * param.s2
                                                + 135. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (115. * param.s1.powi(3)
                                                + 163. * param.s1.powi(2) * param.s2
                                                + 163. * param.s1 * param.s2.powi(2)
                                                + 115. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(2)
                                            * (19. * param.s1.powi(4)
                                                + -254. * param.s1.powi(3) * param.s2
                                                + -1250. * param.s1.powi(2) * param.s2.powi(2)
                                                + -254. * param.s1 * param.s2.powi(3)
                                                + 19. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (210. * param.s1.powi(4)
                                                + -336. * param.s1.powi(3) * param.s2
                                                + 2284. * param.s1.powi(2) * param.s2.powi(2)
                                                + -336. * param.s1 * param.s2.powi(3)
                                                + 210. * param.s2.powi(4))
                                        - param.s12
                                            * (99. * param.s1.powi(5)
                                                + -709. * param.s1.powi(4) * param.s2
                                                + 1370. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1370. * param.s1.powi(2) * param.s2.powi(3)
                                                + -709. * param.s1 * param.s2.powi(4)
                                                + 99. * param.s2.powi(5))))
                        + -2.
                            * param.m1_2
                            * param.s1.powi(3)
                            * (20.
                                * param.m2_2.powi(3)
                                * (-24. * param.s1.powi(4)
                                    + 3. * param.s12.powi(4)
                                    + 3. * param.s12.powi(3) * (5. * param.s1 + -4. * param.s2)
                                    + -91. * param.s1.powi(3) * param.s2
                                    + 47. * param.s1.powi(2) * param.s2.powi(2)
                                    + 65. * param.s1 * param.s2.powi(3)
                                    + 3. * param.s2.powi(4)
                                    + param.s12.powi(2)
                                        * (-63. * param.s1.powi(2)
                                            + 35. * param.s1 * param.s2
                                            + 18. * param.s2.powi(2))
                                    + param.s12
                                        * (69. * param.s1.powi(3)
                                            + 68. * param.s1.powi(2) * param.s2
                                            + -115. * param.s1 * param.s2.powi(2)
                                            + -12. * param.s2.powi(3)))
                                + -30.
                                    * param.m2_2.powi(2)
                                    * (param.s12.powi(5)
                                        + param.s12.powi(4) * (param.s1 + param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (7. * param.s1.powi(2)
                                                + -43. * param.s1 * param.s2
                                                + 7. * param.s2.powi(2))
                                        + 26.
                                            * param.s12.powi(2)
                                            * (param.s1.powi(3)
                                                + -4. * param.s1.powi(2) * param.s2
                                                + -4. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(2)
                                            * (5. * param.s1.powi(3)
                                                + 81. * param.s1.powi(2) * param.s2
                                                + 81. * param.s1 * param.s2.powi(2)
                                                + 5. * param.s2.powi(3))
                                        - param.s12
                                            * (19. * param.s1.powi(4)
                                                + 54. * param.s1.powi(3) * param.s2
                                                + -298. * param.s1.powi(2) * param.s2.powi(2)
                                                + 54. * param.s1 * param.s2.powi(3)
                                                + 19. * param.s2.powi(4)))
                                + -2.
                                    * (3. * param.s12.powi(7)
                                        + -3.
                                            * param.s12.powi(6)
                                            * (5. * param.s1 + 9. * param.s2)
                                        + param.s12.powi(5)
                                            * (27. * param.s1.powi(2)
                                                + 46. * param.s1 * param.s2
                                                + 114. * param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (-15. * param.s1.powi(3)
                                                + 61. * param.s1.powi(2) * param.s2
                                                + 214. * param.s1 * param.s2.powi(2)
                                                + -240. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(4)
                                            * (3. * param.s1.powi(3)
                                                + -13. * param.s1.powi(2) * param.s2
                                                + 22. * param.s1 * param.s2.powi(2)
                                                + 6. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-15. * param.s1.powi(4)
                                                + -164. * param.s1.powi(3) * param.s2
                                                + -374. * param.s1.powi(2) * param.s2.powi(2)
                                                + 34. * param.s1 * param.s2.powi(3)
                                                + 255. * param.s2.powi(4))
                                        + param.s12.powi(2)
                                            * (27. * param.s1.powi(5)
                                                + 71. * param.s1.powi(4) * param.s2
                                                + -258. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1752. * param.s1.powi(2) * param.s2.powi(3)
                                                + -821. * param.s1 * param.s2.powi(4)
                                                + -123. * param.s2.powi(5))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (15. * param.s1.powi(4)
                                                + -8. * param.s1.powi(3) * param.s2
                                                + -243. * param.s1.powi(2) * param.s2.powi(2)
                                                + -568. * param.s1 * param.s2.powi(3)
                                                + -12. * param.s2.powi(4)))
                                + -3.
                                    * param.m2_2
                                    * (4. * param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (-15. * param.s1.powi(2)
                                                + -111. * param.s1 * param.s2
                                                + 100. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (70. * param.s1.powi(3)
                                                + 464. * param.s1.powi(2) * param.s2
                                                + -966. * param.s1 * param.s2.powi(2)
                                                + -40. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (45. * param.s1.powi(4)
                                                + 143. * param.s1.powi(3) * param.s2
                                                + 553. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1027. * param.s1 * param.s2.powi(3)
                                                + 50. * param.s2.powi(4))
                                        + param.s12
                                            * (51. * param.s1.powi(5)
                                                + -156. * param.s1.powi(4) * param.s2
                                                + 1950. * param.s1.powi(3) * param.s2.powi(2)
                                                + -1240. * param.s1.powi(2) * param.s2.powi(3)
                                                + -721. * param.s1 * param.s2.powi(4)
                                                + 116. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(3)
                                            * (11. * param.s1.powi(3)
                                                + -100. * param.s1.powi(2) * param.s2
                                                + -355. * param.s1 * param.s2.powi(2)
                                                + -36. * param.s2.powi(3))
                                        - param.s12.powi(5) * (9. * param.s1 + 44. * param.s2)))
                        + -2.
                            * param.m1_2.powi(3)
                            * param.s1
                            * (3.
                                * param.m2_2
                                * (9. * param.s1.powi(6)
                                    + 4. * param.s12.powi(6)
                                    + -167. * param.s1.powi(5) * param.s2
                                    + -838. * param.s1.powi(4) * param.s2.powi(2)
                                    + 702. * param.s1.powi(3) * param.s2.powi(3)
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
                                            + 890. * param.s1.powi(3) * param.s2.powi(2)
                                            + -400. * param.s1.powi(2) * param.s2.powi(3)
                                            + 159. * param.s1 * param.s2.powi(4)
                                            + -24. * param.s2.powi(5))
                                    - param.s12.powi(5) * (29. * param.s1 + 24. * param.s2))
                                + 2. * (9. * param.s12.powi(7)
                                    + -69. * param.s12.powi(6) * (param.s1 + param.s2)
                                    + 5. * param.s12.powi(5)
                                        * (45. * param.s1.powi(2)
                                            + 62. * param.s1 * param.s2
                                            + 45. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (435. * param.s1.powi(4)
                                            + -116. * param.s1.powi(3) * param.s2
                                            + -404. * param.s1.powi(2) * param.s2.powi(2)
                                            + -116. * param.s1 * param.s2.powi(3)
                                            + 435. * param.s2.powi(4))
                                    + param.s12.powi(2)
                                        * (-279. * param.s1.powi(5)
                                            + 689. * param.s1.powi(4) * param.s2
                                            + 324. * param.s1.powi(3) * param.s2.powi(2)
                                            + 324. * param.s1.powi(2) * param.s2.powi(3)
                                            + 689. * param.s1 * param.s2.powi(4)
                                            + -279. * param.s2.powi(5))
                                    + param.s12
                                        * (99. * param.s1.powi(6)
                                            + -562. * param.s1.powi(5) * param.s2
                                            + 1043. * param.s1.powi(4) * param.s2.powi(2)
                                            + -1920. * param.s1.powi(3) * param.s2.powi(3)
                                            + 1043. * param.s1.powi(2) * param.s2.powi(4)
                                            + -562. * param.s1 * param.s2.powi(5)
                                            + 99. * param.s2.powi(6))
                                    - (param.s1 - param.s2).powi(2)
                                        * (15. * param.s1.powi(5)
                                            + -119. * param.s1.powi(4) * param.s2
                                            + 534. * param.s1.powi(3) * param.s2.powi(2)
                                            + 534. * param.s1.powi(2) * param.s2.powi(3)
                                            + -119. * param.s1 * param.s2.powi(4)
                                            + 15. * param.s2.powi(5))
                                    - param.s12.powi(4)
                                        * (405. * param.s1.powi(3)
                                            + 401. * param.s1.powi(2) * param.s2
                                            + 401. * param.s1 * param.s2.powi(2)
                                            + 405. * param.s2.powi(3)))))
                - param.m0_2.powi(2)
                    * (param.m1_2.powi(3)
                        * (30. * param.s12.powi(7)
                            + -6. * param.s12.powi(6) * (39. * param.s1 + 35. * param.s2)
                            + param.s12.powi(5)
                                * (789. * param.s1.powi(2)
                                    + 904. * param.s1 * param.s2
                                    + 630. * param.s2.powi(2))
                            + 2. * param.s12.powi(3)
                                * (840. * param.s1.powi(4)
                                    + -652. * param.s1.powi(3) * param.s2
                                    + -337. * param.s1.powi(2) * param.s2.powi(2)
                                    + -160. * param.s1 * param.s2.powi(3)
                                    + 525. * param.s2.powi(4))
                            + -2.
                                * param.s12.powi(2)
                                * (570. * param.s1.powi(5)
                                    + -1978. * param.s1.powi(4) * param.s2
                                    + 567. * param.s1.powi(3) * param.s2.powi(2)
                                    + -33. * param.s1.powi(2) * param.s2.powi(3)
                                    + -745. * param.s1 * param.s2.powi(4)
                                    + 315. * param.s2.powi(5))
                            + param.s12
                                * (429. * param.s1.powi(6)
                                    + -3304. * param.s1.powi(5) * param.s2
                                    + 2420. * param.s1.powi(4) * param.s2.powi(2)
                                    + 960. * param.s1.powi(3) * param.s2.powi(3)
                                    + 1901. * param.s1.powi(2) * param.s2.powi(4)
                                    + -1096. * param.s1 * param.s2.powi(5)
                                    + 210. * param.s2.powi(6))
                            - (param.s1 - param.s2).powi(2)
                                * (69. * param.s1.powi(5)
                                    + -809. * param.s1.powi(4) * param.s2
                                    + -1455. * param.s1.powi(3) * param.s2.powi(2)
                                    + 651. * param.s1.powi(2) * param.s2.powi(3)
                                    + -206. * param.s1 * param.s2.powi(4)
                                    + 30. * param.s2.powi(5))
                            - param.s12.powi(4)
                                * (1485. * param.s1.powi(3)
                                    + 989. * param.s1.powi(2) * param.s2
                                    + 1010. * param.s1 * param.s2.powi(2)
                                    + 1050. * param.s2.powi(3)))
                        + param.s1.powi(3)
                            * (-6. * param.s12.powi(7)
                                + 6. * param.s12.powi(6) * (3. * param.s1 + 13. * param.s2)
                                + param.s12.powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 56. * param.s1 * param.s2
                                        + -237. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (27. * param.s1.powi(3)
                                        + -71. * param.s1.powi(2) * param.s2
                                        + -209. * param.s1 * param.s2.powi(2)
                                        + -27. * param.s2.powi(3))
                                + param.s12.powi(4)
                                    * (-75. * param.s1.powi(3)
                                        + -559. * param.s1.powi(2) * param.s2
                                        + 1181. * param.s1 * param.s2.powi(2)
                                        + 285. * param.s2.powi(3))
                                + 8. * param.s12.powi(3)
                                    * (15. * param.s1.powi(4)
                                        + 67. * param.s1.powi(3) * param.s2
                                        + 130. * param.s1.powi(2) * param.s2.powi(2)
                                        + -308. * param.s1 * param.s2.powi(3)
                                        + -15. * param.s2.powi(4))
                                + -4.
                                    * param.s12.powi(2)
                                    * (21. * param.s1.powi(5)
                                        + -4. * param.s1.powi(4) * param.s2
                                        + 498. * param.s1.powi(3) * param.s2.powi(2)
                                        + -252. * param.s1.powi(2) * param.s2.powi(3)
                                        + -269. * param.s1 * param.s2.powi(4)
                                        + 6. * param.s2.powi(5))
                                + -20.
                                    * param.m2_2.powi(3)
                                    * (3. * param.s12.powi(4)
                                        + 6. * param.s12.powi(3)
                                            * (7. * param.s1 + -2. * param.s2)
                                        + -2.
                                            * param.s12.powi(2)
                                            * (27. * param.s1.powi(2)
                                                + 23. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (39. * param.s1.powi(2)
                                                + 44. * param.s1 * param.s2
                                                + 3. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (15. * param.s1.powi(3)
                                                + -76. * param.s1.powi(2) * param.s2
                                                + 17. * param.s1 * param.s2.powi(2)
                                                + 6. * param.s2.powi(3)))
                                + 15.
                                    * param.m2_2.powi(2)
                                    * (7. * param.s12.powi(5)
                                        + param.s12.powi(4)
                                            * (49. * param.s1 + -23. * param.s2)
                                        + param.s12.powi(3)
                                            * (-158. * param.s1.powi(2)
                                                + 128. * param.s1 * param.s2
                                                + 22. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (55. * param.s1.powi(3)
                                                + 127. * param.s1.powi(2) * param.s2
                                                + -179. * param.s1 * param.s2.powi(2)
                                                + param.s2.powi(3))
                                        + param.s12
                                            * (23. * param.s1.powi(4)
                                                + -392. * param.s1.powi(3) * param.s2
                                                + 246. * param.s1.powi(2) * param.s2.powi(2)
                                                + 136. * param.s1 * param.s2.powi(3)
                                                + -13. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(3)
                                            * (31. * param.s1.powi(2)
                                                + 60. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2)))
                                + -6.
                                    * param.m2_2
                                    * (6. * param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (-55. * param.s1.powi(2)
                                                + 416. * param.s1 * param.s2
                                                + -55. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + 16. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (19. * param.s1.powi(3)
                                                + 389. * param.s1.powi(2) * param.s2
                                                + 389. * param.s1 * param.s2.powi(2)
                                                + 19. * param.s2.powi(3))
                                        + 2. * param.s12.powi(3)
                                            * (55. * param.s1.powi(3)
                                                + -227. * param.s1.powi(2) * param.s2
                                                + -227. * param.s1 * param.s2.powi(2)
                                                + 55. * param.s2.powi(3))
                                        + -4.
                                            * param.s12.powi(2)
                                            * (20. * param.s1.powi(4)
                                                + 81. * param.s1.powi(3) * param.s2
                                                + -364. * param.s1.powi(2) * param.s2.powi(2)
                                                + 81. * param.s1 * param.s2.powi(3)
                                                + 20. * param.s2.powi(4))
                                        - param.s12.powi(5) * (param.s1 + param.s2))
                                - (param.s1 - param.s2).powi(5)
                                    * (3. * param.s1.powi(2)
                                        + -10. * param.s1 * param.s2
                                        + -3. * param.s2.powi(2)))
                        + -3.
                            * param.m1_2.powi(2)
                            * param.s1
                            * (6. * param.s12.powi(7)
                                + -2. * param.s12.powi(6) * (25. * param.s1 + 27. * param.s2)
                                + 3. * param.s12.powi(5)
                                    * (63. * param.s1.powi(2)
                                        + 104. * param.s1 * param.s2
                                        + 66. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (520. * param.s1.powi(4)
                                        + 712. * param.s1.powi(3) * param.s2
                                        + -98. * param.s1.powi(2) * param.s2.powi(2)
                                        + 208. * param.s1 * param.s2.powi(3)
                                        + 450. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-396. * param.s1.powi(5)
                                        + 312. * param.s1.powi(4) * param.s2
                                        + -2086. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1074. * param.s1.powi(2) * param.s2.powi(3)
                                        + 498. * param.s1 * param.s2.powi(4)
                                        + -306. * param.s2.powi(5))
                                + param.s12
                                    * (165. * param.s1.powi(6)
                                        + -824. * param.s1.powi(5) * param.s2
                                        + 3044. * param.s1.powi(4) * param.s2.powi(2)
                                        + -2320. * param.s1.powi(3) * param.s2.powi(3)
                                        + 373. * param.s1.powi(2) * param.s2.powi(4)
                                        + -552. * param.s1 * param.s2.powi(5)
                                        + 114. * param.s2.powi(6))
                                + 2. * param.m2_2
                                    * (6. * param.s12.powi(6)
                                        + -3.
                                            * param.s12.powi(5)
                                            * (17. * param.s1 + 12. * param.s2)
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
                                                + 599. * param.s1.powi(3) * param.s2
                                                + 255. * param.s1.powi(2) * param.s2.powi(2)
                                                + -51. * param.s1 * param.s2.powi(3)
                                                + 6. * param.s2.powi(4))
                                        + 2. * param.s12.powi(2)
                                            * (210. * param.s1.powi(4)
                                                + 13. * param.s1.powi(3) * param.s2
                                                + -207. * param.s1.powi(2) * param.s2.powi(2)
                                                + -87. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        - param.s12
                                            * (231. * param.s1.powi(5)
                                                + 724. * param.s1.powi(4) * param.s2
                                                + -1790. * param.s1.powi(3) * param.s2.powi(2)
                                                + 240. * param.s1.powi(2) * param.s2.powi(3)
                                                + -201. * param.s1 * param.s2.powi(4)
                                                + 36. * param.s2.powi(5)))
                                - (param.s1 - param.s2).powi(3)
                                    * (29. * param.s1.powi(4)
                                        + -248. * param.s1.powi(3) * param.s2
                                        + -355. * param.s1.powi(2) * param.s2.powi(2)
                                        + 112. * param.s1 * param.s2.powi(3)
                                        + -18. * param.s2.powi(4))
                                - param.s12.powi(4)
                                    * (405. * param.s1.powi(3)
                                        + 793. * param.s1.powi(2) * param.s2
                                        + 582. * param.s1 * param.s2.powi(2)
                                        + 390. * param.s2.powi(3)))
                        + 3. * param.m1_2
                            * param.s1.powi(2)
                            * (-2. * param.s12.powi(7)
                                + 22. * param.s12.powi(6) * (param.s1 + param.s2)
                                + -25.
                                    * param.s12.powi(5)
                                    * (3. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (115. * param.s1.powi(3)
                                        + 203. * param.s1.powi(2) * param.s2
                                        + 203. * param.s1 * param.s2.powi(2)
                                        + 115. * param.s2.powi(3))
                                + -8.
                                    * param.s12.powi(3)
                                    * (10. * param.s1.powi(4)
                                        + -41. * param.s1.powi(3) * param.s2
                                        + 271. * param.s1.powi(2) * param.s2.powi(2)
                                        + -41. * param.s1 * param.s2.powi(3)
                                        + 10. * param.s2.powi(4))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (13. * param.s1.powi(4)
                                        + 162. * param.s1.powi(3) * param.s2
                                        + 1282. * param.s1.powi(2) * param.s2.powi(2)
                                        + 162. * param.s1 * param.s2.powi(3)
                                        + 13. * param.s2.powi(4))
                                + 4. * param.s12.powi(2)
                                    * (3. * param.s1.powi(5)
                                        + -133. * param.s1.powi(4) * param.s2
                                        + 292. * param.s1.powi(3) * param.s2.powi(2)
                                        + 292. * param.s1.powi(2) * param.s2.powi(3)
                                        + -133. * param.s1 * param.s2.powi(4)
                                        + 3. * param.s2.powi(5))
                                + 5. * param.m2_2.powi(2)
                                    * (3. * param.s12.powi(5)
                                        + -3.
                                            * param.s12.powi(4)
                                            * (13. * param.s1 + 5. * param.s2)
                                        + 6. * param.s12.powi(3)
                                            * (3. * param.s1.powi(2)
                                                + 12. * param.s1 * param.s2
                                                + 5. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (75. * param.s1.powi(3)
                                                + -221. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -15. * param.s2.powi(3))
                                        + (param.s1 - param.s2).powi(2)
                                            * (81. * param.s1.powi(3)
                                                + 227. * param.s1.powi(2) * param.s2
                                                + 39. * param.s1 * param.s2.powi(2)
                                                + -3. * param.s2.powi(3))
                                        + param.s12
                                            * (-213. * param.s1.powi(4)
                                                + 320. * param.s1.powi(3) * param.s2
                                                + 278. * param.s1.powi(2) * param.s2.powi(2)
                                                + -96. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4)))
                                + -2.
                                    * param.m2_2
                                    * (4. * param.s12.powi(6)
                                        + param.s12.powi(4)
                                            * (100. * param.s1.powi(2)
                                                + -111. * param.s1 * param.s2
                                                + -15. * param.s2.powi(2))
                                        + param.s12.powi(3)
                                            * (-40. * param.s1.powi(3)
                                                + -966. * param.s1.powi(2) * param.s2
                                                + 464. * param.s1 * param.s2.powi(2)
                                                + 70. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (50. * param.s1.powi(4)
                                                + -1027. * param.s1.powi(3) * param.s2
                                                + 553. * param.s1.powi(2) * param.s2.powi(2)
                                                + 143. * param.s1 * param.s2.powi(3)
                                                + 45. * param.s2.powi(4))
                                        + param.s12
                                            * (116. * param.s1.powi(5)
                                                + -721. * param.s1.powi(4) * param.s2
                                                + -1240. * param.s1.powi(3) * param.s2.powi(2)
                                                + 1950. * param.s1.powi(2) * param.s2.powi(3)
                                                + -156. * param.s1 * param.s2.powi(4)
                                                + 51. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(3)
                                            * (36. * param.s1.powi(3)
                                                + 355. * param.s1.powi(2) * param.s2
                                                + 100. * param.s1 * param.s2.powi(2)
                                                + -11. * param.s2.powi(3))
                                        - param.s12.powi(5) * (44. * param.s1 + 9. * param.s2))
                                - (param.s1 - param.s2).powi(4)
                                    * (5. * param.s1.powi(3)
                                        + -23. * param.s1.powi(2) * param.s2
                                        + -23. * param.s1 * param.s2.powi(2)
                                        + 5. * param.s2.powi(3))))
                - param.m0_2.powi(4)
                    * (param.m1_2
                        * (15. * param.s12.powi(7)
                            + -3. * param.s12.powi(6) * (43. * param.s1 + 35. * param.s2)
                            + param.s12.powi(5)
                                * (504. * param.s1.powi(2)
                                    + 524. * param.s1 * param.s2
                                    + 315. * param.s2.powi(2))
                            + (param.s1 - param.s2).powi(4)
                                * (66. * param.s1.powi(3)
                                    + -94. * param.s1.powi(2) * param.s2
                                    + 61. * param.s1 * param.s2.powi(2)
                                    + -15. * param.s2.powi(3))
                            + param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (144. * param.s1.powi(4)
                                    + 884. * param.s1.powi(3) * param.s2
                                    + -51. * param.s1.powi(2) * param.s2.powi(2)
                                    + -266. * param.s1 * param.s2.powi(3)
                                    + 105. * param.s2.powi(4))
                            + param.s12.powi(3)
                                * (1665. * param.s1.powi(4)
                                    + -664. * param.s1.powi(3) * param.s2
                                    + -154. * param.s1.powi(2) * param.s2.powi(2)
                                    + 80. * param.s1 * param.s2.powi(3)
                                    + 525. * param.s2.powi(4))
                            + param.s12.powi(2)
                                * (-1035. * param.s1.powi(5)
                                    + 821. * param.s1.powi(4) * param.s2
                                    + 306. * param.s1.powi(3) * param.s2.powi(2)
                                    + 306. * param.s1.powi(2) * param.s2.powi(3)
                                    + 565. * param.s1 * param.s2.powi(4)
                                    + -315. * param.s2.powi(5))
                            - param.s12.powi(4)
                                * (1230. * param.s1.powi(3)
                                    + 814. * param.s1.powi(2) * param.s2
                                    + 685. * param.s1 * param.s2.powi(2)
                                    + 525. * param.s2.powi(3)))
                        + param.s1
                            * (9. * param.s12.powi(7)
                                + -3. * param.s12.powi(6) * (29. * param.s1 + 19. * param.s2)
                                + (param.s1 - param.s2).powi(5)
                                    * (12. * param.s1.powi(2)
                                        + -10. * param.s1 * param.s2
                                        + 3. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (438. * param.s1.powi(2)
                                        + 296. * param.s1 * param.s2
                                        + 153. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (435. * param.s1.powi(4)
                                        + 1256. * param.s1.powi(3) * param.s2
                                        + -490. * param.s1.powi(2) * param.s2.powi(2)
                                        + -64. * param.s1 * param.s2.powi(3)
                                        + 195. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (111. * param.s1.powi(5)
                                        + -1459. * param.s1.powi(4) * param.s2
                                        + 1038. * param.s1.powi(3) * param.s2.powi(2)
                                        + 138. * param.s1.powi(2) * param.s2.powi(3)
                                        + 271. * param.s1 * param.s2.powi(4)
                                        + -99. * param.s2.powi(5))
                                + -6.
                                    * param.m2_2
                                    * (param.s12.powi(6)
                                        + (param.s1 - param.s2).powi(4)
                                            * (6. * param.s1.powi(2)
                                                + -4. * param.s1 * param.s2
                                                + param.s2.powi(2))
                                        + param.s12.powi(4)
                                            * (70. * param.s1.powi(2)
                                                + 36. * param.s1 * param.s2
                                                + 15. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (89. * param.s1.powi(3)
                                                + 44. * param.s1.powi(2) * param.s2
                                                + 9. * param.s1 * param.s2.powi(2)
                                                + -6. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (5. * param.s1.powi(3)
                                                + 32. * param.s1.powi(2) * param.s2
                                                + 17. * param.s1 * param.s2.powi(2)
                                                + 10. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-145. * param.s1.powi(4)
                                                + 296. * param.s1.powi(3) * param.s2
                                                + -54. * param.s1.powi(2) * param.s2.powi(2)
                                                + -4. * param.s1 * param.s2.powi(3)
                                                + 15. * param.s2.powi(4))
                                        - param.s12.powi(5) * (11. * param.s1 + 6. * param.s2))
                                - param.s12.powi(4)
                                    * (780. * param.s1.powi(3)
                                        + 334. * param.s1.powi(2) * param.s2
                                        + 289. * param.s1 * param.s2.powi(2)
                                        + 225. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (138. * param.s1.powi(3)
                                        + 46. * param.s1.powi(2) * param.s2
                                        + -71. * param.s1 * param.s2.powi(2)
                                        + 27. * param.s2.powi(3))))
                - param.m1_2.powi(4)
                    * param.s1
                    * (15. * param.s1.powi(7)
                        + -15. * param.s12.powi(7)
                        + -139. * param.s1.powi(6) * param.s2
                        + 617. * param.s1.powi(5) * param.s2.powi(2)
                        + -1987. * param.s1.powi(4) * param.s2.powi(3)
                        + 323. * param.s1.powi(3) * param.s2.powi(4)
                        + 1373. * param.s1.powi(2) * param.s2.powi(5)
                        + -223. * param.s1 * param.s2.powi(6)
                        + 21. * param.s2.powi(7)
                        + 3. * param.s12.powi(6) * (35. * param.s1 + 37. * param.s2)
                        + param.s12.powi(4)
                            * (525. * param.s1.powi(3)
                                + 415. * param.s1.powi(2) * param.s2
                                + 391. * param.s1 * param.s2.powi(2)
                                + 615. * param.s2.powi(3))
                        + param.s12.powi(3)
                            * (-525. * param.s1.powi(4)
                                + 280. * param.s1.powi(3) * param.s2
                                + 316. * param.s1.powi(2) * param.s2.powi(2)
                                + 496. * param.s1 * param.s2.powi(3)
                                + -645. * param.s2.powi(4))
                        + param.s12.powi(2)
                            * (315. * param.s1.powi(5)
                                + -835. * param.s1.powi(4) * param.s2
                                + 216. * param.s1.powi(3) * param.s2.powi(2)
                                + 756. * param.s1.powi(2) * param.s2.powi(3)
                                + -1249. * param.s1 * param.s2.powi(4)
                                + 405. * param.s2.powi(5))
                        + param.s12
                            * (-105. * param.s1.powi(6)
                                + 584. * param.s1.powi(5) * param.s2
                                + -1189. * param.s1.powi(4) * param.s2.powi(2)
                                + 120. * param.s1.powi(3) * param.s2.powi(3)
                                + -2545. * param.s1.powi(2) * param.s2.powi(4)
                                + 896. * param.s1 * param.s2.powi(5)
                                + -141. * param.s2.powi(6))
                        + -6.
                            * param.m2_2
                            * (param.s1.powi(6)
                                + param.s12.powi(6)
                                + -13. * param.s1.powi(5) * param.s2
                                + 113. * param.s1.powi(4) * param.s2.powi(2)
                                + 498. * param.s1.powi(3) * param.s2.powi(3)
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
                        - param.s12.powi(5)
                            * (315. * param.s1.powi(2)
                                + 416. * param.s1 * param.s2
                                + 351. * param.s2.powi(2)))
                - param.s1.powi(5)
                    * (-3. * param.s12.powi(7)
                        + 3. * param.s12.powi(6) * (7. * param.s1 + 9. * param.s2)
                        + (param.s1 - param.s2).powi(5)
                            * (3. * param.s1.powi(2)
                                + -8. * param.s1 * param.s2
                                + 6. * param.s2.powi(2))
                        + param.s12.powi(4)
                            * (105. * param.s1.powi(3)
                                + 155. * param.s1.powi(2) * param.s2
                                + 206. * param.s1 * param.s2.powi(2)
                                + 330. * param.s2.powi(3))
                        + param.s12.powi(2)
                            * (63. * param.s1.powi(5)
                                + -95. * param.s1.powi(4) * param.s2
                                + -66. * param.s1.powi(3) * param.s2.powi(2)
                                + -126. * param.s1.powi(2) * param.s2.powi(3)
                                + 191. * param.s1 * param.s2.powi(4)
                                + 33. * param.s2.powi(5))
                        + -60.
                            * param.m2_2.powi(5)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + 30.
                            * param.m2_2.powi(4)
                            * (-9. * param.s1.powi(3)
                                + 9. * param.s12.powi(3)
                                + -31. * param.s1.powi(2) * param.s2
                                + 19. * param.s1 * param.s2.powi(2)
                                + 21. * param.s2.powi(3)
                                + 3. * param.s12.powi(2) * (-9. * param.s1 + param.s2)
                                + param.s12
                                    * (27. * param.s1.powi(2)
                                        + 28. * param.s1 * param.s2
                                        + -33. * param.s2.powi(2)))
                        + -20.
                            * param.m2_2.powi(3)
                            * (3. * param.s12.powi(4)
                                + -6. * param.s12.powi(3) * (2. * param.s1 + -7. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + -23. * param.s1 * param.s2
                                        + -27. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1.powi(2)
                                        + 44. * param.s1 * param.s2
                                        + 39. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (6. * param.s1.powi(3)
                                        + 17. * param.s1.powi(2) * param.s2
                                        + -76. * param.s1 * param.s2.powi(2)
                                        + 15. * param.s2.powi(3)))
                        + -6.
                            * param.m2_2
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
                                        + 5. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (15. * param.s1.powi(4)
                                        + -4. * param.s1.powi(3) * param.s2
                                        + -54. * param.s1.powi(2) * param.s2.powi(2)
                                        + 296. * param.s1 * param.s2.powi(3)
                                        + -145. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (6. * param.s1.powi(3)
                                        + -9. * param.s1.powi(2) * param.s2
                                        + -44. * param.s1 * param.s2.powi(2)
                                        + -89. * param.s2.powi(3))
                                - param.s12.powi(5) * (6. * param.s1 + 11. * param.s2))
                        + -15.
                            * param.m2_2.powi(2)
                            * (param.s12.powi(5)
                                + 10.
                                    * param.s12.powi(3)
                                    * (param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + 9. * param.s1.powi(2) * param.s2
                                        + 43. * param.s1 * param.s2.powi(2)
                                        + -61. * param.s2.powi(3))
                                + param.s12
                                    * (5. * param.s1.powi(4)
                                        + -16. * param.s1.powi(3) * param.s2
                                        + 138. * param.s1.powi(2) * param.s2.powi(2)
                                        + -96. * param.s1 * param.s2.powi(3)
                                        + -31. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(3)
                                    * (param.s1.powi(2)
                                        + -8. * param.s1 * param.s2
                                        + -25. * param.s2.powi(2))
                                - param.s12.powi(4) * (5. * param.s1 + 17. * param.s2))
                        - param.s12.powi(3)
                            * (105. * param.s1.powi(4)
                                + 40. * param.s1.powi(3) * param.s2
                                + 10. * param.s1.powi(2) * param.s2.powi(2)
                                + -56. * param.s1 * param.s2.powi(3)
                                + 345. * param.s2.powi(4))
                        - param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (21. * param.s1.powi(3)
                                + -25. * param.s1.powi(2) * param.s2
                                + -46. * param.s1 * param.s2.powi(2)
                                + 78. * param.s2.powi(3))
                        - param.s12.powi(5)
                            * (63. * param.s1.powi(2)
                                + 112. * param.s1 * param.s2
                                + 114. * param.s2.powi(2))))
                * param.lambda_m01_sqrt
                * param.lambda_s12_sqrt
                + 60.
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
                    * ((14. * param.m1_2.powi(2)
                        + 3. * param.s12.powi(2)
                        + param.s12 * (8. * param.s1 + -6. * param.s2)
                        + 3. * (param.s1 - param.s2).powi(2)
                        + -14. * param.m1_2 * (param.s1 + param.s12 - param.s2))
                        * param.s2.powi(2)
                        + param.m2_2.powi(2)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + -2.
                            * param.m2_2
                            * param.s2
                            * (-4. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + param.s12 * (param.s1 + -6. * param.s2)
                                + param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + 7. * param.m1_2 * (param.s1 + param.s2 - param.s12))
                        + param.m0_2.powi(2)
                            * (3. * param.s12.powi(2)
                                + 3. * (param.s1 - param.s2).powi(2)
                                + param.s12 * (-6. * param.s1 + 8. * param.s2))
                        + -2.
                            * param.m0_2
                            * (param.m2_2
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -4. * param.s2.powi(2)
                                    + param.s12 * (-6. * param.s1 + param.s2))
                                + param.s2
                                    * (-4. * param.s12.powi(2)
                                        + 3. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2)
                                        + 7. * param.m1_2 * (param.s12 + param.s2 - param.s1))))
                    * log_diff(
                        param.m0_2 * (param.s1 + param.s12 - param.s2)
                            + param.m1_2 * (param.s1 + param.s2 - param.s12)
                            + param.s1 * (-2. * param.m2_2 + param.s12 + param.s2 - param.s1),
                        param.lambda_m01_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s2 > (param.m0 + param.m2).powi(2) {
        0.0006944444444444444
            * std::f64::consts::PI
            * param.s2.powi(-2)
            * param.lambda_s12_sqrt.powi(-9)
            * ((param.m0_2.powi(5)
                * (param.s12.powi(5)
                    + param.s12
                        * (param.s1 - param.s2).powi(3)
                        * (5. * param.s1 + 23. * param.s2)
                    + 2. * param.s12.powi(3)
                        * (5. * param.s1.powi(2)
                            + 32. * param.s1 * param.s2
                            + -199. * param.s2.powi(2))
                    + -2.
                        * param.s12.powi(2)
                        * (5. * param.s1.powi(3)
                            + 27. * param.s1.powi(2) * param.s2
                            + -231. * param.s1 * param.s2.powi(2)
                            + 199. * param.s2.powi(3))
                    - param.s12.powi(4) * (5. * param.s1 + 23. * param.s2)
                    - (param.s1 - param.s2).powi(5))
                + param.m2_2.powi(5)
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
                + param.s2.powi(5)
                    * (840. * param.m1_2.powi(5)
                        + param.s12.powi(5)
                        + 40.
                            * param.m1_2.powi(3)
                            * (43. * param.s12.powi(2)
                                + 2. * param.s12 * (62. * param.s1 + -43. * param.s2)
                                + 43. * (param.s1 - param.s2).powi(2))
                        + (param.s1 - param.s2).powi(5)
                        + -2100. * param.m1_2.powi(4) * (param.s1 + param.s12 - param.s2)
                        + param.s12.powi(3)
                            * (-398. * param.s1.powi(2)
                                + 64. * param.s1 * param.s2
                                + 10. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (199. * param.s1.powi(3)
                                + -231. * param.s1.powi(2) * param.s2
                                + 27. * param.s1 * param.s2.powi(2)
                                + 5. * param.s2.powi(3))
                        + 6. * param.m1_2
                            * (3. * param.s12.powi(4)
                                + 4. * param.s12.powi(3) * (37. * param.s1 + -3. * param.s2)
                                + 4. * param.s12
                                    * (37. * param.s1 + -3. * param.s2)
                                    * (param.s1 - param.s2).powi(2)
                                + 3. * (param.s1 - param.s2).powi(4)
                                + 2. * param.s12.powi(2)
                                    * (199. * param.s1.powi(2)
                                        + -154. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2)))
                        + -120.
                            * param.m1_2.powi(2)
                            * (4. * param.s12.powi(3)
                                + param.s12.powi(2) * (31. * param.s1 + -12. * param.s2)
                                + 4. * (param.s1 - param.s2).powi(3)
                                + param.s12
                                    * (31. * param.s1.powi(2)
                                        + -43. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2)))
                        - param.s12
                            * (param.s1 - param.s2).powi(3)
                            * (23. * param.s1 + 5. * param.s2)
                        - param.s12.powi(4) * (23. * param.s1 + 5. * param.s2))
                + 2. * param.m2_2.powi(3)
                    * param.s2.powi(2)
                    * (-5. * param.s12.powi(5)
                        + param.s12.powi(4) * (61. * param.s1 + 25. * param.s2)
                        + 2. * param.s12.powi(3)
                            * (23. * param.s1.powi(2)
                                + -52. * param.s1 * param.s2
                                + -25. * param.s2.powi(2))
                        + param.s12.powi(2)
                            * (-454. * param.s1.powi(3)
                                + 1074. * param.s1.powi(2) * param.s2
                                + -54. * param.s1 * param.s2.powi(2)
                                + 50. * param.s2.powi(3))
                        + param.s12
                            * (551. * param.s1.powi(4)
                                + -796. * param.s1.powi(3) * param.s2
                                + -666. * param.s1.powi(2) * param.s2.powi(2)
                                + 176. * param.s1 * param.s2.powi(3)
                                + -25. * param.s2.powi(4))
                        + 60.
                            * param.m1_2.powi(2)
                            * (-4. * param.s1.powi(3)
                                + 4. * param.s12.powi(3)
                                + -31. * param.s1.powi(2) * param.s2
                                + -31. * param.s1 * param.s2.powi(2)
                                + -4. * param.s2.powi(3)
                                + -12. * param.s12.powi(2) * (param.s1 + param.s2)
                                + param.s12
                                    * (12. * param.s1.powi(2)
                                        + 43. * param.s1 * param.s2
                                        + 12. * param.s2.powi(2)))
                        + -12.
                            * param.m1_2
                            * (-37. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 4. * param.s12.powi(3) * (7. * param.s1 + -3. * param.s2)
                                + -162. * param.s1.powi(3) * param.s2
                                + 88. * param.s1.powi(2) * param.s2.powi(2)
                                + 108. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + param.s12.powi(2)
                                    * (-102. * param.s1.powi(2)
                                        + 52. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (54. * param.s1.powi(3)
                                        + 61. * param.s1.powi(2) * param.s2
                                        + -94. * param.s1 * param.s2.powi(2)
                                        + -6. * param.s2.powi(3)))
                        - (param.s1 - param.s2).powi(2)
                            * (199. * param.s1.powi(3)
                                + 597. * param.s1.powi(2) * param.s2
                                + 69. * param.s1 * param.s2.powi(2)
                                + -5. * param.s2.powi(3)))
                + param.m2_2.powi(4)
                    * param.s2
                    * (-23. * param.s1.powi(5)
                        + 5. * param.s12.powi(5)
                        + -773. * param.s1.powi(4) * param.s2
                        + -398. * param.s1.powi(3) * param.s2.powi(2)
                        + 1102. * param.s1.powi(2) * param.s2.powi(3)
                        + 97. * param.s1 * param.s2.powi(4)
                        + -5. * param.s2.powi(5)
                        + 2. * param.s12.powi(3)
                            * (61. * param.s1.powi(2)
                                + 16. * param.s1 * param.s2
                                + 25. * param.s2.powi(2))
                        + -2.
                            * param.s12.powi(2)
                            * (79. * param.s1.powi(3)
                                + 381. * param.s1.powi(2) * param.s2
                                + -81. * param.s1 * param.s2.powi(2)
                                + 25. * param.s2.powi(3))
                        + param.s12
                            * (97. * param.s1.powi(4)
                                + 1528. * param.s1.powi(3) * param.s2
                                + -462. * param.s1.powi(2) * param.s2.powi(2)
                                + -248. * param.s1 * param.s2.powi(3)
                                + 25. * param.s2.powi(4))
                        + 6. * param.m1_2
                            * (3. * param.s1.powi(4)
                                + 3. * param.s12.powi(4)
                                + 148. * param.s1.powi(3) * param.s2
                                + 398. * param.s1.powi(2) * param.s2.powi(2)
                                + 148. * param.s1 * param.s2.powi(3)
                                + 3. * param.s2.powi(4)
                                + -12. * param.s12.powi(3) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + 86. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (3. * param.s1.powi(3)
                                        + 77. * param.s1.powi(2) * param.s2
                                        + 77. * param.s1 * param.s2.powi(2)
                                        + 3. * param.s2.powi(3)))
                        - param.s12.powi(4) * (43. * param.s1 + 25. * param.s2))
                + param.m2_2
                    * param.s2.powi(4)
                    * (-5. * param.s12.powi(5)
                        + 2100. * param.m1_2.powi(4) * (param.s12 - param.s2 - param.s1)
                        + param.s12.powi(4) * (97. * param.s1 + 25. * param.s2)
                        + 2. * param.s12.powi(3)
                            * (551. * param.s1.powi(2)
                                + -124. * param.s1 * param.s2
                                + -25. * param.s2.powi(2))
                        + -80.
                            * param.m1_2.powi(3)
                            * (-62. * param.s1.powi(2)
                                + 19. * param.s1 * param.s12
                                + 43. * param.s12.powi(2)
                                + 19. * param.s1 * param.s2
                                + -86. * param.s12 * param.s2
                                + 43. * param.s2.powi(2))
                        + param.s12.powi(2)
                            * (-398. * param.s1.powi(3)
                                + -462. * param.s1.powi(2) * param.s2
                                + 162. * param.s1 * param.s2.powi(2)
                                + 50. * param.s2.powi(3))
                        + 120.
                            * param.m1_2.powi(2)
                            * (12. * param.s12.powi(3)
                                + param.s12.powi(2) * (50. * param.s1 + -36. * param.s2)
                                + param.s12
                                    * (-31. * param.s1.powi(2)
                                        + -43. * param.s1 * param.s2
                                        + 36. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(2)
                                    * (31. * param.s1 + 12. * param.s2))
                        + -24.
                            * param.m1_2
                            * (3. * param.s12.powi(4)
                                + 12. * param.s12.powi(3) * (9. * param.s1 - param.s2)
                                + 2. * param.s12.powi(2)
                                    * (44. * param.s1.powi(2)
                                        + -94. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (81. * param.s1.powi(3)
                                        + -61. * param.s1.powi(2) * param.s2
                                        + -26. * param.s1 * param.s2.powi(2)
                                        + 6. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(3)
                                    * (37. * param.s1 + 3. * param.s2))
                        - param.s12
                            * (param.s1 - param.s2).powi(2)
                            * (773. * param.s1.powi(2)
                                + 18. * param.s1 * param.s2
                                + 25. * param.s2.powi(2))
                        - (23. * param.s1 + -5. * param.s2) * (param.s1 - param.s2).powi(4))
                + -2.
                    * param.m2_2.powi(2)
                    * param.s2.powi(3)
                    * (-5. * param.s12.powi(5)
                        + param.s12.powi(4) * (79. * param.s1 + 25. * param.s2)
                        + 2. * param.s12.powi(3)
                            * (227. * param.s1.powi(2)
                                + -88. * param.s1 * param.s2
                                + -25. * param.s2.powi(2))
                        + (param.s1 - param.s2).powi(3)
                            * (199. * param.s1.powi(2)
                                + 46. * param.s1 * param.s2
                                + -5. * param.s2.powi(2))
                        + param.s12.powi(2)
                            * (-926. * param.s1.powi(3)
                                + 666. * param.s1.powi(2) * param.s2
                                + 54. * param.s1 * param.s2.powi(2)
                                + 50. * param.s2.powi(3))
                        + param.s12
                            * (199. * param.s1.powi(4)
                                + 796. * param.s1.powi(3) * param.s2
                                + -1074. * param.s1.powi(2) * param.s2.powi(2)
                                + 104. * param.s1 * param.s2.powi(3)
                                + -25. * param.s2.powi(4))
                        + -20.
                            * param.m1_2.powi(3)
                            * (43. * param.s1.powi(2)
                                + 43. * param.s12.powi(2)
                                + 124. * param.s1 * param.s2
                                + 43. * param.s2.powi(2)
                                + -86. * param.s12 * (param.s1 + param.s2))
                        + 60.
                            * param.m1_2.powi(2)
                            * (31. * param.s1.powi(3)
                                + 12. * param.s12.powi(3)
                                + param.s12.powi(2) * (7. * param.s1 + -36. * param.s2)
                                + 31. * param.s1.powi(2) * param.s2
                                + -50. * param.s1 * param.s2.powi(2)
                                + -12. * param.s2.powi(3)
                                + param.s12
                                    * (-50. * param.s1.powi(2)
                                        + 43. * param.s1 * param.s2
                                        + 36. * param.s2.powi(2)))
                        + -6.
                            * param.m1_2
                            * (9. * param.s12.powi(4)
                                + 12. * param.s12.powi(3) * (17. * param.s1 + -3. * param.s2)
                                + (param.s1 - param.s2).powi(2)
                                    * (199. * param.s1.powi(2)
                                        + 222. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-236. * param.s1.powi(2)
                                        + -204. * param.s1 * param.s2
                                        + 54. * param.s2.powi(2))
                                + -4.
                                    * param.s12
                                    * (44. * param.s1.powi(3)
                                        + -199. * param.s1.powi(2) * param.s2
                                        + 51. * param.s1 * param.s2.powi(2)
                                        + 9. * param.s2.powi(3))))
                + param.m0_2.powi(4)
                    * (param.m2_2
                        * (-5. * param.s12.powi(5)
                            + (5. * param.s1 + -23. * param.s2) * (param.s1 - param.s2).powi(4)
                            + param.s12.powi(4) * (25. * param.s1 + 97. * param.s2)
                            + -2.
                                * param.s12.powi(3)
                                * (25. * param.s1.powi(2)
                                    + 124. * param.s1 * param.s2
                                    + -551. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (25. * param.s1.powi(3)
                                    + 81. * param.s1.powi(2) * param.s2
                                    + -231. * param.s1 * param.s2.powi(2)
                                    + -199. * param.s2.powi(3))
                            - param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (25. * param.s1.powi(2)
                                    + 18. * param.s1 * param.s2
                                    + 773. * param.s2.powi(2)))
                        + param.s2
                            * (-23. * param.s12.powi(5)
                                + param.s12.powi(4) * (97. * param.s1 + -773. * param.s2)
                                + 5. * (param.s1 - param.s2).powi(5)
                                + -2.
                                    * param.s12.powi(3)
                                    * (79. * param.s1.powi(2)
                                        + -764. * param.s1 * param.s2
                                        + 199. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (61. * param.s1.powi(3)
                                        + -381. * param.s1.powi(2) * param.s2
                                        + -231. * param.s1 * param.s2.powi(2)
                                        + 551. * param.s2.powi(3))
                                + 6. * param.m1_2
                                    * (3. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (3. * param.s1 + -37. * param.s2)
                                        + -4.
                                            * param.s12
                                            * (3. * param.s1 + -37. * param.s2)
                                            * (param.s1 - param.s2).powi(2)
                                        + 3. * (param.s1 - param.s2).powi(4)
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -154. * param.s1 * param.s2
                                                + 199. * param.s2.powi(2)))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (43. * param.s1 + 97. * param.s2)))
                + -2.
                    * param.m0_2.powi(2)
                    * (param.m2_2.powi(3)
                        * (5. * param.s12.powi(5)
                            + 2. * param.s12.powi(3)
                                * (25. * param.s1.powi(2)
                                    + 52. * param.s1 * param.s2
                                    + -23. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (-50. * param.s1.powi(3)
                                    + 54. * param.s1.powi(2) * param.s2
                                    + -1074. * param.s1 * param.s2.powi(2)
                                    + 454. * param.s2.powi(3))
                            + param.s12
                                * (25. * param.s1.powi(4)
                                    + -176. * param.s1.powi(3) * param.s2
                                    + 666. * param.s1.powi(2) * param.s2.powi(2)
                                    + 796. * param.s1 * param.s2.powi(3)
                                    + -551. * param.s2.powi(4))
                            - (param.s1 - param.s2).powi(2)
                                * (5. * param.s1.powi(3)
                                    + -69. * param.s1.powi(2) * param.s2
                                    + -597. * param.s1 * param.s2.powi(2)
                                    + -199. * param.s2.powi(3))
                            - param.s12.powi(4) * (25. * param.s1 + 61. * param.s2))
                        + param.s2.powi(3)
                            * (199. * param.s12.powi(5)
                                + param.s12.powi(4) * (199. * param.s1 + -551. * param.s2)
                                + -5. * (param.s1 - param.s2).powi(5)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (79. * param.s1 + 61. * param.s2)
                                + param.s12.powi(3)
                                    * (-926. * param.s1.powi(2)
                                        + 796. * param.s1 * param.s2
                                        + 454. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (227. * param.s1.powi(3)
                                        + 333. * param.s1.powi(2) * param.s2
                                        + -537. * param.s1 * param.s2.powi(2)
                                        + -23. * param.s2.powi(3))
                                + -20.
                                    * param.m1_2.powi(3)
                                    * (43. * param.s12.powi(2)
                                        + 43. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (-86. * param.s1 + 124. * param.s2))
                                + 60.
                                    * param.m1_2.powi(2)
                                    * (31. * param.s12.powi(3)
                                        + 12. * (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2)
                                            * (-50. * param.s1 + 31. * param.s2)
                                        + param.s12
                                            * (7. * param.s1.powi(2)
                                                + 43. * param.s1 * param.s2
                                                + -50. * param.s2.powi(2)))
                                + -6.
                                    * param.m1_2
                                    * (199. * param.s12.powi(4)
                                        + 9. * (param.s1 - param.s2).powi(4)
                                        + -176. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + 204.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + -4.
                                            * param.s12.powi(2)
                                            * (59. * param.s1.powi(2)
                                                + -199. * param.s1 * param.s2
                                                + 59. * param.s2.powi(2))))
                        + -3.
                            * param.m2_2.powi(2)
                            * param.s2
                            * (-9. * param.s12.powi(5)
                                + param.s12.powi(4) * (27. * param.s1 + -127. * param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (9. * param.s1.powi(2)
                                        + 154. * param.s1 * param.s2
                                        + -179. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (9. * param.s1.powi(3)
                                        + -435. * param.s1.powi(2) * param.s2
                                        + 341. * param.s1 * param.s2.powi(2)
                                        + 111. * param.s2.powi(3))
                                + param.s12
                                    * (27. * param.s1.powi(4)
                                        + -308. * param.s1.powi(3) * param.s2
                                        + -682. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1040. * param.s1 * param.s2.powi(3)
                                        + -77. * param.s2.powi(4))
                                + 2. * param.m1_2
                                    * (9. * param.s12.powi(4)
                                        + param.s12.powi(3)
                                            * (-36. * param.s1 + 204. * param.s2)
                                        + param.s12.powi(2)
                                            * (54. * param.s1.powi(2)
                                                + -204. * param.s1 * param.s2
                                                + -236. * param.s2.powi(2))
                                        + (param.s1 - param.s2).powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 222. * param.s1 * param.s2
                                                + 199. * param.s2.powi(2))
                                        + -4.
                                            * param.s12
                                            * (9. * param.s1.powi(3)
                                                + 51. * param.s1.powi(2) * param.s2
                                                + -199. * param.s1 * param.s2.powi(2)
                                                + 44. * param.s2.powi(3)))
                                - (param.s1 - param.s2).powi(3)
                                    * (9. * param.s1.powi(2)
                                        + 154. * param.s1 * param.s2
                                        + 77. * param.s2.powi(2)))
                        + 3. * param.m2_2
                            * param.s2.powi(2)
                            * (-77. * param.s12.powi(5)
                                + 77. * param.s12.powi(4) * (param.s1 + param.s2)
                                + 9. * (param.s1 - param.s2).powi(4) * (param.s1 + param.s2)
                                + 2. * param.s12.powi(3)
                                    * (111. * param.s1.powi(2)
                                        + -520. * param.s1 * param.s2
                                        + 111. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (127. * param.s1.powi(2)
                                        + 562. * param.s1 * param.s2
                                        + 127. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (-358. * param.s1.powi(3)
                                        + 682. * param.s1.powi(2) * param.s2
                                        + 682. * param.s1 * param.s2.powi(2)
                                        + -358. * param.s2.powi(3))
                                + -20.
                                    * param.m1_2.powi(2)
                                    * (12. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (-36. * param.s1 + 50. * param.s2)
                                        + param.s12
                                            * (36. * param.s1.powi(2)
                                                + -43. * param.s1 * param.s2
                                                + -31. * param.s2.powi(2))
                                        - (param.s1 - param.s2).powi(2)
                                            * (12. * param.s1 + 31. * param.s2))
                                + 4. * param.m1_2
                                    * (77. * param.s12.powi(4)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (94. * param.s1 + -61. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (51. * param.s1.powi(2)
                                                + 189. * param.s1 * param.s2
                                                + -199. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (26. * param.s1.powi(3)
                                                + -276. * param.s1.powi(2) * param.s2
                                                + 189. * param.s1 * param.s2.powi(2)
                                                + 61. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(3)
                                            * (43. * param.s1 + 77. * param.s2))))
                + 2. * param.m0_2.powi(3)
                    * (param.m2_2.powi(2)
                        * (5. * param.s12.powi(5)
                            + 2. * param.s12.powi(3)
                                * (25. * param.s1.powi(2)
                                    + 88. * param.s1 * param.s2
                                    + -227. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (25. * param.s1.powi(3)
                                    + 27. * param.s1.powi(2) * param.s2
                                    + 333. * param.s1 * param.s2.powi(2)
                                    + -463. * param.s2.powi(3))
                            + param.s12
                                * (25. * param.s1.powi(4)
                                    + -104. * param.s1.powi(3) * param.s2
                                    + 1074. * param.s1.powi(2) * param.s2.powi(2)
                                    + -796. * param.s1 * param.s2.powi(3)
                                    + -199. * param.s2.powi(4))
                            - (param.s1 - param.s2).powi(3)
                                * (5. * param.s1.powi(2)
                                    + -46. * param.s1 * param.s2
                                    + -199. * param.s2.powi(2))
                            - param.s12.powi(4) * (25. * param.s1 + 79. * param.s2))
                        + param.s2.powi(2)
                            * (-199. * param.s12.powi(5)
                                + param.s12.powi(4) * (551. * param.s1 + -199. * param.s2)
                                + -5. * (param.s1 - param.s2).powi(5)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (61. * param.s1 + 79. * param.s2)
                                + param.s12.powi(3)
                                    * (-454. * param.s1.powi(2)
                                        + -796. * param.s1 * param.s2
                                        + 926. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (23. * param.s1.powi(3)
                                        + 537. * param.s1.powi(2) * param.s2
                                        + -333. * param.s1 * param.s2.powi(2)
                                        + -227. * param.s2.powi(3))
                                + 12.
                                    * param.m1_2
                                    * (37. * param.s12.powi(4)
                                        + -54.
                                            * param.s12.powi(3)
                                            * (2. * param.s1 + -3. * param.s2)
                                        + -3. * (param.s1 - param.s2).powi(4)
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (7. * param.s1 + 27. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (51. * param.s1.powi(2)
                                                + -61. * param.s1 * param.s2
                                                + -44. * param.s2.powi(2)))
                                + -60.
                                    * param.m1_2.powi(2)
                                    * (4. * param.s12.powi(3)
                                        + -4. * (param.s1 - param.s2).powi(3)
                                        + param.s12.powi(2)
                                            * (-12. * param.s1 + 31. * param.s2)
                                        + param.s12
                                            * (12. * param.s1.powi(2)
                                                + -43. * param.s1 * param.s2
                                                + 31. * param.s2.powi(2))))
                        + 4. * param.m2_2
                            * param.s2
                            * (8. * param.s12.powi(5)
                                + (param.s1 - param.s2).powi(4) * (param.s1 + 8. * param.s2)
                                + param.s12.powi(4) * (-31. * param.s1 + 191. * param.s2)
                                + param.s12.powi(3)
                                    * (44. * param.s1.powi(2)
                                        + -169. * param.s1 * param.s2
                                        + -199. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (4. * param.s1.powi(2)
                                        + 213. * param.s1 * param.s2
                                        + 191. * param.s2.powi(2))
                                + -3.
                                    * param.m1_2
                                    * (3. * param.s12.powi(4)
                                        + -12.
                                            * param.s12.powi(3)
                                            * (param.s1 + -9. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (3. * param.s1 + 37. * param.s2)
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + -94. * param.s1 * param.s2
                                                + 44. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (6. * param.s1.powi(3)
                                                + -26. * param.s1.powi(2) * param.s2
                                                + -61. * param.s1 * param.s2.powi(2)
                                                + 81. * param.s2.powi(3)))
                                - param.s12.powi(2)
                                    * (26. * param.s1.powi(3)
                                        + 231. * param.s1.powi(2) * param.s2
                                        + -780. * param.s1 * param.s2.powi(2)
                                        + 199. * param.s2.powi(3))))
                + param.m0_2
                    * (param.m2_2.powi(4)
                        * (-5. * param.s1.powi(5)
                            + 5. * param.s12.powi(5)
                            + 97. * param.s1.powi(4) * param.s2
                            + 1102. * param.s1.powi(3) * param.s2.powi(2)
                            + -398. * param.s1.powi(2) * param.s2.powi(3)
                            + -773. * param.s1 * param.s2.powi(4)
                            + -23. * param.s2.powi(5)
                            + 2. * param.s12.powi(3)
                                * (25. * param.s1.powi(2)
                                    + 16. * param.s1 * param.s2
                                    + 61. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(2)
                                * (25. * param.s1.powi(3)
                                    + -81. * param.s1.powi(2) * param.s2
                                    + 381. * param.s1 * param.s2.powi(2)
                                    + 79. * param.s2.powi(3))
                            + param.s12
                                * (25. * param.s1.powi(4)
                                    + -248. * param.s1.powi(3) * param.s2
                                    + -462. * param.s1.powi(2) * param.s2.powi(2)
                                    + 1528. * param.s1 * param.s2.powi(3)
                                    + 97. * param.s2.powi(4))
                            - param.s12.powi(4) * (25. * param.s1 + 43. * param.s2))
                        + param.s2.powi(4)
                            * (-23. * param.s12.powi(5)
                                + -5. * (param.s1 - param.s2).powi(5)
                                + -2100.
                                    * param.m1_2.powi(4)
                                    * (param.s12 + param.s2 - param.s1)
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (97. * param.s1 + 43. * param.s2)
                                + param.s12.powi(4) * (-773. * param.s1 + 97. * param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (199. * param.s1.powi(2)
                                        + -764. * param.s1 * param.s2
                                        + 79. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (551. * param.s1.powi(3)
                                        + -231. * param.s1.powi(2) * param.s2
                                        + -381. * param.s1 * param.s2.powi(2)
                                        + 61. * param.s2.powi(3))
                                + 80.
                                    * param.m1_2.powi(3)
                                    * (62. * param.s12.powi(2)
                                        + -43. * (param.s1 - param.s2).powi(2)
                                        + -19. * param.s12 * (param.s1 + param.s2))
                                + 24.
                                    * param.m1_2
                                    * (37. * param.s12.powi(4)
                                        + 54.
                                            * param.s12.powi(3)
                                            * (3. * param.s1 + -2. * param.s2)
                                        + -3. * (param.s1 - param.s2).powi(4)
                                        + -4.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (27. * param.s1 + 7. * param.s2)
                                        + -2.
                                            * param.s12.powi(2)
                                            * (44. * param.s1.powi(2)
                                                + 61. * param.s1 * param.s2
                                                + -51. * param.s2.powi(2)))
                                + -120.
                                    * param.m1_2.powi(2)
                                    * (31. * param.s12.powi(3)
                                        + param.s12.powi(2)
                                            * (31. * param.s1 + -50. * param.s2)
                                        + -12. * (param.s1 - param.s2).powi(3)
                                        + param.s12
                                            * (-50. * param.s1.powi(2)
                                                + 43. * param.s1 * param.s2
                                                + 7. * param.s2.powi(2))))
                        + 8. * param.m2_2.powi(3)
                            * param.s2
                            * (param.s12.powi(5)
                                + 4. * param.s12.powi(4) * (param.s1 + param.s2)
                                + param.s12.powi(3)
                                    * (-26. * param.s1.powi(2)
                                        + 205. * param.s1 * param.s2
                                        + -26. * param.s2.powi(2))
                                + 11.
                                    * param.s12.powi(2)
                                    * (4. * param.s1.powi(3)
                                        + -21. * param.s1.powi(2) * param.s2
                                        + -21. * param.s1 * param.s2.powi(2)
                                        + 4. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(2)
                                    * (8. * param.s1.powi(3)
                                        + 207. * param.s1.powi(2) * param.s2
                                        + 207. * param.s1 * param.s2.powi(2)
                                        + 8. * param.s2.powi(3))
                                + -3.
                                    * param.m1_2
                                    * (3. * param.s1.powi(4)
                                        + 3. * param.s12.powi(4)
                                        + -4.
                                            * param.s12.powi(3)
                                            * (3. * param.s1 + -7. * param.s2)
                                        + 108. * param.s1.powi(3) * param.s2
                                        + 88. * param.s1.powi(2) * param.s2.powi(2)
                                        + -162. * param.s1 * param.s2.powi(3)
                                        + -37. * param.s2.powi(4)
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(2)
                                                + 26. * param.s1 * param.s2
                                                + -51. * param.s2.powi(2))
                                        + -2.
                                            * param.s12
                                            * (6. * param.s1.powi(3)
                                                + 94. * param.s1.powi(2) * param.s2
                                                + -61. * param.s1 * param.s2.powi(2)
                                                + -54. * param.s2.powi(3)))
                                - param.s12
                                    * (31. * param.s1.powi(4)
                                        + 169. * param.s1.powi(3) * param.s2
                                        + -780. * param.s1.powi(2) * param.s2.powi(2)
                                        + 169. * param.s1 * param.s2.powi(3)
                                        + 31. * param.s2.powi(4)))
                        + -8.
                            * param.m2_2
                            * param.s2.powi(3)
                            * (-8. * param.s12.powi(5)
                                + param.s12.powi(4) * (-191. * param.s1 + 31. * param.s2)
                                + 10.
                                    * param.m1_2.powi(3)
                                    * (43. * param.s1.powi(2)
                                        + -86. * param.s1 * param.s12
                                        + 43. * param.s12.powi(2)
                                        + 19. * param.s1 * param.s2
                                        + 19. * param.s12 * param.s2
                                        + -62. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (199. * param.s1.powi(2)
                                        + 169. * param.s1 * param.s2
                                        + -44. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (199. * param.s1.powi(3)
                                        + -780. * param.s1.powi(2) * param.s2
                                        + 231. * param.s1 * param.s2.powi(2)
                                        + 26. * param.s2.powi(3))
                                + -15.
                                    * param.m1_2.powi(2)
                                    * (43. * param.s12.powi(3)
                                        + -43. * param.s12.powi(2) * (param.s1 + param.s2)
                                        + 43.
                                            * (param.s1 - param.s2).powi(2)
                                            * (param.s1 + param.s2)
                                        + param.s12
                                            * (-43. * param.s1.powi(2)
                                                + 162. * param.s1 * param.s2
                                                + -43. * param.s2.powi(2)))
                                + 3. * param.m1_2
                                    * (77. * param.s12.powi(4)
                                        + 2. * param.s12.powi(3)
                                            * (61. * param.s1 + -94. * param.s2)
                                        + (param.s1 - param.s2).powi(3)
                                            * (77. * param.s1 + 43. * param.s2)
                                        + param.s12.powi(2)
                                            * (-398. * param.s1.powi(2)
                                                + 378. * param.s1 * param.s2
                                                + 102. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (61. * param.s1.powi(3)
                                                + 189. * param.s1.powi(2) * param.s2
                                                + -276. * param.s1 * param.s2.powi(2)
                                                + 26. * param.s2.powi(3)))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (191. * param.s1.powi(2)
                                        + 213. * param.s1 * param.s2
                                        + 4. * param.s2.powi(2))
                                - (param.s1 - param.s2).powi(4) * (8. * param.s1 + param.s2))
                        + -6.
                            * param.m2_2.powi(2)
                            * param.s2.powi(2)
                            * (9. * param.s12.powi(5)
                                + param.s12.powi(4) * (127. * param.s1 + -27. * param.s2)
                                + param.s12.powi(3)
                                    * (-358. * param.s1.powi(2)
                                        + 308. * param.s1 * param.s2
                                        + 18. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (111. * param.s1.powi(3)
                                        + 341. * param.s1.powi(2) * param.s2
                                        + -435. * param.s1 * param.s2.powi(2)
                                        + 9. * param.s2.powi(3))
                                + param.s12
                                    * (77. * param.s1.powi(4)
                                        + -1040. * param.s1.powi(3) * param.s2
                                        + 682. * param.s1.powi(2) * param.s2.powi(2)
                                        + 308. * param.s1 * param.s2.powi(3)
                                        + -27. * param.s2.powi(4))
                                + 20.
                                    * param.m1_2.powi(2)
                                    * (-12. * param.s1.powi(3)
                                        + 12. * param.s12.powi(3)
                                        + -50. * param.s1.powi(2) * param.s2
                                        + 31. * param.s1 * param.s2.powi(2)
                                        + 31. * param.s2.powi(3)
                                        + param.s12.powi(2) * (-36. * param.s1 + 7. * param.s2)
                                        + param.s12
                                            * (36. * param.s1.powi(2)
                                                + 43. * param.s1 * param.s2
                                                + -50. * param.s2.powi(2)))
                                + -4.
                                    * param.m1_2
                                    * (43. * param.s12.powi(4)
                                        + -52. * param.s12.powi(3) * (param.s1 + param.s2)
                                        + -6.
                                            * param.s12.powi(2)
                                            * (17. * param.s1.powi(2)
                                                + -92. * param.s1 * param.s2
                                                + 17. * param.s2.powi(2))
                                        + 2. * param.s12
                                            * (94. * param.s1.powi(3)
                                                + -189. * param.s1.powi(2) * param.s2
                                                + -189. * param.s1 * param.s2.powi(2)
                                                + 94. * param.s2.powi(3))
                                        - (param.s1 - param.s2).powi(2)
                                            * (77. * param.s1.powi(2)
                                                + 276. * param.s1 * param.s2
                                                + 77. * param.s2.powi(2)))
                                - (param.s1 - param.s2).powi(3)
                                    * (77. * param.s1.powi(2)
                                        + 154. * param.s1 * param.s2
                                        + 9. * param.s2.powi(2)))))
                * param.lambda_m02_sqrt
                * param.lambda_s12_sqrt
                + 60.
                    * param.s2.powi(2)
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
                    * ((14. * param.m1_2.powi(2)
                        + 3. * param.s12.powi(2)
                        + param.s12 * (8. * param.s1 + -6. * param.s2)
                        + 3. * (param.s1 - param.s2).powi(2)
                        + -14. * param.m1_2 * (param.s1 + param.s12 - param.s2))
                        * param.s2.powi(2)
                        + param.m2_2.powi(2)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + -2.
                            * param.m2_2
                            * param.s2
                            * (-4. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + param.s12 * (param.s1 + -6. * param.s2)
                                + param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + 7. * param.m1_2 * (param.s1 + param.s2 - param.s12))
                        + param.m0_2.powi(2)
                            * (3. * param.s12.powi(2)
                                + 3. * (param.s1 - param.s2).powi(2)
                                + param.s12 * (-6. * param.s1 + 8. * param.s2))
                        + -2.
                            * param.m0_2
                            * (param.m2_2
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -4. * param.s2.powi(2)
                                    + param.s12 * (-6. * param.s1 + param.s2))
                                + param.s2
                                    * (-4. * param.s12.powi(2)
                                        + 3. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2)
                                        + 7. * param.m1_2 * (param.s12 + param.s2 - param.s1))))
                    * log_diff(
                        (-2. * param.m1_2 + param.s1 + param.s12 - param.s2) * param.s2
                            + param.m2_2 * (param.s1 + param.s2 - param.s12)
                            + param.m0_2 * (param.s12 + param.s2 - param.s1),
                        param.lambda_m02_sqrt * param.lambda_s12_sqrt,
                    ))
    } else {
        0.0
    }) + (if param.s12 > (param.m1 + param.m2).powi(2) {
        0.0006944444444444444
            * std::f64::consts::PI
            * param.s12.powi(-4)
            * param.lambda_s12_sqrt.powi(-9)
            * ((3. * param.m2_2.powi(5) * param.s1.powi(7)
                + -27. * param.m2_2.powi(5) * param.s1.powi(6) * param.s12
                + -9. * param.m2_2.powi(4) * param.s1.powi(7) * param.s12
                + 114. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12.powi(2)
                + 87. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12.powi(2)
                + 6. * param.m2_2.powi(3) * param.s1.powi(7) * param.s12.powi(2)
                + -330. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(3)
                + -438. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(3)
                + -78. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(3)
                + 6. * param.m2_2.powi(2) * param.s1.powi(7) * param.s12.powi(3)
                + 345. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(4)
                + 780. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(4)
                + 237. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(4)
                + -18. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(4)
                + -9. * param.m2_2 * param.s1.powi(7) * param.s12.powi(4)
                + -33. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(5)
                + -435. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(5)
                + -285. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(5)
                + -3. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(5)
                + 57. * param.m2_2 * param.s1.powi(6) * param.s12.powi(5)
                + 3. * param.s1.powi(7) * param.s12.powi(5)
                + -78. * param.m2_2.powi(5) * param.s1 * param.s12.powi(6)
                + -111. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(6)
                + 120. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(6)
                + 75. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(6)
                + -153. * param.m2_2 * param.s1.powi(5) * param.s12.powi(6)
                + -21. * param.s1.powi(6) * param.s12.powi(6)
                + 6. * param.m2_2.powi(5) * param.s12.powi(7)
                + 138. * param.m2_2.powi(4) * param.s1 * param.s12.powi(7)
                + 24. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(7)
                + -120. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(7)
                + 225. * param.m2_2 * param.s1.powi(4) * param.s12.powi(7)
                + 63. * param.s1.powi(5) * param.s12.powi(7)
                + -12. * param.m2_2.powi(4) * param.s12.powi(8)
                + -27. * param.m2_2.powi(3) * param.s1 * param.s12.powi(8)
                + 84. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(8)
                + -195. * param.m2_2 * param.s1.powi(3) * param.s12.powi(8)
                + -105. * param.s1.powi(4) * param.s12.powi(8)
                + 3. * param.m2_2.powi(3) * param.s12.powi(9)
                + -27. * param.m2_2.powi(2) * param.s1 * param.s12.powi(9)
                + 99. * param.m2_2 * param.s1.powi(2) * param.s12.powi(9)
                + 105. * param.s1.powi(3) * param.s12.powi(9)
                + 3. * param.m2_2.powi(2) * param.s12.powi(10)
                + -27. * param.m2_2 * param.s1 * param.s12.powi(10)
                + -63. * param.s1.powi(2) * param.s12.powi(10)
                + 3. * param.m2_2 * param.s12.powi(11)
                + 21. * param.s1 * param.s12.powi(11)
                + -3. * param.s12.powi(12)
                + -21. * param.m2_2.powi(5) * param.s1.powi(6) * param.s2
                + 112. * param.m2_2.powi(5) * param.s1.powi(5) * param.s12 * param.s2
                + 57. * param.m2_2.powi(4) * param.s1.powi(6) * param.s12 * param.s2
                + -206. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12.powi(2) * param.s2
                + -296. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12.powi(2) * param.s2
                + -18. * param.m2_2.powi(3) * param.s1.powi(6) * param.s12.powi(2) * param.s2
                + -56. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12.powi(3) * param.s2
                + 334. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12.powi(3) * param.s2
                + -56. * param.m2_2.powi(3) * param.s1.powi(5) * param.s12.powi(3) * param.s2
                + -78. * param.m2_2.powi(2) * param.s1.powi(6) * param.s12.powi(3) * param.s2
                + -191. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12.powi(4) * param.s2
                + -1256. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12.powi(4) * param.s2
                + -1181. * param.m2_2.powi(3) * param.s1.powi(4) * param.s12.powi(4) * param.s2
                + -56. * param.m2_2.powi(2) * param.s1.powi(5) * param.s12.powi(4) * param.s2
                + 87. * param.m2_2 * param.s1.powi(6) * param.s12.powi(4) * param.s2
                + 280. * param.m2_2.powi(5) * param.s1 * param.s12.powi(5) * param.s2
                + 1459. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12.powi(5) * param.s2
                + 2464. * param.m2_2.powi(3) * param.s1.powi(3) * param.s12.powi(5) * param.s2
                + 559. * param.m2_2.powi(2) * param.s1.powi(4) * param.s12.powi(5) * param.s2
                + -296. * param.m2_2 * param.s1.powi(5) * param.s12.powi(5) * param.s2
                + -27. * param.s1.powi(6) * param.s12.powi(5) * param.s2
                + -38. * param.m2_2.powi(5) * param.s12.powi(6) * param.s2
                + -368. * param.m2_2.powi(4) * param.s1 * param.s12.powi(6) * param.s2
                + -1076. * param.m2_2.powi(3) * param.s1.powi(2) * param.s12.powi(6) * param.s2
                + -536. * param.m2_2.powi(2) * param.s1.powi(3) * param.s12.powi(6) * param.s2
                + 289. * param.m2_2 * param.s1.powi(4) * param.s12.powi(6) * param.s2
                + 112. * param.s1.powi(5) * param.s12.powi(6) * param.s2
                + 70. * param.m2_2.powi(4) * param.s12.powi(7) * param.s2
                + -128. * param.m2_2.powi(3) * param.s1 * param.s12.powi(7) * param.s2
                + -16. * param.m2_2.powi(2) * param.s1.powi(2) * param.s12.powi(7) * param.s2
                + 64. * param.m2_2 * param.s1.powi(3) * param.s12.powi(7) * param.s2
                + -155. * param.s1.powi(4) * param.s12.powi(7) * param.s2
                + -5. * param.m2_2.powi(3) * param.s12.powi(8) * param.s2
                + 152. * param.m2_2.powi(2) * param.s1 * param.s12.powi(8) * param.s2
                + -271. * param.m2_2 * param.s1.powi(2) * param.s12.powi(8) * param.s2
                + 40. * param.s1.powi(3) * param.s12.powi(8) * param.s2
                + -25. * param.m2_2.powi(2) * param.s12.powi(9) * param.s2
                + 152. * param.m2_2 * param.s1 * param.s12.powi(9) * param.s2
                + 95. * param.s1.powi(2) * param.s12.powi(9) * param.s2
                + -25. * param.m2_2 * param.s12.powi(10) * param.s2
                + -88. * param.s1 * param.s12.powi(10) * param.s2
                + 23. * param.s12.powi(11) * param.s2
                + 63. * param.m2_2.powi(5) * param.s1.powi(5) * param.s2.powi(2)
                + -155. * param.m2_2.powi(5) * param.s1.powi(4) * param.s12 * param.s2.powi(2)
                + -153. * param.m2_2.powi(4) * param.s1.powi(5) * param.s12 * param.s2.powi(2)
                + 10.
                    * param.m2_2.powi(5)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 289.
                    * param.m2_2.powi(4)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + -3.
                    * param.m2_2.powi(3)
                    * param.s1.powi(5)
                    * param.s12.powi(2)
                    * param.s2.powi(2)
                + 126.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 490.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 559.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + 237.
                    * param.m2_2.powi(2)
                    * param.s1.powi(5)
                    * param.s12.powi(3)
                    * param.s2.powi(2)
                + -347. * param.m2_2.powi(5) * param.s1 * param.s12.powi(4) * param.s2.powi(2)
                + -1038.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -1040.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -1181.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(4)
                    * param.s2.powi(2)
                + -438. * param.m2_2 * param.s1.powi(5) * param.s12.powi(4) * param.s2.powi(2)
                + 103. * param.m2_2.powi(5) * param.s12.powi(5) * param.s2.powi(2)
                + 205. * param.m2_2.powi(4) * param.s1 * param.s12.powi(5) * param.s2.powi(2)
                + -1008.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + -1040.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(5)
                    * param.s2.powi(2)
                + 334. * param.m2_2 * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(2)
                + 114. * param.s1.powi(5) * param.s12.powi(5) * param.s2.powi(2)
                + -173. * param.m2_2.powi(4) * param.s12.powi(6) * param.s2.powi(2)
                + 475. * param.m2_2.powi(3) * param.s1 * param.s12.powi(6) * param.s2.powi(2)
                + 1992.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(6)
                    * param.s2.powi(2)
                + 490. * param.m2_2 * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(2)
                + -206. * param.s1.powi(4) * param.s12.powi(6) * param.s2.powi(2)
                + -23. * param.m2_2.powi(3) * param.s12.powi(7) * param.s2.powi(2)
                + -85. * param.m2_2.powi(2) * param.s1 * param.s12.powi(7) * param.s2.powi(2)
                + -138. * param.m2_2 * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(2)
                + 10. * param.s1.powi(3) * param.s12.powi(7) * param.s2.powi(2)
                + 77. * param.m2_2.powi(2) * param.s12.powi(8) * param.s2.powi(2)
                + -340. * param.m2_2 * param.s1 * param.s12.powi(8) * param.s2.powi(2)
                + 66. * param.s1.powi(2) * param.s12.powi(8) * param.s2.powi(2)
                + 92. * param.m2_2 * param.s12.powi(9) * param.s2.powi(2)
                + 92. * param.s1 * param.s12.powi(9) * param.s2.powi(2)
                + -76. * param.s12.powi(10) * param.s2.powi(2)
                + -105. * param.m2_2.powi(5) * param.s1.powi(4) * param.s2.powi(3)
                + 40. * param.m2_2.powi(5) * param.s1.powi(3) * param.s12 * param.s2.powi(3)
                + 225. * param.m2_2.powi(4) * param.s1.powi(4) * param.s12 * param.s2.powi(3)
                + 66.
                    * param.m2_2.powi(5)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 64.
                    * param.m2_2.powi(4)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 75.
                    * param.m2_2.powi(3)
                    * param.s1.powi(4)
                    * param.s12.powi(2)
                    * param.s2.powi(3)
                + 120. * param.m2_2.powi(5) * param.s1 * param.s12.powi(3) * param.s2.powi(3)
                + -138.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -536.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -285.
                    * param.m2_2.powi(2)
                    * param.s1.powi(4)
                    * param.s12.powi(3)
                    * param.s2.powi(3)
                + -155. * param.m2_2.powi(5) * param.s12.powi(4) * param.s2.powi(3)
                + 240. * param.m2_2.powi(4) * param.s1 * param.s12.powi(4) * param.s2.powi(3)
                + 1992.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 2464.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(4)
                    * param.s2.powi(3)
                + 780. * param.m2_2 * param.s1.powi(4) * param.s12.powi(4) * param.s2.powi(3)
                + 235. * param.m2_2.powi(4) * param.s12.powi(5) * param.s2.powi(3)
                + -360. * param.m2_2.powi(3) * param.s1 * param.s12.powi(5) * param.s2.powi(3)
                + -1008.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(5)
                    * param.s2.powi(3)
                + -1256. * param.m2_2 * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(3)
                + -330. * param.s1.powi(4) * param.s12.powi(5) * param.s2.powi(3)
                + 85. * param.m2_2.powi(3) * param.s12.powi(6) * param.s2.powi(3)
                + -360. * param.m2_2.powi(2) * param.s1 * param.s12.powi(6) * param.s2.powi(3)
                + -1038. * param.m2_2 * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(3)
                + -56. * param.s1.powi(3) * param.s12.powi(6) * param.s2.powi(3)
                + -115. * param.m2_2.powi(2) * param.s12.powi(7) * param.s2.powi(3)
                + 240. * param.m2_2 * param.s1 * param.s12.powi(7) * param.s2.powi(3)
                + 126. * param.s1.powi(2) * param.s12.powi(7) * param.s2.powi(3)
                + -190. * param.m2_2 * param.s12.powi(8) * param.s2.powi(3)
                + 120. * param.s1 * param.s12.powi(8) * param.s2.powi(3)
                + 140. * param.s12.powi(9) * param.s2.powi(3)
                + 105. * param.m2_2.powi(5) * param.s1.powi(3) * param.s2.powi(4)
                + 95. * param.m2_2.powi(5) * param.s1.powi(2) * param.s12 * param.s2.powi(4)
                + -195. * param.m2_2.powi(4) * param.s1.powi(3) * param.s12 * param.s2.powi(4)
                + 92. * param.m2_2.powi(5) * param.s1 * param.s12.powi(2) * param.s2.powi(4)
                + -271.
                    * param.m2_2.powi(4)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + -120.
                    * param.m2_2.powi(3)
                    * param.s1.powi(3)
                    * param.s12.powi(2)
                    * param.s2.powi(4)
                + 140. * param.m2_2.powi(5) * param.s12.powi(3) * param.s2.powi(4)
                + -340. * param.m2_2.powi(4) * param.s1 * param.s12.powi(3) * param.s2.powi(4)
                + -16.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + 120.
                    * param.m2_2.powi(2)
                    * param.s1.powi(3)
                    * param.s12.powi(3)
                    * param.s2.powi(4)
                + -190. * param.m2_2.powi(4) * param.s12.powi(4) * param.s2.powi(4)
                + -85. * param.m2_2.powi(3) * param.s1 * param.s12.powi(4) * param.s2.powi(4)
                + -1076.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(4)
                    * param.s2.powi(4)
                + -435. * param.m2_2 * param.s1.powi(3) * param.s12.powi(4) * param.s2.powi(4)
                + -115. * param.m2_2.powi(3) * param.s12.powi(5) * param.s2.powi(4)
                + 475. * param.m2_2.powi(2) * param.s1 * param.s12.powi(5) * param.s2.powi(4)
                + 1459. * param.m2_2 * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(4)
                + 345. * param.s1.powi(3) * param.s12.powi(5) * param.s2.powi(4)
                + 85. * param.m2_2.powi(2) * param.s12.powi(6) * param.s2.powi(4)
                + 205. * param.m2_2 * param.s1 * param.s12.powi(6) * param.s2.powi(4)
                + -191. * param.s1.powi(2) * param.s12.powi(6) * param.s2.powi(4)
                + 235. * param.m2_2 * param.s12.powi(7) * param.s2.powi(4)
                + -347. * param.s1 * param.s12.powi(7) * param.s2.powi(4)
                + -155. * param.s12.powi(8) * param.s2.powi(4)
                + -63. * param.m2_2.powi(5) * param.s1.powi(2) * param.s2.powi(5)
                + -88. * param.m2_2.powi(5) * param.s1 * param.s12 * param.s2.powi(5)
                + 99. * param.m2_2.powi(4) * param.s1.powi(2) * param.s12 * param.s2.powi(5)
                + -76. * param.m2_2.powi(5) * param.s12.powi(2) * param.s2.powi(5)
                + 152. * param.m2_2.powi(4) * param.s1 * param.s12.powi(2) * param.s2.powi(5)
                + 84.
                    * param.m2_2.powi(3)
                    * param.s1.powi(2)
                    * param.s12.powi(2)
                    * param.s2.powi(5)
                + 92. * param.m2_2.powi(4) * param.s12.powi(3) * param.s2.powi(5)
                + 152. * param.m2_2.powi(3) * param.s1 * param.s12.powi(3) * param.s2.powi(5)
                + 24.
                    * param.m2_2.powi(2)
                    * param.s1.powi(2)
                    * param.s12.powi(3)
                    * param.s2.powi(5)
                + 77. * param.m2_2.powi(3) * param.s12.powi(4) * param.s2.powi(5)
                + -128. * param.m2_2.powi(2) * param.s1 * param.s12.powi(4) * param.s2.powi(5)
                + -111. * param.m2_2 * param.s1.powi(2) * param.s12.powi(4) * param.s2.powi(5)
                + -23. * param.m2_2.powi(2) * param.s12.powi(5) * param.s2.powi(5)
                + -368. * param.m2_2 * param.s1 * param.s12.powi(5) * param.s2.powi(5)
                + -33. * param.s1.powi(2) * param.s12.powi(5) * param.s2.powi(5)
                + -173. * param.m2_2 * param.s12.powi(6) * param.s2.powi(5)
                + 280. * param.s1 * param.s12.powi(6) * param.s2.powi(5)
                + 103. * param.s12.powi(7) * param.s2.powi(5)
                + 21. * param.m2_2.powi(5) * param.s1 * param.s2.powi(6)
                + 23. * param.m2_2.powi(5) * param.s12 * param.s2.powi(6)
                + -27. * param.m2_2.powi(4) * param.s1 * param.s12 * param.s2.powi(6)
                + -25. * param.m2_2.powi(4) * param.s12.powi(2) * param.s2.powi(6)
                + -27. * param.m2_2.powi(3) * param.s1 * param.s12.powi(2) * param.s2.powi(6)
                + -25. * param.m2_2.powi(3) * param.s12.powi(3) * param.s2.powi(6)
                + -27. * param.m2_2.powi(2) * param.s1 * param.s12.powi(3) * param.s2.powi(6)
                + -5. * param.m2_2.powi(2) * param.s12.powi(4) * param.s2.powi(6)
                + 138. * param.m2_2 * param.s1 * param.s12.powi(4) * param.s2.powi(6)
                + 70. * param.m2_2 * param.s12.powi(5) * param.s2.powi(6)
                + -78. * param.s1 * param.s12.powi(5) * param.s2.powi(6)
                + -38. * param.s12.powi(6) * param.s2.powi(6)
                + -3. * param.m2_2.powi(5) * param.s2.powi(7)
                + 3. * param.m2_2.powi(4) * param.s12 * param.s2.powi(7)
                + 3. * param.m2_2.powi(3) * param.s12.powi(2) * param.s2.powi(7)
                + 3. * param.m2_2.powi(2) * param.s12.powi(3) * param.s2.powi(7)
                + -12. * param.m2_2 * param.s12.powi(4) * param.s2.powi(7)
                + 6. * param.s12.powi(5) * param.s2.powi(7)
                + 60.
                    * param.m0_2.powi(5)
                    * param.s12.powi(5)
                    * (3. * param.s12.powi(2)
                        + 3. * (param.s1 - param.s2).powi(2)
                        + param.s12 * (-6. * param.s1 + 8. * param.s2))
                + param.m1_2.powi(5)
                    * (3. * param.s12.powi(7)
                        + -3. * (param.s1 - param.s2).powi(7)
                        + param.s12
                            * (param.s1 - param.s2).powi(5)
                            * (21. * param.s1 + 29. * param.s2)
                        + param.s12.powi(5)
                            * (63. * param.s1.powi(2)
                                + 124. * param.s1 * param.s2
                                + 139. * param.s2.powi(2))
                        + param.s12.powi(3)
                            * (105. * param.s1.powi(4)
                                + 80. * param.s1.powi(3) * param.s2
                                + 108. * param.s1.powi(2) * param.s2.powi(2)
                                + 240. * param.s1 * param.s2.powi(3)
                                + -533. * param.s2.powi(4))
                        - param.s12.powi(4)
                            * (105. * param.s1.powi(3)
                                + 185. * param.s1.powi(2) * param.s2
                                + 293. * param.s1 * param.s2.powi(2)
                                + 533. * param.s2.powi(3))
                        - param.s12.powi(2)
                            * (param.s1 - param.s2).powi(3)
                            * (63. * param.s1.powi(2)
                                + 124. * param.s1 * param.s2
                                + 139. * param.s2.powi(2))
                        - param.s12.powi(6) * (21. * param.s1 + 29. * param.s2))
                + -30.
                    * param.m0_2.powi(4)
                    * param.s12.powi(4)
                    * (param.m2_2
                        * (21. * param.s12.powi(3)
                            + 9. * (param.s1 - param.s2).powi(3)
                            + param.s12.powi(2) * (-33. * param.s1 + 19. * param.s2)
                            + param.s12
                                * (3. * param.s1.powi(2)
                                    + 28. * param.s1 * param.s2
                                    + -31. * param.s2.powi(2)))
                        + param.s12
                            * (-9. * param.s12.powi(3)
                                + param.s12.powi(2) * (27. * param.s1 + -31. * param.s2)
                                + 3. * (param.s1 - param.s2).powi(2)
                                    * (3. * param.s1 + 7. * param.s2)
                                + param.s12
                                    * (-27. * param.s1.powi(2)
                                        + 28. * param.s1 * param.s2
                                        + 19. * param.s2.powi(2)))
                        + param.m1_2
                            * (9. * param.s12.powi(3)
                                + -9. * (param.s1 - param.s2).powi(3)
                                + param.s12.powi(2) * (-27. * param.s1 + 61. * param.s2)
                                + param.s12
                                    * (27. * param.s1.powi(2)
                                        + -88. * param.s1 * param.s2
                                        + 61. * param.s2.powi(2))))
                + param.m1_2.powi(3)
                    * (param.m2_2.powi(2)
                        * (69. * param.s12.powi(7)
                            + -30. * (param.s1 - param.s2).powi(7)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(5)
                                * (117. * param.s1 + 133. * param.s2)
                            + 4. * param.s12.powi(5)
                                * (285. * param.s1.powi(2)
                                    + 826. * param.s1 * param.s2
                                    + 58. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(4)
                                * (420. * param.s1.powi(3)
                                    + 989. * param.s1.powi(2) * param.s2
                                    + 605. * param.s1 * param.s2.powi(2)
                                    + -688. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (1485. * param.s1.powi(4)
                                    + 1304. * param.s1.powi(3) * param.s2
                                    + 1134. * param.s1.powi(2) * param.s2.powi(2)
                                    + -960. * param.s1 * param.s2.powi(3)
                                    + -2963. * param.s2.powi(4))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(3)
                                * (789. * param.s1.powi(2)
                                    + 1378. * param.s1 * param.s2
                                    + 1093. * param.s2.powi(2))
                            - param.s12.powi(6) * (429. * param.s1 + 947. * param.s2))
                        + 4. * param.m2_2
                            * param.s12
                            * (15. * param.s12.powi(7)
                                + -3.
                                    * (3. * param.s1 + -5. * param.s2)
                                    * (param.s1 - param.s2).powi(6)
                                + param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (69. * param.s1.powi(2)
                                        + -34. * param.s1 * param.s2
                                        + -149. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (279. * param.s1.powi(2)
                                        + 562. * param.s1 * param.s2
                                        + 787. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (405. * param.s1.powi(4)
                                        + 116. * param.s1.powi(3) * param.s2
                                        + -324. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1920. * param.s1 * param.s2.powi(3)
                                        + -653. * param.s2.powi(4))
                                - param.s12.powi(4)
                                    * (435. * param.s1.powi(3)
                                        + 689. * param.s1.powi(2) * param.s2
                                        + 1043. * param.s1 * param.s2.powi(2)
                                        + 653. * param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (225. * param.s1.powi(3)
                                        + 49. * param.s1.powi(2) * param.s2
                                        + -531. * param.s1 * param.s2.powi(2)
                                        + -787. * param.s2.powi(3))
                                - param.s12.powi(6) * (99. * param.s1 + 149. * param.s2))
                        + param.s12.powi(2)
                            * (30. * param.s12.powi(7)
                                + -14. * param.s12.powi(6) * (15. * param.s1 + 19. * param.s2)
                                + -3.
                                    * (param.s1 - param.s2).powi(5)
                                    * (10. * param.s1.powi(2)
                                        + -28. * param.s1 * param.s2
                                        + 23. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (630. * param.s1.powi(2)
                                        + 1096. * param.s1 * param.s2
                                        + 1093. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (210. * param.s1.powi(3)
                                        + -274. * param.s1.powi(2) * param.s2
                                        + -463. * param.s1 * param.s2.powi(2)
                                        + 947. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (525. * param.s1.powi(4)
                                        + 160. * param.s1.powi(3) * param.s2
                                        + -33. * param.s1.powi(2) * param.s2.powi(2)
                                        + -480. * param.s1 * param.s2.powi(3)
                                        + 1376. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-630. * param.s1.powi(5)
                                        + 1010. * param.s1.powi(4) * param.s2
                                        + 674. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1134. * param.s1.powi(2) * param.s2.powi(3)
                                        + -2420. * param.s1 * param.s2.powi(4)
                                        + 232. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (1050. * param.s1.powi(3)
                                        + 1490. * param.s1.powi(2) * param.s2
                                        + 1901. * param.s1 * param.s2.powi(2)
                                        + 2963. * param.s2.powi(3))))
                + 20.
                    * param.m0_2.powi(3)
                    * param.s12.powi(3)
                    * (param.m2_2.powi(2)
                        * (39. * param.s12.powi(4)
                            + 3. * (param.s1 - param.s2).powi(4)
                            + -2. * param.s12.powi(3) * (15. * param.s1 + 17. * param.s2)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(2)
                                * (21. * param.s1 + 19. * param.s2)
                            + -2.
                                * param.s12.powi(2)
                                * (27. * param.s1.powi(2)
                                    + -76. * param.s1 * param.s2
                                    + 23. * param.s2.powi(2)))
                        + param.m1_2.powi(2)
                            * (3. * param.s12.powi(4)
                                + -4.
                                    * param.s12
                                    * (3. * param.s1 + -23. * param.s2)
                                    * (param.s1 - param.s2).powi(2)
                                + 3. * (param.s1 - param.s2).powi(4)
                                + param.s12.powi(3) * (-12. * param.s1 + 92. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + -98. * param.s1 * param.s2
                                        + 115. * param.s2.powi(2)))
                        + param.s12.powi(2)
                            * (3. * param.s12.powi(4)
                                + param.s12.powi(3) * (-12. * param.s1 + 38. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (9. * param.s1.powi(2)
                                        + -17. * param.s1 * param.s2
                                        + -23. * param.s2.powi(2))
                                + 3. * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(2)
                                        + 16. * param.s1 * param.s2
                                        + 13. * param.s2.powi(2))
                                + -2.
                                    * param.s12
                                    * (6. * param.s1.powi(3)
                                        + 23. * param.s1.powi(2) * param.s2
                                        + -76. * param.s1 * param.s2.powi(2)
                                        + 17. * param.s2.powi(3)))
                        + 2. * param.m1_2
                            * (param.m2_2
                                * (24. * param.s12.powi(4)
                                    + -3. * (param.s1 - param.s2).powi(4)
                                    + -5.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (3. * param.s1 + 13. * param.s2)
                                    + param.s12.powi(3) * (-69. * param.s1 + 91. * param.s2)
                                    + param.s12.powi(2)
                                        * (63. * param.s1.powi(2)
                                            + -68. * param.s1 * param.s2
                                            + -47. * param.s2.powi(2)))
                                - param.s12
                                    * (3. * param.s12.powi(4)
                                        + 3. * (param.s1 - param.s2).powi(3)
                                            * (param.s1 + 8. * param.s2)
                                        + param.s12.powi(3)
                                            * (-12. * param.s1 + 65. * param.s2)
                                        + param.s12.powi(2)
                                            * (18. * param.s1.powi(2)
                                                + -115. * param.s1 * param.s2
                                                + 47. * param.s2.powi(2))
                                        + param.s12
                                            * (-12. * param.s1.powi(3)
                                                + 35. * param.s1.powi(2) * param.s2
                                                + 68. * param.s1 * param.s2.powi(2)
                                                + -91. * param.s2.powi(3))))
                        - param.m2_2
                            * param.s12
                            * (33. * param.s12.powi(4)
                                + -3.
                                    * (param.s1 - param.s2).powi(3)
                                    * (7. * param.s1 + 11. * param.s2)
                                + param.s12.powi(3) * (-78. * param.s1 + 40. * param.s2)
                                + 2. * param.s12.powi(2)
                                    * (18. * param.s1.powi(2)
                                        + 77. * param.s1 * param.s2
                                        + -73. * param.s2.powi(2))
                                + 2. * param.s12
                                    * (15. * param.s1.powi(3)
                                        + -112. * param.s1.powi(2) * param.s2
                                        + 77. * param.s1 * param.s2.powi(2)
                                        + 20. * param.s2.powi(3))))
                + -15.
                    * param.m0_2.powi(2)
                    * param.s12.powi(2)
                    * (param.m2_2.powi(3)
                        * (25. * param.s12.powi(5)
                            + param.s12.powi(4) * (31. * param.s1 + -67. * param.s2)
                            + param.s12
                                * (param.s1 - param.s2).powi(3)
                                * (17. * param.s1 + 11. * param.s2)
                            + param.s12.powi(3)
                                * (-122. * param.s1.powi(2)
                                    + 96. * param.s1 * param.s2
                                    + 50. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (25. * param.s1.powi(3)
                                    + 43. * param.s1.powi(2) * param.s2
                                    + -69. * param.s1 * param.s2.powi(2)
                                    + param.s2.powi(3))
                            - (param.s1 - param.s2).powi(5))
                        + param.m2_2.powi(2)
                            * param.s12
                            * (-31. * param.s12.powi(5)
                                + (param.s1 - param.s2).powi(4)
                                    * (7. * param.s1 + 5. * param.s2)
                                + param.s12.powi(4) * (23. * param.s1 + 33. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (55. * param.s1.powi(2)
                                        + -196. * param.s1 * param.s2
                                        + 41. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (49. * param.s1.powi(2)
                                        + 226. * param.s1 * param.s2
                                        + 45. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (79. * param.s1.powi(3)
                                        + -127. * param.s1.powi(2) * param.s2
                                        + -123. * param.s1 * param.s2.powi(2)
                                        + 67. * param.s2.powi(3)))
                        + param.m1_2.powi(3)
                            * ((param.s1 - param.s2).powi(5)
                                + param.s12.powi(4) * (5. * param.s1 + 23. * param.s2)
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 32. * param.s1 * param.s2
                                        + -129. * param.s2.powi(2))
                                + 2. * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + 27. * param.s1.powi(2) * param.s2
                                        + -161. * param.s1 * param.s2.powi(2)
                                        + 129. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1 + 23. * param.s2)
                                - param.s12.powi(5))
                        + param.s12.powi(3)
                            * (param.s12.powi(5)
                                + 2. * param.s12.powi(3)
                                    * (5. * param.s1.powi(2)
                                        + 8. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(2)
                                    * (5. * param.s1.powi(3)
                                        + -9. * param.s1.powi(2) * param.s2
                                        + 69. * param.s1 * param.s2.powi(2)
                                        + -25. * param.s2.powi(3))
                                + param.s12
                                    * (5. * param.s1.powi(4)
                                        + -40. * param.s1.powi(3) * param.s2
                                        + 86. * param.s1.powi(2) * param.s2.powi(2)
                                        + 96. * param.s1 * param.s2.powi(3)
                                        + -67. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + -15. * param.s1.powi(2) * param.s2
                                        + -81. * param.s1 * param.s2.powi(2)
                                        + -25. * param.s2.powi(3))
                                - param.s12.powi(4) * (5. * param.s1 + 11. * param.s2))
                        + param.m2_2
                            * param.s12.powi(2)
                            * (5. * param.s12.powi(5)
                                + param.s12.powi(4) * (-13. * param.s1 + 45. * param.s2)
                                + 2. * param.s12.powi(3)
                                    * (param.s1.powi(2)
                                        + 68. * param.s1 * param.s2
                                        + -67. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(3)
                                    * (7. * param.s1.powi(2)
                                        + 70. * param.s1 * param.s2
                                        + 31. * param.s2.powi(2))
                                + param.s12.powi(2)
                                    * (22. * param.s1.powi(3)
                                        + -358. * param.s1.powi(2) * param.s2
                                        + 246. * param.s1 * param.s2.powi(2)
                                        + 82. * param.s2.powi(3))
                                + param.s12
                                    * (-23. * param.s1.powi(4)
                                        + 128. * param.s1.powi(3) * param.s2
                                        + 254. * param.s1.powi(2) * param.s2.powi(2)
                                        + -392. * param.s1 * param.s2.powi(3)
                                        + 33. * param.s2.powi(4)))
                        + param.m1_2.powi(2)
                            * (param.m2_2
                                * (15. * param.s12.powi(5)
                                    + -3. * (param.s1 - param.s2).powi(5)
                                    + 3. * param.s12
                                        * (param.s1 - param.s2).powi(3)
                                        * (9. * param.s1 + 19. * param.s2)
                                    + param.s12.powi(4) * (-63. * param.s1 + 299. * param.s2)
                                    + 2. * param.s12.powi(3)
                                        * (51. * param.s1.powi(2)
                                            + -296. * param.s1 * param.s2
                                            + 73. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (-78. * param.s1.powi(3)
                                            + 302. * param.s1.powi(2) * param.s2
                                            + 182. * param.s1 * param.s2.powi(2)
                                            + -406. * param.s2.powi(3)))
                                + param.s12
                                    * (3. * param.s12.powi(5)
                                        + -3.
                                            * (param.s1 + -5. * param.s2)
                                            * (param.s1 - param.s2).powi(4)
                                        + -3.
                                            * param.s12.powi(4)
                                            * (5. * param.s1 + 19. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (15. * param.s1.powi(2)
                                                + 72. * param.s1 * param.s2
                                                + -203. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (15. * param.s1.powi(2)
                                                + 6. * param.s1 * param.s2
                                                + 299. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (15. * param.s1.powi(3)
                                                + 45. * param.s1.powi(2) * param.s2
                                                + -91. * param.s1 * param.s2.powi(2)
                                                + -73. * param.s2.powi(3))))
                        + param.m1_2
                            * (-4.
                                * param.m2_2
                                * param.s12
                                * (5. * param.s12.powi(5)
                                    + (param.s1 - param.s2).powi(4)
                                        * (param.s1 + 5. * param.s2)
                                    + param.s12.powi(4) * (-19. * param.s1 + 71. * param.s2)
                                    + param.s12.powi(3)
                                        * (26. * param.s1.powi(2)
                                            + -54. * param.s1 * param.s2
                                            + -76. * param.s2.powi(2))
                                    + param.s12
                                        * (param.s1 - param.s2).powi(2)
                                        * (param.s1.powi(2)
                                            + 88. * param.s1 * param.s2
                                            + 71. * param.s2.powi(2))
                                    + -2.
                                        * param.s12.powi(2)
                                        * (7. * param.s1.powi(3)
                                            + 52. * param.s1.powi(2) * param.s2
                                            + -149. * param.s1 * param.s2.powi(2)
                                            + 38. * param.s2.powi(3)))
                                + param.m2_2.powi(2)
                                    * (81. * param.s12.powi(5)
                                        + 3. * (param.s1 - param.s2).powi(5)
                                        + -3.
                                            * param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (13. * param.s1 + 15. * param.s2)
                                        + param.s12.powi(4)
                                            * (-213. * param.s1 + 65. * param.s2)
                                        + 2. * param.s12.powi(3)
                                            * (75. * param.s1.powi(2)
                                                + 160. * param.s1 * param.s2
                                                + -167. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (9. * param.s1.powi(3)
                                                + -221. * param.s1.powi(2) * param.s2
                                                + 139. * param.s1 * param.s2.powi(2)
                                                + 73. * param.s2.powi(3)))
                                + param.s12.powi(2)
                                    * (-3. * param.s12.powi(5)
                                        + 15. * param.s12.powi(4) * (param.s1 + 3. * param.s2)
                                        + -2.
                                            * param.s12.powi(3)
                                            * (15. * param.s1.powi(2)
                                                + 48. * param.s1 * param.s2
                                                + -73. * param.s2.powi(2))
                                        + 3. * (param.s1 - param.s2).powi(3)
                                            * (param.s1.powi(2)
                                                + -10. * param.s1 * param.s2
                                                + -27. * param.s2.powi(2))
                                        + 2. * param.s12.powi(2)
                                            * (15. * param.s1.powi(3)
                                                + 9. * param.s1.powi(2) * param.s2
                                                + 139. * param.s1 * param.s2.powi(2)
                                                + -167. * param.s2.powi(3))
                                        + param.s12
                                            * (-15. * param.s1.powi(4)
                                                + 72. * param.s1.powi(3) * param.s2
                                                + -442. * param.s1.powi(2) * param.s2.powi(2)
                                                + 320. * param.s1 * param.s2.powi(3)
                                                + 65. * param.s2.powi(4)))))
                + 6. * param.m0_2
                    * param.s12
                    * (param.m1_2.powi(4)
                        * (param.s12.powi(6)
                            + (param.s1 - param.s2).powi(6)
                            + param.s12.powi(4)
                                * (15. * param.s1.powi(2)
                                    + 46. * param.s1 * param.s2
                                    + 113. * param.s2.powi(2))
                            + param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (15. * param.s1.powi(2)
                                    + 46. * param.s1 * param.s2
                                    + 113. * param.s2.powi(2))
                            + -2.
                                * param.s12.powi(3)
                                * (10. * param.s1.powi(3)
                                    + 27. * param.s1.powi(2) * param.s2
                                    + 90. * param.s1 * param.s2.powi(2)
                                    + -249. * param.s2.powi(3))
                            - param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (6. * param.s1 + 13. * param.s2)
                            - param.s12.powi(5) * (6. * param.s1 + 13. * param.s2))
                        + param.m2_2.powi(4)
                            * (6. * param.s12.powi(6)
                                + param.s12.powi(5) * (89. * param.s1 + -28. * param.s2)
                                + (param.s1 - param.s2).powi(6)
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (35. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 14. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-145. * param.s1.powi(2)
                                        + -134. * param.s1 * param.s2
                                        + 53. * param.s2.powi(2))
                                + -2.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(3)
                                        + -148. * param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + 26. * param.s2.powi(3))
                                - param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (11. * param.s1 + 8. * param.s2))
                        + param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (param.s12.powi(6)
                                + param.s12.powi(5) * (19. * param.s1 + 12. * param.s2)
                                + param.s12.powi(4)
                                    * (-80. * param.s1.powi(2)
                                        + 351. * param.s1 * param.s2
                                        + -57. * param.s2.powi(2))
                                + (param.s1 - param.s2).powi(4)
                                    * (6. * param.s1.powi(2)
                                        + 23. * param.s1 * param.s2
                                        + param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (55. * param.s1.powi(3)
                                        + -162. * param.s1.powi(2) * param.s2
                                        + -185. * param.s1 * param.s2.powi(2)
                                        + 44. * param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (55. * param.s1.powi(4)
                                        + 454. * param.s1.powi(3) * param.s2
                                        + -1456. * param.s1.powi(2) * param.s2.powi(2)
                                        + 370. * param.s1 * param.s2.powi(3)
                                        + 57. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(3)
                                        + -414. * param.s1.powi(2) * param.s2
                                        + -375. * param.s1 * param.s2.powi(2)
                                        + -12. * param.s2.powi(3)))
                        + param.s12.powi(4)
                            * (param.s12.powi(6)
                                + -2. * param.s12.powi(5) * (3. * param.s1 + 4. * param.s2)
                                + param.s12.powi(4)
                                    * (15. * param.s1.powi(2)
                                        + 21. * param.s1 * param.s2
                                        + 28. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(3)
                                    * (5. * param.s1.powi(3)
                                        + param.s1.powi(2) * param.s2
                                        + -5. * param.s1 * param.s2.powi(2)
                                        + 13. * param.s2.powi(3))
                                + (param.s1 - param.s2).powi(2)
                                    * (param.s1.powi(4)
                                        + -9. * param.s1.powi(3) * param.s2
                                        + 51. * param.s1.powi(2) * param.s2.powi(2)
                                        + 101. * param.s1 * param.s2.powi(3)
                                        + 6. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (15. * param.s1.powi(4)
                                        + -34. * param.s1.powi(3) * param.s2
                                        + -54. * param.s1.powi(2) * param.s2.powi(2)
                                        + 10. * param.s1 * param.s2.powi(3)
                                        + 53. * param.s2.powi(4))
                                + -2.
                                    * param.s12
                                    * (3. * param.s1.powi(5)
                                        + -18. * param.s1.powi(4) * param.s2
                                        + 32. * param.s1.powi(3) * param.s2.powi(2)
                                        + -148. * param.s1.powi(2) * param.s2.powi(3)
                                        + 67. * param.s1 * param.s2.powi(4)
                                        + 14. * param.s2.powi(5)))
                        + param.m2_2
                            * param.s12.powi(3)
                            * (param.s12.powi(6)
                                + param.s12.powi(4)
                                    * (-10. * param.s1.powi(2)
                                        + -49. * param.s1 * param.s2
                                        + 13. * param.s2.powi(2))
                                + 2. * param.s12.powi(3)
                                    * (15. * param.s1.powi(3)
                                        + 68. * param.s1.powi(2) * param.s2
                                        + -145. * param.s1 * param.s2.powi(2)
                                        + 4. * param.s2.powi(3))
                                + param.s12
                                    * (19. * param.s1.powi(5)
                                        + -104. * param.s1.powi(4) * param.s2
                                        + 706. * param.s1.powi(3) * param.s2.powi(2)
                                        + -464. * param.s1.powi(2) * param.s2.powi(3)
                                        + -189. * param.s1 * param.s2.powi(4)
                                        + 32. * param.s2.powi(5))
                                - param.s12.powi(2)
                                    * (35. * param.s1.powi(4)
                                        + 34. * param.s1.powi(3) * param.s2
                                        + 404. * param.s1.powi(2) * param.s2.powi(2)
                                        + -630. * param.s1 * param.s2.powi(3)
                                        + 37. * param.s2.powi(4))
                                - (param.s1 - param.s2).powi(3)
                                    * (4. * param.s1.powi(3)
                                        + -47. * param.s1.powi(2) * param.s2
                                        + -128. * param.s1 * param.s2.powi(2)
                                        + -9. * param.s2.powi(3))
                                - param.s12.powi(5) * (param.s1 + 8. * param.s2))
                        + param.m1_2.powi(2)
                            * (param.m2_2.powi(2)
                                * (51. * param.s12.powi(6)
                                    + -7.
                                        * param.s12.powi(5)
                                        * (33. * param.s1 + -71. * param.s2)
                                    + 6. * (param.s1 - param.s2).powi(6)
                                    + -3.
                                        * param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (17. * param.s1 + 21. * param.s2)
                                    + param.s12.powi(4)
                                        * (420. * param.s1.powi(2)
                                            + -724. * param.s1 * param.s2
                                            + -892. * param.s2.powi(2))
                                    + 3. * param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (65. * param.s1.powi(2)
                                            + 162. * param.s1 * param.s2
                                            + 121. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (-390. * param.s1.powi(3)
                                            + 26. * param.s1.powi(2) * param.s2
                                            + 1790. * param.s1 * param.s2.powi(2)
                                            + 38. * param.s2.powi(3)))
                                + param.m2_2
                                    * param.s12
                                    * (19. * param.s12.powi(6)
                                        + (4. * param.s1 + -19. * param.s2)
                                            * (param.s1 - param.s2).powi(5)
                                        + param.s12.powi(4)
                                            * (210. * param.s1.powi(2)
                                                + 709. * param.s1 * param.s2
                                                + -723. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (115. * param.s1.powi(3)
                                                + 168. * param.s1.powi(2) * param.s2
                                                + 685. * param.s1 * param.s2.powi(2)
                                                + -996. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (135. * param.s1.powi(4)
                                                + -326. * param.s1.powi(3) * param.s2
                                                + 2284. * param.s1.powi(2) * param.s2.powi(2)
                                                + -1370. * param.s1 * param.s2.powi(3)
                                                + -723. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (39. * param.s1.powi(2)
                                                + -167. * param.s1 * param.s2
                                                + -292. * param.s2.powi(2))
                                        - param.s12.powi(5)
                                            * (99. * param.s1 + 292. * param.s2))
                                + param.s12.powi(2)
                                    * (6. * param.s12.powi(6)
                                        + -9.
                                            * param.s12.powi(5)
                                            * (4. * param.s1 + 7. * param.s2)
                                        + 3. * (param.s1 - param.s2).powi(4)
                                            * (2. * param.s1.powi(2)
                                                + -9. * param.s1 * param.s2
                                                + 17. * param.s2.powi(2))
                                        + 3. * param.s12.powi(4)
                                            * (30. * param.s1.powi(2)
                                                + 67. * param.s1 * param.s2
                                                + 121. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (60. * param.s1.powi(3)
                                                + 87. * param.s1.powi(2) * param.s2
                                                + 120. * param.s1 * param.s2.powi(2)
                                                + -19. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (45. * param.s1.powi(4)
                                                + -27. * param.s1.powi(3) * param.s2
                                                + -207. * param.s1.powi(2) * param.s2.powi(2)
                                                + 895. * param.s1 * param.s2.powi(3)
                                                + -446. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (36. * param.s1.powi(3)
                                                + -69. * param.s1.powi(2) * param.s2
                                                + -270. * param.s1 * param.s2.powi(2)
                                                + -497. * param.s2.powi(3))))
                        + param.m1_2
                            * (param.m2_2.powi(3)
                                * (101. * param.s12.powi(6)
                                    + -4. * (param.s1 - param.s2).powi(6)
                                    + param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (39. * param.s1 + 37. * param.s2)
                                    + param.s12.powi(4)
                                        * (-30. * param.s1.powi(2)
                                            + 1016. * param.s1 * param.s2
                                            + 38. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (290. * param.s1.powi(3)
                                            + -754. * param.s1.powi(2) * param.s2
                                            + -730. * param.s1 * param.s2.powi(2)
                                            + 218. * param.s2.powi(3))
                                    - param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (195. * param.s1.powi(2)
                                            + 334. * param.s1 * param.s2
                                            + 167. * param.s2.powi(2))
                                    - param.s12.powi(5) * (201. * param.s1 + 223. * param.s2))
                                + param.s12.powi(3)
                                    * (-4. * param.s12.powi(6)
                                        + param.s12.powi(5) * (24. * param.s1 + 37. * param.s2)
                                        + param.s12.powi(3)
                                            * (80. * param.s1.powi(3)
                                                + 66. * param.s1.powi(2) * param.s2
                                                + 218. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (-60. * param.s1.powi(4)
                                                + 86. * param.s1.powi(3) * param.s2
                                                + 306. * param.s1.powi(2) * param.s2.powi(2)
                                                + -730. * param.s1 * param.s2.powi(3)
                                                + 38. * param.s2.powi(4))
                                        + param.s12
                                            * (24. * param.s1.powi(5)
                                                + -119. * param.s1.powi(4) * param.s2
                                                + 56. * param.s1.powi(3) * param.s2.powi(2)
                                                + -754. * param.s1.powi(2) * param.s2.powi(3)
                                                + 1016. * param.s1 * param.s2.powi(4)
                                                + -223. * param.s2.powi(5))
                                        - (param.s1 - param.s2).powi(3)
                                            * (4. * param.s1.powi(3)
                                                + -27. * param.s1.powi(2) * param.s2
                                                + 102. * param.s1 * param.s2.powi(2)
                                                + 101. * param.s2.powi(3))
                                        - param.s12.powi(4)
                                            * (60. * param.s1.powi(2)
                                                + 109. * param.s1 * param.s2
                                                + 167. * param.s2.powi(2)))
                                - param.m2_2.powi(2)
                                    * param.s12
                                    * (36. * param.s12.powi(6)
                                        + param.s12.powi(5)
                                            * (-116. * param.s1 + 247. * param.s2)
                                        + param.s12.powi(4)
                                            * (100. * param.s1.powi(2)
                                                + 721. * param.s1 * param.s2
                                                + -857. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (44. * param.s1.powi(2)
                                                + 243. * param.s1 * param.s2
                                                + 133. * param.s2.powi(2))
                                        + 2. * param.s12.powi(3)
                                            * (20. * param.s1.powi(3)
                                                + -1027. * param.s1.powi(2) * param.s2
                                                + 620. * param.s1 * param.s2.powi(2)
                                                + 359. * param.s2.powi(3))
                                        + -2.
                                            * param.s12.powi(2)
                                            * (50. * param.s1.powi(4)
                                                + -483. * param.s1.powi(3) * param.s2
                                                + -553. * param.s1.powi(2) * param.s2.powi(2)
                                                + 975. * param.s1 * param.s2.powi(3)
                                                + 11. * param.s2.powi(4))
                                        - (param.s1 - param.s2).powi(5)
                                            * (4. * param.s1 + 11. * param.s2))
                                - param.m2_2
                                    * param.s12.powi(2)
                                    * (11. * param.s12.powi(6)
                                        + 2. * param.s12.powi(4)
                                            * (45. * param.s1.powi(2)
                                                + 78. * param.s1 * param.s2
                                                + -11. * param.s2.powi(2))
                                        + -4.
                                            * (param.s1 - param.s2).powi(4)
                                            * (param.s1.powi(2)
                                                + -7. * param.s1 * param.s2
                                                + -9. * param.s2.powi(2))
                                        + param.s12
                                            * (param.s1 - param.s2).powi(2)
                                            * (9. * param.s1.powi(3)
                                                + 129. * param.s1.powi(2) * param.s2
                                                + 1215. * param.s1 * param.s2.powi(2)
                                                + 247. * param.s2.powi(3))
                                        + param.s12.powi(3)
                                            * (-70. * param.s1.powi(3)
                                                + 286. * param.s1.powi(2) * param.s2
                                                + -1950. * param.s1 * param.s2.powi(2)
                                                + 718. * param.s2.powi(3))
                                        + param.s12.powi(2)
                                            * (15. * param.s1.powi(4)
                                                + -464. * param.s1.powi(3) * param.s2
                                                + 1106. * param.s1.powi(2) * param.s2.powi(2)
                                                + 1240. * param.s1 * param.s2.powi(3)
                                                + -857. * param.s2.powi(4))
                                        - param.s12.powi(5)
                                            * (51. * param.s1 + 133. * param.s2)))
                        - param.m1_2.powi(3)
                            * (param.m2_2
                                * (9. * param.s12.powi(6)
                                    + 4. * (param.s1 - param.s2).powi(6)
                                    + 2. * param.s12.powi(4)
                                        * (55. * param.s1.powi(2)
                                            + 252. * param.s1 * param.s2
                                            + -419. * param.s2.powi(2))
                                    + param.s12.powi(2)
                                        * (param.s1 - param.s2).powi(2)
                                        * (85. * param.s1.powi(2)
                                            + 274. * param.s1 * param.s2
                                            + 337. * param.s2.powi(2))
                                    + param.s12.powi(3)
                                        * (-130. * param.s1.powi(3)
                                            + -486. * param.s1.powi(2) * param.s2
                                            + 890. * param.s1 * param.s2.powi(2)
                                            + 702. * param.s2.powi(3))
                                    - param.s12.powi(5) * (49. * param.s1 + 167. * param.s2)
                                    - param.s12
                                        * (param.s1 - param.s2).powi(4)
                                        * (29. * param.s1 + 47. * param.s2))
                                + param.s12
                                    * (4. * param.s12.powi(6)
                                        + (4. * param.s1 + -9. * param.s2)
                                            * (param.s1 - param.s2).powi(5)
                                        + param.s12.powi(4)
                                            * (60. * param.s1.powi(2)
                                                + 159. * param.s1 * param.s2
                                                + 337. * param.s2.powi(2))
                                        + -2.
                                            * param.s12.powi(3)
                                            * (40. * param.s1.powi(3)
                                                + 83. * param.s1.powi(2) * param.s2
                                                + 200. * param.s1 * param.s2.powi(2)
                                                + -351. * param.s2.powi(3))
                                        + 2. * param.s12.powi(2)
                                            * (30. * param.s1.powi(4)
                                                + 7. * param.s1.powi(3) * param.s2
                                                + -63. * param.s1.powi(2) * param.s2.powi(2)
                                                + 445. * param.s1 * param.s2.powi(3)
                                                + -419. * param.s2.powi(4))
                                        - param.s12
                                            * (param.s1 - param.s2).powi(3)
                                            * (24. * param.s1.powi(2)
                                                + 3. * param.s1 * param.s2
                                                + -167. * param.s2.powi(2))
                                        - param.s12.powi(5) * (24. * param.s1 + 47. * param.s2)))
                        - param.m2_2.powi(3)
                            * param.s12
                            * (9. * param.s12.powi(6)
                                + param.s12.powi(5) * (101. * param.s1 + -32. * param.s2)
                                + (param.s1 - param.s2).powi(5) * (4. * param.s1 + param.s2)
                                + param.s12.powi(4)
                                    * (-310. * param.s1.powi(2)
                                        + 189. * param.s1 * param.s2
                                        + 37. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (230. * param.s1.powi(3)
                                        + 464. * param.s1.powi(2) * param.s2
                                        + -630. * param.s1 * param.s2.powi(2)
                                        + -8. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (25. * param.s1.powi(4)
                                        + -706. * param.s1.powi(3) * param.s2
                                        + 404. * param.s1.powi(2) * param.s2.powi(2)
                                        + 290. * param.s1 * param.s2.powi(3)
                                        + -13. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (59. * param.s1.powi(2)
                                        + 73. * param.s1 * param.s2
                                        + 8. * param.s2.powi(2))))
                - param.m1_2
                    * (-4.
                        * param.m2_2.powi(3)
                        * param.s12
                        * (6. * param.s12.powi(7)
                            + 2. * param.s12.powi(6) * (6. * param.s1 - param.s2)
                            + 3. * (param.s1 - param.s2).powi(6) * (param.s1 + param.s2)
                            + param.s12.powi(5)
                                * (-123. * param.s1.powi(2)
                                    + 544. * param.s1 * param.s2
                                    + -65. * param.s2.powi(2))
                            + 2. * param.s12.powi(2)
                                * (param.s1 - param.s2).powi(2)
                                * (57. * param.s1.powi(3)
                                    + 221. * param.s1.powi(2) * param.s2
                                    + 198. * param.s1 * param.s2.powi(2)
                                    + 46. * param.s2.powi(3))
                            + param.s12.powi(4)
                                * (255. * param.s1.powi(3)
                                    + -821. * param.s1.powi(2) * param.s2
                                    + -881. * param.s1 * param.s2.powi(2)
                                    + 163. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (-240. * param.s1.powi(4)
                                    + 34. * param.s1.powi(3) * param.s2
                                    + 1752. * param.s1.powi(2) * param.s2.powi(2)
                                    + 90. * param.s1 * param.s2.powi(3)
                                    + -172. * param.s2.powi(4))
                            - param.s12
                                * (param.s1 - param.s2).powi(4)
                                * (27. * param.s1.powi(2)
                                    + 62. * param.s1 * param.s2
                                    + 25. * param.s2.powi(2)))
                        + param.m2_2.powi(4)
                            * (66. * param.s12.powi(7)
                                + 2. * param.s12.powi(6) * (72. * param.s1 + -179. * param.s2)
                                + 15. * (param.s1 - param.s2).powi(7)
                                + 2. * param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(3)
                                    * (252. * param.s1.powi(2)
                                        + 349. * param.s1 * param.s2
                                        + 214. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-1035. * param.s1.powi(2)
                                        + 596. * param.s1 * param.s2
                                        + 833. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (1665. * param.s1.powi(3)
                                        + 821. * param.s1.powi(2) * param.s2
                                        + -1675. * param.s1 * param.s2.powi(2)
                                        + -1087. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-1230. * param.s1.powi(4)
                                        + -664. * param.s1.powi(3) * param.s2
                                        + 306. * param.s1.powi(2) * param.s2.powi(2)
                                        + 720. * param.s1 * param.s2.powi(3)
                                        + 868. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(5)
                                    * (129. * param.s1 + 121. * param.s2))
                        + -4.
                            * param.m2_2
                            * param.s12.powi(3)
                            * (3. * param.s12.powi(7)
                                + -5. * param.s12.powi(6) * (3. * param.s1 + 5. * param.s2)
                                + param.s12.powi(5)
                                    * (27. * param.s1.powi(2)
                                        + 38. * param.s1 * param.s2
                                        + 92. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (-15. * param.s1.powi(3)
                                        + 71. * param.s1.powi(2) * param.s2
                                        + 212. * param.s1 * param.s2.powi(2)
                                        + -172. * param.s2.powi(3))
                                + 3. * (param.s1 - param.s2).powi(4)
                                    * (param.s1.powi(3)
                                        + -5. * param.s1.powi(2) * param.s2
                                        + 12. * param.s1 * param.s2.powi(2)
                                        + 2. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-15. * param.s1.powi(4)
                                        + -164. * param.s1.powi(3) * param.s2
                                        + -258. * param.s1.powi(2) * param.s2.powi(2)
                                        + 90. * param.s1 * param.s2.powi(3)
                                        + 163. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (27. * param.s1.powi(5)
                                        + 61. * param.s1.powi(4) * param.s2
                                        + -374. * param.s1.powi(3) * param.s2.powi(2)
                                        + 1752. * param.s1.powi(2) * param.s2.powi(3)
                                        + -881. * param.s1 * param.s2.powi(4)
                                        + -65. * param.s2.powi(5))
                                - param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (15. * param.s1.powi(4)
                                        + -16. * param.s1.powi(3) * param.s2
                                        + -261. * param.s1.powi(2) * param.s2.powi(2)
                                        + -540. * param.s1 * param.s2.powi(3)
                                        + 2. * param.s2.powi(4)))
                        + -3.
                            * param.m2_2.powi(2)
                            * param.s12.powi(2)
                            * (5. * param.s12.powi(7)
                                + (param.s1 - param.s2).powi(5)
                                    * (2. * param.s1.powi(2)
                                        + -12. * param.s1 * param.s2
                                        + -5. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (-12. * param.s1.powi(2)
                                        + -136. * param.s1 * param.s2
                                        + 99. * param.s2.powi(2))
                                + param.s12.powi(4)
                                    * (80. * param.s1.powi(3)
                                        + 532. * param.s1.powi(2) * param.s2
                                        + -971. * param.s1 * param.s2.powi(2)
                                        + -61. * param.s2.powi(3))
                                + param.s12.powi(2)
                                    * (75. * param.s1.powi(5)
                                        + -203. * param.s1.powi(4) * param.s2
                                        + 2168. * param.s1.powi(3) * param.s2.powi(2)
                                        + -1168. * param.s1.powi(2) * param.s2.powi(3)
                                        + -971. * param.s1 * param.s2.powi(4)
                                        + 99. * param.s2.powi(5))
                                - param.s12.powi(3)
                                    * (115. * param.s1.powi(4)
                                        + 328. * param.s1.powi(3) * param.s2
                                        + 1168. * param.s1.powi(2) * param.s2.powi(2)
                                        + -2240. * param.s1 * param.s2.powi(3)
                                        + 61. * param.s2.powi(4))
                                - param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (22. * param.s1.powi(3)
                                        + -134. * param.s1.powi(2) * param.s2
                                        + -265. * param.s1 * param.s2.powi(2)
                                        + -43. * param.s2.powi(3))
                                - param.s12.powi(6) * (13. * param.s1 + 43. * param.s2))
                        + param.s12.powi(4)
                            * (-15. * param.s12.powi(7)
                                + param.s12.powi(6) * (105. * param.s1 + 121. * param.s2)
                                + param.s12.powi(4)
                                    * (525. * param.s1.powi(3)
                                        + 565. * param.s1.powi(2) * param.s2
                                        + 586. * param.s1 * param.s2.powi(2)
                                        + 868. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (-525. * param.s1.powi(4)
                                        + 80. * param.s1.powi(3) * param.s2
                                        + 306. * param.s1.powi(2) * param.s2.powi(2)
                                        + 720. * param.s1 * param.s2.powi(3)
                                        + -1087. * param.s2.powi(4))
                                + 3. * (param.s1 - param.s2).powi(3)
                                    * (5. * param.s1.powi(4)
                                        + -28. * param.s1.powi(3) * param.s2
                                        + 69. * param.s1.powi(2) * param.s2.powi(2)
                                        + -114. * param.s1 * param.s2.powi(3)
                                        + -22. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (315. * param.s1.powi(5)
                                        + -685. * param.s1.powi(4) * param.s2
                                        + -154. * param.s1.powi(3) * param.s2.powi(2)
                                        + 306. * param.s1.powi(2) * param.s2.powi(3)
                                        + -1675. * param.s1 * param.s2.powi(4)
                                        + 833. * param.s2.powi(5))
                                + param.s12
                                    * (-105. * param.s1.powi(6)
                                        + 524. * param.s1.powi(5) * param.s2
                                        + -814. * param.s1.powi(4) * param.s2.powi(2)
                                        + -664. * param.s1.powi(3) * param.s2.powi(3)
                                        + 821. * param.s1.powi(2) * param.s2.powi(4)
                                        + 596. * param.s1 * param.s2.powi(5)
                                        + -358. * param.s2.powi(6))
                                - param.s12.powi(5)
                                    * (315. * param.s1.powi(2)
                                        + 476. * param.s1 * param.s2
                                        + 428. * param.s2.powi(2))))
                - param.m1_2.powi(2)
                    * (param.m2_2.powi(3)
                        * (171. * param.s12.powi(7)
                            + -30. * (param.s1 - param.s2).powi(7)
                            + param.s12.powi(6) * (-891. * param.s1 + 47. * param.s2)
                            + 2. * param.s12
                                * (param.s1 - param.s2).powi(5)
                                * (123. * param.s1 + 127. * param.s2)
                            + 4. * param.s12.powi(5)
                                * (495. * param.s1.powi(2)
                                    + 464. * param.s1 * param.s2
                                    + -388. * param.s2.powi(2))
                            + -4.
                                * param.s12.powi(4)
                                * (615. * param.s1.powi(3)
                                    + 976. * param.s1.powi(2) * param.s2
                                    + -290. * param.s1 * param.s2.powi(2)
                                    + -707. * param.s2.powi(3))
                            + param.s12.powi(3)
                                * (1875. * param.s1.powi(4)
                                    + 1496. * param.s1.powi(3) * param.s2
                                    + 306. * param.s1.powi(2) * param.s2.powi(2)
                                    + -1440. * param.s1 * param.s2.powi(3)
                                    + -2237. * param.s2.powi(4))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(3)
                                * (891. * param.s1.powi(2)
                                    + 1402. * param.s1 * param.s2
                                    + 967. * param.s2.powi(2)))
                        + 3. * param.m2_2.powi(2)
                            * param.s12
                            * (29. * param.s12.powi(7)
                                + -6.
                                    * (param.s1 + -3. * param.s2)
                                    * (param.s1 - param.s2).powi(6)
                                + -5. * param.s12.powi(6) * (33. * param.s1 + 67. * param.s2)
                                + 2. * param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (25. * param.s1.powi(2)
                                        + -56. * param.s1 * param.s2
                                        + -83. * param.s2.powi(2))
                                + 4. * param.s12.powi(5)
                                    * (99. * param.s1.powi(2)
                                        + 206. * param.s1 * param.s2
                                        + 119. * param.s2.powi(2))
                                + -4.
                                    * param.s12.powi(4)
                                    * (130. * param.s1.powi(3)
                                        + 78. * param.s1.powi(2) * param.s2
                                        + 761. * param.s1 * param.s2.powi(2)
                                        + -101. * param.s2.powi(3))
                                + param.s12.powi(3)
                                    * (405. * param.s1.powi(4)
                                        + -712. * param.s1.powi(3) * param.s2
                                        + 2086. * param.s1.powi(2) * param.s2.powi(2)
                                        + 2320. * param.s1 * param.s2.powi(3)
                                        + -1171. * param.s2.powi(4))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (189. * param.s1.powi(3)
                                        + -415. * param.s1.powi(2) * param.s2
                                        + -1117. * param.s1 * param.s2.powi(2)
                                        + -745. * param.s2.powi(3)))
                        + param.s12.powi(3)
                            * (30. * param.s12.powi(7)
                                + -2. * param.s12.powi(6) * (105. * param.s1 + 127. * param.s2)
                                + param.s12.powi(5)
                                    * (630. * param.s1.powi(2)
                                        + 1024. * param.s1 * param.s2
                                        + 967. * param.s2.powi(2))
                                + -3.
                                    * (param.s1 - param.s2).powi(4)
                                    * (10. * param.s1.powi(3)
                                        + -42. * param.s1.powi(2) * param.s2
                                        + 69. * param.s1 * param.s2.powi(2)
                                        + -57. * param.s2.powi(3))
                                + param.s12
                                    * (param.s1 - param.s2).powi(2)
                                    * (210. * param.s1.powi(4)
                                        + -556. * param.s1.powi(3) * param.s2
                                        + -51. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1950. * param.s1 * param.s2.powi(3)
                                        + 47. * param.s2.powi(4))
                                + 2. * param.s12.powi(3)
                                    * (525. * param.s1.powi(4)
                                        + 40. * param.s1.powi(3) * param.s2
                                        + -207. * param.s1.powi(2) * param.s2.powi(2)
                                        + -720. * param.s1 * param.s2.powi(3)
                                        + 1414. * param.s2.powi(4))
                                + 2. * param.s12.powi(2)
                                    * (-315. * param.s1.powi(5)
                                        + 595. * param.s1.powi(4) * param.s2
                                        + 283. * param.s1.powi(3) * param.s2.powi(2)
                                        + 153. * param.s1.powi(2) * param.s2.powi(3)
                                        + 580. * param.s1 * param.s2.powi(4)
                                        + -776. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (1050. * param.s1.powi(3)
                                        + 1310. * param.s1.powi(2) * param.s2
                                        + 1499. * param.s1 * param.s2.powi(2)
                                        + 2237. * param.s2.powi(3)))
                        + 3. * param.m2_2
                            * param.s12.powi(2)
                            * (18. * param.s12.powi(7)
                                + -2. * param.s12.powi(6) * (57. * param.s1 + 83. * param.s2)
                                + param.s12.powi(5)
                                    * (306. * param.s1.powi(2)
                                        + 552. * param.s1 * param.s2
                                        + 745. * param.s2.powi(2))
                                + param.s12
                                    * (param.s1 - param.s2).powi(3)
                                    * (54. * param.s1.powi(3)
                                        + -150. * param.s1.powi(2) * param.s2
                                        + 181. * param.s1 * param.s2.powi(2)
                                        + 335. * param.s2.powi(3))
                                + 2. * param.s12.powi(3)
                                    * (195. * param.s1.powi(4)
                                        + -104. * param.s1.powi(3) * param.s2
                                        + -537. * param.s1.powi(2) * param.s2.powi(2)
                                        + 1160. * param.s1 * param.s2.powi(3)
                                        + 202. * param.s2.powi(4))
                                + param.s12.powi(2)
                                    * (-198. * param.s1.powi(5)
                                        + 582. * param.s1.powi(4) * param.s2
                                        + 98. * param.s1.powi(3) * param.s2.powi(2)
                                        + 2086. * param.s1.powi(2) * param.s2.powi(3)
                                        + -3044. * param.s1 * param.s2.powi(4)
                                        + 476. * param.s2.powi(5))
                                - param.s12.powi(4)
                                    * (450. * param.s1.powi(3)
                                        + 498. * param.s1.powi(2) * param.s2
                                        + 373. * param.s1 * param.s2.powi(2)
                                        + 1171. * param.s2.powi(3))
                                - (param.s1 - param.s2).powi(5)
                                    * (6. * param.s1.powi(2)
                                        + -20. * param.s1 * param.s2
                                        + 29. * param.s2.powi(2))))
                - param.m1_2.powi(4)
                    * (param.m2_2
                        * (21. * param.s12.powi(7)
                            + -15. * (param.s1 - param.s2).powi(7)
                            + param.s12
                                * (param.s1 - param.s2).powi(5)
                                * (111. * param.s1 + 139. * param.s2)
                            + param.s12.powi(5)
                                * (405. * param.s1.powi(2)
                                    + 896. * param.s1 * param.s2
                                    + 1373. * param.s2.powi(2))
                            + param.s12.powi(3)
                                * (615. * param.s1.powi(4)
                                    + 496. * param.s1.powi(3) * param.s2
                                    + 756. * param.s1.powi(2) * param.s2.powi(2)
                                    + 120. * param.s1 * param.s2.powi(3)
                                    + -1987. * param.s2.powi(4))
                            - param.s12.powi(4)
                                * (645. * param.s1.powi(3)
                                    + 1249. * param.s1.powi(2) * param.s2
                                    + 2545. * param.s1 * param.s2.powi(2)
                                    + -323. * param.s2.powi(3))
                            - param.s12.powi(2)
                                * (param.s1 - param.s2).powi(3)
                                * (351. * param.s1.powi(2)
                                    + 662. * param.s1 * param.s2
                                    + 617. * param.s2.powi(2))
                            - param.s12.powi(6) * (141. * param.s1 + 223. * param.s2))
                        + param.s12
                            * (15. * param.s12.powi(7)
                                + -3.
                                    * (5. * param.s1 + -7. * param.s2)
                                    * (param.s1 - param.s2).powi(6)
                                + param.s12
                                    * (param.s1 - param.s2).powi(4)
                                    * (105. * param.s1.powi(2)
                                        + 4. * param.s1 * param.s2
                                        + -223. * param.s2.powi(2))
                                + param.s12.powi(5)
                                    * (315. * param.s1.powi(2)
                                        + 584. * param.s1 * param.s2
                                        + 617. * param.s2.powi(2))
                                + param.s12.powi(3)
                                    * (525. * param.s1.powi(4)
                                        + 280. * param.s1.powi(3) * param.s2
                                        + 216. * param.s1.powi(2) * param.s2.powi(2)
                                        + 120. * param.s1 * param.s2.powi(3)
                                        + 323. * param.s2.powi(4))
                                - param.s12.powi(4)
                                    * (525. * param.s1.powi(3)
                                        + 835. * param.s1.powi(2) * param.s2
                                        + 1189. * param.s1 * param.s2.powi(2)
                                        + 1987. * param.s2.powi(3))
                                - param.s12.powi(2)
                                    * (param.s1 - param.s2).powi(2)
                                    * (315. * param.s1.powi(3)
                                        + 215. * param.s1.powi(2) * param.s2
                                        + -201. * param.s1 * param.s2.powi(2)
                                        + -1373. * param.s2.powi(3))
                                - param.s12.powi(6) * (105. * param.s1 + 139. * param.s2))))
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
                    * ((14. * param.m1_2.powi(2)
                        + 3. * param.s12.powi(2)
                        + param.s12 * (8. * param.s1 + -6. * param.s2)
                        + 3. * (param.s1 - param.s2).powi(2)
                        + -14. * param.m1_2 * (param.s1 + param.s12 - param.s2))
                        * param.s2.powi(2)
                        + param.m2_2.powi(2)
                            * (3. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + 8. * param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + -6. * param.s12 * (param.s1 + param.s2))
                        + -2.
                            * param.m2_2
                            * param.s2
                            * (-4. * param.s1.powi(2)
                                + 3. * param.s12.powi(2)
                                + param.s12 * (param.s1 + -6. * param.s2)
                                + param.s1 * param.s2
                                + 3. * param.s2.powi(2)
                                + 7. * param.m1_2 * (param.s1 + param.s2 - param.s12))
                        + param.m0_2.powi(2)
                            * (3. * param.s12.powi(2)
                                + 3. * (param.s1 - param.s2).powi(2)
                                + param.s12 * (-6. * param.s1 + 8. * param.s2))
                        + -2.
                            * param.m0_2
                            * (param.m2_2
                                * (3. * param.s1.powi(2)
                                    + 3. * param.s12.powi(2)
                                    + param.s1 * param.s2
                                    + -4. * param.s2.powi(2)
                                    + param.s12 * (-6. * param.s1 + param.s2))
                                + param.s2
                                    * (-4. * param.s12.powi(2)
                                        + 3. * (param.s1 - param.s2).powi(2)
                                        + param.s12 * (param.s1 + param.s2)
                                        + 7. * param.m1_2 * (param.s12 + param.s2 - param.s1))))
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
