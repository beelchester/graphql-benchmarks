# GraphQL Benchmarks <!-- omit from toc -->

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/tailcallhq/graphql-benchmarks)

Explore and compare the performance of the fastest GraphQL frameworks through our comprehensive benchmarks.

- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Benchmark Results](#benchmark-results)
- [Architecture](#architecture)
  - [K6](#k6)
  - [GraphQL](#graphql)
  - [Nginx](#nginx)
  - [Jsonplaceholder](#jsonplaceholder)
- [GraphQL Schema](#graphql-schema)
- [Contribute](#contribute)

[Tailcall]: https://github.com/tailcallhq/tailcall
[Gqlgen]: https://github.com/99designs/gqlgen
[Apollo GraphQL]: https://github.com/apollographql/apollo-server
[Netflix DGS]: https://github.com/netflix/dgs-framework
[Caliban]: https://github.com/ghostdogpr/caliban
[async-graphql]: https://github.com/async-graphql/async-graphql
[Hasura]: https://github.com/hasura/graphql-engine
[GraphQL JIT]: https://github.com/zalando-incubator/graphql-jit

## Introduction

This document presents a comparative analysis of several renowned GraphQL frameworks. Dive deep into the performance metrics, and get insights into their throughput and latency.

> **NOTE:** This is a work in progress suite of benchmarks, and we would appreciate help from the community to add more frameworks or tune the existing ones for better performance.

## Quick Start

Get started with the benchmarks:

1. Click on this [link](https://codespaces.new/tailcallhq/graphql-benchmarks) to set up on GitHub Codespaces.
2. Once set up in Codespaces, initiate the benchmark tests:

```bash
./setup.sh
./run_benchmarks.sh
```

## Benchmark Results

| Throughput (Higher is better) | Latency (Lower is better) | 
|-------:|--------:|
|  `{{ posts { id userId title user { id name email }}}}` |
| ![](assets/posts_users_req.png) | ![](assets/posts_users_latency.png) |
|  `{ posts { title }}` |
| ![](assets/posts_req.png) | ![](assets/posts_latency.png) |
|  `{greet}` |
| ![](assets/greet_req.png) | ![](assets/greet_latency.png) |

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

## Architecture

![Architecture Diagram](assets/architecture.png)

A client (`k6`) sends requests to a GraphQL server to fetch post titles. The GraphQL server, in turn, retrieves data from an external source, `jsonplaceholder.typicode.com`, routed through the `nginx` reverse proxy.

### K6

`k6` serves as our test client, sending GraphQL requests at a high rate.

### GraphQL

Our tested GraphQL server. We evaluated various implementations, ensuring no caching on the GraphQL server side.

### Nginx

A reverse-proxy that caches every response, mitigating rate-limiting and reducing network uncertainties.

### Jsonplaceholder

The primary upstream service forming the base for our GraphQL API. We query its `/posts` API via the GraphQL server.

## GraphQL Schema

Inspect the generated GraphQL schema employed for the benchmarks:

```graphql
schema {
  query: Query
}

type Query {
  posts: [Post]
}

type Post {
  id: Int!
  userId: Int!
  title: String!
  body: String!
  user: User
}

type User {
  id: Int!
  name: String!
  username: String!
  email: String!
  phone: String
  website: String
}
```

## Contribute

Your insights are invaluable! Test these benchmarks, share feedback, or contribute by adding more GraphQL frameworks or refining existing ones. Open an issue or a pull request, and let's build a robust benchmarking resource together!
