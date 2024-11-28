use crate::{
    message_header_codec, Decoder, Encoder, MessageHeaderDecoder, MessageHeaderEncoder,
    MessageType, ReadBuf, Reader, WriteBuf, Writer,
};

pub use decoder::FirstDataBarDecoder;
pub use encoder::FirstDataBarEncoder;

pub const SBE_BLOCK_LENGTH: u16 = 4;
pub const SBE_TEMPLATE_ID: u16 = 205;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub mod encoder {
    use super::{
        Encoder, MessageHeaderEncoder, MessageType, WriteBuf, Writer, SBE_BLOCK_LENGTH,
        SBE_SCHEMA_ID, SBE_SCHEMA_VERSION, SBE_TEMPLATE_ID,
    };

    #[derive(Debug, Default)]
    pub struct FirstDataBarEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for FirstDataBarEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for FirstDataBarEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> FirstDataBarEncoder<'a> {
        #[must_use]
        pub const fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        #[must_use]
        pub const fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        #[must_use]
        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_type(&mut self, value: MessageType) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value as u16);
        }

        /// primitive field 'symbolID'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 2
        #[inline]
        pub fn symbol_id(&mut self, value: u16) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u16_at(offset, value);
        }
    }
} // end encoder

pub mod decoder {
    use super::{
        message_header_codec, Decoder, MessageHeaderDecoder, MessageType, ReadBuf, Reader,
        SBE_TEMPLATE_ID,
    };

    #[derive(Clone, Copy, Debug, Default)]
    pub struct FirstDataBarDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for FirstDataBarDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for FirstDataBarDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> FirstDataBarDecoder<'a> {
        #[must_use]
        pub const fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        #[must_use]
        pub const fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        #[must_use]
        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// REQUIRED enum
        #[inline]
        #[must_use]
        pub fn message_type(&self) -> MessageType {
            self.get_buf().get_u16_at(self.offset).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        #[must_use]
        pub fn symbol_id(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 2)
        }
    }
} // end decoder