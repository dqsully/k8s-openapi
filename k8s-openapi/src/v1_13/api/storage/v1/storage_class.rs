// Generated from definition io.k8s.api.storage.v1.StorageClass

/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageClass {
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    pub allow_volume_expansion: Option<bool>,

    /// Restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
    pub allowed_topologies: Option<Vec<crate::v1_13::api::core::v1::TopologySelectorTerm>>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. \["ro", "soft"\]. Not validated - mount of the PVs will simply fail if one is invalid.
    pub mount_options: Option<Vec<String>>,

    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    pub parameters: Option<std::collections::BTreeMap<String, String>>,

    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.
    pub reclaim_policy: Option<String>,

    /// VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
    pub volume_binding_mode: Option<String>,
}

// Begin storage.k8s.io/v1/StorageClass

// Generated from operation createStorageV1StorageClass

impl StorageClass {
    /// create a StorageClass
    ///
    /// Use [`CreateStorageClassResponse`](./enum.CreateStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_storage_class(
        body: &crate::v1_13::api::storage::v1::StorageClass,
        optional: CreateStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateStorageClassOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation deleteStorageV1CollectionStorageClass

impl StorageClass {
    /// delete collection of StorageClass
    ///
    /// Use [`DeleteCollectionStorageClassResponse`](./enum.DeleteCollectionStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_storage_class(
        optional: DeleteCollectionStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCollectionStorageClassOptional {
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
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses?");
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
// Generated from operation deleteStorageV1StorageClass

impl StorageClass {
    /// delete a StorageClass
    ///
    /// Use [`DeleteStorageClassResponse`](./enum.DeleteStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_storage_class(
        name: &str,
        optional: DeleteStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteStorageClassOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
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
// Generated from operation listStorageV1StorageClass

impl StorageClass {
    /// list or watch objects of kind StorageClass
    ///
    /// Use [`ListStorageClassResponse`](./enum.ListStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_storage_class(
        optional: ListStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListStorageClassOptional {
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
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses?");
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
// Generated from operation patchStorageV1StorageClass

impl StorageClass {
    /// partially update the specified StorageClass
    ///
    /// Use [`PatchStorageClassResponse`](./enum.PatchStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the StorageClass

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_storage_class(
        name: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchStorageClassOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation readStorageV1StorageClass

impl StorageClass {
    /// read the specified StorageClass
    ///
    /// Use [`ReadStorageClassResponse`](./enum.ReadStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_storage_class(
        name: &str,
        optional: ReadStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadStorageClassOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
        }
        if let Some(export) = export {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation replaceStorageV1StorageClass

impl StorageClass {
    /// replace the specified StorageClass
    ///
    /// Use [`ReplaceStorageClassResponse`](./enum.ReplaceStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the StorageClass

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_storage_class(
        name: &str,
        body: &crate::v1_13::api::storage::v1::StorageClass,
        optional: ReplaceStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceStorageClassOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation watchStorageV1StorageClass

impl StorageClass {
    /// watch changes to an object of kind StorageClass. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use [`WatchStorageClassResponse`](./enum.WatchStorageClassResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_storage_class(
        name: &str,
        optional: WatchStorageClassOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchStorageClassOptional {
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
        let __url = format!("/apis/storage.k8s.io/v1/watch/storageclasses/{name}?", name = name);
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
// Generated from operation watchStorageV1StorageClassList

impl StorageClass {
    /// watch individual changes to a list of StorageClass. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchStorageClassListResponse`](./enum.WatchStorageClassListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_storage_class_list(
        optional: WatchStorageClassListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchStorageClassListOptional {
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
        let __url = format!("/apis/storage.k8s.io/v1/watch/storageclasses?");
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
// End storage.k8s.io/v1/StorageClass

impl crate::Resource for StorageClass {
    fn api_version() -> &'static str {
        "storage.k8s.io/v1"
    }

    fn group() -> &'static str {
        "storage.k8s.io"
    }

    fn kind() -> &'static str {
        "StorageClass"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for StorageClass {
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for StorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_allow_volume_expansion,
            Key_allowed_topologies,
            Key_metadata,
            Key_mount_options,
            Key_parameters,
            Key_provisioner,
            Key_reclaim_policy,
            Key_volume_binding_mode,
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
                            "allowVolumeExpansion" => Field::Key_allow_volume_expansion,
                            "allowedTopologies" => Field::Key_allowed_topologies,
                            "metadata" => Field::Key_metadata,
                            "mountOptions" => Field::Key_mount_options,
                            "parameters" => Field::Key_parameters,
                            "provisioner" => Field::Key_provisioner,
                            "reclaimPolicy" => Field::Key_reclaim_policy,
                            "volumeBindingMode" => Field::Key_volume_binding_mode,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageClass;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct StorageClass")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allow_volume_expansion: Option<bool> = None;
                let mut value_allowed_topologies: Option<Vec<crate::v1_13::api::core::v1::TopologySelectorTerm>> = None;
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_parameters: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_provisioner: Option<String> = None;
                let mut value_reclaim_policy: Option<String> = None;
                let mut value_volume_binding_mode: Option<String> = None;

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
                        Field::Key_allow_volume_expansion => value_allow_volume_expansion = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_topologies => value_allowed_topologies = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provisioner => value_provisioner = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reclaim_policy => value_reclaim_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_binding_mode => value_volume_binding_mode = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageClass {
                    allow_volume_expansion: value_allow_volume_expansion,
                    allowed_topologies: value_allowed_topologies,
                    metadata: value_metadata,
                    mount_options: value_mount_options,
                    parameters: value_parameters,
                    provisioner: value_provisioner.ok_or_else(|| serde::de::Error::missing_field("provisioner"))?,
                    reclaim_policy: value_reclaim_policy,
                    volume_binding_mode: value_volume_binding_mode,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageClass",
            &[
                "apiVersion",
                "kind",
                "allowVolumeExpansion",
                "allowedTopologies",
                "metadata",
                "mountOptions",
                "parameters",
                "provisioner",
                "reclaimPolicy",
                "volumeBindingMode",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for StorageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageClass",
            0 +
            2 +
            self.allow_volume_expansion.as_ref().map_or(0, |_| 1) +
            self.allowed_topologies.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.mount_options.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1) +
            1 +
            self.reclaim_policy.as_ref().map_or(0, |_| 1) +
            self.volume_binding_mode.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.allow_volume_expansion {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowVolumeExpansion", value)?;
        }
        if let Some(value) = &self.allowed_topologies {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedTopologies", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.mount_options {
            serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
        }
        if let Some(value) = &self.parameters {
            serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "provisioner", &self.provisioner)?;
        if let Some(value) = &self.reclaim_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reclaimPolicy", value)?;
        }
        if let Some(value) = &self.volume_binding_mode {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeBindingMode", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
