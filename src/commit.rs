use anyhow::Result;
use regex::Regex;

use std::str::FromStr;

use crate::git::{git_check, git_commit, git_push};

lazy_static! {
    static ref JIRA_REGEX: Regex = Regex::new("([A-Z][A-Z0-9]+)-\\d+").unwrap();
    static ref JIRA_REPLACE_REGEX: Regex = Regex::new("([A-Z][A-Z0-9]+)-\\d+[[\\s:]+]?").unwrap();
}

#[derive(Debug)]
pub enum ConventionalCommitType {
    Fix,
    Feature,
    Breaking,
    Chore,
    ContinuousIntegration,
    Docs,
    Refactor,
    Test,
}

// TODO: Can we get rid of these implementations somehow nicely?

impl ConventionalCommitType {
    fn as_str(&self) -> &str {
        match self {
            ConventionalCommitType::Fix => "fix",
            ConventionalCommitType::Feature => "feat",
            ConventionalCommitType::Breaking => "breaking",
            ConventionalCommitType::Chore => "chore",
            ConventionalCommitType::ContinuousIntegration => "ci",
            ConventionalCommitType::Docs => "docs",
            ConventionalCommitType::Refactor => "refactor",
            ConventionalCommitType::Test => "test",
        }
    }
}

impl FromStr for ConventionalCommitType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "fix" => ConventionalCommitType::Fix,
            "breaking" => ConventionalCommitType::Breaking,
            "chore" => ConventionalCommitType::Chore,
            "ci" => ConventionalCommitType::ContinuousIntegration,
            "docs" => ConventionalCommitType::Docs,
            "refactor" => ConventionalCommitType::Refactor,
            "test" => ConventionalCommitType::Test,
            _ => ConventionalCommitType::Feature,
        })
    }
}

fn infer_commit_type(_commit_message: &str) -> ConventionalCommitType {
    ConventionalCommitType::Feature
}

fn infer_ticket_name(commit_message: String) -> (String, Option<String>) {
    let ticket_result = JIRA_REGEX
        .captures(&commit_message)
        .map(|captures| captures.get(0).unwrap().as_str().to_string());

    let new_commit_message = JIRA_REPLACE_REGEX.replace(&commit_message, "").to_string();
    (new_commit_message, ticket_result)
}

pub fn perform_commit(
    message: String,
    commit_type: Option<ConventionalCommitType>,
    ticket: Option<String>,
) -> Result<()> {
    // Use the commit type provided from the cli or infer the right commit type from the message.
    let commit_type = match commit_type {
        Some(c_type) => c_type,
        None => infer_commit_type(&message),
    };

    // Use the ticket provided. If the ticket is not provided, use the ticket regex to search for a ticket and use that .
    let (message, ticket) = match ticket {
        Some(ticket) => (message, Some(ticket)),
        None => infer_ticket_name(message),
    };

    // Now that we have the information either from the cli or inferred, we can form the message.
    // The complete form is [commit_type]: [message] ([ticket])
    let mut final_commit_message = String::new();
    final_commit_message.push_str(commit_type.as_str());
    final_commit_message.push_str(": ");
    final_commit_message.push_str(&message);
    if let Some(ticket) = ticket {
        final_commit_message.push_str(&format!(" ({})", ticket));
    }
    println!("Using Commit Message: '{}'", final_commit_message);

    git_check();
    git_commit(&final_commit_message);
    git_push();
    Ok(())
}
