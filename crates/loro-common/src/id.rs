use crate::{span::IdSpan, CounterSpan};

use super::{Lamport, LoroError, PeerID, ID};
const UNKNOWN: PeerID = 404;
use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
};

impl Debug for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}@{}", self.lamport, self.peer).as_str())
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}@{}", self.lamport, self.peer).as_str())
    }
}

impl TryFrom<&str> for ID {
    type Error = LoroError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.split('@').count() != 2 {
            return Err(LoroError::DecodeError("Invalid ID format".into()));
        }

        let mut iter = value.split('@');
        let counter = iter
            .next()
            .unwrap()
            .parse::<Lamport>()
            .map_err(|_| LoroError::DecodeError("Invalid ID format".into()))?;
        let client_id = iter
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|_| LoroError::DecodeError("Invalid ID format".into()))?;
        Ok(ID {
            peer: client_id,
            lamport: counter,
        })
    }
}

impl PartialOrd for ID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.peer.cmp(&other.peer) {
            core::cmp::Ordering::Equal => self.lamport.cmp(&other.lamport),
            ord => ord,
        }
    }
}

pub const ROOT_ID: ID = ID {
    peer: PeerID::MAX,
    lamport: i32::MAX,
};

impl From<u128> for ID {
    fn from(id: u128) -> Self {
        ID {
            peer: (id >> 64) as PeerID,
            lamport: id as Lamport,
        }
    }
}

impl ID {
    /// The ID of the null object. This should be use rarely.
    pub const NONE_ID: ID = ID::new(u64::MAX, 0);

    #[inline]
    pub const fn new(peer: PeerID, counter: Lamport) -> Self {
        ID {
            peer,
            lamport: counter,
        }
    }

    #[inline]
    pub fn new_root() -> Self {
        ROOT_ID
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.peer == PeerID::MAX
    }

    #[inline]
    pub fn to_span(&self, len: usize) -> IdSpan {
        IdSpan {
            client_id: self.peer,
            counter: CounterSpan::new(self.lamport, self.lamport + len as Lamport),
        }
    }

    #[inline]
    pub fn unknown(counter: Lamport) -> Self {
        ID {
            peer: UNKNOWN,
            lamport: counter,
        }
    }

    #[inline]
    pub fn is_unknown(&self) -> bool {
        self.peer == UNKNOWN
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn is_connected_id(&self, other: &Self, self_len: usize) -> bool {
        self.peer == other.peer && self.lamport + self_len as Lamport == other.lamport
    }

    #[inline]
    pub fn inc(&self, inc: i32) -> Self {
        ID {
            peer: self.peer,
            lamport: self.lamport + inc,
        }
    }

    #[inline]
    pub fn contains(&self, len: Lamport, target: ID) -> bool {
        self.peer == target.peer
            && self.lamport <= target.lamport
            && target.lamport < self.lamport + len
    }
}

impl From<ID> for u128 {
    fn from(id: ID) -> Self {
        ((id.peer as u128) << 64) | id.lamport as u128
    }
}

impl RangeBounds<ID> for (ID, ID) {
    fn start_bound(&self) -> std::ops::Bound<&ID> {
        std::ops::Bound::Included(&self.0)
    }

    fn end_bound(&self) -> std::ops::Bound<&ID> {
        std::ops::Bound::Excluded(&self.1)
    }
}
