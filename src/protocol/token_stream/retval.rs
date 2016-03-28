use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use protocol::DecodeTokenStream;
use protocol::types::*;
use protocol::util::ReadCharStream;
use types::ColumnValue;
use ::TdsResult;

/// 2.2.7.18
#[derive(Debug)]
pub struct TokenStreamRetVal<'a> {
    /// param ordinal (relative position of the param within the request after reordering)
    pos: u16,
    name: String,
    status: u8,
    user_type: u32,
    flags: u16,
    tyinfo: TypeInfo,
    data: ColumnValue<'a>,
}

impl<'a> DecodeTokenStream for TokenStreamRetVal<'a> {
    fn decode<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> TdsResult<TokenStreamRetVal<'a>> {
        let pos = try!(cursor.read_u16::<LittleEndian>());
        let name = try!(cursor.read_b_varchar());
        let status = try!(cursor.read_u8());
        let user_type = try!(cursor.read_u32::<LittleEndian>());
        let flags = try!(cursor.read_u16::<LittleEndian>());
        let tyinfo = try!(TypeInfo::decode(cursor));
        let data = try!(ColumnValue::decode(cursor, &tyinfo));

        Ok(TokenStreamRetVal {
            pos: pos,
            name: name,
            status: status,
            user_type: user_type,
            flags: flags,
            tyinfo: tyinfo,
            data: data,
        })
    }
}