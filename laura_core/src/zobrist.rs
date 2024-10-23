
use std::fmt;

use crate::castle_rights::CastleRights;
use crate::piece::Piece;
use crate::square::Square;

/// A 2D array containing Zobrist hash keys for each piece on every square.
/// This is used to represent piece-square combinations in the hash calculation.
/// Each key is a 64-bit unsigned integer and is unique for each combination.
pub const KEY_PIECE_SQUARE: [[u64; Square::NUM_SQUARES]; Piece::NUM_PIECES] = [
    [9179379850155079678, 10550500542342481924, 13815908865116790790, 5183221082089463817, 13694083998681346056, 5313439612563509260, 10078717387500886027, 9530595725993308168, 1234140559130869777, 9134689826296774672, 3252706324948305940, 4455577012762675222, 7409686970070704149, 13080123655048572950, 15586861893973786646, 2448711879504269341, 15530033545933338648, 13038312743016597531, 304266384963129374, 3356395755471489067, 13183662243846457383, 1949121030123935790, 5941983486501541934, 15297280033071423531, 4181238933903065140, 7565107105029412920, 17268364631777546292, 2488894638978306108, 5145949986824433725, 6529054320266274879, 14390195046386980928, 5454466075307429956, 7731421033900675144, 8461612450797885514, 16881939621641189450, 14499743590493126734, 10396059890082816081, 9928123258287550547, 9847729022918836308, 13618424384694978647, 16345841442937264214, 10139280996809900124, 3424117018998206560, 8814494645493133416, 14058681753087123559, 4948925548587780203, 6940414015600357485, 12333136695709481069, 1165301719194011763, 17168016367267385451, 5476669003289505906, 11910403917403308145, 529940942252521592, 13957526354495758454, 6732557430384146555, 14684300181057589369, 17932746257873201274, 15529095234002253949, 9609597946456201346, 16994364521269510275, 14935480570741717129, 12220179565339814026, 3618722629917610126, 18375176624106238092],
    [13039769253822140560, 1356097174163579037, 12889456467965984922, 11605345754320863388, 16976885545262393499, 14264269535835893918, 4435079067846494376, 463737098735888556, 3736345225345188014, 13617687199457693866, 7445581864613132462, 6199506097416364207, 5460503453426026674, 6145288328483096755, 10830080858646634674, 11142538377476343989, 4697042106809305272, 15421049366389811381, 7246685038496377016, 7255183353107810489, 8753818263118416060, 2339387918743302337, 4117508937784789186, 17798398628063584445, 250172753562343622, 15657616538120126660, 14214058123890286790, 409167201518270669, 866513309265332430, 6421661368818860234, 15462879630083121355, 12873368989582713036, 10807501244360847567, 7464242413353212115, 18076942514674630871, 6418889085184252124, 5885835333201459427, 5748898363158464740, 2342470052570351846, 13944564693640184035, 15880039134176674024, 9763805880420292844, 15810866159582505194, 13879867973728327921, 15698454634325719289, 5892902828863535359, 15002614173983297787, 18215317038101827839, 9445658339668957444, 695314765061320970, 1660732187172210958, 5587274532027396367, 14719013830877704462, 12970165989912035613, 3788979192530381090, 11080439687644813603, 15203634405644099875, 7195515317788838183, 9209246652182163755, 49044401915660593, 3676761331657279796, 15900962825566243121, 3140333577424437562, 566373063785992510],
    [222995612797933888, 7557425525468576062, 8010839154077212993, 15464179454949523775, 17721201421811083586, 16795644005051615557, 9287766466639671624, 11307258204539167050, 17964450286038040904, 18125465551982459211, 3638487770499291474, 15697893641191989584, 6516088811778593110, 2056115215224965464, 8136203741153007959, 17328111783609227604, 8767175625624136026, 6688773239291685216, 5804148076766054756, 13143718781673861474, 9618697550601711983, 1471445275636459892, 18052227541000782190, 9137825379738605937, 12224769852740127089, 13545867433197984113, 15109939725964724596, 6380881152219435385, 3537578889454809467, 1342627548252125564, 1416755737852316031, 15956718173900587390, 8948419022824917381, 1131195884722710921, 911076275464874378, 12726373419831480711, 1024579506692268433, 3326603539147805073, 17034216639285207436, 16935088351889195409, 6141530011123007894, 16051397167801321877, 5144635907913382297, 10798705624172323226, 7702663455723721120, 6403927549316753828, 11069178729642242470, 1384492448768766380, 5176685142623183277, 10900359049120414128, 16787721809236273582, 2753486609697251765, 5266545364154202551, 16331198196213969331, 13744045142174857652, 10610358030454956477, 1472573842100414915, 18148097271782828477, 3340316977109428675, 7440037277878084040, 3958944218033332682, 15691663096893934022, 748698097508016590, 12393877158229045704],
    [13171212512121457094, 5785629995727376846, 17987703208876173768, 7863444495796459987, 15256340898453428689, 5032205665054079447, 2984319482869178841, 1364392721708057050, 1134202722520240604, 9121704056014825949, 7957650277514205663, 246169554226176483, 11446267129270874600, 188371327155825133, 6031411285082481131, 17276204897898471911, 14383827006801738214, 13137557877531933164, 18278295893945412076, 9456021666198630896, 11172322788620859888, 2623715390192968185, 10634719709253546490, 12659652154532684282, 9974032476214827518, 15554303919630356989, 4194982923375198731, 14623277991856480778, 16324309356741181963, 11272975724441285134, 4770033766527769108, 8880953449593373204, 16320919677316489746, 17391628590236047893, 16242574499602958874, 7833586143042574879, 3550862461800557090, 3050991400279980579, 843839315113339429, 14293331507419814436, 4752274640717355571, 6593007527672982069, 18175328130928622130, 14189685326152806965, 2935795415275983418, 5248398362654282299, 12871718760190925368, 564075145184283198, 2917752853468641859, 3718476025430334022, 10704249910053947972, 15284522731207916103, 10120162763176593996, 14519175879492004433, 1744551740275094112, 9439461134684398173, 3752825715153906274, 7963136604484024930, 7814633046571102820, 8386664236802284139, 14676088050320245361, 7961609423473373813, 8773210495231861369, 10421016102880305784],
    [18325896990931587701, 18296408768682072694, 12671157963003519618, 4118594093815796367, 11113022770280852110, 1206344485967686293, 14368877114089216656, 18126723704917693075, 8429476913646361258, 13597308796623297193, 16003208466212960937, 8535022830091242159, 8614395713079124655, 14357605999244542640, 7646465170892085939, 14589032071328570037, 13211141763361256124, 3069396506203081412, 9263248174181814986, 11763707043502162634, 8958053008224340684, 435718698850919119, 4439506016535265998, 13638444965394406090, 3707469582725472979, 4251423963712873169, 10479773692620026579, 3883334904278854358, 4876580970385537749, 3673323064910392024, 17951658908554672855, 8646004249639273184, 13495555517967172320, 13706712238460449516, 4026563865676540660, 17993284766469686002, 7994993972807938812, 12783995419301599994, 17122016249650328318, 2831074890735631110, 8927066009807174405, 11996333545533686538, 5596554709561400078, 15277977681342601996, 16351810287301378827, 2215799739447800599, 17648261158351846161, 2707260261493283608, 12685855643066465045, 1544390074283758370, 9887792507695942431, 5613120782807237408, 4253386306960786216, 11844297454431701797, 5426548642377130797, 5059981526654384944, 10861925969616360243, 6551242366134168378, 15829116343471588162, 4815834875726062408, 4666959506778618700, 15529478695944426314, 8193961409809437517, 5719654791813063506],
    [6138879580265878356, 13814918257225050963, 16644092297179413333, 14109122017842643799, 302558340138670945, 18189476589003135836, 8161366125138477924, 8969565865213993832, 2844570338281374575, 11707948471943623531, 5921982848824476528, 11714457893447381868, 10925705521298461553, 13015001718619599729, 8494975720238682997, 1325515397919075194, 6918693992011625335, 5936324424987835258, 11041836955577351037, 9490229348688036741, 10296263950272693126, 9773317162308375430, 7127740377513214869, 7335649554119228311, 13585062180067457944, 1342554592461353888, 12768900640180839324, 14271089881472437147, 3213844542707645343, 18106721511715425178, 1164042912250176416, 12799591834245032869, 8984720227210126248, 7398053346220325801, 10286255483341038507, 4230207405799017395, 9190198669836053426, 12436092622707626929, 13522649026213770161, 892548348631051198, 6453429262825116608, 18420235103515902910, 9046195285045763010, 13090919407986684869, 1084986631014710222, 2920675499875328976, 2836560386005384146, 13949716604403555278, 2185085144030634965, 13697641380463729615, 13464342107159874514, 17797728861607220178, 14850492747340491732, 1361305380990008285, 261100495945974749, 14297170555117767639, 1832019028394906601, 1350410936590433258, 11672001359589954533, 4108836719365790700, 15251116071469022184, 5388326325811377137, 15039484444864052207, 3939431974684466165],
    [8826651441994701814, 16219618842245311480, 5241991283800916991, 14267035325376836607, 8930709133012829186, 15139394972024077315, 7505720417737360391, 1666968456598488075, 7381343929885309960, 6770181092790801418, 13124280925167668233, 10017178089273863180, 14133417070252155917, 4631877107114189841, 14134721531288196110, 6872274946592044055, 13861551660225596437, 6762408857609120795, 15343699859970130968, 12719469160027491357, 16205916689576403999, 6716291413191924772, 3862470388105702438, 3535569668072938539, 18062068113615973417, 10236891452198308910, 15448243374956278831, 2821694359240295478, 16451543321126112307, 13983538416717020214, 15264256669083089978, 17529266690246931513, 4469408188364436549, 10127879318156928066, 6429238123353175115, 11830964467403660362, 10087630249558477900, 2380130655624756304, 13716747038702152784, 4270127724752956501, 7296329262482949206, 5071582168028816473, 1308784231686100061, 3403383769592900704, 12946319502756402270, 190001323042575464, 14643553061316404323, 417447123815165037, 12580646222184735851, 15643726700901643378, 2578697433658115207, 17122674441159117953, 15239750690893970563, 1609661901050557581, 13970421200052200589, 12653676421460337811, 13406920359351456918, 12277320426890994839, 7988623887994127515, 10507982631391302811, 9523070725345778844, 17923361650359061658, 16733705402284684443, 1940700527607282854],
    [12509807131302380705, 7341823214673093797, 3293793979127387307, 1032779782949627060, 17966049773507099823, 9790992642448704691, 2146277818210591928, 16772190732070286512, 17614887320201108659, 4302828479841100989, 8959356578967678140, 332307475192671424, 9140799434867999934, 5257102822761442500, 11992523088984454337, 8073429011394125001, 15668179751560668358, 10730418600034927819, 5203565658822575309, 14289754253836178650, 18367651865261618396, 18085426130285096158, 7188121836560649443, 12538482242354445542, 7627908888405015788, 8196819058637939951, 2693446538654428401, 9282826561100307698, 17628728397476214000, 44998850051413240, 7501031470220082422, 15358855467913925876, 4360904374167364858, 15293039770137570550, 12914435050133830908, 2843171128437378305, 659261336235719939, 15888093518693932286, 18100861432339498237, 16256306977576410369, 256755195427953930, 15503901920930204936, 11946772765689304330, 16629229153653667084, 1503547606297345302, 4262531731244338454, 2959504361434481941, 9750750580565810453, 12796342029096240402, 13924730570577827093, 5791459803420810527, 6032326966020521250, 1818286151355860261, 12343352861740995873, 17448758893260844322, 17360272800860321063, 5689521881193790766, 6723501248259542323, 11332742041360569650, 13178228647785758002, 17059067918532369713, 6781579829513274675, 854533407933224250, 12458712469171565873],
    [6341039114913572160, 7819349354045390145, 1872236593283212613, 3682273995061531974, 10766312853072721223, 8754844098836147530, 13179269264009608522, 16197954497732289865, 7849020574303452492, 16005927304428070223, 10470096322635773268, 11236151686239323478, 6599209895061925212, 6110009744520666464, 6034659474359072102, 3088319865940473191, 17807298010106353002, 6872576250957139314, 16840039091481206125, 14325035935339746672, 3107325870533016950, 8879934891640823153, 1264758461680917881, 10056717761881380213, 14394501835366532467, 2186926422059978101, 8353953506336517500, 8924006704642239869, 16176902645066380664, 1906262301388651903, 14196553440274711938, 15720190594803533187, 14537365687752134020, 14863728740989531525, 5139785621704648076, 7160634759195698573, 13399392708538342798, 10315513423263602068, 17581962341893770645, 9773291108953742747, 16592638103579389338, 12468378054225272221, 17364752495121046941, 8309127024320370084, 14798393229147274659, 16022980705620684196, 10548002393809530280, 15303110096481662375, 2117362049877736884, 12002723523486029233, 10134345720421289395, 12856850706279476658, 13268697466940437943, 11170889731643121080, 12409119493575493051, 2709975796665214403, 1124827561195406788, 8189956524698744268, 18408189658288887245, 9184243205245552081, 12155061266607517138, 3847648588959509975, 1086914863524447708, 18099934882313102805],
    [1037312343105846750, 3599757885042879968, 16904441819030506973, 17849497173010349533, 4768731163075880423, 4353151619674973673, 4587496169140825584, 3047395582694034933, 3861432453229614582, 6931320591509411319, 13364352534253874686, 4899353800973893123, 3884068798775084548, 12133816350612567554, 461221295579405837, 10502441238531364362, 15271879548640435724, 12797094251429846544, 5531557578977664532, 2931047766294283802, 17154471544058719773, 11122338154320533028, 3192015401022713386, 1806635477748858414, 12815574409272069674, 2000269018185420337, 16581275319563193903, 1162748969084491320, 18074443301795276338, 14611490467242044979, 15897943106663904824, 6250781428758496832, 1563554617273568836, 214009649636322886, 14525484661536007746, 8970482080464862795, 15069916129193291337, 17008612840991512144, 15296951957533851226, 6829041621631977055, 14586382254178023005, 4799965467374231138, 8619953775273428579, 996631696933232232, 2050511281400628842, 15832748036036124263, 10983372373732916844, 4404600357465351792, 16460591470532714091, 13157897902841757299, 8088105355081719415, 12923326300410322551, 4685283689495252606, 9757698873134773891, 14071567477914650243, 8010887325982801545, 13398721912601630345, 14791331686706067081, 13282586157321305739, 15619357867280131720, 6817529997448513168, 3310439771745859216, 1008315421435240086, 1214478173824308887],
    [14537803465971304085, 5652551435968016027, 11552387235919373979, 4308225246530541219, 2643099164961826468, 8913714198291162788, 12547146751203632803, 14028579359297951396, 13363378859129640621, 3926086054506772148, 5327321176928149175, 17060455680648169140, 11897072282964512440, 1838925439081340605, 13338122597719486142, 8429120141180815041, 5446191478309009093, 1752944410447273672, 179598144516517576, 5191324932419182284, 10967406589423769292, 11220479116220155601, 8776525784511573721, 17237230509018859225, 17060529901732118234, 17167721739470444255, 12650048843199117026, 949254490753986285, 9824972666723235565, 446433832567873265, 3475098136860276466, 1367653701814161142, 2648133058902095611, 11841045868815404794, 6278659622034114302, 4466873654389548802, 845736715141883653, 1128721162213814021, 16870529243225081600, 8679844622646935305, 16114480127817291526, 16265927389308667655, 4924572009236692749, 16513684413924024069, 4785274407641333523, 9657155773721235224, 12377041981044014872, 12380992135733153561, 16489121521231406870, 9553541099091547932, 4392210721999314719, 10085882644899075870, 16441666535836639004, 17038312552690331429, 501924748820307758, 10499167334388473644, 5504575555688955695, 7805652386728443695, 3910628628032792368, 15653141389013393194, 5977029956039737142, 11210332348363841333, 11987001279636512565, 17947699322557318963],
    [4660127085923196729, 16785054916306405174, 3685664806274473786, 14190926204714170170, 579249469594361667, 8409489474560255809, 31111741133948739, 1540709693964414793, 273085668238890829, 12770662923023222600, 4777046139954476885, 17954623474490875730, 18377168615091382102, 9932154910145982299, 4094270772317212514, 17018582225103535965, 14904533264620715873, 17355953686241029988, 3802438365776416620, 17696053208614561641, 15285175273739702131, 2319968056078522234, 17791006394067443574, 16403305591660398460, 5057621397855516550, 3925880867078223753, 4634446603694165899, 11305194330291142540, 10925494190404878224, 15567253829896329111, 462821187473158046, 11559483375639396256, 5282223731904401316, 15697987137564100516, 361281199473510316, 1403426366528020397, 11293638222552278954, 12354143298907719606, 5869122978851200953, 17212746319855259578, 9452306128904949694, 639116363522533313, 15943518999560597438, 13753340083909371842, 9547811791824170960, 5829461101155387346, 12435311333191405523, 11937453718821994452, 15778771522464366548, 17356552932406300631, 13331378199518752731, 4082607688427253727, 1984143784688891874, 14331830344533848029, 1283041439174055910, 8056476534048495587, 6444311226180335591, 11338551862556741606, 13034499135150188517, 14016330813983678442, 2515645680388028404, 9792757945889232886, 2545434581273282556, 11903030693198036986]
];

/// A Zobrist hash key for the en passant square.
/// This key is included in the hash only if there is a valid en passant target square.
pub const KEY_ENPASSANT: [u64; Square::NUM_SQUARES] = [
    3840354564925551364, 1202154793604811782, 13654224798553701761, 7354198219890022024, 6569924582553231498, 17554707050661655435, 3897638645021383572, 18037354743283150735, 3022262315982861212, 17827550807876524439, 7607831110120964124, 8530196377218468125, 18175108581333883032, 829105145903748386, 13721534036145751450, 9573220725487527708, 1869052667076657061, 16714101591747572767, 10680031644403748515, 13620461124378117667, 13969977712464483107, 2629969884058745899, 14800573243308448807, 991118716773351347, 17718323058910043055, 153426529989605302, 4709719631268387511, 7244812618890745019, 6534260425699003837, 10398010681030774458, 16458087460518948791, 3333298769183840194, 17004250518568273214, 4959292261086321988, 6079113748582162501, 4629840142902456390, 7508228666971214148, 9578969851287660486, 3355203475256001740, 10796448445994514121, 13367971761005407690, 16846931211435447754, 12649746670079154126, 691312425345178964, 12538838421150895952, 13540850146056241877, 12288258341851473111, 2052462525826066143, 6872692655644893920, 17973276677156799196, 10383722692423724254, 8000886540850003809, 13939480053967654753, 14757812418552713570, 15711082446521932007, 1595619384399569646, 16716318624197119590, 11757936597407337070, 15832095743774343662, 3424961061123640565, 9197118366803623156, 18312807312204078065, 16887222619517978101, 4389598213882121212
];

/// A Zobrist hash key to represent the castling rights in the position.
/// This key is used in combination with other elements such as the piece
/// positions and the side to move to calculate the complete Zobrist hash.
pub const KEY_CASTLE: [u64; CastleRights::NUM_CASTLING_RIGHTS] = [
    15514870633266398266, 4145308009146480642, 1368927690897079779, 9219866075479163426, 18135025827871201084, 9252195192974115523, 16055831919479314978, 7099352943904536037, 14227028345699399555, 17024309137276059108, 3699590268613639980, 7078737726619998058, 5200215836309567793, 17008814149287031565, 10377147447733916373, 2994850491838402462
];

/// A Zobrist hash key for the side to move.
/// This key is XORed into the hash when it's White's turn to move.
/// It's omitted when Black is to move, ensuring the hash differs between
/// different players' turns.
pub const KEY_SIDE: u64 = 5862962466813393681;

/// `Zobrist` is a struct that stores a Zobrist hash value, which is a 64-bit
/// integer used to uniquely identify a specific board position in chess. 
/// Zobrist hashing is efficient for calculating incremental changes in 
/// chess board positions during moves.
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Zobrist(pub u64);

/// Formats the `Zobrist` hash as a hexadecimal string.
///
/// This implementation converts the 64-bit internal value into a
/// fixed-length hexadecimal string (16 characters), with leading zeros
/// if necessary.
impl fmt::Display for Zobrist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl Zobrist {
    
    /// Creates a `Zobrist` instance with a hash value of zero.
    #[inline(always)]
    pub const fn null() -> Self {
        Self(0)
    }

    /// Updates the Zobrist hash to reflect the placement or removal of a piece
    /// on a given square. The hash is updated by XOR-ing the current value with
    /// a precomputed key for the given piece and square.
    #[inline(always)]
    pub fn hash_piece(&mut self, piece: Piece, square: Square) {
        self.0 ^= KEY_PIECE_SQUARE[piece.to_index()][square.to_index()]
    }

    /// Updates the Zobrist hash to reflect the en passant square. The hash is
    /// updated by XOR-ing the current value with a precomputed key for the
    /// en passant square.
    #[inline(always)]
    pub fn hash_enpassant(&mut self, square: Square) {
        self.0 ^= KEY_ENPASSANT[square.to_index()]
    }

    /// Updates the Zobrist hash to reflect changes in the castling rights. The
    /// hash is updated by XOR-ing the current value with a precomputed key for
    /// the current castling rights.
    #[inline(always)]
    pub fn hash_castle(&mut self, castle: CastleRights) {
        self.0 ^= KEY_CASTLE[castle.to_index()]
    }

    /// Updates the Zobrist hash to reflect a change in the side to move. The hash
    /// is updated by XOR-ing the current value with a predefined key for the side.
    #[inline(always)]
    pub fn hash_side(&mut self) {
        self.0 ^= KEY_SIDE
    }


}

#[test]
fn test_zobrist(){
    let zobrist: Zobrist = Zobrist::null();
    println!("{}", zobrist);
}