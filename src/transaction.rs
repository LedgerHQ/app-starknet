use crate::{context::{
    Ctx,
}, utils::print::{printf, printf_fe}, display::sign_tx_ui};

use crate::crypto::pedersen::{
    get_selector_from_name,
    pedersen_hash,
    pedersen_shift
};

use nanos_sdk::starknet::{
    FieldElement
};

pub fn set_tx_fields(data: &mut &[u8], ctx: &mut Ctx) {
    
    let (sender, data) = data.split_at(32);
    ctx.tx_info.sender_address = sender.into();

    let (max_fee, data) = data.split_at(32);
    ctx.tx_info.max_fee = max_fee.into();

    let (chain_id, data) = data.split_at(32);
    ctx.tx_info.chain_id = chain_id.into();

    let (nonce, version) = data.split_at(32);
    ctx.tx_info.nonce = nonce.into();
    ctx.tx_info.version = version.into();

}

pub fn set_tx_calldata_lengths(data: &mut &[u8], ctx: &mut Ctx) {

    let (call_array_len, calldata_len) = data.split_at(32);

    ctx.tx_info.calldata_v0.call_array_len = call_array_len.into();
    ctx.tx_info.calldata_v0.calldata_len = calldata_len.into();

    //printf_fe("callarray_length: ", &ctx.tx_info.calldata.call_array_len);
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata_v0.call_array_len);
}

pub fn set_tx_callarray(data: &mut &[u8], ctx: &mut Ctx, n: usize) {
    
    let (to, data) = data.split_at(32);
    ctx.tx_info.calldata_v0.calls[n].to = to.into();

    //printf_fe("to: ", &ctx.tx_info.calldata.calls[n].to);
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata_v0.calls[n].to);
    
    let (entry_point_length, data) = data.split_first().unwrap();
    ctx.tx_info.calldata_v0.calls[n].entry_point_length = *entry_point_length;

    let (entry_point, data) = data.split_at(*entry_point_length as usize);
    
    //let m = core::str::from_utf8(entry_point).unwrap();
    //printf("Entry point: ");
    //printf(m);
    //printf("\n");
    //printf("store entry point\n");
    
    ctx.tx_info.calldata_v0.calls[n].entry_point[..entry_point.len()].clone_from_slice(entry_point);

    ctx.tx_info.calldata_v0.calls[n].selector = get_selector_from_name(entry_point, entry_point.len() as u32);
    
    //printf_fe("selector: ", &ctx.tx_info.calldata.calls[n].selector);
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata_v0.calls[n].selector);

    let (data_offset, data_len) = data.split_at(32);
    ctx.tx_info.calldata_v0.calls[n].data_offset = data_offset.into();

    //printf_fe("data_offset: ", &ctx.tx_info.calldata.calls[n].data_offset);
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata_v0.calls[n].data_offset);
    
    ctx.tx_info.calldata_v0.calls[n].data_len = data_len.into();

    //printf_fe("data_len: ", &ctx.tx_info.calldata.calls[n].data_len);
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata_v0.calls[n].data_len);

}

pub fn set_tx_calldata(data: &[u8], ctx: &mut Ctx, c: usize) -> Result<bool, ()> {

    if c == 0 {
        //printf_fe("calldata_len: ", &ctx.tx_info.calldata.calldata_len);
        pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata_v0.calldata_len);

         /* display calldata info start */
        printf("####################\n"); 
        printf_fe("Account: ", &ctx.tx_info.sender_address);
        printf_fe("Max Fee: ", &ctx.tx_info.max_fee);
        printf_fe("Nonce: ", &ctx.tx_info.nonce);
        printf("####################\n"); 
        /* display calldata info end */
    }

    let data_len: u8 = ctx.tx_info.calldata_v0.calls[c].data_len.into();

    /* display calldata info start */
    printf(">>>>>>>>>>>>>>>>>>>>>\n");
    printf_fe("to: ", &ctx.tx_info.calldata_v0.calls[c].to);
    printf(core::str::from_utf8(&ctx.tx_info.calldata_v0.calls[c].entry_point).unwrap());
    printf("\n");
    printf_fe("selector: ", &ctx.tx_info.calldata_v0.calls[c].selector); 
    /* display calldata info end */

    let mut s_start: usize;
    let mut s_end: usize;
    let mut s: &[u8];
    for i in 0..data_len {
        s_start = (i * 32).into();
        s_end = s_start + 32;
        s = &data[s_start..s_end];
        printf_fe("calldata #i: ", &s.into());
        pedersen_hash(&mut ctx.hash_info.calldata_hash, &s.into());
    }
    printf(">>>>>>>>>>>>>>>>>>>>>\n");

    let call_array_len : u8 = ctx.tx_info.calldata_v0.call_array_len.into();
    let calldata_len: u8 = ctx.tx_info.calldata_v0.calldata_len.into();
    if c + 1 == call_array_len as usize {
        
        let total_len = 1u8 + call_array_len * 4 + 1u8 + calldata_len; 
        let mut n: FieldElement = total_len.into();

        //printf_fe("n: ", &n);
        pedersen_hash(&mut ctx.hash_info.calldata_hash, &n);

        //printf_fe("Calldata Hash: ", &ctx.hash_info.calldata_hash);

        //printf_fe("Tx Hash = ", &ctx.hash_info.m_hash);

        n = FieldElement::INVOKE;
        //printf_fe("Invoke: ", &n);
        pedersen_hash(&mut ctx.hash_info.m_hash, &n);

        //printf_fe("Version: ", &ctx.tx_info.version);
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.version);

        //printf_fe("Sender: ", &ctx.tx_info.sender_address);
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.sender_address);

        n = FieldElement::ZERO;
        //printf_fe("Zero: ", &n);
        pedersen_hash(&mut ctx.hash_info.m_hash, &n);

        //printf_fe("Calldata hash: ", &ctx.hash_info.calldata_hash);
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.hash_info.calldata_hash);

        //printf_fe("Max fee: ", &ctx.tx_info.max_fee);
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.max_fee);

        //printf_fe("Chain ID: ", &ctx.tx_info.chain_id);
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.chain_id);

        //printf_fe("Nonce: ", &ctx.tx_info.nonce);
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.nonce);

        n = 8.into();
        //printf_fe("Total: ", &n);
        pedersen_hash(&mut ctx.hash_info.m_hash, &n);

        pedersen_shift(&mut ctx.hash_info.m_hash);

        printf_fe("Tx Hash: ", &ctx.hash_info.m_hash);
    }

    sign_tx_ui(&ctx.tx_info, c, data)

}