use crate::{error, Bytes};
use num_traits::ToPrimitive;

const UNTAGGED_SIZE_LIMIT: u8 = 55;

const UNTAGGED_LIMIT: u8 = 127;
const BYTE_ARRAY_OFFSET: u8 = 128;
const LIST_OFFSET: u8 = 192;

// Pattern helpers
const BYTE_ARRAY_UNTAGGED_LIMIT: u8 = BYTE_ARRAY_OFFSET + UNTAGGED_SIZE_LIMIT;
const BYTE_ARRAY_TAGGED_OFFSET: u8 = BYTE_ARRAY_UNTAGGED_LIMIT + 1;
const BYTE_ARRAY_LIMIT: u8 = LIST_OFFSET - 1;
const LIST_UNTAGGED_LIMIT: u8 = LIST_OFFSET + UNTAGGED_SIZE_LIMIT;
const LIST_TAGGED_OFFSET: u8 = LIST_UNTAGGED_LIMIT + 1;

#[derive(Debug, Clone, PartialEq)]
pub enum RLPItem {
    ByteArray(Bytes),
    List(Vec<RLPItem>),
}

impl RLPItem {
    pub fn size(&self) -> usize {
        match self {
            RLPItem::ByteArray(bytes) => bytes.len(),
            RLPItem::List(rlps) => rlps.into_iter().map(|rlp| rlp.size()).sum()
        }
    }

    pub fn flatten(&self) -> Bytes {
        let size = self.size();
        let mut vec = Vec::with_capacity(size);

        fn fill(rlp: &RLPItem, v: &mut Bytes) {
            match rlp {
                RLPItem::ByteArray(bytes) => {
                    v.extend(bytes);
                },
                RLPItem::List(rlps) => {
                    for el in rlps {
                        fill(el, v);
                    }
                }
            }
        }

        fill(self, &mut vec);

        vec
    }
}

#[derive(Debug, PartialEq)]
pub enum DecodingErr {
    Trailing {
        input: Bytes,
        undecoded: Bytes,
        decoded: RLPItem,
    },
    LeadingZerosInSize,
}

pub trait ToRLPItem {
    fn to_rlp_item(&self) -> RLPItem;
}

pub trait FromRLPItem: Sized {
    fn from_rlp_item(item: &RLPItem) -> Result<Self, error::DecodingErr>;
}

pub fn encode(item: &RLPItem) -> Bytes {
    match item {
        RLPItem::ByteArray(bytes) => {
            if bytes.len() == 1 && bytes[0] <= UNTAGGED_LIMIT {
                bytes.to_vec()
            } else {
                add_size(BYTE_ARRAY_OFFSET, bytes.to_vec())
            }
        }
        RLPItem::List(items) => {
            let bytes: Bytes = items.into_iter().flat_map(encode).collect();
            add_size(LIST_OFFSET, bytes)
        }
    }
}

pub fn decode(bytes: &[u8]) -> Result<RLPItem, DecodingErr> {
    // TODO: handle the case of empty bytes
    match try_decode(bytes)? {
        (item, []) => Ok(item),
        (item, rest) => Err(DecodingErr::Trailing {
            input: bytes.to_vec(),
            undecoded: rest.to_vec(),
            decoded: item,
        }),
    }
}

fn try_decode(bytes: &[u8]) -> Result<(RLPItem, &[u8]), DecodingErr> {
    let res = match bytes[0] {
        ..=UNTAGGED_LIMIT =>
            (RLPItem::ByteArray(bytes[0..1].to_vec()), &bytes[1..]),
        BYTE_ARRAY_OFFSET..=BYTE_ARRAY_UNTAGGED_LIMIT => {
            let len: usize = bytes[0] as usize - 128;
            // TODO: Make sure that there is enough bytes
            (
                RLPItem::ByteArray(bytes[1..len + 1].to_vec()),
                &bytes[len + 1..],
            )
        }
        BYTE_ARRAY_TAGGED_OFFSET..=BYTE_ARRAY_LIMIT => {
            let len_bytes: usize = bytes[0] as usize - 183;
            // TODO: Make sure len_bytes > 0 && <= 8
            // TODO: Make sure that there is enough bytes
            // TOOD: Remove the unwrap and try_into
            // TODO: Make sure len does not start with 0 byte
            if bytes[1] == 0 {
                Err(DecodingErr::LeadingZerosInSize)?
            } else {
                let len: usize = bytes_to_size(bytes[1..len_bytes + 1].to_vec());
                (
                    RLPItem::ByteArray(bytes[len_bytes + 1..len_bytes + len + 1].to_vec()),
                    &bytes[len_bytes + len + 1..],
                )
            }
        }
        LIST_OFFSET..=LIST_UNTAGGED_LIMIT => {
            let len: usize = bytes[0] as usize - 192;
            let rest = &bytes[len + 1..];
            let mut list_rest = &bytes[1..len + 1];
            let mut items = Vec::with_capacity(len);
            while !list_rest.is_empty() {
                let decoded = try_decode(&list_rest)?;
                let item = decoded.0;
                list_rest = decoded.1;
                items.push(item);
            }
            items.truncate(items.len());
            (RLPItem::List(items), rest)
        }
        LIST_TAGGED_OFFSET.. => {
            let len_bytes: usize = bytes[0] as usize - 247;
            if bytes[1] == 0 {
                Err(DecodingErr::LeadingZerosInSize)?
            } else {
                let len: usize = bytes_to_size(bytes[1..len_bytes + 1].to_vec());

                let rest = &bytes[1 + len_bytes + len..];
                let mut list_rest = &bytes[1 + len_bytes..1 + len_bytes + len];
                let mut items = Vec::with_capacity(len);
                while !list_rest.is_empty() {
                    let decoded = try_decode(&list_rest)?;
                    let item = decoded.0;
                    list_rest = decoded.1;
                    items.push(item);
                }
                items.truncate(items.len());
                (RLPItem::List(items), rest)
            }
        }
    };

    Ok(res)
}

fn add_size(offset: u8, bytes: Bytes) -> Bytes {
    if bytes.len() <= UNTAGGED_SIZE_LIMIT as usize {
        let mut res = Vec::with_capacity(bytes.len() + 1);
        res.push(offset + bytes.len() as u8);
        res.extend(bytes);
        res
    } else {
        let size_bytes = usize_to_min_be_bytes(bytes.len());
        let tagged_size = (UNTAGGED_SIZE_LIMIT as usize + offset as usize + size_bytes.len())
            .to_u8()
            .expect("Large tagged size");

        let mut res = Vec::with_capacity(bytes.len() + 5);
        res.push(tagged_size);
        res.extend(size_bytes);
        res.extend(bytes);
        res
    }
}

fn bytes_to_size(mut bytes: Bytes) -> usize {
    let total = std::mem::size_of::<usize>();

    bytes.reverse();
    bytes.resize(total, 0);
    bytes.reverse();

    usize::from_be_bytes(bytes.try_into().unwrap())
}

fn usize_to_min_be_bytes(n: usize) -> Bytes {
    let byte_len = n.ilog(256) as usize + 1;
    let bytes = n.to_be_bytes();
    bytes[bytes.len() - byte_len..].to_vec()
}

impl ToRLPItem for u32 {
    fn to_rlp_item(&self) -> RLPItem {
        RLPItem::ByteArray(usize_to_min_be_bytes(*self as usize))
    }
}

impl ToRLPItem for bool {
    fn to_rlp_item(&self) -> RLPItem {
        RLPItem::ByteArray(vec![*self as u8])
    }
}

impl ToRLPItem for Vec<u8> {
    fn to_rlp_item(&self) -> RLPItem {
        RLPItem::ByteArray(self.to_vec())
    }
}

impl ToRLPItem for [RLPItem] {
    fn to_rlp_item(&self) -> RLPItem {
        RLPItem::List(self.to_vec())
    }
}

impl FromRLPItem for u32 {
    fn from_rlp_item(item: &RLPItem) -> Result<Self, error::DecodingErr> {
        match item {
            RLPItem::List(_) => Err(error::DecodingErr::InvalidInt),
            RLPItem::ByteArray(bytes) => {
                if bytes.len() > 0 && bytes.len() <= 4 && bytes[0] != 0 {
                    let mut bytes_vec = vec![0; 4 - bytes.len()];
                    bytes_vec.extend(bytes);
                    let bytes_arr: [u8; 4] = bytes_vec
                        .try_into()
                        .or(Err(error::DecodingErr::InvalidInt))?;
                    Ok(u32::from_be_bytes(bytes_arr))
                } else {
                    Err(error::DecodingErr::InvalidInt)
                }
            }
        }
    }
}

impl FromRLPItem for bool {
    fn from_rlp_item(item: &RLPItem) -> Result<Self, error::DecodingErr> {
        match item {
            RLPItem::List(_) => Err(error::DecodingErr::InvalidBool),
            RLPItem::ByteArray(bytes) => {
                if *bytes == vec![0u8] {
                    Ok(false)
                } else if *bytes == vec![1u8] {
                    Ok(true)
                } else {
                    Err(error::DecodingErr::InvalidBool)
                }
            }
        }
    }
}

impl FromRLPItem for Vec<u8> {
    fn from_rlp_item(item: &RLPItem) -> Result<Self, error::DecodingErr> {
        match item {
            RLPItem::List(_) => Err(error::DecodingErr::InvalidBinary),
            RLPItem::ByteArray(bytes) => Ok(bytes.to_vec()),
        }
    }
}

impl FromRLPItem for Vec<RLPItem> {
    fn from_rlp_item(item: &RLPItem) -> Result<Self, error::DecodingErr> {
        match item {
            RLPItem::ByteArray(_) => Err(error::DecodingErr::InvalidList),
            RLPItem::List(items) => Ok(items.to_vec()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::{prelude::*, collection::VecStrategy};
    use prop::collection::vec;

    fn any_u8vec<TMin, TMax>(min_len: TMin, max_len: TMax) -> VecStrategy<proptest::num::u8::Any>
    where TMin: Into<usize>,
          TMax: Into<usize>
    {
        vec(any::<u8>(), min_len.into()..max_len.into())
    }

    impl proptest::arbitrary::Arbitrary for RLPItem {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            let leaf = any::<Vec<u8>>().prop_map(RLPItem::ByteArray);
            leaf.prop_recursive(
                5,     // deep
                256,   // max nodes
                1000, // max items per collection
                |inner| vec(inner, 0..10000).prop_map(RLPItem::List),
            )
            .boxed()
        }
    }

    fn encode_then_decode(input: RLPItem, expect: Bytes) {
        let encoded = encode(&input);
        let decoded = decode(&encoded);

        assert_eq!(encoded, expect);
        assert_eq!(decoded, Ok(input));
    }

    proptest! {
        #[test]
        fn usize_to_min_be_bytes_plain(n: usize) {
            let expected: Vec<u8> = n.to_be_bytes().iter().skip_while(|n| **n == 0).copied().collect();
            let actual   = usize_to_min_be_bytes(n);
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn encode_decode(rlp: RLPItem) {
            let e = encode(&rlp);
            let d = decode(&e).expect("decoding failed");
            prop_assert_eq!(rlp, d);
        }

        #[test]
        fn one_byte(b in 0..=UNTAGGED_LIMIT) {
            let input = RLPItem::ByteArray(vec![b]);
            let expect = vec![b];
            encode_then_decode(input, expect);
        }


        #[test]
        fn one_byte_size_bytes(input_bytes in any_u8vec(1u8, UNTAGGED_SIZE_LIMIT + 1)) {
            prop_assume!(input_bytes[0] > UNTAGGED_LIMIT);

            let input  = RLPItem::ByteArray(input_bytes.to_vec());
            let expect = vec![BYTE_ARRAY_OFFSET + input_bytes.len() as u8]
                .into_iter()
                .chain(input_bytes)
                .collect();
            encode_then_decode(input, expect);
        }

        #[test]
        fn tagged_size_bytes(input_bytes in any_u8vec(UNTAGGED_SIZE_LIMIT + 1, UNTAGGED_SIZE_LIMIT as usize * 8)) {
            let len = input_bytes.len();
            let len_bytes = len.ilog(256) as u8 + 1;
            let tag = BYTE_ARRAY_OFFSET + UNTAGGED_SIZE_LIMIT as u8 + len_bytes;
            let input = RLPItem::ByteArray(input_bytes.to_vec());
            let expect = vec![tag]
                .into_iter()
                .chain(usize_to_min_be_bytes(len))
                .chain(input_bytes)
                .collect();
            encode_then_decode(input, expect);
        }

        #[test]
        fn one_byte_array_list(
            input_list
                in vec(any::<u8>().prop_map(|n| RLPItem::ByteArray(vec![n % (UNTAGGED_SIZE_LIMIT + 1)])),
                       1..=UNTAGGED_SIZE_LIMIT as usize
        )) {
            let payload: Bytes = input_list.iter().flat_map(|x| encode(x)).collect();
            let tag = LIST_OFFSET + payload.len() as u8;
            let input = RLPItem::List(input_list);
            let expect= vec![tag]
                .into_iter()
                .chain(payload)
                .collect();
            encode_then_decode(input, expect);
        }

        #[test]
        fn byte_array_tagged_size_list(
            input_list
                in vec(any::<u8>().prop_map(|n| RLPItem::ByteArray(vec![n % (UNTAGGED_SIZE_LIMIT + 1)])),
                       (LIST_OFFSET as usize + 1)..=(UNTAGGED_SIZE_LIMIT as usize * 4))
        ) {
            let payload: Bytes = input_list.iter().flat_map(|x| encode(x)).collect();
            let len = payload.len();
            let len_bytes = len.ilog(256) as u8 + 1;
            let tag = LIST_OFFSET + UNTAGGED_SIZE_LIMIT as u8 + len_bytes;

            let input = RLPItem::List(input_list);
            let expect= vec![tag]
                .into_iter()
                .chain(usize_to_min_be_bytes(len))
                .chain(payload)
                .collect();
            encode_then_decode(input, expect);
        }

        #[test]
        fn rlp_size(data in vec(any::<u8>(), 0..20)) {
            let data_size = data.len();
            let calc_size = RLPItem::ByteArray(data).size();

            prop_assert_eq!(calc_size, data_size);
        }

        #[test]
        fn rlp_size_list(data in vec(vec(any::<u8>(), 0..5), 0..5)) {
            let data_size = data.iter().map(|v| v.len()).sum();
            let rlps = data.into_iter().map(RLPItem::ByteArray).collect();
            let calc_size = RLPItem::List(rlps).size();

            prop_assert_eq!(calc_size, data_size);
        }

        #[test]
        fn flatten_size(rlp: RLPItem) {
            let size = rlp.size();
            let flat = rlp.flatten();
            prop_assert_eq!(size, flat.len());
        }
    }

    #[test]
    fn zero_bytes() {
        let input = RLPItem::ByteArray(vec![]);
        let expect = vec![BYTE_ARRAY_OFFSET];
        encode_then_decode(input, expect);
    }

    #[test]
    fn zero_bytes_list() {
        let input = RLPItem::List(vec![]);
        let expect = vec![LIST_OFFSET];
        encode_then_decode(input, expect);
    }

    #[test]
    fn gal_size_encoding_list() {
        let len = 56;
        let len_bytes = 1;
        let tag = LIST_OFFSET + UNTAGGED_SIZE_LIMIT as u8 + len_bytes;
        let input_nums = vec![42; len];
        let input_bytes: Vec<u8> = input_nums.iter().map(|x| *x as u8).collect();

        let input: Bytes = vec![tag + 1]
            .into_iter()
            .chain(vec![0])
            .chain(usize_to_min_be_bytes(len))
            .chain(input_bytes)
            .collect();
        assert_eq!(decode(&input), Err(DecodingErr::LeadingZerosInSize));
    }

    #[test]
    fn gal_size_encoding_byte_array() {
        let len = 256;
        let len_bytes = 2;
        let tag = BYTE_ARRAY_OFFSET + UNTAGGED_SIZE_LIMIT as u8 + len_bytes;
        let input_bytes = vec![42; len];
        let input: Bytes = vec![tag + 1]
            .into_iter()
            .chain(vec![0])
            .chain(usize_to_min_be_bytes(len))
            .chain(input_bytes)
            .collect();
        assert_eq!(decode(&input), Err(DecodingErr::LeadingZerosInSize));
    }
}
