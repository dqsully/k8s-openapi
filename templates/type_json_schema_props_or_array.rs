{% extends "type_json_schema_props.rs" %}

{% block enum_variant_name %}Schemas{% endblock %}

{% block enum_variant_def %}Schemas(Vec<{{ json_schema_props_type_name.as_str() }}>){% endblock %}

{% block deserialize_fn %}
            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok({{ common.type_name }}::Schemas(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))
            }
{%- endblock %}
