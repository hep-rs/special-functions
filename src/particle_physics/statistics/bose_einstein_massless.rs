#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    polynomial(
        x.exp(),
        &[
            0.0,
            0.1013211836423378,
            0.01266514795529222,
            0.003752636431197695,
            0.001583143494411528,
            0.0008105694691387022,
            0.0004690795538997119,
            0.0002953970368581276,
            0.000197892936801441,
            0.0001389865344888035,
            0.0001013211836423378,
            0.0000761241049153552,
        ],
    )
}

pub fn upper(x: f64) -> f64 {
    polynomial(x, &[0.0, 0.3333333333333333, 0.0, -0.01688686394038963])
        + polynomial(
            (-x).exp(),
            &[
                0.0,
                0.1013211836423378,
                0.01266514795529222,
                0.003752636431197695,
                0.001583143494411528,
                0.0008105694691387022,
                0.0004690795538997119,
                0.0002953970368581276,
                0.000197892936801441,
                0.0001389865344888035,
                0.0001013211836423378,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 16] = [
    &[
        -4.238636187078979,
        0.6468911601606184,
        0.002120752631333323,
        0.0002577720970905982,
        0.00002675811767157186,
        2.686579650874516e-6,
        2.816192834902391e-7,
        3.143733388900148e-8,
        3.695091892357734e-9,
        4.509670963090246e-10,
        5.666095250106699e-11,
        7.29089784527631e-12,
        9.57160726896584e-13,
        1.278170280601661e-13,
        1.732020719677517e-14,
        2.377094344990602e-15,
        3.299083481858164e-16,
        4.624199728174102e-17,
        6.538992691058768e-18,
        9.32015754234383e-19,
        1.337947471099227e-19,
        1.933172793277579e-20,
        2.809759105489218e-21,
        4.105983916836233e-22,
        6.030103297611586e-23,
        8.896591723579336e-24,
        1.318150951702945e-24,
        1.960745496773409e-25,
        2.928741820965062e-26,
        4.484492088878301e-27,
        6.590212985082152e-28,
    ],
    &[
        -3.255961707116028,
        0.3349934726882875,
        0.001747515012936469,
        0.0001392997360448565,
        0.00001111344119040012,
        9.74558649933432e-7,
        9.42665324563973e-8,
        9.81463357066919e-9,
        1.078635879097427e-9,
        1.235852265033612e-10,
        1.463847428578472e-11,
        1.781648273303395e-12,
        2.218042209694748e-13,
        2.81464817786211e-14,
        3.630754494057366e-15,
        4.750496710802674e-16,
        6.29332416237381e-17,
        8.429193251946505e-18,
        1.14006202575689e-18,
        1.555473822276789e-19,
        2.138999095410237e-20,
        2.962429668947784e-21,
        4.129477271903626e-22,
        5.790394943241211e-23,
        8.16345944213455e-24,
        1.156659776325327e-24,
        1.646404081796972e-25,
        2.353546474864026e-26,
        3.379183518453422e-27,
        4.967454631746493e-28,
        7.03273435406581e-29,
    ],
    &[
        -2.744769844080575,
        0.1752396064767339,
        0.00099320058827042,
        0.00005641122940840314,
        3.696749004430093e-6,
        2.844448450529248e-7,
        2.46209850950423e-8,
        2.315072967351038e-9,
        2.313852150919601e-10,
        2.423747839491363e-11,
        2.635075549958197e-12,
        2.952658703500394e-13,
        3.392269663329226e-14,
        3.980173263081815e-15,
        4.754493436352411e-16,
        5.768046492114098e-17,
        7.092708424683426e-18,
        8.825651457120101e-19,
        1.10980256345931e-19,
        1.408693787319569e-20,
        1.803193861941804e-21,
        2.325771132304124e-22,
        3.020532972365931e-23,
        3.947537996266155e-24,
        5.188754621883876e-25,
        6.8563080672289e-26,
        9.10391419611256e-27,
        1.214282343237115e-27,
        1.62689647720737e-28,
        2.226682469000393e-29,
        2.95081921111714e-30,
    ],
    &[
        -2.382914897340081,
        0.1878103052010214,
        0.00248087410585498,
        0.0002656868899608372,
        0.00003951457902453669,
        7.429374205920648e-6,
        1.626923488327804e-6,
        3.955917482593293e-7,
        1.037704661189917e-7,
        2.882560340997034e-8,
        8.373065683878481e-9,
        2.520682873103271e-9,
        7.813501906545371e-10,
        2.481620136976687e-10,
        8.045354967403075e-11,
        2.654516555642878e-11,
        8.892516868343975e-12,
        3.018742233214868e-12,
        1.036814761814367e-12,
        3.598135318378223e-13,
        1.260299028513605e-13,
        4.451240269694284e-14,
        1.583982335248372e-14,
        5.675190345042104e-15,
        2.046028401975634e-15,
        7.418726231331119e-16,
        2.704788523994971e-16,
        9.92831082394919e-17,
        3.712126300866708e-17,
        1.531322423255582e-17,
        4.98981730458936e-18,
    ],
    &[
        -2.166888922813937,
        0.02552289703909353,
        0.0001067408077535758,
        4.096654231279706e-6,
        2.792968776588237e-7,
        2.628510054326706e-8,
        2.990447450140879e-9,
        3.845620379608367e-10,
        5.384559994001336e-11,
        8.024751979754259e-12,
        1.254337774630653e-12,
        2.035738544995595e-13,
        3.405888203511693e-14,
        5.842966662513443e-15,
        1.02371791270212e-15,
        1.826042261288145e-16,
        3.30786060778514e-17,
        6.073247856701841e-18,
        1.128293937276492e-18,
        2.118181017678944e-19,
        4.013758236148333e-20,
        7.669558691272372e-21,
        1.476608509308963e-21,
        2.862402340141011e-22,
        5.583437285917837e-23,
        1.095336350125916e-23,
        2.160064885301956e-24,
        4.280615233897434e-25,
        8.533612356741941e-26,
        1.771605159716673e-26,
        3.416464424517895e-27,
    ],
    &[
        -2.128142562347732,
        0.01315349173528006,
        0.00004278018272304613,
        1.742120605981182e-6,
        1.540324936944228e-7,
        2.003177521205898e-8,
        3.220840494069629e-9,
        5.910598220283497e-10,
        1.186878157424424e-10,
        2.544120099184055e-11,
        5.730271064095029e-12,
        1.34179572895136e-12,
        3.241842852061519e-13,
        8.036830105671313e-14,
        2.035849595047422e-14,
        5.252500870168084e-15,
        1.376680645153279e-15,
        3.658072042598755e-16,
        9.83769323952554e-17,
        2.673947293854458e-17,
        7.337133516490255e-18,
        2.030429354410591e-18,
        5.662054546681532e-19,
        1.589911250795258e-19,
        4.492772453757465e-20,
        1.276923266537951e-20,
        3.648672772574485e-21,
        1.048204250157562e-21,
        3.043138097412859e-22,
        9.49330097847296e-23,
        2.54435293609891e-23,
    ],
    &[
        -2.111596836702905,
        0.003351586493065303,
        4.15798178392795e-6,
        1.127628808411709e-7,
        8.576798397937026e-9,
        1.008039509675399e-9,
        1.48042597128512e-10,
        2.489169180180937e-11,
        4.584912144796281e-12,
        9.01938344350122e-13,
        1.864782365670625e-13,
        4.008683756595262e-14,
        8.891882474497647e-15,
        2.023877170744343e-15,
        4.707031667183441e-16,
        1.114988523122511e-16,
        2.683113643134039e-17,
        6.545719856392828e-18,
        1.616199311552258e-18,
        4.033183092834746e-19,
        1.016042596444774e-19,
        2.581432612716352e-20,
        6.608935586674934e-21,
        1.703774875166331e-21,
        4.420109154585978e-22,
        1.153345337828048e-22,
        3.025479788670868e-23,
        7.977795685711424e-24,
        2.12244333970893e-24,
        6.016285420395663e-25,
        1.498745703897916e-25,
    ],
    &[
        -2.107397621517516,
        0.000843689035039963,
        3.462305840857365e-7,
        5.675669326882202e-9,
        3.218824574590763e-10,
        2.877649877065124e-11,
        3.219183348002628e-12,
        4.121324680300306e-13,
        5.776958913869453e-14,
        8.644225976303554e-15,
        1.358919620251588e-15,
        2.220510542005743e-16,
        3.743049166515028e-17,
        6.473099142443246e-18,
        1.143674801160381e-18,
        2.057767204284807e-19,
        3.760866084309299e-20,
        6.967696524034085e-21,
        1.306398932681605e-21,
        2.475419937118379e-22,
        4.734867892293908e-23,
        9.13334799810015e-24,
        1.775231524435731e-24,
        3.474347645598009e-25,
        6.842532254922068e-26,
        1.355352605787004e-26,
        2.698832908851311e-27,
        5.400503219046873e-28,
        1.087216395071463e-28,
        2.280833694616823e-29,
        4.438311039469221e-30,
    ],
    &[
        -2.105707109841783,
        0.0008469544966698815,
        5.084070738755901e-7,
        2.257539987852829e-8,
        -5.715354546215904e-9,
        -5.428133832318364e-9,
        -8.29874769336282e-10,
        1.441718192682342e-9,
        1.002969055781749e-9,
        -1.71062661615617e-10,
        -5.867709191808194e-10,
        -2.067262399570518e-10,
        2.281969798939889e-10,
        2.444558600307038e-10,
        -1.234340629078083e-11,
        -1.65092164578448e-10,
        -8.129088569853545e-11,
        6.902965829885871e-11,
        9.48206513837506e-11,
        2.125215701721482e-12,
        -6.836585489491516e-11,
        -3.855561010653326e-11,
        3.140615767248181e-11,
        4.659566016098521e-11,
        -5.230292804863582e-13,
        -3.803706502271845e-11,
        -1.881696080573438e-11,
        2.318028154825065e-11,
        2.804032555186863e-11,
        -7.606175434642508e-12,
        -1.528751174803836e-11,
    ],
    &[
        -2.103157156914652,
        0.001703858688013491,
        1.330832294333891e-6,
        -4.460332741147814e-8,
        4.606928565381541e-9,
        -7.925897867456973e-10,
        1.72893157144917e-10,
        -4.346226511421901e-11,
        1.201619429931093e-11,
        -3.557478528657777e-12,
        1.109039356318509e-12,
        -3.599877129768634e-13,
        1.207028816604236e-13,
        -4.156388004993908e-14,
        1.463468167306094e-14,
        -5.25111824696284e-15,
        1.914981177030899e-15,
        -7.082602117087371e-16,
        2.652041514664377e-16,
        -1.003927820706465e-16,
        3.837407649472855e-17,
        -1.479608453251241e-17,
        5.749826505798706e-18,
        -2.250303729593398e-18,
        8.864068545620851e-19,
        -3.512525200044773e-19,
        1.400152274277358e-19,
        -5.626598356439773e-20,
        2.314240622813729e-20,
        -1.06016357427341e-20,
        3.690055891042547e-21,
    ],
    &[
        -2.094572376309959,
        0.006893087601490832,
        0.00001277543771826433,
        -6.220235561437492e-7,
        5.009047130350629e-8,
        -6.825408125874659e-9,
        1.184733552983913e-9,
        -2.367357637181222e-10,
        5.195635940689355e-11,
        -1.219648702274568e-11,
        3.012030306127018e-12,
        -7.73928006410464e-13,
        2.052933464554589e-13,
        -5.589978588154501e-14,
        1.555765696634549e-14,
        -4.411018114895898e-15,
        1.270749697841914e-15,
        -3.711910522493242e-16,
        1.097512769272723e-16,
        -3.28007548160014e-17,
        9.89711694351506e-18,
        -3.011982606664717e-18,
        9.23735096290913e-19,
        -2.85284377930862e-19,
        8.866890967961555e-20,
        -2.772000536464586e-20,
        8.713049870243449e-21,
        -2.754593656607079e-21,
        8.822217739369468e-22,
        -3.06716517656032e-22,
        8.897019825116691e-23,
    ],
    &[
        -2.031291228751355,
        0.05654121374384726,
        0.0001050628750040332,
        -0.00005323137366999006,
        5.870728526289558e-6,
        -9.22686588936999e-7,
        1.954049971992705e-7,
        -4.889165372652617e-8,
        1.355647934608e-8,
        -4.037938528473134e-9,
        1.268704710668794e-9,
        -4.154985194093486e-10,
        1.406637086236504e-10,
        -4.893088883413474e-11,
        1.741056370205461e-11,
        -6.314861273724485e-12,
        2.328365978858503e-12,
        -8.708167806098276e-13,
        3.297763212062666e-13,
        -1.262681959609728e-13,
        4.882256324731351e-14,
        -1.90437712341119e-14,
        7.487066407063531e-15,
        -2.964641679389943e-15,
        1.181568817575569e-15,
        -4.737635289051677e-16,
        1.911035077112621e-16,
        -7.773021300856763e-17,
        3.238325850931209e-17,
        -1.504405898906824e-17,
        5.280970971724373e-18,
    ],
    &[
        -1.864477890642902,
        0.1090745011917796,
        -0.00122738480492792,
        -0.00007587486029114917,
        9.76955689662525e-6,
        -9.40352568956938e-7,
        9.89201824510286e-8,
        -1.235442839169331e-8,
        1.780906456917802e-9,
        -2.820159459416584e-10,
        4.747819264729246e-11,
        -8.350026610958255e-12,
        1.518988930656663e-12,
        -2.839886289432437e-13,
        5.431396614233169e-14,
        -1.058893032774627e-14,
        2.098561595259157e-15,
        -4.218542370718157e-16,
        8.586138914859863e-17,
        -1.766812354107244e-17,
        3.671212210271627e-18,
        -7.695007355902907e-19,
        1.625587538925107e-19,
        -3.458510713742171e-20,
        7.405688134516579e-21,
        -1.595129926710656e-21,
        3.45438207106748e-22,
        -7.518728814178631e-23,
        1.647555906025898e-23,
        -3.784813684052465e-24,
        7.954852133959313e-25,
    ],
    &[
        -1.568180720609998,
        0.1823061644699826,
        -0.006163887277220624,
        0.00006413124484717746,
        0.00001447015181989828,
        -2.389007592195535e-6,
        2.78724902518824e-7,
        -3.01553748308073e-8,
        3.311081310365106e-9,
        -3.87271727152127e-10,
        4.915720481885941e-11,
        -6.733541052665644e-12,
        9.78293142603096e-13,
        -1.481819116868844e-13,
        2.311524079067706e-14,
        -3.685630716531149e-15,
        5.98002825048588e-16,
        -9.84616099376522e-17,
        1.641969518438771e-17,
        -2.769181145713494e-18,
        4.717292294053649e-19,
        -8.108415977915181e-20,
        1.405047428484462e-20,
        -2.452565112612282e-21,
        4.30955516631478e-22,
        -7.618551299687276e-23,
        1.354311600649072e-23,
        -2.419857436711201e-24,
        4.348487026519704e-25,
        -8.089159121625219e-26,
        1.417677674803052e-26,
    ],
    &[
        -1.133130760898367,
        0.2409063474310451,
        -0.01713032671552588,
        0.0007583872865552459,
        -0.00003708336609801493,
        -4.454227739182469e-7,
        3.933919515835911e-7,
        -7.470177539953827e-8,
        1.100161385014526e-8,
        -1.472881985118176e-9,
        1.881073249191132e-10,
        -2.356048353952498e-11,
        2.947466000924227e-12,
        -3.737733482073589e-13,
        4.860252203097205e-14,
        -6.530046936298492e-15,
        9.09224885190917e-16,
        -1.310094541286919e-16,
        1.944773701648813e-17,
        -2.957280697811372e-18,
        4.581307571024059e-19,
        -7.197898989437046e-20,
        1.143137503863378e-20,
        -1.830941540438311e-21,
        2.953108821885396e-22,
        -4.791697470717022e-23,
        7.816730921200755e-24,
        -1.281447263145328e-24,
        2.111904029133797e-25,
        -3.586828104803379e-26,
        5.795809207086362e-27,
    ],
    &[
        -0.6871997921098742,
        0.1789304971786812,
        -0.04130274802961691,
        0.0008874510702134045,
        -0.0003053770039344563,
        5.170923116970493e-6,
        -2.539172581006876e-6,
        -3.491260456593553e-8,
        -1.5291367626151e-8,
        -2.39784957520186e-9,
        9.30242605835168e-11,
        -6.854102793244946e-11,
        6.45686076191229e-12,
        -1.597078805692983e-12,
        1.801578340574888e-13,
        -3.391332362007904e-14,
        4.115789485343787e-15,
        -6.854087652161098e-16,
        8.661643080935131e-17,
        -1.35324126738411e-17,
        1.762671455066869e-18,
        -2.666601922880619e-19,
        3.577805998176463e-20,
        -5.353256090863996e-21,
        7.423135513403866e-22,
        -1.115657018784469e-22,
        1.604021554069922e-23,
        -2.446307438346205e-24,
        3.648565667415339e-25,
        -5.80523867779974e-26,
        8.720176786310484e-27,
    ],
];

pub const SPLITS: [f64; 17] = [
    -2.603596108928818,
    -1.335039709540511,
    -0.700761509846357,
    -0.38362240999928,
    -0.06648331015220311,
    -0.0268409226713185,
    -0.007019728930876189,
    -0.002064430495765612,
    -0.0008256058869879677,
    0.0004132187217896766,
    0.002890867939344965,
    0.01280146480956612,
    0.0920862397713353,
    0.2506557896948738,
    0.5677948895419507,
    1.202073089236105,
    2.470629488624412,
];
