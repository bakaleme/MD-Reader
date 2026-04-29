use crate::{markdown::is_markdown, state::AppState};
use dioxus::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<AppState>();

    let files = use_resource(move || async move {
        let Some(dir) = state.current_dir.read().clone() else {
            return Vec::new();
        };
        let mut found = Vec::new();
        let mut visited = HashSet::new();
        let _ = find_markdown_files(&dir, &mut found, &mut visited).await;
        found.sort();
        found
    });

    let expanded = use_signal(HashSet::<PathBuf>::new);

    rsx! {
        div {
            class: "flex flex-col w-72 min-w-[18rem] max-w-md shrink-0 h-full overflow-hidden border-r bg-slate-50 dark:bg-slate-900 border-slate-200 dark:border-slate-800",
            if let Some(ref dir) = *state.current_dir.read() {
                div {
                    class: "flex items-center gap-2 px-4 h-10 border-b shrink-0 bg-slate-50 dark:bg-slate-900 border-slate-200 dark:border-slate-800",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "14", height: "14", view_box: "0 0 24 24",
                        fill: "none", stroke: "currentColor", stroke_width: "2",
                        stroke_linecap: "round", stroke_linejoin: "round",
                        class: "shrink-0 opacity-50",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "m21 21-4.3-4.3" }
                    }
                    input {
                        class: "bg-transparent outline-none text-sm w-full text-slate-900 dark:text-slate-100 placeholder-slate-400",
                        r#type: "text",
                        placeholder: "Filter files...",
                        value: "{state.search_query}",
                        oninput: move |e| state.search_query.set(e.value()),
                    }
                    if !state.search_query.read().is_empty() {
                        button {
                            class: "text-slate-400 hover:text-slate-600 dark:hover:text-slate-300",
                            onclick: move |_| state.search_query.set(String::new()),
                            "×"
                        }
                    }
                }

                div { class: "flex-1 overflow-y-auto p-2",
                    {match files.read().as_ref() {
                        Some(file_list) => {
                            let query = state.search_query.read().to_lowercase();
                            if query.is_empty() {
                                let tree = build_tree(file_list.clone(), dir);
                                rsx! {
                                    ul { class: "space-y-0.5",
                                        for node in tree {
                                            TreeNodeView {
                                                key: "{node.path.display()}",
                                                node,
                                                expanded,
                                                depth: 0,
                                            }
                                        }
                                    }
                                }
                            } else {
                                let filtered: Vec<_> = file_list.iter()
                                    .filter(|p| p.to_string_lossy().to_lowercase().contains(&query))
                                    .cloned()
                                    .collect();
                                if filtered.is_empty() {
                                    rsx! { div { class: "text-sm opacity-50 text-center py-4", "No files match your search." } }
                                } else {
                                    rsx! {
                                        ul { class: "space-y-0.5",
                                            for file in filtered.iter() {
                                                FlatFileItem {
                                                    key: "{file.display()}",
                                                    root: dir.clone(),
                                                    file: file.clone(),
                                                }
                                            }
                                        }
                                        div { class: "text-xs opacity-40 text-center mt-2", "{filtered.len()} result{if filtered.len() != 1 { "s" }}" }
                                    }
                                }
                            }
                        }
                        None => rsx! {
                            div { class: "flex flex-col items-center justify-center py-8 gap-2",
                                div { class: "w-6 h-6 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" }
                                p { class: "text-sm opacity-50", "Scanning..." }
                            }
                        },
                    }}
                }
            } else {
                div { class: "flex-1 flex flex-col items-center justify-center p-6 text-center opacity-50",
                    p { class: "text-sm mb-1", "No folder open." }
                    p { class: "text-xs", "Press Ctrl+O to browse." }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct TreeNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    children: Vec<TreeNode>,
}

#[component]
fn TreeNodeView(node: TreeNode, expanded: Signal<HashSet<PathBuf>>, depth: usize) -> Element {
    let state = use_context::<AppState>();

    if !node.is_dir {
        let is_selected = state.selected_file.read().as_ref() == Some(&node.path);
        return rsx! {
            li {
                class: "flex items-center gap-2 px-2 py-1.5 rounded-md cursor-pointer text-sm",
                class: if is_selected {
                    "bg-blue-600 text-white"
                } else {
                    "text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
                },
                style: "padding-left: calc(0.5rem + {depth * 14}px)",
                title: "{node.path.display()}",
                onclick: move |_| state.select_file(node.path.clone()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "14", height: "14", view_box: "0 0 24 24",
                    fill: "none", stroke: "currentColor", stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    class: "shrink-0 opacity-60",
                    path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                    polyline { points: "14 2 14 8 20 8" }
                }
                span { class: "truncate", "{node.name}" }
            }
        };
    }

    let is_expanded = expanded.read().contains(&node.path);
    let has_children = !node.children.is_empty();

    rsx! {
        li {
            div {
                class: "flex items-center gap-1.5 px-2 py-1.5 rounded-md cursor-pointer text-sm select-none text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800",
                style: "padding-left: calc(0.5rem + {depth * 14}px)",
                onclick: move |_| {
                    if has_children {
                        expanded.with_mut(|set| {
                            if is_expanded {
                                set.remove(&node.path);
                            } else {
                                set.insert(node.path.clone());
                            }
                        });
                    }
                },
                if has_children {
                    span { class: "text-[10px] w-3 shrink-0", if is_expanded { "▼" } else { "▶" } }
                } else {
                    span { class: "w-3 shrink-0" }
                }
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "14", height: "14", view_box: "0 0 24 24",
                    fill: "none", stroke: "currentColor", stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    class: "shrink-0 opacity-60",
                    path { d: "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2z" }
                }
                span { class: "truncate font-medium", "{node.name}" }
            }
            if is_expanded {
                ul { class: "space-y-0.5",
                    for child in node.children.iter().cloned() {
                        TreeNodeView {
                            key: "{child.path.display()}",
                            node: child,
                            expanded,
                            depth: depth + 1,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FlatFileItem(root: PathBuf, file: PathBuf) -> Element {
    let state = use_context::<AppState>();
    let rel = file.strip_prefix(&root).unwrap_or(&file);
    let is_selected = state.selected_file.read().as_ref() == Some(&file);

    rsx! {
        li {
            class: "flex items-center gap-2 px-2 py-1.5 rounded-md cursor-pointer text-sm",
            class: if is_selected {
                "bg-blue-600 text-white"
            } else {
                "text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            },
            style: "padding-left: 0.5rem",
            title: "{file.display()}",
            onclick: move |_| state.select_file(file.clone()),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "14", height: "14", view_box: "0 0 24 24",
                fill: "none", stroke: "currentColor", stroke_width: "2",
                stroke_linecap: "round", stroke_linejoin: "round",
                class: "shrink-0 opacity-60",
                path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                polyline { points: "14 2 14 8 20 8" }
            }
            span { class: "truncate", "{rel.display()}" }
        }
    }
}

async fn find_markdown_files(
    dir: &Path,
    out: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
) {
    let canonical = match dir.canonicalize() {
        Ok(c) => c,
        Err(_) => return,
    };
    if !visited.insert(canonical) {
        return;
    }

    let mut entries = match tokio::fs::read_dir(dir).await {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut dirs = Vec::new();

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        } else if is_markdown(&path) {
            out.push(path);
        }
    }

    for subdir in dirs {
        Box::pin(find_markdown_files(&subdir, out, visited)).await;
    }
}

fn build_tree(files: Vec<PathBuf>, root: &Path) -> Vec<TreeNode> {
    let mut root_node = TreeNode {
        name: String::new(),
        path: root.to_path_buf(),
        is_dir: true,
        children: Vec::new(),
    };

    for file in files {
        let rel = match file.strip_prefix(root) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let mut current = &mut root_node;
        let mut current_path = root.to_path_buf();
        let components: Vec<_> = rel.components().collect();

        for (i, comp) in components.iter().enumerate() {
            let name = comp.as_os_str().to_string_lossy().to_string();
            current_path.push(comp);
            let is_last = i == components.len() - 1;

            let pos = current.children.iter().position(|c| {
                c.name == name && c.is_dir == !is_last
            });

            match pos {
                Some(idx) => {
                    current = &mut current.children[idx];
                }
                None => {
                    let new_node = TreeNode {
                        name: name.clone(),
                        path: current_path.clone(),
                        is_dir: !is_last,
                        children: Vec::new(),
                    };
                    current.children.push(new_node);
                    let idx = current.children.len() - 1;
                    current = &mut current.children[idx];
                }
            }
        }
    }

    sort_tree(&mut root_node.children);
    root_node.children
}

fn sort_tree(nodes: &mut [TreeNode]) {
    nodes.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    for node in nodes.iter_mut() {
        sort_tree(&mut node.children);
    }
}
