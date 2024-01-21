import 'dart:async';
import 'dart:typed_data';

import 'package:fluent_ui/fluent_ui.dart';
import 'package:stroke_text/stroke_text.dart';
import 'package:bramletts_games/src/rust/api/games.dart';
import 'package:bramletts_games/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const App());
}

class App extends StatefulWidget {
  const App({super.key});

  @override
  State<StatefulWidget> createState() => _AppState();
}

class _AppState extends State<App> {
  int navIdx = 0;

  @override
  Widget build(BuildContext context) {
    return FluentApp(
        title: 'Bramlett\'s Games',
        theme: FluentThemeData(
          brightness: Brightness.dark,
          accentColor: Colors.blue,
          visualDensity: VisualDensity.comfortable,
        ),
        home: NavigationView(
          appBar: const NavigationAppBar(title: Text("Bramlett's Games")),
          pane: NavigationPane(
            selected: navIdx,
            onChanged: (newIdx) {
              setState(() {
                navIdx = newIdx;
              });
            },
            displayMode: PaneDisplayMode.auto,
            items: [
              PaneItem(
                  body: const GameList(),
                  icon: const Icon(FluentIcons.download),
                  title: const Text("Download Games")),
              PaneItem(
                  body: const Center(child: Text('b')),
                  icon: const Icon(FluentIcons.blocked),
                  title: const Text("Unblocked Browser"))
            ],
          ),
        ));
  }
}

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
                  ? widget.game.state == GameState.notInstalled
                      ? FilledButton(
                          onPressed: () => dlGame(),
                          child: const Text("Download"))
                      : FilledButton(
                          onPressed: () => {}, child: const Text("Run"))
                  : ProgressBarWidget(
                      color: downloaded ? Colors.blue : Colors.green,
                      progress: progress!),
            ]))));
  }

  void dlGame() async {
    setState(() => progress = Progress.newProgress());
    final bytes = await downloadGame(game: widget.game, progress: progress!);
    progress = null;
    downloaded = true;

    setState(() => progress = Progress.newProgress());
    extractZip(bytes: bytes, game: widget.game, progress: progress!);
    progress = null;
  }
}

class ProgressBarWidget extends StatefulWidget {
  const ProgressBarWidget({super.key, this.color, required this.progress});

  final Color? color;
  final Progress progress;

  @override
  State<ProgressBarWidget> createState() => _ProgressBarWidgetState();
}

class _ProgressBarWidgetState extends State<ProgressBarWidget> {
  late Timer poll;

  @override
  void initState() {
    super.initState();
    poll = Timer.periodic(const Duration(seconds: 1), (timer) {
      setState(() {});
    });
  }

  @override
  void dispose() {
    super.dispose();
    poll.cancel();
  }

  @override
  Widget build(BuildContext context) {
    return ProgressBar(
        activeColor: widget.color,
        value: (widget.progress.getNumerator() /
                widget.progress.getDenominator()) *
            100);
  }
}
