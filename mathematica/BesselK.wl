#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x >= 0;

Do[
  Print[StringTemplate["Approximating BesselK[``, x]"][n]];
  f[x_] := BesselK[n, x];
  output = OpenWrite[FileNameJoin[{
    Directory[],
    "../src/bessel/k" <> ToString[n] <> ".rs"
    }]];
  pg = Which[
    n <= 3, 1,
    n <= 10, 1.5
       ] $MachinePrecision;

  WriteString[
    output,
    "use crate::approximations::polynomial;\n\n"
  ];

  (* Find the series approximation for small x *)
  lower = Normal@Series[f[x], {x, 0, 10}];
  xLower = x /. FindRoot[
    Abs[lower / f[x] - 1] == SetPrecision[$MachineEpsilon, Infinity],
    {x, 2, 0, Infinity},
    WorkingPrecision -> 5 $MachinePrecision,
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
    "lowerPoly" -> ToRustList@lowerPoly,
    "lowerLn" -> ToRustList@lowerLn
    |>]
  ];


  (* Find the series approximation for large x *)
  upper = Normal@Series[f[x], {x, Infinity, 10}];
  xUpper = x /. FindRoot[
    Abs[upper / f[x] - 1] == SetPrecision[$MachineEpsilon, Infinity],
    {x, 2, 0, Infinity},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
          ];
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

  (* Write out the approximation valid for large x. *)
  upper = CoefficientList[Exp[x] Sqrt[x] upper, 1/x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    (-x).exp()
        / x.sqrt()
        * polynomial(
            x.recip(),
            &`upper`
        )
}\n\n"][<|
    "upper" -> ToRustList@upper
    |>]
  ];


  (* Subdivide the remaining interval using Chebyshev polynomials *)
  splits = ChebyshevSplits[
    f[x], {x, xLower, xUpper},
    PrecisionGoal -> pg,
    AccuracyGoal -> Min[10^(-pg), GeometricMean[f[{xLower, xUpper}]]]];
  ChebyshevSplitsToRust[splits, output];

  Close[output];
,
  {n, Range[0, 9]}
];
