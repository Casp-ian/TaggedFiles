# TaggedFiles

# this is far from done, if you want something functional, use some of the projects mentioned in idea & inspiration

## how to build and run locally

note: if you get some error about missing `lsqlite3`, you need to install `libsqlite3-dev`


## Todo

- add option to remove files
- order files based on last used
- add special tags (like: link, dir, hidden and image)
- add subcommand to check for manually added and removed files in directory
- add subtags with `tag/childtag+otherchildtag-excludedchildtag` syntax
- add exclude tags `-excluded` syntax

## idea & inspiration
Original inspiration for this project was a post about [tag based file systems](https://garrit.xyz/posts/2024-04-02-fuck-trees-use-tags).
I wanted to make a way to access my files based on tags, but building an entire file system is out of my grasp for now, and i would also like some of my files to still be accesible from my normal file tree.

The current idea is to store paths of often used files and directories in a database and access them via tags.

The result will be quite similar to something like [zoxide](https://github.com/ajeetdsouza/zoxide).

There is another video i found about [tag studio](https://www.youtube.com/watch?v=wTQeMkYRMcw&t=3s), which has some ideas about sub tags that might be cool to look at later.

## working

### the script itself
Written in rust, not because its the best choice, but because i wanted to learn rust. (and ruby was a little slow)

### the extra part
Because we cant change the current shells directory from the script we need to be a little creative.

This is solved by just calling `cd` on the result of the `getfile` option.

This is the function i added to my fish config
```fish
function tf
    # Path to the command
    set command /mnt/18ac960b-6d14-4864-ba13-cd7adb12357e/Projects/cli/TaggedFiles/target/release/TaggedFiles

    if [ $argv[1] = hx ]
        hx ($command getfile $argv[2..-1])
    else if [ $argv[1] = cd ]
        cd ($command getfile $argv[2..-1])
    else if [ $argv[1] = files ]
        nautilus ($command getfile $argv[2..-1])
    else if [ $argv[1] = echo ]
        echo ($command getfile $argv[2..-1])
    else
        # Try to call taggedFiles with original arguments to continue as if tf is an alias
        $command $argv[1..-1]
    end
end
```

For bash you will have to write an equivalent yourself for now. I will add an example here if someone shares one with me.
