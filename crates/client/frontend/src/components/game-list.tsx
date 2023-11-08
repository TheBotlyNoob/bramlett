"use client";

import { useMutation, useQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import { Button, List, Progress, Tooltip } from "antd";
import { ClimbingBoxLoader } from "react-spinners";
import { Icon } from "@iconify/react";
import Image from "next/image";

const GAMES_QUERY = gql(`
    query Games {
        games {
            id
            name
            icon
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

const UPDATE_GAME_LIST = gql(`
    mutation UpdateGames {
        updateGameList {
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
    const [updateGameList] = useMutation(UPDATE_GAME_LIST);

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
                        }GB out of ${game.status.progress![1] / 1000}GB)`}
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
                            game.status.progress![0]
                        } files out of ${game.status.progress![1]})`}
                    >
                        <Progress
                            strokeColor="#50C878"
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
                <>
                    <List
                        bordered
                        header={
                            process.env.NODE_ENV === "development" && (
                                <Tooltip title="refresh game list">
                                    <Button
                                        onClick={() => {
                                            updateGameList();
                                        }}
                                    >
                                        <Icon icon="mdi:refresh" />
                                    </Button>
                                </Tooltip>
                            )
                        }
                        dataSource={Object.entries(data.games)}
                        renderItem={([_, game]) => (
                            <List.Item>
                                {/* eslint-disable-next-line @next/next/no-img-element */}
                                <img
                                    src={game.icon}
                                    alt={`${game.name}'s icon`}
                                    width="175"
                                    style={{
                                        borderRadius: "1em",
                                        // gray border of 2px
                                        border: "2px solid #808080",
                                    }}
                                />
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
                </>
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
