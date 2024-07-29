# GraphQL Benchmarks <!-- omit from toc -->

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/tailcallhq/graphql-benchmarks)

Explore and compare the performance of the fastest GraphQL frameworks through our comprehensive benchmarks.

- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Benchmark Results](#benchmark-results)
  - [Throughput (Higher is better)](#throughput-higher-is-better)
  - [Latency (Lower is better)](#latency-lower-is-better)
- [Architecture](#architecture)
  - [WRK](#wrk)
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

<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `28,774.10` | `3.46` | `202.11x` |
|| [async-graphql] | `2,061.38` | `48.41` | `14.48x` |
|| [Caliban] | `1,685.51` | `59.05` | `11.84x` |
|| [GraphQL JIT] | `1,332.54` | `74.74` | `9.36x` |
|| [Gqlgen] | `749.16` | `132.45` | `5.26x` |
|| [Netflix DGS] | `367.84` | `174.70` | `2.58x` |
|| [Apollo GraphQL] | `262.36` | `373.95` | `1.84x` |
|| [Hasura] | `142.37` | `563.55` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `58,125.10` | `1.71` | `67.40x` |
|| [async-graphql] | `9,792.15` | `10.34` | `11.35x` |
|| [Caliban] | `9,558.36` | `10.83` | `11.08x` |
|| [Gqlgen] | `2,070.52` | `50.01` | `2.40x` |
|| [Apollo GraphQL] | `1,692.76` | `59.02` | `1.96x` |
|| [Netflix DGS] | `1,600.06` | `70.92` | `1.86x` |
|| [GraphQL JIT] | `1,376.25` | `72.56` | `1.60x` |
|| [Hasura] | `862.39` | `115.69` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `68,591.70` | `1.09` | `26.36x` |
|| [Tailcall] | `59,806.40` | `1.69` | `22.99x` |
|| [async-graphql] | `48,010.70` | `2.14` | `18.45x` |
|| [Gqlgen] | `44,832.90` | `5.47` | `17.23x` |
|| [Netflix DGS] | `8,288.02` | `15.07` | `3.19x` |
|| [Apollo GraphQL] | `7,911.24` | `12.80` | `3.04x` |
|| [GraphQL JIT] | `5,248.79` | `19.03` | `2.02x` |
|| [Hasura] | `2,601.78` | `38.36` | `1.00x` |

<!-- PERFORMANCE_RESULTS_END -->



### 1. `{posts {title body user {name}}}`
#### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram1.png)

#### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram1.png)

### 2. `{posts {title body}}`
#### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram2.png)

#### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram2.png)

### 3. `{greet}`
#### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram3.png)

#### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram3.png)

## Architecture

![Architecture Diagram](assets/architecture.png)

A client (`wrk`) sends requests to a GraphQL server to fetch post titles. The GraphQL server, in turn, retrieves data from an external source, `jsonplaceholder.typicode.com`, routed through the `nginx` reverse proxy.

### WRK

`wrk` serves as our test client, sending GraphQL requests at a high rate.

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
