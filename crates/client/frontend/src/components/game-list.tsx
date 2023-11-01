"use client";

import { useSuspenseQuery } from "@apollo/client";
import { gql } from "@/__generated__";
import { GamesQuery, GraphQlGameStatusInner } from "@/__generated__/graphql";
import { Button, List } from "antd";

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

export default function GamesList() {
    const {
        data: { games },
        error,
    } = useSuspenseQuery(GAMES_QUERY);

    const gameStatus = ({ status }: GamesQuery["games"][0]) => {
        switch (status.status) {
            case GraphQlGameStatusInner.Downloading:
                return <Button>Cancel</Button>;
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
            <List
                bordered
                dataSource={Object.entries(games)}
                renderItem={([_, game]) => (
                    <List.Item>
                        <h2>{game.name}</h2>
                        {gameStatus(game)}
                    </List.Item>
                )}
            />

            {error && JSON.stringify(error)}
        </>
    );
}
