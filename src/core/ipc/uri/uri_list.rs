use crate::core::ipc::uri::Uri;

pub struct UriList;

impl UriList {
    pub fn get_uri_hello(device_id: &str) -> Uri {
        Uri::new_d2d_uri(
            device_id.to_string(),
            "hello".to_string(),
            "V1".to_string(),
            vec!["hello".to_string()],
        )
    }

    pub fn get_uri_who_are_you(device_id: &str) -> Uri {
        Uri::new_d2d_uri(
            device_id.to_string(),
            "hello".to_string(),
            "V1".to_string(),
            vec!["who-are-you".to_string()],
        )
    }

    pub fn get_uri_who_i_am(device_id: &str) -> Uri {
        Uri::new_d2d_uri(
            device_id.to_string(),
            "hello".to_string(),
            "V1".to_string(),
            vec!["who-i-am".to_string()],
        )
    }

    pub fn get_uri_humidity(device_id: &str) -> Uri {
        Uri::new_d2d_uri(
            device_id.to_string(),
            "measurement-value".to_string(),
            "V1".to_string(),
            vec!["humidity".to_string()],
        )
    }

    pub fn get_uri_sound(device_id: &str) -> Uri {
        Uri::new_d2d_uri(
            device_id.to_string(),
            "measurement-value".to_string(),
            "V1".to_string(),
            vec!["sound".to_string()],
        )
    }

    pub fn get_uri_temperature(device_id: &str) -> Uri {
        Uri::new_d2d_uri(
            device_id.to_string(),
            "measurement-value".to_string(),
            "V1".to_string(),
            vec!["temp".to_string()],
        )
    }
}
