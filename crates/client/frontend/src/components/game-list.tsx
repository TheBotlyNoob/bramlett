"use client";

import { useMutation, useQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import { Button, List, Progress, Tooltip } from "antd";
import { ClimbingBoxLoader } from "react-spinners";

const GAMES_QUERY = gql(`
    query Games {
        games {
            id
            name
            status {
                status
                progress
            }
        }
    }
`);

const DOWNLOAD_GAME = gql(`
    mutation DownloadGame($game: GameId!) {
        download(game: $game) {
            void {
                __typename
            }
        }
    }
`);

const RUN_GAME = gql(`
    mutation RunGame($game: GameId!) {
        run(game: $game) {
            void {
                __typename
            }
        }
    }
`);

export default function GamesList() {
    const { loading, data, error, refetch } = useQuery(GAMES_QUERY);
    const [downloadGame] = useMutation(DOWNLOAD_GAME);
    const [runGame] = useMutation(RUN_GAME);

    setInterval(refetch, 1000);

    const gameStatus = (game: GamesQuery["games"][0]) => {
        switch (game.status.status) {
            case GraphQlGameStatusInner.Downloading:
                let downloadProgress =
                    game.status.progress![0] / game.status.progress![1];
                return (
                    <Tooltip
                        placement="topRight"
                        title={`Downloading... (${
                            game.status.progress![0] / 1000
                        } GB out of ${game.status.progress![1] / 1000})`}
                    >
                        <Progress
                            percent={Math.round(downloadProgress * 100)}
                        ></Progress>
                    </Tooltip>
                );
            case GraphQlGameStatusInner.Installing:
                let installProgress =
                    game.status.progress![0] / game.status.progress![1];
                return (
                    <Tooltip
                        placement="topRight"
                        title={`Installing... (${
                            game.status.progress![0] / 1000
                        } GB out of ${game.status.progress![1] / 1000})`}
                    >
                        <Progress
                            percent={Math.round(installProgress * 100)}
                        ></Progress>
                    </Tooltip>
                );
            case GraphQlGameStatusInner.NotDownloaded:
                return (
                    <Button
                        size="large"
                        onClick={() => {
                            downloadGame({ variables: { game: game.id } });
                        }}
                    >
                        Download
                    </Button>
                );
            case GraphQlGameStatusInner.Running:
                return <h3>Running...</h3>;
            case GraphQlGameStatusInner.Stopped:
                return (
                    <Button
                        type="primary"
                        size="large"
                        onClick={() => {
                            runGame({ variables: { game: game.id } });
                        }}
                    >
                        Start
                    </Button>
                );
        }
    };

    return (
        <>
            <ClimbingBoxLoader loading={loading} />

            {data && !error && (
                <List
                    bordered
                    dataSource={Object.entries(data.games)}
                    renderItem={([_, game]) => (
                        <List.Item>
                            <h2
                                style={{
                                    marginRight: "10em",
                                }}
                            >
                                {game.name}
                            </h2>
                            {gameStatus(game)}
                        </List.Item>
                    )}
                />
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
