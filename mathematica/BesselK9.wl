#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x >= 0;

approx = PiecewiseApproximate[
  BesselK[9, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
];

DumpSave[
  FileBaseName[$InputFileName] <> ".mx",
  approx
];

output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/bessel/k9.rs"
  }]];

n = 9;
lower = CoefficientList[approx[[2, 1, 1, 1]], Log[x]];
lowerC = CoefficientList[x^n lower[[1]], x^2];
lowerLn = CoefficientList[x^(-n) lower[[2]], x^2];
upper = CoefficientList[Exp[x] Sqrt[x] approx[[2, 1, 2, 1]], 1 / x];

approx[[2, 1, 1, 1]] = 0;
approx[[2, 1, 2, 1]] = 0;

WriteString[output,
  StringTemplate["use crate::polynomial::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-`n`)
        * polynomial(
            x2,
            &`lowerC`,
        )
        + x.powi(`n`)
            * x.ln()
            * polynomial(
                x2,
                &`lowerLn`,
            )
}

pub fn upper(x: f64) -> f64 {
    (-x).exp()
        / x.sqrt()
        * polynomial(
            x.recip(),
            &`upper`,
        )
}

"][<|
  "n" -> 0,
  "lowerC" -> ToRustList[lowerC],
  "lowerLn" -> ToRustList[lowerLn],
  "upper" -> ToRustList[upper]
  |>]
]

ApproximationToRust[approx, output];
Close[output];
