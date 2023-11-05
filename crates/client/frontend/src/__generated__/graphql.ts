/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  GameId: { input: any; output: any; }
};

export type GraphQlGame = {
  __typename?: 'GraphQLGame';
  id: Scalars['Int']['output'];
  name: Scalars['String']['output'];
  status: GraphQlGameStatus;
};

export type GraphQlGameStatus = {
  __typename?: 'GraphQLGameStatus';
  /** Progress in megabytes */
  progress?: Maybe<Array<Scalars['Int']['output']>>;
  status: GraphQlGameStatusInner;
};

export enum GraphQlGameStatusInner {
  Downloading = 'DOWNLOADING',
  Installing = 'INSTALLING',
  NotDownloaded = 'NOT_DOWNLOADED',
  Running = 'RUNNING',
  Stopped = 'STOPPED'
}

export type Mutation = {
  __typename?: 'Mutation';
  download: Void;
  run: Void;
  updateGameList: Void;
};


export type MutationDownloadArgs = {
  game: Scalars['GameId']['input'];
};


export type MutationRunArgs = {
  game: Scalars['GameId']['input'];
};

export type Query = {
  __typename?: 'Query';
  game?: Maybe<GraphQlGame>;
  games: Array<GraphQlGame>;
};


export type QueryGameArgs = {
  id: Scalars['Int']['input'];
};

export type Void = {
  __typename?: 'Void';
  void: Void;
};

export type GamesQueryVariables = Exact<{ [key: string]: never; }>;


export type GamesQuery = { __typename?: 'Query', games: Array<{ __typename?: 'GraphQLGame', id: number, name: string, status: { __typename?: 'GraphQLGameStatus', status: GraphQlGameStatusInner, progress?: Array<number> | null } }> };

export type DownloadGameMutationVariables = Exact<{
  game: Scalars['GameId']['input'];
}>;


export type DownloadGameMutation = { __typename?: 'Mutation', download: { __typename?: 'Void', void: { __typename: 'Void' } } };

export type RunGameMutationVariables = Exact<{
  game: Scalars['GameId']['input'];
}>;


export type RunGameMutation = { __typename?: 'Mutation', run: { __typename?: 'Void', void: { __typename: 'Void' } } };

export type UpdateGamesMutationVariables = Exact<{ [key: string]: never; }>;


export type UpdateGamesMutation = { __typename?: 'Mutation', updateGameList: { __typename?: 'Void', void: { __typename: 'Void' } } };


export const GamesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"Games"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"games"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"status"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"status"}},{"kind":"Field","name":{"kind":"Name","value":"progress"}}]}}]}}]}}]} as unknown as DocumentNode<GamesQuery, GamesQueryVariables>;
export const DownloadGameDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"DownloadGame"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"game"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"GameId"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"download"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"game"},"value":{"kind":"Variable","name":{"kind":"Name","value":"game"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"void"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"__typename"}}]}}]}}]}}]} as unknown as DocumentNode<DownloadGameMutation, DownloadGameMutationVariables>;
export const RunGameDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"RunGame"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"game"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"GameId"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"run"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"game"},"value":{"kind":"Variable","name":{"kind":"Name","value":"game"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"void"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"__typename"}}]}}]}}]}}]} as unknown as DocumentNode<RunGameMutation, RunGameMutationVariables>;
export const UpdateGamesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateGames"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateGameList"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"void"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"__typename"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateGamesMutation, UpdateGamesMutationVariables>;