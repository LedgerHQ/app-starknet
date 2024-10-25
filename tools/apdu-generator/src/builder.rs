use crate::apdu::{Apdu, ApduHeader};
use crate::types::{Call, DeployAccountV1, DeployAccountV3, FieldElement, Ins, InvokeV1, InvokeV3};
use ethereum_types::U256;

pub enum ApduError {
    InternalError,
}

pub fn data_to_apdu(data: Vec<FieldElement>, cla: u8, ins: u8, p1: u8, p2: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2,
    };
    let mut apdu = Apdu::new(apdu_header);

    for felt in data {
        let arr: [u8; 32] = felt.try_into().unwrap();
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

    let fixed_hash: String = String::from(hash.trim_start_matches("0x"));
    let data: [u8; 32] = FieldElement(U256::from_str_radix(fixed_hash.as_str(), 16).unwrap())
        .try_into()
        .unwrap();
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

    let mut fe: FieldElement = FieldElement(U256::from_str_radix(&tx.sender_address, 16).unwrap());
    let mut data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.max_fee, 10).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.chain_id, 16).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.nonce, 10).unwrap());
    data = fe.try_into().unwrap();
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

    let mut fe: FieldElement = FieldElement(U256::from_str_radix(&tx.sender_address, 16).unwrap());
    let mut data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.tip, 10).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.l1_gas_bounds, 16).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.l2_gas_bounds, 16).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.chain_id, 16).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.nonce, 10).unwrap());
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement(U256::from_str_radix(&tx.data_availability_mode, 10).unwrap());
    data = fe.try_into().unwrap();
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

    let mut fe: FieldElement = FieldElement::try_from(tx.contract_address.as_str()).unwrap();
    let mut data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.chain_id.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.nonce.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.data_availability_mode.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.class_hash.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.contract_address_salt.as_str()).unwrap();
    data = fe.try_into().unwrap();
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

    let mut fe: FieldElement = FieldElement::try_from(tx.contract_address.as_str()).unwrap();
    let mut data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.class_hash.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.contract_address_salt.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.chain_id.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    fe = FieldElement::try_from(tx.nonce.as_str()).unwrap();
    data = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();

    apdu
}

pub fn tx_fees(fees: &[FieldElement], cla: u8, ins: Ins, p1: u8) -> Apdu {
    let apdu_header = ApduHeader {
        cla: cla,
        ins: ins.into(),
        p1,
        p2: 0x00,
    };
    let mut apdu = Apdu::new(apdu_header);

    for f in fees {
        let data: [u8; 32] = (*f).try_into().unwrap();
        apdu.append(data.as_slice()).unwrap();
    }
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

    let fe = FieldElement(U256::from(calls.len()));
    let data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();
    apdu
}

pub fn call(call: &Call, cla: u8, ins: Ins, p1: u8) -> Vec<Apdu> {
    let mut apdu_list: Vec<Apdu> = Vec::new();
    let mut fe: [u8; 32] = [0u8; 32];
    let data: Vec<FieldElement> = call.into();

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
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
            }
            apdu_list.push(apdu);

            apdu = Apdu::new(ApduHeader {
                p2: 0x02,
                ..apdu_header
            });
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
                d.0.to_big_endian(&mut fe);
                apdu.append(&fe).unwrap();
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
                            d.0.to_big_endian(&mut fe);
                            apdu.append(&fe).unwrap();
                        }
                        apdu_list.push(apdu);
                    }
                    None => {
                        apdu = Apdu::new(ApduHeader {
                            p2: 0x02,
                            ..apdu_header
                        });
                        apdu_list.push(apdu);
                        break;
                    }
                }
            }
        }
        _ => (),
    }
    apdu_list
}

pub fn constructor_calldata(calldata: &[FieldElement], cla: u8, ins: Ins, p1: u8) -> Vec<Apdu> {
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
    let fe = FieldElement(U256::from(calldata.len()));
    let data: [u8; 32] = fe.try_into().unwrap();
    apdu.append(data.as_slice()).unwrap();
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
                    let data: [u8; 32] = (*d).try_into().unwrap();
                    apdu.append(data.as_slice()).unwrap();
                }
                apdu_list.push(apdu);
            }
            None => break,
        }
    }
    apdu_list
}
