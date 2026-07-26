//! Generates the REST client from TfL's Swagger 2.0 document.
//!
//! TfL's spec is the source of truth for everything under
//! `crates/tfl-api-client/src/generated`, and nothing else. The GraphQL layer is
//! hand-written on top and this never touches it.
//!
//! Writing a generator rather than driving openapi-generator is a deliberate
//! trade: TfL uses a very small corner of Swagger 2.0 — no `allOf`/`oneOf`/
//! `discriminator`, no inline object schemas, only path and query parameters —
//! and the document has not meaningfully changed in years. A few hundred lines
//! that emit exactly the client we want beats a JVM dependency plus a fixup pass
//! patching a general-purpose tool's output.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Write as _,
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

// ---------------------------------------------------------------- spec model

#[derive(Deserialize)]
pub struct Spec {
    definitions: BTreeMap<String, Definition>,
    paths: BTreeMap<String, BTreeMap<String, Operation>>,
}

#[derive(Deserialize)]
struct Definition {
    #[serde(default)]
    properties: BTreeMap<String, Schema>,
}

#[derive(Deserialize, Clone, Default)]
struct Schema {
    #[serde(rename = "$ref")]
    reference: Option<String>,
    #[serde(rename = "type")]
    ty: Option<String>,
    format: Option<String>,
    items: Option<Box<Schema>>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct Operation {
    #[serde(rename = "operationId")]
    operation_id: String,
    summary: Option<String>,
    description: Option<String>,
    #[serde(default)]
    parameters: Vec<Param>,
    #[serde(default)]
    responses: BTreeMap<String, Response>,
}

#[derive(Deserialize, Clone)]
struct Param {
    name: String,
    #[serde(rename = "in")]
    location: String,
    #[serde(default)]
    required: bool,
    #[serde(rename = "type")]
    ty: Option<String>,
    format: Option<String>,
    items: Option<Schema>,
    #[serde(rename = "collectionFormat")]
    collection_format: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct Response {
    schema: Option<Schema>,
}

// ------------------------------------------------------------------- output

pub struct Generated {
    pub models: String,
    pub endpoints: String,
    pub model_count: usize,
    pub endpoint_count: usize,
}

pub fn generate(spec: &Spec) -> Result<Generated> {
    let names = TypeNames::resolve(&spec.definitions);
    let models = emit_models(spec, &names)?;
    let (endpoints, endpoint_count) = emit_endpoints(spec, &names)?;
    Ok(Generated {
        models,
        endpoints,
        model_count: spec.definitions.len(),
        endpoint_count,
    })
}

/// Maps Swagger definition keys onto Rust type names.
///
/// TfL's keys carry the full CLR namespace
/// (`Tfl.Api.Presentation.Entities.StopPoint`). The last segment alone is
/// usually what you want, but it collides — there is both a `Fares.Journey` and
/// a `JourneyPlanner.Journey` — so colliding names absorb the segment above
/// them until they are distinct.
struct TypeNames {
    rust: HashMap<String, String>,
    /// Definitions with no properties at all — TfL's `System.Object`, and
    /// anything else the spec declines to describe. A `$ref` to one of these
    /// decodes as raw JSON: an empty struct would parse successfully and throw
    /// the entire payload away, which is the worst of both.
    opaque: BTreeSet<String>,
}

impl TypeNames {
    fn resolve(definitions: &BTreeMap<String, Definition>) -> Self {
        let opaque: BTreeSet<String> = definitions
            .iter()
            .filter(|(_, d)| d.properties.is_empty())
            .map(|(k, _)| k.clone())
            .collect();
        let keys: Vec<&str> = definitions.keys().map(String::as_str).collect();
        let mut depth: HashMap<&str, usize> = keys.iter().map(|k| (*k, 1)).collect();

        // Widen any name that more than one key claims, until all are unique.
        loop {
            let mut claims: HashMap<String, Vec<&str>> = HashMap::new();
            for key in &keys {
                claims.entry(tail(key, depth[key])).or_default().push(key);
            }
            let contested: Vec<&str> = claims
                .into_iter()
                .filter(|(_, ks)| ks.len() > 1)
                .flat_map(|(_, ks)| ks)
                .filter(|k| depth[k] < k.split('.').count())
                .collect();
            if contested.is_empty() {
                break;
            }
            for key in contested {
                *depth.get_mut(key).unwrap() += 1;
            }
        }

        Self {
            rust: keys
                .iter()
                .map(|k| ((*k).to_string(), pascal(&tail(k, depth[k]))))
                .collect(),
            opaque,
        }
    }

    fn rust_name(&self, reference: &str) -> Result<&str> {
        let key = reference.trim_start_matches("#/definitions/");
        self.rust
            .get(key)
            .map(String::as_str)
            .with_context(|| format!("unknown $ref `{reference}`"))
    }

    fn is_opaque(&self, reference: &str) -> bool {
        self.opaque
            .contains(reference.trim_start_matches("#/definitions/"))
    }
}

/// The last `n` dot-separated segments of a definition key, minus the vendor
/// prefixes every TfL type shares.
fn tail(key: &str, n: usize) -> String {
    let segments: Vec<&str> = key
        .split('.')
        .filter(|s| !matches!(*s, "Tfl" | "Api" | "Presentation" | "Entities" | "Common"))
        .collect();
    let start = segments.len().saturating_sub(n.max(1));
    segments[start..].concat()
}

// ------------------------------------------------------------------- models

fn emit_models(spec: &Spec, names: &TypeNames) -> Result<String> {
    let mut out = String::from(
        "//! Response types, generated from TfL's Swagger document. Do not edit.\n\
         //!\n\
         //! Every field is `Option` because the spec marks almost nothing as required\n\
         //! and TfL means it: a live `/StopPoint` response carries about half of the\n\
         //! fields declared here. Absence is normal, not an error.\n\n\
         use serde::{Deserialize, Serialize};\n",
    );

    for (key, def) in &spec.definitions {
        let name = names.rust_name(key)?;
        out.push('\n');
        let _ = writeln!(
            out,
            "/// `{key}`\n#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]\n#[serde(default)]\npub struct {name} {{"
        );
        for (field, schema) in &def.properties {
            if let Some(doc) = doc_comment(schema.description.as_deref(), 1) {
                out.push_str(&doc);
            }
            let rust_field = snake(field);
            let ident = if rust_field == *field {
                rust_field.clone()
            } else {
                let _ = writeln!(out, "    #[serde(rename = \"{field}\")]");
                rust_field
            };
            let ty = rust_type(schema, names)?;
            // TfL declares booleans it then sends as strings, and serde fails
            // the whole response on one bad field.
            if ty == "bool" {
                out.push_str(
                    "    #[serde(default, deserialize_with = \"crate::de::bool_or_string\")]\n",
                );
            }
            let _ = writeln!(out, "    pub {}: Option<{ty}>,", escape_ident(&ident));
        }
        out.push_str("}\n");
    }
    Ok(out)
}

fn rust_type(schema: &Schema, names: &TypeNames) -> Result<String> {
    if let Some(reference) = &schema.reference {
        if names.is_opaque(reference) {
            return Ok("serde_json::Value".into());
        }
        return Ok(format!(
            "crate::generated::models::{}",
            names.rust_name(reference)?
        ));
    }
    let ty = schema.ty.as_deref().unwrap_or("object");
    Ok(match ty {
        "string" => "String".into(),
        "boolean" => "bool".into(),
        "number" => "f64".into(),
        "integer" => match schema.format.as_deref() {
            Some("int64") => "i64".into(),
            _ => "i32".into(),
        },
        "array" => {
            let items = schema.items.as_deref().cloned().unwrap_or_default();
            format!("Vec<{}>", rust_type(&items, names)?)
        }
        // TfL returns bare `System.Object` from a handful of endpoints. Keeping
        // the payload as raw JSON is honest; there is nothing to model.
        "object" => "serde_json::Value".into(),
        other => bail!("unhandled schema type `{other}`"),
    })
}

// ---------------------------------------------------------------- endpoints

fn emit_endpoints(spec: &Spec, names: &TypeNames) -> Result<(String, usize)> {
    let mut out = String::from(
        "//! One method per TfL endpoint, generated from the Swagger document. Do not edit.\n\
         //!\n\
         //! These only build a path and query string; every request goes through\n\
         //! [`Client::get`], which owns authentication, caching and retries.\n\n\
         use crate::{Client, Result};\n",
    );

    let mut taken = BTreeSet::new();
    let mut count = 0;

    for (path, methods) in &spec.paths {
        for (verb, op) in methods {
            if verb != "get" {
                bail!("unexpected {verb} on {path}: TfL's API is read-only");
            }
            out.push('\n');
            out.push_str(&emit_operation(path, op, names, &mut taken)?);
            count += 1;
        }
    }
    Ok((out, count))
}

fn emit_operation(
    path: &str,
    op: &Operation,
    names: &TypeNames,
    taken: &mut BTreeSet<String>,
) -> Result<String> {
    // Placeholders are resolved against the path parameters first and then,
    // case-insensitively, against the query parameters. Several TfL paths
    // interpolate a value they only ever declare as a query parameter —
    // `/Place/{type}/At/{Lat}/{Lon}` declares `lat` and `lon` as query — and
    // taking the declaration literally emits a reference to a binding that does
    // not exist.
    let placeholders = placeholders(path);
    let mut in_path: Vec<&Param> = Vec::new();
    for name in &placeholders {
        let matched = op
            .parameters
            .iter()
            .find(|p| p.location == "path" && p.name == *name)
            .or_else(|| {
                op.parameters
                    .iter()
                    .find(|p| p.name.eq_ignore_ascii_case(name))
            });
        match matched {
            Some(p) => in_path.push(p),
            None => bail!("{path} interpolates `{name}` but declares no such parameter"),
        }
    }

    // A parameter interpolated into the path is not also sent as a query
    // parameter.
    let query: Vec<&Param> = op
        .parameters
        .iter()
        .filter(|p| p.location == "query")
        .filter(|p| !in_path.iter().any(|ip| ip.name == p.name))
        .collect();
    let (required, optional): (Vec<&Param>, Vec<&Param>) = query.iter().partition(|p| p.required);
    let path_params = in_path;

    let method = unique_method_name(op, &path_params, taken);
    let returns = response_type(op, names)?;

    // Required parameters are positional; the long tail of optional query
    // parameters (the journey planner alone has 27) goes in a Default struct so
    // callers set only what they mean.
    let opts_ty = (!optional.is_empty()).then(|| format!("{}Options", pascal(&method)));

    let mut args = String::new();
    for p in path_params.iter().chain(required.iter()) {
        let _ = write!(
            args,
            ", {}: {}",
            escape_ident(&snake(&p.name)),
            param_type(p, Owned::No)?
        );
    }
    if let Some(ty) = &opts_ty {
        let _ = write!(args, ", options: &{ty}");
    }

    let mut out = String::new();

    if let Some(ty) = &opts_ty {
        let _ = writeln!(
            out,
            "/// Optional query parameters for [`Client::{method}`].\n#[derive(Debug, Clone, Default)]\npub struct {ty} {{"
        );
        for p in &optional {
            if let Some(doc) = doc_comment(p.description.as_deref(), 1) {
                out.push_str(&doc);
            }
            let _ = writeln!(
                out,
                "    pub {}: Option<{}>,",
                escape_ident(&snake(&p.name)),
                param_type(p, Owned::Yes)?
            );
        }
        out.push_str("}\n\n");
    }

    out.push_str("impl Client {\n");
    let summary = op.summary.as_deref().or(op.description.as_deref());
    if let Some(doc) = doc_comment(summary, 1) {
        out.push_str(&doc);
    }
    let _ = writeln!(out, "    ///\n    /// `GET {path}`");
    let _ = writeln!(
        out,
        "    pub async fn {method}(&self{args}) -> Result<{returns}> {{"
    );

    // Path parameters are interpolated; arrays become the comma-joined lists
    // TfL uses for its batch endpoints. Only strings are percent-encoded —
    // numbers have nothing to escape and `segment` takes a `&str`.
    let mut literal = path.to_string();
    let mut interpolations = String::new();
    for (placeholder, p) in placeholders.iter().zip(&path_params) {
        let ident = escape_ident(&snake(&p.name));
        let value = match p.ty.as_deref() {
            Some("array") => format!("crate::join({ident})"),
            Some("string") | None => format!("crate::segment({ident})"),
            _ => ident.clone(),
        };
        literal = literal.replace(
            &format!("{{{placeholder}}}"),
            &format!("{{{}}}", snake(&p.name)),
        );
        let _ = write!(interpolations, ", {} = {value}", snake(&p.name));
    }
    // Locals are underscore-prefixed because TfL has parameters called `query`
    // and `path`; generated identifiers never start with an underscore, so this
    // cannot collide.
    let _ = writeln!(
        out,
        "        let __path = format!(\"{literal}\"{interpolations});"
    );
    let _ = writeln!(
        out,
        "        let mut __query: Vec<(&str, String)> = Vec::new();"
    );
    for p in &required {
        let _ = writeln!(
            out,
            "        {}",
            push_query(p, &escape_ident(&snake(&p.name)))
        );
    }
    for p in &optional {
        let ident = escape_ident(&snake(&p.name));
        let _ = writeln!(
            out,
            "        if let Some(value) = &options.{ident} {{ {} }}",
            push_query(p, "value")
        );
    }
    // List endpoints go through `get_list`, which tolerates TfL answering a
    // single-id batch request with a bare object instead of a one-element
    // array.
    let getter = if returns.starts_with("Vec<") {
        "get_list"
    } else {
        "get"
    };
    let _ = writeln!(
        out,
        "        self.{getter}(&__path, &__query).await\n    }}\n}}"
    );
    Ok(out)
}

/// The `{placeholder}` names in a path template, in order.
fn placeholders(path: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = path;
    while let Some(open) = rest.find('{') {
        let Some(close) = rest[open..].find('}') else {
            break;
        };
        found.push(rest[open + 1..open + close].to_string());
        rest = &rest[open + close + 1..];
    }
    found
}

fn push_query(p: &Param, value: &str) -> String {
    let name = &p.name;
    if p.ty.as_deref() != Some("array") {
        return format!("__query.push((\"{name}\", {value}.to_string()));");
    }
    // `multi` repeats the key; everything else is a comma-joined list.
    //
    // Except where TfL contradicts itself inside one parameter definition:
    // `accessibilityPreference` is declared `multi` and described as "must be a
    // comma separated list", and the repeated form really does 400 while the
    // joined form works. Its own prose is the better evidence, since it
    // describes what the service does rather than what the generator was told.
    let says_comma_separated = p.description.as_deref().is_some_and(|d| {
        d.to_lowercase().contains("comma") && d.to_lowercase().contains("separat")
    });
    if p.collection_format.as_deref() == Some("multi") && !says_comma_separated {
        format!("for item in {value} {{ __query.push((\"{name}\", item.to_string())); }}")
    } else {
        format!("__query.push((\"{name}\", crate::join({value})));")
    }
}

/// Whether a parameter is being rendered as a borrowed function argument or as
/// a field of an owned, `Default`-constructed options struct.
#[derive(Clone, Copy, PartialEq)]
enum Owned {
    Yes,
    No,
}

fn param_type(p: &Param, owned: Owned) -> Result<String> {
    Ok(match p.ty.as_deref().unwrap_or("string") {
        "string" if owned == Owned::Yes => "String".into(),
        "string" => "&str".into(),
        "boolean" => "bool".into(),
        "number" => "f64".into(),
        "integer" => match p.format.as_deref() {
            Some("int64") => "i64".into(),
            _ => "i32".into(),
        },
        "array" => {
            let inner = p
                .items
                .as_ref()
                .and_then(|i| i.ty.as_deref())
                .unwrap_or("string");
            match (inner, owned) {
                ("string", Owned::Yes) => "Vec<String>".into(),
                ("string", Owned::No) => "&[&str]".into(),
                ("integer", Owned::Yes) => "Vec<i32>".into(),
                ("integer", Owned::No) => "&[i32]".into(),
                (other, _) => bail!("unhandled array parameter item type `{other}`"),
            }
        }
        other => bail!("unhandled parameter type `{other}`"),
    })
}

fn response_type(op: &Operation, names: &TypeNames) -> Result<String> {
    let Some(schema) = op.responses.get("200").and_then(|r| r.schema.as_ref()) else {
        return Ok("()".into());
    };
    rust_type(schema, names)
}

/// Turns `StopPoint_Search` into `stop_point_search`, disambiguating the four
/// operation ids TfL reuses across two paths (`/StopPoint` and
/// `/StopPoint/{ids}` are both `StopPoint_Get`) by the path parameters that
/// actually distinguish them.
fn unique_method_name(
    op: &Operation,
    path_params: &[&Param],
    taken: &mut BTreeSet<String>,
) -> String {
    let base = snake(&op.operation_id.replace('_', ""));
    let mut name = base.clone();
    if taken.contains(&name) && !path_params.is_empty() {
        let by: Vec<String> = path_params.iter().map(|p| snake(&p.name)).collect();
        name = format!("{base}_by_{}", by.join("_"));
    }
    let mut n = 2;
    while taken.contains(&name) {
        name = format!("{base}_{n}");
        n += 1;
    }
    taken.insert(name.clone());
    name
}

// -------------------------------------------------------------------- naming

fn snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 && !out.ends_with('_') {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else if ch.is_alphanumeric() {
            out.push(ch);
        } else if !out.ends_with('_') && !out.is_empty() {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

fn pascal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = true;
    for ch in s.chars() {
        if !ch.is_alphanumeric() {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

/// `type` is the only Swagger field name in the document that Rust reserves.
fn escape_ident(s: &str) -> String {
    const RESERVED: &[&str] = &[
        "type", "ref", "match", "move", "box", "mod", "use", "self", "crate", "fn", "let", "loop",
        "where", "impl", "const", "static", "struct", "enum", "abstract", "final", "override",
        "async", "await", "dyn", "in", "as", "if", "else", "for", "return",
    ];
    if RESERVED.contains(&s) {
        format!("r#{s}")
    } else {
        s.to_string()
    }
}

/// Renders a Swagger description as a doc comment.
///
/// TfL's descriptions are pasted out of .NET XML docs and arrive with the
/// original source indentation, so each line is re-trimmed and blank runs are
/// collapsed.
fn doc_comment(text: Option<&str>, indent: usize) -> Option<String> {
    let text = text?.trim();
    if text.is_empty() {
        return None;
    }
    let pad = "    ".repeat(indent);
    let mut out = String::new();
    let mut blank = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            blank = true;
            continue;
        }
        if blank && !out.is_empty() {
            let _ = writeln!(out, "{pad}///");
        }
        blank = false;
        let _ = writeln!(out, "{pad}/// {}", line.replace('\r', ""));
    }
    Some(out)
}
