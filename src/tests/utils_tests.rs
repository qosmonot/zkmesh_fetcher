use crate::utils::parse_rss_date;
use crate::config::read_blogs_from_file;
use crate::feed_types::FeedType;
use chrono::NaiveDate;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_read_blogs_from_file() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let path = temp_file.path().to_str().expect("Failed to get path as string");
    
    let mut file = File::create(path).expect("Failed to create file");
    writeln!(file, "TestBlog|https://test.com|Substack").expect("Failed to write TestBlog to file");
    writeln!(file, "AnotherBlog|https://another.com|RSS").expect("Failed to write AnotherBlog to file");
    writeln!(file, "AlsoThisBlog|https://alsothis.com|Atom").expect("Failed to write AlsoThisBlog to file");
    
    let blogs = read_blogs_from_file(path).unwrap();
    
    assert_eq!(blogs.len(), 3);
    assert_eq!(blogs[0].name, "TestBlog");
    assert_eq!(blogs[0].domain, "https://test.com");
    assert_eq!(blogs[0].feed_type, FeedType::Substack);
    assert_eq!(blogs[1].name, "AnotherBlog");
    assert_eq!(blogs[1].domain, "https://another.com");
    assert_eq!(blogs[1].feed_type, FeedType::RSS);
    assert_eq!(blogs[2].name, "AlsoThisBlog");
    assert_eq!(blogs[2].domain, "https://alsothis.com");
    assert_eq!(blogs[2].feed_type, FeedType::Atom);    
}

#[test]
fn test_parse_rss_date() {
    let date1 = "Tue, 01 Oct 2024 12:00:00 GMT";
    let date2 = "2024-10-01T12:00:00+00:00";
    let date3 = "2024-10-01";
    
    assert_eq!(
        parse_rss_date(date1).expect("Failed to parse date1"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    assert_eq!(
        parse_rss_date(date2).expect("Failed to parse date2"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );assert_eq!(
        parse_rss_date(date3).expect("Failed to parse date3"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    
    assert!(parse_rss_date("Invalid Date").is_err());
}