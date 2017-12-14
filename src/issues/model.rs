//! Module with the data types used by issue producer.

use std::fmt;

use url::{Url, Host};


const GITHUB_HOST: &'static str = "github.com";


#[derive(Debug)]
pub struct Issue {
    /// GitHub repository where this issue comes from.
    pub repo: Repository,
    /// Issue number.
    pub number: usize,
    /// Issue title.
    pub title: String,
    // TODO: more interesting issue data
}

impl fmt::Display for Issue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}] #{}: {}", self.repo, self.number, self.title)
    }
}



/// Represents a GitHub repository.
#[derive(Debug)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

impl Repository {
    #[inline]
    pub fn new<O: ToString, N: ToString>(owner: O, name: N) -> Self {
        Repository {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }

    pub fn from_url(repo_url: &str) -> Option<Self> {
        let parsed = Url::parse(repo_url).ok()?;
        if parsed.host() == Some(Host::Domain(GITHUB_HOST)) {
            let segs = parsed.path_segments().map(|ps| ps.collect()).unwrap_or_else(Vec::new);
            if segs.len() == 2 {
                // github.com/$OWNER/$REPO
                return Some(Repository::new(segs[0], segs[1]));
            }
        }
        None
    }
}

impl fmt::Display for Repository {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.owner, self.name)
    }
}
