<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Apollo GraphQL] | `3,211.44` | `31.09` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 2 | `{ posts { title }}` |
|| [Apollo GraphQL] | `3,585.25` | `27.87` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 3 | `{ greet }` |
|| [Apollo GraphQL] | `3,738.32` | `26.69` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |

<!-- PERFORMANCE_RESULTS_END -->
