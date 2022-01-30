use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::Status;
use std::marker::PhantomData;
use prost::{DecodeError, Message};
use log::{debug};
use tonic::Code;

#[derive(Debug, Clone)]
struct CustomCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}


impl<T, U> Default for CustomCodec<T, U> {
    fn default() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<T, U> Codec for CustomCodec<T, U> where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
{
    type Encode = T;
    type Decode = U;
    type Encoder = CustomCodecEncoder<T>;
    type Decoder = CustomCodecDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        CustomCodecEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        CustomCodecDecoder(PhantomData)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CustomCodecEncoder<T>(PhantomData<T>);

impl<T: Message> Encoder for CustomCodecEncoder<T> {
    type Item = T;
    type Error = Status;


    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        debug!("ENCODE");
        debug!("ITEM: {:?}", item);

        item.encode(buf)
            .expect("Message only errors if not enough space");

        debug!("BUF: {:?}", buf);

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CustomCodecDecoder<U>(PhantomData<U>);

impl<U: Message + Default> Decoder for CustomCodecDecoder<U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        debug!("DECODE");
        debug!("BUF: {:?}", buf);
        let item = Message::decode(buf)
            .map(Option::Some)
            .map_err(from_decode_error)?;

        debug!("ITEM: {:?}", item);
        Ok(item)
    }
}

fn from_decode_error(error: DecodeError) -> crate::Status {
    // Map Protobuf parse errors to an INTERNAL status code, as per
    // https://github.com/grpc/grpc/blob/master/doc/statuscodes.md
    Status::new(Code::Internal, error.to_string())
}