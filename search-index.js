var searchIndex = JSON.parse('{\
"special_functions":{"doc":"Library providing pure rust implementation of various …","t":[14,0,0,0,0,0,5,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,5,5,5,5,5,5,5,5,5,5,5,5,0,0,5,5,5,5,5,5,5,5,5,5,5,5],"n":["approx_fn","approximations","basic","bessel","other","particle_physics","chebyshev","linear","piecewise_chebyshev","piecewise_polynomial","piecewise_polynomial_ratio","polynomial","polynomial_ratio","linear","i0","i1","i2","i3","i4","i5","i6","i7","i8","i9","j0","j1","j2","j3","j4","j5","j6","j7","j8","j9","k0","k1","k1_on_k2","k2","k3","k4","k5","k6","k7","k8","k9","y0","y1","y2","y3","y4","y5","y6","y7","y8","y9","binomial","gamma","harmonic_number","polylog","li0","li1","li2","li3","li4","li5","li6","li7","li8","li9","kallen_lambda","kallen_lambda_sqrt","pave_absorptive","statistics","a","b","c","d","bose_einstein_massive","bose_einstein_massless","bose_einstein_normalized_massive","bose_einstein_normalized_massless","fermi_dirac_massive","fermi_dirac_massless","fermi_dirac_normalized_massive","fermi_dirac_normalized_massless"],"q":["special_functions","","","","","","special_functions::approximations","","","","","","","special_functions::approximations::linear","special_functions::bessel","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","special_functions::other","","","","special_functions::other::polylog","","","","","","","","","","special_functions::particle_physics","","","","special_functions::particle_physics::pave_absorptive","","","","special_functions::particle_physics::statistics","","","","","","",""],"d":["Create a function from a module containing all the …","Approximations","Basic functions.","Bessel functions","Miscellaneous functions","Particle statistics","Evaluates a series of Chebyshev functions at x.","Linear interpolation","Evaluates an arbitrary piecewise Chebyshev function at a …","Evaluates an arbitrary piecewise single-variable …","Evaluates an arbitrary piecewise ratio of single-variable …","Evaluates an arbitrary single-variable polynomial at a …","Evaluates an arbitrary ratio of single-variable …","Perform linear interpolation on data.","Approximation of modified Bessel function <code>$I_0(x)$</code> for …","Approximation of modified Bessel function <code>$I_1(x)$</code> for …","Approximation of modified Bessel function <code>$I_2(x)$</code> for …","Approximation of modified Bessel function <code>$I_3(x)$</code> for …","Approximation of modified Bessel function <code>$I_4(x)$</code> for …","Approximation of modified Bessel function <code>$I_5(x)$</code> for …","Approximation of modified Bessel function <code>$I_6(x)$</code> for …","Approximation of modified Bessel function <code>$I_7(x)$</code> for …","Approximation of modified Bessel function <code>$I_8(x)$</code> for …","Approximation of modified Bessel function <code>$I_9(x)$</code> for …","Approximation of modified Bessel function <code>$J_0(x)$</code> for …","Approximation of modified Bessel function <code>$J_1(x)$</code> for …","Approximation of modified Bessel function <code>$J_2(x)$</code> for …","Approximation of modified Bessel function <code>$J_3(x)$</code> for …","Approximation of modified Bessel function <code>$J_4(x)$</code> for …","Approximation of modified Bessel function <code>$J_5(x)$</code> for …","Approximation of modified Bessel function <code>$J_6(x)$</code> for …","Approximation of modified Bessel function <code>$J_7(x)$</code> for …","Approximation of modified Bessel function <code>$J_8(x)$</code> for …","Approximation of modified Bessel function <code>$J_9(x)$</code> for …","Approximation of modified Bessel function <code>$K_0(x)$</code> for …","Approximation of modified Bessel function <code>$K_1(x)$</code> for …","Approximatino of the ratio of Bessel function …","Approximation of modified Bessel function <code>$K_2(x)$</code> for …","Approximation of modified Bessel function <code>$K_3(x)$</code> for …","Approximation of modified Bessel function <code>$K_4(x)$</code> for …","Approximation of modified Bessel function <code>$K_5(x)$</code> for …","Approximation of modified Bessel function <code>$K_6(x)$</code> for …","Approximation of modified Bessel function <code>$K_7(x)$</code> for …","Approximation of modified Bessel function <code>$K_8(x)$</code> for …","Approximation of modified Bessel function <code>$K_9(x)$</code> for …","Approximation of modified Bessel function <code>$Y_0(x)$</code> for …","Approximation of modified Bessel function <code>$Y_1(x)$</code> for …","Approximation of modified Bessel function <code>$Y_2(x)$</code> for …","Approximation of modified Bessel function <code>$Y_3(x)$</code> for …","Approximation of modified Bessel function <code>$Y_4(x)$</code> for …","Approximation of modified Bessel function <code>$Y_5(x)$</code> for …","Approximation of modified Bessel function <code>$Y_6(x)$</code> for …","Approximation of modified Bessel function <code>$Y_7(x)$</code> for …","Approximation of modified Bessel function <code>$Y_8(x)$</code> for …","Approximation of modified Bessel function <code>$Y_9(x)$</code> for …","Binomial coefficient","Approximatino of the gamma function for <code>$x &gt; 0$</code>.","Approximation of the Harmonic number extended to all …","Polylogarithms functions","Approximation of polylogarithm function <code>$\\\\Li_0(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_1(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_2(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_3(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_4(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_5(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_6(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_7(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_8(x)$</code> for …","Approximation of polylogarithm function <code>$\\\\Li_9(x)$</code> for …","Kallen lambda function:","Square root of the Kallen lambda function:","Passarino-Veltman coefficient functions.","Particle Statistics","Absorptive part of the Passarin-Veltman coefficient …","Absorptive part of the Passarin-Veltman coefficient …","Absorptive part of the Passarin-Veltman coefficient …","Absorptive part of the Passarin-Veltman coefficient …","Equilibrium number density of massive Bose-Einstein …","Equilibrium number density of a massless Bose-Einstein …","Equilibrium number density of massive Bose-Einstein …","Equilibrium number density of massless Bose-Einstein …","Equilibrium number density of massive Fermi-Dirac …","Equilibrium number density of a massless Fermi-Dirac …","Equilibrium number density of massive Fermi-Dirac …","Equilibrium number density of massless Fermi-Dirac …"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[null,null,null,null,null,null,[[["f64",15]],["f64",15]],null,[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["i32",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],null,[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],null,null,[[["i32",15],["f64",15]],["f64",15]],[[["i32",15],["f64",15]],["f64",15]],[[["i32",15],["f64",15]],["f64",15]],[[["i32",15],["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]],[[["f64",15]],["f64",15]]],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};