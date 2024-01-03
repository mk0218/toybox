use leptos::*;

#[derive(Clone)]
struct TreeNodeInfo {
    label: String,
    children: Vec<RwSignal<TreeNodeInfo>>
}

impl TreeNodeInfo {
    fn new(label: &str, children: Vec<RwSignal<TreeNodeInfo>>) -> Self {
        TreeNodeInfo { label: String::from(label), children }
    }
}

#[component]
fn TreeNode(node: TreeNodeInfo) -> impl IntoView {
    let children: Vec<_> = node.children.iter().map(|n| view! {
        <TreeNode node={n.get()} />
    }).collect();

    view! {
        <div>
            {node.label}
            {children}
        </div>
    }
}

#[component]
fn Tree(nodes: Vec<RwSignal<TreeNodeInfo>>) -> impl IntoView {
    let children: Vec<_> = nodes.iter().map(|n| view! {
        <TreeNode node={n.get()} />
    }).collect();

    view! {
        <div>{children}</div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let tree = vec![
        RwSignal::new(TreeNodeInfo::new("1", vec![RwSignal::new(TreeNodeInfo::new("1-1", vec![]))])),
        RwSignal::new(TreeNodeInfo::new("2", vec![])),
        RwSignal::new(TreeNodeInfo::new("3", vec![])),
    ];
    view! {
        <Tree nodes={tree} />
    }
}