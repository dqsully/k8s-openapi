// Generated from definition io.k8s.api.policy.v1beta1.PodDisruptionBudget

/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDisruptionBudget {
    pub metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the PodDisruptionBudget.
    pub spec: Option<crate::v1_13::api::policy::v1beta1::PodDisruptionBudgetSpec>,

    /// Most recently observed status of the PodDisruptionBudget.
    pub status: Option<crate::v1_13::api::policy::v1beta1::PodDisruptionBudgetStatus>,
}

// Begin policy/v1beta1/PodDisruptionBudget

// Generated from operation createPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// create a PodDisruptionBudget
    ///
    /// Use [`CreateNamespacedPodDisruptionBudgetResponse`](./enum.CreateNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
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
    pub fn create_namespaced_pod_disruption_budget(
        namespace: &str,
        body: &crate::v1_13::api::policy::v1beta1::PodDisruptionBudget,
        optional: CreateNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateNamespacedPodDisruptionBudgetOptional {
            dry_run,
            include_uninitialized,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(include_uninitialized) = include_uninitialized {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation deletePolicyV1beta1CollectionNamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// delete collection of PodDisruptionBudget
    ///
    /// Use [`DeleteCollectionNamespacedPodDisruptionBudgetResponse`](./enum.DeleteCollectionNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_namespaced_pod_disruption_budget(
        namespace: &str,
        optional: DeleteCollectionNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteCollectionNamespacedPodDisruptionBudgetOptional {
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
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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
// Generated from operation deletePolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// delete a PodDisruptionBudget
    ///
    /// Use [`DeleteNamespacedPodDisruptionBudgetResponse`](./enum.DeleteNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        optional: DeleteNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteNamespacedPodDisruptionBudgetOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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
// Generated from operation listPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// Use [`ListNamespacedPodDisruptionBudgetResponse`](./enum.ListNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_namespaced_pod_disruption_budget(
        namespace: &str,
        optional: ListNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListNamespacedPodDisruptionBudgetOptional {
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
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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
// Generated from operation listPolicyV1beta1PodDisruptionBudgetForAllNamespaces

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    ///
    /// Use [`ListPodDisruptionBudgetForAllNamespacesResponse`](./enum.ListPodDisruptionBudgetForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_pod_disruption_budget_for_all_namespaces(
        optional: ListPodDisruptionBudgetForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListPodDisruptionBudgetForAllNamespacesOptional {
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
        let __url = format!("/apis/policy/v1beta1/poddisruptionbudgets?");
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
// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// partially update the specified PodDisruptionBudget
    ///
    /// Use [`PatchNamespacedPodDisruptionBudgetResponse`](./enum.PatchNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedPodDisruptionBudgetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// partially update status of the specified PodDisruptionBudget
    ///
    /// Use [`PatchNamespacedPodDisruptionBudgetStatusResponse`](./enum.PatchNamespacedPodDisruptionBudgetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchNamespacedPodDisruptionBudgetStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchNamespacedPodDisruptionBudgetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// read the specified PodDisruptionBudget
    ///
    /// Use [`ReadNamespacedPodDisruptionBudgetResponse`](./enum.ReadNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedPodDisruptionBudgetOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
        }
        if let Some(export) = export {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// read status of the specified PodDisruptionBudget
    ///
    /// Use [`ReadNamespacedPodDisruptionBudgetStatusResponse`](./enum.ReadNamespacedPodDisruptionBudgetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedPodDisruptionBudgetStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadNamespacedPodDisruptionBudgetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// replace the specified PodDisruptionBudget
    ///
    /// Use [`ReplaceNamespacedPodDisruptionBudgetResponse`](./enum.ReplaceNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::policy::v1beta1::PodDisruptionBudget,
        optional: ReplaceNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedPodDisruptionBudgetOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudgetStatus

impl PodDisruptionBudget {
    /// replace status of the specified PodDisruptionBudget
    ///
    /// Use [`ReplaceNamespacedPodDisruptionBudgetStatusResponse`](./enum.ReplaceNamespacedPodDisruptionBudgetStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects

    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_pod_disruption_budget_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_13::api::policy::v1beta1::PodDisruptionBudget,
        optional: ReplaceNamespacedPodDisruptionBudgetStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceNamespacedPodDisruptionBudgetStatusOptional {
            dry_run,
            pretty,
        } = optional;
        let __url = format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
        }
        if let Some(pretty) = pretty {
        }
        let __url = __query_pairs.finish();
}
// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudget

impl PodDisruptionBudget {
    /// watch changes to an object of kind PodDisruptionBudget. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use [`WatchNamespacedPodDisruptionBudgetResponse`](./enum.WatchNamespacedPodDisruptionBudgetResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///     name of the PodDisruptionBudget

    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_pod_disruption_budget(
        name: &str,
        namespace: &str,
        optional: WatchNamespacedPodDisruptionBudgetOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedPodDisruptionBudgetOptional {
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
        let __url = format!("/apis/policy/v1beta1/watch/namespaces/{namespace}/poddisruptionbudgets/{name}?", name = name, namespace = namespace);
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
// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudgetList

impl PodDisruptionBudget {
    /// watch individual changes to a list of PodDisruptionBudget. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchNamespacedPodDisruptionBudgetListResponse`](./enum.WatchNamespacedPodDisruptionBudgetListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_pod_disruption_budget_list(
        namespace: &str,
        optional: WatchNamespacedPodDisruptionBudgetListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchNamespacedPodDisruptionBudgetListOptional {
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
        let __url = format!("/apis/policy/v1beta1/watch/namespaces/{namespace}/poddisruptionbudgets?", namespace = namespace);
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
// Generated from operation watchPolicyV1beta1PodDisruptionBudgetListForAllNamespaces

impl PodDisruptionBudget {
    /// watch individual changes to a list of PodDisruptionBudget. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use [`WatchPodDisruptionBudgetListForAllNamespacesResponse`](./enum.WatchPodDisruptionBudgetListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_pod_disruption_budget_list_for_all_namespaces(
        optional: WatchPodDisruptionBudgetListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchPodDisruptionBudgetListForAllNamespacesOptional {
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
        let __url = format!("/apis/policy/v1beta1/watch/poddisruptionbudgets?");
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
// End policy/v1beta1/PodDisruptionBudget

impl crate::Resource for PodDisruptionBudget {
    fn api_version() -> &'static str {
        "policy/v1beta1"
    }

    fn group() -> &'static str {
        "policy"
    }

    fn kind() -> &'static str {
        "PodDisruptionBudget"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for PodDisruptionBudget {
    type Ty = crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for PodDisruptionBudget {
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
            type Value = PodDisruptionBudget;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct PodDisruptionBudget")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_13::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_13::api::policy::v1beta1::PodDisruptionBudgetSpec> = None;
                let mut value_status: Option<crate::v1_13::api::policy::v1beta1::PodDisruptionBudgetStatus> = None;

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

                Ok(PodDisruptionBudget {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDisruptionBudget",
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

impl serde::Serialize for PodDisruptionBudget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDisruptionBudget",
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
