# smart-publish-test

The following command publishes `smart-add-one` crate to crates.io automatically. A new tag is added and pushed to the repository.

```bash
cargo smart-release --no-changelog --update-crates-index --execute smart-add-one
```
