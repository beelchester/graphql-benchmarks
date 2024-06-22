import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';
import axios from 'axios';
import { Agent } from 'http';
import DataLoader from 'dataloader';

// Create a new axios instance with connection pooling.
const httpAgent = new Agent({ keepAlive: true });
const axiosInstance = axios.create({
	httpAgent
});


const typeDefs = `#graphql
  
  type User {
		id: Int!
		name: String!
		username: String!
		email: String!
		phone: String
		website: String
	}

	type Post {
		id: Int!
		userId: Int!
		title: String!
		body: String!
		user: User
	}

  type Query {
		posts: [Post]
  }
`;

async function batchUsers(userIds) {
	console.log("batch userIds", userIds)
	// ex: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
	// userIds are the batch keys
	// makes only one request per user to /user/:id instead of duplicates of requests for the same user
	const requests = userIds.map(async id => {
		console.log("request for", id)
		const response = await axiosInstance.get(`http://jsonplaceholder.typicode.com/users/${id}`, {
		});
		return response.data;
	});
	// wait for all the 10 requests (in case of 10 users) to finish
	return await Promise.all(requests);
}

const userLoader = new DataLoader(batchUsers)

const resolvers = {
	Query: {
		posts: async () => {
			try {
				const response = await axiosInstance.get('http://jsonplaceholder.typicode.com/posts', {
					proxy: {
						protocol: 'http',
						host: '127.0.0.1',
						port: 3000
					},
				});
				return response.data;
			} catch (error) {
				console.log(error)
				throw new Error('Failed to fetch posts:', error);
			}
		},
	},
	Post: {
		user: async (post) => {
			return userLoader.load(post.userId);
		}
	}
};

const server = new ApolloServer({ typeDefs, resolvers });

const { url } = await startStandaloneServer(server, {
	listen: { port: 8000 },
});

console.log(`🚀  Server ready at: ${url}`);
