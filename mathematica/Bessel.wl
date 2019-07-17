#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Bessel K Functions *)

$Assumptions = x >= 0;

approx = PiecewiseApproximate[
  BesselK[0, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k0.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[1, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k1.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[2, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k2.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[3, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k3.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[4, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k4.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[5, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k5.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[6, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k6.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[7, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k7.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[8, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k8.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];


approx = PiecewiseApproximate[
  BesselK[9, x],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
         ];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/k9.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];
