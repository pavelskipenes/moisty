# moisty

moisty is a hobby project / library that aims to implement different tools used in swimming. It varies from device drivers to file parsers. For now it only parses `meetsetup.xml` files which looks to be a save dump of JechSoft Victoria. By having the parser implemented it ended up being a pretty decent validator for those files because the original software does not perform such checks as strictly.

## Examples

Get registered events from a `meetsetup.xml` file:

```
~/Repositories/Personal/moisty> cargo run -- -i -m `~/Repositories/Meets/2024/Atlantic Race/Approbert/Stevneoppsett/meetsetup.xml`
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/moisty -i -m '/home/pavel/Repositories/Meets/2024/Atlantic Race/Approbert/Stevneoppsett/meetset
up.xml'`
╭────┬──────────┬───────────────────┬──────────────┬────────────╮
│ id │ distance │ style             │ gender_group │ date       │
├────┼──────────┼───────────────────┼──────────────┼────────────┤
│ 1  │ 100m     │ butterfly         │ female       │ 2024-03-02 │
│ 2  │ 100m     │ butterfly         │ male         │ 2024-03-02 │
│ 3  │ 50m      │ freestyle         │ female       │ 2024-03-02 │
│ 4  │ 50m      │ freestyle         │ male         │ 2024-03-02 │
│ 5  │ 100m     │ backstroke        │ female       │ 2024-03-02 │
│ 6  │ 100m     │ backstroke        │ male         │ 2024-03-02 │
│ 7  │ 50m      │ breaststroke      │ female       │ 2024-03-02 │
│ 8  │ 50m      │ breaststroke      │ male         │ 2024-03-02 │
│ 9  │ 100m     │ individual medley │ female       │ 2024-03-02 │
│ 10 │ 100m     │ individual medley │ male         │ 2024-03-02 │
│ 11 │ 100m     │ freestyle         │ female       │ 2024-03-02 │
│ 12 │ 100m     │ freestyle         │ male         │ 2024-03-02 │
│ 13 │ 50m      │ backstroke        │ female       │ 2024-03-02 │
│ 14 │ 50m      │ backstroke        │ male         │ 2024-03-02 │
│ 15 │ 50m      │ butterfly         │ female       │ 2024-03-02 │
│ 16 │ 50m      │ butterfly         │ male         │ 2024-03-02 │
│ 17 │ 100m     │ breaststroke      │ female       │ 2024-03-02 │
│ 18 │ 100m     │ breaststroke      │ male         │ 2024-03-02 │
│ 19 │ 4x50m    │ freestyle         │ mixed        │ 2024-03-02 │
╰────┴──────────┴───────────────────┴──────────────┴────────────╯
[INFO]: 1/1 successfully meet files parsed

```

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

- use rust nighlty.
- download some meets of the internet `cargo run -cargo run -- --download`. It will save them in your users cache directory.
- `cargo run` will run in `/moisty/src/main.rs` which will try to parse every file in `CACHE_DIR/meets/*.xml`.
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
