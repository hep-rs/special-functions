#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    polynomial(
        x.recip(),
        &[
            -1.644934066848226,
            -1.,
            -0.25,
            -0.1111111111111111,
            -0.0625,
            -0.04,
            -0.02777777777777778,
            -0.02040816326530612,
            -0.015625,
            -0.01234567901234568,
            -0.01,
        ],
    ) + polynomial((-x).ln(), &[0.0, 0.0, -0.5])
}

pub fn upper(x: f64) -> f64 {
    let xm1 = x - 1.0;
    let ln = (-xm1).ln();
    polynomial(
        xm1,
        &[
            1.644934066848226,
            1.,
            -0.25,
            0.1111111111111111,
            -0.0625,
            0.04,
            -0.02777777777777778,
            0.02040816326530612,
            -0.015625,
            0.01234567901234568,
            -0.01,
        ],
    ) + ln
        * polynomial(
            xm1,
            &[
                0.0,
                -1.,
                0.5,
                -0.3333333333333333,
                0.25,
                -0.2,
                0.1666666666666667,
                -0.1428571428571429,
                0.125,
                -0.1111111111111111,
                0.1,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 18] = [
    &[
        -4.791580453502824,
        0.4034029064897387,
        0.01004811191893422,
        0.0003854835808285281,
        0.00001761140939819351,
        8.856258976347919e-7,
        4.733941758571437e-8,
        2.640132800129175e-9,
        1.519211090305524e-10,
        8.954987020587302e-12,
        5.380417525203988e-13,
        3.283447838586828e-14,
        2.02985039896117e-15,
        1.268660041523397e-16,
        8.003730275419847e-18,
        5.09052940087417e-19,
        3.260749037529992e-20,
        2.10181628455374e-21,
        1.362369184883285e-22,
        8.874931294970478e-24,
        5.807521294556788e-25,
        3.815825088070624e-26,
        2.516513343076174e-27,
        1.665271782182745e-28,
        1.105414881122923e-29,
        7.358908877597407e-31,
        4.911965751987388e-32,
        3.286754853743767e-33,
        2.204364356786038e-34,
        1.48825887161851e-35,
        9.97728429899811e-37,
    ],
    &[
        -3.885108378291822,
        0.5090135540816245,
        0.01725446403807677,
        0.000911880874350004,
        0.00005769713996749619,
        4.030577837753324e-6,
        2.998948962024359e-7,
        2.331423411112768e-8,
        1.872093371776466e-9,
        1.541168999141724e-10,
        1.294098145205668e-11,
        1.104294596633086e-12,
        9.55038191312128e-14,
        8.353563265256679e-15,
        7.377899101780673e-16,
        6.571155112878192e-17,
        5.89581316080034e-18,
        5.324327725669994e-19,
        4.836083946880802e-20,
        4.41539113075771e-21,
        4.050126102565182e-22,
        3.730795617059635e-23,
        3.449875594141907e-24,
        3.20133720318419e-25,
        2.980301531422959e-26,
        2.782784304337554e-27,
        2.605504700400471e-28,
        2.445742082302779e-29,
        2.301400464741808e-30,
        2.189432843213739e-31,
        2.050529459760305e-32,
    ],
    &[
        -3.043882111578073,
        0.3211781742626747,
        0.007455935164071402,
        0.0002736059132375112,
        0.00001209305648977082,
        5.921456654613077e-7,
        3.095109071509633e-8,
        1.692990824375196e-9,
        9.5762124388301e-11,
        5.558280572283072e-12,
        3.29300518240517e-13,
        1.983801531409937e-14,
        1.211802340038252e-15,
        7.489571706494815e-17,
        4.675654025737461e-18,
        2.944456996947312e-19,
        1.868415759358443e-20,
        1.193601774689437e-21,
        7.670794581965685e-23,
        4.956154090351683e-24,
        3.217675669385355e-25,
        2.09814372179226e-26,
        1.373577429540106e-27,
        9.02500988352504e-29,
        5.949617554462249e-30,
        3.93425920189835e-31,
        2.608965375779296e-32,
        1.734663282997537e-33,
        1.156197071232311e-34,
        7.758289923367932e-36,
        5.170603440009389e-37,
    ],
    &[
        -2.328329890873959,
        0.3985280401564155,
        0.01248868349213409,
        0.0006283419055895877,
        0.00003833691930220254,
        2.60132520030423e-6,
        1.888892842278576e-7,
        1.437822813912325e-8,
        1.133232877368149e-9,
        9.17413922758276e-11,
        7.586674891014633e-12,
        6.383536767594572e-13,
        5.449049005294816e-14,
        4.708218905780194e-15,
        4.110636606215192e-16,
        3.621367323265958e-17,
        3.215561385112148e-18,
        2.875139827367784e-19,
        2.586687960537141e-20,
        2.340077677700201e-21,
        2.127543385089862e-22,
        1.943048199919697e-23,
        1.78184049074472e-24,
        1.640137997124301e-25,
        1.514899161203824e-26,
        1.403655147844422e-27,
        1.304384793014452e-28,
        1.215421147136803e-29,
        1.135463919420927e-30,
        1.072470965346478e-31,
        9.97588589632202e-33,
    ],
    &[
        -1.674829728698535,
        0.2468902293511686,
        0.005251406911641563,
        0.0001825343765246162,
        7.752949642665112e-6,
        3.677819069662264e-7,
        1.872060540606372e-8,
        1.000782720683666e-9,
        5.547021404133601e-11,
        3.161211972411929e-12,
        1.841751179039932e-13,
        1.092461034735194e-14,
        6.57738214228623e-16,
        4.010150866853101e-17,
        2.471376300775224e-18,
        1.537299863641749e-19,
        9.64074751455395e-21,
        6.089437756639837e-22,
        3.870887074270718e-23,
        2.474676919234454e-24,
        1.590205622711627e-25,
        1.026603110898604e-26,
        6.655539146753057e-28,
        4.331478371514707e-29,
        2.828929589382148e-30,
        1.853611852688446e-31,
        1.218197993142378e-32,
        8.028293127881997e-34,
        5.304656858115112e-35,
        3.528834694050262e-36,
        2.332173713202445e-37,
    ],
    &[
        -1.13010312478616,
        0.3005585134583645,
        0.008547119338591007,
        0.0004051130616995626,
        0.00002366358075735321,
        1.551023706627009e-6,
        1.09404005510046e-7,
        8.120796986720411e-9,
        6.258633708903911e-10,
        4.964693216812655e-11,
        4.029405254024115e-12,
        3.331688786058937e-13,
        2.797582248285e-14,
        2.379819081513965e-15,
        2.047041008191372e-16,
        1.777777550064006e-17,
        1.556926854210742e-18,
        1.373616280932204e-19,
        1.219857115356741e-20,
        1.089673405948371e-21,
        9.78523078826996e-23,
        8.829043727289901e-24,
        8.000827534232745e-25,
        7.278979237090441e-26,
        6.646251628148142e-27,
        6.088741940696649e-28,
        5.59514410328241e-29,
        5.15619202626669e-30,
        4.764574587530092e-31,
        4.45098259248375e-32,
        4.096801459364849e-33,
    ],
    &[
        -0.6415521019800431,
        0.1823894491883166,
        0.003483506816372934,
        0.0001133937199190875,
        4.591824504488745e-6,
        2.096984556381412e-7,
        1.033735331800989e-8,
        5.373433490534002e-10,
        2.904187507951256e-11,
        1.617256231051136e-12,
        9.22164051274233e-14,
        5.360136664422127e-15,
        3.165536013673687e-16,
        1.894653477459288e-17,
        1.147022509387232e-18,
        7.01289036165891e-20,
        4.324724235345712e-21,
        2.687249426218469e-22,
        1.681023785405091e-23,
        1.057898539047802e-24,
        6.693511914747807e-26,
        4.255767602868969e-27,
        2.717821154199976e-28,
        1.742665295668296e-29,
        1.121526818593129e-30,
        7.242329570726137e-32,
        4.691431174567905e-33,
        3.047819640392234e-34,
        1.985396737918559e-35,
        1.30207255210259e-36,
        8.486362803998519e-38,
    ],
    &[
        -0.2434271798612358,
        0.2173923078690536,
        0.005487907890025648,
        0.0002419803885865869,
        0.00001340941773079902,
        8.425027945196708e-7,
        5.732131154030368e-8,
        4.120802339557029e-9,
        3.084542249664409e-10,
        2.381339604756168e-11,
        1.883885557928561e-12,
        1.520114778419734e-13,
        1.24680310275239e-14,
        1.036780407378745e-15,
        8.722922798459337e-17,
        7.413519850262899e-18,
        6.356380918680248e-19,
        5.492329646779807e-20,
        4.77839068889806e-21,
        4.182771911192461e-22,
        3.68156533603368e-23,
        3.256526624570323e-24,
        2.893548907567969e-25,
        2.581593970526787e-26,
        2.311930948792247e-27,
        2.077585662440053e-28,
        1.872936702734529e-29,
        1.693416257542575e-30,
        1.535385935271407e-31,
        1.40705233086689e-32,
        1.271345619995969e-33,
    ],
    &[
        0.1065815070590063,
        0.1289685448399565,
        0.002158867793815236,
        0.0000648740881337927,
        2.477024400113862e-6,
        1.078215109717211e-7,
        5.098354863454114e-9,
        2.552299280035175e-10,
        1.332135320513352e-11,
        7.177742897494193e-13,
        3.965721953141195e-14,
        2.235964060154002e-15,
        1.281964757420754e-16,
        7.453972327100211e-18,
        4.386239350361842e-19,
        2.607778420109228e-20,
        1.564384438298253e-21,
        9.45882211221756e-23,
        5.759160644647617e-24,
        3.528429786350662e-25,
        2.173832999415399e-26,
        1.346034838621907e-27,
        8.372747983275722e-29,
        5.22979599181529e-30,
        3.279083665580566e-31,
        2.063175536011027e-32,
        1.302316765891366e-33,
        8.244954064263373e-35,
        5.234365544701374e-36,
        3.345079138251419e-37,
        2.125480088788872e-38,
    ],
    &[
        0.3848759307281073,
        0.150251406813008,
        0.003277565895055213,
        0.0001321741557089373,
        6.850734341302854e-6,
        4.070607223534045e-7,
        2.635561730940161e-8,
        1.810020561064075e-9,
        1.297611205754677e-10,
        9.61163217586792e-12,
        7.304808589173127e-13,
        5.66793130323858e-14,
        4.473609720900079e-15,
        3.581855665597422e-16,
        2.902973161782787e-17,
        2.377530285112706e-18,
        1.965007997482191e-19,
        1.637094404854316e-20,
        1.373579767528082e-21,
        1.159764727047818e-22,
        9.8477901496026e-24,
        8.404643787864135e-25,
        7.206164028819838e-26,
        6.20460004440032e-27,
        5.362802928167873e-28,
        4.65158695970909e-29,
        4.047816462584347e-30,
        3.533002225972174e-31,
        3.092434971669153e-32,
        2.73453021796479e-33,
        2.386861336963906e-34,
    ],
    &[
        0.6243421104469786,
        0.08700325900325788,
        0.001237996588719921,
        0.00003362587069639613,
        1.187840237558459e-6,
        4.836420530339742e-8,
        2.151918603401645e-9,
        1.017310451997989e-10,
        5.025664083527508e-12,
        2.567048972263591e-13,
        1.346017841851221e-14,
        7.208238467161513e-16,
        3.927745165320851e-17,
        2.17152362510549e-18,
        1.215463190945651e-19,
        6.875809941608893e-21,
        3.925609578484994e-22,
        2.259431843977483e-23,
        1.309765488167591e-24,
        7.64099777637295e-26,
        4.483135466184772e-27,
        2.643896394687169e-28,
        1.566490670884935e-29,
        9.32073875032711e-31,
        5.567417246860755e-32,
        3.337332373935648e-33,
        2.007082501758022e-34,
        1.210717618738044e-35,
        7.3238925092833e-37,
        4.458251019822619e-38,
        2.700433519437166e-39,
    ],
    &[
        0.8098224281672757,
        0.0989427877017795,
        0.001800329170311587,
        0.00006463525718962085,
        3.053722317547074e-6,
        1.671530088488727e-7,
        1.002494280325162e-8,
        6.397804257479915e-10,
        4.270695798117222e-11,
        2.949410000199827e-12,
        2.091863099391229e-13,
        1.515746795087702e-14,
        1.11777696169863e-15,
        8.36500973151765e-17,
        6.33859142858727e-18,
        4.854793326035304e-19,
        3.753079532751842e-20,
        2.925126654376714e-21,
        2.296304226867029e-22,
        1.814253353444839e-23,
        1.441650273712367e-24,
        1.151511385022528e-25,
        9.24082496630405e-27,
        7.447388165162525e-28,
        6.02543301952851e-29,
        4.892419245839956e-30,
        3.985538152999929e-31,
        3.256635886191688e-32,
        2.668658236991647e-33,
        2.207262857299939e-34,
        1.805542609588706e-35,
    ],
    &[
        0.96585094987502,
        0.0558506179265253,
        0.0006479709621058301,
        0.00001535852206758575,
        4.846045170915813e-7,
        1.779894991174473e-8,
        7.179129401476833e-10,
        3.085104956053899e-11,
        1.387744741860198e-12,
        6.461377553242897e-14,
        3.090596997460816e-15,
        1.510615372941353e-16,
        7.51575962708861e-18,
        3.795151396385183e-19,
        1.940627983215441e-20,
        1.003090545017125e-21,
        5.233634754632174e-23,
        2.753138182444435e-24,
        1.458807871369685e-25,
        7.779767927400368e-27,
        4.172929308625728e-28,
        2.249946799457915e-29,
        1.218839881074528e-30,
        6.631008161978341e-32,
        3.621681366074048e-33,
        1.985171080745652e-34,
        1.091738750931359e-35,
        6.022307121494887e-37,
        3.331478498086816e-38,
        1.853467376991493e-39,
        1.027331179816972e-40,
    ],
    &[
        1.083435469916309,
        0.06193849971748418,
        0.000894678646343413,
        0.00002731974385625241,
        1.122558548843288e-6,
        5.391840727128605e-8,
        2.849642161401276e-9,
        1.606281820632331e-10,
        9.48341552521466e-12,
        5.797696579409595e-13,
        3.642197925156734e-14,
        2.338553142411917e-15,
        1.52861379482603e-16,
        1.014215290616413e-17,
        6.814811359975579e-19,
        4.629021356158595e-20,
        3.174033875057673e-21,
        2.194384892523159e-22,
        1.528178168001922e-23,
        1.071138601651063e-24,
        7.551501721438962e-26,
        5.351634287034228e-27,
        3.810571302846038e-28,
        2.724950555590885e-29,
        1.956277530401941e-30,
        1.409497062085562e-31,
        1.018908600873169e-32,
        7.388112330913628e-34,
        5.372460233688527e-35,
        3.937754359718074e-36,
        2.86260751184017e-37,
    ],
    &[
        1.215799704460743,
        0.07083014401686696,
        0.001384427084416346,
        0.00006018144978540941,
        3.573031849787234e-6,
        2.493268770828111e-7,
        1.919040736325762e-8,
        1.577348467976204e-9,
        1.358944818773114e-10,
        1.212897684762614e-11,
        1.112750332646354e-12,
        1.043615758475933e-13,
        9.96594380091965e-15,
        9.66115594598616e-16,
        9.48571881616923e-17,
        9.41572619983761e-18,
        9.43515426770602e-19,
        9.5332947084666e-20,
        9.70316443750524e-21,
        9.94049315227095e-22,
        1.024306553804436e-22,
        1.061028852744455e-23,
        1.104290594841878e-24,
        1.154281273909384e-25,
        1.211293860792242e-26,
        1.27571818240031e-27,
        1.348038080757025e-28,
        1.428833514286419e-29,
        1.518971306862758e-30,
        1.637155101179803e-31,
        1.729308473723172e-32,
    ],
    &[
        1.328084018150231,
        0.04053840645214118,
        0.000549493114726029,
        0.00001754234029933334,
        7.761620083213169e-7,
        4.055277460743973e-8,
        2.341543788961636e-9,
        1.445153089505041e-10,
        9.35354252609981e-12,
        6.273626276820248e-13,
        4.326107768090813e-14,
        3.050022032854226e-15,
        2.189698387155514e-16,
        1.595983216933318e-17,
        1.178217074857838e-18,
        8.793910724225455e-20,
        6.62619225648448e-21,
        5.034493103871908e-22,
        3.853298811221616e-23,
        2.968527571625113e-24,
        2.300292379170478e-25,
        1.791865158778786e-26,
        1.402461296071779e-27,
        1.102433130176371e-28,
        8.700143594004022e-30,
        6.89081723073956e-31,
        5.475952253849957e-32,
        4.364979872306391e-33,
        3.489455103085498e-34,
        2.814780263301274e-35,
        2.247062149124007e-36,
    ],
    &[
        1.39118378119706,
        0.022179595145703,
        0.0001899799118182836,
        4.0006387154288e-6,
        1.179063618155692e-7,
        4.115164507047955e-9,
        1.588993541746544e-10,
        6.561532971011157e-12,
        2.842204893715196e-13,
        1.276009692410759e-14,
        5.89022553599941e-16,
        2.780134048322319e-17,
        1.336273814554484e-18,
        6.520825110297546e-20,
        3.223098502657243e-21,
        1.610692899001166e-22,
        8.126128963676548e-24,
        4.133992036351446e-25,
        2.118580654138586e-26,
        1.092836741073525e-27,
        5.670247241914173e-29,
        2.957545530550822e-30,
        1.549980347502647e-31,
        8.158277842095512e-33,
        4.311070033739055e-34,
        2.286352050315642e-35,
        1.216597351749868e-36,
        6.493594782457822e-38,
        3.475864515492947e-39,
        1.870849294730402e-40,
        1.003622681264489e-41,
    ],
    &[
        1.437242692950581,
        0.02393154767734067,
        0.0002529252970845598,
        6.841426056404361e-6,
        2.612114234287905e-7,
        1.183909168239826e-8,
        5.941880628424985e-10,
        3.190534989016623e-11,
        1.797509877707338e-12,
        1.049756823354463e-13,
        6.304130171018322e-15,
        3.871192091721851e-16,
        2.420915709545644e-17,
        1.53711521570156e-18,
        9.88571524365229e-20,
        6.428156991949504e-21,
        4.219912610349647e-22,
        2.793449948444866e-23,
        1.862825935647421e-24,
        1.250380847915533e-25,
        8.442127526164319e-27,
        5.729898045697105e-28,
        3.907594235035307e-29,
        2.676403130582374e-30,
        1.84038879825064e-31,
        1.270107714881969e-32,
        8.794650120368544e-34,
        6.108468276487133e-35,
        4.254937458650786e-36,
        2.986138192327318e-37,
        2.080412503862483e-38,
    ],
];

pub const SPLITS: [f64; 19] = [
    -14.67338716866669,
    -10.7659222474645,
    -6.858457326262302,
    -4.904724865661205,
    -2.950992405060108,
    -1.974126174759559,
    -0.997259944459011,
    -0.5088268293087362,
    -0.02039371415846185,
    0.2238228434166753,
    0.4680394009918125,
    0.5901476797793811,
    0.7122559585669496,
    0.7733100979607339,
    0.8343642373545182,
    0.8954183767483025,
    0.925945446445195,
    0.941208981293641,
    0.956472516142087,
];