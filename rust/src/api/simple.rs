pub use serde::{Deserialize, Serialize};
pub use time::OffsetDateTime;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParentStruct {
    pub id: String,
    pub name: String,
    pub child: Option<ChildStruct>,
}

impl ParentStruct {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_variant1() -> Self {
        Self {
            id: "1".to_string(),
            name: "parent".to_string(),
            child: Some(ChildStruct {
                id: "1.1".to_string(),
                name: "child".to_string(),
                parent_id: "1".to_string(),
                variant: EnumStruct::Variant1(VariantNonOpaqueStruct {
                    id: "1.1.1".to_string(),
                    name: "variant".to_string(),
                    timestamp: OffsetDateTime::now_utc(),
                }),
            }),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn new_variant2() -> Self {
        Self {
            id: "2".to_string(),
            name: "parent".to_string(),
            child: Some(ChildStruct {
                id: "2.1".to_string(),
                name: "child".to_string(),
                parent_id: "2".to_string(),
                variant: EnumStruct::Variant2(VariantOpaqueStruct {
                    id: "2.1.1".to_string(),
                    name: "variant".to_string(),
                    timestamp: OffsetDateTime::now_utc(),
                }),
            }),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn new_variant3() -> Self {
        Self {
            id: "3".to_string(),
            name: "parent".to_string(),
            child: Some(ChildStruct {
                id: "3.1".to_string(),
                name: "child".to_string(),
                parent_id: "3".to_string(),
                variant: EnumStruct::Variant3,
            }),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn new_variant4() -> Self {
        Self {
            id: "4".to_string(),
            name: "parent".to_string(),
            child: Some(ChildStruct {
                id: "4.1".to_string(),
                name: "child".to_string(),
                parent_id: "4".to_string(),
                variant: EnumStruct::Variant4(VariantWithoutTimestamp {
                    id: "4.1.1".to_string(),
                    name: "variant".to_string(),
                }),
            }),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct ChildStruct {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub variant: EnumStruct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub enum EnumStruct {
    Variant1(VariantNonOpaqueStruct),
    Variant2(VariantOpaqueStruct),

    Variant3,
    Variant4(VariantWithoutTimestamp),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct VariantNonOpaqueStruct {
    pub id: String,
    pub name: String,
    #[serde(with = "time::serde::timestamp")]
    pub timestamp: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[flutter_rust_bridge::frb(opaque)]
pub struct VariantOpaqueStruct {
    pub id: String,
    pub name: String,
    #[serde(with = "time::serde::timestamp")]
    pub timestamp: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct VariantWithoutTimestamp {
    pub id: String,
    pub name: String,
}