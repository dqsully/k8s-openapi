// Generated from definition io.k8s.api.auditregistration.v1alpha1.AuditSink

/// AuditSink represents a cluster level audit sink
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuditSink {
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the audit configuration spec
    pub spec: Option<crate::v1_13::api::auditregistration::v1alpha1::AuditSinkSpec>,
}

// Begin auditregistration.k8s.io/v1alpha1/AuditSink

// Generated from operation createAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// create an AuditSink
    ///
    /// Use [`CreateAuditSinkResponse`](./enum.CreateAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_audit_sink(
        body: &crate::v1_13::api::auditregistration::v1alpha1::AuditSink,
        optional: CreateAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateAuditSinkOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation deleteAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// delete an AuditSink
    ///
    /// Use [`DeleteAuditSinkResponse`](./enum.DeleteAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the AuditSink
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_audit_sink(
        name: &str,
        optional: DeleteAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteAuditSinkOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(grace_period_seconds) = grace_period_seconds {
        }
        if let Some(orphan_dependents) = orphan_dependents {
        }
        if let Some(pretty) = pretty {
        }
        if let Some(propagation_policy) = propagation_policy {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation deleteAuditregistrationV1alpha1CollectionAuditSink

impl AuditSink {
    /// delete collection of AuditSink
    ///
    /// Use [`DeleteCollectionAuditSinkResponse`](./enum.DeleteCollectionAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_audit_sink(
        optional: DeleteCollectionAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCollectionAuditSinkOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
        }
        if let Some(field_selector) = field_selector {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(label_selector) = label_selector {
        }
        if let Some(limit) = limit {
        }
        if let Some(pretty) = pretty {
        }
        if let Some(resource_version) = resource_version {
        }
        if let Some(timeout_seconds) = timeout_seconds {
        }
        if let Some(watch) = watch {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation listAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// list or watch objects of kind AuditSink
    ///
    /// Use [`ListAuditSinkResponse`](./enum.ListAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_audit_sink(
        optional: ListAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListAuditSinkOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
        }
        if let Some(field_selector) = field_selector {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(label_selector) = label_selector {
        }
        if let Some(limit) = limit {
        }
        if let Some(pretty) = pretty {
        }
        if let Some(resource_version) = resource_version {
        }
        if let Some(timeout_seconds) = timeout_seconds {
        }
        if let Some(watch) = watch {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation patchAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// partially update the specified AuditSink
    ///
    /// Use [`PatchAuditSinkResponse`](./enum.PatchAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the AuditSink

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_audit_sink(
        name: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchAuditSinkOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation readAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// read the specified AuditSink
    ///
    /// Use [`ReadAuditSinkResponse`](./enum.ReadAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the AuditSink
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_audit_sink(
        name: &str,
        optional: ReadAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadAuditSinkOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
        }
        if let Some(export) = export {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation replaceAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// replace the specified AuditSink
    ///
    /// Use [`ReplaceAuditSinkResponse`](./enum.ReplaceAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the AuditSink

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_audit_sink(
        name: &str,
        body: &crate::v1_13::api::auditregistration::v1alpha1::AuditSink,
        optional: ReplaceAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceAuditSinkOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/auditsinks/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation watchAuditregistrationV1alpha1AuditSink

impl AuditSink {
    /// watch changes to an object of kind AuditSink. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use [`WatchAuditSinkResponse`](./enum.WatchAuditSinkResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the AuditSink
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_audit_sink(
        name: &str,
        optional: WatchAuditSinkOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchAuditSinkOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/watch/auditsinks/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
        }
        if let Some(field_selector) = field_selector {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(label_selector) = label_selector {
        }
        if let Some(limit) = limit {
        }
        if let Some(pretty) = pretty {
        }
        if let Some(resource_version) = resource_version {
        }
        if let Some(timeout_seconds) = timeout_seconds {
        }
        if let Some(watch) = watch {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation watchAuditregistrationV1alpha1AuditSinkList

impl AuditSink {
    /// watch individual changes to a list of AuditSink. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchAuditSinkListResponse`](./enum.WatchAuditSinkListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_audit_sink_list(
        optional: WatchAuditSinkListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchAuditSinkListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/auditregistration.k8s.io/v1alpha1/watch/auditsinks?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
        }
        if let Some(field_selector) = field_selector {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(label_selector) = label_selector {
        }
        if let Some(limit) = limit {
        }
        if let Some(pretty) = pretty {
        }
        if let Some(resource_version) = resource_version {
        }
        if let Some(timeout_seconds) = timeout_seconds {
        }
        if let Some(watch) = watch {
        }
        let __url = __query_pairs.finish();
}
// End auditregistration.k8s.io/v1alpha1/AuditSink

impl crate::Resource for AuditSink {
    fn api_version() -> &'static str {
        "auditregistration.k8s.io/v1alpha1"
    }

    fn group() -> &'static str {
        "auditregistration.k8s.io"
    }

    fn kind() -> &'static str {
        "AuditSink"
    }

    fn version() -> &'static str {
        "v1alpha1"
    }
}

impl crate::Metadata for AuditSink {
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for AuditSink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditSink;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct AuditSink")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_13::api::auditregistration::v1alpha1::AuditSinkSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AuditSink {
                    metadata: value_metadata,
                    spec: value_spec,
                })
            }
        }

        deserializer.deserialize_struct(
            "AuditSink",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for AuditSink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AuditSink",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
