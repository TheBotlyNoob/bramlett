"use client";

import { useMutation, useQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import { Button, Card, Col, List, Progress, Row, Tooltip } from "antd";
import { ClimbingBoxLoader } from "react-spinners";
import { Icon } from "@iconify/react";

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

const DELETE_GAME = gql(`
    mutation DeleteGame($game: GameId!) {
        delete(game: $game) {
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
    const [deleteGame] = useMutation(DELETE_GAME);

    setInterval(() => {
        if (
            typeof window !== "undefined" &&
            document.visibilityState === "visible"
        ) {
            refetch();
        }
    }, 1500);

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
                        type="default"
                        size="middle"
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
                    <>
                        <Button
                            danger
                            size="small"
                            onClick={() => {
                                deleteGame({ variables: { game: game.id } });
                            }}
                        >
                            Delete
                        </Button>
                        <Button
                            type="primary"
                            size="middle"
                            onClick={() => {
                                runGame({ variables: { game: game.id } });
                            }}
                        >
                            Start
                        </Button>
                    </>
                );
        }
    };

    return (
        <>
            <ClimbingBoxLoader loading={loading} />

            {data && !error && (
                <>
                    {process.env.NODE_ENV === "development" && (
                        <Tooltip title="refresh game list">
                            <Button
                                onClick={() => {
                                    updateGameList();
                                }}
                            >
                                <Icon icon="mdi:refresh" />
                            </Button>
                        </Tooltip>
                    )}
                    {Object.values(data.games)
                        .reduce(
                            (acc: GamesQuery["games"][], _, i, original) => {
                                if (i % 3 === 0) {
                                    acc.push(original.slice(i, i + 3));
                                }
                                return acc;
                            },
                            [],
                        )
                        .map((row, key) => (
                            <Row
                                key={key}
                                gutter={[16, { xs: 8, sm: 16, md: 24, lg: 32 }]}
                                style={{ marginTop: "1em" }}
                                justify="space-evenly"
                            >
                                {row.map((game, key) => (
                                    <Col key={key} span={6}>
                                        <Card
                                            title={game.name}
                                            extra={gameStatus(game)}
                                        >
                                            {/* eslint-disable-next-line @next/next/no-img-element */}
                                            <img
                                                src={game.icon}
                                                alt={`${game.name}'s icon`}
                                                width="100%"
                                                style={{
                                                    borderRadius: "1em",
                                                    // gray border of 2px
                                                    border: "2px solid #808080",
                                                }}
                                            />
                                        </Card>
                                    </Col>
                                ))}
                            </Row>
                        ))}
                </>
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
