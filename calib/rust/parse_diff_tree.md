# parse_diff_tree

`parse_diff_tree(output)` reads the output of `git diff-tree -r -z`, as bytes, into a list of changes, one per file.
A change has a side before and a side after, each a path and a blob, or none.

The output is fields between zero bytes; an empty field is skipped. It reads them in order, a file at a time, until
none are left:

1. The first field is the file's header. It must be UTF-8, else it fails with "unexpected `git diff-tree` output"
   and the header, its invalid bytes replaced.
2. Without its leading `:`, the header splits on spaces into exactly five parts: the mode before, the mode after,
   the blob before, the blob after and the status. Any other number of parts fails with the same error.
3. The next field is the path before. A rename or a copy (a status starting with `R` or `C`) has its path after in
   the field after that; any other change's path after is its path before. A missing path fails with "no path
   after" the header. A path is made from its field's bytes by `path_of`.
4. A side is there only for a regular file, its mode starting with `100`, and holds its path and its blob. A copy
   has no side before, whatever its mode: its source stays as it was.
5. The change, its two sides, goes into the list.

It returns the list.
