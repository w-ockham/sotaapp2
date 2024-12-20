use chrono::{DateTime, Utc};
use derive_new::new;
use std::time::Duration;

use crate::model::AwardProgram;

#[derive(new, Debug)]
pub struct BoundingBox {
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
}

// 中心位置と中心位置からの距離を指定するための構造体
#[derive(new, Debug)]
pub struct CenterRadius {
    pub lon: f64,
    pub lat: f64,
    pub rad: f64,
}

impl<T> From<Vec<T>> for CreateRef<T> {
    fn from(requests: Vec<T>) -> Self {
        Self { requests }
    }
}

#[derive(Default, Debug)]
pub struct FindRef {
    pub program: Vec<AwardProgram>,
    pub ref_id: Option<String>,
    pub name: Option<String>,
    pub bbox: Option<BoundingBox>,
    pub center: Option<CenterRadius>,
    pub min_elev: Option<i32>,
    pub min_area: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl FindRef {
    pub fn is_sota(&self) -> bool {
        self.program.contains(&AwardProgram::SOTA)
    }

    pub fn is_pota(&self) -> bool {
        self.program.contains(&AwardProgram::POTA)
    }

    pub fn is_wwff(&self) -> bool {
        self.program.contains(&AwardProgram::WWFF)
    }
}

pub struct FindRefBuilder {
    pub param: FindRef,
}

impl Default for FindRefBuilder {
    fn default() -> Self {
        let param = FindRef {
            program: Vec::<AwardProgram>::new(),
            ..Default::default()
        };
        Self { param }
    }
}

impl FindRefBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn program(mut self, program: AwardProgram) -> Self {
        self.param.program.push(program);
        self
    }

    pub fn sota(mut self) -> Self {
        self.param.program.push(AwardProgram::SOTA);
        self
    }

    pub fn pota(mut self) -> Self {
        self.param.program.push(AwardProgram::POTA);
        self
    }

    pub fn wwff(mut self) -> Self {
        self.param.program.push(AwardProgram::WWFF);
        self
    }

    pub fn ref_id(mut self, id: String) -> Self {
        self.param.ref_id = Some(id.to_uppercase());
        self
    }

    pub fn name(mut self, n: String) -> Self {
        self.param.name = Some(n);
        self
    }

    pub fn bbox(mut self, min_lon: f64, min_lat: f64, max_lon: f64, max_lat: f64) -> Self {
        let bbox = BoundingBox::new(min_lon, min_lat, max_lon, max_lat);
        self.param.bbox = Some(bbox);
        self
    }

    pub fn center(mut self, lon: f64, lat: f64, radius: f64) -> Self {
        let cr = CenterRadius::new(lon, lat, radius);
        self.param.center = Some(cr);
        self
    }

    pub fn min_elev(mut self, elev: i32) -> Self {
        self.param.min_elev = Some(elev);
        self
    }

    pub fn min_area(mut self, area: i32) -> Self {
        self.param.min_area = Some(area);
        self
    }

    pub fn limit(mut self, l: i32) -> Self {
        self.param.limit = Some(l);
        self
    }

    pub fn offset(mut self, o: i32) -> Self {
        self.param.offset = Some(o);
        self
    }

    pub fn build(self) -> FindRef {
        self.param
    }
}

pub struct CreateRef<T> {
    pub requests: Vec<T>,
}

pub struct UpdateRef<T> {
    pub requests: Vec<T>,
}

pub struct FindResult<T> {
    pub counts: usize,
    pub results: Option<Vec<T>>,
}

impl<T> FindResult<T> {
    pub fn get_reference(self) -> Option<T> {
        self.results.and_then(|mut r| r.pop())
    }
}

pub struct FindAppResult<SOTA, POTA> {
    pub sota: Option<FindResult<SOTA>>,
    pub pota: Option<FindResult<POTA>>,
}
#[derive(Debug)]
pub enum DeleteRef<T> {
    Delete(T),
    DeleteAll,
}

pub struct UpdateAct<T> {
    pub requests: Vec<T>,
}

impl<T> From<Vec<T>> for UpdateAct<T> {
    fn from(requests: Vec<T>) -> Self {
        Self { requests }
    }
}

#[derive(Default, Debug)]
pub struct FindAct {
    pub after: Option<DateTime<Utc>>,
    pub before: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
}

#[derive(Default)]
pub struct FindActBuilder {
    pub param: FindAct,
}

impl FindActBuilder {
    pub fn after(mut self, aft: DateTime<Utc>) -> Self {
        self.param.after = Some(aft);
        self
    }

    pub fn before(mut self, bfr: DateTime<Utc>) -> Self {
        self.param.before = Some(bfr);
        self
    }

    pub fn duration(mut self, drt: Duration) -> Self {
        self.param.duration = Some(drt);
        self
    }

    pub fn build(self) -> FindAct {
        self.param
    }
}
#[derive(Debug)]
pub struct DeleteAct {
    pub before: DateTime<Utc>,
}
