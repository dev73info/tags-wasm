use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// ── 输入类型（匹配后端 /resources/tags 返回结构） ──

#[derive(Deserialize)]
struct RawTagItem {
    id: u32,
    name: String,
    #[serde(default)]
    alias: String,
    sort_order: i32,
}

#[derive(Deserialize)]
struct RawTagGroup {
    id: u32,
    parent_group_id: Option<u32>,
    name: String,
    #[serde(default)]
    alias: String,
    sort_order: i32,
    child_groups: Vec<RawTagGroup>,
    items: Vec<RawTagItem>,
}

// ── 输出类型（camelCase 序列化，匹配前端类型） ──

#[derive(Serialize)]
struct TagOption {
    id: u32,
    name: String,
    alias: String,
}

#[derive(Serialize)]
struct PublishTagGroup {
    key: String,
    group_id: u32,
    group_name: String,
    group_alias: String,
    group_path: Vec<String>,
    group_path_aliases: Vec<String>,
    label: String,
    depth: u32,
    items: Vec<TagOption>,
}

#[derive(Serialize)]
struct CatalogEntry {
    key: String,
    platform: String,
    label: String,
    group_name: String,
    path_label: String,
    icon: String,
    publish_groups: Vec<PublishTagGroup>,
}

#[derive(Serialize)]
struct CatalogRoot {
    key: String,
    label: String,
    first_entry_key: Option<String>,
    entries: Vec<CatalogEntry>,
}

#[derive(Serialize)]
struct ProcessedTagTree {
    roots: Vec<CatalogRoot>,
}

#[derive(Serialize)]
struct AdminTreeNode {
    key: String,
    id: u32,
    kind: String,
    label: String,
    alias: String,
    parent_group_id: Option<u32>,
    children: Vec<AdminTreeNode>,
}

#[derive(Serialize)]
struct AdminGroupPath {
    group_id: u32,
    path: Vec<String>,
}

#[derive(Serialize)]
struct ProcessedAdminTagTree {
    tree_nodes: Vec<AdminTreeNode>,
    group_count: u32,
    tag_count: u32,
    group_paths: Vec<AdminGroupPath>,
}

// ── 处理逻辑 ──

fn normalize(name: &str) -> String {
    name.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_alias(alias: &str) -> String {
    alias.split_whitespace().collect::<Vec<_>>().join("-")
}

fn sort_groups(groups: &mut [RawTagGroup]) {
    groups.sort_by(|a, b| {
        a.sort_order
            .cmp(&b.sort_order)
            .then_with(|| a.id.cmp(&b.id))
    });
}

fn make_sorted_items(mut items: Vec<RawTagItem>) -> Vec<TagOption> {
    items.sort_by(|a, b| {
        a.sort_order
            .cmp(&b.sort_order)
            .then_with(|| a.id.cmp(&b.id))
    });
    items
        .into_iter()
        .map(|i| TagOption {
            id: i.id,
            name: i.name,
            alias: normalize_alias(&i.alias),
        })
        .collect()
}

fn collect_publish_groups(
    mut groups: Vec<RawTagGroup>,
    parent_path: &[String],
    parent_alias_path: &[String],
    depth: u32,
) -> Vec<PublishTagGroup> {
    sort_groups(&mut groups);
    let mut result = Vec::new();

    for group in groups {
        let RawTagGroup {
            id,
            name,
            alias,
            child_groups,
            items,
            ..
        } = group;

        let normalized = normalize(&name);
        let normalized_alias = normalize_alias(&alias);
        let mut path = parent_path.to_vec();
        path.push(normalized.clone());
        let mut alias_path = parent_alias_path.to_vec();
        alias_path.push(normalized_alias.clone());

        let sorted = make_sorted_items(items);
        if !sorted.is_empty() {
            result.push(PublishTagGroup {
                key: format!("{}::{}", id, alias_path.join("::")),
                group_id: id,
                group_name: normalized,
                group_alias: normalized_alias,
                group_path: path.clone(),
                group_path_aliases: alias_path.clone(),
                label: path.join(" / "),
                depth,
                items: sorted,
            });
        }

        result.extend(collect_publish_groups(
            child_groups,
            &path,
            &alias_path,
            depth + 1,
        ));
    }

    result
}

fn build_processed_tree(mut raw: Vec<RawTagGroup>) -> ProcessedTagTree {
    let mut roots: Vec<_> = raw
        .drain(..)
        .filter(|g| g.parent_group_id.is_none())
        .collect();
    sort_groups(&mut roots);

    let catalog_roots = roots
        .into_iter()
        .map(|root| {
            let RawTagGroup {
                name,
                alias,
                mut child_groups,
                ..
            } = root;

            let root_label = normalize(&name);
            let root_key = normalize_alias(&alias);
            sort_groups(&mut child_groups);

            let first_entry_key = child_groups.first().map(|c| normalize_alias(&c.alias));

            let entries = child_groups
                .into_iter()
                .map(|child| {
                    let RawTagGroup {
                        name,
                        alias,
                        child_groups: sub_groups,
                        ..
                    } = child;

                    let label = normalize(&name);
                    let entry_key = normalize_alias(&alias);
                    let path_label = format!("{} / {}", root_label, label);
                    let publish_groups = collect_publish_groups(
                        sub_groups,
                        &[label.clone()],
                        std::slice::from_ref(&entry_key),
                        0,
                    );

                    CatalogEntry {
                        key: entry_key,
                        platform: label.clone(),
                        label: label.clone(),
                        group_name: label,
                        path_label,
                        icon: String::from("📁"),
                        publish_groups,
                    }
                })
                .collect();

            CatalogRoot {
                key: root_key,
                label: root_label,
                first_entry_key,
                entries,
            }
        })
        .collect();

    ProcessedTagTree {
        roots: catalog_roots,
    }
}

fn build_admin_tree_nodes(
    mut groups: Vec<RawTagGroup>,
    parent_group_id: Option<u32>,
    parent_path: &[String],
    group_paths: &mut Vec<AdminGroupPath>,
    group_count: &mut u32,
    tag_count: &mut u32,
) -> Vec<AdminTreeNode> {
    sort_groups(&mut groups);

    groups
        .into_iter()
        .map(|group| {
            let RawTagGroup {
                id,
                name,
                alias,
                child_groups,
                items,
                ..
            } = group;

            *group_count += 1;

            let normalized = normalize(&name);
            let normalized_alias = normalize_alias(&alias);
            let mut path = parent_path.to_vec();
            path.push(normalized.clone());
            group_paths.push(AdminGroupPath {
                group_id: id,
                path: path.clone(),
            });

            let child_nodes = build_admin_tree_nodes(
                child_groups,
                Some(id),
                &path,
                group_paths,
                group_count,
                tag_count,
            );

            let tag_nodes = make_sorted_items(items)
                .into_iter()
                .map(|item| {
                    *tag_count += 1;
                    AdminTreeNode {
                        key: format!("tag:{}", item.id),
                        id: item.id,
                        kind: String::from("tag"),
                        label: item.name,
                        alias: item.alias,
                        parent_group_id: Some(id),
                        children: Vec::new(),
                    }
                })
                .collect::<Vec<_>>();

            let mut children = child_nodes;
            children.extend(tag_nodes);

            AdminTreeNode {
                key: format!("group:{}", id),
                id,
                kind: String::from("group"),
                label: normalized,
                alias: normalized_alias,
                parent_group_id,
                children,
            }
        })
        .collect()
}

fn build_processed_admin_tree(mut raw: Vec<RawTagGroup>) -> ProcessedAdminTagTree {
    let mut roots: Vec<_> = raw
        .drain(..)
        .filter(|group| group.parent_group_id.is_none())
        .collect();
    sort_groups(&mut roots);

    let mut group_paths = Vec::new();
    let mut group_count = 0;
    let mut tag_count = 0;
    let tree_nodes = build_admin_tree_nodes(
        roots,
        None,
        &[],
        &mut group_paths,
        &mut group_count,
        &mut tag_count,
    );

    ProcessedAdminTagTree {
        tree_nodes,
        group_count,
        tag_count,
        group_paths,
    }
}

// ── WASM 导出 ──

#[wasm_bindgen]
pub fn process_tags(raw: JsValue) -> Result<JsValue, JsValue> {
    let groups: Vec<RawTagGroup> =
        serde_wasm_bindgen::from_value(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let tree = build_processed_tree(groups);

    serde_wasm_bindgen::to_value(&tree).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn process_admin_tags(raw: JsValue) -> Result<JsValue, JsValue> {
    let groups: Vec<RawTagGroup> =
        serde_wasm_bindgen::from_value(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let tree = build_processed_admin_tree(groups);

    serde_wasm_bindgen::to_value(&tree).map_err(|e| JsValue::from_str(&e.to_string()))
}
