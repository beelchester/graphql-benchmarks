# GraphQL Benchmarks <!-- omit from toc -->

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/tailcallhq/graphql-benchmarks)

Explore and compare the performance of the fastest GraphQL frameworks through our comprehensive benchmarks.

- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Benchmark Results](#benchmark-results)
  - [Throughput (Higher is better)](#throughput-higher-is-better)
  - [Latency (Lower is better)](#latency-lower-is-better)
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
| ![](assets/posts_users_reqs.png) | ![](assets/posts_users_latency.png) |
|  `{ posts { title }}` |
| ![](assets/posts_reqs.png) | ![](assets/posts_latency.png) |
|  `{greet}` |
| ![](assets/greet_reqs.png) | ![](assets/greet_latency.png) |

<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `0.00` | `0.00` | `x` |
|| [Netflix DGS] | `0.00` | `0.00` | `x` |
|| [Hasura] | `0.00` | `0.00` | `x` |
|| [GraphQL JIT] | `0.00` | `0.00` | `x` |
|| [Gqlgen] | `0.00` | `0.00` | `x` |
|| [Caliban] | `0.00` | `0.00` | `x` |
|| [async-graphql] | `0.00` | `0.00` | `x` |
|| [Apollo GraphQL] | `0.00` | `0.00` | `x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `0.00` | `0.00` | `x` |
|| [Netflix DGS] | `0.00` | `0.00` | `x` |
|| [Hasura] | `0.00` | `0.00` | `x` |
|| [GraphQL JIT] | `0.00` | `0.00` | `x` |
|| [Gqlgen] | `0.00` | `0.00` | `x` |
|| [Caliban] | `0.00` | `0.00` | `x` |
|| [async-graphql] | `0.00` | `0.00` | `x` |
|| [Apollo GraphQL] | `0.00` | `0.00` | `x` |
| 3 | `{ greet }` |
|| [Tailcall] | `0.00` | `0.00` | `x` |
|| [Netflix DGS] | `0.00` | `0.00` | `x` |
|| [Hasura] | `0.00` | `0.00` | `x` |
|| [GraphQL JIT] | `0.00` | `0.00` | `x` |
|| [Gqlgen] | `0.00` | `0.00` | `x` |
|| [Caliban] | `0.00` | `0.00` | `x` |
|| [async-graphql] | `0.00` | `0.00` | `x` |
|| [Apollo GraphQL] | `0.00` | `0.00` | `x` |

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
