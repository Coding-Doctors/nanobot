use serenity::client::Context;
use serenity::model::Message;
use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::env;
use ::store::{CommandCounter, EventCounter};

command!(commands(context, _message, _args) {
    let list = {
        let mut s = "Commands used:\n".to_owned();

        let data = context.data.lock().unwrap();
        let counter = data.get::<CommandCounter>().unwrap();

        for (k, v) in counter.iter().collect::<BTreeMap<_, _>>() {
            let _ = write!(s, "- {name}: {amount}\n", name=k, amount=v);
        }

        s
    };

    let _ = context.say(&list);
});

command!(eval(context, message, args) {
    let query = args.join(" ");

    let s = {
        let mut runnable = match File::open("./runnable.rs") {
            Ok(runnable) => runnable,
            Err(_) => {
                let _ = context.say("Err opening runnable");

                return Ok(());
            },
        };

        let mut s = String::new();
        let _ = runnable.read_to_string(&mut s);


        s = s.replace("{CHANNEL_ID}", &message.channel_id.0.to_string())
            .replace("{CODE}", &query);

        s
    };

    let id = message.id.0.to_string();

    {
        let mut f = File::create(&id).expect("err creating runnable");

        let _ = f.write(s.as_bytes());
    }

    let path = format!("./out_{}", id);

    {
        let command = Command::new("rustc")
            .arg(&id)
            .arg("--crate-name")
            .arg("runner")
            .arg("-L")
            .arg("target/debug/deps")
            .arg("-o")
            .arg(&path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();

        if let Err(why) = command {
            let _ = context.say(&format!("Err creating file: {:?}", why));

            return Ok(());
        }
    }

    if let Err(why) = Command::new(&path).output() {
        let _ = context.say(&format!("Err running program: {:?}", why));
    }

    let _ = fs::remove_file(id);
    let _ = fs::remove_file(path);
});

command!(events(context) {
    let list = {
        let mut s = "Events received:\n".to_owned();

        let data = context.data.lock().unwrap();
        let counter = data.get::<EventCounter>().unwrap();

        for (k, v) in counter.iter().collect::<BTreeMap<_, _>>() {
            let _ = write!(s, "- {name}: {amount}\n", name=k, amount=v);
        }

        s
    };

    let _ = context.say(&list);
});

command!(set_name(context, message, args) {
    if args.is_empty() {
        let _ = message.reply("No name given");

        return Ok(());
    }

    let name = args.join(" ");

    let _ = match context.edit_profile(|p| p.username(&name)) {
        Ok(_) => message.reply(":ok_hand:"),
        Err(why) => {
            warn!("Err setting name: {:?}", why);

            message.reply(":x: Error editing name")
        },
    };
});

command!(set_status(context, message, args) {
    let author_id = match env::var("AUTHOR_ID").map(|x| x.parse::<u64>()) {
        Ok(Ok(author_id)) => author_id,
        _ => {
            error!("Valid AUTHOR_ID env var not set");

            return Ok(());
        },
    };

    if message.author.id != author_id {
        return Ok(());
    }

    context.set_game_name(&args.join(" "));
});
