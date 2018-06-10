// Generated from definition io.k8s.api.core.v1.ReplicationControllerStatus

/// ReplicationControllerStatus represents the current status of a replication controller.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicationControllerStatus {
    /// The number of available replicas (ready for at least minReadySeconds) for this replication controller.
    pub available_replicas: Option<i32>,

    /// Represents the latest available observations of a replication controller's current state.
    pub conditions: Option<Vec<::v1_10::api::core::v1::ReplicationControllerCondition>>,

    /// The number of pods that have labels matching the labels of the pod template of the replication controller.
    pub fully_labeled_replicas: Option<i32>,

    /// ObservedGeneration reflects the generation of the most recently observed replication controller.
    pub observed_generation: Option<i64>,

    /// The number of ready replicas for this replication controller.
    pub ready_replicas: Option<i32>,

    /// Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    pub replicas: i32,
}

impl<'de> ::serde::Deserialize<'de> for ReplicationControllerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_conditions,
            Key_fully_labeled_replicas,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
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
                            "availableReplicas" => Field::Key_available_replicas,
                            "conditions" => Field::Key_conditions,
                            "fullyLabeledReplicas" => Field::Key_fully_labeled_replicas,
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationControllerStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ReplicationControllerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_conditions: Option<Vec<::v1_10::api::core::v1::ReplicationControllerCondition>> = None;
                let mut value_fully_labeled_replicas: Option<i32> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fully_labeled_replicas => value_fully_labeled_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ReplicationControllerStatus {
                    available_replicas: value_available_replicas,
                    conditions: value_conditions,
                    fully_labeled_replicas: value_fully_labeled_replicas,
                    observed_generation: value_observed_generation,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas.ok_or_else(|| ::serde::de::Error::missing_field("replicas"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ReplicationControllerStatus",
            &[
                "availableReplicas",
                "conditions",
                "fullyLabeledReplicas",
                "observedGeneration",
                "readyReplicas",
                "replicas",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ReplicationControllerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicationControllerStatus",
            0 +
            (if self.available_replicas.is_some() { 1 } else { 0 }) +
            (if self.conditions.is_some() { 1 } else { 0 }) +
            (if self.fully_labeled_replicas.is_some() { 1 } else { 0 }) +
            (if self.observed_generation.is_some() { 1 } else { 0 }) +
            (if self.ready_replicas.is_some() { 1 } else { 0 }) +
            1,
        )?;
        if let Some(value) = &self.available_replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", value)?;
        }
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.fully_labeled_replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fullyLabeledReplicas", value)?;
        }
        if let Some(value) = &self.observed_generation {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
