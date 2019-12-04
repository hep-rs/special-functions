#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x > 0;

nRange = ToExpression /@ Rest @ $ScriptCommandLine;
If[nRange === {},
   nRange = Range[0, 9];
];

Do[
  Print[StringTemplate["Approximating BesselY[``, x]"][n]];
  f[x_] := BesselY[n, x];
  output = OpenWrite[FileNameJoin[{
    Directory[],
    "../src/bessel/y" <> ToString[n] <> ".rs"
    }]];

  WriteString[
    output,
    "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
  ];

  (* Find the series approximation for small x *)
  lower = Normal@Series[f[x], {x, 0, 10}];
  xLower = x /. FindRoot[
    Abs[lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 2, 0, Infinity},
    WorkingPrecision -> 4 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

  (* Write out the approximation valid for small x. *)
  {lowerPoly, lowerLn} = CoefficientList[lower, Log[x]];
  lowerPoly = CoefficientList[x^n lowerPoly, x^2];
  lowerLn = CoefficientList[x^(-n) lowerLn, x^2];
  WriteString[
    output,
    StringTemplate["pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-`n`)
        * polynomial(
            x2,
            &`lowerPoly`,
          )
    + x.powi(`n`)
        * x.ln()
        * polynomial(
            x2,
            &`lowerLn`,
          )
}\n\n"][<|
    "n" -> n,
    "lowerPoly" -> RustForm@lowerPoly,
    "lowerLn" -> RustForm@lowerLn
    |>]
  ];


  (* Find the series approximation for large x *)
  upper = Normal@Series[f[x], {x, Infinity, 10}] // ExpToTrig;
  k = 1;
  xUpper = N[(BesselYZero[n, k] + BesselYZero[n, k + 1])/2, 100];
  While[
    Abs[upper/f[x] - 1] >
       SetPrecision[$MachineEpsilon/100, \[Infinity]] /. x -> xUpper,
    k++;
    xUpper = N[(BesselYZero[n, k] + BesselYZero[n, k + 1])/2, 100];
  ]
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

  (* Write out the approximation valid for large x. *)
  upper = CoefficientList[Sqrt[x] upper, 1/x];
  (* For some reason, these should be cos(x) and sin(x) *)
  upperSin = Coefficient[upper, Sin[x]];
  upperCos = Coefficient[upper, Cos[x]];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    let (sin, cos) = x.sin_cos();
    let xr = x.recip();

    (sin * polynomial(xr, &`upperSin`)
    + cos * polynomial(xr, &`upperCos`)) / x.sqrt()
}\n\n"][<|
    "upperSin" -> RustForm@upperSin,
    "upperCos" -> RustForm@upperCos
    |>]
  ];


  (* Subdivide the remaining interval using Chebyshev polynomials *)
  splits = ChebyshevSplits[f[x], {x, xLower, xUpper}, MaxRecursion -> 12];
  ChebyshevSplitsRustForm[splits, output];

  Close[output];
,
  {n, nRange}
];
