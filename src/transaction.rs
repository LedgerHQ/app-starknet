use crate::{context::{
    FieldElement,
    Ctx,
}, utils::print::{printf, printf_slice}};

use crate::crypto::pedersen::{
    get_selector_from_name,
    pedersen_hash,
    pedersen_shift
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

    ctx.tx_info.calldata.call_array_len = call_array_len.into();
    ctx.tx_info.calldata.calldata_len = calldata_len.into();

    printf("hash = ");
    printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
    printf("\n");

    printf("callarray_length: ");
    printf_slice::<64>(&ctx.tx_info.calldata.call_array_len.value[..]);
    printf("\n");
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata.call_array_len);
    printf("hash = ");
    printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
    printf("\n");
}

pub fn set_tx_callarray(data: &mut &[u8], ctx: &mut Ctx, n: usize) {
    
    let (to, data) = data.split_at(32);
    ctx.tx_info.calldata.calls[n].to = to.into();

    printf("to: ");
    printf_slice::<64>(&ctx.tx_info.calldata.calls[n].to.value[..]);
    printf("\n");
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata.calls[n].to);
    printf("hash = ");
    printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
    printf("\n");
    
    let (entry_point_length, data) = data.split_first().unwrap();
    ctx.tx_info.calldata.calls[n].entry_point_length = *entry_point_length;

    let (entry_point, data) = data.split_at(*entry_point_length as usize);
    
    let m = core::str::from_utf8(entry_point).unwrap();
    printf("Entry point:\n");
    printf(m);
    printf("\n");
    printf("store entry point\n");
    
    ctx.tx_info.calldata.calls[n].entry_point[..entry_point.len()].clone_from_slice(entry_point);

    ctx.tx_info.calldata.calls[n].selector = get_selector_from_name(entry_point, entry_point.len() as u32);
    
    printf("selector: ");
    printf_slice::<64>(&ctx.tx_info.calldata.calls[n].selector.value[..]);
    printf("\n");
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata.calls[n].selector);
    printf("hash = ");
    printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
    printf("\n");


    let (data_offset, data_len) = data.split_at(32);
    ctx.tx_info.calldata.calls[n].data_offset = data_offset.into();

    printf("data_offset: ");
    printf_slice::<64>(&ctx.tx_info.calldata.calls[n].data_offset.value[..]);
    printf("\n");
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata.calls[n].data_offset);
    printf("hash = ");
    printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
    printf("\n");
    
    ctx.tx_info.calldata.calls[n].data_len = data_len.into();

    printf("data_len: ");
    printf_slice::<64>(&ctx.tx_info.calldata.calls[n].data_len.value[..]);
    printf("\n");
    pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata.calls[n].data_len);
    printf("hash = ");
    printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
    printf("\n");

}

pub fn set_tx_calldata(data: &mut &[u8], ctx: &mut Ctx, c: usize){

    if c == 0 {

        printf("calldata_len: ");
        printf_slice::<64>(&ctx.tx_info.calldata.calldata_len.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.calldata_hash, &ctx.tx_info.calldata.calldata_len);
        printf("hash = ");
        printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
        printf("\n");
    }
    
    let data_len: u8 = ctx.tx_info.calldata.calls[c].data_len.into();
    for _i in 0..data_len {
        let s = &data[..32];
        printf("calldata i: ");
        printf_slice::<64>(s);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.calldata_hash, &s.into());
        printf("hash = ");
        printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
        printf("\n");
        *data = &data[32..];
    }

    let call_array_len : u8 = ctx.tx_info.calldata.call_array_len.into();
    let calldata_len: u8 = ctx.tx_info.calldata.calldata_len.into();
    if c + 1 == call_array_len as usize {
        
        let total_len = 1u8 + call_array_len * 4 + 1u8 + calldata_len; 
        let mut n: FieldElement = total_len.into();

        printf("n: ");
        printf_slice::<64>(&n.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.calldata_hash, &n);
        printf("hash = ");
        printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
        printf("\n");

        printf("Calldata Hash:\n");
        printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
        printf("\n");

        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        n = FieldElement::INVOKE;
        printf("Invoke: ");
        printf_slice::<64>(&n.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &n);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        printf("Version: ");
        printf_slice::<64>(&ctx.tx_info.version.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.version);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        printf("Sender: ");
        printf_slice::<64>(&ctx.tx_info.sender_address.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.sender_address);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        n = FieldElement::ZERO;
        printf("Zero: ");
        printf_slice::<64>(&n.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &n);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        printf("Calldata hash: ");
        printf_slice::<64>(&ctx.hash_info.calldata_hash.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.hash_info.calldata_hash);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        printf("Max fee: ");
        printf_slice::<64>(&ctx.tx_info.max_fee.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.max_fee);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        printf("Chain ID: ");
        printf_slice::<64>(&ctx.tx_info.chain_id.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.chain_id);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        printf("Nonce: ");
        printf_slice::<64>(&ctx.tx_info.nonce.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &ctx.tx_info.nonce);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        n = 8.into();
        printf("Total: ");
        printf_slice::<64>(&n.value[..]);
        printf("\n");
        pedersen_hash(&mut ctx.hash_info.m_hash, &n);
        printf("Tx Hash = ");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");

        pedersen_shift(&mut ctx.hash_info.m_hash);

        printf("Hash:\n");
        printf_slice::<64>(&ctx.hash_info.m_hash.value[..]);
        printf("\n");
    }

}

