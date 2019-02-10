{% extends "type_json_schema_props.rs" %}

{% block enum_variant_name %}Bool{% endblock %}

{% block enum_variant_def %}Bool(bool){% endblock %}

{% block deserialize_fn %}
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok({{ common.type_name }}::Bool(v))
            }
{%- endblock %}
