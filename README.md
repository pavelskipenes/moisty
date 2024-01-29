# moisty

moisty is a hobby project / library that aims to implement different tools used in swimming. It varies from device drivers to file parsers. For now it only parses `meetsetup.xml` files which looks to be a save dump of JechSoft Victoria. By having the parser implemented it ended up being a pretty decent validator for those files because the original software does not perform such checks as strictly.

## TODO

### jechsoft

- [ ] parse `meetsetup.xml` file. This file contains meet setup.
- [ ] serialize `meetsetup.xml` file to json, because why not?
- [ ] parse `meetresult.xml`. This file contains meet results.
- [ ] serialize `meetresult.xml` file to json, because why not?
- [ ] parse `uni_p.txt`. This file contains meet enrollment information
- [ ] parse `tryggivann.csv` exports.

## Contributing

If you are bored feel free to tweak on this as well.

- use rust nighlty
- download some meets of the internet `cargo run -cargo run -- --download`
- `cargo run` will run in `/moisty/src/main.rs` which will just try to parse every file in `/assets/meets/*.xml`
- this will cause some parsing errors like this:

```
   Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/moisty`
[ERROR][førdestemnet_2023.xml]: custom: missing field `Sessions`
[ERROR][årdalsstemne_rekrutt_2024.xml]: Events.Event[0].SesId: Expected token EndElement, found Characters
[ERROR][årdalsstemnet_2024.xml]: custom: missing field `Sessions`
[ERROR][saltenaqua_medley_5.xml]: custom: missing field `Sessions`
[ERROR][borås_craft_meet_2024.xml]: Sessions.Session[0].SessionStartTime: custom: received an empty string, expected a 4 char long string containing numbers representing time of day formatted as 24 hour ('hhmm') with leading zeroes. Min value '0000' max value '2359'.
[ERROR][vårduppen_rekrutt.xml]: Events.Event[0].SesId: Expected token EndElement, found Characters
[ERROR][vårduppen.xml]: custom: missing field `Sessions`
[ERROR][kronborg_open_2024.xml]: Sessions.Session[0].SessionStartTime: custom: received an empty string, expected a 4 char long string containing numbers representing time of day formatted as 24 hour ('hhmm') with leading zeroes. Min value '0000' max value '2359'.
[INFO]: 22/30 successfully meet files parsed
```

- user facing cli tools can be created in `/moisty/src`
- libraries that interface with jechsoft can be placed in `/jechsoft/src/<project>`
