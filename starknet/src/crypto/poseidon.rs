// Code ported from the implementation here:
// https://github.com/lambdaclass/lambdaworks/blob/dfe4f0eaf9b0d62d85e881c87622c37f1e1b2ea2/crypto/src/hash/poseidon/mod.rs

use crate::types::FieldElement;
extern crate alloc;
use crate::types::P;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use ledger_secure_sdk_sys::*;

const RATE: usize = 2;
const CAPACITY: usize = 1;
//const ALPHA: u32 = 3;
const N_FULL_ROUNDS: usize = 8;
const N_PARTIAL_ROUNDS: usize = 83;
const STATE_SIZE: usize = RATE + CAPACITY;

//const N_ROUND_CONSTANTS_ROWS: usize = 91;
const N_ROUND_CONSTANTS_COLS: usize = 3;
// The following constants are used for an optimized version of Poseidon hash, as suggested in Appendix B from
// the Poseidon paper (available at https://eprint.iacr.org/2019/458.pdf).
// In partial rounds, instead of adding constants to all the state, we add a constant just to the state
// to which the S box is applied (non-linear). The constants for the other positions are "moved forward" and
// added at the end.
const ROUND_CONSTANTS: [&str; 107] = [
    "06861759ea556a2339dd92f9562a30b9e58e2ad98109ae4780b7fd8eac77fe6f",
    "03827681995d5af9ffc8397a3d00425a3da43f76abf28a64e4ab1a22f27508c4",
    "03a3956d2fad44d0e7f760a2277dc7cb2cac75dc279b2d687a0dbe17704a8309",
    "0626c47a7d421fe1f13c4282214aa759291c78f926a2d1c6882031afe67ef4cd",
    "078985f8e16505035bd6df5518cfd41f2d327fcc948d772cadfe17baca05d6a6",
    "05427f10867514a3204c659875341243c6e26a68b456dc1d142dcf34341696ff",
    "05af083f36e4c729454361733f0883c5847cd2c5d9d4cb8b0465e60edce699d7",
    "07d71701bde3d06d54fa3f74f7b352a52d3975f92ff84b1ac77e709bfd388882",
    "0603da06882019009c26f8a6320a1c5eac1b64f699ffea44e39584467a6b1d3e",
    "04332a6f6bde2f288e79ce13f47ad1cdeebd8870fd13a36b613b9721f6453a5d",
    "053d0ebf61664c685310a04c4dec2e7e4b9a813aaeff60d6c9e8caeb5cba78e7",
    "05346a68894845835ae5ebcb88028d2a6c82f99f928494ee1bfc2d15eaabfebc",
    "04B085EB1DF4258C3453CC97445954BF3433B6AB9DD5A99592864C00F54A3F9A",
    "0731CFD19D508285965F12A079B2A169FDFE0A8E610E6F2D5CA5D7B0961F6D96",
    "0217D08B5339852BCC6F7A774936B3E72ECD9E1F9A73D743F8079C1E3587EEAA",
    "000C935DD633B0FD63599B13C850DAB3CB966BA510C81B20959E267008518C6E",
    "052AF8D378DD6772EE187ED23F79A7D98CF5A0A387103971467FE940E7B8B2BE",
    "0294851C98B2682F1EC9918B9F12FCCEAA6E28A7B79B2E506362CDA595F8AB75",
    "011B59990BACC280824D1021418D4F589DA8C30063471494C204B169AB086064",
    "04B4DF56E3D7753F91960D59AE099B9BEB2CE690E6BBDCD0B599D49CEB2ACD6A",
    "005EECFA15A757DC3ECAE9FBD8FF06E466243534F30629FC5F1CF09EB5161AC4",
    "0680BFDD8B9680E04659227634A1EC5282E5A7CEF81B15677F8448BDA4279059",
    "01D0BF8FAB0A1A7A14E2930794F7A3065C17E10B1CEDD791B8877D97ACD85053",
    "02C2C8C79F808ACE54BA207053C0D412C0FC11A610F14C48876701A37E32F464",
    "0354EC9ED01D20EC52AAE19A9B858D3474D8234C11AD7BCE630AD56C54AFA562",
    "030DF20FCF6427BAC38BB5D1A42287F4E4136AC5892340E994E6EA28DEEC1E55",
    "0528CF329C64E7EE3040BAFBDEFF61E241D99B424091E31472EDA296FC9C6778",
    "040416F24F623534634789660DF5435EBF0C3E0C69E6C5B5FF6E757930BD1960",
    "00380C8F936E2ED9FD488AE3BAC7DCE315BA21B11E88339CD5444435CCC9EA38",
    "01CC4F5D5603D176F1A8E344392EFD2D03AD0541832829D245E0E2291F255B75",
    "05728917AF5DA91F9539310D99F5D142E011D6C8E015EA5423C502AA99C09752",
    "00EFB450A9E86E1A46E295A348F0F23590925107D17C56D7C788FECC17219AA1",
    "02020D74D36C421AE1A025616B342D0784B8FCD977DE6C53A6C26693774DCA99",
    "07CFB309B75FD3BF2705558AE511DC82335050969F4BF84FA2B7B4F583989287",
    "04651E48B2E9349A5365E009ECE626809D7B7D02A617EB98C785A784812D75E9",
    "00D77627B270F65122D0269719DA923CCAE822D9AAD0F0947A3B5C8F71C0DCC7",
    "0199AD3D641B54C4D571B3FE37773A8B82B003377F0DD8B7D3B7758C32908EA8",
    "044F33640A8ECFD3973E2E9172A7333482B2D297BE2DA289319E72D137CDFE6E",
    "07E4ADF9894D964189D00A02DCF1E6BE7F801234F5216EAB6B6F366B6701ABF7",
    "03641FA5B3C90452F5FF808F8A9817EDA7C6AECFB5471DFDCA559FB4E711EE90",
    "03DE5729EFD2FCBD897A49A78FA923FC306DF32E6E2F0E02D0EEE2C2CC3F3533",
    "062691891A3FC1E27F622966CA0BE20C06563500C8F06C9BDB77BD2882D6C994",
    "06608D3BF11C18E4688739F72205763D1590CC4F9885AE1D86E96E0604BAA0BE",
    "011C9C9B39CAC71E3419726CE779116D07249F51CBDDA4FD98C25CBBF593A316",
    "061E23B58203269CAEF0850F74DA27B9748E3312EA40C6844DD68C557C462AD7",
    "04182CD9AB1D9488F870A572010BC2A3D9878440B25951E4CE010855CF83BDC8",
    "0520FE6C4A096793F9055E6823116D15F1DF2FE89D306F9965F6A59F4F3ECB71",
    "0346B2B2D6E5810129E093093DCD3DFA99ED6D71F47723EA3FBE4D4E2FD4AFA1",
    "01359CA923E7F1448EC1DD2A3684BEE4E8B682C8E8E973ACEA72877CE9F7E6CF",
    "047C655F55CF307800DFEFDAD24DE86FDE9DEADAB145A1B392420F37B95D9675",
    "04AB291F16555FA8A968CD7C9C285A9598EFD925F2D58B7AA38AD87DCA8441A8",
    "039F409C7C782101223D1F6F7D86C21A22C44EF959510E392C9C7C5D17C629C5",
    "044BE36B782F882AD86EECB0CD6BEB02E1A2F9FB5587A3BABFACEAD0CAFB6052",
    "050A1DFDE9B504AD2906DB6EB5B507203CD1CEB394C52CE7107679A53A0D538B",
    "05C753C14DA89E287B181C0DD11AC6C3680BDD7F1017DAE083E7AEBBEAB183AB",
    "02CF6306ED32232106C8015A3B180F386EEE93E15F7B4F4FA57746525FC0520C",
    "02C2014634D52E27420873CF347429091DFC6380689BD4F54D7D8E502C1C3A09",
    "03CFB9C5BD93E02B2FDACDE2058E33E5975C446345F010D850FC09CDF86ED8A1",
    "0363FA71A383CF3897933F1411FC5F806E311E84F72CB50A9EA4E1281F6B0299",
    "0728199657067EE16947B3FC76271676B4901B2A3686CFFEBCB960DA91B05DF8",
    "03FDFBD47D27F3D34F0723B728E8921DC9BDE34A9872DF5A652A078D7E4EE021",
    "07F241379440CACD7DC0EFBE7858EB7DE53CC02CA7D24197945C453398EFF449",
    "05B2E8771EA9A0004E3BF056F3727797CBB457A27574D5F104354E52A5C25F0B",
    "00A8DDBCE708DE44A7E0B3B0333146E1E910245BE6BF822EA057A081BDA2E23E",
    "02D521E0DACA24E431AA47CD90A0F551C12270E533835613EDCE2E19AA9B0F61",
    "06CDBC0F2AA54D2CF7D5AC3B93F855AF03EEF7B07AAEE00341A6266C30E08AE6",
    "03DD96A17111EC8F4C5DA3AD6794C0961CEEE452CBE92C7A0941112B36ED9BF3",
    "05EAFB1EDEEDC5C07AC07FDD06159344A2CFB92196A65D9EC0C5E732C36687DC",
    "04AB038D7B09EDA9324577B260FEAEBDBCEC5A7B7C7F449B312CFCD065C207E6",
    "04CA71981E4DF6B505D2B0D94E235608463C58052570F68E495FC80C7FDEF220",
    "06DEE9C6DA4617E32AA419899C8EA8137E9B59D7E2759FFE573C15B77E413D2F",
    "058F9E60B34DDAB84DCBE2396065A4305B4A795A4770E4541E625D0460C6F186",
    "047B7B4A802A10C1E6C9C735DB6C34042D290906F274BEA8FCECEF17FC9AF632",
    "01849BCDB9AD7171096ECC936A186774084A074BE0BFC0FBB9463A06A2BD430C",
    "041870FBE04438348AF5767BDDAECD8AEA3B49B4217547DEC4D699B1466736CC",
    "0226C04E598076A9FA02AA64557DAF28C0EC42E3D4DA68D1965029D284738B07",
    "01F0E971F0485A5B42EB92D6655C3DDB475CEC4371F269A95335B2A7D6DAC0FB",
    "009F31CC2907DCCBF994D35AA47EE3F4EBDF3703F795047A7B40DD3926431563",
    "04B40CCE78F3B641E31CE4DF58CE5A42C22CFBC198C84451FFE8CCA4C64BD7D2",
    "0191660489E4BD8A3E4563173DE4A226F3AC736962FDFB70F72CB93CE50F8B9F",
    "018C0919618DB971F74EB01F293F2DAEA814B475103373DC7ED8DD4C7B467410",
    "035B60253848530E845C8753121577D0EF37002E941C3DC1FB240BD57EADC803",
    "01AE99DB1575AE91C8B43A9F71A5F362581AD9B413D97FA6FD029134957451D5",
    "03E6E1D0F3F8A0F728148EBCBD5D7D337D7CB8FEB58A37D2D1DFB357E172647B",
    "018BC36DFFA8F96A659E1A171B55D2706EE3E9AD619E16F5C38DD1F4A209B8F3",
    "02C7A3EF1AFB6A302B54AFC3A107FF9199A16EFE9A1CC3AB83FA5B64893DE4ED",
    "053A7BD889BED07BF5E27DD8E92F6AE85E4FE4E84B0C6DDE9856E94469DE4BD7",
    "04D383FF7FFC6318FDA704ACA35995F86BEC5A02CE9A0BF9D3CC0CC2F03CCEA9",
    "04667B6762FB8AD53D07EF7E8A65B21CA96E0B3503037710D1292519C326F5CD",
    "002CC8B43E75CF0B42A93C39EA98BCD46055DCCC9589F02EB7FB536422E5921F",
    "06B32EE98680871D38751447BFD76086BA4DF0E7BE59C55F4B2CE25582BF9C60",
    "03E907927C7182FAAA3B3C81358B82E734EFAC1F0609F0862D635CB1387102A3",
    "03F3A5057B3A08975F0253728E512AF78D2F437973F6A93793EA5E8424FBC6EA",
    "014B491D73724779F8AA74B3FD8AA5821C21E1017224726A7A946BB6CA68D8F5",
    "05C8278C7BBFC30AE7F60E514FE3B9367ACA84C54AD1373861695EA4ABB814EF",
    "064851937F9836EE5A08A7DDE65E44B467018A82BA3BF99BBA0B4502755C8074",
    "06A9AC84251294769ECA450FFB52B441882BE77CB85F422FF9EA5E73F1D971DC",
    "037EC35B710B0D04C9A2B71F2F7BD098C6A81D991D27F0FC1884F5CA545064DE",
    "005334f75b052c0235119816883040da72c6d0a61538bdfff46d6a242bfeb7a1",
    "05d0af4fcbd9e056c1020cca9d871ae68f80ee4af2ec6547cd49d6dca50aa431",
    "030131bce2fba5694114a19c46d24e00b4699dc00f1d53ba5ab99537901b1e65",
    "05646a95a7c1ae86b34c0750ed2e641c538f93f13161be3c4957660f2e788965",
    "04b9f291d7b430c79fac36230a11f43e78581f5259692b52c90df47b7d4ec01a",
    "05006d393d3480f41a98f19127072dc83e00becf6ceb4d73d890e74abae01a13",
    "062c9d42199f3b260e7cb8a115143106acf4f702e6b346fd202dc3b26a679d80",
    "051274d092db5099f180b1a8a13b7f2c7606836eabd8af54bf1d9ac2dc5717a5",
    "061fc552b8eb75e17ad0fb7aaa4ca528f415e14f0d9cdbed861a8db0bfff0c5b",
];

#[derive(Clone)]
pub struct PoseidonStark252;

impl PoseidonStark252 {
    pub fn hades_permutation(state: &mut [FieldElement]) {
        let mut index = 0;
        for _ in 0..N_FULL_ROUNDS / 2 {
            Self::full_round(state, index);
            index += N_ROUND_CONSTANTS_COLS;
        }
        Self::partial_round_loop(state, &mut index);
        for _ in 0..N_FULL_ROUNDS / 2 {
            Self::full_round(state, index);
            index += N_ROUND_CONSTANTS_COLS;
        }
    }

    #[inline]
    fn full_round(state: &mut [FieldElement], index: usize) {
        unsafe {
            let mut bn_s0: cx_bn_t = cx_bn_t::default();
            let mut bn_p: cx_bn_t = cx_bn_t::default();
            let mut bn_res: cx_bn_t = cx_bn_t::default();
            let mut bn_rc: cx_bn_t = cx_bn_t::default();
            let mut bn_e: cx_bn_t = cx_bn_t::default();

            cx_bn_lock(32, 0);

            cx_bn_alloc(&mut bn_res, 32);
            cx_bn_alloc_init(&mut bn_p, 32, P.value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_e, 32, [3u8; 1].as_ptr(), 1);
            cx_bn_alloc(&mut bn_s0, 32);
            cx_bn_alloc(&mut bn_rc, 32);

            for (i, value) in state.iter_mut().enumerate() {
                /* state[i] = state[i] + ROUND_CONSTANTS[index + i] */
                cx_bn_init(bn_s0, value.value.as_ptr(), 32);
                cx_bn_init(
                    bn_rc,
                    FieldElement::from(ROUND_CONSTANTS[index + i])
                        .value
                        .as_ptr(),
                    32,
                );
                cx_bn_mod_add(bn_res, bn_s0, bn_rc, bn_p);
                /* state[i] = state[i] * state[i] * state[i] */
                cx_bn_mod_pow_bn(bn_s0, bn_res, bn_e, bn_p);
                cx_bn_export(bn_s0, value.value.as_mut_ptr(), 32);
            }

            /* Mix */
            let mut bn_s1: cx_bn_t = cx_bn_t::default();
            let mut bn_s2: cx_bn_t = cx_bn_t::default();
            let mut bn_t: cx_bn_t = cx_bn_t::default();
            let mut bn_two: cx_bn_t = cx_bn_t::default();
            let mut bn_three: cx_bn_t = cx_bn_t::default();

            cx_bn_alloc(&mut bn_t, 32);
            cx_bn_init(bn_s0, state[0].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_s1, 32, state[1].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_s2, 32, state[2].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_two, 32, FieldElement::TWO.value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_three, 32, FieldElement::THREE.value.as_ptr(), 32);

            /* Compute t */
            cx_bn_mod_add(bn_res, bn_s0, bn_s1, bn_p);
            cx_bn_mod_add(bn_t, bn_res, bn_s2, bn_p);

            /* Update state */
            /* s0 = t + 2 * s0 */
            cx_bn_mod_mul(bn_res, bn_s0, bn_two, bn_p);
            cx_bn_mod_add(bn_s0, bn_t, bn_res, bn_p);

            /* s1 = t - 2 * s1 */
            cx_bn_mod_mul(bn_res, bn_s1, bn_two, bn_p);
            cx_bn_mod_sub(bn_s1, bn_t, bn_res, bn_p);

            /* s2 = t - 3 * s2 */
            cx_bn_mod_mul(bn_res, bn_s2, bn_three, bn_p);
            cx_bn_mod_sub(bn_s2, bn_t, bn_res, bn_p);

            /* Fix Sandra */
            let mut bn_zero: cx_bn_t = cx_bn_t::default();
            cx_bn_alloc_init(&mut bn_zero, 32, FieldElement::ZERO.value.as_ptr(), 32);
            cx_bn_mod_sub(bn_s0, bn_s0, bn_zero, bn_p);

            cx_bn_export(bn_s0, state[0].value.as_mut_ptr(), 32);
            cx_bn_export(bn_s1, state[1].value.as_mut_ptr(), 32);
            cx_bn_export(bn_s2, state[2].value.as_mut_ptr(), 32);

            cx_bn_unlock();
        }
    }

    #[inline]
    fn _partial_round(state: &mut [FieldElement], index: usize) {
        unsafe {
            let mut bn_p: cx_bn_t = cx_bn_t::default();
            let mut bn_res: cx_bn_t = cx_bn_t::default();
            let mut bn_rc: cx_bn_t = cx_bn_t::default();
            let mut bn_e: cx_bn_t = cx_bn_t::default();
            let mut bn_s2: cx_bn_t = cx_bn_t::default();

            cx_bn_lock(32, 0);

            cx_bn_alloc(&mut bn_res, 32);

            /* s2 = s2 + ROUND_CONSTANTS[index] */
            cx_bn_alloc_init(&mut bn_s2, 32, state[2].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_p, 32, P.value.as_ptr(), 32);
            cx_bn_alloc_init(
                &mut bn_rc,
                32,
                FieldElement::from(ROUND_CONSTANTS[index]).value.as_ptr(),
                32,
            );
            cx_bn_mod_add(bn_res, bn_s2, bn_rc, bn_p);

            /* s2 = s2 * s2 * s2 */
            cx_bn_alloc_init(&mut bn_e, 32, [3u8; 1].as_ptr(), 1);
            cx_bn_mod_pow_bn(bn_s2, bn_res, bn_e, bn_p);

            /**** Mix ****/
            let mut bn_s0: cx_bn_t = cx_bn_t::default();
            let mut bn_s1: cx_bn_t = cx_bn_t::default();
            let mut bn_t: cx_bn_t = cx_bn_t::default();
            let mut bn_two: cx_bn_t = cx_bn_t::default();
            let mut bn_three: cx_bn_t = cx_bn_t::default();

            cx_bn_alloc(&mut bn_t, 32);
            cx_bn_alloc_init(&mut bn_s0, 32, state[0].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_s1, 32, state[1].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_two, 32, FieldElement::TWO.value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_three, 32, FieldElement::THREE.value.as_ptr(), 32);

            /* Compute t */
            cx_bn_mod_add(bn_res, bn_s0, bn_s1, bn_p);
            cx_bn_mod_add(bn_t, bn_res, bn_s2, bn_p);

            /* Update state */
            /* s0 = t + 2 * s0 */
            cx_bn_mod_mul(bn_res, bn_s0, bn_two, bn_p);
            cx_bn_mod_add(bn_s0, bn_t, bn_res, bn_p);

            /* s1 = t - 2 * s1 */
            cx_bn_mod_mul(bn_res, bn_s1, bn_two, bn_p);
            cx_bn_mod_sub(bn_s1, bn_t, bn_res, bn_p);

            /* s2 = t - 3 * s2 */
            cx_bn_mod_mul(bn_res, bn_s2, bn_three, bn_p);
            cx_bn_mod_sub(bn_s2, bn_t, bn_res, bn_p);

            /* Fix Sandra */
            let mut bn_zero: cx_bn_t = cx_bn_t::default();
            cx_bn_alloc_init(&mut bn_zero, 32, FieldElement::ZERO.value.as_ptr(), 32);
            cx_bn_mod_sub(bn_s0, bn_s0, bn_zero, bn_p);

            cx_bn_export(bn_s0, state[0].value.as_mut_ptr(), 32);
            cx_bn_export(bn_s1, state[1].value.as_mut_ptr(), 32);
            cx_bn_export(bn_s2, state[2].value.as_mut_ptr(), 32);

            cx_bn_unlock();
        }
    }

    #[inline]
    fn partial_round_loop(state: &mut [FieldElement], index: &mut usize) {
        unsafe {
            let mut bn_p: cx_bn_t = cx_bn_t::default();
            let mut bn_res: cx_bn_t = cx_bn_t::default();
            let mut bn_rc: cx_bn_t = cx_bn_t::default();
            let mut bn_e: cx_bn_t = cx_bn_t::default();
            let mut bn_s2: cx_bn_t = cx_bn_t::default();
            let mut bn_s0: cx_bn_t = cx_bn_t::default();
            let mut bn_s1: cx_bn_t = cx_bn_t::default();
            let mut bn_t: cx_bn_t = cx_bn_t::default();
            let mut bn_two: cx_bn_t = cx_bn_t::default();
            let mut bn_three: cx_bn_t = cx_bn_t::default();
            let mut bn_zero: cx_bn_t = cx_bn_t::default();

            cx_bn_lock(32, 0);

            cx_bn_alloc(&mut bn_res, 32);

            cx_bn_alloc_init(&mut bn_s0, 32, state[0].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_s1, 32, state[1].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_s2, 32, state[2].value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_p, 32, P.value.as_ptr(), 32);
            cx_bn_alloc(&mut bn_rc, 32);
            cx_bn_alloc_init(&mut bn_e, 32, [3u8; 1].as_ptr(), 1);
            cx_bn_alloc(&mut bn_t, 32);
            cx_bn_alloc_init(&mut bn_two, 32, FieldElement::TWO.value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_three, 32, FieldElement::THREE.value.as_ptr(), 32);
            cx_bn_alloc_init(&mut bn_zero, 32, FieldElement::ZERO.value.as_ptr(), 32);

            for _ in 0..N_PARTIAL_ROUNDS {
                /* s2 = s2 + ROUND_CONSTANTS[index] */
                cx_bn_init(
                    bn_rc,
                    FieldElement::from(ROUND_CONSTANTS[*index]).value.as_ptr(),
                    32,
                );
                cx_bn_mod_add(bn_res, bn_s2, bn_rc, bn_p);

                /* s2 = s2 * s2 * s2 */
                cx_bn_mod_pow_bn(bn_s2, bn_res, bn_e, bn_p);

                /**** Mix ****/

                /* Compute t */
                cx_bn_mod_add(bn_res, bn_s0, bn_s1, bn_p);
                cx_bn_mod_add(bn_t, bn_res, bn_s2, bn_p);

                /* Update state */
                /* s0 = t + 2 * s0 */
                cx_bn_mod_mul(bn_res, bn_s0, bn_two, bn_p);
                cx_bn_mod_add(bn_s0, bn_t, bn_res, bn_p);

                /* s1 = t - 2 * s1 */
                cx_bn_mod_mul(bn_res, bn_s1, bn_two, bn_p);
                cx_bn_mod_sub(bn_s1, bn_t, bn_res, bn_p);

                /* s2 = t - 3 * s2 */
                cx_bn_mod_mul(bn_res, bn_s2, bn_three, bn_p);
                cx_bn_mod_sub(bn_s2, bn_t, bn_res, bn_p);

                /* Fix Sandra */
                cx_bn_mod_sub(bn_s0, bn_s0, bn_zero, bn_p);

                *index += 1;
            }

            cx_bn_export(bn_s0, state[0].value.as_mut_ptr(), 32);
            cx_bn_export(bn_s1, state[1].value.as_mut_ptr(), 32);
            cx_bn_export(bn_s2, state[2].value.as_mut_ptr(), 32);

            cx_bn_unlock();
        }
    }

    #[cfg(feature = "poseidon")]
    pub fn hash(x: &FieldElement, y: &FieldElement) -> FieldElement {
        let mut state: Vec<FieldElement> = Vec::with_capacity(3);
        state.push(x.clone());
        state.push(y.clone());
        state.push(FieldElement::from(2u8));
        Self::hades_permutation(&mut state);
        let x = &state[0];
        x.clone()
    }

    #[cfg(feature = "poseidon")]
    pub fn hash_single(x: &FieldElement) -> FieldElement {
        let mut state: Vec<FieldElement> = Vec::with_capacity(3);
        state.push(x.clone());
        state.push(FieldElement::ZERO);
        state.push(FieldElement::ONE);
        Self::hades_permutation(&mut state);
        let x = &state[0];
        x.clone()
    }

    pub fn hash_many(inputs: &[FieldElement]) -> FieldElement {
        let r = RATE; // chunk size

        // Pad input with 1 followed by 0's (if necessary).
        let mut values = inputs.to_owned();
        values.push(FieldElement::from(1u8));
        values.resize(((values.len() + r - 1) / r) * r, FieldElement::ZERO);

        assert!(values.len() % r == 0);
        let mut state: Vec<FieldElement> = Vec::from([FieldElement::ZERO; STATE_SIZE]);

        // Process each block
        for block in values.chunks(r) {
            let mut block_state: Vec<FieldElement> = state[0..r]
                .iter()
                .zip(block)
                .map(|(s, b)| *s + *b)
                .collect();

            block_state.extend_from_slice(&state[r..]);

            Self::hades_permutation(&mut block_state);
            state = block_state;
        }

        state[0]
    }
}

// Code ported from the implementation here:
// https://github.com/xJonathanLEI/starknet-rs/blob/7bb13d3f02f23949cf3c263e1b53ffcc43990ce6/starknet-crypto/src/poseidon_hash.rs#L13
#[derive(Debug, Default)]
pub struct PoseidonHasher {
    state: [FieldElement; 3],
    buffer: Option<FieldElement>,
}

impl PoseidonHasher {
    /// Creates a new [PoseidonHasher].
    pub fn new() -> Self {
        Self::default()
    }

    /// Absorbs message into the hash.
    pub fn update(&mut self, msg: FieldElement) {
        match self.buffer.take() {
            Some(previous_message) => {
                self.state[0] += previous_message;
                self.state[1] += msg;
                PoseidonStark252::hades_permutation(&mut self.state);
            }
            None => {
                self.buffer = Some(msg);
            }
        }
    }

    /// Finishes and returns hash.
    pub fn finalize(mut self) -> FieldElement {
        // Applies padding
        match self.buffer.take() {
            Some(last_message) => {
                self.state[0] += last_message;
                self.state[1] += FieldElement::ONE;
            }
            None => {
                self.state[0] += FieldElement::ONE;
            }
        }
        PoseidonStark252::hades_permutation(&mut self.state);

        self.state[0]
    }
}

pub fn poseidon_shift(hash: &mut FieldElement) {
    let mut hash256: cx_bn_t = cx_bn_t::default();

    unsafe {
        cx_bn_lock(32, 0);
        cx_bn_alloc_init(&mut hash256, 32, hash.value[..].as_ptr(), 32);

        let mut bits_count: u32 = 256;
        let mut set: bool = false;
        while bits_count > 0 {
            cx_bn_tst_bit(hash256, bits_count - 1, &mut set);
            if set {
                break;
            } else {
                bits_count -= 1;
            }
        }

        if bits_count > 252 {
            cx_bn_unlock();
        } else {
            cx_bn_shl(hash256, 4);
            cx_bn_export(hash256, hash.value[..].as_mut_ptr(), 32);
            cx_bn_destroy(&mut hash256);
            cx_bn_unlock();
        }
    }
}
