use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub mod uri_list;

#[derive(Serialize, Deserialize, Clone)]
pub enum UriType {
    Field,
    D2D,
}

pub struct ParseUriTypeError;

impl FromStr for UriType {
    type Err = ParseUriTypeError;

    fn from_str(input: &str) -> Result<UriType, Self::Err> {
        match input {
            "field" => Ok(UriType::Field),
            "d2d" => Ok(UriType::D2D),
            _ => Err(ParseUriTypeError),
        }
    }
}

impl fmt::Display for UriType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UriType::Field => write!(f, "field"),
            UriType::D2D => write!(f, "d2d"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UriBase {
    name: String,
    r#type: UriType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UriDevice {
    id: String,
}

impl UriDevice {
    pub fn get_id(&self) -> &String {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UriFacet {
    name: String,
    version: String,
}

impl UriFacet {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UriFields {
    names: Vec<String>,
}

impl UriFields {
    pub fn get_names(&self) -> &Vec<String> {
        &self.names
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Uri {
    base: UriBase,
    device: Option<UriDevice>,
    facet: UriFacet,
    fields: UriFields,
}

impl Uri {
    pub fn new_field_uri(
        facet_name: String,
        facet_version: String,
        fields_names: Vec<String>,
    ) -> Uri {
        Uri {
            base: UriBase {
                name: "jhome".to_string(),
                r#type: UriType::Field,
            },
            device: None,
            facet: UriFacet {
                name: facet_name,
                version: facet_version,
            },
            fields: UriFields {
                names: fields_names,
            },
        }
    }

    pub fn new_d2d_uri(
        device_id: String,
        facet_name: String,
        facet_version: String,
        fields_names: Vec<String>,
    ) -> Uri {
        Uri {
            base: UriBase {
                name: "jhome".to_string(),
                r#type: UriType::D2D,
            },
            device: Some(UriDevice { id: device_id }),
            facet: UriFacet {
                name: facet_name,
                version: facet_version,
            },
            fields: UriFields {
                names: fields_names,
            },
        }
    }

    pub fn get_base(&self) -> &UriBase {
        &self.base
    }

    pub fn get_device(&self) -> &Option<UriDevice> {
        &self.device
    }

    pub fn get_facet(&self) -> &UriFacet {
        &self.facet
    }

    pub fn get_fields(&self) -> &UriFields {
        &self.fields
    }
}

pub struct ParseUriError;

impl FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split: Vec<&str> = s.split('/').collect();

        //UriBase
        let uri_base_name = split.first().ok_or(ParseUriError)?.to_string();
        split.remove(0);
        let uri_base_type = UriType::from_str(split.first().ok_or(ParseUriError)?)
            .ok()
            .ok_or(ParseUriError)?;
        split.remove(0);

        let uri_base = UriBase {
            name: uri_base_name,
            r#type: uri_base_type,
        };

        //UriDevice
        let mut uri_device = None;
        if matches!(uri_base.r#type, UriType::D2D) {
            let uri_device_id = split.first().ok_or(ParseUriError)?.to_string();
            split.remove(0);

            uri_device = Some(UriDevice { id: uri_device_id });
        }

        //UriFacet
        let uri_facet_name = split.first().ok_or(ParseUriError)?.to_string();
        split.remove(0);
        let uri_facet_version = split.first().ok_or(ParseUriError)?.to_string();
        split.remove(0);

        let uri_facet = UriFacet {
            name: uri_facet_name,
            version: uri_facet_version,
        };

        //UriFields
        let mut uri_fields = UriFields { names: Vec::new() };
        for s in split {
            uri_fields.names.push(s.to_string());
        }

        Ok(Uri {
            base: uri_base,
            device: uri_device,
            facet: uri_facet,
            fields: uri_fields,
        })
    }
}

impl From<Uri> for String {
    fn from(value: Uri) -> Self {
        let string;
        if let Some(ref device) = value.device {
            string = format!(
                "{}/{}/{}/{}/{}",
                value.base.name,
                value.base.r#type,
                device.id,
                value.facet.name,
                value.facet.version
            );
        } else {
            string = format!(
                "{}/{}/{}/{}",
                value.base.name, value.base.r#type, value.facet.name, value.facet.version
            );
        }
        let string = value
            .fields
            .names
            .iter()
            .fold(string, |s, c| s + &format!("/{}", c));
        string
    }
}

impl From<&Uri> for String {
    fn from(value: &Uri) -> Self {
        String::from(value.clone())
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
