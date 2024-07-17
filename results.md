<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Apollo GraphQL] | `281.22` | `322.67` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 2 | `{ posts { title }}` |
|| [Apollo GraphQL] | `318.22` | `298.73` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |
| 3 | `{ greet }` |
|| [Apollo GraphQL] | `2,483.69` | `40.18` | `-nanx` |
|| [Tailcall] | `-nan` | `-nan` | `-nanx` |
|| [Netflix DGS] | `-nan` | `-nan` | `-nanx` |
|| [Hasura] | `-nan` | `-nan` | `-nanx` |
|| [GraphQL JIT] | `-nan` | `-nan` | `-nanx` |
|| [Gqlgen] | `-nan` | `-nan` | `-nanx` |
|| [Caliban] | `-nan` | `-nan` | `-nanx` |
|| [async-graphql] | `-nan` | `-nan` | `-nanx` |

<!-- PERFORMANCE_RESULTS_END -->
