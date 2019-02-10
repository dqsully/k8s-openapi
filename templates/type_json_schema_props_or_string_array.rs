{% extends "type_json_schema_props.rs" %}

{% block enum_variant_name %}Strings{% endblock %}

{% block enum_variant_def %}Strings(Vec<String>){% endblock %}

{% block deserialize_fn %}
            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok({{ common.type_name }}::Strings(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))
            }
{%- endblock %}
