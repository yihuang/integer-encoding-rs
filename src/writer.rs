use std::io::{Result, Write};

use crate::fixed::FixedInt;
use crate::varint::VarInt;

#[cfg(not(feature = "use_futures_types"))]
use tokio::{io::AsyncWriteExt, prelude::*};

#[cfg(feature = "use_futures_types")]
use futures::{io::AsyncWriteExt, prelude::*};

/// A trait for writing integers in VarInt encoding to any `Write` type. This packs encoding and
/// writing into one step.
pub trait VarIntWriter {
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<usize>;
}

/// Like VarIntWriter, but asynchronous.
#[async_trait::async_trait]
pub trait VarIntAsyncWriter {
    /// Write a VarInt integer to an asynchronous writer.
    async fn write_varint_async<VI: VarInt + Send>(&mut self, n: VI) -> Result<usize>;
}

#[async_trait::async_trait]
impl<AW: AsyncWrite + Send + Unpin> VarIntAsyncWriter for AW {
    async fn write_varint_async<VI: VarInt + Send>(&mut self, n: VI) -> Result<usize> {
        let mut buf = [0 as u8; 10];
        let b = n.encode_var(&mut buf);
        self.write(&buf[0..b]).await
    }
}

impl<Inner: Write> VarIntWriter for Inner {
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<usize> {
        let mut buf = [0 as u8; 10];
        let used = n.encode_var(&mut buf[..]);

        self.write(&buf[0..used])
    }
}

/// A trait for writing integers without encoding (i.e. `FixedInt`) to any `Write` type.
pub trait FixedIntWriter {
    fn write_fixedint<FI: FixedInt>(&mut self, n: FI) -> Result<usize>;
}

#[async_trait::async_trait]
pub trait FixedIntAsyncWriter {
    async fn write_fixedint_async<FI: FixedInt + Send>(&mut self, n: FI) -> Result<usize>;
}

#[async_trait::async_trait]
impl<AW: AsyncWrite + Unpin + Send> FixedIntAsyncWriter for AW {
    async fn write_fixedint_async<FI: FixedInt + Send>(&mut self, n: FI) -> Result<usize> {
        let mut buf = [0 as u8; 8];
        n.encode_fixed(&mut buf[0..FI::required_space()]);
        self.write(&buf[0..FI::required_space()]).await
    }
}

impl<W: Write> FixedIntWriter for W {
    fn write_fixedint<FI: FixedInt>(&mut self, n: FI) -> Result<usize> {
        let mut buf = [0 as u8; 8];
        n.encode_fixed(&mut buf[0..FI::required_space()]);

        self.write(&buf[0..FI::required_space()])
    }
}
