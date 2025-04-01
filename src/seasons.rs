use serde::{Deserialize, Serialize};

use crate::{JikanClient, JikanError, anime::*, common::Pagination};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonsListResponse {
    pub data: Vec<SeasonInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfo {
    pub year: u32,
    pub seasons: Vec<String>,
}

pub enum FilterType {
    None,
    TV,
    Movie,
    OVA,
    Special,
    ONA,
    Music,
}

/// Configuration options for season queries
#[derive(Default)]
pub struct SeasonQueryParams {
    filter: Option<FilterType>,
    sfw: Option<bool>,
    unapproved: Option<bool>,
    continuing: Option<bool>,
    page: Option<u32>,
    limit: Option<u32>,
}

impl SeasonQueryParams {
    /// Create a new empty SeasonQueryParams
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the filter type for the query
    pub fn filter(mut self, filter: FilterType) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set the SFW (Safe For Work) filter
    pub fn sfw(mut self, sfw: bool) -> Self {
        self.sfw = Some(sfw);
        self
    }

    /// Set the unapproved filter
    pub fn unapproved(mut self, unapproved: bool) -> Self {
        self.unapproved = Some(unapproved);
        self
    }

    /// Set the continuing filter
    pub fn continuing(mut self, continuing: bool) -> Self {
        self.continuing = Some(continuing);
        self
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the limit of results per page
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Internal method to convert parameters to query string
    fn to_query_params(&self) -> String {
        let mut params = Vec::new();

        if let Some(f) = &self.filter {
            let filter_str = match f {
                FilterType::None => String::new(),
                FilterType::TV => "tv".to_string(),
                FilterType::Movie => "movie".to_string(),
                FilterType::OVA => "ova".to_string(),
                FilterType::Special => "special".to_string(),
                FilterType::ONA => "ona".to_string(),
                FilterType::Music => "music".to_string(),
            };
            if !filter_str.is_empty() {
                params.push(format!("filter={}", filter_str));
            }
        }

        if let Some(s) = self.sfw {
            if s {
                params.push("sfw".to_string());
            }
        }

        if let Some(u) = self.unapproved {
            if u {
                params.push("unapproved".to_string());
            }
        }

        if let Some(c) = self.continuing {
            if c {
                params.push("continuing".to_string());
            }
        }

        if let Some(p) = self.page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = self.limit {
            params.push(format!("limit={}", l));
        }

        if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        }
    }
}

impl JikanClient {
    /// Returns anime currently airing in the current season
    pub async fn get_season_now(
        &self,
        params: Option<SeasonQueryParams>,
    ) -> Result<SeasonResponse, JikanError> {
        let query = params.map(|p| p.to_query_params()).unwrap_or_default();
        self.get(&format!("/seasons/now{}", query)).await
    }

    /// Returns anime for the specified season
    pub async fn get_season(
        &self,
        year: u32,
        season: &str,
        params: Option<SeasonQueryParams>,
    ) -> Result<SeasonResponse, JikanError> {
        let query = params.map(|p| p.to_query_params()).unwrap_or_default();
        self.get(&format!("/seasons/{}/{}{}", year, season, query))
            .await
    }

    /// Returns the list of available seasons
    pub async fn get_seasons_list(&self) -> Result<SeasonsListResponse, JikanError> {
        self.get("/seasons").await
    }

    /// Returns anime airing in the upcoming season
    pub async fn get_season_upcoming(
        &self,
        params: Option<SeasonQueryParams>,
    ) -> Result<SeasonResponse, JikanError> {
        let query = params.map(|p| p.to_query_params()).unwrap_or_default();
        self.get(&format!("/seasons/upcoming{}", query)).await
    }
}
