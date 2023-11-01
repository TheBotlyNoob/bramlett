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
};

export type GraphQlGame = {
  __typename?: 'GraphQLGame';
  id: Scalars['Int']['output'];
  name: Scalars['String']['output'];
  status: GraphQlGameStatus;
};

export type GraphQlGameStatus = {
  __typename?: 'GraphQLGameStatus';
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

export type Query = {
  __typename?: 'Query';
  game?: Maybe<GraphQlGame>;
  games: Array<GraphQlGame>;
};


export type QueryGameArgs = {
  id: Scalars['Int']['input'];
};

export type GamesQueryVariables = Exact<{ [key: string]: never; }>;


export type GamesQuery = { __typename?: 'Query', games: Array<{ __typename?: 'GraphQLGame', name: string, status: { __typename?: 'GraphQLGameStatus', status: GraphQlGameStatusInner } }> };


export const GamesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"Games"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"games"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"status"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"status"}}]}}]}}]}}]} as unknown as DocumentNode<GamesQuery, GamesQueryVariables>;