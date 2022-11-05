# test-plan-workspace

```shell
# Import test plan `plan_a`
$ testground plan import --from ./plan_a/

# Run the test plan but the error happens due to workspace
$ testground run single \
  --plan=plan_a \
  --testcase=plan_a \
  --builder=docker:generic \
  --runner=local:docker \
  --instances=3 \
  --wait

...
...

error: failed to get `utility_a` as a dependency of package `plan_a v0.1.0 (/usr/src/test-plan/plan)`

Caused by:
  failed to load source for dependency `utility_a`

Caused by:
  Unable to update /usr/src/test-plan/utilities/utility_a

Caused by:
  failed to read `/usr/src/test-plan/utilities/utility_a/Cargo.toml`

Caused by:
  No such file or directory (os error 2)

...
...
```

