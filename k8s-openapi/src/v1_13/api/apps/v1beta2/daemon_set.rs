// Generated from definition io.k8s.api.apps.v1beta2.DaemonSet

/// DEPRECATED - This group version of DaemonSet is deprecated by apps/v1/DaemonSet. See the release notes for more information. DaemonSet represents the configuration of a daemon set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSet {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The desired behavior of this daemon set. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_13::api::apps::v1beta2::DaemonSetSpec>,

    /// The current status of this daemon set. This data may be out of date by some window of time. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_13::api::apps::v1beta2::DaemonSetStatus>,
}

// Begin apps/v1beta2/DaemonSet

// Generated from operation createAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// create a DaemonSet
    ///
    /// Use [`CreateNamespacedDaemonSetResponse`](./enum.CreateNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_namespaced_daemon_set(
        namespace: &str,
        body: &crate::v1_13::api::apps::v1beta2::DaemonSet,
        optional: CreateNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateNamespacedDaemonSetOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation deleteAppsV1beta2CollectionNamespacedDaemonSet

impl DaemonSet {
    /// delete collection of DaemonSet
    ///
    /// Use [`DeleteCollectionNamespacedDaemonSetResponse`](./enum.DeleteCollectionNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_namespaced_daemon_set(
        namespace: &str,
        optional: DeleteCollectionNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCollectionNamespacedDaemonSetOptional {
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
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
// Generated from operation deleteAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// delete a DaemonSet
    ///
    /// Use [`DeleteNamespacedDaemonSetResponse`](./enum.DeleteNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_daemon_set(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteNamespacedDaemonSetOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
// Generated from operation listAppsV1beta2DaemonSetForAllNamespaces

impl DaemonSet {
    /// list or watch objects of kind DaemonSet
    ///
    /// Use [`ListDaemonSetForAllNamespacesResponse`](./enum.ListDaemonSetForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_daemon_set_for_all_namespaces(
        optional: ListDaemonSetForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListDaemonSetForAllNamespacesOptional {
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
        let __url = format!("/apis/apps/v1beta2/daemonsets?");
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
// Generated from operation listAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// list or watch objects of kind DaemonSet
    ///
    /// Use [`ListNamespacedDaemonSetResponse`](./enum.ListNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_namespaced_daemon_set(
        namespace: &str,
        optional: ListNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListNamespacedDaemonSetOptional {
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
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
// Generated from operation patchAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// partially update the specified DaemonSet
    ///
    /// Use [`PatchNamespacedDaemonSetResponse`](./enum.PatchNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_daemon_set(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedDaemonSetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation patchAppsV1beta2NamespacedDaemonSetStatus

impl DaemonSet {
    /// partially update status of the specified DaemonSet
    ///
    /// Use [`PatchNamespacedDaemonSetStatusResponse`](./enum.PatchNamespacedDaemonSetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_daemon_set_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedDaemonSetStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedDaemonSetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation readAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// read the specified DaemonSet
    ///
    /// Use [`ReadNamespacedDaemonSetResponse`](./enum.ReadNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_daemon_set(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedDaemonSetOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
        }
        if let Some(export) = export {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation readAppsV1beta2NamespacedDaemonSetStatus

impl DaemonSet {
    /// read status of the specified DaemonSet
    ///
    /// Use [`ReadNamespacedDaemonSetStatusResponse`](./enum.ReadNamespacedDaemonSetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_daemon_set_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedDaemonSetStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedDaemonSetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation replaceAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// replace the specified DaemonSet
    ///
    /// Use [`ReplaceNamespacedDaemonSetResponse`](./enum.ReplaceNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_daemon_set(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::apps::v1beta2::DaemonSet,
        optional: ReplaceNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedDaemonSetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation replaceAppsV1beta2NamespacedDaemonSetStatus

impl DaemonSet {
    /// replace status of the specified DaemonSet
    ///
    /// Use [`ReplaceNamespacedDaemonSetStatusResponse`](./enum.ReplaceNamespacedDaemonSetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_daemon_set_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::apps::v1beta2::DaemonSet,
        optional: ReplaceNamespacedDaemonSetStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedDaemonSetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta2/namespaces/{namespace}/daemonsets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation watchAppsV1beta2DaemonSetListForAllNamespaces

impl DaemonSet {
    /// watch individual changes to a list of DaemonSet. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchDaemonSetListForAllNamespacesResponse`](./enum.WatchDaemonSetListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_daemon_set_list_for_all_namespaces(
        optional: WatchDaemonSetListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchDaemonSetListForAllNamespacesOptional {
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
        let __url = format!("/apis/apps/v1beta2/watch/daemonsets?");
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
// Generated from operation watchAppsV1beta2NamespacedDaemonSet

impl DaemonSet {
    /// watch changes to an object of kind DaemonSet. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use [`WatchNamespacedDaemonSetResponse`](./enum.WatchNamespacedDaemonSetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the DaemonSet

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_daemon_set(
        name: &str,
        namespace: &str,
        optional: WatchNamespacedDaemonSetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedDaemonSetOptional {
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
        let __url = format!("/apis/apps/v1beta2/watch/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
// Generated from operation watchAppsV1beta2NamespacedDaemonSetList

impl DaemonSet {
    /// watch individual changes to a list of DaemonSet. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchNamespacedDaemonSetListResponse`](./enum.WatchNamespacedDaemonSetListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_daemon_set_list(
        namespace: &str,
        optional: WatchNamespacedDaemonSetListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedDaemonSetListOptional {
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
        let __url = format!("/apis/apps/v1beta2/watch/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
// End apps/v1beta2/DaemonSet

impl crate::Resource for DaemonSet {
    fn api_version() -> &'static str {
        "apps/v1beta2"
    }

    fn group() -> &'static str {
        "apps"
    }

    fn kind() -> &'static str {
        "DaemonSet"
    }

    fn version() -> &'static str {
        "v1beta2"
    }
}

impl crate::Metadata for DaemonSet {
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for DaemonSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
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
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSet;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct DaemonSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_13::api::apps::v1beta2::DaemonSetSpec> = None;
                let mut value_status: Option<crate::v1_13::api::apps::v1beta2::DaemonSetStatus> = None;

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
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSet {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSet",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DaemonSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSet",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
