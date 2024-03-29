pub mod data;
pub mod device;
pub mod system;
pub mod uri;

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use zenoh::{prelude::r#async::*, subscriber::Subscriber};

use crate::core::ipc::device::hello::Hello;
use crate::core::ipc::system::who_are_you::{What, Who, WhoAreYou};
use crate::core::ipc::uri::uri_list::UriList;
use crate::core::ipc::uri::Uri;
use crate::core::model::device::DeviceModel;

use self::data::measurement::value::{DataValue, MeasurementValue, Quality};
use self::device::hello::HealthState;

pub struct Ipc {
    device_id: String,
    pub session: Session,
}

pub struct IpcHelloMessage<T> {
    pub device_id: String,
    pub hello: Hello,
    pub sender_channel: Sender<T>,
}

pub struct IpcWhoIAmMessage<T> {
    pub device_id: String,
    pub model: DeviceModel,
    pub sender_channel: Sender<T>,
}

pub struct IpcWhoAreYouMessage<T> {
    pub sender_id: String,
    pub who_are_you: WhoAreYou,
    pub sender_channel: Sender<T>,
}

pub type IpcHelloCallback<T> =
    Box<dyn Fn(Result<IpcHelloMessage<T>, String>) + Send + Sync + 'static>;

pub type IpcWhoIAmCallback<T> =
    Box<dyn Fn(Result<IpcWhoIAmMessage<T>, String>) + Send + Sync + 'static>;

pub type IpcWhoAreYouCallback<T> =
    Box<dyn Fn(Result<IpcWhoAreYouMessage<T>, String>) + Send + Sync + 'static>;

impl Ipc {
    pub async fn new(device_id: String) -> Ipc {
        let session = zenoh::open(config::default()).res().await.unwrap();
        Ipc { device_id, session }
    }

    pub async fn publish_who_are_you(&self, device_id: String) {
        let who_are_you = WhoAreYou::new(Who::Id(device_id), What::All);
        let json = serde_json::to_string_pretty(&who_are_you).unwrap();
        let uri = UriList::get_uri_who_are_you(&self.device_id);
        println!("Publish on {}", uri);
        self.session.put(uri.to_string(), json).res().await.unwrap();
    }

    pub async fn publish_hello(&self, state: HealthState) {
        let hello = Hello::new(
            state,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time error")
                .as_millis(),
        );
        let json = serde_json::to_string_pretty(&hello).unwrap();
        let uri = UriList::get_uri_hello(&self.device_id);
        println!("Publish on {}", uri);
        self.session.put(uri.to_string(), json).res().await.unwrap();
    }

    pub async fn publish_who_i_am(&self, model: DeviceModel) {
        let json = serde_json::to_string_pretty(&model).unwrap();
        let uri = UriList::get_uri_who_i_am(&self.device_id);
        println!("Publish on {}", uri);
        self.session.put(uri.to_string(), json).res().await.unwrap();
    }

    pub async fn publish_humidity(&self, value: u8) {
        let humidity = MeasurementValue::new(
            Uuid::new_v4().to_string(),
            "def:meas:humidity".to_string(),
            DataValue::U8(value),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time error")
                .as_millis(),
            Quality::Ok,
        );
        let json = serde_json::to_string_pretty(&humidity).unwrap();
        let uri = UriList::get_uri_humidity(&self.device_id);
        //println!("Publish on {}", uri);
        self.session.put(uri.to_string(), json).res().await.unwrap();
    }

    pub async fn publish_sound(&self, value: u8) {
        let sound = MeasurementValue::new(
            Uuid::new_v4().to_string(),
            "def:meas:sound".to_string(),
            DataValue::U8(value),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time error")
                .as_millis(),
            Quality::Ok,
        );
        let json = serde_json::to_string_pretty(&sound).unwrap();
        let uri = UriList::get_uri_sound(&self.device_id);
        //println!("Publish on {}", uri);
        self.session.put(uri.to_string(), json).res().await.unwrap();
    }

    pub async fn publish_temperature(&self, value: i16) {
        let sound = MeasurementValue::new(
            Uuid::new_v4().to_string(),
            "def:meas:temp".to_string(),
            DataValue::I16(value),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time error")
                .as_millis(),
            Quality::Ok,
        );
        let json = serde_json::to_string_pretty(&sound).unwrap();
        let uri = UriList::get_uri_temperature(&self.device_id);
        //println!("Publish on {}", uri);
        self.session.put(uri.to_string(), json).res().await.unwrap();
    }

    pub async fn subscribe_hello<T: Send + Sync + 'static>(
        &self,
        device_id: String,
        subscriber_callback: IpcHelloCallback<T>,
        sender_channel: Sender<T>,
    ) -> anyhow::Result<Subscriber<()>> {
        let callback = move |sample: Sample| {
            let uri = Uri::from_str(&sample.key_expr);
            let Ok(uri) = uri else {
                subscriber_callback(Err("Uri error".to_string()));
                return;
            };

            let hello = serde_json::from_str::<Hello>(&sample.value.to_string());
            let Ok(hello) = hello else {
                subscriber_callback(Err("Payload error".to_string()));
                return;
            };

            let device = uri.get_device();
            let Some(device) = device else {
                subscriber_callback(Err("Device error".to_string()));
                return;
            };

            let message = IpcHelloMessage {
                device_id: device.get_id().clone(),
                hello,
                sender_channel: sender_channel.clone(),
            };
            subscriber_callback(Ok(message));
        };

        let uri = UriList::get_uri_hello(&device_id);
        println!("Subscribe on {}", uri);

        let subscriber = self
            .session
            .declare_subscriber(uri.to_string())
            .callback_mut(callback)
            .res()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(subscriber)
    }

    pub async fn subscribe_who_i_am<T: Send + Sync + 'static>(
        &self,
        device_id: String,
        subscriber_callback: IpcWhoIAmCallback<T>,
        sender_channel: Sender<T>,
    ) -> anyhow::Result<Subscriber<()>> {
        let callback = move |sample: Sample| {
            let uri = Uri::from_str(&sample.key_expr);
            let Ok(uri) = uri else {
                subscriber_callback(Err("Uri error".to_string()));
                return;
            };

            let model = serde_json::from_str::<DeviceModel>(&sample.value.to_string());
            let Ok(model) = model else {
                subscriber_callback(Err("Payload error".to_string()));
                return;
            };

            let device = uri.get_device();
            let Some(device) = device else {
                subscriber_callback(Err("Device error".to_string()));
                return;
            };

            let message = IpcWhoIAmMessage {
                device_id: device.get_id().clone(),
                model,
                sender_channel: sender_channel.clone(),
            };
            subscriber_callback(Ok(message));
        };

        let uri = UriList::get_uri_who_i_am(&device_id);
        println!("Subscribe on {}", uri);

        let subscriber = self
            .session
            .declare_subscriber(uri.to_string())
            .callback_mut(callback)
            .res()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(subscriber)
    }

    pub async fn subscribe_who_are_you<T: Send + Sync + 'static>(
        &self,
        device_id: String,
        subscriber_callback: IpcWhoAreYouCallback<T>,
        sender_channel: Sender<T>,
    ) -> anyhow::Result<Subscriber<()>> {
        let callback = move |sample: Sample| {
            let uri = Uri::from_str(&sample.key_expr);
            let Ok(uri) = uri else {
                subscriber_callback(Err("Uri error".to_string()));
                return;
            };

            let who_are_you = serde_json::from_str::<WhoAreYou>(&sample.value.to_string());
            let Ok(who_are_you) = who_are_you else {
                subscriber_callback(Err("Payload error".to_string()));
                return;
            };

            let device = uri.get_device();
            let Some(device) = device else {
                subscriber_callback(Err("Device error".to_string()));
                return;
            };

            let message = IpcWhoAreYouMessage {
                sender_id: device.get_id().clone(),
                who_are_you,
                sender_channel: sender_channel.clone(),
            };
            subscriber_callback(Ok(message));
        };

        let uri = UriList::get_uri_who_are_you(&device_id);
        println!("Subscribe on {}", uri);

        let subscriber = self
            .session
            .declare_subscriber(uri.to_string())
            .callback_mut(callback)
            .res()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(subscriber)
    }
}
