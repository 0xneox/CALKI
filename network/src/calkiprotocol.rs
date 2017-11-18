// CALKI
// Copyright 2016-2017 Zibbit Labs.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! A multiplexed calki protocol

use byteorder::{BigEndian, ByteOrder};
use bytes::BufMut;
use bytes::BytesMut;
use std::io;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Framed, Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;

pub type CalkiRequest = Vec<u8>;
pub type CalkiResponse = Vec<u8>;

/// Our multiplexed line-based codec
pub struct CalkiCodec;

/// Protocol definition
pub struct CalkiProto;

/// Implementation of the multiplexed line-based protocol.
///
/// Frames begin with a 4 byte header, consisting of the numeric request ID
/// encoded in network order, followed by the frame payload encoded as a UTF-8
/// string and terminated with a '\n' character:
///
/// # An example frame:
///
/// +-- request id --+------- frame payload --------+
/// |                |                              |
/// | \xDEADBEEF+len | This is the frame payload    |
/// |                |                              |
/// +----------------+------------------------------+
///

fn opt_bytes_extend(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

impl Decoder for CalkiCodec {
    type Item = CalkiRequest;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        let buf_len = buf.len();
        if buf_len < 8 {
            return Ok(None);
        }

        // check flag and msglen
        let request_id = BigEndian::read_u64(buf.as_ref());
        if request_id & 0xffffffff00000000 != 0xDEADBEEF00000000 {
            return Ok(None);
        }
        let msg_len = request_id & 0x00000000ffffffff;
        if (msg_len + 8) > buf_len as u64 {
            return Ok(None);
        }
        // ok skip the flag
        buf.split_to(8);
        // get msg
        let payload = buf.split_to(msg_len as usize).to_vec();

        trace!("decode msg {:?} {:?}", request_id, payload);

        Ok(Some(payload))
    }
}

impl Encoder for CalkiCodec {
    type Item = CalkiResponse;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        let request_id = 0xDEADBEEF00000000 + msg.len();
        trace!("encode msg {:?} {:?}", request_id, msg);

        let mut encoded_request_id = [0; 8];
        BigEndian::write_u64(&mut encoded_request_id, request_id as u64);

        opt_bytes_extend(buf, &encoded_request_id);
        opt_bytes_extend(buf, &msg);

        Ok(())
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for CalkiProto {
    type Request = CalkiRequest;
    type Response = CalkiResponse;

    /// `Framed<T, CalkiCodec>` is the return value of `io.framed(CalkiCodec)`
    type Transport = Framed<T, CalkiCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(CalkiCodec))
    }
}
