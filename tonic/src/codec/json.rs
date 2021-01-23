use super::{Codec, DecodeBuf, Decoder, Encoder};
use crate::Status;
use std::marker::PhantomData;
use crate::codec::EncodeBuf;
use bytes::{BufMut, Buf};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct JsonCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}

impl<T, U> Default for JsonCodec<T, U> {
    fn default() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<T, U> Codec for JsonCodec<T, U>
    where
        T: /*Message +*/Serialize + Sync + Send + 'static,
        U: /*Message +*/DeserializeOwned + Sync + Default + Send + 'static,
{
    type Encode = T;
    type Decode = U;

    type Encoder = JsonEncoder<T>;
    type Decoder = JsonDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        JsonEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        JsonDecoder(PhantomData)
    }
}

#[derive(Debug, Clone, Default)]
pub struct JsonEncoder<T: Send + Sync>(PhantomData<T>);

impl<T: Send + Sync + Serialize/*: Message*/> Encoder for JsonEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        let serialized = serde_json::to_string(&item).unwrap();
        buf.put(serialized.into_bytes().as_slice());
        Ok(())
    }
}

/// A [`Decoder`] that knows how to decode `U`.
#[derive(Debug, Clone, Default)]
pub struct JsonDecoder<U>(PhantomData<U>);

impl<U: /*Message + */Default + DeserializeOwned> Decoder for JsonDecoder<U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let r = buf.remaining();
        let deserialized: U = {
            let chunk = buf.chunk();
            let json = String::from_utf8_lossy(chunk);
            serde_json::from_str(&json.clone()).unwrap()
        };
        buf.advance(r);
        Ok(Option::Some(deserialized))
    }
}