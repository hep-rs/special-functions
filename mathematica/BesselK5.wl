#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x >= 0;

approx = PiecewiseApproximate[
  BesselK[5, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/data/k5.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];
