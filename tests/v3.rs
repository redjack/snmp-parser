extern crate der_parser;
extern crate snmp_parser;
extern crate nom;

use nom::IResult;
use snmp_parser::*;
use der_parser::DerObject;

static SNMPV3_REQ: &'static [u8] = include_bytes!("../assets/snmpv3_req.bin");

#[test]
fn test_snmp_v3_req() {
    let empty = &b""[..];
    let bytes = SNMPV3_REQ;
    let sp = [48, 14, 4, 0, 2, 1, 0, 2, 1, 0, 4, 0, 4, 0, 4, 0];
    let cei = [0x80, 0x00, 0x1f, 0x88, 0x80, 0x59, 0xdc, 0x48, 0x61, 0x45, 0xa2, 0x63, 0x22];
    let data = SnmpPdu::Generic(SnmpGenericPdu{
        pdu_type: PduType::GetRequest,
        req_id: 2098071598,
        err: ErrorStatus::NoError,
        err_index: 0,
        var: DerObject::from_seq(vec![])
    });
    let expected = IResult::Done(empty,SnmpV3Message{
        version: 3,
        header_data: HeaderData{
            msg_id: 821490644,
            msg_max_size: 65507,
            msg_flags: 4,
            msg_security_model: 3,
        },
        security_params: &sp,
        data: ScopedPduData::Plaintext(
            ScopedPdu{
                ctx_engine_id: &cei,
                ctx_engine_name: b"",
                data: data,
            }
        ),
    });
    let res = parse_snmp_v3(&bytes);
    // eprintln!("{:?}", res);
    assert_eq!(res, expected);
}


#[test]
fn test_snmp_v3_req_encrypted() {
    let bytes = include_bytes!("../assets/snmpv3_req_encrypted.bin");
    let res = parse_snmp_v3(bytes);
    // eprintln!("{:?}", res);
    match res {
        IResult::Done(rem,msg) => {
            assert!(rem.is_empty());
            assert_eq!(msg.version, 3);
            assert_eq!(msg.header_data.msg_security_model, 3);
        },
        _ => assert!(false),
    }
}
