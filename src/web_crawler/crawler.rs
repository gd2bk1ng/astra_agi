// =============================================================================
//  Astra AGI - Web Crawler
//  File: crawler.rs
//
//  Description:
//      Implements focused real-time web crawling with rate limiting and politeness.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-26
//
//  License:
//      Dual licensed under the MIT and Apache 2.0 licenses.
//      See LICENSE-MIT and LICENSE-APACHE at the repository root for details.
// =============================================================================

use reqwest::Client;
use tokio::time::{sleep, Duration};
use anyhow::{Result, Context};
use url::Url;
use std::collections::{HashSet, VecDeque};

pub struct WebCrawler {
    client: Client,
    visited: HashSet<String>,
    queue: VecDeque<String>,
    rate_limit_ms: u64,
}

impl WebCrawler {
    pub fn new(rate_limit_ms: u64) -> Self {
        Self {
            client: Client::new(),
            visited: HashSet::new(),
            queue: VecDeque::new(),
            rate_limit_ms,
        }
    }

    pub fn enqueue(&mut self, url: &str) {
        if !self.visited.contains(url) {
            self.queue.push_back(url.to_string());
        }
    }

    pub async fn crawl_next(&mut self) -> Result<Option<String>> {
        if let Some(url) = self.queue.pop_front() {
            if self.visited.contains(&url) {
                return Ok(None);
            }
            let resp = self.client.get(&url).send().await.context("Failed to fetch URL")?;
            let body = resp.text().await.context("Failed to read response body")?;
            self.visited.insert(url.clone());

            // Respect rate limit
            sleep(Duration::from_millis(self.rate_limit_ms)).await;

            Ok(Some(body))
        } else {
            Ok(None)
        }
    }

    /// Example focused crawl starting from seed URLs
    pub async fn focused_crawl(&mut self, seeds: &[&str], max_pages: usize) -> Result<()> {
        for &seed in seeds {
            self.enqueue(seed);
        }

        let mut pages_crawled = 0;

        while pages_crawled < max_pages {
            match self.crawl_next().await? {
                Some(content) => {
                    pages_crawled += 1;
                    // TODO: Pass content to ingestion pipeline
                    println!("Crawled page #{} with {} chars", pages_crawled, content.len());
                }
                None => break,
            }
        }

        Ok(())
    }
}
