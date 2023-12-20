use std::borrow::Cow;

const ACME_CHALLENGE_SUBDOMAIN: &str = "_acme-challenge";

pub struct RootDomainAndChallengeRecord<'a> {
    root_domain: &'a str,
    challenge_record_name: Cow<'a, str>,
}

impl<'a> RootDomainAndChallengeRecord<'a> {
    pub fn root_domain(&self) -> &str {
        self.root_domain
    }

    pub fn challenge_record_name(&self) -> &str {
        &self.challenge_record_name
    }
}

/// Returns `(root_domain, challenge_record_name)`.
pub fn get_domain_names_to_use(domain: &str) -> RootDomainAndChallengeRecord {
    // find the second dot counting from the end
    if let Some(rightmost_dot_idx) = domain.rfind('.') {
        if let Some(sep) = domain[0..rightmost_dot_idx].rfind('.') {
            let challenge_record_name = format!("{}.{}", ACME_CHALLENGE_SUBDOMAIN, &domain[0..sep]);
            return RootDomainAndChallengeRecord {
                root_domain: &domain[(sep + 1)..domain.len()],
                challenge_record_name: Cow::Owned(challenge_record_name),
            };
        }
    }

    RootDomainAndChallengeRecord {
        root_domain: domain,
        challenge_record_name: ACME_CHALLENGE_SUBDOMAIN.into(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_domain_names_to_use() {
        use super::get_domain_names_to_use;

        let result = get_domain_names_to_use("example");
        assert_eq!(result.root_domain(), "example");
        assert_eq!(result.challenge_record_name(), "_acme-challenge");

        let result = get_domain_names_to_use("example.com");
        assert_eq!(result.root_domain(), "example.com");
        assert_eq!(result.challenge_record_name(), "_acme-challenge");

        let result = get_domain_names_to_use("test.example.com");
        assert_eq!(result.root_domain(), "example.com");
        assert_eq!(result.challenge_record_name(), "_acme-challenge.test");

        let result = get_domain_names_to_use("foo.bar.example.com");
        assert_eq!(result.root_domain(), "example.com");
        assert_eq!(result.challenge_record_name(), "_acme-challenge.foo.bar");
    }
}
