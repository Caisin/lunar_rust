use crate::{
    solar_month::{SolarMonth, SolarMonthRef, SolarMonthRefHelper},
    util::locked_ref_trait::LockRef,
};
use chrono::{Datelike, NaiveDate};
use std::sync::{Arc, Mutex, MutexGuard};

pub struct SolarHalfYear {
    __year: i64,
    __month: i64,
}

pub type SolarHalfYearRef = Arc<Mutex<SolarHalfYear>>;

pub trait SolarHalfYearRefHelper: LockRef {
    fn get_year(&self) -> i64;
    fn get_month(&self) -> i64;
    fn to_string(&self) -> String;
    fn to_full_string(&self) -> String;
    fn get_index(&self) -> i64;
    fn get_months(&self) -> Vec<SolarMonthRef>;
    fn next(&self, half_years: i64) -> SolarHalfYearRef;
}

impl SolarHalfYearRefHelper for SolarHalfYearRef {
    fn get_year(&self) -> i64 {
        self.as_locked_ref().__year
    }

    fn get_month(&self) -> i64 {
        self.as_locked_ref().__month
    }

    fn to_string(&self) -> String {
        format!("{}.{}", self.get_year(), self.get_index())
    }

    fn to_full_string(&self) -> String {
        format!(
            "{}年{}半年",
            self.get_year(),
            if self.get_index() == 1 { "上" } else { "下" }
        )
    }

    fn get_index(&self) -> i64 {
        (self.get_month() as f64 / MONTH_COUNT as f64).ceil() as i64
    }

    fn get_months(&self) -> Vec<SolarMonthRef> {
        let mut months = vec![];
        let index = self.get_index() - 1;
        for i in 0..MONTH_COUNT {
            months.push(SolarMonth::from_ym(
                self.get_year(),
                MONTH_COUNT * index + i + 1,
            ));
        }
        months
    }

    fn next(&self, half_years: i64) -> SolarHalfYearRef {
        let m =
            SolarMonth::from_ym(self.get_year(), self.get_month()).next(MONTH_COUNT * half_years);
        SolarHalfYear::from_ym(m.get_year(), m.get_month())
    }
}

impl LockRef for SolarHalfYearRef {
    type Output<'a>
        = MutexGuard<'a, SolarHalfYear>
    where
        Self: 'a;
    fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
        self.lock().unwrap()
    }
}

impl SolarHalfYear {
    fn __init(year: i64, month: i64) -> SolarHalfYearRef {
        Arc::new(Mutex::new(Self {
            __year: year,
            __month: month,
        }))
    }

    pub fn from_date(date: NaiveDate) -> SolarHalfYearRef {
        Self::__init(date.year() as i64, date.month() as i64)
    }

    pub fn from_ym(year: i64, month: i64) -> SolarHalfYearRef {
        Self::__init(year, month)
    }
}

const MONTH_COUNT: i64 = 6;
