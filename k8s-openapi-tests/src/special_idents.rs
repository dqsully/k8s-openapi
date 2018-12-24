#[test]
fn special_idents() {
	use k8s_openapi::api::core::v1 as api;
	use k8s_openapi::api::authorization::v1 as authorization;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;
	use k8s_openapi::api::rbac::v1beta1 as rbac;

	let _ = api::FCVolumeSource { target_wwns: Default::default(), ..Default::default() };

	let _ = api::ServiceSpec { external_ips: Default::default(), ..Default::default() };

	let _ = authorization::NonResourceRule { non_resource_urls: Default::default(), ..Default::default() };

	let _ = meta::APIGroup { server_address_by_client_cidrs: Default::default(), ..Default::default() };

	let _ = meta::APIVersions { server_address_by_client_cidrs: Default::default(), ..Default::default() };

	let _ = rbac::PolicyRule { non_resource_urls: Default::default(), ..Default::default() };
}
