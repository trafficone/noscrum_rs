# Contributing to NoScrum
Thank you for your interest in contributing to the NoScrum codebase. Any help is highly appreciated.

## How to Contribute
Anything from copy editing to translation, and bug fixes to feature requests are welcome and encouraged. If you have an idea of how to add to the project then we want your labor. If you want to contribute but aren't sure where to start, the best way to get started is just adding an issue saying you're interested! Don't hesitate!

There are two main points to keep in mind when contributing:
1. Keep your changes small and modular.
2. Focus on what will help **you** in the short-term.

It's better to have a dozen finished small features than one 6 month project that may never get finished.

### Filing a Pull Request
Once your change is complete, submit a pull request into the `testing` branch of the main repository. A contributor will be notified and review your code, as well as guarantee the Github Actions tests run. 

> If after 7 days your code has not been reviewed, please file an issue.

#### If your PR is Approved
Your code will be merged into the `testing` branch, which will automatically be pushed out to `main` and deployed if test deployment succeeds. Congratulations!

#### If your PR is Rejected
If your PR doesn't past tests, fix your code and test locally/on your own fork until it does.

If your changes were rejected by the reviewer, make the changes they suggest (or provide reasoning why the reviewer is wrong).

If it's unclear why your changes were rejected, create an issue to change this document.

### Contributing Code
If you are contributing code directly, the style guideline is: use the formatting provided by `cargo fmt`. The settings for `cargo clippy` are set in the main.rs file, and your code shouldn't add any new warnings.

Your change should be 
- clearly defined -- do exactly what you said you were going to do
- explicitly tested -- write/modify tests to guarantee functionality going forward
- small and modular -- should not impact, or be impacted by code outside of the contribution
  - if this isn't possible, revisit the first two points