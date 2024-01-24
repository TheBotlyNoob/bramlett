import 'dart:async';
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
