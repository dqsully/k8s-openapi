
// Generated from operation {{ operation.id.as_str() }}

{% match type_name_and_ref_path_and_parent_mod_rs %}
    {%- when Some with (value) -%}
impl {{ value.0 }} {
    {%- else -%}
{%- endmatch %}
{{ operation.description.as_ref().map(AsRef::as_ref)|comment_indent(indent, "") }}
{%- if !operation.description.as_ref().map_or(0 != 0, String::is_empty) -%}
{{ indent }}///
{%- endif %}
{{ indent }}/// Use [`{{ operation_result_name.as_str() }}`](./enum.{{ operation_result_name.as_str() }}.html) to parse the HTTP response.
{%- if !parameters.is_empty() %}
{{ indent }}///
{{ indent }}/// # Arguments
    {%- for value in required_parameters %}
{{ indent }}///
{{ indent }}/// * `{{ value.0.as_ref() }}`
{{ value.2.schema.description.as_ref().map(AsRef::as_ref)|comment_indent(indent, "    ") }}
    {%- endfor -%}
    {%- if !optional_parameters.is_empty() -%}
{{ indent }}///
{{ indent }}/// * `optional`
{{ indent }}///
{{ indent }}///     Optional parameters. Use `Default::default()` to not pass any.
    {%- endif %}
{%- endif %}
{{ indent }}pub fn {{ operation_fn_name.as_ref() }}(
{%- for value in required_parameters %}
{{ indent }}    {{ value.0.as_ref() }}: {{ value.1.as_ref() }},
{%- endfor %}
{%- if !optional_parameters.is_empty() %}
{{ indent }}    optional: {{ operation_optional_parameters_name.as_str() }}{% if any_optional_fields_have_lifetimes %}<'_>{% endif %},
{%- endif %}
{{ indent }}) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
{%- if !optional_parameters.is_empty() %}
{{ indent }}    let {{ operation_optional_parameters_name.as_str() }} {
    {%- for value in optional_parameters %}
{{ indent }}        {{ value.0.as_ref() }},
    {%- endfor %}
{{ indent }}    } = optional;
{{ indent }}    let __url = format!("{{ path }}{% if have_query_parameters %}?{% endif %}"
    {%- for value in parameters -%}
        {%- if value.2.location == crate::swagger20::ParameterLocation::Path -%}
, {{ value.0.as_ref() }} = {{ value.0.as_ref() }}
        {%- endif -%}
    {%- endfor -%}
);
{{ indent }}    let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
    {%- for value in parameters %}
        {%- if value.2.location == crate::swagger20::ParameterLocation::Query %}
            {%- if value.2.required %}
                {%- match value.2.schema.kind %}
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::Boolean) %}{{ indent }}    __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }}.to_string());
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::Integer { .. }) %}{{ indent }}    __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }}.to_string());
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::Number { .. }) %}{{ indent }}    __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }}.to_string());
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::String { .. }) %}{{ indent }}    __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }});
                    {%- else %}compile_error!("parameter {{ value.0.as_ref() }} is in the query string but is a {:?}", {{ "value.2" }});
                {%- endmatch %}
            {%- else %}
{{ indent }}    if let Some({{ value.0.as_ref() }}) = {{ value.0.as_ref() }} {
                {%- match value.2.schema.kind %}
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::Boolean) %}{{ indent }}        __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }}.to_string());
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::Integer { .. }) %}{{ indent }}        __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }}.to_string());
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::Number { .. }) %}{{ indent }}        __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }}.to_string());
                    {%- when crate::swagger20::SchemaKind::Ty with (crate::swagger20::Type::String { .. }) %}{{ indent }}        __query_pairs.append_pair("{{ value.2.name.as_str() }}", &{{ value.0.as_ref() }});
                    {%- else %}compile_error!("parameter {{ value.0.as_ref() }} is in the query string but is a {:?}", {{ "value.2" }});
                {%- endmatch %}
{{ indent }}    }
            {%- endif %}
        {%- endif %}
    {%- endfor %}
{{ indent }}    let __url = __query_pairs.finish();
{%- endif %}
{%- if type_name_and_ref_path_and_parent_mod_rs.is_some() %}
}
{%- endif %}
