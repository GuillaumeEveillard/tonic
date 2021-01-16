use super::{Codec, DecodeBuf, Decoder, Encoder};
use crate::Status;
use std::marker::PhantomData;
use crate::codec::EncodeBuf;

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
        T: /*Message +*/Sync + Send + 'static,
        U: /*Message +*/Sync + Default + Send + 'static,
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

impl<T: Send + Sync/*: Message*/> Encoder for JsonEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // item.encode(buf)
        //     .expect("Message only errors if not enough space");

        Ok(())
    }
}

/// A [`Decoder`] that knows how to decode `U`.
#[derive(Debug, Clone, Default)]
pub struct JsonDecoder<U>(PhantomData<U>);

impl<U: /*Message + */Default> Decoder for JsonDecoder<U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        // let item = Message::decode(buf)
        //     .map(Option::Some)
        //     .map_err(from_decode_error)?;

        // Ok(item)
        Ok(Option::None)
    }
}