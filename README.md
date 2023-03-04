# moisty

moisty is a hobby project / library that aims to implement different tools used in swimming. It varies from device drivers to file parsers.

# TODO

## url endpoints
- [x] parse meet_info endpoint
- [ ] athlete records endpoint

## jechsoft
- [ ] parse `meetsetup.xml` file. This file contains meet setup.
- [ ] parse `meetresult.xml`. This file contains meet results.
- [ ] parse `uni_p.txt`. This file contains meet enrollment information
- [ ] parse `tryggivann.csv` exports.

# Contribing
1. use rust nighlty
2. add this to `.gitconfig` to ignore changes made to `Cargo.lock` in `git diff`
```.gitconfig
[diff "generated"]
	command = true
```
