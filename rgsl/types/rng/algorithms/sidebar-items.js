initSidebarItems({"fn":[["cmrg","This is a combined multiple recursive generator by L’Ecuyer. Its sequence is,"],["gfsr4","The gfsr4 generator is like a lagged-fibonacci generator, and produces each number as an xor’d sum of four previous values."],["mrg","This is a fifth-order multiple recursive generator by L’Ecuyer, Blouin and Coutre. Its sequence is,"],["mt19937","The MT19937 generator of Makoto Matsumoto and Takuji Nishimura is a variant of the twisted generalized feedback shift-register algorithm, and is known as the “Mersenne Twister” generator. It has a Mersenne prime period of 2^19937 - 1 (about 10^6000) and is equi-distributed in 623 dimensions. It has passed the DIEHARD statistical tests. It uses 624 words of state per generator and is comparable in speed to the other generators. The original generator used a default seed of 4357 and choosing s equal to zero in gsl_rng_set reproduces this. Later versions switched to 5489 as the default seed, you can choose this explicitly via gsl_rng_set instead if you require it."],["ranlux","The ranlux generator is an implementation of the original algorithm developed by Lüscher. It uses a lagged-fibonacci-with-skipping algorithm to produce “luxury random numbers”. It is a 24-bit generator, originally designed for single-precision IEEE floating point numbers. This implementation is based on integer arithmetic, while the second-generation versions RANLXS and RANLXD described above provide floating-point implementations which will be faster on many platforms. The period of the generator is about 10^171. The algorithm has mathematically proven properties and it can provide truly decorrelated numbers at a known level of randomness. The default level of decorrelation recommended by Lüscher is provided by gsl_rng_ranlux, while gsl_rng_ranlux389 gives the highest level of randomness, with all 24 bits decorrelated. Both types of generator use 24 words of state per generator."],["ranlux389","The ranlux generator is an implementation of the original algorithm developed by Lüscher. It uses a lagged-fibonacci-with-skipping algorithm to produce “luxury random numbers”. It is a 24-bit generator, originally designed for single-precision IEEE floating point numbers. This implementation is based on integer arithmetic, while the second-generation versions RANLXS and RANLXD described above provide floating-point implementations which will be faster on many platforms. The period of the generator is about 10^171. The algorithm has mathematically proven properties and it can provide truly decorrelated numbers at a known level of randomness. The default level of decorrelation recommended by Lüscher is provided by gsl_rng_ranlux, while gsl_rng_ranlux389 gives the highest level of randomness, with all 24 bits decorrelated. Both types of generator use 24 words of state per generator."],["ranlxd1","This generator produces double precision output (48 bits) from the RANLXS generator. The library provides two luxury levels ranlxd1 and ranlxd2, in increasing order of strength."],["ranlxd2","This generator produces double precision output (48 bits) from the RANLXS generator. The library provides two luxury levels ranlxd1 and ranlxd2, in increasing order of strength."],["ranlxs0","The generator ranlxs0 is a second-generation version of the RANLUX algorithm of Lüscher, which produces “luxury random numbers”. This generator provides single precision output (24 bits) at three luxury levels ranlxs0, ranlxs1 and ranlxs2, in increasing order of strength. It uses double-precision floating point arithmetic internally and can be significantly faster than the integer version of ranlux, particularly on 64-bit architectures. The period of the generator is about 10^171. The algorithm has mathematically proven properties and can provide truly decorrelated numbers at a known level of randomness. The higher luxury levels provide increased decorrelation between samples as an additional safety margin."],["ranlxs1","The generator ranlxs0 is a second-generation version of the RANLUX algorithm of Lüscher, which produces “luxury random numbers”. This generator provides single precision output (24 bits) at three luxury levels ranlxs0, ranlxs1 and ranlxs2, in increasing order of strength. It uses double-precision floating point arithmetic internally and can be significantly faster than the integer version of ranlux, particularly on 64-bit architectures. The period of the generator is about 10^171. The algorithm has mathematically proven properties and can provide truly decorrelated numbers at a known level of randomness. The higher luxury levels provide increased decorrelation between samples as an additional safety margin."],["ranlxs2","The generator ranlxs0 is a second-generation version of the RANLUX algorithm of Lüscher, which produces “luxury random numbers”. This generator provides single precision output (24 bits) at three luxury levels ranlxs0, ranlxs1 and ranlxs2, in increasing order of strength. It uses double-precision floating point arithmetic internally and can be significantly faster than the integer version of ranlux, particularly on 64-bit architectures. The period of the generator is about 10^171. The algorithm has mathematically proven properties and can provide truly decorrelated numbers at a known level of randomness. The higher luxury levels provide increased decorrelation between samples as an additional safety margin."],["taus","This is a maximally equidistributed combined Tausworthe generator by L’Ecuyer. The sequence is,"],["taus2","This is a maximally equidistributed combined Tausworthe generator by L’Ecuyer. The sequence is,"]]});