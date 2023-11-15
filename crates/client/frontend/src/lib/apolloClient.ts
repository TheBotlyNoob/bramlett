import { ApolloClient, HttpLink, InMemoryCache } from '@apollo/client/core';

const apolloClient = new ApolloClient({
	link: new HttpLink({
		uri: 'http://localhost:8635/graphql'
	}),
	cache: new InMemoryCache()
});

export default apolloClient;
