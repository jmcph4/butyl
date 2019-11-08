use crate::errors;
use crate::formats;
use crate::dos;
use crate::coff;

pub fn infer_format(data: &[u8]) -> formats::Format {
   unimplemented!(); 
}

pub fn get_file_as(data: &[u8], format: formats::Format) ->
    Result<formats::File, errors::ButylError> {
    Ok(match format {
        DOS => formats::File::DOS(dos::DosFile::from_le_bytes(data)?),
        COFF => formats::File::COFF(coff::CoffFile::from_le_bytes(data)?),
        _ => formats::File::Unknown(())
    })
}

