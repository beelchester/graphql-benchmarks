<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `26,053.80` | `3.82` | `132.44x` |
|| [async-graphql] | `2,037.76` | `49.34` | `10.36x` |
|| [Caliban] | `1,554.56` | `63.96` | `7.90x` |
|| [GraphQL JIT] | `1,366.96` | `72.85` | `6.95x` |
|| [Gqlgen] | `801.15` | `123.92` | `4.07x` |
|| [Netflix DGS] | `368.80` | `166.16` | `1.87x` |
|| [Apollo GraphQL] | `270.88` | `363.13` | `1.38x` |
|| [Hasura] | `196.73` | `495.47` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `54,877.90` | `1.82` | `63.59x` |
|| [async-graphql] | `9,697.99` | `10.47` | `11.24x` |
|| [Caliban] | `9,117.56` | `11.34` | `10.56x` |
|| [Gqlgen] | `2,213.88` | `46.77` | `2.57x` |
|| [Apollo GraphQL] | `1,745.78` | `57.21` | `2.02x` |
|| [Netflix DGS] | `1,591.93` | `70.12` | `1.84x` |
|| [GraphQL JIT] | `1,407.22` | `70.97` | `1.63x` |
|| [Hasura] | `863.02` | `115.63` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `62,435.60` | `1.18` | `22.21x` |
|| [Tailcall] | `57,240.30` | `1.76` | `20.36x` |
|| [async-graphql] | `47,686.90` | `2.12` | `16.96x` |
|| [Gqlgen] | `47,641.30` | `5.21` | `16.95x` |
|| [Netflix DGS] | `8,191.99` | `14.79` | `2.91x` |
|| [Apollo GraphQL] | `8,149.65` | `12.50` | `2.90x` |
|| [GraphQL JIT] | `5,236.03` | `19.07` | `1.86x` |
|| [Hasura] | `2,811.23` | `35.94` | `1.00x` |

<!-- PERFORMANCE_RESULTS_END -->
