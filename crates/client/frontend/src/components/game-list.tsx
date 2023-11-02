"use client";

import { useQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import { Button, List } from "antd";
import { ClimbingBoxLoader } from "react-spinners";

const GAMES_QUERY = gql(`
    query Games {
        games {
            name
            status {
                status
            }
        }
    }
`);

const DOWNLOAD_GAME = gql(`
    mutation DownloadGame($game: GameId!) {
        download(game: $game) {
            status {
                status
            }
        }
    }
`);

export default function GamesList() {
    const { loading, data, error, refetch } = useQuery(GAMES_QUERY);

    // setInterval(refetch, 1000);

    const gameStatus = ({ status }: GamesQuery["games"][0]) => {
        switch (status.status) {
            case GraphQlGameStatusInner.Downloading:
                return <></>;
            case GraphQlGameStatusInner.Installing:
                return <Button>Cancel</Button>;
            case GraphQlGameStatusInner.NotDownloaded:
                return <Button>Download</Button>;
            case GraphQlGameStatusInner.Running:
                return <Button>Stop</Button>;
            case GraphQlGameStatusInner.Stopped:
                return <Button>Start</Button>;
        }
    };

    return (
        <>
            <ClimbingBoxLoader loading={loading} />

            {data && (
                <List
                    bordered
                    dataSource={Object.entries(data.games)}
                    renderItem={([_, game]) => (
                        <List.Item>
                            <h2>{game.name}</h2>
                            {gameStatus(game)}
                        </List.Item>
                    )}
                />
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
