# Partial library

[![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/partial.png
[travis]: https://travis-ci.org/ptal/partial

Similar to `Option<T>` with an additional Fake variant for accumulating errors beyond the first one. For example, in compiler, if a code analysis failed, we wish to continue a little further to obtain more errors.

Please consult the [documentation](https://docs.rs/partial).
