// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

pub mod decls;

use std::path::Path;
use std::sync::Arc;

use crate::io::InMemorySourceResolver;
use crate::io::SourceResolver;

use super::parse_source;

use super::QasmSemanticParseResult;

use miette::Report;

use expect_test::Expect;

pub(crate) fn parse_all<P>(
    path: P,
    sources: impl IntoIterator<Item = (Arc<str>, Arc<str>)>,
) -> miette::Result<QasmSemanticParseResult, Vec<Report>>
where
    P: AsRef<Path>,
{
    let resolver = InMemorySourceResolver::from_iter(sources);
    let source = resolver.resolve(path.as_ref()).map_err(|e| vec![e])?.1;
    let res = parse_source(source, path, &resolver).map_err(|e| vec![e])?;
    if res.source.has_errors() {
        let errors = res
            .errors()
            .into_iter()
            .map(|e| Report::new(e.clone()))
            .collect();
        Err(errors)
    } else {
        Ok(res)
    }
}

pub(crate) fn parse<S>(source: S) -> miette::Result<QasmSemanticParseResult, Vec<Report>>
where
    S: AsRef<str>,
{
    let resolver = InMemorySourceResolver::from_iter([("test".into(), source.as_ref().into())]);
    let res = parse_source(source, "test", &resolver).map_err(|e| vec![e])?;
    if res.source.has_errors() {
        let errors = res
            .errors()
            .into_iter()
            .map(|e| Report::new(e.clone()))
            .collect();
        return Err(errors);
    }
    Ok(res)
}

pub(super) fn check(input: &str, expect: &Expect) {
    check_map(input, expect);
}

fn check_map<S>(input: S, expect: &Expect)
where
    S: AsRef<str>,
{
    let resolver = InMemorySourceResolver::from_iter([("test".into(), input.as_ref().into())]);
    let res = parse_source(input, "test", &resolver)
        .map_err(|e| vec![e])
        .expect("failed to parse");
    let errors = res.all_errors();

    if errors.is_empty() {
        expect.assert_eq(&res.program.to_string());
    } else {
        expect.assert_eq(&format!(
            "{}\n\n{:?}",
            res.program,
            errors
                .iter()
                .map(|e| Report::new(e.clone()))
                .collect::<Vec<_>>()
        ));
    }
}
