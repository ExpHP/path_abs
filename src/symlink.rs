/* Copyright (c) 2018 Garrett Berg, vitiral@gmail.com
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */
use std::fs;
use std::fmt;
use std::io;
use std_prelude::*;

use super::{Error, Result};
use super::{PathArc, PathEntry};

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// a `PathAbs` that was a symbolic link at the time of initialization, with associated methods.
pub struct PathSymlink(pub(crate) PathEntry);

impl PathSymlink {
    /// Instantiate a new `PathSymlink`. The file must exist or `io::Error` will be returned.
    ///
    /// Returns `io::ErrorKind::InvalidInput` if the path exists but is not a file.
    ///
    // TODO FIXME EXAMPLE
    pub fn new<P: AsRef<Path>>(path: P) -> Result<PathSymlink> {
        PathEntry::new(path).and_then(PathSymlink::from_entry)
    }

    /// Consume the `PathEntry`, validating that the path is a symlink and returning `PathSymlink`.
    /// The file must exist or `io::Error` will be returned.
    ///
    /// If the path is something else, returns `io::ErrorKind::InvalidInput`.
    ///
    /// > This does not call [`PathArc::canonicalize_entry`], instead trusting that the input is
    /// > already canonicalized in this manner.
    ///
    /// [`PathArc::canonicalize_entry`]: struct.PathArc.html#method.canonicalize_entry
    ///
    // TODO FIXME EXAMPLE
    pub fn from_entry(entry: PathEntry) -> Result<PathSymlink> {
        if entry.symlink_metadata()?.file_type().is_symlink() {
            Ok(PathSymlink::from_entry_unchecked(entry))
        } else {
            Err(Error::new(
                io::Error::new(io::ErrorKind::InvalidInput, "path is not a link"),
                "resolving",
                entry.into(),
            ))
        }
    }

    #[inline(always)]
    /// Do the conversion _without checking_.
    ///
    /// This is typically used by external libraries when the type is already known
    /// through some other means (to avoid a syscall).
    pub fn from_entry_unchecked(entry: PathEntry) -> PathSymlink {
        PathSymlink(entry)
    }

    // TODO FIXME test
    // TODO FIXME examples
    /// Get the target path of the symlink.
    ///
    /// This differs from [`read_link`] in that relative symlinks are corrected if necessary
    /// by attaching the directory stem of the input path.
    ///
    /// [`read_link`]: #method.read_link
    pub fn target(&self) -> Result<PathArc> {
        // TODO FIXME does this work on windows?
        self.read_link().map(|link| self.with_file_name(link.as_path()))
    }

    /// Visit the target of this symlink, which must exist.
    ///
    /// Note that the target could also be a symlink, which is why this returns
    /// `PathEntry` and not [`PathAbs`].  To fully resolve a chain of symlinks,
    /// use [`canonicalize`].
    ///
    /// [`PathAbs`]: struct.PathAbs.html
    /// [`canonicalize`]: #method.canonicalize
    pub fn follow(&self) -> Result<PathEntry> {
        self.target().and_then(|path| path.canonicalize_entry())
    }

    /// Rename a symlink with the same behavior as [`std::fs::rename`].
    ///
    /// Be aware that renaming a symlink to a location in another directory
    /// may break the connection between the symlink and its target.
    // TODO FIXME EXAMPLES
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

    // TODO FIXME review
    /// Remove (delete) the link from the filesystem, consuming self.
    // TODO FIXME examples
    pub fn remove(self) -> Result<()> {
        fs::remove_file(&self).map_err(|err| Error::new(err, "removing", self.into()))
    }

    /// Return a reference to a basic `std::path::Path`
    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }

    /// Create a mock symlink type. *For use in tests only*.
    ///
    /// See the docs for [`PathAbs::mock`](struct.PathAbs.html#method.mock)
    pub fn mock<P: AsRef<Path>>(path: P) -> PathSymlink {
        PathSymlink(PathEntry::mock(path))
    }
}

impl fmt::Debug for PathSymlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

// NOTE: Impossible. PathAbs has stronger invariants.
// impl AsRef<PathAbs> for PathSymlink

impl AsRef<PathEntry> for PathSymlink {
    fn as_ref(&self) -> &PathEntry {
        &self.0
    }
}

impl AsRef<PathArc> for PathSymlink {
    fn as_ref(&self) -> &PathArc {
        &self.0
    }
}

impl AsRef<Path> for PathSymlink {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<PathBuf> for PathSymlink {
    fn as_ref(&self) -> &PathBuf {
        self.0.as_ref()
    }
}

// NOTE: Impossible. PathAbs has stronger invariants.
// impl Borrow<PathAbs> for PathSymlink

impl Borrow<PathEntry> for PathSymlink {
    fn borrow(&self) -> &PathEntry {
        self.as_ref()
    }
}

impl Borrow<PathArc> for PathSymlink {
    fn borrow(&self) -> &PathArc {
        self.as_ref()
    }
}

impl Borrow<Path> for PathSymlink {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathSymlink {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

// NOTE: Impossible. PathAbs has stronger invariants.
// impl<'a> Borrow<PathAbs> for &'a PathSymlink

impl<'a> Borrow<PathEntry> for &'a PathSymlink {
    fn borrow(&self) -> &PathEntry {
        self.as_ref()
    }
}

impl<'a> Borrow<PathArc> for &'a PathSymlink {
    fn borrow(&self) -> &PathArc {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathSymlink {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathSymlink {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl Deref for PathSymlink {
    type Target = PathEntry;

    fn deref(&self) -> &PathEntry {
        &self.0
    }
}

// NOTE: Impossible. PathAbs has stronger invariants.
// impl Into<PathAbs> for PathSymlink;

impl Into<PathEntry> for PathSymlink {
    /// Downgrades the `PathSymlink` into a `PathEntry`
    fn into(self) -> PathEntry {
        self.0.into()
    }
}

impl Into<PathArc> for PathSymlink {
    /// Downgrades the `PathSymlink` into a `PathArc`
    fn into(self) -> PathArc {
        (self.0).0
    }
}

impl Into<PathBuf> for PathSymlink {
    /// Downgrades the `PathSymlink` into a `PathBuf`. Avoids a clone if this is the only reference.
    ///
    // TODO FIXME examples
    fn into(self) -> PathBuf {
        let arc: PathArc = self.into();
        arc.into()
    }
}
