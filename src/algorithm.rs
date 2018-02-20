use std_prelude::*;
use std::path::Component;

// NOTE: The `clean_path` function is adapted from rust-lang/rust PR #47363
//       which is copyright 2018 The Rust Developers, licensed under either
//       the Apache License, Version 2.0, or the MIT license.

/// Returns a cleaned representation of the path with all current
/// directory (.) and parent directory (..) references resolved.
///
/// This is a purely logical calculation; the file system is not accessed. Namely,
/// this leaves symbolic links intact and does not validate that the target exists.
///
/// This may change the meaning of a path involving symbolic links and parent directory
/// references.
///
/// # Examples
///
/// ```ignore
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("/recipes/./snacks/../desserts/banana_creme_pie.txt");
/// assert_eq!(clean_path(path), PathBuf::from("/recipes/desserts/banana_creme_pie.txt"));
/// let path = Path::new("../.././lots///of////./separators/");
/// assert_eq!(clean_path(path), PathBuf::from("../../lots/of/separators"));
/// let path = Path::new("/../../../cannot_go_above_root");
/// assert_eq!(clean_path(path), PathBuf::from("/cannot_go_above_root"));
/// ```
pub(crate) fn clean_path(path: &Path) -> PathBuf {
    let mut stack: Vec<Component> = vec![];

    // We assume .components() removes redundant consecutive path separators.
    // Note that .components() also does some normalization of '.' on its own anyways.
    // This '.' normalization happens to be compatible with the approach below.
    for component in path.components() {
        match component {
            // Drop CurDir components, do not even push onto the stack.
            Component::CurDir => {},

            // For ParentDir components, we need to use the contents of the stack.
            Component::ParentDir => {
                // Look at the top element of stack, if any.
                let top = stack.last().cloned();

                match top {
                    // A component is on the stack, need more pattern matching.
                    Some(c) => {
                        match c {
                            // Push the ParentDir on the stack.
                            Component::Prefix(_) => { stack.push(component); },

                            // The parent of a RootDir is itself, so drop the ParentDir (no-op).
                            Component::RootDir => {},

                            // A CurDir should never be found on the stack,
                            // since they are dropped when seen.
                            Component::CurDir => { unreachable!(); },

                            // If a ParentDir is found, it must be due to it
                            // piling up at the start of a path.
                            // Push the new ParentDir onto the stack.
                            Component::ParentDir => { stack.push(component); },

                            // If a Normal is found, pop it off.
                            Component::Normal(_) => { let _ = stack.pop(); }
                        }
                    },

                    // Stack is empty, so path is empty, just push.
                    None => { stack.push(component); }
                }
            },

            // All others, simply push onto the stack.
            _ => { stack.push(component); },
        }
    }

    // If an empty PathBuf would be returned, instead return CurDir ('.').
    if stack.is_empty() {
        return PathBuf::from(Component::CurDir.as_os_str());
    }

    let mut norm_path = PathBuf::new();

    for item in &stack {
        norm_path.push(item.as_os_str());
    }

    norm_path
}
