use crate::{
    solar_month::SolarMonthRefHelper,
    solar_season::{SolarSeason, SolarSeasonRefHelper},
};

#[test]
fn solar_season_test() {
    let season = SolarSeason::from_ym(2019, 5);
    assert_eq!("2019.2", season.to_string());
    assert_eq!("2019年2季度", season.to_full_string());
    assert_eq!("2019.3", season.next(1).to_string());
    assert_eq!("2019年3季度", season.next(1).to_full_string());
}

#[test]
fn solar_season_get_months_test() {
    let season = SolarSeason::from_ym(2019, 5);
    let months = season.get_months();
    assert_eq!(3, months.len());
    assert_eq!("2019-4", months[0].to_string());
    assert_eq!("2019-5", months[1].to_string());
    assert_eq!("2019-6", months[2].to_string());
}
