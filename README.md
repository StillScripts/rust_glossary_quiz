# Rust Glossary Quiz

#### *Save glossary terms and test your knowledge with multiple choice questions from the command line.*
---
#### Add terms
```
cargo run add [term]
```
This will enable you to add a new glossary term to a [term].txt file. 

For example -
```
cargo run add science
```
will enable you to add a new glossary term to a science.txt file

---
#### Practice with multiple-choice questions
```
cargo run practice [term]
```
This will enable you to run a multiple choice test based on the terms that are stored in your [term].txt file. 

For example -
```
cargo run practice science
```
will enable you to get tested on the terms in your science.txt file.

Try 'cargo run add demo' or 'cargo run practice demo' to get the hang of it.
