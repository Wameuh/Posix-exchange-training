use super::*;

const DEFAULT_BUFFER_SIZE: usize = 4096;
#[test]
fn new_without_filesize() {
    let key:u64= std::u64::MAX - String::from("1".to_owned() + env!("CARGO_PKG_VERSION")).split(".").collect::<Vec<&str>>().join("").parse::<u64>().unwrap(); 

    let my_header = Header::new(Some(key), None, DEFAULT_BUFFER_SIZE);

    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&key.clone().to_be_bytes());
    data.resize(DEFAULT_BUFFER_SIZE, 0);

    assert_eq!(my_header.data_.len(), DEFAULT_BUFFER_SIZE);
    assert!(my_header.extract_key(&data).is_some());
    assert_eq!(my_header.extract_key(&data).unwrap(), key);
    assert!(my_header.extract_file_size(&data).is_some());
    assert_eq!(my_header.extract_file_size(&data).unwrap(), 0);
}

#[test]
fn new_with_filesize() {
    { //random filesize

        let key = 5199;
        let filesize: usize = rand::random::<usize>();

        let my_header = Header::new(Some(key), Some(filesize.clone()), DEFAULT_BUFFER_SIZE);
        assert_eq!(my_header.data_.len(), DEFAULT_BUFFER_SIZE);
        assert!(my_header.extract_key(&my_header.data_).is_some());
        assert_eq!(my_header.extract_key(&my_header.data_).unwrap(), key);
        assert!(my_header.extract_file_size(&my_header.data_).is_some());
        assert_eq!(my_header.extract_file_size(&my_header.data_).unwrap(), filesize);

    }
    { //max filesize
        let key = 0;

        let my_header = Header::new(Some(key), Some(std::usize::MAX), DEFAULT_BUFFER_SIZE);
        assert_eq!(my_header.data_.len(), DEFAULT_BUFFER_SIZE);
        assert!(my_header.extract_key(&my_header.data_).is_some());
        assert_eq!(my_header.extract_key(&my_header.data_).unwrap(), key);
        assert!(my_header.extract_file_size(&my_header.data_).is_some());
        assert_eq!(my_header.extract_file_size(&my_header.data_).unwrap(), std::usize::MAX);

    }
}


#[test]
fn data_len_issue() {
    { //data.len() < default_buffer_size

        let filesize: usize = rand::random::<usize>();

        let mut my_header = Header::new(Some(215619549), Some(filesize.clone()), DEFAULT_BUFFER_SIZE);
        my_header.data_.resize(2, 0);
        assert!(my_header.extract_key(&my_header.data_).is_none());
        assert!(my_header.extract_file_size(&my_header.data_).is_none());

    }
    { //default_buffer_size too small

        let filesize: usize = rand::random::<usize>();

        let my_header = Header::new(Some(564656), Some(filesize.clone()), 2);
        assert!(my_header.extract_key(&my_header.data_).is_none());
        assert!(my_header.extract_key(&my_header.data_).is_none());

    }
}

#[test]
fn get_data_test() {
    let key = 5199;
    let filesize: usize = rand::random::<usize>();

    let my_header = Header::new(Some(key), Some(filesize.clone()), DEFAULT_BUFFER_SIZE);
    

    let my_second_header = Header::new(None,None,DEFAULT_BUFFER_SIZE);

    assert_eq!(my_second_header.extract_key(my_header.get_data()), Some(key));
    assert_eq!(my_second_header.extract_file_size(my_header.get_data()), Some(filesize));
}