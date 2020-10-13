use std::collections::HashSet;
use std::convert::TryFrom;

use super::context::Context;
use super::error::Error;
use super::percent_encoded_character_decoder::PercentEncodedCharacterDecoder;

pub fn decode_element<T>(
    element: T,
    allowed_characters: &'static HashSet<char>,
    context: Context
) -> Result<Vec<u8>, Error>
    where T: AsRef<str>
{
    let mut decoding_pec = false;
    let mut pec_decoder = PercentEncodedCharacterDecoder::new();
    element
        .as_ref()
        .chars()
        .filter_map(|c| {
            if decoding_pec {
                pec_decoder
                    .next(c)
                    .map_err(Into::into)
                    .transpose()
                    .map(|c| {
                        decoding_pec = false;
                        c
                    })
            } else if c == '%' {
                decoding_pec = true;
                None
            } else if allowed_characters.contains(&c) {
                Some(Ok(c as u8))
            } else {
                Some(Err(Error::IllegalCharacter(context)))
            }
        })
        .collect()
}

pub fn encode_element(
    element: &[u8],
    allowed_characters: &HashSet<char>
) -> String {
    element.iter()
        .map(|ci| {
            match char::try_from(*ci) {
                Ok(c) if allowed_characters.contains(&c) => c.to_string(),
                _ => format!("%{:X}", ci),
            }
        })
        .collect()
}
