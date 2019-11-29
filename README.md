# Nom double issue

The `number:complete::double` parser returns an unexpected result when passed the input `". "`. I 
would expect it to fail, but it returns `Ok((" ", 0.0))`. If the lexical feature is disabled, then 
the expected error of: `Err(Error((" ", Digit))` is returned.
