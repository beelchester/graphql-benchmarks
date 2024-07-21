<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `13,644.60` | `7.19` | `75.80x` |
|| [async-graphql] | `1,776.53` | `56.08` | `9.87x` |
|| [Caliban] | `1,336.87` | `74.73` | `7.43x` |
|| [GraphQL JIT] | `1,231.99` | `80.94` | `6.84x` |
|| [Gqlgen] | `753.42` | `132.27` | `4.19x` |
|| [Apollo GraphQL] | `263.96` | `376.26` | `1.47x` |
|| [Hasura] | `180.00` | `554.92` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `17,742.50` | `5.44` | `22.19x` |
|| [async-graphql] | `6,499.67` | `15.19` | `8.13x` |
|| [Caliban] | `6,365.03` | `15.30` | `7.96x` |
|| [Gqlgen] | `1,910.10` | `52.04` | `2.39x` |
|| [Apollo GraphQL] | `1,564.77` | `63.49` | `1.96x` |
|| [GraphQL JIT] | `1,270.30` | `78.30` | `1.59x` |
|| [Hasura] | `799.67` | `123.97` | `1.00x` |
| 3 | `{ greet }` |
|| [Tailcall] | `17,755.70` | `5.43` | `22.09x` |
|| [Caliban] | `6,446.90` | `15.08` | `8.02x` |
|| [async-graphql] | `6,348.23` | `15.53` | `7.90x` |
|| [Gqlgen] | `1,908.87` | `52.01` | `2.38x` |
|| [Apollo GraphQL] | `1,605.03` | `61.82` | `2.00x` |
|| [GraphQL JIT] | `1,278.23` | `77.82` | `1.59x` |
|| [Hasura] | `803.73` | `123.24` | `1.00x` |

<!-- PERFORMANCE_RESULTS_END -->
