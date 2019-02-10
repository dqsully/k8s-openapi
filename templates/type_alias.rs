{% extends "type.rs" %}

{% block type_definition -%}
#[derive(Clone, Debug, {% if can_be_default %}Default, {% endif %}PartialEq)]
pub struct {{ common.type_name }}(pub {{ inner_type_name.as_ref() }});

impl<'de> serde::Deserialize<'de> for {{ common.type_name }} {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = {{ common.type_name }};

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{{ common.type_name }}")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok({{ common.type_name }}(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("{{ common.type_name }}", Visitor)
    }
}

impl serde::Serialize for {{ common.type_name }} {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("{{ common.type_name }}", &self.0)
    }
}
{% endblock %}
