// Handwritten operational behavior for the authority-verified ordinary Message Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// owns only behavior the current bootstrap language cannot yet express:
// structural runtime traits, the ordinary Input/Output role seating, and the
// allocated Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext)))]
#[doc(hidden)]
pub enum WireValue {
    Text(std::string::String), Integer(u64), Boolean(bool),
    Sequence(#[rkyv(omit_bounds)] Vec<WireValue>),
    Absent, Present(#[rkyv(omit_bounds)] Box<WireValue>),
    Product(#[rkyv(omit_bounds)] Vec<WireValue>),
    Variant { ordinal: u16, #[rkyv(omit_bounds)] fields: Vec<WireValue> },
}
#[derive(Debug, thiserror::Error)]
#[error("structural wire value does not match the authority-verified Interface")]
#[doc(hidden)]
pub struct WireShapeError;

/// Current-stage structural behavior shared by Interfaces that import these
/// producer-owned types.
#[doc(hidden)]
pub trait WireShape: Sized {
    fn to_wire(&self) -> WireValue;
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError>;
}

impl WireShape for std::string::String {
    fn to_wire(&self) -> WireValue { WireValue::Text(self.clone()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Text(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for u64 {
    fn to_wire(&self) -> WireValue { WireValue::Integer(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Integer(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for bool {
    fn to_wire(&self) -> WireValue { WireValue::Boolean(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Boolean(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl<Value: WireShape> WireShape for Vec<Value> {
    fn to_wire(&self) -> WireValue { WireValue::Sequence(self.iter().map(WireShape::to_wire).collect()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        let WireValue::Sequence(values) = value else { return Err(WireShapeError) };
        values.into_iter().map(Value::from_wire).collect()
    }
}
impl<Value: WireShape> WireShape for Option<Value> {
    fn to_wire(&self) -> WireValue { match self { Some(value) => WireValue::Present(Box::new(value.to_wire())), None => WireValue::Absent } }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        match value { WireValue::Present(value) => Ok(Some(Value::from_wire(*value)?)), WireValue::Absent => Ok(None), _ => Err(WireShapeError) }
    }
}
fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = [$(stringify!($field)),*].len();
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}

wire_external_newtype!(z2Vb4S, Vec<z2VRQt>);
wire_external_newtype!(z2Vf92, std::string::String);
wire_newtype!(z2VUUz, z2VQY5);
wire_enum!(z2Vdpc { unit { 0 => z2VRYp : "Seated", 1 => z2Vb3J : "Reseated" } unary {  } });
wire_struct!(z2VPq6 { field_0: z2Vdkj, field_1: z2VTaw });
wire_external_newtype!(z2Vari, std::string::String);
wire_external_newtype!(z2VRPH, Vec<z2Vc1h>);
wire_struct!(z2Vcjf { field_0: z2VSgb, field_1: z2VYa4 });
wire_newtype!(z2VSVi, z2Vari);
wire_newtype!(z2VQy1, z2VNPW);
wire_enum!(z2VdsV { unit { 0 => z2VXeo : "Send", 1 => z2Vcnf : "Inbox" } unary {  } });
wire_enum!(z2VLsC { unit { 0 => z2VNAm : "QueryThreads", 1 => z2VQEV : "Submit", 2 => z2Vd2H : "QueryThread", 3 => z2Vbd2 : "SubscribeThread", 4 => z2VSMR : "QueryInbox", 5 => z2VQ4p : "QueryAgentRegistry", 6 => z2VT63 : "SubmitStamped", 7 => z2VMAg : "AssignAgentIdentity", 8 => z2VPq9 : "BindAgentEndpoint" } unary {  } });
wire_struct!(z2VRQt { field_0: z2VLZR, field_1: z2VW54, field_2: z2VNcG, field_3: z2VTiK, field_4: z2VY18 });
wire_enum!(z2VVDs { unit { 0 => z2VRQJ : "None" } unary { 1 => z2VaXN(z2Vcjf) : "Related" } });
wire_external_newtype!(z2VdUj, std::string::String);
wire_newtype!(z2VY18, z2Vf2p);
wire_external_newtype!(z2VPn2, u64);
wire_struct!(z2VYf6 { field_0: z2VLsC, field_1: z2Vc6L });
wire_external_newtype!(z2VTaw, std::string::String);
wire_external_newtype!(z2VSgb, std::string::String);
wire_enum!(z2VLC8 { unit { 0 => z2VLjY : "All" } unary {  } });
wire_enum!(z2Vcfd { unit { 1 => z2VRLv : "None" } unary { 0 => z2VNpk(z2VTE1) : "Pinned" } });
wire_external_newtype!(z2Vd8W, std::string::String);
wire_external_newtype!(z2VYrY, u64);
wire_enum!(z2Vbmb { unit { 0 => z2VSMd : "NotDead", 1 => z2VbAt : "Killed" } unary {  } });
wire_external_newtype!(z2VYZK, u64);
wire_enum!(z2VNbH { unit { 1 => z2VZTo : "None" } unary { 0 => z2Vb3C(z2VMBf) : "Bound" } });
wire_struct!(z2VTE1 { field_0: z2VPEW, field_1: z2VYrY });
wire_enum!(z2VUqb { unit {  } unary { 0 => z2Vd9P(z2VPn2) : "UnixUser", 1 => z2VZGs(z2VLkk) : "System" } });
wire_enum!(z2VXMQ { unit { 1 => z2VNZi : "None" } unary { 0 => z2VdeM(z2VdUj) : "Resumed" } });
wire_struct!(z2VbGY { field_0: z2VUSt, field_1: z2Vd8W });
wire_external_newtype!(z2Vcmo, std::string::String);
wire_enum!(z2Ve52 { unit { 0 => z2VQY2 : "UnknownThread", 1 => z2Vdxv : "StoreRejected" } unary {  } });
wire_external_newtype!(z2VWzi, Vec<z2VQpv>);
wire_enum!(z2Vc6L { unit { 1 => z2VXy5 : "NotInPrototypeScope" } unary { 0 => z2VX1C(z2VaAP) : "DependencyMissing", 2 => z2VVTa(z2VUpa) : "ResourceUnavailable" } });
wire_external_newtype!(z2VNcG, std::string::String);
wire_struct!(z2VR9d { field_0: z2VV6N });
wire_external_newtype!(z2VQDX, u64);
wire_struct!(z2VMBf { field_0: z2VUs6, field_1: z2VRqE });
wire_newtype!(z2VZa8, z2Vcmo);
wire_enum!(z2VaAP { unit { 0 => z2VYw7 : "Mind", 1 => z2VeQ8 : "Router", 2 => z2VPX9 : "Harness", 3 => z2VXM9 : "Terminal" } unary {  } });
wire_struct!(z2VevD { field_0: z2VNPW, field_1: z2Vcfd, field_2: z2VXMQ });
wire_struct!(z2VUzX { field_0: z2VS1e });
wire_struct!(z2Vc72 { field_0: z2VNPW, field_1: z2VNbH, field_2: z2VXMQ, field_3: z2Vbmb, field_4: z2Vcfd });
wire_struct!(z2Vc1h { field_0: z2VPq6, field_1: z2VcCL, field_2: z2VYZK });
wire_newtype!(z2VXE7, z2VLZR);
wire_struct!(z2VLtS { field_0: z2VUSt, field_1: z2VVDs, field_2: z2VMa5, field_3: z2VQDX });
wire_newtype!(z2VP29, z2VW5p);
wire_external_newtype!(z2Vf2p, u64);
wire_struct!(z2VdZd { field_0: z2VNPW, field_1: z2Vdpc });
wire_newtype!(z2Vasi, z2VZuS);
wire_external_newtype!(z2VS1e, Vec<z2Vc72>);
wire_struct!(z2VY2v { field_0: z2Vari, field_1: z2VdsV, field_2: z2VNcG, field_3: z2VTiK });
wire_newtype!(z2VcCL, z2VQY5);
wire_enum!(z2VTJ1 { unit {  } unary { 0 => z2VWSr(z2VY3v) : "External", 1 => z2VS4W(z2VPq6) : "InternalComponentInstance", 2 => z2VW74(z2Vdkj) : "Internal" } });
wire_external_newtype!(z2VLZR, u64);
wire_external_newtype!(z2VZuS, std::string::String);
wire_struct!(z2VRGD { field_0: z2Vb4S });
wire_newtype!(z2VPEF, z2Ve52);
wire_enum!(z2VPW5 { unit { 0 => z2VZW7 : "RecipientNotFound", 1 => z2Veun : "StoreRejected" } unary {  } });
wire_enum!(z2VTiK { unit { 1 => z2VR2m : "None" } unary { 0 => z2VPTM(z2VUSt) : "Named" } });
wire_enum!(z2VUpa { unit { 0 => z2Vd7N : "RouterSocket", 1 => z2VZts : "Store", 2 => z2Vem6 : "PeerCredentials", 3 => z2VTL3 : "MessageSocket" } unary {  } });
wire_enum!(z2VUs6 { unit { 0 => z2VTin : "HarnessSocket", 1 => z2VZk6 : "PtySocket" } unary {  } });
wire_external_newtype!(z2VPEW, u64);
wire_external_newtype!(z2VNPW, std::string::String);
wire_enum!(z2VY3v { unit { 0 => z2VRFq : "Owner" } unary { 1 => z2VVrk(z2VLai) : "Network", 2 => z2Vcfq(z2VaiJ) : "OtherPersona", 3 => z2VPYs(z2VLkk) : "System", 4 => z2VN6o(z2VPn2) : "NonOwnerUser" } });
wire_struct!(z2VVAD { field_0: z2VNPW, field_1: z2VMBf, field_2: z2VPEW, field_3: z2VYrY });
wire_struct!(z2VMd2 { field_0: z2VUSt, field_1: z2Vd8W, field_2: z2VVDs });
wire_external_newtype!(z2VMa5, Vec<z2Vd8W>);
wire_enum!(z2VYJe { unit { 0 => z2VPkz : "All" } unary { 1 => z2VbtY(z2VNPW) : "ByAgent" } });
wire_enum!(z2VW5p { unit { 0 => z2VYA6 : "UnknownAgentIdentifier", 1 => z2VYC4 : "StoreRejected" } unary {  } });
wire_struct!(z2VaiJ { field_0: z2Vf92, field_1: z2VZa8 });
wire_newtype!(z2VRqE, z2VQY5);
wire_newtype!(z2VZEr, z2VPW5);
wire_newtype!(z2VPa3, z2VYZK);
wire_newtype!(z2VaVk, z2VYZK);
wire_external_newtype!(z2VYa4, std::string::String);
wire_enum!(z2Vdkj { unit { 0 => z2VZNT : "Introspect", 1 => z2VUbT : "Terminal", 2 => z2VY7m : "System", 3 => z2VeAv : "Mind", 4 => z2VeLi : "Spirit", 5 => z2VYF3 : "Message", 6 => z2VPrB : "Harness", 7 => z2VMBD : "Router", 8 => z2VTqZ : "Orchestrate" } unary {  } });
wire_external_newtype!(z2VQY5, std::string::String);
wire_struct!(z2VQpv { field_0: z2VLZR, field_1: z2VW54, field_2: z2VNcG, field_3: z2VY18 });
wire_external_newtype!(z2VLai, std::string::String);
wire_struct!(z2Ve71 { field_0: z2VY2v, field_1: z2VTJ1, field_2: z2VY18 });
wire_newtype!(z2VRJp, z2VQY5);
wire_struct!(z2VL2C { field_0: z2VUUz, field_1: z2VPa3, field_2: z2VRJp, field_3: z2VaVk, field_4: z2VZv9, field_5: z2VRPH, field_6: z2VUqb });
wire_external_newtype!(z2VW54, std::string::String);
wire_newtype!(z2VZv9, z2VQY5);
wire_external_newtype!(z2VUSt, std::string::String);
wire_external_newtype!(z2VLkk, std::string::String);
wire_external_newtype!(z2VV6N, Vec<z2VLtS>);
wire_newtype!(z2VVrY, z2VUSt);
wire_struct!(z2VYbP { field_0: z2VUSt, field_1: z2VVDs, field_2: z2VMa5, field_3: z2VWzi });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}

archive_root!(z2Vb4S);
archive_root!(z2Vf92);
archive_root!(z2VUUz);
archive_root!(z2Vdpc);
archive_root!(z2VPq6);
archive_root!(z2Vari);
archive_root!(z2VRPH);
archive_root!(z2Vcjf);
archive_root!(z2VSVi);
archive_root!(z2VQy1);
archive_root!(z2VdsV);
archive_root!(z2VLsC);
archive_root!(z2VRQt);
archive_root!(z2VVDs);
archive_root!(z2VdUj);
archive_root!(z2VY18);
archive_root!(z2VPn2);
archive_root!(z2VYf6);
archive_root!(z2VTaw);
archive_root!(z2VSgb);
archive_root!(z2VLC8);
archive_root!(z2Vcfd);
archive_root!(z2Vd8W);
archive_root!(z2VYrY);
archive_root!(z2Vbmb);
archive_root!(z2VYZK);
archive_root!(z2VNbH);
archive_root!(z2VTE1);
archive_root!(z2VUqb);
archive_root!(z2VXMQ);
archive_root!(z2VbGY);
archive_root!(z2Vcmo);
archive_root!(z2Ve52);
archive_root!(z2VWzi);
archive_root!(z2Vc6L);
archive_root!(z2VNcG);
archive_root!(z2VR9d);
archive_root!(z2VQDX);
archive_root!(z2VMBf);
archive_root!(z2VZa8);
archive_root!(z2VaAP);
archive_root!(z2VevD);
archive_root!(z2VUzX);
archive_root!(z2Vc72);
archive_root!(z2Vc1h);
archive_root!(z2VXE7);
archive_root!(z2VLtS);
archive_root!(z2VP29);
archive_root!(z2Vf2p);
archive_root!(z2VdZd);
archive_root!(z2Vasi);
archive_root!(z2VS1e);
archive_root!(z2VY2v);
archive_root!(z2VcCL);
archive_root!(z2VTJ1);
archive_root!(z2VLZR);
archive_root!(z2VZuS);
archive_root!(z2VRGD);
archive_root!(z2VPEF);
archive_root!(z2VPW5);
archive_root!(z2VTiK);
archive_root!(z2VUpa);
archive_root!(z2VUs6);
archive_root!(z2VPEW);
archive_root!(z2VNPW);
archive_root!(z2VY3v);
archive_root!(z2VVAD);
archive_root!(z2VMd2);
archive_root!(z2VMa5);
archive_root!(z2VYJe);
archive_root!(z2VW5p);
archive_root!(z2VaiJ);
archive_root!(z2VRqE);
archive_root!(z2VZEr);
archive_root!(z2VPa3);
archive_root!(z2VaVk);
archive_root!(z2VYa4);
archive_root!(z2Vdkj);
archive_root!(z2VQY5);
archive_root!(z2VQpv);
archive_root!(z2VLai);
archive_root!(z2Ve71);
archive_root!(z2VRJp);
archive_root!(z2VL2C);
archive_root!(z2VW54);
archive_root!(z2VZv9);
archive_root!(z2VUSt);
archive_root!(z2VLkk);
archive_root!(z2VV6N);
archive_root!(z2VVrY);
archive_root!(z2VYbP);

pub enum Input { Submit(z2VY2v), SubmitStamped(z2Ve71), QueryInbox(z2VSVi), AssignAgentIdentity(z2VevD), BindAgentEndpoint(z2VVAD), QueryAgentRegistry(z2VYJe), QueryThread(z2VVrY), SubscribeThread(z2VMd2), QueryThreads(z2VLC8) }
wire_enum!(Input { unit { } unary { 0 => Submit(z2VY2v) : "Submit", 1 => SubmitStamped(z2Ve71) : "SubmitStamped", 2 => QueryInbox(z2VSVi) : "QueryInbox", 3 => AssignAgentIdentity(z2VevD) : "AssignAgentIdentity", 4 => BindAgentEndpoint(z2VVAD) : "BindAgentEndpoint", 5 => QueryAgentRegistry(z2VYJe) : "QueryAgentRegistry", 6 => QueryThread(z2VVrY) : "QueryThread", 7 => SubscribeThread(z2VMd2) : "SubscribeThread", 8 => QueryThreads(z2VLC8) : "QueryThreads" } });
archive_root!(Input);

pub enum Output { SubmissionAccepted(z2VXE7), SubmissionRejected(z2VZEr), InboxListing(z2VRGD), AgentIdentityAssigned(z2VdZd), AgentEndpointBound(z2VQy1), AgentRegistryListing(z2VUzX), AgentRegistryRejected(z2VP29), MessageRequestUnimplemented(z2VYf6), Error(z2Vasi), ThreadListing(z2VYbP), ThreadSubscribed(z2VbGY), ThreadIndexListing(z2VR9d), ThreadRejected(z2VPEF) }
wire_enum!(Output { unit { } unary { 0 => SubmissionAccepted(z2VXE7) : "SubmissionAccepted", 1 => SubmissionRejected(z2VZEr) : "SubmissionRejected", 2 => InboxListing(z2VRGD) : "InboxListing", 3 => AgentIdentityAssigned(z2VdZd) : "AgentIdentityAssigned", 4 => AgentEndpointBound(z2VQy1) : "AgentEndpointBound", 5 => AgentRegistryListing(z2VUzX) : "AgentRegistryListing", 6 => AgentRegistryRejected(z2VP29) : "AgentRegistryRejected", 7 => MessageRequestUnimplemented(z2VYf6) : "MessageRequestUnimplemented", 8 => Error(z2Vasi) : "Error", 9 => ThreadListing(z2VYbP) : "ThreadListing", 10 => ThreadSubscribed(z2VbGY) : "ThreadSubscribed", 11 => ThreadIndexListing(z2VR9d) : "ThreadIndexListing", 12 => ThreadRejected(z2VPEF) : "ThreadRejected" } });
archive_root!(Output);

pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(1) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason { Rejected, Unavailable }

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal { pub reason: EngineRefusalReason, pub detail: std::string::String }

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self { Self { reason: EngineRefusalReason::Rejected, detail } }
    pub fn unavailable(detail: std::string::String) -> Self { Self { reason: EngineRefusalReason::Unavailable, detail } }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")] FrameEncode,
    #[error("failed to decode bound signal frame")] ArchiveDecode,
    #[error("unexpected signal frame body")] UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")] OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute { Submit, SubmitStamped, QueryInbox, AssignAgentIdentity, BindAgentEndpoint, QueryAgentRegistry, QueryThread, SubscribeThread, QueryThreads }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute { SubmissionAccepted, SubmissionRejected, InboxListing, AgentIdentityAssigned, AgentEndpointBound, AgentRegistryListing, AgentRegistryRejected, MessageRequestUnimplemented, Error, ThreadListing, ThreadSubscribed, ThreadIndexListing, ThreadRejected }

impl Input {
    pub fn route(&self) -> InputRoute { match self { Self::Submit(_) => InputRoute::Submit, Self::SubmitStamped(_) => InputRoute::SubmitStamped, Self::QueryInbox(_) => InputRoute::QueryInbox, Self::AssignAgentIdentity(_) => InputRoute::AssignAgentIdentity, Self::BindAgentEndpoint(_) => InputRoute::BindAgentEndpoint, Self::QueryAgentRegistry(_) => InputRoute::QueryAgentRegistry, Self::QueryThread(_) => InputRoute::QueryThread, Self::SubscribeThread(_) => InputRoute::SubscribeThread, Self::QueryThreads(_) => InputRoute::QueryThreads } }
    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(signal_frame::RootCode::new(0), signal_frame::VariantCode::new(self.route() as u8))
    }
    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(route, FrameBody::Request { exchange, request: signal_frame::Request::from_payload(self) })
    }
    pub fn encode_request_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl Output {
    pub fn route(&self) -> OutputRoute { match self { Self::SubmissionAccepted(_) => OutputRoute::SubmissionAccepted, Self::SubmissionRejected(_) => OutputRoute::SubmissionRejected, Self::InboxListing(_) => OutputRoute::InboxListing, Self::AgentIdentityAssigned(_) => OutputRoute::AgentIdentityAssigned, Self::AgentEndpointBound(_) => OutputRoute::AgentEndpointBound, Self::AgentRegistryListing(_) => OutputRoute::AgentRegistryListing, Self::AgentRegistryRejected(_) => OutputRoute::AgentRegistryRejected, Self::MessageRequestUnimplemented(_) => OutputRoute::MessageRequestUnimplemented, Self::Error(_) => OutputRoute::Error, Self::ThreadListing(_) => OutputRoute::ThreadListing, Self::ThreadSubscribed(_) => OutputRoute::ThreadSubscribed, Self::ThreadIndexListing(_) => OutputRoute::ThreadIndexListing, Self::ThreadRejected(_) => OutputRoute::ThreadRejected } }
    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(signal_frame::RootCode::new(1), signal_frame::VariantCode::new(self.route() as u8))
    }
    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)));
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }
    pub fn encode_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for Input {}
impl signal_frame::SignalOperationHeads for Input { const HEADS: &'static [&'static str] = &["Submit", "SubmitStamped", "QueryInbox", "AssignAgentIdentity", "BindAgentEndpoint", "QueryAgentRegistry", "QueryThread", "SubscribeThread", "QueryThreads"]; }
impl signal_frame::LogVariant for Input {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, Input, Output>;
pub type FrameBody = signal_frame::ExchangeFrameBody<Input, Output>;
pub type Request = signal_frame::Request<Input>;
pub type ReplyEnvelope = signal_frame::Reply<Output>;
pub type RequestBuilder = signal_frame::RequestBuilder<Input>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }
    pub fn decode_single_request(bytes: &[u8]) -> Result<(signal_frame::ExchangeIdentifier, Input), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 { return Err(SignalFrameError::OperationCount { found }); }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
