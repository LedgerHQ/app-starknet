use crate::apdu::{Apdu, ApduHeader};
use crate::types::{
    Call, DeployAccountV1, DeployAccountV3, Ins, InvokeV1, InvokeV3, ResourceBounds,
};
use starknet_types_core::felt::Felt;

pub enum ApduError {
    InternalError,
}

pub fn data_to_apdu(data: Vec<Felt>, cla: u8, ins: u8, p1: u8, p2: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2,
    };
    let mut apdu = Apdu::new(apdu_header);

    for felt in data {
        let arr = felt.to_bytes_be();
        apdu.append(&arr[..]).unwrap();
    }
    apdu
}

pub fn hash_to_apdu(hash: &str, cla: u8, ins: Ins, sub_ins: u8, show_hash: bool) -> Apdu {
    let header: ApduHeader = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1: sub_ins,
        p2: match show_hash {
            true => 0x01,
            false => 0x00,
        },
    };
    let mut apdu = Apdu::new(header);
    // Convert the hash string to a Felt and then to bytes
    let data: [u8; 32] = Felt::from_hex_unchecked(hash).to_bytes_be();
    apdu.append(&data[..]).unwrap();
    apdu
}

/// Build Derivation path APDU
pub fn derivation_path(path: &str, cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    let mut bip32_path: Vec<u32> = Vec::new();
    if let Some(spath) = path.strip_prefix("m/") {
        for s in spath.split('/') {
            let val: u32 = match s.ends_with('\'') {
                true => 0x80000000 + s.strip_suffix('\'').unwrap().parse::<u32>().unwrap(),
                false => s.parse::<u32>().unwrap(),
            };
            bip32_path.push(val);
        }
        for val in bip32_path {
            apdu.append(val.to_be_bytes().as_slice()).unwrap();
        }
    }
    apdu
}

pub fn tx_fields_invoke_v1(tx: &InvokeV1, cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    let mut fe = Felt::from_hex_unchecked(&tx.sender_address);
    let mut data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.max_fee);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.chain_id);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.nonce);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn tx_fields_invoke_v3(tx: &InvokeV3, cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    let mut fe = Felt::from_hex_unchecked(&tx.sender_address);
    let mut data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.chain_id);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.nonce);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.data_availability_mode);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn tx_fields_deploy_v3(tx: &DeployAccountV3, cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    let mut fe = Felt::from_hex_unchecked(&tx.contract_address);
    let mut data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.chain_id);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.nonce);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.data_availability_mode);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.class_hash);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.contract_address_salt);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn tx_fields_deploy_v1(tx: &DeployAccountV1, cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    let mut fe = Felt::from_hex_unchecked(&tx.contract_address);
    let mut data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.class_hash);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.contract_address_salt);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.max_fee);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.chain_id);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    fe = Felt::from_hex_unchecked(&tx.nonce);
    data = fe.to_bytes_be();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn tx_fees(tip: &str, resources: &ResourceBounds, cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    // Tip
    let _ = apdu.append(Felt::from_hex_unchecked(tip).to_bytes_be().as_slice());

    // L1 Gas
    let mut resource_buffer = [0; 32];
    resource_buffer[2..8].copy_from_slice(b"L1_GAS");
    resource_buffer[8..16].copy_from_slice(
        &u64::from_str_radix(resources.l1_gas.max_amount.trim_start_matches("0x"), 16)
            .unwrap()
            .to_be_bytes(),
    );
    resource_buffer[16..].copy_from_slice(
        &u128::from_str_radix(
            resources.l1_gas.max_price_per_unit.trim_start_matches("0x"),
            16,
        )
        .unwrap()
        .to_be_bytes(),
    );
    let _ = apdu.append(&resource_buffer).unwrap();

    // L2 Gas
    let mut resource_buffer = [0; 32];
    resource_buffer[2..8].copy_from_slice(b"L2_GAS");
    resource_buffer[8..16].copy_from_slice(
        &u64::from_str_radix(resources.l2_gas.max_amount.trim_start_matches("0x"), 16)
            .unwrap()
            .to_be_bytes(),
    );
    resource_buffer[16..].copy_from_slice(
        &u128::from_str_radix(
            resources.l2_gas.max_price_per_unit.trim_start_matches("0x"),
            16,
        )
        .unwrap()
        .to_be_bytes(),
    );
    let _ = apdu.append(&resource_buffer).unwrap();

    // L1 Data Gas
    let mut resource_buffer = [0; 32];
    resource_buffer[1..8].copy_from_slice(b"L1_DATA");
    resource_buffer[8..16].copy_from_slice(
        &u64::from_str_radix(
            resources.l1_data_gas.max_amount.trim_start_matches("0x"),
            16,
        )
        .unwrap()
        .to_be_bytes(),
    );
    resource_buffer[16..].copy_from_slice(
        &u128::from_str_radix(
            resources
                .l1_data_gas
                .max_price_per_unit
                .trim_start_matches("0x"),
            16,
        )
        .unwrap()
        .to_be_bytes(),
    );
    let _ = apdu.append(&resource_buffer).unwrap();

    apdu
}

pub fn paymaster_data(_data: &[String], cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let apdu = Apdu::new(apdu_header);
    apdu
}

pub fn accound_deployment_data(_tx: &[String], cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let apdu = Apdu::new(apdu_header);
    apdu
}

pub fn calls_nb(calls: &[Call], cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    let fe = Felt::from(calls.len());
    let data = fe.to_bytes_be();
    let _ = apdu.append(data.as_slice()).unwrap();
    apdu
}

pub fn call(call: &Call, cla: u8, ins: Ins, p1: u8) -> Vec<Apdu> {
    let mut apdu_list: Vec<Apdu> = Vec::new();
    let data: Vec<Felt> = call.into();

    let nb_apdu = data.chunks(7).len();

    match nb_apdu {
        1 => {
            let apdu_header = ApduHeader {
                cla: cla,
                ins: ins.into(),
                p1,
                p2: 0x00,
            };
            let mut apdu = Apdu::new(apdu_header);

            let data = data.chunks(7).next().unwrap();
            for d in data {
                let _ = apdu.append(&d.to_bytes_be()).unwrap();
            }
            apdu_list.push(apdu);
        }
        2.. => {
            let mut iter = data.chunks(7);

            let mut apdu_header = ApduHeader {
                cla,
                ins: ins.into(),
                p1,
                p2: 0x00,
            };
            let mut apdu = Apdu::new(apdu_header);
            let data = iter.next().unwrap();
            for d in data {
                let _ = apdu.append(&d.to_bytes_be()).unwrap();
            }
            apdu_list.push(apdu);

            loop {
                let next = iter.next();
                match next {
                    Some(felts) => {
                        apdu_header = ApduHeader {
                            cla,
                            ins: ins.into(),
                            p1,
                            p2: 0x01,
                        };
                        apdu = Apdu::new(apdu_header);
                        for d in felts {
                            let _ = apdu.append(&d.to_bytes_be()).unwrap();
                        }
                        apdu_list.push(apdu);
                    }
                    None => break,
                }
            }
        }
        _ => (),
    }
    apdu_list
}

pub fn constructor_calldata(calldata: &[String], cla: u8, ins: Ins, p1: u8) -> Vec<Apdu> {
    let mut apdu_list: Vec<Apdu> = Vec::new();
    let mut p1 = p1;

    // calldata length apdu
    let apdu_header = ApduHeader {
        cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);
    let fe = Felt::from(calldata.len());
    let data: [u8; 32] = fe.to_bytes_be();
    let _ = apdu.append(data.as_slice()).unwrap();
    p1 += 1;
    apdu_list.push(apdu);

    // calldata apdu
    let mut iter = calldata.chunks(7);

    loop {
        let next = iter.next();
        match next {
            Some(s) => {
                let apdu_header = ApduHeader {
                    cla,
                    ins: ins.into(),
                    p1,
                    p2: 0x00,
                };
                let mut apdu = Apdu::new(apdu_header);
                for d in s {
                    let _ = apdu
                        .append(&Felt::from_hex_unchecked(d).to_bytes_be())
                        .unwrap();
                }
                apdu_list.push(apdu);
            }
            None => break,
        }
    }
    apdu_list
}
