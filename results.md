<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
|| [Apollo GraphQL] | `-nan` | `-nan` | `-nanx` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
|| [Apollo GraphQL] | `-nan` | `-nan` | `-nanx` |
| 3 | `{ greet }` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
|| [Apollo GraphQL] | `-nan` | `-nan` | `-nanx` |

<!-- PERFORMANCE_RESULTS_END -->
