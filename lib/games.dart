import 'dart:async';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:stroke_text/stroke_text.dart';
import 'package:bramletts_games/src/rust/api/games.dart';
import 'package:bramletts_games/src/rust/frb_generated.dart';

class GameList extends StatefulWidget {
  const GameList({super.key});

  @override
  State<GameList> createState() => _GameListState();
}

class _GameListState extends State<GameList> {
  late Future<Games> futureGames;

  @override
  void initState() {
    super.initState();
    futureGames = fetchGames();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<Games>(
      future: futureGames,
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return ScaffoldPage.scrollable(children: [
            Wrap(
                spacing: 10.0,
                runSpacing: 10.0,
                children: snapshot.data!.games
                    .map((game) => GameWidget(game: game))
                    .toList(growable: false))
          ]);
        } else if (snapshot.hasError) {
          return Text('${snapshot.error}');
        }

        // By default, show a loading spinner.
        return const ProgressRing();
      },
    );
  }
}

class GameWidget extends StatefulWidget {
  const GameWidget({super.key, required this.game});

  final Game game;

  @override
  State<GameWidget> createState() => _GameWidgetState();
}

class _GameWidgetState extends State<GameWidget> {
  Progress? progress;
  bool downloaded = false;
  late bool installed = widget.game.state == GameState.installed;

  @override
  Widget build(BuildContext context) {
    final theme = FluentTheme.of(context);

    return SizedBox(
        width: 200,
        height: 300,
        child: Container(
            decoration: BoxDecoration(
                image: DecorationImage(
                    image: NetworkImage(widget.game.icon), fit: BoxFit.cover)),
            child: Card(
                child: Column(children: [
              StrokeText(
                  text: widget.game.name,
                  textStyle: theme.typography.subtitle,
                  strokeColor: theme.inactiveBackgroundColor,
                  strokeWidth: 10),
              progress == null
                  ? installed
                      ? FilledButton(
                          onPressed: () => run(), child: const Text("Run"))
                      : FilledButton(
                          onPressed: () => dl(), child: const Text("Download"))
                  : ProgressBarWidget(
                      color: widget.game.state == GameState.installed
                          ? Colors.magenta
                          : downloaded
                              ? Colors.blue
                              : Colors.yellow,
                      progress: progress!),
            ]))));
  }

  void dl() async {
    setState(() => progress = Progress.newProgress());
    final bytes = await downloadGame(game: widget.game, progress: progress!);
    setState(() => progress = null);
    downloaded = true;

    setState(() => progress = Progress.newProgress());
    await extractZip(bytes: bytes, game: widget.game, progress: progress!);
    setState(() {
      progress = null;
      installed = true;
    });
  }

  void run() async {
    setState(() => progress = Progress.newProgress());
    await runGame(game: widget.game);
    setState(() => progress = null);
  }
}
