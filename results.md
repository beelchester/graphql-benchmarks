<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Apollo GraphQL] | `384.10` | `262.38` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 2 | `{ posts { title }}` |
|| [Apollo GraphQL] | `310.41` | `305.64` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 3 | `{ greet }` |
|| [Apollo GraphQL] | `2,457.96` | `40.62` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |

<!-- PERFORMANCE_RESULTS_END -->
