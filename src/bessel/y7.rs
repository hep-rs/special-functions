#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-7)
        * polynomial(
            x2,
            &[
                -29335.43911069815,
                -1222.309962945756,
                -30.5577490736439,
                -0.6366197723675813,
                -0.01326291192432461,
                -0.0003315727981081153,
                -0.0000138155332545048,
                -1.393750553606109e-6,
                6.090121697800107e-8,
            ],
        )
        + x.powi(7) * x.ln() * polynomial(x2, &[9.868238038932e-7, -3.083824387166251e-8])
}

pub fn upper(x: f64) -> f64 {
    let x1 = x.recip();
    let amp = x1
        * polynomial(
            x.powi(-2),
            &[
                0.6366197723675813,
                15.5176069514598,
                544.0860937355591,
                19383.06708932929,
                623286.751091245,
                1.612754468448597e7,
                2.771921742646025e8,
                1.737400949408491e9,
                -1.180889707801084e10,
                2.593036983379879e11,
            ],
        );
    let amp = amp.sqrt();
    let phase = x - 11.78097245096172
        + x1 * polynomial(
            x1,
            &[
                24.375,
                0.0,
                86.8359375,
                0.0,
                652.9833984375,
                0.0,
                -9330.05578177316,
            ],
        );

    amp * phase.sin()
}

pub const COEFFICIENTS: [&[f64]; 20] = [
    &[
        -65480.22926607363,
        89525.53572719439,
        -41488.83334627216,
        15184.58778090925,
        -4736.43302132085,
        1315.543956229564,
        -334.4939939621962,
        79.32623370319731,
        -17.78054338616682,
        3.803725045567154,
        -0.782389183089595,
        0.155626899803489,
        -0.03007301450310072,
        0.005666304687632774,
        -0.001044153793296252,
        0.000188650693619418,
        -0.00003348841417004849,
        5.851223402994521e-6,
        -1.007804409170006e-6,
        1.713379176158098e-7,
        -2.878546995971658e-8,
        4.783747198295362e-9,
        -7.870788586569203e-10,
        1.283096309279469e-10,
        -2.073908067845722e-11,
        3.325638710748715e-12,
        -5.293640827866096e-13,
        8.368501544996005e-14,
        -1.315165433504615e-14,
        2.101282506584624e-15,
        -3.185657896560618e-16,
    ],
    &[
        -2434.496934337917,
        2479.459717055247,
        -791.4175909281673,
        194.8375348425013,
        -40.47952471905056,
        7.451688450351115,
        -1.252227467005255,
        0.195930313864536,
        -0.02894141603088133,
        0.004076859730810303,
        -0.000551860225089856,
        0.0000722090314084563,
        -9.17569438551186e-6,
        1.136586495196064e-6,
        -1.376625204177124e-7,
        1.634494076745221e-8,
        -1.906475326993166e-9,
        2.188482033802659e-10,
        -2.47621448191946e-11,
        2.765308418508693e-12,
        -3.051469780507443e-13,
        3.330592158603436e-14,
        -3.598854789603456e-15,
        3.852800168862325e-16,
        -4.089391433074976e-17,
        4.306050485135773e-18,
        -4.500678479953811e-19,
        4.671665460533051e-20,
        -4.818370972522502e-21,
        4.989611366709588e-22,
        -5.033577865311347e-23,
    ],
    &[
        -199.5491315571036,
        244.2282773932993,
        -99.0574240544902,
        31.62030859849418,
        -8.6032859502904,
        2.085868513961816,
        -0.4633025394869479,
        0.0960425254939912,
        -0.0188270632327228,
        0.003523798148393328,
        -0.0006343448838489642,
        0.0001104575735346494,
        -0.00001868879824804011,
        3.083657679493393e-6,
        -4.976769387496805e-7,
        7.875949000307335e-8,
        -1.224720549813855e-8,
        1.874636528067295e-9,
        -2.828782174516053e-10,
        4.213571169145354e-11,
        -6.20241971648243e-12,
        9.03153705528078e-13,
        -1.302056137544244e-13,
        1.859941443392935e-14,
        -2.634311148435507e-15,
        3.70167171090179e-16,
        -5.163320884453462e-17,
        7.152811658558904e-18,
        -9.84856503314037e-19,
        1.371768683003687e-19,
        -1.832447922447045e-20,
    ],
    &[
        -9.4490567516247,
        11.95980008605762,
        -5.412402857755753,
        1.974989834734855,
        -0.623681143912195,
        0.1772896806781937,
        -0.04645097379395719,
        0.01140814481752021,
        -0.002657786377780298,
        0.0005925576738939583,
        -0.0001272861021273017,
        0.00002648310470389228,
        -5.359555955892961e-6,
        1.058646047793064e-6,
        -2.046738210947385e-7,
        3.882281013828248e-8,
        -7.239150572740187e-9,
        1.32922482806228e-9,
        -2.406852996246319e-10,
        4.303143419068313e-11,
        -7.604691169124851e-12,
        1.329696214678187e-12,
        -2.302313835916938e-13,
        3.95040962491164e-14,
        -6.721617868698679e-15,
        1.134795257584639e-15,
        -1.901975349772166e-16,
        3.166299637142175e-17,
        -5.241200577820293e-18,
        8.841266112943926e-19,
        -1.408227767547975e-19,
    ],
    &[
        -0.5305373030079309,
        0.6290199953044587,
        -0.1370256187715042,
        0.0765139420787527,
        -0.0279037607476401,
        0.006638036670395978,
        -0.001920207630055528,
        0.0005417910778093253,
        -0.0001324435528390297,
        0.00003169580523080151,
        -7.497446871138591e-6,
        1.709676349099245e-6,
        -3.790734796698538e-7,
        8.235991689584494e-8,
        -1.754981307700069e-8,
        3.673071525749424e-9,
        -7.565957709303351e-10,
        1.536204606613188e-10,
        -3.078452135520174e-11,
        6.095410624992086e-12,
        -1.193691958103201e-12,
        2.314085471030184e-13,
        -4.444266072740486e-14,
        8.461613576648125e-15,
        -1.598107893228537e-15,
        2.995702765804515e-16,
        -5.576320136512773e-17,
        1.03124643188099e-17,
        -1.89733697361945e-18,
        3.57713908249012e-19,
        -6.294328940455243e-20,
    ],
    &[
        0.04215300633979627,
        -0.0944499299025826,
        0.04167408762993946,
        0.1759374235000063,
        -0.02051409685315811,
        -0.03601910070249633,
        0.000604816669870911,
        0.003500689202657834,
        0.000096647497203256,
        -0.0001922218419569218,
        -0.00001113980080254598,
        7.091292603880402e-6,
        4.994387592067538e-7,
        -1.779629254938113e-7,
        -1.580083021132632e-8,
        3.620985786596491e-9,
        2.843921257162698e-10,
        -4.546892543846943e-11,
        -6.258194885377548e-12,
        8.958656285916254e-13,
        3.524466531698398e-15,
        5.435527521787692e-15,
        -2.903160479386364e-15,
        5.106048772734997e-16,
        -8.666456209406263e-17,
        1.768197380186592e-17,
        -3.560923391776288e-18,
        6.88662604547364e-19,
        -1.328801482657131e-19,
        2.651214908779366e-20,
        -4.903315528056074e-21,
    ],
    &[
        0.07097287158056084,
        -0.01485626738103786,
        0.1338545263182473,
        0.03632168385728715,
        -0.0918077115474942,
        -0.00831383204696605,
        0.0148530985940884,
        0.0008935643813701452,
        -0.001147981314023216,
        -0.00005886166235149871,
        0.00005319934399520508,
        2.587472372067603e-6,
        -1.660671125164316e-6,
        -7.978801258767548e-8,
        3.749396628744375e-8,
        1.801391031461862e-9,
        -6.425768913466988e-10,
        -3.089780117189858e-11,
        8.65830262325744e-12,
        4.152835500520073e-13,
        -9.42175063547924e-14,
        -4.487078697360762e-15,
        8.457107515066789e-16,
        3.980942284152727e-17,
        -6.370610061916713e-18,
        -2.951647055396788e-19,
        4.085063406996923e-20,
        1.856430479366009e-21,
        -2.257020159565997e-22,
        -9.98239664583226e-24,
        1.085426532301334e-24,
    ],
    &[
        0.0128085071736875,
        -0.02154652755352094,
        0.0280787079723437,
        -0.1257933487111458,
        -0.01485060404706637,
        0.03880214948452159,
        0.002147424277777237,
        -0.00445972803705828,
        -0.0001578686627724763,
        0.0002760728721086912,
        7.344162628085935e-6,
        -0.00001080539752064999,
        -2.406989432446643e-7,
        2.933039527011844e-7,
        5.883049886815482e-9,
        -5.866840001371773e-9,
        -1.108892708848468e-10,
        9.02721832170681e-11,
        1.650556925162711e-12,
        -1.103251184404024e-12,
        -1.978452670748671e-14,
        1.097840304152269e-14,
        1.943090931548159e-16,
        -9.07291957537935e-17,
        -1.588106587911678e-18,
        6.328890973573218e-19,
        1.095266045979172e-20,
        -3.776951571563219e-21,
        -6.453438920541963e-23,
        1.941738253734495e-23,
        3.284173083056378e-25,
    ],
    &[
        -0.05175936962267571,
        0.00549802476690026,
        -0.0923823130919247,
        -0.00110981666488662,
        0.07547927199651826,
        -0.001019591979152182,
        -0.01354939905048781,
        0.0002194634685024778,
        0.001138314600876714,
        -0.00001833800685153378,
        -0.00005636629021479887,
        8.562389105330648e-7,
        1.852857274597285e-6,
        -2.58791908523637e-8,
        -4.353683547767905e-8,
        5.506863349869882e-10,
        7.693916049952251e-10,
        -8.722171499586745e-12,
        -1.061392015104128e-11,
        1.070273783114615e-13,
        1.175969172577099e-13,
        -1.048798149800091e-15,
        -1.070189634756771e-15,
        8.405771798592166e-18,
        8.146413824348315e-18,
        -5.617298071306491e-20,
        -5.2654824758253e-20,
        3.180241300768461e-22,
        2.926638822917169e-22,
        -1.539416416527086e-24,
        -1.41386882664239e-24,
    ],
    &[
        0.01116028689888674,
        0.02029517284623598,
        0.01767845024610608,
        0.0991615982397556,
        -0.01861708134987594,
        -0.0318407995295464,
        0.003708057380946646,
        0.003784468725456443,
        -0.000335845459361387,
        -0.0002414847765959122,
        0.00001764780155770073,
        9.7092496511743e-6,
        -6.085501555214509e-7,
        -2.697166352724216e-7,
        1.486036879966972e-8,
        5.500353444235011e-9,
        -2.707786852804823e-10,
        -8.597867511231598e-11,
        3.825831722924307e-12,
        1.064108907284597e-12,
        -4.316802919068225e-14,
        -1.069416066005186e-14,
        3.981740994251907e-16,
        8.905671001239987e-17,
        -3.059913305198985e-18,
        -6.248193244946245e-19,
        1.990261571848883e-20,
        3.744786682298018e-21,
        -1.110311623654338e-22,
        -1.931144722320036e-23,
        5.372805099663347e-25,
    ],
    &[
        0.03637188075771967,
        -0.01387898062962946,
        0.06461368490411979,
        -0.04970558982786811,
        -0.05371494248307988,
        0.01702262632437357,
        0.00975424532728436,
        -0.002115192646471745,
        -0.0008284045024698582,
        0.0001400256041235045,
        0.0000414411518838271,
        -5.811191507770792e-6,
        -1.374979239575152e-6,
        1.659408753209794e-7,
        3.257321574025921e-8,
        -3.465985100336119e-9,
        -5.796293469873032e-10,
        5.530920356919256e-11,
        8.041061963141112e-12,
        -6.967511053588342e-13,
        -8.947981614658214e-14,
        7.108286694224142e-15,
        8.169277895945892e-16,
        -5.994841019507573e-17,
        -6.232240018804161e-18,
        4.250556148282894e-19,
        4.033624414836034e-20,
        -2.569820267644855e-21,
        -2.243333378904891e-22,
        1.334680904194264e-23,
        1.083796122920011e-24,
    ],
    &[
        -0.03014715985684843,
        -0.01113730893870888,
        -0.05184389507839134,
        -0.05601254653874127,
        0.04648770183968069,
        0.01791874848782175,
        -0.008757161678096692,
        -0.002125150914310092,
        0.0007661861634383756,
        0.0001353720948532073,
        -0.00003934585397426075,
        -5.433826792327669e-6,
        1.336591209416012e-6,
        1.506741671443115e-7,
        -3.234602505231816e-8,
        -3.066217982615052e-9,
        5.867817693621285e-10,
        4.78102896533737e-11,
        -8.282727421302365e-12,
        -5.900093727808504e-13,
        9.36135301391159e-14,
        5.910020844592123e-15,
        -8.666071761199145e-16,
        -4.903662617236907e-17,
        6.693183467140033e-18,
        3.426766711946795e-19,
        -4.379394738603568e-20,
        -2.045145294466985e-21,
        2.459116498383451e-22,
        1.050040671673349e-23,
        -1.198096738061609e-24,
    ],
    &[
        -0.01317209532367104,
        0.0190664058528001,
        -0.0238981126616125,
        0.07729764535218282,
        0.01888499743331546,
        -0.02582772775234056,
        -0.003336760782047771,
        0.003162668054656003,
        0.0002766073847025421,
        -0.0002072007237631601,
        -0.00001351870658287034,
        8.533918154134952e-6,
        4.383117348495783e-7,
        -2.423611282539197e-7,
        -1.014661489528219e-8,
        5.043111969572884e-9,
        1.764136983447026e-10,
        -8.028269029264277e-11,
        -2.390876836324124e-12,
        1.010009311338825e-12,
        2.5988664288085e-14,
        -1.0299245372182e-14,
        -2.31758589694968e-16,
        8.687494749392051e-17,
        1.727030134648879e-18,
        -6.163820615480252e-19,
        -1.091944142620732e-20,
        3.730304676967337e-21,
        5.93374680857899e-23,
        -1.939782973420762e-23,
        -2.80174844583441e-25,
    ],
    &[
        0.03496555273009141,
        -0.001560054345367548,
        0.06075603729679485,
        0.0006428329380776876,
        -0.05329078995426445,
        0.0003063413193211142,
        0.00994564004799359,
        -0.00008366259268143065,
        -0.000864340984626793,
        8.123131583707792e-6,
        0.0000441527099786539,
        -4.343812096879972e-7,
        -1.493540252891488e-6,
        1.498401826614e-8,
        3.602109525064149e-8,
        -3.636592785803634e-10,
        -6.516615671121092e-10,
        6.569436840242238e-12,
        9.17842275269357e-12,
        -9.19394200160465e-14,
        -1.035567326916488e-13,
        1.027271302917936e-15,
        9.57337492607619e-16,
        -9.38232820725694e-18,
        -7.385907410051909e-18,
        7.138858481971649e-20,
        4.828481143947457e-20,
        -4.596692886243438e-22,
        -2.709403098956976e-22,
        2.525769632821085e-24,
        1.319279460388716e-24,
    ],
    &[
        -0.011028517319158,
        -0.0164701841200457,
        -0.01844386994865203,
        -0.06981207423734863,
        0.01768391111354018,
        0.02311636755855614,
        -0.0034401803834501,
        -0.002813307181618125,
        0.0003092067414557046,
        0.0001833820146860398,
        -0.0000162753853910042,
        -7.51984135054876e-6,
        5.658928358078342e-7,
        2.127335382095642e-7,
        -1.400182564911258e-8,
        -4.411225710211302e-9,
        2.594501819898355e-10,
        7.000270057920298e-11,
        -3.737467975871768e-12,
        -8.781549351702939e-13,
        4.307202188414865e-14,
        8.931098055308529e-15,
        -4.062217102906383e-16,
        -7.515042350154826e-17,
        3.193690466765182e-18,
        5.319775128161788e-19,
        -2.12535501079114e-20,
        -3.212552297880487e-21,
        1.212819142771373e-22,
        1.667135149045711e-23,
        -6.000054958441251e-25,
    ],
    &[
        -0.02432022956813068,
        0.01221577075127291,
        -0.04257039743717816,
        0.04591919176502585,
        0.03670691489457535,
        -0.01563512942271368,
        -0.006793863776266162,
        0.001942185251973515,
        0.0005863930645402653,
        -0.0001289173368519429,
        -0.00002976912220473233,
        5.376430198996369e-6,
        1.001175154985778e-6,
        -1.545480022242009e-7,
        -2.401410632989988e-8,
        3.253963483631319e-9,
        4.321682914444859e-10,
        -5.239814450235489e-11,
        -6.056295795847774e-12,
        6.6659799698537e-13,
        6.799850127787657e-14,
        -6.87144449484565e-15,
        -6.256499584144215e-16,
        5.857254392684614e-17,
        4.804746332133869e-18,
        -4.198102131159548e-19,
        -3.126981059831058e-20,
        2.565617980766417e-21,
        1.74694128985265e-22,
        -1.346680640211962e-23,
        -8.46971145798295e-25,
    ],
    &[
        0.02655705686847644,
        0.007094931590018607,
        0.04572198589384394,
        0.032827520453358,
        -0.04102732264421233,
        -0.01066969351433751,
        0.007746730513325571,
        0.0012803090290545,
        -0.0006800747210714929,
        -0.00008238562097997131,
        0.00003507191010465654,
        3.336831908914253e-6,
        -1.197311373426878e-6,
        -9.32652832956437e-8,
        2.913607978299811e-8,
        1.911087113979412e-9,
        -5.317273162377329e-10,
        -2.99727240969123e-11,
        7.553330855080096e-12,
        3.716301935899789e-13,
        -8.593321839709027e-14,
        -3.735960337590386e-15,
        8.008749437992622e-16,
        3.107490687799314e-17,
        -6.22759577796405e-18,
        -2.174566080107296e-19,
        4.102415443532029e-20,
        1.298220124987773e-21,
        -2.319034568261358e-22,
        -6.660809848699734e-24,
        1.137269765063679e-24,
    ],
    &[
        0.004189183924389972,
        -0.01616827439370369,
        0.007746321548653903,
        -0.06484169708561927,
        -0.005810276206945116,
        0.0217496317786622,
        0.000992031106344296,
        -0.002673112034268482,
        -0.00007937584502294293,
        0.0001758144365257851,
        3.729346873094909e-6,
        -7.27158814474455e-6,
        -1.155940760165647e-7,
        2.074278354907649e-7,
        2.540539423512372e-9,
        -4.336224699001995e-9,
        -4.158664600513402e-11,
        6.935980391590847e-11,
        5.252954304029338e-13,
        -8.768504503434917e-13,
        -5.256568272979391e-15,
        8.985452632326959e-15,
        4.250010403182495e-17,
        -7.616650325494039e-17,
        -2.816116584631839e-19,
        5.430465949682181e-19,
        1.543204077837254e-21,
        -3.302296793749329e-21,
        -7.014529788730813e-24,
        1.725258197517018e-23,
        2.627539448837838e-26,
    ],
    &[
        -0.02752220244889554,
        0.004452748157893522,
        -0.04771049390437349,
        0.01442296803983663,
        0.04212497066764642,
        -0.005108030350822292,
        -0.007892052234103731,
        0.000652530279976076,
        0.0006883693355066491,
        -0.00004438091230531628,
        -0.00003529263854947151,
        1.892695132261944e-6,
        1.19829261344346e-6,
        -5.555754592518284e-8,
        -2.901005706286817e-8,
        1.193179789687425e-9,
        5.268381308599265e-10,
        -1.958024598511333e-11,
        -7.448917872353842e-12,
        2.536410110573143e-13,
        8.436620381981577e-14,
        -2.660329497516618e-15,
        -7.82895827260258e-16,
        2.305780539456827e-17,
        6.062680430544611e-18,
        -1.679338865226246e-19,
        -3.977927870928022e-20,
        1.042270316824705e-21,
        2.240066233764546e-22,
        -5.552424001808065e-24,
        -1.094492754749901e-24,
    ],
    &[
        0.01531149654828443,
        0.01208383762265681,
        0.02613962796836111,
        0.05041814137154,
        -0.02394255366989785,
        -0.01676018816544957,
        0.004567306691793916,
        0.002046221328629365,
        -0.0004044825873982367,
        -0.0001337841201212241,
        0.00002103087330360981,
        5.502386949181197e-6,
        -7.236447337831605e-7,
        -1.56122113120102e-7,
        1.774515138695305e-8,
        3.246859557195184e-9,
        -3.262856379267763e-10,
        -5.16748315752269e-11,
        4.669258476258182e-12,
        6.500891395536305e-13,
        -5.350764318990079e-14,
        -6.63001335983683e-15,
        5.022407171236682e-16,
        5.59386710791493e-17,
        -3.932855063709876e-18,
        -3.970106510080532e-19,
        2.608646261393237e-20,
        2.403469279136378e-21,
        -1.484629293833145e-22,
        -1.250203292530461e-23,
        7.329182734918766e-25,
    ],
];

pub const SPLITS: [f64; 21] = [
    0.7533298643901151,
    1.268154088319746,
    1.782978312249377,
    2.812626760108639,
    4.871923655827164,
    8.990517447264212,
    17.22770503013831,
    25.46489261301241,
    33.7020801958865,
    41.9392677787606,
    50.1764553616347,
    58.41364294450879,
    66.65083052738289,
    74.88801811025699,
    83.12520569313108,
    91.3623932760052,
    99.5995808588793,
    107.8367684417534,
    116.0739560246275,
    124.3111436075016,
    132.5483311903757,
];
