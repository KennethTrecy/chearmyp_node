# Chearmyp Node
A concrete implementation of nodes for Chearmyp language.

Add it to the dependencies:
```
[dependencies.chearmyp_node]
git = "https://github.com/KennethTrecy/chearmyp_node"
tag = "v0.6.0"
```

You may also activate all the features:
```
[dependencies.chearmyp_node]
git = "https://github.com/KennethTrecy/chearmyp_node"
tag = "v0.6.0"
features = ["no_std", "assertable_node"]
```

## Origin
It was in a repository with the parser library. Yet it has been forked as some possible use cases
may not need a parser.

Som parts of the repository uses the template based from [`filled_bare_metal`] branch of [Feo
Template].

## Nodes
The nodes here have been implemented as enumerations. The attacher collections and node queues are
`VecDeque`s.

### Author
Coded by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
