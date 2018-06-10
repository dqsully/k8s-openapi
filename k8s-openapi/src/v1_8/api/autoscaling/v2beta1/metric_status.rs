// Generated from definition io.k8s.api.autoscaling.v2beta1.MetricStatus

/// MetricStatus describes the last-read state of a single metric.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetricStatus {
    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    pub object: Option<::v1_8::api::autoscaling::v2beta1::ObjectMetricStatus>,

    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    pub pods: Option<::v1_8::api::autoscaling::v2beta1::PodsMetricStatus>,

    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    pub resource: Option<::v1_8::api::autoscaling::v2beta1::ResourceMetricStatus>,

    /// type is the type of metric source.  It will match one of the fields below.
    pub type_: String,
}

impl<'de> ::serde::Deserialize<'de> for MetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_object,
            Key_pods,
            Key_resource,
            Key_type_,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "object" => Field::Key_object,
                            "pods" => Field::Key_pods,
                            "resource" => Field::Key_resource,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MetricStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct MetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_object: Option<::v1_8::api::autoscaling::v2beta1::ObjectMetricStatus> = None;
                let mut value_pods: Option<::v1_8::api::autoscaling::v2beta1::PodsMetricStatus> = None;
                let mut value_resource: Option<::v1_8::api::autoscaling::v2beta1::ResourceMetricStatus> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_object => value_object = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pods => value_pods = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MetricStatus {
                    object: value_object,
                    pods: value_pods,
                    resource: value_resource,
                    type_: value_type_.ok_or_else(|| ::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "MetricStatus",
            &[
                "object",
                "pods",
                "resource",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for MetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MetricStatus",
            0 +
            (if self.object.is_some() { 1 } else { 0 }) +
            (if self.pods.is_some() { 1 } else { 0 }) +
            (if self.resource.is_some() { 1 } else { 0 }) +
            1,
        )?;
        if let Some(value) = &self.object {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "object", value)?;
        }
        if let Some(value) = &self.pods {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "pods", value)?;
        }
        if let Some(value) = &self.resource {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
