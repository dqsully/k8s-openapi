// Generated from definition io.k8s.api.core.v1.WindowsSecurityContextOptions

/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowsSecurityContextOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field. This field is alpha-level and is only honored by servers that enable the WindowsGMSA feature flag.
    pub gmsa_credential_spec: Option<String>,

    /// GMSACredentialSpecName is the name of the GMSA credential spec to use. This field is alpha-level and is only honored by servers that enable the WindowsGMSA feature flag.
    pub gmsa_credential_spec_name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for WindowsSecurityContextOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_gmsa_credential_spec,
            Key_gmsa_credential_spec_name,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "gmsaCredentialSpec" => Field::Key_gmsa_credential_spec,
                            "gmsaCredentialSpecName" => Field::Key_gmsa_credential_spec_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WindowsSecurityContextOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct WindowsSecurityContextOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_gmsa_credential_spec: Option<String> = None;
                let mut value_gmsa_credential_spec_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_gmsa_credential_spec => value_gmsa_credential_spec = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gmsa_credential_spec_name => value_gmsa_credential_spec_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WindowsSecurityContextOptions {
                    gmsa_credential_spec: value_gmsa_credential_spec,
                    gmsa_credential_spec_name: value_gmsa_credential_spec_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "WindowsSecurityContextOptions",
            &[
                "gmsaCredentialSpec",
                "gmsaCredentialSpecName",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for WindowsSecurityContextOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WindowsSecurityContextOptions",
            self.gmsa_credential_spec.as_ref().map_or(0, |_| 1) +
            self.gmsa_credential_spec_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.gmsa_credential_spec {
            serde::ser::SerializeStruct::serialize_field(&mut state, "gmsaCredentialSpec", value)?;
        }
        if let Some(value) = &self.gmsa_credential_spec_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "gmsaCredentialSpecName", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
