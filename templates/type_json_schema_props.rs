{% extends "type.rs" %}

{% block type_definition -%}
#[derive(Clone, Debug, PartialEq)]
pub enum {{ common.type_name }} {
    Schema(Box<{{ json_schema_props_type_name.as_str() }}>),
    {% block enum_variant_def %}{% endblock %},
}

impl<'de> serde::Deserialize<'de> for {{ common.type_name }} {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = {{ common.type_name }};

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "enum {{ common.type_name }}")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                Ok({{ common.type_name }}::Schema(serde::de::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))
            }
{% block deserialize_fn %}{% endblock %}
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for {{ common.type_name }} {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            {{ common.type_name }}::Schema(value) => value.serialize(serializer),
            {{ common.type_name }}::{% block enum_variant_name %}{% endblock %}(value) => value.serialize(serializer),
        }
    }
}
{% endblock %}
