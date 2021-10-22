use crate::node::{Node, NodeChild, NodeType};

pub fn walk<T: Fn(&Node)>(node: &Node, cb: T) {
    match &node.children {
        Box(NodeChild::Node(child), ..) => {
            cb(node);
            walk(child, cb);
        }
        Box(NodeChild::KV(x, ..), ..) => {
            cb(node);
            // let y = x as { key: Node, value: Node };
            // walk(&child., cb);
        }
        Box(NodeChild::List(list), ..) => {
            cb(node);
            for n in list {
                cb(n, &cb);
            }
        }
        Box(NodeChild::String(str), ..) => {
            cb(&node);
        }
        Box(NodeChild::Unknown(), ..) => {
            println!("Unknown node");
        }
    }


    match node.node_type {
        NodeType::Root => {
            let child = &node.children as NodeChild::Node(node);
            cb(node);
            walk(child, cb);
        }
        NodeType::ObjectExpression => {}
        NodeType::ObjectProperty => {
            let property = &node.children as NodeChild::KV();
            walk(node.children[0], cb);
            walk(node.children[1], cb);
        }
        NodeType::ArrayExpression => {}
        _ => cb(node)
    }

    /*if (node.type == NodeType::Root) {
        walk(node.children[0], cb, context);
    } else if (node.type == NodeType::ObjectExpression) {
        int size = node.children.size();
        int i = 0;
        while (i < size) {
            walk(node.children[i], cb, context);
            i++;
        }
    } else if (node.type == NodeType::ObjectProperty) {
        walk(node.children[0], cb, context);
        walk(node.children[1], cb, context);
    } else if (node.type == NodeType::ArrayExpression) {
        int size = node.children.size();
        int i = 0;
        while (i < size) {
            walk(node.children[i], cb, context);
            i++;
        }
    }*/
}