// Generated from definition io.k8s.api.networking.v1.NetworkPolicy

/// NetworkPolicy describes what network traffic is allowed for a set of Pods
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicy {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior for this NetworkPolicy.
    pub spec: Option<crate::v1_11::api::networking::v1::NetworkPolicySpec>,
}

// Begin networking.k8s.io/v1/NetworkPolicy

// Generated from operation createNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// create a NetworkPolicy
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedNetworkPolicyResponse`]`>` constructor, or [`CreateNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_namespaced_network_policy(
        namespace: &str,
        body: &crate::v1_11::api::networking::v1::NetworkPolicy,
        optional: CreateNamespacedNetworkPolicyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let CreateNamespacedNetworkPolicyOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request(
            crate::http::Method::POST,
            std::borrow::Cow::Owned(__url),
            &[
                ("pretty", pretty.as_ref().map(|value| value as _)),
            ],
            Some(("application/json", body)),
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Optional parameters of [`NetworkPolicy::create_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedNetworkPolicyOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::create_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedNetworkPolicyResponse {
    Ok(crate::v1_11::api::networking::v1::NetworkPolicy),
    Created(crate::v1_11::api::networking::v1::NetworkPolicy),
    Accepted(crate::v1_11::api::networking::v1::NetworkPolicy),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedNetworkPolicyResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedNetworkPolicyResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedNetworkPolicyResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((CreateNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteNetworkingV1CollectionNamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// delete collection of NetworkPolicy
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionNamespacedNetworkPolicyResponse`]`>` constructor, or [`DeleteCollectionNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    pub fn delete_collection_namespaced_network_policy(
        namespace: &str,
        delete_optional: crate::v1_11::DeleteOptional<'_>,
        list_optional: crate::v1_11::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request2(
            crate::http::Method::DELETE,
            __url,
            |__query_pairs| list_optional.__serialize(__query_pairs),
            Some(("application/json", &delete_optional)),
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<DeleteCollectionNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::delete_collection_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionNamespacedNetworkPolicyResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::networking::v1::NetworkPolicyList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedNetworkPolicyResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionNamespacedNetworkPolicyResponse::OkValue(result), buf.len()))
                }
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((DeleteCollectionNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// delete a NetworkPolicy
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteNamespacedNetworkPolicyResponse`]`>` constructor, or [`DeleteNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_namespaced_network_policy(
        name: &str,
        namespace: &str,
        optional: crate::v1_11::DeleteOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request(
            crate::http::Method::DELETE,
            std::borrow::Cow::Owned(__url),
            &[],
            Some(("application/json", &optional)),
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<DeleteNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::delete_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteNamespacedNetworkPolicyResponse {
    OkStatus(crate::v1_11::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_11::api::networking::v1::NetworkPolicy),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedNetworkPolicyResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteNamespacedNetworkPolicyResponse::OkValue(result), buf.len()))
                }
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((DeleteNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// list or watch objects of kind NetworkPolicy
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNamespacedNetworkPolicyResponse`]`>` constructor, or [`ListNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_namespaced_network_policy(
        namespace: &str,
        optional: crate::v1_11::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request2(
            crate::http::Method::GET,
            __url,
            |__query_pairs| optional.__serialize(__query_pairs),
            None,
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<ListNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::list_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListNamespacedNetworkPolicyResponse {
    Ok(crate::v1_11::api::networking::v1::NetworkPolicyList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNamespacedNetworkPolicyResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listNetworkingV1NetworkPolicyForAllNamespaces

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// list or watch objects of kind NetworkPolicy
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListNetworkPolicyForAllNamespacesResponse`]`>` constructor, or [`ListNetworkPolicyForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_network_policy_for_all_namespaces(
        optional: crate::v1_11::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListNetworkPolicyForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/apis/networking.k8s.io/v1/networkpolicies?";
        let __request = crate::__build_request2(
            crate::http::Method::GET,
            __url.to_owned(),
            |__query_pairs| optional.__serialize(__query_pairs),
            None,
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<ListNetworkPolicyForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::list_network_policy_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListNetworkPolicyForAllNamespacesResponse {
    Ok(crate::v1_11::api::networking::v1::NetworkPolicyList),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListNetworkPolicyForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListNetworkPolicyForAllNamespacesResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListNetworkPolicyForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// partially update the specified NetworkPolicy
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchNamespacedNetworkPolicyResponse`]`>` constructor, or [`PatchNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_namespaced_network_policy(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::v1_11::PatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request2(
            crate::http::Method::PATCH,
            __url,
            |__query_pairs| optional.__serialize(__query_pairs),
            Some((match body {
                crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
                crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
                crate::v1_11::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
            }, body)),
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<PatchNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::patch_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchNamespacedNetworkPolicyResponse {
    Ok(crate::v1_11::api::networking::v1::NetworkPolicy),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchNamespacedNetworkPolicyResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((PatchNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// read the specified NetworkPolicy
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedNetworkPolicyResponse`]`>` constructor, or [`ReadNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_namespaced_network_policy(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedNetworkPolicyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let ReadNamespacedNetworkPolicyOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request(
            crate::http::Method::GET,
            std::borrow::Cow::Owned(__url),
            &[
                ("exact", exact.as_ref().map(|value| value as _)),
                ("export", export.as_ref().map(|value| value as _)),
                ("pretty", pretty.as_ref().map(|value| value as _)),
            ],
            None,
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Optional parameters of [`NetworkPolicy::read_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedNetworkPolicyOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::read_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedNetworkPolicyResponse {
    Ok(crate::v1_11::api::networking::v1::NetworkPolicy),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedNetworkPolicyResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// replace the specified NetworkPolicy
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceNamespacedNetworkPolicyResponse`]`>` constructor, or [`ReplaceNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the NetworkPolicy
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_namespaced_network_policy(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::api::networking::v1::NetworkPolicy,
        optional: ReplaceNamespacedNetworkPolicyOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let ReplaceNamespacedNetworkPolicyOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request(
            crate::http::Method::PUT,
            std::borrow::Cow::Owned(__url),
            &[
                ("pretty", pretty.as_ref().map(|value| value as _)),
            ],
            Some(("application/json", body)),
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Optional parameters of [`NetworkPolicy::replace_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceNamespacedNetworkPolicyOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::replace_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceNamespacedNetworkPolicyResponse {
    Ok(crate::v1_11::api::networking::v1::NetworkPolicy),
    Created(crate::v1_11::api::networking::v1::NetworkPolicy),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedNetworkPolicyResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceNamespacedNetworkPolicyResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReplaceNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchNetworkingV1NamespacedNetworkPolicy

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// list or watch objects of kind NetworkPolicy
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNamespacedNetworkPolicyResponse`]`>` constructor, or [`WatchNamespacedNetworkPolicyResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_namespaced_network_policy(
        namespace: &str,
        optional: crate::v1_11::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNamespacedNetworkPolicyResponse>), crate::RequestError> {
        let __url = format!("/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies?",
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let __request = crate::__build_request2(
            crate::http::Method::GET,
            __url,
            |__query_pairs| optional.__serialize(__query_pairs),
            None,
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<WatchNamespacedNetworkPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::watch_namespaced_network_policy`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchNamespacedNetworkPolicyResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent<NetworkPolicy>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchNamespacedNetworkPolicyResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchNamespacedNetworkPolicyResponse::Ok(result), byte_offset))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchNamespacedNetworkPolicyResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchNetworkingV1NetworkPolicyForAllNamespaces

#[cfg(feature = "api")]
impl NetworkPolicy {
    /// list or watch objects of kind NetworkPolicy
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchNetworkPolicyForAllNamespacesResponse`]`>` constructor, or [`WatchNetworkPolicyForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_network_policy_for_all_namespaces(
        optional: crate::v1_11::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchNetworkPolicyForAllNamespacesResponse>), crate::RequestError> {
        let __url = "/apis/networking.k8s.io/v1/networkpolicies?";
        let __request = crate::__build_request2(
            crate::http::Method::GET,
            __url.to_owned(),
            |__query_pairs| optional.__serialize(__query_pairs),
            None,
        )?;
        Ok((__request, crate::ResponseBody::new))
    }
}

/// Use `<WatchNetworkPolicyForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`NetworkPolicy::watch_network_policy_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchNetworkPolicyForAllNamespacesResponse {
    Ok(crate::v1_11::apimachinery::pkg::apis::meta::v1::WatchEvent<NetworkPolicy>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchNetworkPolicyForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchNetworkPolicyForAllNamespacesResponse::Ok(result), byte_offset))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchNetworkPolicyForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// End networking.k8s.io/v1/NetworkPolicy

impl crate::Resource for NetworkPolicy {
    fn api_version() -> &'static str {
        "networking.k8s.io/v1"
    }

    fn group() -> &'static str {
        "networking.k8s.io"
    }

    fn kind() -> &'static str {
        "NetworkPolicy"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for NetworkPolicy {
    type Ty = crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for NetworkPolicy {
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
            type Value = NetworkPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct NetworkPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_11::api::networking::v1::NetworkPolicySpec> = None;

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

                Ok(NetworkPolicy {
                    metadata: value_metadata,
                    spec: value_spec,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicy",
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

impl serde::Serialize for NetworkPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicy",
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
