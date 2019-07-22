use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-0)
        * polynomial(
            x2,
            &[
                2.,
                -0.5,
                0.10824143945730155,
                0.015964564399219575,
                0.0006209629499756117,
                0.000011796141758852788,
                1.3465023364738628e-7,
            ],
        )
        + x.powi(0)
            * x.ln()
            * polynomial(
                x2,
                &[
                    -0.125,
                    -0.010416666666666666,
                    -0.0003255208333333333,
                    -5.4253472222222224e-6,
                    -5.651403356481482e-8,
                ],
            )
}

pub fn upper(x: f64) -> f64 {
    (-x).exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                1.2533141373155001,
                2.349964007466563,
                1.0281092532666212,
                -0.38554096997498294,
                0.39758912528670115,
                -0.6460823285908894,
                1.4133050937925706,
                -3.8613514169689873,
                12.60972572103935,
                -47.8118766922742,
            ],
        )
}

pub const NUMERATORS: [&[f64]; 19] = [
    &[],
    &[
        -2.25231551642142e52,
        -1.4701992380953516e53,
        -2.622555639057652e53,
        -1.4157636784429578e53,
        6.82076160847012e51,
        1.499932036392716e52,
        -3.577040670755865e51,
        3.3467625317920507e50,
        -1.206896365632368e49,
    ],
    &[
        65844.85964326313,
        66630.61529919883,
        6326.156405638131,
        -8869.74062514484,
        1896.6081018507216,
        -200.4793551197249,
        12.030265768940092,
        -0.39791045713531586,
        0.005720137586055571,
    ],
    &[
        -123.50572174012353,
        -33.113404538901044,
        20.44311645171803,
        -3.7594389702684152,
        0.3791939310059777,
        -0.0240213463292324,
        0.000991635943334445,
        -0.000026181123685135734,
        4.050267005274029e-7,
        -2.813139056479694e-9,
    ],
    &[
        -0.0416552996123127,
        0.013706289376870068,
        -0.002045198625269591,
        0.00018156723164502637,
        -0.000010564005542524355,
        4.175358652370189e-7,
        -1.1204970520999936e-8,
        1.9676684131990227e-10,
        -2.0505833420825303e-12,
        9.656920110681498e-15,
    ],
    &[
        -3.740212967768205e-6,
        1.0311312582878947e-6,
        -1.2802872039586866e-7,
        9.393260026794066e-9,
        -4.486172702135361e-10,
        1.4458265715400473e-11,
        -3.1431961790023025e-13,
        4.443055970326649e-15,
        -3.704111645538571e-17,
        1.3871225332765666e-19,
    ],
    &[
        -1.3023657954772943e-9,
        3.070515138240452e-10,
        -3.247238821524792e-11,
        2.021261118812656e-12,
        -8.158597515793141e-14,
        2.2139660432850983e-15,
        -4.038066366360845e-17,
        4.772162111998586e-19,
        -3.315026061194236e-21,
        1.0310508715384958e-23,
    ],
    &[
        -8.780384434966079e-13,
        1.8025228433247678e-13,
        -1.6557255994832307e-14,
        8.92996263922338e-16,
        -3.115859498929458e-17,
        7.292635807074099e-19,
        -1.1446918396298838e-20,
        1.1617604141922692e-22,
        -6.916613195770812e-25,
        1.840113616893005e-27,
    ],
    &[
        -8.583280582012292e-16,
        1.5576153219904519e-16,
        -1.2626975392222977e-17,
        6.000774606680457e-19,
        -1.8421313043977003e-20,
        3.787682901298705e-22,
        -5.2156458161183554e-24,
        4.637382584109212e-26,
        -2.4155448894808823e-28,
        5.615410502970453e-31,
    ],
    &[
        -1.0557941139691711e-18,
        1.7149628827158555e-19,
        -1.2430208965989907e-20,
        5.275979900302552e-22,
        -1.4450442429120814e-23,
        2.648271603875872e-25,
        -3.2471639669014424e-27,
        2.5684437439914726e-29,
        -1.1891087997305757e-31,
        2.4548212195321467e-34,
    ],
    &[
        -1.5119529041228472e-21,
        2.221264426444244e-22,
        -1.4550125121870168e-23,
        5.5770140063191415e-25,
        -1.3783789419561985e-26,
        2.2778663637532227e-28,
        -2.5167960228771384e-30,
        1.7926768645069386e-32,
        -7.468987320325111e-35,
        1.3867489646457375e-37,
    ],
    &[
        -2.4070381227427096e-24,
        3.2264462121196844e-25,
        -1.9271621447391263e-26,
        6.73188289253104e-28,
        -1.5154809776355274e-29,
        2.2799667570159696e-31,
        -2.2921629307582565e-33,
        1.4848502324579342e-35,
        -5.623652845690233e-38,
        9.48703566835994e-41,
    ],
    &[
        -4.1376876294426085e-27,
        5.097944194299196e-28,
        -2.797659490272609e-29,
        8.975010416513172e-31,
        -1.8547792818124454e-32,
        2.5605934814260733e-34,
        -2.361350033027895e-36,
        1.4026130910719115e-38,
        -4.869190070957152e-41,
        7.526607653989277e-44,
    ],
    &[
        -7.533193798417909e-30,
        8.585534730680272e-31,
        -4.356838725302183e-32,
        1.2920335175203328e-33,
        -2.4674892135921053e-35,
        3.1469798239798277e-37,
        -2.68024213343799e-39,
        1.46989315484418e-41,
        -4.709951484838106e-44,
        6.718186183864046e-47,
    ],
    &[
        -1.4334181503622076e-32,
        1.5194759574545442e-33,
        -7.169939607048861e-35,
        1.9766208507307618e-36,
        -3.5083396750780734e-38,
        4.157491550501004e-40,
        -3.289271585849846e-42,
        1.6753281721325512e-44,
        -4.984503686795914e-47,
        6.600161427058747e-50,
    ],
    &[
        -2.8237800722616455e-35,
        2.7974465882011695e-36,
        -1.2333925350034875e-37,
        3.176415936931724e-39,
        -5.265705272017108e-41,
        5.826963476362356e-43,
        -4.304116650647723e-45,
        2.0463370095022183e-47,
        -5.68216577699097e-50,
        7.020775490879033e-53,
    ],
    &[
        -5.719389613039471e-38,
        5.317611309657196e-39,
        -2.1999724721580492e-40,
        5.31546843313009e-42,
        -8.265667831386566e-44,
        8.578507294834383e-46,
        -5.942019463808065e-48,
        2.648756962157978e-50,
        -6.894928318863672e-53,
        7.985267428461355e-56,
    ],
    &[
        8.505769025093498e-41,
        -8.121403894072935e-42,
        3.4933951769614415e-43,
        -8.91453768816311e-45,
        1.4944776368296463e-46,
        -1.7198296709883808e-48,
        1.3758573364883947e-50,
        -7.555294359984301e-53,
        2.7254489338619395e-55,
        -5.8319390360800704e-58,
        5.6211516798277754e-61,
    ],
    &[],
];

pub const DENOMINATORS: [&[f64]; 19] = [
    &[],
    &[
        1.,
        1.975603785115892e44,
        -1.126158587602902e52,
        -7.35097774504559e52,
        -1.3394636118828368e53,
        -8.910351161074386e52,
        -2.75434808996441e52,
        -5.426089762967357e51,
        -6.103526539495797e50,
        -5.383829319447004e49,
    ],
    &[
        1.,
        -9.148082061401182,
        32964.98424825512,
        33176.43590816229,
        11778.707761851449,
        2685.7858548491863,
        373.0791166181916,
        43.40810880511884,
        2.6004924589424276,
        0.19862796501573074,
    ],
    &[
        1.,
        -2.714185081958397,
        -58.54154149696607,
        -18.147193127404776,
        -6.593219186330625,
        -0.687202670326942,
        -0.16470385043571792,
        -0.005025153202946227,
        -0.001565299855893754,
        1.6131248837393236e-6,
        -5.016856713848194e-6,
    ],
    &[
        1.,
        -1.0652567234483805,
        0.5008572922005184,
        -0.15078775551720308,
        0.028272787820495995,
        -0.0039641870421223335,
        0.000364153626888175,
        -0.000025386598033251623,
        1.023692631955274e-6,
        -2.8788919369668026e-8,
    ],
    &[
        1.,
        -0.6697319481040708,
        0.20290602182036604,
        -0.036590503648118625,
        0.0043417501368065185,
        -0.0003530314976689802,
        0.000019778595104617043,
        -7.427161865001979e-7,
        1.7173469023231015e-8,
        -1.9295792185763744e-10,
    ],
    &[
        1.,
        -0.48073769987918097,
        0.10397780374874487,
        -0.013298213058347717,
        0.0011101660043318833,
        -0.00006286620392904581,
        2.421182876971281e-6,
        -6.136878374281154e-8,
        9.335070583528294e-10,
        -6.544081433165184e-12,
    ],
    &[
        1.,
        -0.3716276042804808,
        0.061920003221350974,
        -0.006075963349177303,
        0.0003873147261559688,
        -0.000016651502934180603,
        4.83464900167989e-7,
        -9.156345832246007e-9,
        1.0285859863261301e-10,
        -5.236433794441115e-13,
    ],
    &[
        1.,
        -0.30148645303033655,
        0.04066014921702303,
        -0.0032211872939500023,
        0.00016529096091530716,
        -5.700871511209124e-6,
        1.322561756712039e-7,
        -1.9918636853546474e-9,
        1.7690078891075648e-11,
        -7.067783167261286e-14,
    ],
    &[
        1.,
        -0.2529665499788482,
        0.02858243017705279,
        -0.0018938497093992051,
        0.00008112481196851242,
        -2.3307222949193477e-6,
        4.4931497149954064e-8,
        -5.607412546950714e-10,
        4.113230689297245e-12,
        -1.3521052156205751e-14,
    ],
    &[
        1.,
        -0.21756792966119426,
        0.02112043094156028,
        -0.0012009268363783229,
        0.000044089634875784076,
        -1.0841015620482335e-6,
        1.785838406910601e-8,
        -1.901070496351609e-10,
        1.1871257449225196e-12,
        -3.314474956290072e-15,
    ],
    &[
        1.,
        -0.1906797304470851,
        0.016210311970445524,
        -0.0008065396871582419,
        0.000025886702117195542,
        -5.55925022865443e-7,
        7.989671609566508e-9,
        -7.411623558615081e-11,
        4.0278554980819467e-13,
        -9.772913392210915e-16,
    ],
    &[
        1.,
        -0.16960216464057656,
        0.01281741693132524,
        -0.0005665713896151623,
        0.000016145181563854556,
        -3.0762033446778505e-7,
        3.919500289815797e-9,
        -3.2207742073352625e-11,
        1.549088536391412e-13,
        -3.323193754987108e-16,
    ],
    &[
        1.,
        -0.15265713413059684,
        0.010379741549609845,
        -0.000412613575121267,
        0.000010568757369271911,
        -1.809101104956514e-7,
        2.0696830897204933e-9,
        -1.5261577330039814e-11,
        6.582669553422751e-14,
        -1.2655126122516328e-16,
    ],
    &[
        1.,
        -0.13875027892422798,
        0.008571887894734271,
        -0.0003094957369571765,
        7.1977244758078455e-6,
        -1.1182096568007186e-7,
        1.160570371078096e-9,
        -7.760335438103082e-12,
        3.033819227085137e-14,
        -5.2837149564982347e-17,
    ],
    &[
        1.,
        -0.12713921150512084,
        0.007195392655117418,
        -0.00023792812234949738,
        5.066099016727946e-6,
        -7.203711665683425e-8,
        6.840989245064419e-10,
        -4.184020299655713e-12,
        1.4955861626580098e-14,
        -2.3806876686013068e-17,
    ],
    &[
        1.,
        -0.1173034451698758,
        0.006123872731257991,
        -0.0001867508557388228,
        3.6663652143835335e-6,
        -4.805721721577669e-8,
        4.2058236430922783e-10,
        -2.3699520631215022e-12,
        7.802761506409511e-15,
        -1.1436710374534716e-17,
    ],
    &[
        1.,
        -0.12112617096586444,
        0.006610521735039851,
        -0.000214068011836038,
        4.5552758415227245e-6,
        -6.656018371064752e-8,
        6.763360000045966e-10,
        -4.719348713974291e-12,
        2.164302174868882e-14,
        -5.890828434640965e-17,
        7.22660415401485e-20,
    ],
    &[],
];

pub const SPLITS: [f64; 20] = [
    0.,
    0.2445452364809969,
    1.7467874147334395,
    4.751271771238325,
    10.760240484248095,
    16.769209197257865,
    22.778177910267633,
    28.787146623277405,
    34.79611533628717,
    40.805084049296944,
    46.814052762306716,
    52.82302147531649,
    58.83199018832625,
    64.84095890133602,
    70.8499276143458,
    76.85889632735557,
    82.86786504036533,
    88.87683375337511,
    96.38804464463732,
    std::f64::INFINITY,
];
