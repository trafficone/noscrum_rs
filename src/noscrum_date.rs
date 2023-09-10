use chrono::naive::NaiveDate;
use dynomite::{Attribute, AttributeValue, AttributeError};
use serde::{Serialize,Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoScrumDate(NaiveDate);
impl NoScrumDate {
    /// Create a new NoScrumDate (chrono::naive::NaiveDate) given a year, month and day value.
    /// In general, it is preferred to use the into_attr() and from_attr() for NoScrumDate SerDe.
    /// 
    /// # Errors
    ///
    /// This function will return an error if NaiveDate::from_ymd_opt(...) 
    /// returns None.
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self,AttributeError> {
        if let Some(date) = NaiveDate::from_ymd_opt(year,month,day).map(NoScrumDate) {
            Ok(date) 
        } else {
            Err(AttributeError::InvalidFormat)
        }
   }
}

impl Attribute for NoScrumDate {
    fn into_attr(self) -> AttributeValue {
       AttributeValue{
        s: Some(self.0.format("%Y-%m-%d").to_string()),
        ..AttributeValue::default()
       }
    }

    fn from_attr(value: AttributeValue) -> Result<Self, AttributeError> {
        if let Some(date_string) = value.s {
            match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
                Ok(date_val) => Ok(NoScrumDate(date_val)),
                Err(_) => Err(AttributeError::InvalidFormat)
            }
        } else {
            Err(AttributeError::InvalidType)
        }
    }
}

#[cfg(test)]
mod tests {
    use dynomite::{Attribute,AttributeValue};
    use crate::noscrum_date::NoScrumDate;

    #[test]
    fn test_date_from_attr(){
        let mut test_value = AttributeValue{s: Some("2020-03-05".to_owned()),..Default::default()};
        assert_eq!(NoScrumDate::from_attr(test_value), NoScrumDate::new(2020,3,5));
        test_value = AttributeValue{s: Some("1935-01-01".to_owned()),..Default::default()};
        assert_eq!(NoScrumDate::from_attr(test_value), NoScrumDate::new(1935,1,1));
        test_value = AttributeValue{s: Some("0-01-01".to_owned()),..Default::default()};
        assert_eq!(NoScrumDate::from_attr(test_value), NoScrumDate::new(0,1,1));
        test_value = AttributeValue{s: Some("2525-02-28".to_owned()),..Default::default()};
        assert_eq!(NoScrumDate::from_attr(test_value), NoScrumDate::new(2525,2,28));
    }

    #[test]
    fn test_attr_from_date(){
        if let Ok(date) = NoScrumDate::new(1997,5,13) {
            assert_eq!(date.into_attr().s, Some("1997,5,13".to_owned()));
        } else {
            assert_eq!("","Date Failed to Parse");
        }
    }
}