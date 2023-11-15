import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	schema: 'http://localhost:8635/graphql',
	documents: './src/lib/gql/*.gql',
	generates: {
		'./src/lib/gql/index.ts': {
			plugins: ['typescript', 'typescript-operations', 'graphql-codegen-svelte-apollo'],
			config: {
				clientPath: '$lib/apolloClient.ts',
				asyncQuery: true
			}
		}
	}
};
export default config;
