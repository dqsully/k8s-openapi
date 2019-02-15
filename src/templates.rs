use askama::Template;

pub(super) struct TypeCommon<'a> {
	pub(super) definition_path: &'a str,
	pub(super) description_comment_text: Option<&'a str>,
	pub(super) type_name: &'a str,
}

#[derive(Template)]
#[template(path = "type_alias.rs")]
pub(super) struct TypeAlias<'a> {
	pub(super) common: &'a TypeCommon<'a>,
	pub(super) inner_type_name: std::borrow::Cow<'a, str>,
	pub(super) can_be_default: bool,
}

#[derive(Template)]
#[template(path = "type_int_or_string.rs")]
pub(super) struct TypeIntOrString<'a> {
	pub(super) common: &'a TypeCommon<'a>,
}

#[derive(Template)]
#[template(path = "type_json_schema_props_or_array.rs")]
pub(super) struct TypeJSONSchemaPropsOrArray<'a> {
	pub(super) common: &'a TypeCommon<'a>,
	pub(super) json_schema_props_type_name: String,
}

#[derive(Template)]
#[template(path = "type_json_schema_props_or_bool.rs")]
pub(super) struct TypeJSONSchemaPropsOrBool<'a> {
	pub(super) common: &'a TypeCommon<'a>,
	pub(super) json_schema_props_type_name: String,
}

#[derive(Template)]
#[template(path = "type_json_schema_props_or_string_array.rs")]
pub(super) struct TypeJSONSchemaPropsOrStringArray<'a> {
	pub(super) common: &'a TypeCommon<'a>,
	pub(super) json_schema_props_type_name: String,
}

#[derive(Template)]
#[template(path = "type_properties_pre.rs")]
pub(super) struct TypePropertiesPre<'a> {
	pub(super) common: &'a TypeCommon<'a>,
	pub(super) can_be_default: bool,
	pub(super) properties: Vec<TypeProperty<'a>>,
	pub(super) resource_metadata: Option<(std::borrow::Cow<'a, str>, &'a str, &'a str, &'a str)>,
	pub(super) metadata_property_ty: Option<(bool, std::borrow::Cow<'a, str>)>,
}

#[derive(Template)]
#[template(path = "type_properties_post.rs")]
pub(super) struct TypePropertiesPost {
}

pub(super) struct TypeProperty<'a> {
	pub(super) name: &'a crate::swagger20::PropertyName,
	pub(super) schema: &'a crate::swagger20::Schema,
	pub(super) required: bool,
	pub(super) field_name: std::borrow::Cow<'static, str>,
	pub(super) field_type_name: String,
}

#[derive(Template)]
#[template(path = "operations_group_pre.rs")]
pub(super) struct OperationsGroupPre<'a> {
	pub(super) kubernetes_group_kind_version: &'a crate::swagger20::KubernetesGroupKindVersion,
}

#[derive(Template)]
#[template(path = "operations_group_post.rs")]
pub(super) struct OperationsGroupPost<'a> {
	pub(super) kubernetes_group_kind_version: &'a crate::swagger20::KubernetesGroupKindVersion,
}

#[derive(Template)]
#[template(path = "operation.rs")]
pub(super) struct Operation<'a> {
	pub(super) kubernetes_group_kind_version: &'a crate::swagger20::KubernetesGroupKindVersion,
	pub(super) path: &'a crate::swagger20::Path,
	pub(super) path_item: &'a crate::swagger20::PathItem,
	pub(super) operation: &'a crate::swagger20::Operation,
	pub(super) type_name_and_ref_path_and_parent_mod_rs: Option<(&'a str, &'a crate::swagger20::RefPath, &'a mut std::io::BufWriter<std::fs::File>)>,

	pub(super) operation_id: std::borrow::Cow<'a, str>,
	pub(super) operation_result_name: String,
	pub(super) operation_optional_parameters_name: String,
	pub(super) operation_responses: Vec<(&'a str, &'a str, Option<&'a crate::swagger20::Schema>, bool)>,
	pub(super) indent: &'static str,
	pub(super) operation_fn_name: std::borrow::Cow<'static, str>,
	pub(super) parameters: &'a [(std::borrow::Cow<'a, str>, std::borrow::Cow<'a, str>, &'a crate::swagger20::Parameter)],
	pub(super) required_parameters: Vec<&'a (std::borrow::Cow<'a, str>, std::borrow::Cow<'a, str>, &'a crate::swagger20::Parameter)>,
	pub(super) optional_parameters: Vec<&'a (std::borrow::Cow<'a, str>, std::borrow::Cow<'a, str>, &'a crate::swagger20::Parameter)>,
	pub(super) any_optional_fields_have_lifetimes: bool,
	pub(super) have_query_parameters: bool,
}

impl<'a> Operation<'a> {
	pub(super) fn new(
		kubernetes_group_kind_version: &'a crate::swagger20::KubernetesGroupKindVersion,
		path: &'a crate::swagger20::Path,
		path_item: &'a crate::swagger20::PathItem,
		operation: &'a crate::swagger20::Operation,
		type_name_and_ref_path_and_parent_mod_rs: Option<(&'a str, &'a crate::swagger20::RefPath, &'a mut std::io::BufWriter<std::fs::File>)>,
		parameters: &'a [(std::borrow::Cow<'a, str>, std::borrow::Cow<'a, str>, &'a crate::swagger20::Parameter)],
	) -> Result<Self, super::Error> {
		let operation_id =
			if type_name_and_ref_path_and_parent_mod_rs.is_some() {
				// For functions associatd with types (eg `Pod::list_core_v1_namespaced_pod`), the API version contained in the operation name
				// is already obvious from the type's path (`core::v1::Pod`), so it can be stripped (`list_namespaced_pod`).
				let tag: String =
					operation.tag.split('_')
					.map(|part| {
						let mut chars = part.chars();
						if let Some(first_char) = chars.next() {
							let rest_chars = chars.as_str();
							std::borrow::Cow::Owned(format!("{}{}", first_char.to_uppercase(), rest_chars))
						}
						else {
							std::borrow::Cow::Borrowed("")
						}
					})
					.collect();

				std::borrow::Cow::Owned(operation.id.replace(&tag, ""))
			}
			else {
				// Functions not associated with types (eg `::get_core_v1_api_resources`) get emitted at the mod root,
				// so their ID should be used as-is.
				std::borrow::Cow::Borrowed(&*operation.id)
			};

		let (operation_result_name, operation_optional_parameters_name) = {
			let mut chars = operation_id.chars();
			let first_char = chars.next().ok_or_else(|| format!("operation has empty ID: {:?}", operation))?.to_uppercase();
			let rest_chars = chars.as_str();
			(
				format!("{}{}Response", first_char, rest_chars),
				format!("{}{}Optional", first_char, rest_chars),
			)
		};

		let operation_responses: Result<Vec<_>, _> =
			operation.responses.iter()
			.map(|(&status_code, schema)| {
				let http_status_code = match status_code {
					reqwest::StatusCode::ACCEPTED => "ACCEPTED",
					reqwest::StatusCode::CREATED => "CREATED",
					reqwest::StatusCode::OK => "OK",
					reqwest::StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
					_ => return Err(format!("unrecognized status code {}", status_code)),
				};

				let variant_name = match status_code {
					reqwest::StatusCode::ACCEPTED => "Accepted",
					reqwest::StatusCode::CREATED => "Created",
					reqwest::StatusCode::OK => "Ok",
					reqwest::StatusCode::UNAUTHORIZED => "Unauthorized",
					_ => return Err(format!("unrecognized status code {}", status_code)),
				};

				let schema = schema.as_ref();

				let is_delete_ok_status = if let Some(schema) = schema {
					match &schema.kind {
						crate::swagger20::SchemaKind::Ref(ref_path) if
							&**ref_path == "io.k8s.apimachinery.pkg.apis.meta.v1.Status" &&
							operation.method == crate::swagger20::Method::Delete &&
							status_code == reqwest::StatusCode::OK => true,

						_ => false,
					}
				}
				else {
					false
				};

				Ok((http_status_code, variant_name, schema, is_delete_ok_status))
			})
			.collect();
		let operation_responses = operation_responses?;

		let indent = if type_name_and_ref_path_and_parent_mod_rs.is_some() { "    " } else { "" };

		let operation_fn_name = super::get_rust_ident(&operation_id);

		let (required_parameters, optional_parameters): (Vec<_>, Vec<_>) = parameters.iter().partition(|(_, _, parameter)| parameter.required);
		let any_optional_fields_have_lifetimes = optional_parameters.iter().any(|(_, parameter_type, _)| parameter_type.starts_with('&'));

		let have_query_parameters = parameters.iter().any(|(_, _, parameter)| parameter.location == crate::swagger20::ParameterLocation::Query);

		Ok(Operation {
			kubernetes_group_kind_version,
			path,
			path_item,
			operation,
			type_name_and_ref_path_and_parent_mod_rs,

			operation_id,
			operation_result_name,
			operation_optional_parameters_name,
			operation_responses,
			indent,
			operation_fn_name,
			parameters,
			required_parameters,
			optional_parameters,
			any_optional_fields_have_lifetimes,
			have_query_parameters,
		})
	}
}

mod filters {
	pub(super) fn comment_indent(comment: &Option<&str>, pre_indent: &str, post_indent: &str) -> askama::Result<String> {
		use std::fmt::Write;

		let comment = if let Some(comment) = comment { comment } else { return Ok(String::new()); };

		let mut result = String::new();

		for line in comment.lines() {
			if line.is_empty() {
				writeln!(result, "{}///", pre_indent)?;
			}
			else {
				let line =
					line
					.replace(r"\", r"\\")
					.replace("[", r"\[")
					.replace("]", r"\]")
					.replace("<", r"\<")
					.replace(">", r"\>");
				writeln!(result, "{}///{} {}", pre_indent, post_indent, line)?;
			}
		}

		Ok(result)
	}
}
