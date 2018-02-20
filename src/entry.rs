/* Copyright (c) 2018 Garrett Berg, vitiral@gmail.com
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */
//! The absolute path type, the root type for _most_ `Path*` types in this module
//! (except for `PathArc`).
use std::fmt;
use std::fs;
use std_prelude::*;

use super::{Error, Result};
use super::{PathArc, PathAbs, PathDir, PathFile, PathSymlink};

// FIXME
// OPEN QUESTIONS:
// * What are the advantages of PathAbs having canonicalized paths?
//   Do any of those advantages still apply for the weaker canonicalization scheme in PathEntry?
// * If not, could something else be gained by further loosening the restrictions on PathEntry?
//   Possible choices of invariants: (can pick multiple)
//   - The path is absolute (but not necessarily canonical in any way)
//   - The path was verified to exist at some point in the past.

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// An canonicalized form of a directory entry that is guaranteed (when created) to exist.
///
/// `PathEntry` is slightly more general than [`PathAbs`] in that it is capable of
/// representing a symbolic link. To this end, it uses a slightly weaker form of
/// canonicalization in that a symbolic link in the final component is not
/// automatically followed.  This is implemented in [`canonicalize_entry`].
///
/// To clarify, when it is said that the entry at some point "existed," it is not meant that
/// [`exists`] returned `true` (because a `exists` may return false for a symlink to a missing
/// file).  Rather, it means that [`symlink_metadata`] returned `Some(_)`.
///
/// [`PathAbs`]: struct.PathAbs.html
/// [`canonicalize_entry`]: struct.PathArc.html#method.canonicalize_entry
/// [`exists`]: #method.exists
/// [`symlink_metadata`]: #method.symlink_metadata
pub struct PathEntry(pub(crate) PathArc);

impl PathEntry {
    /// Instantiate a new `PathEntry`, canonicalizing it via [`canonicalize_entry`].
    ///
    // TODO FIXME EXAMPLE
    ///
    /// [`canonicalize_entry`]: struct.PathArc.html#method.canonicalize_entry
    pub fn new<P: AsRef<Path>>(path: P) -> Result<PathEntry> {
        PathArc::new(path).canonicalize_entry()
    }

    // TODO FIXME DOC
    pub fn entry_join<P: AsRef<Path>>(&self, path: P) -> Result<PathEntry> {
        self.join(path).canonicalize_entry()
    }

    /// Resolve the `PathEntry` as a `PathFile`. Return an error if it is anything
    /// else (such as a file or symlink).
    pub fn into_file_strict(self) -> Result<PathFile> {
        PathFile::from_entry_strict(self)
    }

    /// Resolve the `PathEntry` as a `PathDir`. Return an error if it is anything
    /// else (such as a directory or symlink).
    pub fn into_dir_strict(self) -> Result<PathDir> {
        PathDir::from_entry_strict(self)
    }

    /// Resolve the `PathAbs` as a `PathSymlink`. Return an error if it is not a symlink.
    pub fn into_symlink(self) -> Result<PathSymlink> {
        PathSymlink::from_entry(self)
    }

    /// Get the parent directory of this path as a `PathDir`.
    ///
    /// > This does not make additional syscalls, as the parent by definition must be a directory
    /// > and exist.
    ///
    // TODO FIXME EXAMPLE
    pub fn parent_dir(&self) -> Option<PathDir> {
        match self.parent() {
            Some(p) => Some(PathDir(PathAbs(PathArc::new(p)))),
            None => None,
        }
    }

    /// Rename (move) a directory entry.
    ///
    /// This corresponds to [`fs::rename`].  The precise semantics when `to` already exists
    /// may differ based on the operating system and type of directory entry.
    ///
    /// This will not work if the new name is on a different mount point.
    ///
    // FIXME TODO EXAMPLE
    pub fn rename<P: AsRef<Path>>(self, to: P) -> Result<PathSymlink> {
        fs::rename(&self, &to).map_err(|err| {
            Error::new(
                err,
                &format!("renaming to {} from", to.as_ref().display()),
                self.clone().into(),
            )
        })?;
        Ok(PathSymlink::new(to)?)
    }

    /// Return a reference to a basic `std::path::Path`
    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }

    /// For constructing mocked paths during tests. This is effectively the same as a `PathBuf`.
    ///
    /// This is NOT checked for validity so the file may or may not actually exist and will
    /// NOT be, in any way, an absolute or canonicalized path.
    ///
    // TODO FIXME EXAMPLE
    pub fn mock<P: AsRef<Path>>(fake_path: P) -> PathEntry {
        PathEntry(PathArc::new(fake_path))
    }
}

impl fmt::Debug for PathEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<PathArc> for PathEntry {
    fn as_ref(&self) -> &PathArc {
        &self.0
    }
}

impl AsRef<Path> for PathEntry {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<PathBuf> for PathEntry {
    fn as_ref(&self) -> &PathBuf {
        self.0.as_ref()
    }
}

impl Borrow<PathArc> for PathEntry {
    fn borrow(&self) -> &PathArc {
        self.as_ref()
    }
}

impl Borrow<Path> for PathEntry {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathEntry {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<'a> Borrow<PathArc> for &'a PathEntry {
    fn borrow(&self) -> &PathArc {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathEntry {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathEntry {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl Deref for PathEntry {
    type Target = PathArc;

    fn deref(&self) -> &PathArc {
        &self.0
    }
}

impl Into<PathArc> for PathEntry {
    fn into(self) -> PathArc {
        self.0
    }
}
