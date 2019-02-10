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
#[template(path = "type_properties.rs")]
pub(super) struct TypeProperties<'a> {
	pub(super) common: &'a TypeCommon<'a>,
	pub(super) can_be_default: bool,
	pub(super) properties: Vec<TypeProperty<'a>>,
}

pub(super) enum Type<'a> {
	Alias(TypeAlias<'a>),
	IntOrString(TypeIntOrString<'a>),
	JSONSchemaPropsOrArray(TypeJSONSchemaPropsOrArray<'a>),
	JSONSchemaPropsOrBool(TypeJSONSchemaPropsOrBool<'a>),
	JSONSchemaPropsOrStringArray(TypeJSONSchemaPropsOrStringArray<'a>),
	Properties(TypeProperties<'a>),
}

impl<'a> askama::Template for Type<'a> {
	fn render_into(&self, writer: &mut std::fmt::Write) -> askama::Result<()> {
		match self {
			Type::Alias(template) => template.render_into(writer),
			Type::IntOrString(template) => template.render_into(writer),
			Type::JSONSchemaPropsOrArray(template) => template.render_into(writer),
			Type::JSONSchemaPropsOrBool(template) => template.render_into(writer),
			Type::JSONSchemaPropsOrStringArray(template) => template.render_into(writer),
			Type::Properties(template) => template.render_into(writer),
		}
	}

	fn extension(&self) -> Option<&str> {
		Some("rs")
	}
}

pub(super) struct TypeProperty<'a> {
	pub(super) name: &'a crate::swagger20::PropertyName,
	pub(super) schema: &'a crate::swagger20::Schema,
	pub(super) required: bool,
	pub(super) field_name: std::borrow::Cow<'static, str>,
	pub(super) field_type_name: String,
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
