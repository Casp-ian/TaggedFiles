# TaggedFiles

## Todo

- get basic functionality back to the level of the old version


- add option te remove connections
- add option to list tags
- make it faster (maybe rewrite in other language)
- order options based on last used
- make configurable
- check for files being deleted

## idea & inspiration
Original inspiration for this project was a post about [tag based file systems](https://garrit.xyz/posts/2024-04-02-fuck-trees-use-tags).
I wanted to make a way to access my files based on tags, but building an entire file system is out of my grasp for now, and i would also like some of my files to still be accesible from my normal file tree.

The current idea is to store paths of often used files and directories in a database and access them via tags.

The result will be quite similar to something like [zoxide](https://github.com/ajeetdsouza/zoxide).

There is another video i found about [tag studio](https://www.youtube.com/watch?v=wTQeMkYRMcw&t=3s), which has some ideas about sub tags that might be cool to look at later.

## working

### the script itself
Written in rust, not because its the best choice, but because i wanted to learn rust.

### the extra part
Because we cant change the current shells directory from the script we need to be a little creative.

This is solved by just calling `cd` on the result of the `getfile` option.

This is the function i added to my fish config
```
function tf
  /home/caspian/Projects/cli/TaggedFiles/taggedFiles.rb $argv
  eval (/home/caspian/Projects/cli/TaggedFiles/taggedFiles.rb --getCommand)
end
```

You will have to write an equivalent in bash yourself for now. Ill add an example here if someone shares one with me.
