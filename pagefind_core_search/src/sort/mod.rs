//! Sorting functionality for search results

use std::cmp::Ordering;
use std::collections::HashMap;

/// Options for sorting search results
#[derive(Debug, Clone)]
pub struct SortOptions {
    /// List of sort criteria in order of priority
    pub criteria: Vec<SortCriterion>,
}

impl SortOptions {
    /// Create new sort options with a single criterion
    pub fn new(field: String, order: SortOrder) -> Self {
        Self {
            criteria: vec![SortCriterion { field, order }],
        }
    }

    /// Add an additional sort criterion
    pub fn add_criterion(&mut self, field: String, order: SortOrder) {
        self.criteria.push(SortCriterion { field, order });
    }

    /// Compare two items based on the sort criteria
    pub fn compare(&self, a_meta: &HashMap<String, String>, b_meta: &HashMap<String, String>) -> Ordering {
        for criterion in &self.criteria {
            let a_value = a_meta.get(&criterion.field);
            let b_value = b_meta.get(&criterion.field);

            let comparison = match (a_value, b_value) {
                (Some(a), Some(b)) => self.compare_values(a, b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            };

            if comparison != Ordering::Equal {
                return match criterion.order {
                    SortOrder::Asc => comparison,
                    SortOrder::Desc => comparison.reverse(),
                };
            }
        }

        Ordering::Equal
    }

    fn compare_values(&self, a: &str, b: &str) -> Ordering {
        // Try to parse as numbers first
        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
            a_num.partial_cmp(&b_num).unwrap_or(Ordering::Equal)
        } else {
            // Fall back to string comparison
            a.cmp(b)
        }
    }
}

impl Default for SortOptions {
    fn default() -> Self {
        Self {
            criteria: vec![],
        }
    }
}

/// A single sort criterion
#[derive(Debug, Clone)]
pub struct SortCriterion {
    /// The field to sort by
    pub field: String,
    /// The sort order
    pub order: SortOrder,
}

/// Sort order direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    /// Ascending order (A-Z, 0-9)
    Asc,
    /// Descending order (Z-A, 9-0)
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Asc
    }
}

impl From<&str> for SortOrder {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "desc" | "descending" => SortOrder::Desc,
            _ => SortOrder::Asc,
        }
    }
}

/// Errors that can occur during sort operations
#[derive(Debug)]
pub enum SortError {
    InvalidField(String),
    InvalidOrder(String),
}

impl std::fmt::Display for SortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortError::InvalidField(field) => write!(f, "Invalid sort field: {}", field),
            SortError::InvalidOrder(order) => write!(f, "Invalid sort order: {}", order),
        }
    }
}

impl std::error::Error for SortError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_options_creation() {
        let sort = SortOptions::new("title".to_string(), SortOrder::Asc);
        assert_eq!(sort.criteria.len(), 1);
        assert_eq!(sort.criteria[0].field, "title");
        assert_eq!(sort.criteria[0].order, SortOrder::Asc);
    }

    #[test]
    fn test_sort_order_from_str() {
        assert_eq!(SortOrder::from("asc"), SortOrder::Asc);
        assert_eq!(SortOrder::from("desc"), SortOrder::Desc);
        assert_eq!(SortOrder::from("descending"), SortOrder::Desc);
        assert_eq!(SortOrder::from("invalid"), SortOrder::Asc);
    }

    #[test]
    fn test_value_comparison() {
        let sort = SortOptions::default();
        
        let mut a_meta = HashMap::new();
        let mut b_meta = HashMap::new();
        
        // Test numeric comparison
        a_meta.insert("score".to_string(), "10".to_string());
        b_meta.insert("score".to_string(), "5".to_string());
        assert_eq!(sort.compare_values("10", "5"), Ordering::Greater);
        
        // Test string comparison
        assert_eq!(sort.compare_values("apple", "banana"), Ordering::Less);
    }
}