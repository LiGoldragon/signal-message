#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type Ordinary = String;
#[rustfmt::skip]
pub type Meta = String;
#[rustfmt::skip]
pub type Flow = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Configuration {
    pub ordinary: Ordinary,
    pub meta: Meta,
    pub flow: Flow,
}
