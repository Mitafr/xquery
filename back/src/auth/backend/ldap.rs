use std::borrow::Borrow;

use ldap3::{ldap_escape, Ldap, Scope, SearchEntry};

pub async fn search(
    connection: &mut Ldap,
    base: &str,
    filter: &str,
    account: &str,
) -> Vec<SearchEntry> {
    let a = ldap_escape(account);
    let search_filter = filter.replace("{account}", a.borrow());

    let s = connection
        .search(base, Scope::Subtree, &search_filter, [""])
        .await
        .unwrap();
    match s.success() {
        Ok(r) => r.0.into_iter().map(SearchEntry::construct).collect(),
        Err(e) => {
            tracing::error!("{}", e.to_string());
            Vec::new()
        }
    }
}
