#!/usr/bin/env wolframscript

Needs["X`"];

(*********************************************************)
(* RustForm definitions *)
(*********************************************************)

ClearAll[RustForm];

RustForm::usage = "Convert certain Mathematica objects into Rust";
RustForm::unimplemented = "Unable to convert `` into Rust form.";
RustForm[x_] := (Message[RustForm::unimplemented, x];
   StringTemplate["RustForm[``]"][x]);


(* Define HeavisideTheta first so it takes precedence *)
RustForm[Times[HeavisideTheta[a_ - b_], y__]] := StringTemplate[
"(if `a` > `b` { `y` } else { 0.0 })"][<|
  "a" -> RustForm[a],
  "b" -> RustForm[b],
  "y" -> RustForm[Times[y]]
|>];
RustForm[Times[HeavisideTheta[x_], y__]] := StringTemplate[
"(if `x` > 0.0 { `y` } else { 0.0 })"][<|
  "x" -> RustForm[x],
  "y" -> RustForm[Times[y]]
|>];

RustForm[m0]   = "param.m0";
RustForm[m0^2] = "param.m0_2";
RustForm[m0^n_Integer?EvenQ] := StringTemplate["param.m0_2.powi(``)"][n / 2];
RustForm[m0^n_Integer?OddQ] := StringTemplate["param.m0.powi(``)"][n];
RustForm[m1]   = "param.m1";
RustForm[m1^2] = "param.m1_2";
RustForm[m1^n_Integer?EvenQ] := StringTemplate["param.m1_2.powi(``)"][n / 2];
RustForm[m1^n_Integer?OddQ] := StringTemplate["param.m1.powi(``)"][n];
RustForm[m2]   = "param.m2";
RustForm[m2^2] = "param.m2_2";
RustForm[m2^n_Integer?EvenQ] := StringTemplate["param.m2_2.powi(``)"][n / 2];
RustForm[m2^n_Integer?OddQ] := StringTemplate["param.m2.powi(``)"][n];
RustForm[m3]   = "param.m3";
RustForm[m3^2] = "param.m3_2";
RustForm[m3^n_Integer?EvenQ] := StringTemplate["param.m3_2.powi(``)"][n / 2];
RustForm[m3^n_Integer?OddQ] := StringTemplate["param.m3.powi(``)"][n];

RustForm[s1]  = "param.s1";
RustForm[s2]  = "param.s2";
RustForm[s3]  = "param.s3";
RustForm[s4]  = "param.s4";
RustForm[s12] = "param.s12";
RustForm[s23] = "param.s23";

RustForm[Sqrt[Kallen\[Lambda][s1, s12, s2]]] = "param.lambda_s12_sqrt";
RustForm[Kallen\[Lambda][s1, s12, s2]] := "param.lambda_s12_sqrt.powi(2)";
RustForm[Power[Kallen\[Lambda][s1, s12, s2], Rational[n_Integer, 2]]] := StringTemplate["param.lambda_s12_sqrt.powi(``)"][n];
RustForm[Power[Kallen\[Lambda][s1, s12, s2], n_Integer]] := StringTemplate["param.lambda_s12_sqrt.powi(``)"][2 n];

RustForm[Sqrt[Kallen\[Lambda][s1, s23, s4]]] = "param.lambda_s14_sqrt";
RustForm[Kallen\[Lambda][s1, s23, s4]] := "param.lambda_s14_sqrt.powi(2)";
RustForm[Power[Kallen\[Lambda][s1, s23, s4], Rational[n_Integer, 2]]] := StringTemplate["param.lambda_s14_sqrt.powi(``)"][n];
RustForm[Power[Kallen\[Lambda][s1, s23, s4], n_Integer]] := StringTemplate["param.lambda_s14_sqrt.powi(``)"][2 n];

RustForm[Sqrt[Kallen\[Lambda][s12, s3, s4]]] = "param.lambda_s34_sqrt";
RustForm[Kallen\[Lambda][s12, s3, s4]] := "param.lambda_s34_sqrt.powi(2)";
RustForm[Power[Kallen\[Lambda][s12, s3, s4], Rational[n_Integer, 2]]] := StringTemplate["param.lambda_s34_sqrt.powi(``)"][n];
RustForm[Power[Kallen\[Lambda][s12, s3, s4], n_Integer]] := StringTemplate["param.lambda_s34_sqrt.powi(``)"][2 n];

RustForm[Sqrt[Kallen\[Lambda][s2, s23, s3]]] = "param.lambda_s23_sqrt";
RustForm[Kallen\[Lambda][s2, s23, s3]] := "param.lambda_s23_sqrt.powi(2)";
RustForm[Power[Kallen\[Lambda][s2, s23, s3], Rational[n_Integer, 2]]] := StringTemplate["param.lambda_s23_sqrt.powi(``)"][n];
RustForm[Power[Kallen\[Lambda][s2, s23, s3], n_Integer]] := StringTemplate["param.lambda_s23_sqrt.powi(``)"][2 n];

RustForm[Sqrt[Kallen\[Lambda][m0^2, m1^2, s1]]]  = "param.lambda_m01_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m0^2, m2^2, s12]]] = "param.lambda_m02_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m0^2, m2^2, s2]]] = "param.lambda_m02_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m0^2, m3^2, s4]]]  = "param.lambda_m03_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m1^2, m2^2, s12]]]  = "param.lambda_m12_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m1^2, m2^2, s2]]]  = "param.lambda_m12_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m1^2, m3^2, s23]]] = "param.lambda_m13_sqrt";
RustForm[Sqrt[Kallen\[Lambda][m2^2, m3^2, s3]]]  = "param.lambda_m23_sqrt";

RustForm[Infinity] = "std::f64::INFINITY";
RustForm[-Infinity] = "std::f64::NEG_INFINITY";
RustForm[ComplexInfinity] = "std::f64::INFINITY";
RustForm[0] := "0.0";

RustForm[n_?NumericQ] := StringReplace[
  ToString[CForm[N[n, $MachinePrecision]]],
  RegularExpression["(\\d)\\.e"] -> "$1e"
];
RustForm[l_List] := "[" <> StringRiffle[RustForm /@ l, ", "] <> "]";

RustForm[Plus[a_, b__]] := StringRiffle[
  If[MatchQ[_?AtomQ | _?NumericQ | _Power | _Times | _Log][#],
    RustForm[#],
    StringJoin["(", RustForm[#], ")"]
  ] & /@ {a, b}, " + "];

RustForm[Plus[a_, Times[-1, b_]]] := StringTemplate["`a` - `b`"][<|
  "a" -> If[MatchQ[_?AtomQ | _?NumericQ | _Power | _Times | _Plus | _Log][a],
    RustForm[a],
    StringJoin["(", RustForm[a], ")"]
  ],
  "b" -> If[MatchQ[_?AtomQ | _?NumericQ | _Power | _Times | _Log][b],
    RustForm[b],
    StringJoin["(", RustForm[b], ")"]
  ]
|>];

RustForm[Times[a_, b__]] := StringRiffle[
  If[MatchQ[_?AtomQ | _?NumericQ | _Power | _Log][#],
    RustForm[#],
    StringJoin["(", RustForm[#], ")"]
  ] & /@ {a, b}, " * "];

RustForm[Divide[a_, b_]] := StringTemplate["`a` / `b`"][<|
  "a" -> If[MatchQ[_?AtomQ | _?NumericQ | _Power | _Times | _Log][a],
    RustForm[a],
    StringJoin["(", RustForm[a], ")"]
  ],
  "b" -> If[MatchQ[_?AtomQ | _?NumericQ | _Power | _Log][b],
    RustForm[b],
    StringJoin["(", RustForm[b], ")"]
  ]
|>];

(* Handle integer powers and powers of square roots *)
RustForm[Power[a_, b_Integer]] := StringTemplate["`a`.powi(`b`)"][<|
  "a" -> If[MatchQ[_?AtomQ | _?NumericQ][a],
    RustForm[a],
    StringJoin["(", RustForm[a], ")"]
  ],
  "b" -> b
|>];
RustForm[Power[a_, Rational[1, 2]]] := StringTemplate["`a`.sqrt()"][<|
  "a" -> If[MatchQ[_?AtomQ | _?NumericQ][a],
    RustForm[a],
    StringJoin["(", RustForm[a], ")"]
  ]
|>];
RustForm[Power[a_, Rational[b_Integer, 2]]] := StringTemplate["`a`.sqrt().powi(`b`)"][<|
  "a" -> If[MatchQ[_?AtomQ | _?NumericQ][a],
    RustForm[a],
    StringJoin["(", RustForm[a], ")"]
  ],
  "b" -> b
|>];

(* Logarithms always appear in the form of Log[(a + b) / (a - b)] which is
   handled by `log_diff` *)
RustForm[Log[x_]] := StringTemplate["log_diff(`a`, `b`)"][<|
  "a" -> RustForm[Simplify[(Numerator[x] + Denominator[x]) / 2]],
  "b" -> RustForm[Simplify[(Numerator[x] - Denominator[x]) / 2]]
|>];

(*********************************************************)
(* C Function *)
(*********************************************************)

(* Define the logarithmic parts *)
(* The logarithmic parts take the form of `Log[(a + b)/(a - b)]` which is define
  to handle accurately when b << a. *)
Echo["Defining discPVC"];
$f = FileNameJoin[{$TemporaryDirectory, "discPVC.mx"}];

If[FileExistsQ[$f],
  Get[$f]
  ,
  Do[
    Echo[
      StringTemplate["-> Refining discPVC[`r`, `n1`, `n2`, ...]"][<|
        "r" -> r,
        "n1" -> n1,
        "n2" -> n2
      |>]
    ];

    With[{r = r, n1 = n1, n2 = n2},
      discPVC[r, n1, n2, s1_, s12_, s2_, m0_, m1_, m2_] = Sum[
        Simplify@Expand@LoopRefine[
          PVC[r, n1, n2, s1, s12, s2, m0, m1, m2]/(2 I),
          Part -> Discontinuity[s]],
        {s, {s1, s12, s2}}]
    ],
    {r, 0, 2},
    {n1, 0, 2},
    {n2, 0, 2}
  ];
  DumpSave[$f, discPVC]
];

Do[
  If[n2 > n1, Continue[]];

  Echo[StringTemplate["Outputting c`r``n1``n2`.rs"][<|
      "r" -> r,
      "n1" -> n1,
      "n2" -> n2
    |>]];

  $f = OpenWrite[
    StringTemplate[
      FileNameJoin[{
        DirectoryName[$InputFileName],
        "..",
        "src",
        "particle_physics",
        "pave_absorptive",
        "c",
        "explicit",
        "c`r``n1``n2`.rs"
      }]
    ][<|
      "r" -> r,
      "n1" -> n1,
      "n2" -> n2
    |>]
  ];

  WriteString[$f,
"use super::{log_diff, Parameters};

"];

  WriteString[$f,
    StringTemplate[
"pub(crate) fn c`r``n1``n2`(param: &Parameters) -> f64 {
    `disc`
}"][<|
        "r" -> r,
        "n1" -> n1,
        "n2" -> n2,
        "disc" -> RustForm[discPVC[r, n1, n2, s1, s12, s2, m0, m1, m2]]
      |>]
  ];

  Close[$f];
  ,
  {r, 0, 2},
  {n1, 0, 2},
  {n2, 0, 2}
]

(*********************************************************)
(* D Function *)
(*********************************************************)

Echo["Defining discPVD"];
$f = FileNameJoin[{$TemporaryDirectory, "discPVD.mx"}];

If[FileExistsQ[$f],
  Get[$f]
  ,
  Do[
    If[r + n1 + n2 + n3 > 1, Continue[]];

    Echo[
      StringTemplate["-> Refining discPVD[`r`, `n1`, `n2`, `n3`, ...]"][<|
        "r" -> r,
        "n1" -> n1,
        "n2" -> n2,
        "n3" -> n3
      |>]
    ];

    With[{r = r, n1 = n1, n2 = n2, n3 = n3},
      discPVD[r, n1, n2, n3, s1_, s2_, s3_, s4_, s12_, s23_, m0_, m1_, m2_, m3_] = Sum[
        Simplify@Expand@LoopRefine[
          PVD[r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3]/(2 I),
          Part -> Discontinuity[s]],
        {s, {s1, s2, s3, s4, s12, s23}}]
    ],
    {r, 0, 1},
    {n1, 0, 1},
    {n2, 0, 1},
    {n3, 0, 1}
  ];
  DumpSave[$f, discPVD]
];

Do[
  If[r + n1 + n2 + n3 > 1, Continue[]];

  Echo[StringTemplate["Outputting d`r``n1``n2``n3`.rs"][<|
      "r" -> r,
      "n1" -> n1,
      "n2" -> n2,
      "n3" -> n3
    |>]];

  $f = OpenWrite[
    StringTemplate[
      FileNameJoin[{
        DirectoryName[$InputFileName],
        "..",
        "src",
        "particle_physics",
        "pave_absorptive",
        "d",
        "explicit",
        "d`r``n1``n2``n3`.rs"
      }]
    ][<|
      "r" -> r,
      "n1" -> n1,
      "n2" -> n2,
      "n3" -> n3
    |>]
  ];

  WriteString[$f,
"use super::{log_diff, Parameters};

"];

  WriteString[$f,
    StringTemplate[
"pub(crate) fn d`r``n1``n2``n3`(param: &Parameters) -> f64 {
    `disc`
}"][<|
        "r" -> r,
        "n1" -> n1,
        "n2" -> n2,
        "n3" -> n3,
        "disc" -> RustForm[discPVD[r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3]]
      |>]
  ];

  Close[$f];
  ,
  {r, 0, 1},
  {n1, 0, 1},
  {n2, 0, 1},
  {n3, 0, 1}
]