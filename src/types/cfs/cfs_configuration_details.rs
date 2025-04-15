use std::fmt;

/// Struct used by get_configuration when only one CFS configuration is fetched. This means we will
/// CFS confiugration layers will have extra information from the VCS/Gitea1
pub struct ConfigurationDetails {
    pub name: String,
    pub last_updated: String,
    pub config_layers: Vec<LayerDetails>,
}

impl ConfigurationDetails {
    pub fn new(name: &str, last_updated: &str, config_layers: Vec<LayerDetails>) -> Self {
        Self {
            name: String::from(name),
            last_updated: String::from(last_updated),
            config_layers,
        }
    }
}

impl fmt::Display for ConfigurationDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nConfig Details:\n - name: {}\n - last updated: {}\nLayers:",
            self.name, self.last_updated
        )?;

        for (i, config_layer) in self.config_layers.iter().enumerate() {
            write!(f, "\n Layer {}:{}", i, config_layer)?;
        }

        Ok(())
    }
}

pub struct LayerDetails {
    pub name: String,
    pub repo_name: String,
    pub commit_id: String,
    pub author: String,
    pub commit_date: String,
    pub branch: String,
    pub tag: String,
    pub playbook: String, // pub most_recent_commit: bool,
}

impl LayerDetails {
    pub fn new(
        name: &str,
        repo_name: &str,
        commit_id: &str,
        author: &str,
        commit_date: &str,
        branch: &str,
        tag: &str,
        playbook: &str,
        // most_recent_commit: bool,
    ) -> Self {
        Self {
            name: String::from(name),
            repo_name: String::from(repo_name),
            commit_id: String::from(commit_id),
            author: String::from(author),
            commit_date: String::from(commit_date),
            branch: branch.to_string(),
            tag: tag.to_string(),
            playbook: playbook.to_string(),
            // most_recent_commit,
        }
    }
}

impl fmt::Display for LayerDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n - name: {}\n - repo name: {}\n - commit id: {}\n - commit date: {}\n - author: {}\n - branch: {}\n - tag: {}\n - playbook: {}",
            self.name, self.repo_name, self.commit_id, self.commit_date, self.author, self.branch, self.tag, self.playbook
        )
    }
}
