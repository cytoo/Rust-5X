#Rust-5X
## a multi function telegram bot made in rust

by [cytolytic](https://t.me/cytolytic)

### what commands does it have?

#### it includes but is not limited to:

```rust
enum Cmds
{
    Kick, // used for kicking the user that you reply to
    
    Help, // shows the help menu
    
    Ping, // does a quick speed test
    
    Del, // deletes the message that you reply to

    Admins, // lists the admins in the group

    Pin, // pin the message that you reply to

    Leave, // leaves the group the function was called on

    GSI, // makes an erfan gsi

    SH, // does a bash command

    Info, // gets the information of the user that you replyto
}
```

## how to set it up?
#### super easy to do!

first of all you need to have rust installed! you can install it here [here](https://www.rust-lang.org/tools/install) <br><br>
now you'll need to setup some environment variables 
```
// for windows

set TELOXIDE_TOKEN=<your token here>
set OWNER_ID=<your own telegram id>

//anything else

export TELOXIDE_TOKEN=<your token here>
export OWNER_ID=<your own telegram id>
```
now follow thees setps <br>
`git clone https://github.com/aktham3210/Rust-5X` <br> <br>
`cd Rust-5X`<br><br>
`cargo run`<br>

##### and then you're good to go!. hope you enjoy it :)
