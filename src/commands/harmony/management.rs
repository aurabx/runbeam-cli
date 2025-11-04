use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde_json::{Map, Value};
use std::cmp;

fn resolve_instance(
    id: Option<&str>,
    label: Option<&str>,
) -> Result<crate::storage::HarmonyInstance> {
    let list = crate::storage::load_harmony_instances()?;
    if let Some(id) = id {
        if let Some(inst) = list.into_iter().find(|i| i.id == id) {
            return Ok(inst);
        }
        return Err(anyhow!("no instance with id '{}'", id));
    }
    if let Some(label) = label {
        if let Some(inst) = list.into_iter().find(|i| i.label == label) {
            return Ok(inst);
        }
        return Err(anyhow!("no instance with label '{}'", label));
    }
    Err(anyhow!("must supply --id or --label"))
}

fn render_json_table(v: &Value) {
    match v {
        Value::Object(obj) => render_kv_table(obj),
        Value::Array(arr) => render_array_of_objects(arr),
        _ => println!("{}", v),
    }
}

fn render_kv_table(obj: &Map<String, Value>) {
    let mut rows: Vec<(String, String)> = obj
        .iter()
        .map(|(k, v)| (k.clone(), stringify_value(v)))
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));

    let mut w_key = "KEY".len();
    let mut w_val = "VALUE".len();
    for (k, val) in &rows {
        w_key = cmp::max(w_key, k.len());
        w_val = cmp::max(w_val, val.len());
    }

    println!(
        "{k:<kw$} | {v:<vw$}",
        k = "KEY",
        v = "VALUE",
        kw = w_key,
        vw = w_val
    );
    println!(
        "{k:-<kw$}-+-{v:-<vw$}",
        k = "",
        v = "",
        kw = w_key,
        vw = w_val
    );
    for (k, val) in rows {
        println!(
            "{k:<kw$} | {v:<vw$}",
            k = k,
            v = val,
            kw = w_key,
            vw = w_val
        );
    }
}

fn render_array_of_objects(arr: &[Value]) {
    if arr.is_empty() {
        println!("(no results)");
        return;
    }
    // Collect union of keys
    let mut cols: Vec<String> = Vec::new();
    for v in arr {
        if let Value::Object(m) = v {
            for k in m.keys() {
                if !cols.iter().any(|c| c == k) {
                    cols.push(k.clone());
                }
            }
        }
    }
    if cols.is_empty() {
        // Not objects; just print JSON
        println!(
            "{}",
            serde_json::to_string_pretty(&Value::Array(arr.to_vec())).unwrap()
        );
        return;
    }

    // Compute widths
    let mut widths: Vec<usize> = cols.iter().map(|c| cmp::max(c.len(), 1)).collect();
    for v in arr {
        if let Value::Object(m) = v {
            for (ci, col) in cols.iter().enumerate() {
                let s = stringify_value(m.get(col).unwrap_or(&Value::Null));
                widths[ci] = cmp::max(widths[ci], s.len());
            }
        }
    }

    // Header
    for (i, col) in cols.iter().enumerate() {
        if i > 0 {
            print!(" | ");
        }
        print!("{val:<w$}", val = col.to_uppercase(), w = widths[i]);
    }
    println!();
    // Separator
    for (i, w) in widths.iter().copied().enumerate() {
        if i > 0 {
            print!("-+-");
        }
        print!("{dash:-<w$}", dash = "", w = w);
    }
    println!();
    // Rows
    for v in arr {
        if let Value::Object(m) = v {
            for (i, col) in cols.iter().enumerate() {
                if i > 0 {
                    print!(" | ");
                }
                let s = stringify_value(m.get(col).unwrap_or(&Value::Null));
                print!("{val:<w$}", val = s, w = widths[i]);
            }
            println!();
        }
    }
}

fn stringify_value(v: &Value) -> String {
    match v {
        Value::Null => "".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(a) => {
            // Join simple values, else compact JSON
            let joined: Option<String> = a
                .iter()
                .map(|x| match x {
                    Value::String(s) => Some(s.clone()),
                    Value::Number(n) => Some(n.to_string()),
                    Value::Bool(b) => Some(b.to_string()),
                    _ => None,
                })
                .collect::<Option<Vec<String>>>()
                .map(|v| v.join(","));
            joined.unwrap_or_else(|| serde_json::to_string(a).unwrap_or_default())
        }
        Value::Object(_) => serde_json::to_string(v).unwrap_or_default(),
    }
}

fn base_url(inst: &crate::storage::HarmonyInstance) -> String {
    format!(
        "http://{}:{}/{}",
        inst.ip,
        inst.port,
        inst.path_prefix.trim_matches('/')
    )
}

pub fn info(id: Option<&str>, label: Option<&str>) -> Result<()> {
    let inst = resolve_instance(id, label)?;
    let url = format!("{}/info", base_url(&inst));
    let client = Client::new();
    let resp = client
        .get(&url)
        .send()
        .with_context(|| format!("GET {}", url))?;

    if !resp.status().is_success() {
        return Err(anyhow!("{} {}", resp.status(), url));
    }

    let json: Value = resp.json().context("parsing JSON response")?;
    if let Some(arr) = json.get("routes").and_then(|v| v.as_array()) {
        render_array_of_objects(arr);
    } else {
        render_json_table(&json);
    }
    Ok(())
}

pub fn pipelines(id: Option<&str>, label: Option<&str>) -> Result<()> {
    let inst = resolve_instance(id, label)?;
    let url = format!("{}/pipelines", base_url(&inst));
    let client = Client::new();
    let resp = client
        .get(&url)
        .send()
        .with_context(|| format!("GET {}", url))?;

    if !resp.status().is_success() {
        return Err(anyhow!("{} {}", resp.status(), url));
    }

    let json: Value = resp.json().context("parsing JSON response")?;
    if let Some(arr) = json.get("pipelines").and_then(|v| v.as_array()) {
        render_array_of_objects(arr);
    } else {
        render_json_table(&json);
    }
    Ok(())
}

pub fn routes(id: Option<&str>, label: Option<&str>, json: bool) -> Result<()> {
    let inst = resolve_instance(id, label)?;
    let url = format!("{}/routes", base_url(&inst));
    let client = Client::new();
    let resp = client
        .get(&url)
        .send()
        .with_context(|| format!("GET {}", url))?;

    if !resp.status().is_success() {
        return Err(anyhow!("{} {}", resp.status(), url));
    }

    let json_value: Value = resp.json().context("parsing JSON response")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&json_value)?);
    } else {
        // Render as table
        if let Some(routes_array) = json_value.get("routes").and_then(|v| v.as_array()) {
            render_array_of_objects(routes_array);
        } else {
            render_json_table(&json_value);
        }
    }
    Ok(())
}

pub fn reload(id: Option<&str>, label: Option<&str>) -> Result<()> {
    let inst = resolve_instance(id, label)?;
    let url = format!("http://{}:{}/api/reload", inst.ip, inst.port);
    let client = Client::new();
    let resp = client
        .post(&url)
        .send()
        .with_context(|| format!("POST {}", url))?;

    if !resp.status().is_success() {
        return Err(anyhow!("{} {}", resp.status(), url));
    }

    let json: Value = resp.json().context("parsing JSON response")?;
    println!("✓ Reload triggered successfully");
    render_json_table(&json);
    Ok(())
}
