use canopus_engine::remarks::Remark;
use std::io::{self, Write};
use uuid::Uuid;

pub struct Object {
    properties: Vec<ObjectProperty>,
}

struct ObjectProperty {
    property_name: String,
    property_value: String,
}

impl Object {
    fn new(properties: Vec<ObjectProperty>) -> Object {
        Object { properties }
    }
}

pub fn write_object(object: impl Into<Object>, mut writer: impl Write) -> io::Result<()> {
    let object = object.into();

    if object.properties.is_empty() {
        return Ok(());
    }

    let mut json = serde_json::json!({});

    object.properties.into_iter().for_each(|property| {
        json[property.property_name] = serde_json::Value::from(property.property_value);
    });

    serde_json::to_writer_pretty(&mut writer, &json)?;

    Ok(())
}

impl From<Remark> for Object {
    fn from(value: Remark) -> Self {
        let properties = vec![
            ObjectProperty {
                property_name: "ID".to_string(),
                property_value: value.id().to_string(),
            },
            ObjectProperty {
                property_name: "Essence".to_string(),
                property_value: value.essence().to_string(),
            },
            ObjectProperty {
                property_name: "CreatedAt".to_string(),
                property_value: value.created_at().to_string(),
            },
            ObjectProperty {
                property_name: "UpdatedAt".to_string(),
                property_value: value.updated_at().to_string(),
            },
        ];

        Object::new(properties)
    }
}

impl From<Uuid> for Object {
    fn from(value: Uuid) -> Self {
        Object::new(vec![ObjectProperty {
            property_name: "ID".to_string(),
            property_value: value.to_string(),
        }])
    }
}
