{% extends "type.rs" %}

{% block type_definition -%}
#[derive(Clone, Debug, {% if can_be_default %}Default, {% endif %}PartialEq)]
pub struct {{ common.type_name }} {
{%- for property in properties %}
{%- if loop.index0 > 0 %}
{% endif %}
{{ property.schema.description.as_ref().map(AsRef::as_ref)|comment_indent("    ", "") }}    pub {{ property.field_name.as_ref() }}: {{ property.field_type_name.as_str() }},
{%- endfor %}
}
{% endblock %}
