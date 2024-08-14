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
|| [Tailcall] | `28,128.00` | `3.54` | `133.18x` |
|| [async-graphql] | `1,991.41` | `50.31` | `9.43x` |
|| [Caliban] | `1,666.83` | `59.72` | `7.89x` |
|| [GraphQL JIT] | `1,311.20` | `75.92` | `6.21x` |
|| [Gqlgen] | `819.49` | `121.07` | `3.88x` |
|| [Netflix DGS] | `368.90` | `166.69` | `1.75x` |
|| [Apollo GraphQL] | `271.22` | `362.03` | `1.28x` |
|| [Hasura] | `211.21` | `473.26` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `57,212.70` | `1.74` | `72.96x` |
|| [async-graphql] | `10,057.10` | `10.02` | `12.83x` |
|| [Caliban] | `9,554.16` | `10.80` | `12.18x` |
|| [Gqlgen] | `2,232.00` | `46.42` | `2.85x` |
|| [Apollo GraphQL] | `1,770.05` | `56.43` | `2.26x` |
|| [Netflix DGS] | `1,593.60` | `70.12` | `2.03x` |
|| [GraphQL JIT] | `1,368.82` | `72.96` | `1.75x` |
|| [Hasura] | `784.14` | `127.56` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `68,386.90` | `1.12` | `27.96x` |
|| [Tailcall] | `58,703.60` | `1.71` | `24.00x` |
|| [Gqlgen] | `47,788.80` | `5.11` | `19.54x` |
|| [async-graphql] | `47,498.90` | `2.18` | `19.42x` |
|| [Apollo GraphQL] | `8,213.39` | `12.40` | `3.36x` |
|| [Netflix DGS] | `8,212.32` | `14.56` | `3.36x` |
|| [GraphQL JIT] | `5,073.43` | `19.68` | `2.07x` |
|| [Hasura] | `2,445.53` | `40.88` | `1.00x` |

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
