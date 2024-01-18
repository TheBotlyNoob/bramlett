import 'package:fluent_ui/fluent_ui.dart';
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
                  body: GameList(),
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
          return ScaffoldPage.scrollable(children: [Text('a')]);
        } else if (snapshot.hasError) {
          return Text('${snapshot.error}');
        }

        // By default, show a loading spinner.
        return const ProgressRing();
      },

      // [
      //     Wrap(spacing: 10.0, runSpacing: 10.0, children: <Widget>[
      //       SizedBox(
      //           width: 250,
      //           height: 200,
      //           child: Card(
      //               child: Column(children: <Widget>[
      //             Text(
      //               'Long Long Long Game Name',
      //               style: FluentTheme.of(context).typography.subtitle!,
      //             ),
      //             Text('Desc'),
      //           ]))),
      //     ])
      //   ]
    );
  }
}
