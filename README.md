Experimental and minimal Python binding for [fst](https://github.com/BurntSushi/fst) using [pyo3](https://github.com/PyO3/pyo3)

```python
import pyfst
smarter_set = pyfst.FstSet('smarter_encryption.fst')
print(smarter_set)
print(len(smarter_set))
return smarter_set.contains('eff.org')
```

There is no Python binding to build the .fst file.
