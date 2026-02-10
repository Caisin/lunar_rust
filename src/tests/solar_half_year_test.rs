use crate::{
    solar_half_year::{SolarHalfYear, SolarHalfYearRefHelper},
    solar_month::SolarMonthRefHelper,
};

#[test]
fn solar_half_year_test() {
    let half_year = SolarHalfYear::from_ym(2019, 5);
    assert_eq!("2019.1", half_year.to_string());
    assert_eq!("2019年上半年", half_year.to_full_string());
    assert_eq!("2019.2", half_year.next(1).to_string());
    assert_eq!("2019年下半年", half_year.next(1).to_full_string());
}

#[test]
fn solar_half_year_get_months_test() {
    let half_year = SolarHalfYear::from_ym(2019, 5);
    let months = half_year.get_months();
    assert_eq!(6, months.len());
    assert_eq!("2019-1", months[0].to_string());
    assert_eq!("2019-6", months[5].to_string());
}
