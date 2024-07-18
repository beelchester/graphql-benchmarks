<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Apollo GraphQL] | `103.85` | `566.24` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 2 | `{ posts { title }}` |
|| [Apollo GraphQL] | `858.25` | `116.25` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 3 | `{ greet }` |
|| [Apollo GraphQL] | `2,595.56` | `38.44` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |

<!-- PERFORMANCE_RESULTS_END -->
