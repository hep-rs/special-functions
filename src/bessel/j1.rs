#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(1)
        * polynomial(
            x2,
            &[
                0.5,
                -0.0625,
                0.002604166666666667,
                -0.00005425347222222222,
                6.781684027777778e-7,
                -5.651403356481481e-9,
            ],
        )
}

pub fn upper(x: f64) -> f64 {
    let (sin, cos) = x.sin_cos();
    let x1 = x.recip();

    (cos * polynomial(
        x1,
        &[
            -0.5641895835477563,
            0.2115710938304086,
            -0.06611596682200269,
            -0.05785147096925235,
            0.08135363105051112,
            0.1566057397722339,
            -0.3817264906948202,
            -1.124729838654381,
            3.883832724103409,
            15.37350453290933,
        ],
    ) + sin
        * polynomial(
            x1,
            &[
                0.5641895835477563,
                0.2115710938304086,
                0.06611596682200269,
                -0.05785147096925235,
                -0.08135363105051112,
                0.1566057397722339,
                0.3817264906948202,
                -1.124729838654381,
                -3.883832724103409,
                15.37350453290933,
            ],
        ))
        / x.sqrt()
}

pub const COEFFICIENTS: [&[f64]; 32] = [
    &[
        0.375182995086779,
        0.00809299216988594,
        -0.1985295665167133,
        0.004790678662635996,
        0.007991112799673472,
        -0.0001743653462736176,
        -0.0001326241895189606,
        2.480537775215875e-6,
        1.207507683631741e-6,
        -1.951471685991793e-8,
        -6.957469741319945e-9,
        9.85561148215486e-11,
        2.767292453556513e-11,
        -3.482444065283416e-13,
        -8.058024368933629e-14,
        9.11317806159964e-16,
        1.792403964668342e-16,
        -1.839606613082286e-18,
        -3.145381622088273e-19,
        2.953876240819773e-21,
        4.466069994270235e-22,
        -3.864784728741113e-24,
        -5.236579797830611e-25,
        4.200968775635597e-27,
        5.156237561473226e-28,
        -3.854856173484174e-30,
        -4.324074417186897e-31,
        3.026404749984975e-33,
        3.125546528456843e-34,
        -2.054983360669699e-36,
        -1.96745655250198e-37,
    ],
    &[
        -0.1456644755794905,
        -0.2048888885809634,
        0.1557442868265319,
        0.01086268132264193,
        -0.007178592606324091,
        -0.0001740761444010901,
        0.0001258855997176014,
        1.337245318499798e-6,
        -1.178629578477505e-6,
        -5.737707554937952e-9,
        6.90102848804417e-9,
        1.382237911080999e-11,
        -2.772525848606016e-11,
        -1.201904577778224e-14,
        8.127207356762089e-14,
        -3.890219350399837e-17,
        -1.816177347849588e-16,
        1.867571211264276e-19,
        3.197742572911706e-19,
        -4.36657059998925e-22,
        -4.551659310703825e-22,
        7.157746905478515e-25,
        5.346976694954022e-25,
        -9.07411844323635e-28,
        -5.272622661016693e-28,
        9.32120984980167e-31,
        4.426767091256712e-31,
        -7.981621523818571e-34,
        -3.202726217404199e-34,
        5.807055663259976e-37,
        2.017548052860656e-37,
    ],
    &[
        0.100201822011371,
        0.2064267685914929,
        -0.1078500881157381,
        -0.01774683270777722,
        0.005655947506548829,
        0.0003990741476887951,
        -0.0001069547078791009,
        -4.248681723763829e-6,
        1.048249923834561e-6,
        2.680842258006263e-8,
        -6.321516534377999e-9,
        -1.128372675116809e-10,
        2.591259460015286e-11,
        3.408723951574322e-13,
        -7.70517564973587e-14,
        -7.769664062487889e-16,
        1.74008936388266e-16,
        1.385838087871872e-18,
        -3.088342481554376e-19,
        -1.988879541865284e-21,
        4.423323071718071e-22,
        2.347694142271772e-24,
        -5.221896592663472e-25,
        -2.32039983324464e-27,
        5.169839195345358e-28,
        1.948931655636293e-30,
        -4.354690041188807e-31,
        -1.408507155789279e-33,
        3.159162152200317e-34,
        8.847936457540857e-37,
        -1.994668278656622e-37,
    ],
    &[
        -0.07295123692271101,
        -0.2046195140575802,
        0.07888059334796994,
        0.01987824761416767,
        -0.004285065584216469,
        -0.0005125681054384414,
        0.0000853425869795118,
        6.085534420005707e-6,
        -8.728833417995192e-7,
        -4.191040978515914e-8,
        5.436551456965949e-9,
        1.897670124352887e-10,
        -2.283186954720626e-11,
        -6.106839830092736e-13,
        6.915548418941568e-14,
        1.472668095059178e-15,
        -1.584247701379925e-16,
        -2.765262380126296e-18,
        2.843624431651635e-19,
        4.16234496979274e-21,
        -4.109805689482212e-22,
        -5.138407683844184e-24,
        4.887625158845424e-25,
        5.299263080800332e-27,
        -4.868400289718194e-28,
        -4.635612488455292e-30,
        4.121668529417105e-31,
        3.483816298644744e-33,
        -3.002983221083145e-34,
        -2.27271491954385e-36,
        1.903023241895307e-37,
    ],
    &[
        0.0526911826185857,
        0.2016641042641759,
        -0.05778128752229234,
        -0.02065450613526676,
        0.003203803363241844,
        0.0005652102460066417,
        -0.00006567120669920671,
        -7.118908277305533e-6,
        6.93210824696181e-7,
        5.159386289899044e-8,
        -4.442373557635775e-9,
        -2.437596106229814e-10,
        1.911002893129678e-11,
        8.126773566436406e-13,
        -5.903742216040587e-14,
        -2.018877683694823e-15,
        1.374534771952761e-16,
        3.887970241473766e-18,
        -2.50029043592711e-19,
        -5.981293042237326e-21,
        3.653717953162528e-22,
        7.525831609003968e-24,
        -4.38552010486754e-25,
        -7.892997462317894e-27,
        4.402424350795111e-28,
        7.008785075991258e-30,
        -3.751950684833707e-31,
        -5.338810585166627e-33,
        2.749203932043851e-34,
        3.52561871255059e-36,
        -1.750790656782289e-37,
    ],
    &[
        -0.03613916513946174,
        -0.1973801723424414,
        0.04086137833203519,
        0.02079857789330536,
        -0.002315198183240472,
        -0.0005873658868427698,
        0.00004859562390987761,
        7.648439681250649e-6,
        -5.260731810797959e-7,
        -5.723818156259933e-8,
        3.455885252734758e-9,
        2.783249268773484e-10,
        -1.520730516488494e-11,
        -9.51402180144666e-13,
        4.792652688012861e-14,
        2.414714541849322e-15,
        -1.13520405422008e-16,
        -4.736397636416518e-18,
        2.095591876795509e-19,
        7.402273370556003e-21,
        -3.101176162941771e-22,
        -9.44142391582698e-24,
        3.762787009919657e-25,
        1.002000213316556e-26,
        -3.812653487856901e-28,
        -8.990284886229922e-30,
        3.27564846392568e-31,
        6.91112723459045e-33,
        -2.417125011442959e-34,
        -4.60115082734428e-36,
        1.548812975485373e-37,
    ],
    &[
        0.02195110005911863,
        0.191580322275056,
        -0.02655929394327337,
        -0.02054121461507097,
        0.001557732604487442,
        0.0005911956690093833,
        -0.00003368655239528113,
        -7.857668318845686e-6,
        3.748965423453232e-7,
        6.004107317963864e-8,
        -2.527202143171827e-9,
        -2.978386705436575e-10,
        1.138770468320223e-11,
        1.036935045064721e-12,
        -3.66630612399094e-14,
        -2.675320359038881e-15,
        8.849895496501662e-17,
        5.324101082907324e-18,
        -1.661046427081553e-19,
        -8.427161958578789e-21,
        2.494073624290855e-22,
        1.086899040334663e-23,
        -3.064835847995974e-25,
        -1.164829845093817e-26,
        3.140174818393561e-28,
        1.054149232943024e-29,
        -2.724357509958119e-31,
        -8.165374961819269e-33,
        2.027695248177194e-34,
        5.472914548799869e-36,
        -1.309207792030469e-37,
    ],
    &[
        -0.0094848952103205,
        -0.1841875161217421,
        0.01411770614537169,
        0.01997943258534995,
        -0.0008969212904872158,
        -0.0005822501500208126,
        0.00002052814658312037,
        7.84475964343765e-6,
        -2.389235202721744e-7,
        -6.079993078427296e-8,
        1.67183079669515e-9,
        3.059093591492421e-10,
        -7.77792884547062e-12,
        -1.079632929451567e-12,
        2.574451497291423e-14,
        2.82108604799712e-15,
        -6.36606044271762e-17,
        -5.679787842173337e-18,
        1.220279309120507e-19,
        9.08500602565086e-21,
        -1.866310041150601e-22,
        -1.182828642539223e-23,
        2.330724964089056e-25,
        1.278351008380473e-26,
        -2.422136999598589e-28,
        -1.165610794822562e-29,
        2.127857061986086e-31,
        9.08957903830664e-33,
        -1.601364714758818e-34,
        -6.129071589157745e-36,
        1.044164758827507e-37,
    ],
    &[
        -0.001590922454486678,
        0.1752100348635406,
        -0.003139329058784595,
        -0.01916521702504299,
        0.000313294677142926,
        0.0005634805502914542,
        -8.835209082658058e-6,
        -7.665367388448401e-6,
        1.167786896635036e-7,
        6.001850719746272e-8,
        -8.922071067250049e-10,
        -3.051426647708921e-10,
        4.431831177920046e-12,
        1.088101615252514e-12,
        -1.544117983086385e-14,
        -2.871641637815284e-15,
        3.980205428012957e-17,
        5.836109800804426e-18,
        -7.896674703926474e-20,
        -9.41685793694033e-21,
        1.243277085852557e-22,
        1.235911259076133e-23,
        -1.591577815948335e-25,
        -1.345543517163529e-26,
        1.689682383259505e-28,
        1.235079452375254e-29,
        -1.512191336570065e-31,
        -9.6897068524764e-33,
        1.156670848756829e-34,
        6.569625445931432e-36,
        -7.650772123124862e-38,
    ],
    &[
        0.01145319980927682,
        -0.1647164002129533,
        -0.006593218670796642,
        0.01813351074312768,
        0.0002043063631617271,
        -0.0005367161063989548,
        -1.572798258958865e-6,
        7.354291250753741e-6,
        -7.323920827702961e-9,
        -5.802777479182185e-8,
        1.870100858617184e-10,
        2.973839302888568e-10,
        -1.370572006668351e-12,
        -1.06900804279276e-12,
        5.895217414533246e-15,
        2.843745383620553e-15,
        -1.740450438038061e-17,
        -5.82397140393909e-18,
        3.800858122486687e-20,
        9.46622316551043e-21,
        -6.431115192173358e-23,
        -1.250961418413652e-23,
        8.709756189294054e-26,
        1.370684499673309e-26,
        -9.67589991951017e-29,
        -1.26564623938335e-29,
        8.989565053308803e-32,
        9.9840147385014e-33,
        -7.095366265373559e-35,
        -6.803309373443169e-36,
        4.820333765651574e-38,
    ],
    &[
        -0.02019639011367044,
        0.1528187727827203,
        0.01520100204661376,
        -0.01691255573059782,
        -0.0006623262079456938,
        0.0005032679944985264,
        0.00001080666862826489,
        -6.935769578306314e-6,
        -9.02340107017913e-8,
        5.506109513453016e-8,
        4.45635776064717e-10,
        -2.839834341614845e-10,
        -1.397947052021054e-12,
        1.027497406455188e-12,
        2.817678518357974e-15,
        -2.75117536233339e-15,
        -3.24205589068763e-18,
        5.67060833714159e-18,
        1.353910547114108e-22,
        -9.2743967811595e-21,
        7.845474476211777e-24,
        1.232928858612446e-23,
        -1.862443684703536e-26,
        -1.358577335793612e-26,
        2.748986481638117e-29,
        1.261156551623612e-29,
        -3.053964389325511e-32,
        -9.99827043034327e-33,
        2.73107300113762e-35,
        6.844725583969103e-36,
        -2.036076905566188e-38,
    ],
    &[
        0.02786980656158424,
        -0.1396620394566276,
        -0.02275168036703153,
        0.01552811655600402,
        0.001064494586592859,
        -0.000464198426425212,
        -0.00001893337885690606,
        6.428509621113779e-6,
        1.764112523132487e-7,
        -5.129649807788481e-8,
        -1.007273512358656e-9,
        2.659834514944772e-10,
        3.870869547221947e-12,
        -9.67652603948902e-13,
        -1.065556804961448e-14,
        2.605294295121296e-15,
        2.195972352658645e-17,
        -5.399532656588679e-18,
        -3.499932530018429e-20,
        8.878875677332361e-21,
        4.419671463901691e-23,
        -1.186557981145091e-23,
        -4.501086991881355e-26,
        1.314098658860365e-26,
        3.74099788595421e-29,
        -1.225762353898812e-29,
        -2.550901990139507e-32,
        9.76222857694819e-33,
        1.420862859015603e-35,
        -6.712053019169704e-36,
        -6.3141760499238e-39,
    ],
    &[
        -0.03449847464234104,
        0.1254161331027308,
        0.02928333958601446,
        -0.01400528343255311,
        -0.001412993470040047,
        0.0004204480031653067,
        0.00002599360239128253,
        -5.848240380468286e-6,
        -2.515380387077086e-7,
        4.688014300212006e-8,
        1.499024012937695e-9,
        -2.44236134317693e-10,
        -6.047300253593927e-12,
        8.928460689710599e-13,
        1.759443051515071e-14,
        -2.415684133185604e-15,
        -3.863753532694298e-17,
        5.031139180002391e-18,
        6.627687478522155e-20,
        -8.313292140496365e-21,
        -9.12163933624995e-23,
        1.116264999014905e-23,
        1.029168334324327e-25,
        -1.241971672263734e-26,
        -9.68867973406106e-29,
        1.163656863797714e-29,
        7.723185248865495e-32,
        -9.30728892882462e-33,
        -5.278151513898437e-35,
        6.425383962139652e-36,
        3.125602103540992e-38,
    ],
    &[
        0.0400955676741777,
        -0.1102701331683986,
        -0.03481857753538045,
        0.01236921230613389,
        0.001709169134248944,
        -0.0003728977395057286,
        -0.00003201337410355642,
        5.209043765714734e-6,
        3.158368942462538e-7,
        -4.193940475448279e-8,
        -1.921738908693112e-9,
        2.19475057647005e-10,
        7.927403495248263e-12,
        -8.05979331254785e-13,
        -2.362106453823166e-14,
        2.190660300735389e-15,
        5.320736219127128e-17,
        -4.583406462247805e-18,
        -9.37692245075337e-20,
        7.607920307251906e-21,
        1.328084759035482e-22,
        -1.026124580643681e-23,
        -1.544691755712127e-25,
        1.146675303782529e-26,
        1.501794793378327e-28,
        -1.07893467107131e-29,
        -1.238722287265086e-31,
        8.665069037679606e-33,
        8.778073435272905e-35,
        -6.005604505372843e-36,
        -5.402356032167006e-38,
    ],
    &[
        -0.04467018657122738,
        0.0944273503725042,
        0.03937323883762048,
        -0.01064535642533995,
        -0.001953983394958515,
        0.0003223987342707815,
        0.00003701191551476636,
        -4.524052216941978e-6,
        -3.694826561250769e-7,
        3.659017851075271e-8,
        2.276200530863254e-9,
        -1.923578580781142e-10,
        -9.5123707984611e-12,
        7.096375567722282e-13,
        2.873024398621412e-14,
        -1.937640302088914e-15,
        -6.563167994757843e-17,
        4.072487702425647e-18,
        1.17355110428467e-19,
        -6.790288757884205e-21,
        -1.687110576921263e-22,
        9.19900369794143e-24,
        1.992468342366349e-25,
        -1.032423432898331e-26,
        -1.967566943638764e-28,
        9.75527374076686e-30,
        1.648860531241895e-31,
        -7.866596259772521e-33,
        -1.187432462544454e-34,
        5.473682283548916e-36,
        7.428325986951338e-38,
    ],
    &[
        0.04823238867485944,
        -0.07810096795424755,
        -0.04296202141761829,
        0.008859437250363352,
        0.00214830767729661,
        -0.0002697848271265764,
        -0.00004100693681847699,
        3.805802069670877e-6,
        4.126466315432536e-7,
        -3.094088350692018e-8,
        -2.563304167854619e-9,
        1.634910991231112e-10,
        1.080468977435315e-11,
        -6.061855682626631e-13,
        -3.292384309668345e-14,
        1.663390460373675e-15,
        7.589768981267547e-17,
        -3.513147283718554e-18,
        -1.369740491580476e-19,
        5.885711226895164e-21,
        1.987741066730682e-22,
        -8.01085581974202e-24,
        -2.369891685999395e-25,
        9.03173993728028e-27,
        2.36271691251129e-28,
        -8.571793710567048e-30,
        -1.999036273619706e-31,
        6.941877063025682e-33,
        1.453438923675498e-34,
        -4.850229300989942e-36,
        -9.17942439981213e-38,
    ],
    &[
        -0.05079648942093156,
        0.06151001581445011,
        0.04560214451174851,
        -0.007037274908564053,
        -0.002293117644723013,
        0.0002158758739222592,
        0.00004401824380933764,
        -3.066392227660413e-6,
        -4.455282915140161e-7,
        2.509453078244048e-8,
        2.784204784472886e-9,
        -1.334441572608615e-10,
        -1.180847341918345e-11,
        4.978249203337725e-13,
        3.62107965277703e-14,
        -1.37418188769874e-15,
        -8.401467062896515e-17,
        2.919056819657416e-18,
        1.526171035842849e-19,
        -4.917682070155426e-21,
        -2.229403760549281e-22,
        6.7293883113038e-24,
        2.675678396347144e-25,
        -7.626463741642447e-27,
        -2.685306038494221e-28,
        7.274419013166602e-30,
        2.28701222628511e-31,
        -5.91967409395018e-33,
        -1.673745751087143e-34,
        4.155235530326757e-36,
        1.063956141028486e-37,
    ],
    &[
        0.05238321571107888,
        -0.04487556749640132,
        -0.04731574790536535,
        0.005204541309885212,
        0.00238962181279995,
        -0.000161475680594617,
        -0.00004607016979605535,
        2.317529357991484e-6,
        4.683774145715842e-7,
        -1.914962928847954e-8,
        -2.940426626168985e-9,
        1.027562582776535e-10,
        1.252975162939792e-11,
        -3.86625353084139e-13,
        -3.860733993079645e-14,
        1.075882133312302e-15,
        9.001285563535147e-17,
        -2.302979747171059e-18,
        -1.643217321923516e-19,
        3.908139358737204e-21,
        2.412336067556709e-22,
        -5.385101767861385e-24,
        -2.909682047758937e-25,
        6.143369669050272e-27,
        2.934702608960028e-28,
        -5.896731528190748e-30,
        -2.511801113314325e-31,
        4.827384447907887e-33,
        1.847280973692904e-34,
        -3.407900467493917e-36,
        -1.179960814736054e-37,
    ],
    &[
        -0.05302105359297669,
        0.02841711426058185,
        0.04813141654658577,
        -0.003386471874396866,
        -0.002439344240752558,
        0.0001073668665683243,
        0.00004719316379524546,
        -1.570505885386716e-6,
        -4.815088814636834e-7,
        1.320038505541618e-8,
        3.033939175070453e-9,
        -7.193920917747913e-11,
        -1.297668867829793e-11,
        2.745394777091103e-13,
        4.013726467075263e-14,
        -7.74002429567254e-16,
        -9.39429135894055e-17,
        1.676873082856031e-18,
        1.721686856619305e-19,
        -2.87761768658922e-21,
        -2.537508393387064e-22,
        4.006569464813456e-24,
        3.072765518827554e-25,
        -4.615290256661677e-27,
        -3.111422785048824e-28,
        4.470384020726864e-30,
        2.673509014611145e-31,
        -3.690958139684179e-33,
        -1.973855842871333e-34,
        2.626516272135124e-36,
        1.26565553823762e-37,
    ],
    &[
        0.05274700214891679,
        -0.01234910730734276,
        -0.04808507254500248,
        0.001607558160381119,
        0.002444173837027464,
        -0.00005430401302596334,
        -0.00004742475090823873,
        8.361384551574547e-7,
        4.853117233895229e-7,
        -7.336470499460582e-9,
        -3.067203977920682e-9,
        4.147743061804101e-11,
        1.315971707275372e-11,
        -1.634065790464204e-13,
        -4.083206338401151e-14,
        4.737149006521917e-16,
        9.5875545753479e-17,
        -1.051930091291557e-18,
        -1.762797721516152e-19,
        1.84531632546066e-21,
        2.60656850236512e-22,
        -2.620507343295874e-24,
        -3.16670694389616e-25,
        3.072961513948496e-27,
        3.217010659382982e-28,
        -3.025077102336421e-30,
        -2.773214076617009e-31,
        2.534825371891064e-33,
        2.054064581867157e-34,
        -1.828375352277179e-36,
        -1.321282169346103e-37,
    ],
    &[
        -0.05160687077489512,
        -0.003122319817815661,
        0.04722038969711612,
        0.0001087635822225325,
        -0.002406388411415282,
        3.005955217694007e-6,
        0.0000468100097839524,
        -1.246842287446546e-7,
        -4.802535882305488e-7,
        1.642539367464016e-9,
        3.043196616765786e-9,
        -1.182633031432871e-11,
        -1.3091589417354e-11,
        5.494949654555808e-14,
        4.073091447886653e-14,
        -1.798499683661651e-16,
        -9.59009465639099e-17,
        4.385823478075764e-19,
        1.768158219632665e-19,
        -8.291061063579336e-22,
        -2.62179501754158e-22,
        1.251775332903534e-24,
        3.194123138563591e-25,
        -1.545001737101273e-27,
        -3.253940927261511e-28,
        1.588506103009049e-30,
        2.812867291559011e-31,
        -1.38181904031794e-33,
        -2.089201142379218e-34,
        1.029688045929231e-36,
        1.347566764180816e-37,
    ],
    &[
        0.04965521322182174,
        0.01780242504777694,
        -0.04558883472672268,
        -0.001740417010348244,
        0.002328658980642176,
        0.00004585220353960032,
        -0.00004540166798756359,
        -5.542535555347663e-7,
        4.668814856827647e-7,
        3.802397069877221e-9,
        -2.965407523635684e-9,
        -1.659028736160931e-11,
        1.278735437779708e-11,
        4.923269797350317e-14,
        -3.988048958606665e-14,
        -1.031187307910238e-16,
        9.41280059745155e-17,
        1.535247710513362e-19,
        -1.739744832569641e-19,
        -1.545054436843767e-22,
        2.586052863777976e-22,
        7.667044278494396e-26,
        -3.158400292242646e-25,
        5.784737258184984e-29,
        3.225533090071151e-28,
        -1.862816410938981e-31,
        -2.795205859992244e-31,
        2.530889953046813e-34,
        2.081186451878593e-34,
        -2.455040241211449e-37,
        -1.345667972816223e-37,
    ],
    &[
        -0.04695496560022066,
        -0.03151069854459094,
        0.04324940891179591,
        0.003266975006564253,
        -0.002214038240162759,
        -0.0000916469462525541,
        0.00004325988848439922,
        1.191822876917164e-6,
        -4.458194673461629e-7,
        -8.925653440548954e-9,
        2.83782478647735e-9,
        4.338329644708315e-11,
        -1.226426441398133e-11,
        -1.476743184317203e-13,
        3.833458616886646e-14,
        3.710966328240775e-16,
        -9.06832040242466e-17,
        -7.155725832492229e-19,
        1.67987659207451e-19,
        1.090435006450144e-21,
        -2.502746549834002e-22,
        -1.343857176716072e-24,
        3.063625693472273e-25,
        1.364339881791947e-27,
        -3.135870026442057e-28,
        -1.158168253277853e-30,
        2.72367266402929e-31,
        8.319901296144897e-34,
        -2.032501448607533e-34,
        -5.103642776643074e-37,
        1.317129627845907e-37,
    ],
    &[
        0.04357683972358043,
        0.04408343913303161,
        -0.04026814521101179,
        -0.004669926938576556,
        0.002065936133698039,
        0.0001338137607665242,
        -0.00004045180303384122,
        -1.78001742598326e-6,
        4.177637615514879e-7,
        1.366155719965597e-8,
        -2.664901648843226e-9,
        -6.820146908139571e-11,
        1.154162335172414e-11,
        2.390536096694897e-13,
        -3.615359683379835e-14,
        -6.203938660452927e-16,
        8.570918928625462e-17,
        1.239617580593631e-18,
        -1.591185042805704e-19,
        -1.965098001555602e-21,
        2.375769135362363e-22,
        2.530891903073824e-24,
        -2.91451689012991e-25,
        -2.699781408782559e-27,
        2.989716516575463e-28,
        2.423705378478701e-30,
        -2.60233764035196e-31,
        -1.855913235226572e-33,
        1.946122685245636e-34,
        1.225415326526743e-36,
        -1.263835517994588e-37,
    ],
    &[
        -0.03959851352792718,
        -0.05537603576225528,
        0.0367174045952839,
        0.005932918291571554,
        -0.001888084842138949,
        -0.0001718538891497842,
        0.00003705083724622743,
        2.311764105297477e-6,
        -3.834757928420482e-7,
        -1.795209809656783e-8,
        2.451511077290472e-9,
        9.07345923325734e-11,
        -1.064058182221006e-11,
        -3.222032173611554e-13,
        3.340383174178394e-14,
        8.477513275491291e-16,
        -7.936305661676331e-17,
        -1.718641810806047e-18,
        1.47657962690476e-19,
        2.766486991559356e-21,
        -2.209445968973934e-22,
        -3.621062952457701e-24,
        2.716346435463962e-25,
        3.929193722324684e-27,
        -2.792435213501317e-28,
        -3.591589081031267e-30,
        2.43581849469629e-31,
        2.803136053262018e-33,
        -1.825458436140615e-34,
        -1.888527125469237e-36,
        1.187964993902763e-37,
    ],
    &[
        0.03510365441399581,
        0.06526492569353174,
        -0.03267500822027288,
        -0.007041958086436029,
        0.001684495131268831,
        0.0002053402260098439,
        -0.00003313586502093746,
        -2.781000363757992e-6,
        3.437734570415378e-7,
        2.174750964921708e-8,
        -2.202889538019445e-9,
        -1.107162986750613e-10,
        9.58388842887783e-12,
        3.961198375430547e-13,
        -3.015671309870954e-14,
        -1.050365057611289e-15,
        7.181435366585419e-17,
        2.146600521955e-18,
        -1.339208726290001e-19,
        -3.484246344236847e-21,
        2.008473081192935e-22,
        4.599936576022501e-24,
        -2.474861535815765e-25,
        -5.035894331825212e-27,
        2.549898874179785e-28,
        4.645576431910201e-30,
        -2.229199389789975e-31,
        -3.66015824401213e-33,
        1.674284114606054e-34,
        2.49001820380851e-36,
        -1.091948577367076e-37,
    ],
    &[
        -0.03018080773684911,
        -0.0736492047244717,
        0.028223237652845,
        0.007985590359396021,
        -0.001459405755955948,
        -0.0002339222237763812,
        0.00002879022556683131,
        3.182738947188642e-6,
        -2.995209840403881e-7,
        -2.500675967143019e-8,
        1.924571962898603e-9,
        1.279264729031574e-10,
        -8.395604317749354e-12,
        -4.599724878917383e-13,
        2.648786209537985e-14,
        1.225907284751965e-15,
        -6.324285407986612e-17,
        -2.518463528333116e-18,
        1.182416853820219e-19,
        4.109737233418314e-21,
        -1.777850634788914e-22,
        -5.455440968756774e-24,
        2.196198696366532e-25,
        6.005893275670211e-27,
        -2.268398574825311e-28,
        -5.57200796565056e-30,
        1.98794708920988e-31,
        4.415593558030066e-33,
        -1.496676453564149e-34,
        -3.021698332759544e-36,
        9.78423151788895e-38,
    ],
    &[
        0.02492218040001085,
        0.08045186928414596,
        -0.02344773254385602,
        -0.00875502678218459,
        0.001217227462569216,
        0.0002573296963262809,
        -0.00002410063325349498,
        -3.513118244016011e-6,
        2.516176945116951e-7,
        2.769793271171152e-8,
        -1.6223197720827e-9,
        -1.421931333427191e-10,
        7.10078866010263e-12,
        5.131089848998599e-13,
        -2.247609940254855e-14,
        -1.372542921532312e-15,
        5.383614047035303e-17,
        2.830247617223532e-18,
        -1.009698646591683e-19,
        -4.636088435156077e-21,
        1.522812174439605e-22,
        6.177931616409417e-24,
        -1.886793985845629e-25,
        -6.827963801336809e-27,
        1.954548264023472e-28,
        6.359873287569877e-30,
        -1.717824643548225e-31,
        -5.060222984742009e-33,
        1.296946376551875e-34,
        3.476906566717394e-36,
        -8.501866199278897e-38,
    ],
    &[
        -0.01942234801636366,
        -0.08562067541188616,
        0.01843631326275225,
        0.00934423827671214,
        -0.000962483026736579,
        -0.0002753754417123053,
        0.00001915600853329417,
        3.769436902866605e-6,
        -2.00985938719157e-7,
        -2.979849196336725e-8,
        1.302043737688945e-9,
        1.533937131219889e-10,
        -5.725162402301618e-12,
        -5.550603234371172e-13,
        1.820238066074582e-14,
        1.488940614778942e-15,
        -4.378704327447188e-17,
        -3.079038008919313e-18,
        8.24650416690865e-20,
        5.05822951015397e-21,
        -1.248750638811855e-22,
        -6.760233339126871e-24,
        1.553289892215821e-25,
        7.493687133109566e-27,
        -1.615186438961463e-28,
        -7.000852031700903e-30,
        1.424803130279806e-31,
        5.587027561240595e-33,
        -1.079570828512856e-34,
        -3.850534579616787e-36,
        7.101521445005121e-38,
    ],
    &[
        0.01377691300137354,
        0.08912860526298136,
        -0.01327775465307607,
        -0.00975000423788744,
        0.0006997446877855524,
        0.0002879566316830227,
        -0.00001404625670149801,
        -3.9501718343051e-6,
        1.485584919716101e-7,
        3.129541218906566e-8,
        -9.69723406491393e-10,
        -1.614556904481553e-10,
        4.294757480931005e-12,
        5.855427476907921e-13,
        -1.37486884500909e-14,
        -1.574277720292715e-15,
        3.32909832532798e-17,
        3.262997457068677e-18,
        -6.309200760155332e-20,
        -5.372903919305646e-21,
        9.61142238193436e-23,
        7.197655657359692e-24,
        -1.202439971531789e-25,
        -7.997467582372471e-27,
        1.25727602117666e-28,
        7.489326768396028e-30,
        -1.114972252642603e-31,
        -5.991198287419586e-33,
        8.491240785933658e-35,
        4.139033701143869e-36,
        -5.613022541167767e-38,
    ],
    &[
        -0.008081140020891256,
        -0.0909739376982141,
        0.008060535956530907,
        0.00997191872648482,
        -0.0004335702796045281,
        -0.000295054940945429,
        8.861020053104992e-6,
        4.054979102315726e-6,
        -9.52656697568553e-8,
        -3.218517870274562e-8,
        6.313247389873021e-10,
        1.663565368238256e-10,
        -2.835558524484861e-12,
        -6.044573813507073e-13,
        9.19690202734132e-15,
        1.628238798349288e-15,
        -2.254326592982293e-17,
        -3.381362066162044e-18,
        4.321562836336352e-20,
        5.578660442112287e-21,
        -6.65469382717584e-23,
        -7.487979202998224e-24,
        8.410126351888968e-26,
        8.33651543726112e-27,
        -8.878036965279809e-29,
        -7.822365502464263e-30,
        7.944507990726337e-32,
        6.270121353318335e-33,
        -6.102091737781519e-35,
        -4.340404518641021e-36,
        4.066432146195491e-38,
    ],
    &[
        0.002428594265648831,
        0.0911799255640934,
        -0.002871590906523606,
        -0.01001235372877215,
        0.0001684393005849337,
        0.0002967354131015819,
        -3.688427869511416e-6,
        -4.084677573923995e-6,
        4.202241454377322e-8,
        3.247365060592278e-8,
        -2.927175600308825e-10,
        -1.681229698454427e-10,
        1.373143850096626e-12,
        6.118873539999655e-13,
        -4.627665690187943e-15,
        -1.651007423438016e-15,
        1.17363759560102e-17,
        3.434423329790477e-18,
        -2.319576733279223e-20,
        -5.675821989134361e-21,
        3.671438843139014e-23,
        7.631411872003266e-24,
        -4.756954984655993e-26,
        -8.510796135725047e-27,
        5.136800949912456e-29,
        7.999672367786679e-30,
        -4.692980865082898e-32,
        -6.423337612810584e-33,
        3.673903296974628e-35,
        4.454168373329844e-36,
        -2.491601441963956e-38,
    ],
];

pub const SPLITS: [f64; 33] = [
    0.3488841264349171,
    3.35673434965313,
    6.364584572871343,
    9.37243479608956,
    12.38028501930777,
    15.38813524252598,
    18.3959854657442,
    21.40383568896241,
    24.41168591218062,
    27.41953613539884,
    30.42738635861705,
    33.43523658183526,
    36.44308680505348,
    39.45093702827169,
    42.4587872514899,
    45.46663747470811,
    48.47448769792633,
    51.48233792114454,
    54.49018814436275,
    57.49803836758097,
    60.50588859079918,
    63.51373881401739,
    66.52158903723561,
    69.52943926045382,
    72.53728948367203,
    75.54513970689025,
    78.55298993010846,
    81.56084015332667,
    84.56869037654489,
    87.5765405997631,
    90.5843908229813,
    93.5922410461995,
    96.6000912694177,
];
