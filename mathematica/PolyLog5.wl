#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x < 1;
n = 5;

approx = PiecewiseMiniMax[
  PolyLog[n, x],
  {x, -Infinity, 1},
  "StartGuess" -> 0.5,
  "EndGuess" -> -10
];

DumpSave[
  FileBaseName[$InputFileName] <> ".mx",
  approx
];

output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/polylog/li" <> ToString[n] <> ".rs"
  }]];

lower = CoefficientList[approx[[2, 1, 1, 1]], Log[-x]];
lowerR = CoefficientList[lower[[1]], 1 / x];
lowerLn = {0} ~ Join ~ lower[[2;;]];
upper = CoefficientList[approx[[2, 1, 2, 1]], Log[1 - x]];
upperC = CoefficientList[upper[[1]], x];
upperLn = CoefficientList[upper[[2]], x];

approx[[2, 1, 1, 1]] = 0;
approx[[2, 1, 2, 1]] = 0;

WriteString[output,
  StringTemplate["use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    polynomial(
        x.recip(),
        &`lowerR`,
    ) + polynomial(
        (-x).ln(),
        &`lowerLn`,
    )
}

pub fn upper(x: f64) -> f64 {
    polynomial(
        x,
        &`upperR`,
    ) + (1.0 - x).ln() * polynomial(
        x,
        &`upperLn`,
    )
}

"][<|
  "n" -> 0,
  "lowerR" -> ToRustList[lowerC],
  "lowerLn" -> ToRustList[lowerLn],
  "upperR" -> ToRustList[upperR],
  "upperLn" -> ToRustList[upperLn]
  |>]
]

ApproximationToRust[approx, output];
Close[output];
