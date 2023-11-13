"use client";

import { useMutation, useQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import {
    Button,
    ButtonGroup,
    Card,
    Progress,
    Tooltip,
    Image,
    CardBody,
    Divider,
    CardFooter,
    CardHeader,
} from "@nextui-org/react";
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
    const { loading, data, error } = useQuery(GAMES_QUERY, {
        pollInterval: 3000,
    });
    const [downloadGame] = useMutation(DOWNLOAD_GAME);
    const [runGame] = useMutation(RUN_GAME);
    const [updateGameList] = useMutation(UPDATE_GAME_LIST);
    const [deleteGame] = useMutation(DELETE_GAME);

    const gameStatus = (game: GamesQuery["games"][0]) => {
        switch (game.status.status) {
            case GraphQlGameStatusInner.Downloading:
                let downloadProgress =
                    Math.round(
                        (game.status.progress![0] / game.status.progress![1]) *
                            100,
                    ) || 0;
                return (
                    <Tooltip
                        showArrow
                        content={`Downloading... (${
                            game.status.progress![0] / 1000
                        }GB out of ${game.status.progress![1] / 1000}GB)`}
                    >
                        <Progress value={downloadProgress} />
                    </Tooltip>
                );
            case GraphQlGameStatusInner.Installing:
                let installProgress =
                    Math.round(
                        (game.status.progress![0] / game.status.progress![1]) *
                            100,
                    ) || 0;
                return (
                    <Tooltip
                        showArrow
                        content={`Installing... (${
                            game.status.progress![0]
                        } files out of ${game.status.progress![1]})`}
                    >
                        <Progress color="success" value={installProgress} />
                    </Tooltip>
                );
            case GraphQlGameStatusInner.NotDownloaded:
                return (
                    <Button
                        color="primary"
                        onClick={() =>
                            downloadGame({ variables: { game: game.id } })
                        }
                    >
                        Download
                    </Button>
                );
            case GraphQlGameStatusInner.Running:
                return <h3>Running...</h3>;
            case GraphQlGameStatusInner.Stopped:
                return (
                    <ButtonGroup>
                        <Button
                            color="danger"
                            onClick={() =>
                                deleteGame({ variables: { game: game.id } })
                            }
                        >
                            Delete
                        </Button>
                        <Divider orientation="vertical" />
                        <Button
                            color="success"
                            onClick={() =>
                                runGame({ variables: { game: game.id } })
                            }
                        >
                            Start
                        </Button>
                    </ButtonGroup>
                );
        }
    };

    return (
        <>
            <ClimbingBoxLoader loading={loading} />

            {data && !error && (
                <>
                    {process.env.NODE_ENV === "development" && (
                        <Button
                            isIconOnly
                            aria-label="Refresh game list"
                            onClick={() => updateGameList()}
                        >
                            <Icon icon="mdi:refresh" />
                        </Button>
                    )}
                    <div className="grid grid-cols-4 gap-4">
                        {Object.values(data.games).map((game, key) => (
                            <Card key={key} className="max-w-xs">
                                <CardHeader className="whitespace-nowrap overflow-scroll">
                                    <span className="text-xl font-bold m-auto">
                                        {game.name}
                                    </span>
                                </CardHeader>
                                <CardBody>
                                    <Image
                                        src={game.icon}
                                        alt={`${game.name}'s icon`}
                                        className="m-0"
                                    />
                                    <Divider className="mt-3" />
                                </CardBody>
                                <CardFooter className="flex justify-center">
                                    {gameStatus(game)}
                                </CardFooter>
                            </Card>
                        ))}
                    </div>
                </>
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
