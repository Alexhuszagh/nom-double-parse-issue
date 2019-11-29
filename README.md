# Nom double issue

The `complete::double` parser returns an unexpected result when passed the input `". "`. I would 
expect it to fail, but it returns `Ok((" ", 0.0))`. What's really strange is that copy and pasting
the implementation into the project yields the expected result: `Err(Error((" ", Digit))`.