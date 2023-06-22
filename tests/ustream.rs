use lang_inner::compress::{
    UStream,
    ser::{deserialize_bin, serialize_bin},
    Compress,
};
use lang_inner::{LanguageRaw, Table};

#[test]
fn lang_compress_test() -> std::io::Result<()> {
    let module_inner = std::fs::read_to_string("languages/en.fckl")?;
    let lang: LanguageRaw = LanguageRaw::from_text(&*module_inner).expect("Unable to parse en");
    let (transition, tt, td) = lang_inner::tables::tabularize(&lang);
    let transition_ustream = UStream::compress(&transition);
    for i in 0..transition.len() {
        for n in 0..256 {
            assert_eq!(
                transition[i][n], transition_ustream.element(i as u16, n as u8),
                "{}:{}", i, n
            )
        }
    }
    let tt_ustream = UStream::compress(&tt);
    for i in 0..tt.len() {
        for n in 0..256 {
            assert_eq!(
                tt[i][n], tt_ustream.element(i as u16, n as u8),
                "{}:{}", i, n
            )
        }
    }
    let td_ustream = UStream::compress(&td);
    for i in 0..td.len() {
        for n in 0..256 {
            assert_eq!(
                td[i][n], td_ustream.element(i as u16, n as u8),
                "{}:{}", i, n
            )
        }
    }
    Ok(())
}
#[test]
fn ser_de() -> Result<(), String> {
        let module_inner =
            std::fs::read_to_string("languages/en.fckl").map_err(|e| e.to_string())?;
        let lang: LanguageRaw = LanguageRaw::from_text(&*module_inner).expect("Unable to parse en");
        let (transition, tt, td) = lang_inner::tables::tabularize(&lang);
        let transition_ustream = UStream::compress(&transition);
        let tt_ustream = UStream::compress(&tt);
        let td_ustream = UStream::compress(&td);
        let deser = deserialize_bin(((&transition_ustream, &tt_ustream, &td_ustream)));
        let mut deser = deser.iter().cloned();
        let (ser_de_transition_ustream, ser_de_tt_ustream, ser_de_td_ustream) =
            serialize_bin(&mut deser).ok_or("Deserialization error".to_string())?;
        assert!(deser.next().is_none());
        for i in 0..transition.len() as u16 {
            for n in 0..=255 {
                assert_eq!(
                    ser_de_transition_ustream.element(i, n),
                    transition_ustream.element(i, n)
                )
            }
        }
        for i in 0..tt.len() as u16 {
            for n in 0..=255 {
                assert_eq!(ser_de_tt_ustream.element(i, n), tt_ustream.element(i, n))
            }
        }
        for i in 0..td.len() as u16 {
            for n in 0..=255 {
                assert_eq!(ser_de_td_ustream.element(i, n), td_ustream.element(i, n))
            }
        }
        Ok(())
    }
