<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `14,034.50` | `6.99` | `51.58x` |
|| [async-graphql] | `1,809.98` | `55.05` | `6.65x` |
|| [Caliban] | `1,361.43` | `73.34` | `5.00x` |
|| [Hasura] | `1,344.23` | `74.18` | `4.94x` |
|| [GraphQL JIT] | `1,259.02` | `79.21` | `4.63x` |
|| [Gqlgen] | `762.92` | `130.62` | `2.80x` |
|| [Apollo GraphQL] | `272.08` | `364.71` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `18,114.70` | `5.33` | `14.07x` |
|| [async-graphql] | `6,552.20` | `15.03` | `5.09x` |
|| [Caliban] | `6,491.57` | `14.99` | `5.04x` |
|| [Hasura] | `2,216.00` | `44.91` | `1.72x` |
|| [Gqlgen] | `1,929.77` | `51.43` | `1.50x` |
|| [Apollo GraphQL] | `1,608.63` | `61.74` | `1.25x` |
|| [GraphQL JIT] | `1,287.20` | `77.26` | `1.00x` |
| 3 | `{ greet }` |
|| [Tailcall] | `18,048.20` | `5.36` | `14.03x` |
|| [async-graphql] | `6,542.87` | `15.09` | `5.09x` |
|| [Caliban] | `6,517.67` | `14.93` | `5.07x` |
|| [Hasura] | `2,105.80` | `47.21` | `1.64x` |
|| [Gqlgen] | `1,929.33` | `51.48` | `1.50x` |
|| [Apollo GraphQL] | `1,617.30` | `61.42` | `1.26x` |
|| [GraphQL JIT] | `1,286.60` | `77.27` | `1.00x` |

<!-- PERFORMANCE_RESULTS_END -->
