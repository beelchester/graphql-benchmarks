use async_graphql::{
    dataloader::{DataLoader, Loader},
    http::GraphiQLSource,
    Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_axum::GraphQL;
use axum::{
    async_trait,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use futures::future::try_join_all;
use reqwest::Proxy;
use std::{collections::HashMap, time::Duration};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[derive(SimpleObject, serde::Serialize, serde::Deserialize, Debug, Clone)]

pub struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    phone: String,
    website: String,
}

#[derive(Debug, SimpleObject, serde::Serialize, serde::Deserialize)]
pub struct Post {
    id: u32,
    #[serde(rename = "userId")]
    user_id: u32,
    title: String,
    body: String,
    user: Option<User>,
}

pub struct QueryRoot;

const BASE_URL_POSTS: &str = "http://jsonplaceholder.typicode.com/posts";
const BASE_URL_USERS: &str = "http://jsonplaceholder.typicode.com/users";

pub struct UserLoader {
    client: reqwest::Client,
}

// #[async_trait]
impl Loader<u32> for UserLoader {
    type Value = User;
    type Error = ();

    async fn load(&self, keys: &[u32]) -> Result<HashMap<u32, Self::Value>, Self::Error> {
        let mut results = HashMap::new();
        for &key in keys {
            let user_url = format!("{}/{}", BASE_URL_USERS, key);
            // println!("Fetching user: {}", user_url);
            let user = self
                .client
                .get(&user_url)
                .send()
                .await
                .unwrap()
                .json::<User>()
                .await
                .unwrap();
            results.insert(key, user);
        }
        Ok(results)
    }
}

#[Object]
impl QueryRoot {
    pub async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let client = ctx.data_unchecked::<reqwest::Client>();

        // Fetch posts
        let posts = client
            .get(BASE_URL_POSTS)
            .send()
            .await?
            .json::<Vec<Post>>()
            .await?;

        let user_loader = ctx.data_unchecked::<DataLoader<UserLoader>>();

        // Fetch users and pair them with posts
        let mut post_user_futures = Vec::new();
        for post in posts {
            post_user_futures.push(async move {
                match user_loader.load_one(post.user_id).await {
                    Ok(Some(user)) => Ok(Post {
                        user: Some(user),
                        ..post
                    }),
                    Ok(None) => Ok(post),
                    Err(e) => Err(e),
                }
            });
        }

        let posts_with_users: Vec<Post> = try_join_all(post_user_futures).await.unwrap();

        Ok(posts_with_users)
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new()
        .proxy(Proxy::all("http://127.0.0.1:3000").unwrap())
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .tcp_nodelay(true)
        .no_gzip()
        .http2_keep_alive_timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(200)
        .build()
        .unwrap();

    let user_loader = UserLoader {
        client: client.clone(),
    };
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(client)
        .data(DataLoader::new(user_loader, tokio::spawn))
        .finish();

    let app = Router::new().route("/graphql", get(graphiql).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    let listener = std::net::TcpListener::bind("127.0.0.1:8000").unwrap();
    listener.set_nonblocking(true).unwrap();
    axum::serve(TcpListener::from_std(listener).unwrap(), app)
        .await
        .unwrap();
}
