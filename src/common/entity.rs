use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts, Query};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::error::CommonError;

pub struct Pokemon {}

pub trait HasService {
    const SERVICE: Service;
}

#[derive(Deserialize, Clone)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 16,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct PaginationNavigation {
    pub items: Vec<PaginationNavigationItem>,
}

#[derive(Deserialize, Clone)]
pub struct PaginationNavigationItem {
    pub hide: bool,
    pub page: usize,
    pub is_current: bool,
}

impl Pagination {
    pub fn offset(&self) -> usize {
        self.page * self.page_size
    }
    pub fn limit(&self) -> usize {
        self.page_size
    }
    pub fn get_total_pages(&self, count: usize) -> usize {
        (count as f64 / self.page_size as f64).ceil() as usize
    }
    pub fn get_navigation(
        &self,
        total_pages: usize,
        num_pages_to_show: usize,
    ) -> PaginationNavigation {
        let navigation_pages = self.get_navigation_pages(total_pages, num_pages_to_show);
        let current_page = self.page;

        let mut items: Vec<PaginationNavigationItem> = navigation_pages
            .into_iter()
            .map(|page| PaginationNavigationItem {
                hide: false,
                page,
                is_current: page == current_page,
            })
            .collect();

        // modify the first and last items to hide if they do not connect to the first and last page
        if let Some(first) = items.first_mut() {
            if (first.page.saturating_sub(1) > 0) && (!first.is_current) {
                first.hide = true;
            }
        }
        let final_page = total_pages.saturating_sub(1);
        if let Some(last) = items.last_mut() {
            if (last.page + 1 < final_page) && (!last.is_current) {
                last.hide = true;
            }
        }

        // add the first and last page if they are not already in the list
        if items.first().map(|i| i.page) != Some(0) {
            items.insert(
                0,
                PaginationNavigationItem {
                    hide: false,
                    page: 0,
                    is_current: false,
                },
            );
        }
        if items.last().map(|i| i.page) != Some(final_page) {
            items.push(PaginationNavigationItem {
                hide: false,
                page: final_page,
                is_current: false,
            });
        }

        PaginationNavigation { items }
    }

    pub fn get_navigation_pages(&self, total_pages: usize, num_pages_to_show: usize) -> Vec<usize> {
        let num_side_pages = (num_pages_to_show - 1) / 2;
        let mut page_start = self.page.saturating_sub(num_side_pages);
        let final_page = total_pages.saturating_sub(1);
        let mut page_end = (self.page + num_side_pages).min(final_page);

        // if there are not enough pages on the right side, show more on the left side
        // if there are not enough pages on the left side, show more on the right side
        // if there are not enough pages on both sides, show all pages
        let mut len_pages = page_end.saturating_sub(page_start) + 1;
        while len_pages < num_pages_to_show && (page_start > 0 || page_end < final_page) {
            if page_start > 0 {
                page_start = page_start.saturating_sub(1);
            }
            if page_end < final_page {
                page_end += 1;
            }
            len_pages = page_end.saturating_sub(page_start) + 1;
        }

        let mut pages = vec![];
        for i in page_start..=page_end {
            pages.push(i);
        }
        pages
    }
}

// `#[derive(FromRef)]` makes them sub states so they can be extracted independently
#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
// sql type
#[sqlx(type_name = "service")]
pub enum Service {
    #[serde(rename = "pokemon")]
    #[sqlx(rename = "pokemon")]
    Pokemon,
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Service::Pokemon => write!(f, "pokemon"),
        }
    }
}

impl std::str::FromStr for Service {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pokemon" => Ok(Service::Pokemon),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryName {
    pub name: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for QueryName
where
    S: Send + Sync,
{
    type Rejection = CommonError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<QueryName>::from_request_parts(parts, state)
            .await
            .map_err(|_| CommonError::ValidationError("Cannot get query param".into()))?;

        let name = query.name.clone();
        if name.len() < 2 {
            return Err(CommonError::ValidationError(
                "Name must be at least 2 characters long".into(),
            ));
        }
        Ok(Self { name })
    }
}

#[cfg(test)]
mod test_pagination {
    use super::*;

    #[test]
    fn check_one_page() {
        let pagination = Pagination {
            page: 0,
            page_size: 20,
        };
        let pages = pagination.get_navigation_pages(1, 7);
        assert_eq!(pages, vec![0]);
    }

    #[test]
    fn check_two_pages() {
        let pagination = Pagination {
            page: 1,
            page_size: 20,
        };
        let pages = pagination.get_navigation_pages(2, 7);
        assert_eq!(pages, vec![0, 1]);
    }

    #[test]
    fn check_insufficient_pages() {
        let pagination = Pagination {
            page: 2,
            page_size: 20,
        };
        let pages = pagination.get_navigation_pages(3, 7);
        assert_eq!(pages, vec![0, 1, 2]);
    }

    #[test]
    fn check_many_pages() {
        let pagination = Pagination {
            page: 10,
            page_size: 20,
        };
        let pages = pagination.get_navigation_pages(20, 7);
        assert_eq!(pages, vec![7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn check_many_pages_right_skewed() {
        let pagination = Pagination {
            page: 2,
            page_size: 20,
        };
        let pages = pagination.get_navigation_pages(20, 7);
        assert_eq!(pages, vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn check_many_pages_left_skewed() {
        let pagination = Pagination {
            page: 19,
            page_size: 20,
        };
        let pages = pagination.get_navigation_pages(20, 7);
        assert_eq!(pages, vec![13, 14, 15, 16, 17, 18, 19]);
    }

    #[test]
    fn check_changed_num_pages_to_show() {
        let mut pagination = Pagination {
            page: 10,
            page_size: 20,
        };

        pagination.page = 10;
        let pages = pagination.get_navigation_pages(20, 9);
        assert_eq!(pages, vec![6, 7, 8, 9, 10, 11, 12, 13, 14]);

        pagination.page = 2;
        let pages = pagination.get_navigation_pages(20, 9);
        assert_eq!(pages, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        pagination.page = 18;
        let pages = pagination.get_navigation_pages(20, 9);
        assert_eq!(pages, vec![11, 12, 13, 14, 15, 16, 17, 18, 19]);
    }
}
