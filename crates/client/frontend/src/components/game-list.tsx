import { useSuspenseQuery, gql } from "@apollo/client";
import { Button, List } from "antd";

export default function GamesList() {
    "use client";

    const { data, error } = useSuspenseQuery(gql``);

    const gameStatus = (game: Game) => {
        switch (game.status) {
            case "NotDownloaded":
                return <Button onClick={() => {}}>Download</Button>;
            case "Downloading":
                return <Button>Cancel</Button>;
            case "Running":
                return <p>Running...</p>;
            case "Stopped":
                return <Button>Start</Button>;
        }
    };

    return (
        <>
            <List
                bordered
                dataSource={Object.entries(data)}
                renderItem={([_, game]) => (
                    <List.Item>
                        <h2>{game.info.name}</h2>
                        <p>{gameStatus(game)}</p>
                    </List.Item>
                )}
            />

            {error && JSON.stringify(error)}
        </>
    );
}
