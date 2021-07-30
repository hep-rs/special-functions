#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x > 0;

nRange = ToExpression /@ Rest @ $ScriptCommandLine;
If[nRange === {},
   nRange = Range[0, 9];
];

Do[
  Print[StringTemplate["Approximating BesselJ[``, x]"][n]];
  f[x_] := BesselJ[n, x];
  output = OpenWrite[FileNameJoin[{
    Directory[],
    "../src/bessel/j" <> ToString[n] <> ".rs"
    }]];

  WriteString[
    output,
    "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
  ];

  (* Find the series approximation for small x *)
  lower = Normal@Series[x^(-n) f[x], {x, 0, 10}];
  xLower = x /. FindRoot[
    Abs[x^n lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 2, 0, Infinity},
    WorkingPrecision -> 4 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

  (* Write out the approximation valid for small x. *)
  lower = CoefficientList[lower, x^2];
  WriteString[
    output,
    StringTemplate["pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(`n`)
        * polynomial(
            x2,
            &`lower`,
          )
}\n\n"][<|
    "n" -> n,
    "lower" -> RustForm@lower
    |>]
  ];


  (* Find the series approximation for large x *)
  (* We split this using the amplitude and phase *)
  upperAmpSq = Normal@Series[BesselJ[n, x]^2 + BesselY[n, x]^2, {x, Infinity, 20}] // Simplify;
  upperPhase = (
    x 
    - (1/2 n + 1/4) Pi 
    + (mu - 1)/(2 (4 x)) 
    + ((mu - 1) (mu - 25))/(6 (4 x)^3) 
    + ((mu - 1) (mu^2 - 114 mu + 1073))/(5 (4 x)^5) 
    + ((mu - 1) (5 mu^3 - 1535 mu^2 + 54703 mu - 375733))/(14 (4 x)^7)
  ) /. {mu -> 4 n^2};
  upper = Sqrt[upperAmpSq] Cos[upperPhase];

  k = 1;
  xUpper = N[(BesselJZero[n, k] + BesselJZero[n, k + 1])/2, 100];
  While[
    Abs[upper/f[x] - 1] >
       SetPrecision[$MachineEpsilon/100, \[Infinity]] /. x -> xUpper,
    k++;
    xUpper = N[(BesselJZero[n, k] + BesselJZero[n, k + 1])/2, 100];
  ]
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

  (* Write out the approximation valid for large x. *)
  upperAmpSq = CoefficientList[x upperAmpSq, x^-2];
  upperPhase = CoefficientList[x (upperPhase - x + (1/2 n + 1/4) Pi), 1 / x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    let x1 = x.recip();
    let amp = x1 * polynomial(
      x.powi(-2),
      &`upperAmpSq`,
    );
    let amp = amp.sqrt();
    let phase = x - `const` + x1 * polynomial(
      x1,
      &`upperPhase`,
    );

    amp * phase.cos()
}\n\n"][<|
      "upperAmpSq" -> RustForm@upperAmpSq,
      "upperPhase" -> RustForm@upperPhase,
      "const" -> RustForm[(1/2 n + 1/4) Pi]
    |>]
  ];

  (* Subdivide the remaining interval using Chebyshev polynomials *)
  outer = Identity;
  inner = Identity;
  splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xLower, inner@xUpper}];
  ChebyshevSplitsRustForm[splits, output];

  Close[output];
,
  {n, nRange}
];
