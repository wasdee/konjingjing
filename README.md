# konjingjing
A hyper-fast Python module for validating Thai Citizen Card ID written in Rust.

The library name, 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

## Get Started
```bash
$ pip install konjingjing
```

```python
from konjingjing import verify_id

assert verify_id('1112034563562') # true
assert verify_id('11120345635') == False # false digit is less than 13
assert verify_id('1112034563s62') == False # false mix with alpha
```

## Notes 😋
This is my first python library written in rust. This is another weekend project. 

![](https://i.imgflip.com/4p243t.jpg)


### Idea-Fork from
1. [one in npm](https://github.com/jukbot/thai-citizen-id-validator)

### Sensei And Inspiration
1. https://www.youtube.com/watch?v=D9r__qxtRMQ
2. https://github.com/mre/hyperjson