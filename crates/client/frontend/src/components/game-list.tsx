"use client";

import { useMutation, useQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import {
    Button,
    ButtonGroup,
    Card,
    CircularProgress,
    Tooltip,
    CircularProgressLabel,
    IconButton,
    Image,
    CardBody,
    Divider,
    CardFooter,
    Heading,
    SimpleGrid,
} from "@chakra-ui/react";
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
                    Math.round(
                        (game.status.progress![0] / game.status.progress![1]) *
                            100,
                    ) || 0;
                return (
                    <Tooltip
                        label={`Downloading... (${
                            game.status.progress![0] / 1000
                        }GB out of ${game.status.progress![1] / 1000}GB)`}
                    >
                        <CircularProgress value={downloadProgress}>
                            <CircularProgressLabel>
                                {downloadProgress}%
                            </CircularProgressLabel>
                        </CircularProgress>
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
                        label={`Downloading... (${
                            game.status.progress![0]
                        } files out of ${game.status.progress![1]})`}
                    >
                        <CircularProgress value={installProgress}>
                            <CircularProgressLabel>
                                {installProgress}%
                            </CircularProgressLabel>
                        </CircularProgress>
                    </Tooltip>
                );
            case GraphQlGameStatusInner.NotDownloaded:
                return (
                    <Button
                        colorScheme="blue"
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
                    <ButtonGroup isAttached>
                        <Button
                            colorScheme="red"
                            onClick={() =>
                                deleteGame({ variables: { game: game.id } })
                            }
                        >
                            Delete
                        </Button>
                        <Button
                            colorScheme="green"
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
                        <IconButton
                            aria-label="Refresh game list"
                            icon={<Icon icon="mdi:refresh" />}
                            onClick={() => updateGameList()}
                        />
                    )}
                    <SimpleGrid columns={3} spacing={4}>
                        {Object.values(data.games).map((game, key) => (
                            <Card key={key}>
                                <CardBody>
                                    <Image
                                        src={game.icon}
                                        alt={`${game.name}'s icon`}
                                        borderRadius="15px"
                                        border="1px"
                                        borderColor="gray.200"
                                    />
                                    <Heading mt={6} size="md">
                                        {game.name}
                                    </Heading>
                                </CardBody>
                                <Divider />
                                <CardFooter>{gameStatus(game)}</CardFooter>
                            </Card>
                        ))}
                    </SimpleGrid>
                </>
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
