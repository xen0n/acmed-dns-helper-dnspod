use crate::defs::ACME_CHALLENGE_SUBDOMAIN;

/// Returns `(root_domain, challenge_record_name)`.
pub fn get_domain_names_to_use(domain: &str) -> (&str, String) {
    // find the second dot counting from the end
    if let Some(rightmost_dot_idx) = domain.rfind('.') {
        if let Some(sep) = domain[0..rightmost_dot_idx].rfind('.') {
            return (
                &domain[(sep + 1)..domain.len()],
                format!("{}.{}", ACME_CHALLENGE_SUBDOMAIN, &domain[0..sep]),
            );
        }
    }

    return (domain, ACME_CHALLENGE_SUBDOMAIN.to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_domain_names_to_use() {
        use crate::get_domain_names_to_use;

        let (root, challenge_record) = get_domain_names_to_use("example");
        assert_eq!(root, "example");
        assert_eq!(challenge_record, "_acme-challenge");

        let (root, challenge_record) = get_domain_names_to_use("example.com");
        assert_eq!(root, "example.com");
        assert_eq!(challenge_record, "_acme-challenge");

        let (root, challenge_record) = get_domain_names_to_use("test.example.com");
        assert_eq!(root, "example.com");
        assert_eq!(challenge_record, "_acme-challenge.test");

        let (root, challenge_record) = get_domain_names_to_use("foo.bar.example.com");
        assert_eq!(root, "example.com");
        assert_eq!(challenge_record, "_acme-challenge.foo.bar");
    }
}