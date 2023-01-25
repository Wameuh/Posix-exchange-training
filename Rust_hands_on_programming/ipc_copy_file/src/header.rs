
use std::mem::size_of;


pub struct Header {
    data_: Vec<u8>,
    default_buffer_size_: usize,
}


impl  Header {
    pub fn new(key_option: Option<u64>, filesize_option: Option<usize>, default_buffer_size: usize) -> Header {
        
        let mut data: Vec<u8> = Vec::new();

        if let Some(key) = key_option {
            data.extend_from_slice(&key.clone().to_be_bytes());
        }

        if let Some(filesize) = filesize_option {
            data.extend_from_slice(&filesize.to_be_bytes());
        }

        data.resize(default_buffer_size, 0);

        return Header {data_: data, default_buffer_size_: default_buffer_size};
    }

    pub fn extract_key(&self, extern_data: &Vec<u8>) -> Option<u64> {
        if extern_data.len() < self.default_buffer_size_|| (size_of::<u64>() > self.default_buffer_size_)  {
            return None;
        }

        //We can unwrap the try_into() because the only case that try_from will return error is when the len is incorrect, but the slice resolve this.
        Some(u64::from_be_bytes(extern_data[0..size_of::<u64>()].try_into().unwrap()))
    }

    pub fn extract_file_size(&self, extern_data: &Vec<u8>) -> Option<usize> {
        if (extern_data.len() < self.default_buffer_size_ )|| extern_data.len() < size_of::<u64>()+size_of::<usize>() {
            return None;
        }
        //We can unwrap the try_into() because the only case that try_from will return error is when the len is incorrect, but the slice resolve this.
        Some(usize::from_be_bytes(extern_data[size_of::<u64>()..size_of::<u64>()+size_of::<usize>()].try_into().unwrap()))
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data_
    }
}



#[cfg(test)]
mod header_tests;