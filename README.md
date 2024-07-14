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
|| [Tailcall] | `30,524.30` | `3.26` | `111.04x` |
|| [async-graphql] | `1,904.04` | `52.56` | `6.93x` |
|| [Caliban] | `1,600.47` | `62.46` | `5.82x` |
|| [Hasura] | `1,526.50` | `65.25` | `5.55x` |
|| [GraphQL JIT] | `1,378.01` | `72.26` | `5.01x` |
|| [Gqlgen] | `780.69` | `127.13` | `2.84x` |
|| [Netflix DGS] | `360.28` | `209.06` | `1.31x` |
|| [Apollo GraphQL] | `274.88` | `357.00` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `62,413.90` | `1.59` | `44.03x` |
|| [async-graphql] | `9,619.17` | `10.48` | `6.79x` |
|| [Caliban] | `9,338.28` | `11.07` | `6.59x` |
|| [Hasura] | `2,554.98` | `39.14` | `1.80x` |
|| [Gqlgen] | `2,209.78` | `46.99` | `1.56x` |
|| [Apollo GraphQL] | `1,785.00` | `55.95` | `1.26x` |
|| [Netflix DGS] | `1,600.07` | `70.10` | `1.13x` |
|| [GraphQL JIT] | `1,417.67` | `70.44` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `69,167.40` | `1.04` | `26.82x` |
|| [Tailcall] | `63,680.30` | `1.58` | `24.69x` |
|| [async-graphql] | `51,186.10` | `2.00` | `19.84x` |
|| [Gqlgen] | `47,690.50` | `5.23` | `18.49x` |
|| [Netflix DGS] | `8,233.97` | `14.54` | `3.19x` |
|| [Apollo GraphQL] | `8,142.00` | `12.45` | `3.16x` |
|| [GraphQL JIT] | `5,293.60` | `18.86` | `2.05x` |
|| [Hasura] | `2,579.36` | `38.69` | `1.00x` |

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
