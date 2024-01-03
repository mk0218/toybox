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
macro_rules! tree {
    {} => { Vec::new::<Vec<RwSignal<TreeNodeInfo>>>() }
}
macro_rules! tree {
    [$({$l:literal, $e: expr}),*] => {
        {
            let mut tree: Vec<RwSignal<TreeNodeInfo>> = vec![];
        
            $(
                tree.push(RwSignal::new(TreeNodeInfo::new($l, $e)));
            )*
            
            tree
    }
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
    let tree = tree![
        {"1", tree![{"1-1", tree![]}, {"1-2", tree![]}]},
        {"2", tree![]},
        {"3", tree![]}
    ];
    view! {
        <Tree nodes={tree} />
    }
}