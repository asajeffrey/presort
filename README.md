# presort

This crate provides a simple interface for mutable arrays with sorting.

It is optimize for the case where array update does not change the sort order, in which case sorting is O(1).