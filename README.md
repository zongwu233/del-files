# del-files

a mini tool for deleting target dir files recursively.

## use

`git clone `

`cargo build`

if you want to delete all `target` dir for  `/home/my-rust-project` .

`cargo run  /home/my-rust-project`

will find and delete `target` dir in `/home/my-rust-project` recursively.

You can set custom dir name. For example:

`cargo run  /home/my-rust-project build`

will clear all `build` dir in  `/home/my-rust-project` recursively.

## More Information

NOTE: this program can run in Linux and macOS, but NOT adapted for windows. 

## License

- [MIT license](https://github.com/zongwu233/del-files/blob/master/LICENSE)

